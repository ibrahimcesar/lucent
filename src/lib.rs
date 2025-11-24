//! # 💎 Lucent
//!
//! Crystal clear code insights.
//!
//! Lucent is a fast, extensible code metrics analyzer that helps you understand
//! code quality, complexity, and maintainability across multiple programming languages.
//!
//! ## Features
//!
//! - 🎨 Beautiful terminal output with colors and emojis
//! - 📊 Rich metrics: line counts, entropy, and complexity
//! - 🌍 Multi-language support (13+ languages)
//! - 🚀 Fast Rust-powered analysis
//! - 📤 JSON export for CI/CD integration
//!
//! ## Example
//!
//! ```no_run
//! use lucent::{Scanner, FileMetrics, ProjectMetrics, display_metrics, OutputFormat};
//!
//! # fn main() -> anyhow::Result<()> {
//! // Scan a directory
//! let scanner = Scanner::new(".")?;
//! let files = scanner.scan()?;
//!
//! // Analyze metrics
//! let mut project_metrics = ProjectMetrics::new();
//! for file in files {
//!     let metrics = FileMetrics::analyze(&file.path, file.language)?;
//!     project_metrics.add_file(file.language.name, &metrics);
//! }
//!
//! // Display results
//! display_metrics(&project_metrics, OutputFormat::Pretty);
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Core metrics types and calculations
pub mod metrics;

/// Language detection and analysis
pub mod language;

/// File system traversal and filtering
pub mod scanner;

/// Git integration for version control analysis
pub mod git;

/// Output formatters
pub mod output;

/// Configuration handling
pub mod config;

/// Snapshot and history tracking
pub mod snapshot;

/// Threshold checking and quality gates
pub mod threshold;

/// Plugin system for extensibility
pub mod plugin;

// Re-exports for convenience
pub use config::Config;
pub use language::{Language, LANGUAGES};
pub use metrics::{FileMetrics, ProjectMetrics, LanguageMetrics};
pub use output::{display_metrics, OutputFormat};
pub use scanner::{Scanner, SourceFile};
pub use snapshot::{Snapshot, SnapshotManager, SnapshotComparison};
pub use threshold::{ThresholdConfig, ValidationResult, ThresholdViolation, Severity};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_workflow() -> anyhow::Result<()> {
        // This is a basic integration test showing the workflow
        let scanner = Scanner::new(".")?;
        let files = scanner.scan()?;

        let mut project_metrics = ProjectMetrics::new();
        for file in files.iter().take(1) {
            // Just test one file
            let metrics = FileMetrics::analyze(&file.path, file.language)?;
            project_metrics.add_file(file.language.name, &metrics);
        }

        assert!(project_metrics.total_files > 0 || files.is_empty());
        Ok(())
    }
}
