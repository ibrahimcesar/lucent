//! Built-in plugins that ship with Lucent
//!
//! These plugins provide the default functionality using regex-based analysis.

use super::api::{MetricProvider, PluginMetadata, PluginCapability};
use crate::metrics::FileMetrics;
use crate::language::LANGUAGES;
use anyhow::Result;
use std::path::Path;

/// Built-in regex-based analyzer
///
/// This is the default analyzer that uses pattern matching to calculate metrics.
/// It supports all languages that Lucent knows about.
pub struct RegexAnalyzer;

impl RegexAnalyzer {
    /// Create a new regex analyzer instance
    pub fn new() -> Self {
        Self
    }
}

impl MetricProvider for RegexAnalyzer {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "lucent-builtin-regex".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            author: "Lucent Team".to_string(),
            description: "Built-in regex-based code analyzer".to_string(),
            api_version: crate::plugin::PLUGIN_API_VERSION.to_string(),
            capabilities: vec![PluginCapability::MetricProvider(
                LANGUAGES.iter().map(|l| l.name.to_string()).collect()
            )],
        }
    }

    fn supported_languages(&self) -> Vec<&str> {
        LANGUAGES.iter().map(|l| l.name).collect()
    }

    fn analyze(&self, path: &Path, _source: &str, language: &str) -> Result<Option<FileMetrics>> {
        // Find the language definition
        let lang = LANGUAGES.iter().find(|l| l.name.eq_ignore_ascii_case(language));

        if let Some(lang) = lang {
            // Use the existing FileMetrics::analyze method
            // This calls all the regex-based analysis functions
            let metrics = FileMetrics::analyze(path, lang)?;
            Ok(Some(metrics))
        } else {
            Ok(None)
        }
    }

    fn priority(&self) -> u32 {
        // Lower priority than external plugins
        // This ensures tree-sitter or other plugins are preferred
        10
    }
}

impl Default for RegexAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_analyzer_metadata() {
        let analyzer = RegexAnalyzer::new();
        let metadata = analyzer.metadata();

        assert_eq!(metadata.name, "lucent-builtin-regex");
        assert!(!metadata.capabilities.is_empty());
    }

    #[test]
    fn test_regex_analyzer_supports_languages() {
        let analyzer = RegexAnalyzer::new();

        assert!(analyzer.supports_language("Rust"));
        assert!(analyzer.supports_language("Python"));
        assert!(analyzer.supports_language("JavaScript"));
        assert!(!analyzer.supports_language("Unknown"));
    }

    #[test]
    fn test_regex_analyzer_priority() {
        let analyzer = RegexAnalyzer::new();
        assert_eq!(analyzer.priority(), 10);
    }

    #[test]
    fn test_regex_analyzer_analyze_rust() {
        let analyzer = RegexAnalyzer::new();
        let source = r#"
            fn main() {
                println!("Hello, world!");
            }
        "#;

        // Create a temporary file for testing
        use std::fs;
        use std::env;

        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_plugin.rs");
        fs::write(&test_file, source).unwrap();

        let result = analyzer.analyze(&test_file, source, "Rust");

        // Clean up
        let _ = fs::remove_file(&test_file);

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_regex_analyzer_analyze_unknown_language() {
        let analyzer = RegexAnalyzer::new();
        let source = "some code";

        let path = Path::new("test.unknown");
        let result = analyzer.analyze(path, source, "Unknown");

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
