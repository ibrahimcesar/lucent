pub mod pattern;
pub mod violation;
pub mod ast;
pub mod metric;

use std::path::Path;

pub use pattern::PatternRule;
pub use violation::{Violation, Severity};
pub use ast::{AstRule, AstMatcher};
pub use metric::{MetricRule, MetricType};

use crate::language::Language;

/// Trait for all custom rules
pub trait Rule: Send + Sync {
    /// Rule unique identifier
    fn name(&self) -> &str;

    /// Rule description
    fn description(&self) -> &str;

    /// Rule severity level
    fn severity(&self) -> Severity;

    /// Check if rule applies to this file
    fn applies_to(&self, file: &Path, language: &Language) -> bool;

    /// Analyze file and return violations
    fn check(&self, file: &Path, content: &str, language: &Language) -> Vec<Violation>;
}

/// Collection of custom rules
pub struct RuleEngine {
    rules: Vec<Box<dyn Rule>>,
}

impl RuleEngine {
    /// Create a new rule engine
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a rule to the engine
    pub fn add_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule);
    }

    /// Check all applicable rules against a file
    pub fn check_file(
        &self,
        file: &Path,
        content: &str,
        language: &Language,
    ) -> Vec<Violation> {
        let mut violations = Vec::new();

        for rule in &self.rules {
            if rule.applies_to(file, language) {
                let mut rule_violations = rule.check(file, content, language);
                violations.append(&mut rule_violations);
            }
        }

        violations
    }

    /// Get number of rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Get all rule names
    pub fn rule_names(&self) -> Vec<&str> {
        self.rules.iter().map(|r| r.name()).collect()
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}
