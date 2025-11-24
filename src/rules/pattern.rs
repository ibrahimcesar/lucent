use super::{Rule, Severity, Violation};
use crate::language::Language;
use regex::Regex;
use std::path::Path;

/// A pattern-based rule using regex matching
pub struct PatternRule {
    name: String,
    description: String,
    pattern: Regex,
    severity: Severity,
    file_patterns: Vec<String>,
    exclude_patterns: Vec<String>,
    language: Option<String>,
    suggestion: Option<String>,
}

impl PatternRule {
    /// Create a new pattern rule
    pub fn new(
        name: String,
        description: String,
        pattern: String,
        severity: Severity,
    ) -> Result<Self, regex::Error> {
        Ok(Self {
            name,
            description,
            pattern: Regex::new(&pattern)?,
            severity,
            file_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            language: None,
            suggestion: None,
        })
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

    /// Check if file path matches the patterns
    fn matches_file_patterns(&self, file: &Path) -> bool {
        let file_str = file.to_string_lossy();

        // If no patterns specified, match all files
        if self.file_patterns.is_empty() {
            return true;
        }

        // Check if file matches any include pattern
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
}

impl Rule for PatternRule {
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

    fn check(&self, file: &Path, content: &str, _language: &Language) -> Vec<Violation> {
        let mut violations = Vec::new();
        let file_str = file.to_string_lossy().to_string();

        for (line_num, line) in content.lines().enumerate() {
            if let Some(mat) = self.pattern.find(line) {
                let mut violation = Violation::new(
                    self.name.clone(),
                    self.severity,
                    self.description.clone(),
                    file_str.clone(),
                    line_num + 1, // 1-indexed
                    mat.start(),
                );

                if let Some(ref suggestion) = self.suggestion {
                    violation = violation.with_suggestion(suggestion.clone());
                }

                violation = violation.with_code_snippet(line.to_string());

                violations.push(violation);
            }
        }

        violations
    }
}

/// Simple glob pattern matching
fn glob_match(pattern: &str, path: &str) -> bool {
    // Handle ** (match any directory depth)
    if pattern.contains("**") {
        let parts: Vec<&str> = pattern.split("**").collect();
        if parts.len() == 2 {
            let prefix = parts[0];
            let suffix = parts[1].trim_start_matches('/');

            // Check prefix
            if !prefix.is_empty() && !path.starts_with(prefix) {
                return false;
            }

            // Check suffix
            if !suffix.is_empty() && !path.ends_with(suffix) {
                return false;
            }

            return true;
        }
    }

    // Handle * (match within directory)
    if pattern.contains('*') {
        let pattern_regex = pattern
            .replace(".", "\\.")
            .replace("*", ".*");

        if let Ok(re) = Regex::new(&format!("^{}$", pattern_regex)) {
            return re.is_match(path);
        }
    }

    // Exact match
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
        assert!(glob_match("**/tests/**", "src/tests/unit.rs"));
    }

    #[test]
    fn test_pattern_rule() {
        let rule = PatternRule::new(
            "no-unwrap".to_string(),
            "Avoid .unwrap() calls".to_string(),
            r"\.unwrap\(".to_string(),
            Severity::Warning,
        )
        .unwrap()
        .with_file_patterns(vec!["**/*.rs".to_string()])
        .with_suggestion("Use `?` or `.unwrap_or()` instead".to_string());

        let code = "let value = result.unwrap();\nlet ok = result.is_ok();";
        let file = Path::new("src/main.rs");
        let language = Language::new("Rust");

        let violations = rule.check(file, code, &language);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 1);
        assert_eq!(violations[0].severity, Severity::Warning);
    }
}
