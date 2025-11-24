//! Configuration file settings for Lucent
//!
//! This module handles loading and parsing of `.lucent.toml` configuration files.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use anyhow::{Context, Result};

/// Main configuration structure for Lucent
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct LucentConfig {
    /// Project metadata
    pub project: ProjectConfig,

    /// Quality thresholds
    pub thresholds: ThresholdConfig,

    /// Quality gate settings
    pub quality_gates: QualityGateConfig,

    /// Ignore patterns
    pub ignore: IgnoreConfig,

    /// Language-specific overrides
    #[serde(default)]
    pub languages: HashMap<String, LanguageOverride>,

    /// Output configuration
    pub output: OutputConfig,

    /// Plugin configuration
    #[serde(default)]
    pub plugins: PluginConfig,

    /// Custom rules configuration
    #[serde(default)]
    pub rules_config: RulesGlobalConfig,

    /// Custom rules definitions
    #[serde(default)]
    pub rules: Vec<RuleConfig>,
}

/// Plugin configuration
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PluginConfig {
    /// List of enabled plugin names
    #[serde(default)]
    pub enabled: Vec<String>,

    /// Whether to use plugins for analysis (vs built-in regex)
    #[serde(default = "default_true")]
    pub use_for_analysis: bool,

    /// Fallback to built-in analyzer if plugin fails
    #[serde(default = "default_true")]
    pub fallback_on_error: bool,
}

fn default_true() -> bool {
    true
}

/// Global rules configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RulesGlobalConfig {
    /// Enable custom rules checking
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Exit with code 1 on error-level violations
    #[serde(default = "default_true")]
    pub fail_on_error: bool,

    /// Exit with code 1 on warning-level violations
    #[serde(default)]
    pub fail_on_warning: bool,

    /// Maximum number of violations to report (0 = unlimited)
    #[serde(default)]
    pub max_violations: usize,

    /// Run rules in parallel
    #[serde(default = "default_true")]
    pub parallel: bool,
}

impl Default for RulesGlobalConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            fail_on_error: true,
            fail_on_warning: false,
            max_violations: 0,
            parallel: true,
        }
    }
}

/// Individual rule configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuleConfig {
    /// Rule unique identifier
    pub name: String,

    /// Rule description
    pub description: String,

    /// Rule type: "pattern", "ast", "metric", or "plugin"
    #[serde(rename = "type")]
    pub rule_type: String,

    /// Severity: "error", "warning", or "info"
    pub severity: String,

    // Pattern rule fields
    /// Regex pattern (for pattern rules)
    #[serde(default)]
    pub pattern: Option<String>,

    // File filtering
    /// File patterns to include (glob-style)
    #[serde(default)]
    pub file_patterns: Vec<String>,

    /// File patterns to exclude (glob-style)
    #[serde(default)]
    pub exclude_patterns: Vec<String>,

    /// Target language filter
    #[serde(default)]
    pub language: Option<String>,

    // Optional fields
    /// Suggestion text for fixing violation
    #[serde(default)]
    pub suggestion: Option<String>,

    /// Enable auto-fix (future feature)
    #[serde(default)]
    pub autofix: bool,

    /// Per-file vs project-wide application
    #[serde(default = "default_true")]
    pub per_file: bool,

    // Metric rule fields
    /// Metric name (for metric rules)
    #[serde(default)]
    pub metric: Option<String>,

    /// Maximum value threshold
    #[serde(default)]
    pub max_value: Option<f64>,

    /// Minimum value threshold
    #[serde(default)]
    pub min_value: Option<f64>,

    // AST rule fields
    /// AST check type (for ast rules)
    #[serde(default)]
    pub check: Option<String>,

    /// Maximum match arms (for ast rules)
    #[serde(default)]
    pub max_match_arms: Option<usize>,

    /// Maximum function parameters (for ast rules)
    #[serde(default)]
    pub max_params: Option<usize>,

    /// Maximum nesting depth (for ast rules)
    #[serde(default)]
    pub max_nesting_depth: Option<usize>,
}

impl RuleConfig {
    /// Validate the rule configuration
    pub fn validate(&self) -> anyhow::Result<()> {
        // Validate rule type
        match self.rule_type.as_str() {
            "pattern" | "ast" | "metric" | "plugin" => {},
            _ => anyhow::bail!("Invalid rule type: {}. Must be 'pattern', 'ast', 'metric', or 'plugin'", self.rule_type),
        }

        // Validate severity
        match self.severity.as_str() {
            "error" | "warning" | "info" => {},
            _ => anyhow::bail!("Invalid severity: {}. Must be 'error', 'warning', or 'info'", self.severity),
        }

        // Type-specific validation
        match self.rule_type.as_str() {
            "pattern" => {
                if self.pattern.is_none() {
                    anyhow::bail!("Pattern rules must have a 'pattern' field");
                }
            },
            "metric" => {
                if self.metric.is_none() {
                    anyhow::bail!("Metric rules must have a 'metric' field");
                }
                if self.max_value.is_none() && self.min_value.is_none() {
                    anyhow::bail!("Metric rules must have either 'max_value' or 'min_value'");
                }
            },
            "ast" => {
                if self.check.is_none() && self.max_match_arms.is_none() && self.max_params.is_none() && self.max_nesting_depth.is_none() {
                    anyhow::bail!("AST rules must have at least one check parameter");
                }
            },
            _ => {}
        }

        Ok(())
    }
}

/// Project metadata configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectConfig {
    /// Project name
    #[serde(default)]
    pub name: Option<String>,

    /// Project version
    #[serde(default)]
    pub version: Option<String>,
}

/// Threshold configuration for metrics
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThresholdConfig {
    /// Maximum allowed cyclomatic complexity
    pub max_cyclomatic_complexity: f64,

    /// Maximum allowed cognitive complexity
    pub max_cognitive_complexity: f64,

    /// Maximum allowed nesting depth
    pub max_nesting_depth: f64,

    /// Maximum allowed function length (lines)
    pub max_function_length: usize,

    /// Minimum required comment quality (0.0-1.0)
    pub min_comment_quality: f64,

    /// Minimum required maintainability index (0-100)
    pub min_maintainability_index: f64,

    /// Maximum allowed code density (0.0-1.0)
    pub max_code_density: f64,

    /// Minimum required code density (0.0-1.0)
    pub min_code_density: f64,

    /// Maximum allowed long functions
    pub max_long_functions: usize,

    /// Maximum function parameters
    pub max_function_parameters: usize,

    /// Maximum allowed deep nesting instances
    pub max_deep_nesting_instances: usize,

    /// Maximum allowed magic numbers
    pub max_magic_numbers: usize,
}

/// Quality gate configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QualityGateConfig {
    /// Fail build if complexity threshold exceeded
    pub fail_on_complexity: bool,

    /// Fail build if nesting threshold exceeded
    pub fail_on_nesting: bool,

    /// Fail build if maintainability threshold not met
    pub fail_on_maintainability: bool,

    /// Fail build if code smells threshold exceeded
    pub fail_on_code_smells: bool,

    /// Only warn, don't fail
    pub warn_only: bool,
}

/// Ignore patterns configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IgnoreConfig {
    /// Patterns to ignore (glob-style)
    #[serde(default)]
    pub patterns: Vec<String>,

    /// Use .lucentignore file
    pub use_lucentignore: bool,

    /// Use .gitignore file
    pub use_gitignore: bool,
}

/// Language-specific override configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LanguageOverride {
    /// Override max cyclomatic complexity for this language
    pub max_cyclomatic_complexity: Option<f64>,

    /// Override max cognitive complexity for this language
    pub max_cognitive_complexity: Option<f64>,

    /// Override max nesting depth for this language
    pub max_nesting_depth: Option<f64>,

    /// Override min maintainability index for this language
    pub min_maintainability_index: Option<f64>,

    /// Override max function length for this language
    pub max_function_length: Option<usize>,

    /// Additional file extensions for this language
    #[serde(default)]
    pub extensions: Vec<String>,
}

/// Output configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputConfig {
    /// Default output format
    pub default_format: String,

    /// Show per-file metrics by default
    pub show_per_file: bool,

    /// Use colors in output
    pub colorize: bool,

    /// Include charts in HTML reports
    pub charts_in_html: bool,
}

// Default implementations

impl Default for LucentConfig {
    fn default() -> Self {
        Self {
            project: ProjectConfig::default(),
            thresholds: ThresholdConfig::default(),
            quality_gates: QualityGateConfig::default(),
            ignore: IgnoreConfig::default(),
            languages: HashMap::new(),
            output: OutputConfig::default(),
            plugins: PluginConfig::default(),
            rules_config: RulesGlobalConfig::default(),
            rules: Vec::new(),
        }
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: None,
            version: None,
        }
    }
}

impl Default for ThresholdConfig {
    fn default() -> Self {
        Self {
            max_cyclomatic_complexity: 20.0,
            max_cognitive_complexity: 30.0,
            max_nesting_depth: 5.0,
            max_function_length: 50,
            min_comment_quality: 0.3,
            min_maintainability_index: 65.0,
            max_code_density: 0.8,
            min_code_density: 0.5,
            max_long_functions: 10,
            max_function_parameters: 5,
            max_deep_nesting_instances: 5,
            max_magic_numbers: 20,
        }
    }
}

impl Default for QualityGateConfig {
    fn default() -> Self {
        Self {
            fail_on_complexity: false,
            fail_on_nesting: false,
            fail_on_maintainability: false,
            fail_on_code_smells: false,
            warn_only: true,  // Default to warnings only, not failing builds
        }
    }
}

impl Default for IgnoreConfig {
    fn default() -> Self {
        Self {
            patterns: vec![
                "target/".to_string(),
                "node_modules/".to_string(),
                "vendor/".to_string(),
                "dist/".to_string(),
                "build/".to_string(),
            ],
            use_lucentignore: true,
            use_gitignore: true,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            default_format: "pretty".to_string(),
            show_per_file: false,
            colorize: true,
            charts_in_html: true,
        }
    }
}

// Implementation methods

impl LucentConfig {
    /// Load configuration with search hierarchy
    ///
    /// Searches for `.lucent.toml` in:
    /// 1. Current directory
    /// 2. Parent directories (up to root)
    /// 3. User config (~/.config/lucent/config.toml)
    /// 4. Falls back to defaults
    pub fn load() -> Result<Self> {
        // Try to find project config
        if let Some(config) = Self::find_project_config()? {
            return Ok(config);
        }

        // Try user config
        if let Some(config) = Self::load_user_config()? {
            return Ok(config);
        }

        // Use defaults
        Ok(Self::default())
    }

    /// Find and load project-level config (.lucent.toml)
    fn find_project_config() -> Result<Option<Self>> {
        let mut current = std::env::current_dir()
            .context("Failed to get current directory")?;

        loop {
            let config_path = current.join(".lucent.toml");
            if config_path.exists() {
                let config = Self::from_file(&config_path)?;
                return Ok(Some(config));
            }

            // Move to parent directory
            if !current.pop() {
                break;
            }
        }

        Ok(None)
    }

    /// Load user-level config (~/.config/lucent/config.toml)
    fn load_user_config() -> Result<Option<Self>> {
        if let Some(home) = dirs::home_dir() {
            let config_path = home.join(".config/lucent/config.toml");
            if config_path.exists() {
                let config = Self::from_file(&config_path)?;
                return Ok(Some(config));
            }
        }

        Ok(None)
    }

    /// Load configuration from a specific file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Self = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        Ok(config)
    }

    /// Save configuration to a file
    pub fn to_file(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize configuration")?;

        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
        }

        std::fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Get threshold for a specific language (with override support)
    pub fn get_max_complexity(&self, language: &str) -> f64 {
        self.languages
            .get(language)
            .and_then(|o| o.max_cyclomatic_complexity)
            .unwrap_or(self.thresholds.max_cyclomatic_complexity)
    }

    /// Get max nesting depth for a specific language
    pub fn get_max_nesting(&self, language: &str) -> f64 {
        self.languages
            .get(language)
            .and_then(|o| o.max_nesting_depth)
            .unwrap_or(self.thresholds.max_nesting_depth)
    }

    /// Get max function length for a specific language
    pub fn get_max_function_length(&self, language: &str) -> usize {
        self.languages
            .get(language)
            .and_then(|o| o.max_function_length)
            .unwrap_or(self.thresholds.max_function_length)
    }

    /// Load .lucentignore patterns
    pub fn load_lucentignore() -> Result<Vec<String>> {
        let ignore_path = Path::new(".lucentignore");
        if !ignore_path.exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(ignore_path)
            .context("Failed to read .lucentignore")?;

        let patterns: Vec<String> = content
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|line| line.trim().to_string())
            .collect();

        Ok(patterns)
    }

    /// Get all ignore patterns (config + .lucentignore)
    pub fn get_all_ignore_patterns(&self) -> Result<Vec<String>> {
        let mut patterns = self.ignore.patterns.clone();

        if self.ignore.use_lucentignore {
            let lucentignore_patterns = Self::load_lucentignore()?;
            patterns.extend(lucentignore_patterns);
        }

        Ok(patterns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LucentConfig::default();
        assert_eq!(config.thresholds.max_cyclomatic_complexity, 20.0);
        assert_eq!(config.thresholds.max_nesting_depth, 5.0);
        assert_eq!(config.thresholds.min_maintainability_index, 65.0);
    }

    #[test]
    fn test_toml_serialization() {
        let config = LucentConfig::default();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("max_cyclomatic_complexity"));
    }

    #[test]
    fn test_language_override() {
        let mut config = LucentConfig::default();

        let rust_override = LanguageOverride {
            max_cyclomatic_complexity: Some(15.0),
            max_cognitive_complexity: None,
            max_nesting_depth: None,
            min_maintainability_index: None,
            max_function_length: None,
            extensions: vec![],
        };

        config.languages.insert("Rust".to_string(), rust_override);

        assert_eq!(config.get_max_complexity("Rust"), 15.0);
        assert_eq!(config.get_max_complexity("Python"), 20.0); // Default
    }
}
