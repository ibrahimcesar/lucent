use std::fmt;
use serde::{Deserialize, Serialize};

/// Severity level for rule violations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => write!(f, "ERROR"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Info => write!(f, "INFO"),
        }
    }
}

impl Severity {
    /// Parse severity from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "error" => Some(Severity::Error),
            "warning" => Some(Severity::Warning),
            "info" => Some(Severity::Info),
            _ => None,
        }
    }
}

/// A rule violation found in code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    /// Rule that was violated
    pub rule: String,

    /// Severity of the violation
    pub severity: Severity,

    /// Human-readable message
    pub message: String,

    /// File path where violation occurred
    pub file: String,

    /// Line number (1-indexed)
    pub line: usize,

    /// Column number (0-indexed)
    pub column: usize,

    /// Optional suggestion for fixing
    pub suggestion: Option<String>,

    /// Code snippet showing the violation
    pub code_snippet: Option<String>,
}

impl Violation {
    /// Create a new violation
    pub fn new(
        rule: String,
        severity: Severity,
        message: String,
        file: String,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            rule,
            severity,
            message,
            file,
            line,
            column,
            suggestion: None,
            code_snippet: None,
        }
    }

    /// Add a suggestion to the violation
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    /// Add a code snippet to the violation
    pub fn with_code_snippet(mut self, snippet: String) -> Self {
        self.code_snippet = Some(snippet);
        self
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{} - {} [{}]: {}",
            self.file, self.line, self.column, self.rule, self.severity, self.message
        )
    }
}
