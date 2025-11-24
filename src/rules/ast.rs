use super::{Rule, Severity, Violation};
use crate::language::Language;
use std::path::Path;
use tree_sitter::{Node, Parser};

/// AST-based rule using tree-sitter
pub struct AstRule {
    name: String,
    description: String,
    severity: Severity,
    matcher: AstMatcher,
    file_patterns: Vec<String>,
    exclude_patterns: Vec<String>,
    language: Option<String>,
    suggestion: Option<String>,
}

/// AST matching patterns
#[derive(Debug, Clone)]
pub enum AstMatcher {
    /// Functions without documentation comments
    FunctionWithoutDocs,

    /// Public functions without documentation
    PublicFunctionWithoutDocs,

    /// Functions with too many parameters
    TooManyParameters { max: usize },

    /// Match expressions with too many arms
    TooManyMatchArms { max: usize },

    /// Deep nesting beyond threshold
    DeepNesting { max_depth: usize },

    /// Unused variables
    UnusedVariable,

    /// Missing error handling (unwrapped Results)
    UnhandledResult,

    /// Custom tree-sitter query
    CustomQuery { query: String },
}

impl AstRule {
    /// Create a new AST rule
    pub fn new(
        name: String,
        description: String,
        severity: Severity,
        matcher: AstMatcher,
    ) -> Self {
        Self {
            name,
            description,
            severity,
            matcher,
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

    /// Get tree-sitter parser for language
    fn get_parser(language: &Language) -> Option<Parser> {
        let mut parser = Parser::new();

        let lang = match language.name {
            "Rust" => Some(tree_sitter_rust::language()),
            "Python" => Some(tree_sitter_python::language()),
            "JavaScript" | "TypeScript" => Some(tree_sitter_javascript::language()),
            _ => None,
        }?;

        parser.set_language(&lang).ok()?;
        Some(parser)
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

    /// Check for functions without documentation
    fn check_function_without_docs(&self, tree: &tree_sitter::Tree, source: &str, public_only: bool) -> Vec<(usize, usize, String)> {
        let mut violations = Vec::new();
        let root = tree.root_node();

        self.visit_functions(root, source, &mut |node, source| {
            // Check if function is public (for Rust)
            if public_only {
                let func_text = &source[node.byte_range()];
                // Simple check: if starts with "pub fn"
                if !func_text.trim_start().starts_with("pub ") {
                    return; // Not public, skip
                }
            }

            // Check for doc comment before function
            // Look through previous siblings for doc comments
            let mut has_doc = false;
            let mut current = node.prev_sibling();

            while let Some(prev) = current {
                let kind = prev.kind();

                if kind == "line_comment" {
                    let comment_text = &source[prev.byte_range()];
                    if comment_text.starts_with("///") {
                        has_doc = true;
                        break;
                    }
                } else if kind != "attribute_item" {
                    // Stop if we hit something that's not a comment or attribute
                    break;
                }

                current = prev.prev_sibling();
            }

            if has_doc {
                return; // Has doc comment
            }

            let start = node.start_position();
            let func_text = &source[node.byte_range()];
            let func_name = extract_function_name(func_text);

            violations.push((start.row + 1, start.column, func_name));
        });

        violations
    }

    /// Check for functions with too many parameters
    fn check_too_many_parameters(&self, tree: &tree_sitter::Tree, source: &str, max_params: usize) -> Vec<(usize, usize, String)> {
        let mut violations = Vec::new();
        let root = tree.root_node();

        self.visit_functions(root, source, &mut |node, source| {
            if let Some(params) = node.child_by_field_name("parameters") {
                let param_count = count_parameters(params);

                if param_count > max_params {
                    let start = node.start_position();
                    let func_text = &source[node.byte_range()];
                    let func_name = extract_function_name(func_text);

                    violations.push((
                        start.row + 1,
                        start.column,
                        format!("{} ({} parameters, max {})", func_name, param_count, max_params)
                    ));
                }
            }
        });

        violations
    }

    /// Check for deep nesting beyond threshold
    fn check_deep_nesting(&self, tree: &tree_sitter::Tree, source: &str, max_depth: usize) -> Vec<(usize, usize, String)> {
        let mut violations = Vec::new();
        let root = tree.root_node();

        self.visit_all_nodes(root, source, 0, &mut |node, source, depth| {
            if depth > max_depth {
                // Only report on block-like structures
                if is_nesting_node(node) {
                    let start = node.start_position();
                    let context = get_node_context(node, source);

                    violations.push((
                        start.row + 1,
                        start.column,
                        format!("{} (depth {}, max {})", context, depth, max_depth)
                    ));
                }
            }
        });

        violations
    }

    /// Check for match expressions with too many arms
    fn check_too_many_match_arms(&self, tree: &tree_sitter::Tree, source: &str, max_arms: usize) -> Vec<(usize, usize, String)> {
        let mut violations = Vec::new();
        let root = tree.root_node();

        self.visit_match_expressions(root, source, &mut |node, _source| {
            let arm_count = count_match_arms(node);

            if arm_count > max_arms {
                let start = node.start_position();

                violations.push((
                    start.row + 1,
                    start.column,
                    format!("match expression ({} arms, max {})", arm_count, max_arms)
                ));
            }
        });

        violations
    }

    /// Check for unhandled Result types (missing ? or unwrap handling)
    fn check_unhandled_result(&self, tree: &tree_sitter::Tree, source: &str) -> Vec<(usize, usize, String)> {
        let mut violations = Vec::new();
        let root = tree.root_node();

        // Look for function calls that return Result without proper handling
        self.visit_call_expressions(root, source, &mut |node, source| {
            // Check if this call is immediately followed by ? or .unwrap()
            if let Some(parent) = node.parent() {
                let parent_kind = parent.kind();

                // If parent is try_expression, it's handled with ?
                if parent_kind == "try_expression" {
                    return;
                }

                // Check if followed by .unwrap(), .expect(), or similar
                let node_text = &source[node.byte_range()];
                if node_text.contains(".unwrap") || node_text.contains(".expect") {
                    return;
                }

                // Check if the call is in a let statement or return
                if parent_kind == "let_declaration" || parent_kind == "return_statement" {
                    // These are OK - result is being bound or returned
                    return;
                }

                let start = node.start_position();
                let call_name = extract_call_name(node_text);

                violations.push((
                    start.row + 1,
                    start.column,
                    format!("call to {} may return Result", call_name)
                ));
            }
        });

        violations
    }

    /// Visit all function nodes
    fn visit_functions<F>(&self, node: Node, source: &str, callback: &mut F)
    where
        F: FnMut(Node, &str),
    {
        if node.kind() == "function_item" ||
           node.kind() == "function_declaration" ||
           node.kind() == "function_definition" {
            callback(node, source);
        }

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.visit_functions(child, source, callback);
            }
        }
    }

    /// Visit all nodes with depth tracking
    fn visit_all_nodes<F>(&self, node: Node, source: &str, depth: usize, callback: &mut F)
    where
        F: FnMut(Node, &str, usize),
    {
        let new_depth = if is_nesting_node(node) {
            depth + 1
        } else {
            depth
        };

        callback(node, source, new_depth);

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.visit_all_nodes(child, source, new_depth, callback);
            }
        }
    }

    /// Visit all match expressions
    fn visit_match_expressions<F>(&self, node: Node, source: &str, callback: &mut F)
    where
        F: FnMut(Node, &str),
    {
        if node.kind() == "match_expression" {
            callback(node, source);
        }

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.visit_match_expressions(child, source, callback);
            }
        }
    }

    /// Visit all call expressions
    fn visit_call_expressions<F>(&self, node: Node, source: &str, callback: &mut F)
    where
        F: FnMut(Node, &str),
    {
        if node.kind() == "call_expression" {
            callback(node, source);
        }

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.visit_call_expressions(child, source, callback);
            }
        }
    }
}

impl Rule for AstRule {
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

    fn check(&self, file: &Path, content: &str, language: &Language) -> Vec<Violation> {
        let mut violations = Vec::new();
        let file_str = file.to_string_lossy().to_string();

        // Get parser for this language
        let mut parser = match Self::get_parser(language) {
            Some(p) => p,
            None => return violations, // Language not supported for AST analysis
        };

        // Parse the code
        let tree = match parser.parse(content, None) {
            Some(t) => t,
            None => return violations, // Parse failed
        };

        // Apply matcher
        let matches = match &self.matcher {
            AstMatcher::FunctionWithoutDocs => {
                self.check_function_without_docs(&tree, content, false)
            }
            AstMatcher::PublicFunctionWithoutDocs => {
                self.check_function_without_docs(&tree, content, true)
            }
            AstMatcher::TooManyParameters { max } => {
                self.check_too_many_parameters(&tree, content, *max)
            }
            AstMatcher::TooManyMatchArms { max } => {
                self.check_too_many_match_arms(&tree, content, *max)
            }
            AstMatcher::DeepNesting { max_depth } => {
                self.check_deep_nesting(&tree, content, *max_depth)
            }
            AstMatcher::UnusedVariable => {
                // TODO: Implement unused variable detection
                // This is complex and requires symbol table analysis
                Vec::new()
            }
            AstMatcher::UnhandledResult => {
                self.check_unhandled_result(&tree, content)
            }
            AstMatcher::CustomQuery { query: _ } => {
                // TODO: Implement custom query support
                Vec::new()
            }
        };

        // Convert matches to violations
        for (line, column, context) in matches {
            let mut violation = Violation::new(
                self.name.clone(),
                self.severity,
                format!("{} - {}", self.description, context),
                file_str.clone(),
                line,
                column,
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

/// Extract function name from function text
fn extract_function_name(text: &str) -> String {
    // Simple extraction - look for "fn name" or "function name"
    if let Some(fn_pos) = text.find("fn ") {
        let after_fn = &text[fn_pos + 3..];
        if let Some(end) = after_fn.find(|c: char| c == '(' || c == '<' || c.is_whitespace()) {
            return after_fn[..end].trim().to_string();
        }
    }

    if let Some(func_pos) = text.find("function ") {
        let after_func = &text[func_pos + 9..];
        if let Some(end) = after_func.find(|c: char| c == '(' || c.is_whitespace()) {
            return after_func[..end].trim().to_string();
        }
    }

    "unknown".to_string()
}

/// Count parameters in a parameters node
fn count_parameters(params_node: Node) -> usize {
    let mut count = 0;

    for i in 0..params_node.child_count() {
        if let Some(child) = params_node.child(i) {
            if child.kind() == "parameter" ||
               child.kind() == "typed_parameter" ||
               child.kind() == "identifier" {
                count += 1;
            }
        }
    }

    // Simple fallback: return what we counted
    count
}

/// Count match arms in a match expression
fn count_match_arms(match_node: Node) -> usize {
    let mut count = 0;

    for i in 0..match_node.child_count() {
        if let Some(child) = match_node.child(i) {
            if child.kind() == "match_arm" {
                count += 1;
            }
        }
    }

    count
}

/// Check if a node contributes to nesting depth
fn is_nesting_node(node: Node) -> bool {
    matches!(
        node.kind(),
        "block" | "if_expression" | "while_expression" | "for_expression" |
        "loop_expression" | "match_expression" | "match_arm" |
        "closure_expression" | "function_item"
    )
}

/// Get context description for a node
fn get_node_context(node: Node, _source: &str) -> String {
    let kind = node.kind();
    match kind {
        "if_expression" => "if statement".to_string(),
        "while_expression" => "while loop".to_string(),
        "for_expression" => "for loop".to_string(),
        "loop_expression" => "loop".to_string(),
        "match_expression" => "match expression".to_string(),
        "block" => "block".to_string(),
        _ => kind.to_string(),
    }
}

/// Extract function/method name from call expression text
fn extract_call_name(text: &str) -> String {
    // Simple extraction - get the identifier before the (
    if let Some(paren_pos) = text.find('(') {
        let before_paren = &text[..paren_pos];
        // Get the last identifier (handles foo.bar.baz())
        if let Some(last_dot) = before_paren.rfind('.') {
            return before_paren[last_dot + 1..].trim().to_string();
        }
        return before_paren.trim().to_string();
    }
    "unknown".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_function_name() {
        assert_eq!(extract_function_name("fn hello() {"), "hello");
        assert_eq!(extract_function_name("fn test_function(x: i32) {"), "test_function");
        assert_eq!(extract_function_name("function myFunc() {"), "myFunc");
    }

    #[test]
    fn test_glob_match() {
        assert!(glob_match("*.rs", "main.rs"));
        assert!(glob_match("src/**/*.rs", "src/main.rs"));
        assert!(glob_match("src/**/*.rs", "src/lib/utils.rs"));
        assert!(!glob_match("*.rs", "main.js"));
    }
}
