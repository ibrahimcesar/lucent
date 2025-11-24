//! Caching layer for file analysis results
//!
//! Caches analysis results based on file content hash to avoid re-analyzing
//! unchanged files.

use crate::metrics::FileMetrics;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Cache entry for a file's analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Content hash (using file size + mtime as simple hash)
    pub hash: u64,

    /// Last modified time
    pub mtime: SystemTime,

    /// File size in bytes
    pub size: u64,

    /// Cached metrics
    pub metrics: FileMetrics,
}

/// Analysis cache manager
pub struct AnalysisCache {
    /// Cache storage
    cache: HashMap<PathBuf, CacheEntry>,

    /// Cache file path
    cache_path: PathBuf,

    /// Whether cache is dirty (needs saving)
    dirty: bool,
}

impl AnalysisCache {
    /// Create a new cache
    pub fn new() -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("lucent");

        fs::create_dir_all(&cache_dir)?;

        let cache_path = cache_dir.join("analysis_cache.json");

        let cache = if cache_path.exists() {
            Self::load_cache(&cache_path)?
        } else {
            HashMap::new()
        };

        Ok(Self {
            cache,
            cache_path,
            dirty: false,
        })
    }

    /// Load cache from disk
    fn load_cache(path: &Path) -> Result<HashMap<PathBuf, CacheEntry>> {
        let contents = fs::read_to_string(path)?;
        let cache = serde_json::from_str(&contents)?;
        Ok(cache)
    }

    /// Save cache to disk
    pub fn save(&self) -> Result<()> {
        if !self.dirty {
            return Ok(());
        }

        let contents = serde_json::to_string_pretty(&self.cache)?;
        fs::write(&self.cache_path, contents)?;
        Ok(())
    }

    /// Get cached metrics for a file if still valid
    pub fn get(&self, path: &Path) -> Option<&FileMetrics> {
        let entry = self.cache.get(path)?;

        // Validate cache entry
        if let Ok(metadata) = fs::metadata(path) {
            let size = metadata.len();
            let mtime = metadata.modified().ok()?;

            // Check if file hasn't changed
            if entry.size == size && entry.mtime == mtime {
                return Some(&entry.metrics);
            }
        }

        None
    }

    /// Store metrics in cache
    pub fn insert(&mut self, path: PathBuf, metrics: FileMetrics) -> Result<()> {
        let metadata = fs::metadata(&path)?;
        let size = metadata.len();
        let mtime = metadata.modified()?;

        // Simple hash: combine size and mtime
        let hash = size;

        let entry = CacheEntry {
            hash,
            mtime,
            size,
            metrics,
        };

        self.cache.insert(path, entry);
        self.dirty = true;

        Ok(())
    }

    /// Clear all cached entries
    pub fn clear(&mut self) {
        self.cache.clear();
        self.dirty = true;
    }

    /// Get number of cached entries
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Remove stale entries (files that no longer exist)
    pub fn cleanup(&mut self) {
        self.cache.retain(|path, _| path.exists());
        self.dirty = true;
    }
}

impl Drop for AnalysisCache {
    fn drop(&mut self) {
        // Auto-save on drop
        let _ = self.save();
    }
}

impl Default for AnalysisCache {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            cache: HashMap::new(),
            cache_path: PathBuf::from("analysis_cache.json"),
            dirty: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cache_creation() {
        let cache = AnalysisCache::new();
        assert!(cache.is_ok());
    }

    #[test]
    fn test_cache_insert_and_get() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "fn main() {}").unwrap();

        let mut cache = AnalysisCache::new().unwrap();
        let metrics = FileMetrics::default();

        cache.insert(test_file.clone(), metrics.clone()).unwrap();

        let cached = cache.get(&test_file);
        assert!(cached.is_some());
    }
}
