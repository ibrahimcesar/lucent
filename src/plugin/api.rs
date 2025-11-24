//! Plugin API traits and types
//!
//! Defines the core interfaces that plugins must implement.

use crate::metrics::FileMetrics;
use anyhow::Result;
use std::path::Path;

/// Metadata about a plugin
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    /// Plugin name
    pub name: String,

    /// Plugin version
    pub version: String,

    /// Plugin author
    pub author: String,

    /// Plugin description
    pub description: String,

    /// API version this plugin was built against
    pub api_version: String,

    /// Capabilities provided by this plugin
    pub capabilities: Vec<PluginCapability>,
}

/// Capabilities that a plugin can provide
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginCapability {
    /// Provides metric analysis for specific languages
    MetricProvider(Vec<String>),

    /// Provides custom output formatting
    OutputFormatter(String),

    /// Provides additional quality checks
    QualityChecker,
}

/// Main trait that all metric provider plugins must implement
///
/// This trait allows plugins to analyze source code and produce metrics.
/// Plugins can provide language-specific or cross-language analysis.
pub trait MetricProvider: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;

    /// Get list of languages this plugin supports
    ///
    /// Return language names (e.g., "Rust", "Python", "JavaScript")
    fn supported_languages(&self) -> Vec<&str>;

    /// Check if this plugin can handle the given language
    fn supports_language(&self, language: &str) -> bool {
        self.supported_languages()
            .iter()
            .any(|lang| lang.eq_ignore_ascii_case(language))
    }

    /// Analyze source code and produce metrics
    ///
    /// # Arguments
    /// * `path` - Path to the source file
    /// * `source` - Source code content
    /// * `language` - Language name
    ///
    /// # Returns
    /// * `Ok(Some(metrics))` - Plugin successfully analyzed the file
    /// * `Ok(None)` - Plugin doesn't support this language/file
    /// * `Err(e)` - Analysis failed
    fn analyze(&self, path: &Path, source: &str, language: &str) -> Result<Option<FileMetrics>>;

    /// Get plugin priority (higher = used first)
    ///
    /// When multiple plugins support the same language, the one with
    /// highest priority is used. Default is 50.
    fn priority(&self) -> u32 {
        50
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin;

    impl MetricProvider for TestPlugin {
        fn metadata(&self) -> PluginMetadata {
            PluginMetadata {
                name: "test-plugin".to_string(),
                version: "1.0.0".to_string(),
                author: "test".to_string(),
                description: "Test plugin".to_string(),
                api_version: crate::plugin::PLUGIN_API_VERSION.to_string(),
                capabilities: vec![PluginCapability::MetricProvider(vec!["Rust".to_string()])],
            }
        }

        fn supported_languages(&self) -> Vec<&str> {
            vec!["Rust"]
        }

        fn analyze(&self, _path: &Path, _source: &str, language: &str) -> Result<Option<FileMetrics>> {
            if self.supports_language(language) {
                Ok(Some(FileMetrics::default()))
            } else {
                Ok(None)
            }
        }
    }

    #[test]
    fn test_plugin_supports_language() {
        let plugin = TestPlugin;
        assert!(plugin.supports_language("Rust"));
        assert!(plugin.supports_language("rust"));
        assert!(!plugin.supports_language("Python"));
    }

    #[test]
    fn test_plugin_metadata() {
        let plugin = TestPlugin;
        let metadata = plugin.metadata();
        assert_eq!(metadata.name, "test-plugin");
        assert_eq!(metadata.version, "1.0.0");
    }

    #[test]
    fn test_default_priority() {
        let plugin = TestPlugin;
        assert_eq!(plugin.priority(), 50);
    }
}
