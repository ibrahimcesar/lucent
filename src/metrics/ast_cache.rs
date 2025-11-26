//! AST caching layer for tree-sitter parsed trees and metrics
//!
//! Provides caching for:
//! - Parsed AST metrics (file-level)
//! - Query results (query string + file hash)
//! - Lazy parsing support (parse only when needed)

use super::ast_metrics::{AstAnalyzer, AstFileMetrics, QueryMatch};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

/// Cache key combining file path and content hash
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    path: PathBuf,
    content_hash: u64,
}

impl CacheKey {
    fn new(path: &Path, content: &str) -> Self {
        Self {
            path: path.to_path_buf(),
            content_hash: Self::hash_content(content),
        }
    }

    fn hash_content(content: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }
}

/// Cached AST metrics entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstCacheEntry {
    /// Content hash for validation
    pub content_hash: u64,
    /// File modification time
    pub mtime: Option<SystemTime>,
    /// Cached metrics
    pub metrics: AstFileMetrics,
    /// Language used for analysis
    pub language: String,
}

/// Cached query result
#[derive(Debug, Clone)]
struct QueryCacheEntry {
    content_hash: u64,
    query_hash: u64,
    matches: Vec<QueryMatch>,
}

/// Thread-safe AST cache for parallel analysis
pub struct AstCache {
    /// Metrics cache (path -> entry)
    metrics_cache: RwLock<HashMap<PathBuf, AstCacheEntry>>,
    /// Query results cache (path + query -> matches)
    query_cache: RwLock<HashMap<(PathBuf, u64), QueryCacheEntry>>,
    /// Statistics
    stats: RwLock<CacheStats>,
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of cache hits for metrics
    pub metrics_hits: usize,
    /// Number of cache misses for metrics
    pub metrics_misses: usize,
    /// Number of cache hits for queries
    pub query_hits: usize,
    /// Number of cache misses for queries
    pub query_misses: usize,
}

impl CacheStats {
    /// Calculate hit rate for metrics
    pub fn metrics_hit_rate(&self) -> f64 {
        let total = self.metrics_hits + self.metrics_misses;
        if total == 0 {
            0.0
        } else {
            self.metrics_hits as f64 / total as f64
        }
    }

    /// Calculate hit rate for queries
    pub fn query_hit_rate(&self) -> f64 {
        let total = self.query_hits + self.query_misses;
        if total == 0 {
            0.0
        } else {
            self.query_hits as f64 / total as f64
        }
    }
}

impl Default for AstCache {
    fn default() -> Self {
        Self::new()
    }
}

impl AstCache {
    /// Create a new empty cache
    pub fn new() -> Self {
        Self {
            metrics_cache: RwLock::new(HashMap::new()),
            query_cache: RwLock::new(HashMap::new()),
            stats: RwLock::new(CacheStats::default()),
        }
    }

    /// Get cached metrics or compute them
    pub fn get_or_compute_metrics(
        &self,
        path: &Path,
        content: &str,
        language: &str,
    ) -> Option<AstFileMetrics> {
        let content_hash = CacheKey::hash_content(content);

        // Try to get from cache
        {
            let cache = self.metrics_cache.read().ok()?;
            if let Some(entry) = cache.get(path) {
                if entry.content_hash == content_hash && entry.language == language {
                    // Update stats
                    if let Ok(mut stats) = self.stats.write() {
                        stats.metrics_hits += 1;
                    }
                    return Some(entry.metrics.clone());
                }
            }
        }

        // Cache miss - compute metrics
        if let Ok(mut stats) = self.stats.write() {
            stats.metrics_misses += 1;
        }

        let mut analyzer = AstAnalyzer::new(language)?;
        let metrics = analyzer.analyze(content)?;

        // Store in cache
        let entry = AstCacheEntry {
            content_hash,
            mtime: std::fs::metadata(path).ok().and_then(|m| m.modified().ok()),
            metrics: metrics.clone(),
            language: language.to_string(),
        };

        if let Ok(mut cache) = self.metrics_cache.write() {
            cache.insert(path.to_path_buf(), entry);
        }

        Some(metrics)
    }

    /// Get cached query results or compute them
    pub fn get_or_compute_query(
        &self,
        path: &Path,
        content: &str,
        language: &str,
        query: &str,
    ) -> Option<Vec<QueryMatch>> {
        let content_hash = CacheKey::hash_content(content);
        let query_hash = CacheKey::hash_content(query);

        // Try to get from cache
        {
            let cache = self.query_cache.read().ok()?;
            let key = (path.to_path_buf(), query_hash);
            if let Some(entry) = cache.get(&key) {
                if entry.content_hash == content_hash {
                    // Update stats
                    if let Ok(mut stats) = self.stats.write() {
                        stats.query_hits += 1;
                    }
                    return Some(entry.matches.clone());
                }
            }
        }

        // Cache miss - compute query results
        if let Ok(mut stats) = self.stats.write() {
            stats.query_misses += 1;
        }

        let mut analyzer = AstAnalyzer::new(language)?;
        let matches = analyzer.query_matches(content, query)?;

        // Store in cache
        let entry = QueryCacheEntry {
            content_hash,
            query_hash,
            matches: matches.clone(),
        };

        if let Ok(mut cache) = self.query_cache.write() {
            cache.insert((path.to_path_buf(), query_hash), entry);
        }

        Some(matches)
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        self.stats.read().map(|s| s.clone()).unwrap_or_default()
    }

    /// Clear the entire cache
    pub fn clear(&self) {
        if let Ok(mut cache) = self.metrics_cache.write() {
            cache.clear();
        }
        if let Ok(mut cache) = self.query_cache.write() {
            cache.clear();
        }
    }

    /// Remove entries for a specific file
    pub fn invalidate(&self, path: &Path) {
        if let Ok(mut cache) = self.metrics_cache.write() {
            cache.remove(path);
        }
        if let Ok(mut cache) = self.query_cache.write() {
            cache.retain(|(p, _), _| p != path);
        }
    }

    /// Get number of cached entries
    pub fn len(&self) -> usize {
        let metrics_len = self.metrics_cache.read().map(|c| c.len()).unwrap_or(0);
        let query_len = self.query_cache.read().map(|c| c.len()).unwrap_or(0);
        metrics_len + query_len
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Lazy AST analyzer that only parses when needed
pub struct LazyAstAnalyzer {
    language: String,
    cache: Arc<AstCache>,
}

impl LazyAstAnalyzer {
    /// Create a new lazy analyzer with shared cache
    pub fn new(language: &str, cache: Arc<AstCache>) -> Option<Self> {
        // Verify language is supported
        if super::ast_metrics::AstAnalyzer::new(language).is_none() {
            return None;
        }

        Some(Self {
            language: language.to_string(),
            cache,
        })
    }

    /// Analyze source code with caching
    pub fn analyze(&self, path: &Path, content: &str) -> Option<AstFileMetrics> {
        self.cache.get_or_compute_metrics(path, content, &self.language)
    }

    /// Run a query with caching
    pub fn query(&self, path: &Path, content: &str, query: &str) -> Option<Vec<QueryMatch>> {
        self.cache.get_or_compute_query(path, content, &self.language, query)
    }

    /// Run a query and count matches
    pub fn query_count(&self, path: &Path, content: &str, query: &str) -> Option<usize> {
        self.query(path, content, query).map(|m| m.len())
    }
}

/// Parallel AST analyzer for batch processing
pub struct ParallelAstAnalyzer {
    cache: Arc<AstCache>,
}

impl Default for ParallelAstAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ParallelAstAnalyzer {
    /// Create a new parallel analyzer
    pub fn new() -> Self {
        Self {
            cache: Arc::new(AstCache::new()),
        }
    }

    /// Create with an existing cache
    pub fn with_cache(cache: Arc<AstCache>) -> Self {
        Self { cache }
    }

    /// Analyze multiple files in parallel
    pub fn analyze_batch(&self, files: &[(PathBuf, String, String)]) -> Vec<(PathBuf, Option<AstFileMetrics>)> {
        use rayon::prelude::*;

        files.par_iter()
            .map(|(path, content, language)| {
                let metrics = self.cache.get_or_compute_metrics(path, content, language);
                (path.clone(), metrics)
            })
            .collect()
    }

    /// Run a query on multiple files in parallel
    pub fn query_batch(
        &self,
        files: &[(PathBuf, String, String)],
        query: &str,
    ) -> Vec<(PathBuf, Option<Vec<QueryMatch>>)> {
        use rayon::prelude::*;

        files.par_iter()
            .map(|(path, content, language)| {
                let matches = self.cache.get_or_compute_query(path, content, language, query);
                (path.clone(), matches)
            })
            .collect()
    }

    /// Get the underlying cache
    pub fn cache(&self) -> Arc<AstCache> {
        Arc::clone(&self.cache)
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        self.cache.stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_cache_metrics() {
        let cache = AstCache::new();
        let path = PathBuf::from("test.rs");
        let content = "fn foo() {} fn bar() {}";

        // First call should miss
        let metrics1 = cache.get_or_compute_metrics(&path, content, "Rust").unwrap();

        // Second call should hit
        let metrics2 = cache.get_or_compute_metrics(&path, content, "Rust").unwrap();

        assert_eq!(metrics1.function_count, metrics2.function_count);

        let stats = cache.stats();
        assert_eq!(stats.metrics_hits, 1);
        assert_eq!(stats.metrics_misses, 1);
    }

    #[test]
    fn test_cache_invalidation() {
        let cache = AstCache::new();
        let path = PathBuf::from("test.rs");
        let content = "fn foo() {}";

        // Populate cache
        cache.get_or_compute_metrics(&path, content, "Rust");
        assert!(!cache.is_empty());

        // Invalidate
        cache.invalidate(&path);

        // Should be empty for metrics, but verify through stats
        let stats = cache.stats();
        assert_eq!(stats.metrics_misses, 1);
    }

    #[test]
    fn test_lazy_analyzer() {
        let cache = Arc::new(AstCache::new());
        let analyzer = LazyAstAnalyzer::new("Rust", cache.clone()).unwrap();

        let path = PathBuf::from("test.rs");
        let content = "fn foo() { if true { } }";

        let metrics = analyzer.analyze(&path, content).unwrap();
        assert_eq!(metrics.function_count, 1);
        assert!(metrics.cyclomatic_complexity >= 1);

        // Verify cache was used
        let stats = cache.stats();
        assert_eq!(stats.metrics_misses, 1);

        // Second call should hit cache
        let _ = analyzer.analyze(&path, content);
        let stats = cache.stats();
        assert_eq!(stats.metrics_hits, 1);
    }

    #[test]
    fn test_parallel_analyzer() {
        let analyzer = ParallelAstAnalyzer::new();

        let files = vec![
            (PathBuf::from("a.rs"), "fn a() {}".to_string(), "Rust".to_string()),
            (PathBuf::from("b.rs"), "fn b() {}".to_string(), "Rust".to_string()),
            (PathBuf::from("c.rs"), "fn c() {}".to_string(), "Rust".to_string()),
        ];

        let results = analyzer.analyze_batch(&files);
        assert_eq!(results.len(), 3);

        for (_, metrics) in results {
            assert!(metrics.is_some());
            assert_eq!(metrics.unwrap().function_count, 1);
        }
    }
}
