//! Threshold checking and quality gate enforcement
//!
//! This module provides threshold checking capabilities for enforcing
//! code quality standards in CI/CD pipelines.

use crate::metrics::ProjectMetrics;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Threshold configuration for quality gates
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThresholdConfig {
    /// Maximum average cyclomatic complexity allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_complexity: Option<f64>,

    /// Maximum average nesting depth allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nesting: Option<f64>,

    /// Minimum average maintainability index required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_maintainability: Option<f64>,

    /// Maximum average function length allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_function_length: Option<f64>,

    /// Maximum number of code smells allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_code_smells: Option<usize>,

    /// Maximum number of TODOs allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_todos: Option<usize>,

    /// Whether to fail on any warning
    pub fail_on_warning: bool,

    /// Per-language threshold overrides
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub per_language: HashMap<String, LanguageThresholds>,
}

/// Language-specific threshold overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageThresholds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_complexity: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nesting: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_maintainability: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_function_length: Option<f64>,
}

/// Result of threshold validation
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    /// All thresholds passed
    Passed,

    /// Some thresholds failed (with violations)
    Failed(Vec<ThresholdViolation>),

    /// Warnings detected (non-critical)
    Warning(Vec<ThresholdViolation>),
}

impl ValidationResult {
    /// Check if validation passed
    pub fn is_pass(&self) -> bool {
        matches!(self, ValidationResult::Passed)
    }

    /// Check if validation failed
    pub fn is_fail(&self) -> bool {
        matches!(self, ValidationResult::Failed(_))
    }

    /// Check if there are warnings
    pub fn is_warning(&self) -> bool {
        matches!(self, ValidationResult::Warning(_))
    }

    /// Get all violations
    pub fn violations(&self) -> Vec<ThresholdViolation> {
        match self {
            ValidationResult::Failed(v) | ValidationResult::Warning(v) => v.clone(),
            ValidationResult::Passed => vec![],
        }
    }
}

/// A threshold violation
#[derive(Debug, Clone, PartialEq)]
pub struct ThresholdViolation {
    /// Metric that was violated
    pub metric: String,

    /// Expected value (threshold)
    pub expected: String,

    /// Actual value found
    pub actual: String,

    /// Severity level
    pub severity: Severity,
}

/// Severity of a threshold violation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Critical violation (should fail build)
    Error,

    /// Non-critical violation (warning only)
    Warning,
}

impl ThresholdConfig {
    /// Create a new empty threshold configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any thresholds are configured
    pub fn has_thresholds(&self) -> bool {
        self.max_complexity.is_some()
            || self.max_nesting.is_some()
            || self.min_maintainability.is_some()
            || self.max_function_length.is_some()
            || self.max_code_smells.is_some()
            || self.max_todos.is_some()
    }

    /// Validate project metrics against configured thresholds
    pub fn validate(&self, metrics: &ProjectMetrics) -> ValidationResult {
        if !self.has_thresholds() {
            return ValidationResult::Passed;
        }

        let mut violations = Vec::new();

        if metrics.by_language.is_empty() {
            return ValidationResult::Passed;
        }

        // Check each language separately with per-language thresholds
        for (language, lang_metrics) in &metrics.by_language {
            // Get thresholds for this language (per-language overrides or global)
            let max_complexity = self.per_language
                .get(language)
                .and_then(|t| t.max_complexity)
                .or(self.max_complexity);

            let max_nesting = self.per_language
                .get(language)
                .and_then(|t| t.max_nesting)
                .or(self.max_nesting);

            let min_maintainability = self.per_language
                .get(language)
                .and_then(|t| t.min_maintainability)
                .or(self.min_maintainability);

            let max_function_length = self.per_language
                .get(language)
                .and_then(|t| t.max_function_length)
                .or(self.max_function_length);

            // Check complexity threshold
            if let Some(max_complexity) = max_complexity {
                if lang_metrics.avg_cyclomatic > max_complexity {
                    violations.push(ThresholdViolation {
                        metric: format!("{} - Average Complexity", language),
                        expected: format!("<= {:.1}", max_complexity),
                        actual: format!("{:.1}", lang_metrics.avg_cyclomatic),
                        severity: Severity::Error,
                    });
                }
            }

            // Check nesting threshold
            if let Some(max_nesting) = max_nesting {
                if lang_metrics.avg_nesting_depth > max_nesting {
                    violations.push(ThresholdViolation {
                        metric: format!("{} - Average Nesting Depth", language),
                        expected: format!("<= {:.1}", max_nesting),
                        actual: format!("{:.1}", lang_metrics.avg_nesting_depth),
                        severity: Severity::Error,
                    });
                }
            }

            // Check maintainability threshold
            if let Some(min_maintainability) = min_maintainability {
                if lang_metrics.avg_maintainability_index < min_maintainability {
                    violations.push(ThresholdViolation {
                        metric: format!("{} - Average Maintainability Index", language),
                        expected: format!(">= {:.0}", min_maintainability),
                        actual: format!("{:.0}", lang_metrics.avg_maintainability_index),
                        severity: Severity::Error,
                    });
                }
            }

            // Check function length threshold
            if let Some(max_function_length) = max_function_length {
                if lang_metrics.avg_function_length > max_function_length {
                    violations.push(ThresholdViolation {
                        metric: format!("{} - Average Function Length", language),
                        expected: format!("<= {:.0} lines", max_function_length),
                        actual: format!("{:.0} lines", lang_metrics.avg_function_length),
                        severity: Severity::Warning,
                    });
                }
            }
        }

        // Check global code smells threshold (aggregated across all languages)
        if let Some(max_code_smells) = self.max_code_smells {
            let total_code_smells: usize = metrics
                .by_language
                .values()
                .map(|l| {
                    l.long_functions
                        + l.long_parameter_lists
                        + l.deep_nesting_instances
                        + l.magic_numbers
                })
                .sum();

            if total_code_smells > max_code_smells {
                violations.push(ThresholdViolation {
                    metric: "Total Code Smells".to_string(),
                    expected: format!("<= {}", max_code_smells),
                    actual: format!("{}", total_code_smells),
                    severity: Severity::Warning,
                });
            }
        }

        // Check global TODOs threshold (aggregated across all languages)
        if let Some(max_todos) = self.max_todos {
            let total_todos: usize = metrics
                .by_language
                .values()
                .map(|l| l.todo_count)
                .sum();

            if total_todos > max_todos {
                violations.push(ThresholdViolation {
                    metric: "Total TODOs".to_string(),
                    expected: format!("<= {}", max_todos),
                    actual: format!("{}", total_todos),
                    severity: Severity::Warning,
                });
            }
        }

        // Categorize violations by severity
        let errors: Vec<_> = violations
            .iter()
            .filter(|v| v.severity == Severity::Error)
            .cloned()
            .collect();

        let warnings: Vec<_> = violations
            .iter()
            .filter(|v| v.severity == Severity::Warning)
            .cloned()
            .collect();

        // Determine result
        if !errors.is_empty() {
            ValidationResult::Failed(errors)
        } else if !warnings.is_empty() {
            if self.fail_on_warning {
                ValidationResult::Failed(warnings)
            } else {
                ValidationResult::Warning(warnings)
            }
        } else {
            ValidationResult::Passed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::LanguageMetrics;
    use std::collections::HashMap;

    fn create_test_metrics(
        complexity: f64,
        nesting: f64,
        maintainability: f64,
        function_length: f64,
        code_smells: usize,
        todos: usize,
    ) -> ProjectMetrics {
        let mut by_language = HashMap::new();
        by_language.insert(
            "Rust".to_string(),
            LanguageMetrics {
                file_count: 10,
                total_lines: 1000,
                code_lines: 800,
                comment_lines: 100,
                blank_lines: 100,
                avg_entropy: 3.5,
                avg_cyclomatic: complexity,
                avg_cognitive: complexity * 1.5,
                avg_nesting_depth: nesting,
                total_functions: 40,
                avg_function_length: function_length,
                todo_count: todos,
                fixme_count: 0,
                hack_count: 0,
                avg_comment_quality: 0.5,
                low_quality_comments: 2,
                high_quality_comments: 8,
                long_functions: code_smells / 4,
                long_parameter_lists: code_smells / 4,
                deep_nesting_instances: code_smells / 4,
                magic_numbers: code_smells / 4,
                avg_maintainability_index: maintainability,
            },
        );

        ProjectMetrics {
            total_files: 10,
            by_language,
        }
    }

    #[test]
    fn test_no_thresholds() {
        let config = ThresholdConfig::new();
        let metrics = create_test_metrics(10.0, 3.0, 50.0, 30.0, 20, 10);

        let result = config.validate(&metrics);
        assert_eq!(result, ValidationResult::Passed);
        assert!(result.is_pass());
    }

    #[test]
    fn test_all_pass() {
        let mut config = ThresholdConfig::new();
        config.max_complexity = Some(15.0);
        config.max_nesting = Some(5.0);
        config.min_maintainability = Some(60.0);

        let metrics = create_test_metrics(10.0, 3.0, 70.0, 30.0, 20, 10);

        let result = config.validate(&metrics);
        assert_eq!(result, ValidationResult::Passed);
    }

    #[test]
    fn test_complexity_violation() {
        let mut config = ThresholdConfig::new();
        config.max_complexity = Some(5.0);

        let metrics = create_test_metrics(10.0, 3.0, 70.0, 30.0, 20, 10);

        let result = config.validate(&metrics);
        assert!(result.is_fail());

        let violations = result.violations();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].metric, "Rust - Average Complexity");
        assert_eq!(violations[0].severity, Severity::Error);
    }

    #[test]
    fn test_maintainability_violation() {
        let mut config = ThresholdConfig::new();
        config.min_maintainability = Some(80.0);

        let metrics = create_test_metrics(10.0, 3.0, 50.0, 30.0, 20, 10);

        let result = config.validate(&metrics);
        assert!(result.is_fail());

        let violations = result.violations();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].metric, "Rust - Average Maintainability Index");
    }

    #[test]
    fn test_multiple_violations() {
        let mut config = ThresholdConfig::new();
        config.max_complexity = Some(5.0);
        config.max_nesting = Some(2.0);
        config.min_maintainability = Some(80.0);

        let metrics = create_test_metrics(10.0, 3.0, 50.0, 30.0, 20, 10);

        let result = config.validate(&metrics);
        assert!(result.is_fail());

        let violations = result.violations();
        assert_eq!(violations.len(), 3);
    }

    #[test]
    fn test_warning_only() {
        let mut config = ThresholdConfig::new();
        config.max_function_length = Some(20.0);
        config.max_code_smells = Some(10);

        let metrics = create_test_metrics(5.0, 2.0, 80.0, 30.0, 20, 5);

        let result = config.validate(&metrics);
        assert!(result.is_warning());
        assert!(!result.is_fail());

        let violations = result.violations();
        assert_eq!(violations.len(), 2);
        assert!(violations.iter().all(|v| v.severity == Severity::Warning));
    }

    #[test]
    fn test_fail_on_warning() {
        let mut config = ThresholdConfig::new();
        config.max_function_length = Some(20.0);
        config.fail_on_warning = true;

        let metrics = create_test_metrics(5.0, 2.0, 80.0, 30.0, 10, 5);

        let result = config.validate(&metrics);
        assert!(result.is_fail());
    }

    #[test]
    fn test_has_thresholds() {
        let mut config = ThresholdConfig::new();
        assert!(!config.has_thresholds());

        config.max_complexity = Some(10.0);
        assert!(config.has_thresholds());
    }

    #[test]
    fn test_per_language_thresholds() {
        let mut config = ThresholdConfig::new();

        // Set global thresholds
        config.max_complexity = Some(15.0);
        config.max_nesting = Some(5.0);

        // Set stricter thresholds for Rust
        let mut per_language = HashMap::new();
        per_language.insert(
            "Rust".to_string(),
            LanguageThresholds {
                max_complexity: Some(10.0),  // Stricter than global
                max_nesting: Some(3.0),      // Stricter than global
                min_maintainability: None,
                max_function_length: None,
            },
        );
        config.per_language = per_language;

        // Create metrics with two languages
        let mut by_language = HashMap::new();

        // Rust violates its stricter threshold but not global
        by_language.insert(
            "Rust".to_string(),
            LanguageMetrics {
                file_count: 5,
                total_lines: 500,
                code_lines: 400,
                comment_lines: 50,
                blank_lines: 50,
                avg_entropy: 3.5,
                avg_cyclomatic: 12.0,  // Violates Rust threshold (10.0) but not global (15.0)
                avg_cognitive: 18.0,
                avg_nesting_depth: 2.0,
                total_functions: 20,
                avg_function_length: 25.0,
                todo_count: 5,
                fixme_count: 0,
                hack_count: 0,
                avg_comment_quality: 0.5,
                low_quality_comments: 1,
                high_quality_comments: 4,
                long_functions: 2,
                long_parameter_lists: 1,
                deep_nesting_instances: 1,
                magic_numbers: 3,
                avg_maintainability_index: 70.0,
            },
        );

        // Python passes global thresholds
        by_language.insert(
            "Python".to_string(),
            LanguageMetrics {
                file_count: 5,
                total_lines: 500,
                code_lines: 400,
                comment_lines: 50,
                blank_lines: 50,
                avg_entropy: 3.5,
                avg_cyclomatic: 8.0,  // Passes global threshold
                avg_cognitive: 12.0,
                avg_nesting_depth: 3.0,
                total_functions: 20,
                avg_function_length: 25.0,
                todo_count: 3,
                fixme_count: 0,
                hack_count: 0,
                avg_comment_quality: 0.5,
                low_quality_comments: 1,
                high_quality_comments: 4,
                long_functions: 1,
                long_parameter_lists: 0,
                deep_nesting_instances: 1,
                magic_numbers: 2,
                avg_maintainability_index: 75.0,
            },
        );

        let metrics = ProjectMetrics {
            total_files: 10,
            by_language,
        };

        let result = config.validate(&metrics);
        assert!(result.is_fail());

        let violations = result.violations();
        assert_eq!(violations.len(), 1);
        assert!(violations[0].metric.contains("Rust"));
        assert!(violations[0].metric.contains("Complexity"));
    }

    #[test]
    fn test_per_language_different_violations() {
        let mut config = ThresholdConfig::new();

        // Set global max complexity
        config.max_complexity = Some(10.0);

        // Rust gets stricter threshold
        let mut per_language = HashMap::new();
        per_language.insert(
            "Rust".to_string(),
            LanguageThresholds {
                max_complexity: Some(5.0),
                max_nesting: None,
                min_maintainability: None,
                max_function_length: None,
            },
        );
        config.per_language = per_language;

        // Create metrics
        let mut by_language = HashMap::new();

        // Rust at 8.0 - violates Rust threshold (5.0) but not global (10.0)
        by_language.insert(
            "Rust".to_string(),
            LanguageMetrics {
                file_count: 1,
                total_lines: 100,
                code_lines: 80,
                comment_lines: 10,
                blank_lines: 10,
                avg_entropy: 3.5,
                avg_cyclomatic: 8.0,
                avg_cognitive: 12.0,
                avg_nesting_depth: 2.0,
                total_functions: 10,
                avg_function_length: 20.0,
                todo_count: 0,
                fixme_count: 0,
                hack_count: 0,
                avg_comment_quality: 0.5,
                low_quality_comments: 0,
                high_quality_comments: 5,
                long_functions: 0,
                long_parameter_lists: 0,
                deep_nesting_instances: 0,
                magic_numbers: 0,
                avg_maintainability_index: 70.0,
            },
        );

        // JavaScript at 12.0 - violates global threshold (10.0)
        by_language.insert(
            "JavaScript".to_string(),
            LanguageMetrics {
                file_count: 1,
                total_lines: 100,
                code_lines: 80,
                comment_lines: 10,
                blank_lines: 10,
                avg_entropy: 3.5,
                avg_cyclomatic: 12.0,
                avg_cognitive: 18.0,
                avg_nesting_depth: 2.0,
                total_functions: 10,
                avg_function_length: 20.0,
                todo_count: 0,
                fixme_count: 0,
                hack_count: 0,
                avg_comment_quality: 0.5,
                low_quality_comments: 0,
                high_quality_comments: 5,
                long_functions: 0,
                long_parameter_lists: 0,
                deep_nesting_instances: 0,
                magic_numbers: 0,
                avg_maintainability_index: 70.0,
            },
        );

        let metrics = ProjectMetrics {
            total_files: 2,
            by_language,
        };

        let result = config.validate(&metrics);
        assert!(result.is_fail());

        let violations = result.violations();
        assert_eq!(violations.len(), 2);

        // Check that both languages have violations
        let has_rust_violation = violations.iter().any(|v| v.metric.contains("Rust"));
        let has_js_violation = violations.iter().any(|v| v.metric.contains("JavaScript"));
        assert!(has_rust_violation);
        assert!(has_js_violation);
    }
}
