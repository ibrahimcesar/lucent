//! Example Lucent Dynamic Plugin
//!
//! This is a template for creating dynamic plugins that can be loaded
//! by Lucent at runtime.
//!
//! # Building
//!
//! ```bash
//! cargo build --release
//! ```
//!
//! This will create a shared library:
//! - Linux: `target/release/liblucent_example_plugin.so`
//! - macOS: `target/release/liblucent_example_plugin.dylib`
//! - Windows: `target/release/lucent_example_plugin.dll`
//!
//! # Installing
//!
//! Copy the shared library to one of Lucent's plugin directories:
//! - `./plugins/` (current directory)
//! - `~/.local/share/lucent/plugins/` (user plugins)
//! - `/usr/local/lib/lucent/plugins/` (system plugins, Linux/macOS)

use lucent::metrics::FileMetrics;
use lucent::plugin::{DynamicPlugin, PluginCapability, PluginMetadata, PLUGIN_API_VERSION};
use std::path::Path;

/// Example plugin that adds custom metrics
#[derive(Default)]
pub struct ExamplePlugin;

impl DynamicPlugin for ExamplePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "example-plugin".to_string(),
            version: "0.1.0".to_string(),
            author: "Lucent Team".to_string(),
            description: "Example dynamic plugin demonstrating the plugin API".to_string(),
            api_version: PLUGIN_API_VERSION.to_string(),
            capabilities: vec![
                PluginCapability::MetricProvider(vec!["Custom".to_string()]),
            ],
        }
    }

    fn supported_languages(&self) -> Vec<String> {
        // This plugin supports a fictional "Custom" language
        // Replace with actual language names your plugin supports
        vec!["Custom".to_string()]
    }

    fn analyze(&self, path: &Path, source: &str, language: &str) -> Option<FileMetrics> {
        // Only analyze files we support
        if !self.supports_language(language) {
            return None;
        }

        // Create metrics for the file
        let mut metrics = FileMetrics::default();

        // Count lines
        let lines: Vec<&str> = source.lines().collect();
        metrics.total_lines = lines.len();

        // Count non-empty lines as code
        metrics.code_lines = lines.iter().filter(|l| !l.trim().is_empty()).count();

        // Simple comment detection (lines starting with #)
        metrics.comment_lines = lines
            .iter()
            .filter(|l| l.trim().starts_with('#'))
            .count();

        metrics.blank_lines = metrics.total_lines - metrics.code_lines - metrics.comment_lines;

        // Set the file path
        metrics.path = path.to_string_lossy().to_string();

        // Log that we analyzed the file (for demonstration)
        eprintln!(
            "[ExamplePlugin] Analyzed {}: {} lines, {} code, {} comments",
            path.display(),
            metrics.total_lines,
            metrics.code_lines,
            metrics.comment_lines
        );

        Some(metrics)
    }

    fn priority(&self) -> u32 {
        // Higher than default (50) so this plugin takes precedence
        // for "Custom" language files
        100
    }

    fn on_unload(&self) {
        eprintln!("[ExamplePlugin] Plugin unloaded");
    }
}

// Export the plugin using Lucent's macro
lucent::declare_plugin!(ExamplePlugin);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let plugin = ExamplePlugin;
        let metadata = plugin.metadata();
        assert_eq!(metadata.name, "example-plugin");
        assert_eq!(metadata.version, "0.1.0");
    }

    #[test]
    fn test_supported_languages() {
        let plugin = ExamplePlugin;
        assert!(plugin.supports_language("Custom"));
        assert!(plugin.supports_language("custom")); // Case insensitive
        assert!(!plugin.supports_language("Rust"));
    }

    #[test]
    fn test_analyze() {
        let plugin = ExamplePlugin;
        let source = "# Comment\ncode line\n\nanother line";
        let path = Path::new("test.custom");

        let metrics = plugin.analyze(path, source, "Custom").unwrap();

        assert_eq!(metrics.total_lines, 4);
        assert_eq!(metrics.comment_lines, 1);
        assert_eq!(metrics.blank_lines, 1);
    }

    #[test]
    fn test_unsupported_language() {
        let plugin = ExamplePlugin;
        let result = plugin.analyze(Path::new("test.rs"), "fn main() {}", "Rust");
        assert!(result.is_none());
    }
}
