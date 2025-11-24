//! Plugin system for extending Lucent's capabilities
//!
//! This module provides a plugin API that allows extending Lucent with:
//! - Custom metric providers (e.g., tree-sitter based analysis)
//! - Language-specific analyzers
//! - Custom output formatters
//! - Additional code quality checks

mod api;
mod registry;
mod builtin;

pub use api::{MetricProvider, PluginMetadata, PluginCapability};
pub use registry::{PluginRegistry, PluginError};
pub use builtin::RegexAnalyzer;

/// Plugin system version for compatibility checking
pub const PLUGIN_API_VERSION: &str = "0.1.0";
