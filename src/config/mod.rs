//! Configuration and CLI argument parsing
//!
//! This module handles command-line argument parsing using clap
//! and configuration file loading.

mod settings;

pub use settings::*;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::output::OutputFormat as OutputFormatEnum;

/// 💎 Lucent - Crystal clear code insights
///
/// A fast code metrics analyzer that provides insights into code quality,
/// complexity, and maintainability across multiple programming languages.
#[derive(Parser, Debug)]
#[command(name = "lucent")]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Path to analyze (file or directory)
    #[arg(default_value = ".", global = true)]
    pub path: PathBuf,

    /// Output format
    #[arg(short, long, value_enum, default_value = "pretty", global = true)]
    pub format: OutputFormat,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Show per-file metrics (not just aggregated)
    #[arg(long, global = true)]
    pub per_file: bool,

    /// Maximum average cyclomatic complexity allowed
    #[arg(long, global = true)]
    pub max_complexity: Option<f64>,

    /// Maximum average nesting depth allowed
    #[arg(long, global = true)]
    pub max_nesting: Option<f64>,

    /// Minimum average maintainability index required
    #[arg(long, global = true)]
    pub min_maintainability: Option<f64>,

    /// Maximum average function length allowed (lines)
    #[arg(long, global = true)]
    pub max_function_length: Option<f64>,

    /// Maximum number of code smells allowed
    #[arg(long, global = true)]
    pub max_code_smells: Option<usize>,

    /// Maximum number of TODOs allowed
    #[arg(long, global = true)]
    pub max_todos: Option<usize>,

    /// Fail on warnings (treat warnings as errors)
    #[arg(long, global = true)]
    pub fail_on_warning: bool,

    /// Path to configuration file (defaults to .lucent.toml)
    #[arg(short = 'c', long, global = true)]
    pub config_file: Option<PathBuf>,
}

/// Subcommands for lucent
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Snapshot management commands
    Snapshot {
        #[command(subcommand)]
        command: SnapshotCommands,
    },

    /// Git integration commands
    Git {
        #[command(subcommand)]
        command: GitCommands,
    },

    /// Initialize project configuration and hooks
    Init {
        /// Install git pre-commit hooks
        #[arg(long)]
        hooks: bool,

        /// Create configuration file (.lucent.toml)
        #[arg(long)]
        config: bool,
    },

    /// Generate quality badges for README
    Badge {
        /// Badge output format (url, markdown, svg)
        #[arg(short = 't', long = "type", value_enum, default_value = "markdown")]
        badge_format: BadgeFormat,

        /// Output file path (for SVG format)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Launch interactive TUI (Terminal User Interface) mode
    ///
    /// An interactive terminal-based interface for exploring code metrics
    /// with real-time navigation, sorting, filtering, and drill-down views.
    #[command(alias = "ui")]
    Tui {
        /// Enable real-time file watching
        #[arg(short, long)]
        watch: bool,
    },
}

/// Badge format options
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum BadgeFormat {
    /// Raw shields.io URL
    Url,
    /// Markdown badge syntax
    Markdown,
    /// Generate SVG file
    Svg,
}

/// Snapshot subcommands
#[derive(Subcommand, Debug)]
pub enum SnapshotCommands {
    /// Save a snapshot of current metrics
    Save {
        /// Optional description for the snapshot
        #[arg(short, long)]
        description: Option<String>,
    },

    /// List all saved snapshots
    List {
        /// Number of snapshots to show (default: all)
        #[arg(short, long)]
        limit: Option<usize>,
    },

    /// Compare two snapshots
    Compare {
        /// Base snapshot ID (older)
        base: String,

        /// Current snapshot ID (newer)
        current: String,
    },

    /// Delete a snapshot
    Delete {
        /// Snapshot ID to delete
        id: String,
    },

    /// Clean up old snapshots
    Cleanup {
        /// Number of snapshots to keep
        #[arg(short, long, default_value = "10")]
        keep: usize,
    },
}

/// Git integration subcommands
#[derive(Subcommand, Debug)]
pub enum GitCommands {
    /// Analyze only changed files (diff mode)
    Diff {
        /// Reference to compare against (e.g., main, HEAD~1, commit hash)
        #[arg(short, long)]
        base: Option<String>,

        /// Include uncommitted changes
        #[arg(short, long)]
        uncommitted: bool,
    },

    /// Analyze commit history
    History {
        /// Number of commits to analyze
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Skip N commits between analyses (for performance)
        #[arg(short, long, default_value = "0")]
        skip: usize,
    },

    /// Show trend analysis
    Trends {
        /// Number of commits to analyze for trends
        #[arg(short, long, default_value = "20")]
        limit: usize,

        /// Metric to visualize (complexity, maintainability, lines)
        #[arg(short, long, default_value = "complexity")]
        metric: String,

        /// Chart height in lines
        #[arg(long, default_value = "10")]
        height: usize,
    },

    /// Show blame analysis (complexity attribution)
    Blame {
        /// Specific file to analyze (default: analyze all)
        file: Option<PathBuf>,

        /// Show top N contributors
        #[arg(short = 'n', long, default_value = "10")]
        top: usize,
    },
}

/// Output format options for CLI
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    /// Beautiful terminal output with colors and emojis
    Pretty,
    /// JSON format for programmatic consumption
    Json,
    /// Markdown format for reports and documentation
    Markdown,
    /// CSV format for data analysis
    Csv,
    /// Interactive HTML report with charts
    Html,
}

impl From<OutputFormat> for OutputFormatEnum {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Pretty => OutputFormatEnum::Pretty,
            OutputFormat::Json => OutputFormatEnum::Json,
            OutputFormat::Markdown => OutputFormatEnum::Markdown,
            OutputFormat::Csv => OutputFormatEnum::Csv,
            OutputFormat::Html => OutputFormatEnum::Html,
        }
    }
}

impl Config {
    /// Parse configuration from command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if !self.path.exists() {
            return Err(format!("Path does not exist: {}", self.path.display()));
        }

        Ok(())
    }

    /// Create a ThresholdConfig from CLI flags merged with file config
    ///
    /// Priority order:
    /// 1. CLI flags (highest priority)
    /// 2. Custom config file (--config-file)
    /// 3. .lucent.toml file (auto-discovered)
    /// 4. No thresholds (default)
    pub fn threshold_config(&self) -> crate::threshold::ThresholdConfig {
        // Try to load configuration
        let file_config = if let Some(config_path) = &self.config_file {
            // Load from specified file
            LucentConfig::from_file(config_path).ok()
        } else {
            // Auto-discover .lucent.toml
            LucentConfig::load().ok()
        };

        // Build per-language thresholds from file config
        let per_language = file_config
            .as_ref()
            .map(|c| {
                c.languages
                    .iter()
                    .map(|(lang, override_config)| {
                        (
                            lang.clone(),
                            crate::threshold::LanguageThresholds {
                                max_complexity: override_config.max_cyclomatic_complexity,
                                max_nesting: override_config.max_nesting_depth,
                                min_maintainability: override_config.min_maintainability_index,
                                max_function_length: override_config.max_function_length.map(|v| v as f64),
                            },
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();

        // CLI flags override file config
        crate::threshold::ThresholdConfig {
            max_complexity: self.max_complexity.or_else(|| {
                file_config.as_ref().map(|c| c.thresholds.max_cyclomatic_complexity)
            }),
            max_nesting: self.max_nesting.or_else(|| {
                file_config.as_ref().map(|c| c.thresholds.max_nesting_depth)
            }),
            min_maintainability: self.min_maintainability.or_else(|| {
                file_config.as_ref().map(|c| c.thresholds.min_maintainability_index)
            }),
            max_function_length: self.max_function_length.or_else(|| {
                file_config.as_ref().map(|c| c.thresholds.max_function_length as f64)
            }),
            max_code_smells: self.max_code_smells.or_else(|| {
                file_config.as_ref().map(|c| {
                    c.thresholds.max_long_functions
                        + c.thresholds.max_deep_nesting_instances
                        + c.thresholds.max_magic_numbers
                })
            }),
            max_todos: self.max_todos,
            // Only use config file fail_on_warning if CLI flag is not set
            // CLI flag is always false by default, so we need to check explicitly
            fail_on_warning: if self.fail_on_warning {
                true
            } else {
                file_config.as_ref()
                    .map(|c| !c.quality_gates.warn_only)
                    .unwrap_or(false)
            },
            per_language,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::parse_from(&["lucent"]);
        assert_eq!(config.path, PathBuf::from("."));
        assert!(!config.verbose);
    }

    #[test]
    fn test_custom_path() {
        let config = Config::parse_from(&["lucent", "src/"]);
        assert_eq!(config.path, PathBuf::from("src/"));
    }

    #[test]
    fn test_json_format() {
        let config = Config::parse_from(&["lucent", "--format", "json"]);
        assert!(matches!(config.format, OutputFormat::Json));
    }

    #[test]
    fn test_verbose_flag() {
        let config = Config::parse_from(&["lucent", "--verbose"]);
        assert!(config.verbose);
    }
}
