//! # Lucent
//!
//! Crystal clear code insights.
//!
//! Lucent is a fast, extensible code metrics analyzer that helps you understand
//! code quality, complexity, and maintainability across multiple programming languages.

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Core metrics types and calculations
pub mod metrics {
    //! Code metrics calculations
}

/// Language detection and analysis
pub mod language {
    //! Language-specific analyzers
}

/// File system traversal and filtering
pub mod scanner {
    //! File discovery and filtering
}

/// Output formatters
pub mod output {
    //! Various output formats (JSON, Markdown, etc.)
}

/// Configuration handling
pub mod config {
    //! Configuration file parsing and defaults
}

// Re-exports for convenience
pub use metrics::*;
pub use language::*;
pub use scanner::*;
pub use output::*;
pub use config::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Placeholder test
        assert_eq!(2 + 2, 4);
    }
}
