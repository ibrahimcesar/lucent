//! Plugin system for extending Lucent's capabilities
//!
//! This module provides a plugin API that allows extending Lucent with:
//! - Custom metric providers (e.g., tree-sitter based analysis)
//! - Language-specific analyzers
//! - Custom output formatters
//! - Additional code quality checks
//!
//! # Static vs Dynamic Plugins
//!
//! ## Static Plugins
//!
//! Static plugins implement `MetricProvider` and are compiled into your application.
//! Use these when you control the build process.
//!
//! ## Dynamic Plugins
//!
//! Dynamic plugins are loaded at runtime from shared libraries (`.so`/`.dylib`/`.dll`).
//! They implement `DynamicPlugin` and are discovered from plugin directories.
//!
//! See the `dynamic` module documentation for how to create dynamic plugins.

mod api;
mod registry;
mod builtin;
pub mod dynamic;

pub use api::{MetricProvider, PluginMetadata, PluginCapability};
pub use registry::{PluginRegistry, PluginError};
pub use builtin::RegexAnalyzer;
pub use dynamic::{
    DynamicPlugin, DynamicPluginManager, DynamicPluginError, DynamicPluginResult,
    LoadedPlugin, DYNAMIC_PLUGIN_API_VERSION,
};

/// Plugin system version for compatibility checking
pub const PLUGIN_API_VERSION: &str = "0.1.0";
