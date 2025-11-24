//! Plugin registry for managing loaded plugins
//!
//! The registry discovers, loads, and manages plugins at runtime.

use super::api::MetricProvider;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

/// Errors that can occur during plugin operations
#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    NotFound(String),

    #[error("Plugin already registered: {0}")]
    AlreadyRegistered(String),

    #[error("No plugin available for language: {0}")]
    NoProviderForLanguage(String),

    #[error("Plugin load failed: {0}")]
    LoadFailed(String),
}

/// Registry for managing plugins
///
/// The registry maintains a collection of loaded plugins and provides
/// methods to find appropriate plugins for specific languages.
pub struct PluginRegistry {
    /// Registered metric providers by name
    providers: HashMap<String, Arc<dyn MetricProvider>>,

    /// Language to provider mapping (for fast lookup)
    language_map: HashMap<String, Vec<String>>,
}

impl PluginRegistry {
    /// Create a new empty plugin registry
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            language_map: HashMap::new(),
        }
    }

    /// Register a new plugin
    ///
    /// # Arguments
    /// * `provider` - The metric provider to register
    ///
    /// # Returns
    /// * `Ok(())` - Plugin registered successfully
    /// * `Err(PluginError)` - Plugin with same name already registered
    pub fn register(&mut self, provider: Arc<dyn MetricProvider>) -> Result<(), PluginError> {
        let name = provider.metadata().name.clone();

        // Check if already registered
        if self.providers.contains_key(&name) {
            return Err(PluginError::AlreadyRegistered(name));
        }

        // Build language mapping
        for language in provider.supported_languages() {
            let lang_key = language.to_lowercase();
            self.language_map
                .entry(lang_key)
                .or_insert_with(Vec::new)
                .push(name.clone());
        }

        // Register provider
        self.providers.insert(name, provider);

        Ok(())
    }

    /// Get a plugin by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn MetricProvider>> {
        self.providers.get(name).cloned()
    }

    /// Find the best plugin for a given language
    ///
    /// Returns the plugin with highest priority that supports the language.
    pub fn find_for_language(&self, language: &str) -> Option<Arc<dyn MetricProvider>> {
        let lang_key = language.to_lowercase();
        let provider_names = self.language_map.get(&lang_key)?;

        // Find plugin with highest priority
        provider_names
            .iter()
            .filter_map(|name| self.providers.get(name))
            .max_by_key(|provider| provider.priority())
            .cloned()
    }

    /// Get all registered plugins
    pub fn list_all(&self) -> Vec<Arc<dyn MetricProvider>> {
        self.providers.values().cloned().collect()
    }

    /// Get plugins that support a specific language
    pub fn list_for_language(&self, language: &str) -> Vec<Arc<dyn MetricProvider>> {
        let lang_key = language.to_lowercase();
        if let Some(provider_names) = self.language_map.get(&lang_key) {
            provider_names
                .iter()
                .filter_map(|name| self.providers.get(name).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all supported languages across all plugins
    pub fn supported_languages(&self) -> Vec<String> {
        let mut languages: Vec<String> = self.language_map.keys().cloned().collect();
        languages.sort();
        languages
    }

    /// Get number of registered plugins
    pub fn count(&self) -> usize {
        self.providers.len()
    }

    /// Check if a language is supported by any plugin
    pub fn is_language_supported(&self, language: &str) -> bool {
        let lang_key = language.to_lowercase();
        self.language_map.contains_key(&lang_key)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::api::{PluginCapability, PluginMetadata};
    use crate::metrics::FileMetrics;
    use anyhow::Result;
    use std::path::Path;

    struct MockPlugin {
        name: String,
        languages: Vec<String>,
        priority: u32,
    }

    impl MetricProvider for MockPlugin {
        fn metadata(&self) -> PluginMetadata {
            PluginMetadata {
                name: self.name.clone(),
                version: "1.0.0".to_string(),
                author: "test".to_string(),
                description: "Mock plugin".to_string(),
                api_version: crate::plugin::PLUGIN_API_VERSION.to_string(),
                capabilities: vec![PluginCapability::MetricProvider(self.languages.clone())],
            }
        }

        fn supported_languages(&self) -> Vec<&str> {
            self.languages.iter().map(|s| s.as_str()).collect()
        }

        fn analyze(&self, _path: &Path, _source: &str, _language: &str) -> Result<Option<FileMetrics>> {
            Ok(Some(FileMetrics::default()))
        }

        fn priority(&self) -> u32 {
            self.priority
        }
    }

    #[test]
    fn test_register_plugin() {
        let mut registry = PluginRegistry::new();
        let plugin = Arc::new(MockPlugin {
            name: "test".to_string(),
            languages: vec!["Rust".to_string()],
            priority: 50,
        });

        assert!(registry.register(plugin).is_ok());
        assert_eq!(registry.count(), 1);
    }

    #[test]
    fn test_duplicate_registration() {
        let mut registry = PluginRegistry::new();
        let plugin1 = Arc::new(MockPlugin {
            name: "test".to_string(),
            languages: vec!["Rust".to_string()],
            priority: 50,
        });
        let plugin2 = Arc::new(MockPlugin {
            name: "test".to_string(),
            languages: vec!["Python".to_string()],
            priority: 50,
        });

        assert!(registry.register(plugin1).is_ok());
        assert!(matches!(
            registry.register(plugin2),
            Err(PluginError::AlreadyRegistered(_))
        ));
    }

    #[test]
    fn test_find_for_language() {
        let mut registry = PluginRegistry::new();
        let plugin = Arc::new(MockPlugin {
            name: "rust-plugin".to_string(),
            languages: vec!["Rust".to_string()],
            priority: 50,
        });

        registry.register(plugin).unwrap();

        assert!(registry.find_for_language("Rust").is_some());
        assert!(registry.find_for_language("rust").is_some()); // Case insensitive
        assert!(registry.find_for_language("Python").is_none());
    }

    #[test]
    fn test_priority_selection() {
        let mut registry = PluginRegistry::new();

        let plugin_low = Arc::new(MockPlugin {
            name: "low-priority".to_string(),
            languages: vec!["Rust".to_string()],
            priority: 30,
        });

        let plugin_high = Arc::new(MockPlugin {
            name: "high-priority".to_string(),
            languages: vec!["Rust".to_string()],
            priority: 70,
        });

        registry.register(plugin_low).unwrap();
        registry.register(plugin_high).unwrap();

        let best = registry.find_for_language("Rust").unwrap();
        assert_eq!(best.metadata().name, "high-priority");
    }

    #[test]
    fn test_list_all() {
        let mut registry = PluginRegistry::new();

        let plugin1 = Arc::new(MockPlugin {
            name: "plugin1".to_string(),
            languages: vec!["Rust".to_string()],
            priority: 50,
        });

        let plugin2 = Arc::new(MockPlugin {
            name: "plugin2".to_string(),
            languages: vec!["Python".to_string()],
            priority: 50,
        });

        registry.register(plugin1).unwrap();
        registry.register(plugin2).unwrap();

        assert_eq!(registry.list_all().len(), 2);
    }

    #[test]
    fn test_supported_languages() {
        let mut registry = PluginRegistry::new();

        let plugin = Arc::new(MockPlugin {
            name: "multi-lang".to_string(),
            languages: vec!["Rust".to_string(), "Python".to_string(), "JavaScript".to_string()],
            priority: 50,
        });

        registry.register(plugin).unwrap();

        let languages = registry.supported_languages();
        assert_eq!(languages.len(), 3);
        assert!(languages.contains(&"rust".to_string()));
        assert!(languages.contains(&"python".to_string()));
        assert!(languages.contains(&"javascript".to_string()));
    }
}
