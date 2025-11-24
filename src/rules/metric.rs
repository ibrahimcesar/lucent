use super::{Rule, Severity, Violation};
use crate::language::Language;
use crate::metrics::FileMetrics;
use std::path::Path;

/// Metric-based rule that applies thresholds to code metrics
pub struct MetricRule {
    name: String,
    description: String,
    severity: Severity,
    metric_type: MetricType,
    file_patterns: Vec<String>,
    exclude_patterns: Vec<String>,
    language: Option<String>,
    suggestion: Option<String>,
}

/// Types of metrics to check
#[derive(Debug, Clone)]
pub enum MetricType {
    /// Cyclomatic complexity threshold
    CyclomaticComplexity { max: f64 },

    /// Cognitive complexity threshold
    CognitiveComplexity { max: f64 },

    /// Nesting depth threshold
    NestingDepth { max: usize },

    /// Function length threshold (lines)
    FunctionLength { max: usize },

    /// File size threshold (lines)
    FileSize { max: usize },

    /// Comment ratio thresholds
    CommentRatio { min: f64, max: f64 },

    /// Code density thresholds
    CodeDensity { min: f64, max: f64 },

    /// Maintainability index threshold
    MaintainabilityIndex { min: f64 },

    /// Halstead volume threshold
    HalsteadVolume { max: f64 },

    /// Halstead difficulty threshold
    HalsteadDifficulty { max: f64 },

    /// Number of functions threshold
    FunctionCount { max: usize },

    /// TODO/FIXME/HACK markers threshold
    TodoMarkers { max: usize },
}

impl MetricRule {
    /// Create a new metric rule
    pub fn new(
        name: String,
        description: String,
        severity: Severity,
        metric_type: MetricType,
    ) -> Self {
        Self {
            name,
            description,
            severity,
            metric_type,
            file_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            language: None,
            suggestion: None,
        }
    }

    /// Set file patterns to include
    pub fn with_file_patterns(mut self, patterns: Vec<String>) -> Self {
        self.file_patterns = patterns;
        self
    }

    /// Set file patterns to exclude
    pub fn with_exclude_patterns(mut self, patterns: Vec<String>) -> Self {
        self.exclude_patterns = patterns;
        self
    }

    /// Set target language
    pub fn with_language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    /// Set suggestion text
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    /// Check if file matches patterns
    fn matches_file_patterns(&self, file: &Path) -> bool {
        let file_str = file.to_string_lossy();

        if self.file_patterns.is_empty() {
            return true;
        }

        for pattern in &self.file_patterns {
            if glob_match(pattern, &file_str) {
                return true;
            }
        }

        false
    }

    /// Check if file should be excluded
    fn is_excluded(&self, file: &Path) -> bool {
        let file_str = file.to_string_lossy();

        for pattern in &self.exclude_patterns {
            if glob_match(pattern, &file_str) {
                return true;
            }
        }

        false
    }

    /// Check metrics against thresholds
    fn check_metrics(&self, metrics: &FileMetrics) -> Option<String> {
        match &self.metric_type {
            MetricType::CyclomaticComplexity { max } => {
                if metrics.cyclomatic_complexity as f64 > *max {
                    Some(format!(
                        "cyclomatic complexity {} exceeds threshold {}",
                        metrics.cyclomatic_complexity, max
                    ))
                } else {
                    None
                }
            }
            MetricType::CognitiveComplexity { max } => {
                if metrics.cognitive_complexity as f64 > *max {
                    Some(format!(
                        "cognitive complexity {} exceeds threshold {}",
                        metrics.cognitive_complexity, max
                    ))
                } else {
                    None
                }
            }
            MetricType::NestingDepth { max } => {
                if metrics.max_nesting_depth > *max {
                    Some(format!(
                        "nesting depth {} exceeds threshold {}",
                        metrics.max_nesting_depth, max
                    ))
                } else {
                    None
                }
            }
            MetricType::FunctionLength { max } => {
                if metrics.avg_function_length > (*max) as f64 {
                    Some(format!(
                        "average function length {:.1} exceeds threshold {}",
                        metrics.avg_function_length, max
                    ))
                } else {
                    None
                }
            }
            MetricType::FileSize { max } => {
                if metrics.total_lines > *max {
                    Some(format!(
                        "file size {} lines exceeds threshold {}",
                        metrics.total_lines, max
                    ))
                } else {
                    None
                }
            }
            MetricType::CommentRatio { min, max } => {
                let ratio = metrics.comment_ratio() * 100.0;
                if ratio < *min {
                    Some(format!(
                        "comment ratio {:.1}% below minimum {:.1}%",
                        ratio, min
                    ))
                } else if ratio > *max {
                    Some(format!(
                        "comment ratio {:.1}% exceeds maximum {:.1}%",
                        ratio, max
                    ))
                } else {
                    None
                }
            }
            MetricType::CodeDensity { min, max } => {
                // Calculate code density as percentage of non-blank lines that are code
                let density = if metrics.total_lines > 0 {
                    (metrics.code_lines as f64 / metrics.total_lines as f64) * 100.0
                } else {
                    0.0
                };
                if density < *min {
                    Some(format!(
                        "code density {:.1}% below minimum {:.1}%",
                        density, min
                    ))
                } else if density > *max {
                    Some(format!(
                        "code density {:.1}% exceeds maximum {:.1}%",
                        density, max
                    ))
                } else {
                    None
                }
            }
            MetricType::MaintainabilityIndex { min } => {
                if metrics.maintainability_index < *min {
                    Some(format!(
                        "maintainability index {:.1} below threshold {:.1}",
                        metrics.maintainability_index, min
                    ))
                } else {
                    None
                }
            }
            MetricType::HalsteadVolume { max } => {
                if metrics.halstead_volume > *max {
                    Some(format!(
                        "Halstead volume {:.1} exceeds threshold {:.1}",
                        metrics.halstead_volume, max
                    ))
                } else {
                    None
                }
            }
            MetricType::HalsteadDifficulty { max } => {
                if metrics.halstead_difficulty > *max {
                    Some(format!(
                        "Halstead difficulty {:.1} exceeds threshold {:.1}",
                        metrics.halstead_difficulty, max
                    ))
                } else {
                    None
                }
            }
            MetricType::FunctionCount { max } => {
                if metrics.function_count > *max {
                    Some(format!(
                        "function count {} exceeds threshold {}",
                        metrics.function_count, max
                    ))
                } else {
                    None
                }
            }
            MetricType::TodoMarkers { max } => {
                let total_markers = metrics.todo_count + metrics.fixme_count + metrics.hack_count;
                if total_markers > *max {
                    Some(format!(
                        "TODO/FIXME/HACK markers {} exceeds threshold {}",
                        total_markers, max
                    ))
                } else {
                    None
                }
            }
        }
    }
}

impl Rule for MetricRule {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn severity(&self) -> Severity {
        self.severity
    }

    fn applies_to(&self, file: &Path, language: &Language) -> bool {
        // Check language filter
        if let Some(ref lang) = self.language {
            if lang != &language.name {
                return false;
            }
        }

        // Check file patterns
        if !self.matches_file_patterns(file) {
            return false;
        }

        // Check exclusions
        if self.is_excluded(file) {
            return false;
        }

        true
    }

    fn check(&self, file: &Path, _content: &str, language: &Language) -> Vec<Violation> {
        let mut violations = Vec::new();

        // Analyze the file to get metrics
        let metrics = match FileMetrics::analyze(file, language) {
            Ok(m) => m,
            Err(_) => return violations, // Failed to analyze
        };

        // Check metrics against thresholds
        if let Some(violation_msg) = self.check_metrics(&metrics) {
            let mut violation = Violation::new(
                self.name.clone(),
                self.severity,
                format!("{} - {}", self.description, violation_msg),
                file.to_string_lossy().to_string(),
                1, // Line 1 for file-level metrics
                0,
            );

            if let Some(ref suggestion) = self.suggestion {
                violation = violation.with_suggestion(suggestion.clone());
            }

            violations.push(violation);
        }

        violations
    }
}

/// Simple glob pattern matching
fn glob_match(pattern: &str, path: &str) -> bool {
    if pattern.contains("**") {
        let parts: Vec<&str> = pattern.split("**").collect();
        if parts.len() == 2 {
            let prefix = parts[0];
            let suffix = parts[1].trim_start_matches('/');

            if !prefix.is_empty() && !path.starts_with(prefix) {
                return false;
            }

            if !suffix.is_empty() && !path.ends_with(suffix) {
                return false;
            }

            return true;
        }
    }

    if pattern.contains('*') {
        let pattern_regex = pattern.replace(".", "\\.").replace("*", ".*");
        if let Ok(re) = regex::Regex::new(&format!("^{}$", pattern_regex)) {
            return re.is_match(path);
        }
    }

    path == pattern
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glob_match() {
        assert!(glob_match("*.rs", "main.rs"));
        assert!(glob_match("src/**/*.rs", "src/main.rs"));
        assert!(glob_match("src/**/*.rs", "src/lib/utils.rs"));
        assert!(!glob_match("*.rs", "main.js"));
    }
}
