//! Dynamic plugin loading from shared libraries
//!
//! This module provides support for loading plugins from shared libraries
//! (`.so` on Linux, `.dylib` on macOS, `.dll` on Windows).
//!
//! # Plugin Development
//!
//! To create a dynamic plugin:
//!
//! 1. Create a new Rust library with `crate-type = ["cdylib"]`
//! 2. Implement `DynamicPlugin` trait
//! 3. Export the plugin using `declare_plugin!` macro
//!
//! # Example Plugin
//!
//! ```rust,ignore
//! use lucent::plugin::{DynamicPlugin, PluginMetadata, PluginCapability};
//! use lucent::metrics::FileMetrics;
//! use std::path::Path;
//!
//! pub struct MyPlugin;
//!
//! impl DynamicPlugin for MyPlugin {
//!     fn metadata(&self) -> PluginMetadata {
//!         PluginMetadata {
//!             name: "my-plugin".to_string(),
//!             version: "1.0.0".to_string(),
//!             author: "Your Name".to_string(),
//!             description: "My custom plugin".to_string(),
//!             api_version: lucent::plugin::PLUGIN_API_VERSION.to_string(),
//!             capabilities: vec![PluginCapability::MetricProvider(vec!["Rust".to_string()])],
//!         }
//!     }
//!
//!     fn supported_languages(&self) -> Vec<String> {
//!         vec!["Rust".to_string()]
//!     }
//!
//!     fn analyze(&self, path: &Path, source: &str, language: &str) -> Option<FileMetrics> {
//!         // Your analysis logic here
//!         Some(FileMetrics::default())
//!     }
//! }
//!
//! lucent::declare_plugin!(MyPlugin);
//! ```

use super::api::{PluginCapability, PluginMetadata};
use crate::metrics::FileMetrics;
use libloading::{Library, Symbol};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;

/// Plugin API version for ABI compatibility
pub const DYNAMIC_PLUGIN_API_VERSION: u32 = 1;

/// Errors that can occur during dynamic plugin loading
#[derive(Debug, Error)]
pub enum DynamicPluginError {
    /// Failed to load the shared library
    #[error("Failed to load plugin library: {0}")]
    LoadError(#[from] libloading::Error),

    /// Plugin API version mismatch
    #[error("Plugin API version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: u32, actual: u32 },

    /// Plugin initialization failed
    #[error("Plugin initialization failed: {0}")]
    InitError(String),

    /// Plugin not found at path
    #[error("Plugin not found: {0}")]
    NotFound(PathBuf),

    /// Invalid plugin (missing required symbols)
    #[error("Invalid plugin: missing symbol '{0}'")]
    MissingSymbol(String),
}

/// Result type for dynamic plugin operations
pub type DynamicPluginResult<T> = Result<T, DynamicPluginError>;

/// Trait that dynamic plugins must implement
///
/// Unlike `MetricProvider`, this trait uses owned types to ensure
/// safe FFI across shared library boundaries.
pub trait DynamicPlugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;

    /// Get list of languages this plugin supports
    fn supported_languages(&self) -> Vec<String>;

    /// Check if this plugin supports the given language
    fn supports_language(&self, language: &str) -> bool {
        self.supported_languages()
            .iter()
            .any(|lang| lang.eq_ignore_ascii_case(language))
    }

    /// Analyze source code and produce metrics
    fn analyze(&self, path: &Path, source: &str, language: &str) -> Option<FileMetrics>;

    /// Get plugin priority (higher = used first)
    fn priority(&self) -> u32 {
        50
    }

    /// Called when plugin is being unloaded
    fn on_unload(&self) {}
}

/// Type alias for the plugin creation function
pub type CreatePluginFn = unsafe extern "C" fn() -> *mut dyn DynamicPlugin;

/// Type alias for the plugin destruction function
pub type DestroyPluginFn = unsafe extern "C" fn(*mut dyn DynamicPlugin);

/// Type alias for the API version function
pub type ApiVersionFn = unsafe extern "C" fn() -> u32;

/// A loaded dynamic plugin
pub struct LoadedPlugin {
    /// The plugin instance
    plugin: Box<dyn DynamicPlugin>,
    /// The library handle (kept to prevent unloading)
    _library: Arc<Library>,
    /// Path to the plugin file
    path: PathBuf,
}

impl LoadedPlugin {
    /// Get the plugin instance
    pub fn plugin(&self) -> &dyn DynamicPlugin {
        &*self.plugin
    }

    /// Get the path to the plugin file
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get plugin metadata
    pub fn metadata(&self) -> PluginMetadata {
        self.plugin.metadata()
    }
}

/// Manager for loading and managing dynamic plugins
pub struct DynamicPluginManager {
    /// Loaded plugins
    plugins: Vec<LoadedPlugin>,
    /// Plugin search paths
    search_paths: Vec<PathBuf>,
}

impl Default for DynamicPluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicPluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        let mut search_paths = vec![
            // Current directory plugins
            PathBuf::from("./plugins"),
            // User plugins
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("lucent")
                .join("plugins"),
        ];

        // Add system plugin directory on Unix
        #[cfg(unix)]
        {
            search_paths.push(PathBuf::from("/usr/local/lib/lucent/plugins"));
            search_paths.push(PathBuf::from("/usr/lib/lucent/plugins"));
        }

        Self {
            plugins: Vec::new(),
            search_paths,
        }
    }

    /// Add a search path for plugins
    pub fn add_search_path(&mut self, path: impl Into<PathBuf>) {
        self.search_paths.push(path.into());
    }

    /// Load a plugin from a specific path
    pub fn load_plugin(&mut self, path: impl AsRef<Path>) -> DynamicPluginResult<&LoadedPlugin> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(DynamicPluginError::NotFound(path.to_path_buf()));
        }

        unsafe {
            // Load the library
            let library = Library::new(path)?;
            let library = Arc::new(library);

            // Check API version
            let api_version: Symbol<ApiVersionFn> = library
                .get(b"lucent_plugin_api_version")
                .map_err(|_| DynamicPluginError::MissingSymbol("lucent_plugin_api_version".to_string()))?;

            let version = api_version();
            if version != DYNAMIC_PLUGIN_API_VERSION {
                return Err(DynamicPluginError::VersionMismatch {
                    expected: DYNAMIC_PLUGIN_API_VERSION,
                    actual: version,
                });
            }

            // Get the create function
            let create: Symbol<CreatePluginFn> = library
                .get(b"lucent_plugin_create")
                .map_err(|_| DynamicPluginError::MissingSymbol("lucent_plugin_create".to_string()))?;

            // Create the plugin instance
            let plugin_ptr = create();
            if plugin_ptr.is_null() {
                return Err(DynamicPluginError::InitError("Plugin creation returned null".to_string()));
            }

            let plugin = Box::from_raw(plugin_ptr);

            let loaded = LoadedPlugin {
                plugin,
                _library: library,
                path: path.to_path_buf(),
            };

            self.plugins.push(loaded);
            Ok(self.plugins.last().unwrap())
        }
    }

    /// Discover and load all plugins from search paths
    pub fn discover_plugins(&mut self) -> Vec<DynamicPluginResult<PathBuf>> {
        let mut results = Vec::new();

        for search_path in &self.search_paths.clone() {
            if !search_path.exists() {
                continue;
            }

            if let Ok(entries) = std::fs::read_dir(search_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if Self::is_plugin_file(&path) {
                        match self.load_plugin(&path) {
                            Ok(_) => results.push(Ok(path)),
                            Err(e) => results.push(Err(e)),
                        }
                    }
                }
            }
        }

        results
    }

    /// Check if a file is a potential plugin based on extension
    fn is_plugin_file(path: &Path) -> bool {
        let extension = path.extension().and_then(OsStr::to_str);

        match extension {
            #[cfg(target_os = "linux")]
            Some("so") => true,
            #[cfg(target_os = "macos")]
            Some("dylib") => true,
            #[cfg(target_os = "windows")]
            Some("dll") => true,
            _ => false,
        }
    }

    /// Get all loaded plugins
    pub fn plugins(&self) -> &[LoadedPlugin] {
        &self.plugins
    }

    /// Get plugins that support a specific language
    pub fn plugins_for_language(&self, language: &str) -> Vec<&LoadedPlugin> {
        self.plugins
            .iter()
            .filter(|p| p.plugin.supports_language(language))
            .collect()
    }

    /// Get the best plugin for a language (highest priority)
    pub fn best_plugin_for_language(&self, language: &str) -> Option<&LoadedPlugin> {
        self.plugins_for_language(language)
            .into_iter()
            .max_by_key(|p| p.plugin.priority())
    }

    /// Analyze a file using the best available plugin
    pub fn analyze(
        &self,
        path: &Path,
        source: &str,
        language: &str,
    ) -> Option<FileMetrics> {
        if let Some(plugin) = self.best_plugin_for_language(language) {
            plugin.plugin.analyze(path, source, language)
        } else {
            None
        }
    }

    /// Get number of loaded plugins
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Unload all plugins
    pub fn unload_all(&mut self) {
        for plugin in &self.plugins {
            plugin.plugin.on_unload();
        }
        self.plugins.clear();
    }
}

impl Drop for DynamicPluginManager {
    fn drop(&mut self) {
        self.unload_all();
    }
}

/// Macro for declaring a dynamic plugin
///
/// This macro generates the necessary FFI exports for a plugin.
///
/// # Example
///
/// ```rust,ignore
/// use lucent::declare_plugin;
///
/// struct MyPlugin;
/// // ... implement DynamicPlugin ...
///
/// declare_plugin!(MyPlugin);
/// ```
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty) => {
        #[no_mangle]
        pub extern "C" fn lucent_plugin_api_version() -> u32 {
            $crate::plugin::DYNAMIC_PLUGIN_API_VERSION
        }

        #[no_mangle]
        pub extern "C" fn lucent_plugin_create() -> *mut dyn $crate::plugin::DynamicPlugin {
            let plugin = <$plugin_type>::default();
            let boxed: Box<dyn $crate::plugin::DynamicPlugin> = Box::new(plugin);
            Box::into_raw(boxed)
        }

        #[no_mangle]
        pub extern "C" fn lucent_plugin_destroy(plugin: *mut dyn $crate::plugin::DynamicPlugin) {
            if !plugin.is_null() {
                unsafe {
                    let _ = Box::from_raw(plugin);
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager_creation() {
        let manager = DynamicPluginManager::new();
        assert_eq!(manager.plugin_count(), 0);
        assert!(!manager.search_paths.is_empty());
    }

    #[test]
    fn test_add_search_path() {
        let mut manager = DynamicPluginManager::new();
        let initial_count = manager.search_paths.len();
        manager.add_search_path("/custom/path");
        assert_eq!(manager.search_paths.len(), initial_count + 1);
    }

    #[test]
    fn test_is_plugin_file() {
        #[cfg(target_os = "linux")]
        {
            assert!(DynamicPluginManager::is_plugin_file(Path::new("plugin.so")));
            assert!(!DynamicPluginManager::is_plugin_file(Path::new("plugin.dll")));
        }

        #[cfg(target_os = "macos")]
        {
            assert!(DynamicPluginManager::is_plugin_file(Path::new("plugin.dylib")));
            assert!(!DynamicPluginManager::is_plugin_file(Path::new("plugin.dll")));
        }

        #[cfg(target_os = "windows")]
        {
            assert!(DynamicPluginManager::is_plugin_file(Path::new("plugin.dll")));
            assert!(!DynamicPluginManager::is_plugin_file(Path::new("plugin.so")));
        }
    }

    #[test]
    fn test_load_nonexistent_plugin() {
        let mut manager = DynamicPluginManager::new();
        let result = manager.load_plugin("/nonexistent/plugin.so");
        assert!(matches!(result, Err(DynamicPluginError::NotFound(_))));
    }
}
