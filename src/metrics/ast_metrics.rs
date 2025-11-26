//! AST-based metrics calculation using tree-sitter
//!
//! This module provides accurate metrics by parsing source code into an AST
//! rather than relying on regex patterns.

use serde::{Deserialize, Serialize};
use tree_sitter::{Node, Parser, Query, QueryCursor};

/// AST-based metrics for a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AstFileMetrics {
    /// Number of functions/methods
    pub function_count: usize,
    /// Total lines in all functions
    pub total_function_lines: usize,
    /// Maximum function length
    pub max_function_length: usize,
    /// Average function length
    pub avg_function_length: f64,
    /// Cyclomatic complexity (accurate)
    pub cyclomatic_complexity: usize,
    /// Cognitive complexity (accurate)
    pub cognitive_complexity: usize,
    /// Maximum nesting depth
    pub max_nesting_depth: usize,
    /// Number of parameters across all functions
    pub total_parameters: usize,
    /// Maximum parameters in a single function
    pub max_parameters: usize,
    /// Number of classes/structs/types
    pub type_count: usize,
    /// Number of imports/use statements
    pub import_count: usize,
}

/// AST-based metrics analyzer
pub struct AstAnalyzer {
    parser: Parser,
    language_name: String,
}

impl AstAnalyzer {
    /// Create a new AST analyzer for the given language
    pub fn new(language_name: &str) -> Option<Self> {
        let mut parser = Parser::new();

        let lang = match language_name {
            "Rust" => tree_sitter_rust::language(),
            "Python" => tree_sitter_python::language(),
            "JavaScript" | "TypeScript" => tree_sitter_javascript::language(),
            "Go" => tree_sitter_go::language(),
            _ => return None,
        };

        parser.set_language(&lang).ok()?;

        Some(Self {
            parser,
            language_name: language_name.to_string(),
        })
    }

    /// Analyze source code and return AST-based metrics
    pub fn analyze(&mut self, source: &str) -> Option<AstFileMetrics> {
        let tree = self.parser.parse(source, None)?;
        let root = tree.root_node();
        let source_bytes = source.as_bytes();

        let mut metrics = AstFileMetrics::default();

        // Analyze functions
        self.analyze_functions(root, source_bytes, &mut metrics);

        // Calculate complexity metrics
        self.calculate_complexity(root, source_bytes, &mut metrics);

        // Count types and imports
        self.count_types_and_imports(root, &mut metrics);

        // Calculate averages
        if metrics.function_count > 0 {
            metrics.avg_function_length = metrics.total_function_lines as f64 / metrics.function_count as f64;
        }

        Some(metrics)
    }

    /// Analyze all functions in the AST
    fn analyze_functions(&self, node: Node, source: &[u8], metrics: &mut AstFileMetrics) {
        let kind = node.kind();

        // Check if this is a function node
        let is_function = match self.language_name.as_str() {
            "Rust" => kind == "function_item" || kind == "impl_item",
            "Python" => kind == "function_definition",
            "JavaScript" | "TypeScript" => {
                kind == "function_declaration" ||
                kind == "function" ||
                kind == "arrow_function" ||
                kind == "method_definition"
            }
            "Go" => kind == "function_declaration" || kind == "method_declaration",
            _ => false,
        };

        if is_function && kind != "impl_item" {
            metrics.function_count += 1;

            // Calculate function length
            let start_line = node.start_position().row;
            let end_line = node.end_position().row;
            let func_length = end_line - start_line + 1;

            metrics.total_function_lines += func_length;
            if func_length > metrics.max_function_length {
                metrics.max_function_length = func_length;
            }

            // Count parameters
            let params = self.count_function_parameters(node, source);
            metrics.total_parameters += params;
            if params > metrics.max_parameters {
                metrics.max_parameters = params;
            }
        }

        // Recurse into children
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.analyze_functions(child, source, metrics);
            }
        }
    }

    /// Count parameters for a function node
    fn count_function_parameters(&self, node: Node, _source: &[u8]) -> usize {
        // Try to find parameters node
        if let Some(params) = node.child_by_field_name("parameters") {
            let mut count = 0;
            for i in 0..params.child_count() {
                if let Some(child) = params.child(i) {
                    let kind = child.kind();
                    // Count parameter-like nodes
                    if kind.contains("parameter") ||
                       kind == "identifier" ||
                       kind == "typed_parameter" ||
                       kind == "default_parameter" {
                        count += 1;
                    }
                }
            }
            return count;
        }
        0
    }

    /// Calculate cyclomatic and cognitive complexity
    fn calculate_complexity(&self, node: Node, source: &[u8], metrics: &mut AstFileMetrics) {
        let mut max_depth = 0;

        self.visit_for_complexity(node, source, 0, metrics, &mut max_depth);
        metrics.max_nesting_depth = max_depth;
    }

    /// Recursive visitor for complexity calculation
    fn visit_for_complexity(
        &self,
        node: Node,
        source: &[u8],
        depth: usize,
        metrics: &mut AstFileMetrics,
        max_depth: &mut usize,
    ) {
        let kind = node.kind();

        // Track max depth
        if depth > *max_depth {
            *max_depth = depth;
        }

        // Decision points that increase cyclomatic complexity
        let is_decision_point = match self.language_name.as_str() {
            "Rust" => matches!(
                kind,
                "if_expression" | "if_let_expression" |
                "while_expression" | "while_let_expression" |
                "for_expression" | "loop_expression" |
                "match_arm" | "binary_expression"
            ),
            "Python" => matches!(
                kind,
                "if_statement" | "elif_clause" |
                "while_statement" | "for_statement" |
                "except_clause" | "with_statement" |
                "match_statement" | "case_clause" |
                "boolean_operator" | "conditional_expression"
            ),
            "JavaScript" | "TypeScript" => matches!(
                kind,
                "if_statement" | "while_statement" |
                "for_statement" | "for_in_statement" |
                "do_statement" | "switch_case" |
                "catch_clause" | "ternary_expression" |
                "binary_expression"
            ),
            "Go" => matches!(
                kind,
                "if_statement" | "for_statement" |
                "expression_switch_statement" | "type_switch_statement" |
                "select_statement" | "expression_case" |
                "type_case" | "communication_case"
            ),
            _ => false,
        };

        // Check for boolean operators (&&, ||)
        let is_boolean_op = if kind == "binary_expression" {
            if let Some(op) = node.child_by_field_name("operator") {
                let op_text = op.utf8_text(source).unwrap_or("");
                op_text == "&&" || op_text == "||" || op_text == "and" || op_text == "or"
            } else {
                false
            }
        } else {
            false
        };

        if is_decision_point || is_boolean_op {
            metrics.cyclomatic_complexity += 1;

            // Cognitive complexity: add 1 + nesting level for control structures
            if !is_boolean_op {
                metrics.cognitive_complexity += 1 + depth;
            } else {
                // Boolean operators add 1 (not nested)
                metrics.cognitive_complexity += 1;
            }
        }

        // Determine if this node increases nesting
        let is_nesting_construct = match self.language_name.as_str() {
            "Rust" => matches!(
                kind,
                "if_expression" | "while_expression" |
                "for_expression" | "loop_expression" |
                "match_expression" | "closure_expression"
            ),
            "Python" => matches!(
                kind,
                "if_statement" | "while_statement" |
                "for_statement" | "with_statement" |
                "try_statement" | "function_definition" |
                "class_definition"
            ),
            "JavaScript" | "TypeScript" => matches!(
                kind,
                "if_statement" | "while_statement" |
                "for_statement" | "for_in_statement" |
                "do_statement" | "switch_statement" |
                "try_statement" | "function_declaration" |
                "arrow_function"
            ),
            "Go" => matches!(
                kind,
                "if_statement" | "for_statement" |
                "expression_switch_statement" | "type_switch_statement" |
                "select_statement" | "function_declaration"
            ),
            _ => false,
        };

        let new_depth = if is_nesting_construct { depth + 1 } else { depth };

        // Recurse into children
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.visit_for_complexity(child, source, new_depth, metrics, max_depth);
            }
        }
    }

    /// Count types (structs, classes, etc.) and imports
    fn count_types_and_imports(&self, node: Node, metrics: &mut AstFileMetrics) {
        let kind = node.kind();

        // Type definitions
        let is_type = match self.language_name.as_str() {
            "Rust" => matches!(kind, "struct_item" | "enum_item" | "type_item" | "trait_item"),
            "Python" => kind == "class_definition",
            "JavaScript" | "TypeScript" => kind == "class_declaration" || kind == "interface_declaration",
            "Go" => kind == "type_declaration",
            _ => false,
        };

        // Import statements
        let is_import = match self.language_name.as_str() {
            "Rust" => kind == "use_declaration",
            "Python" => matches!(kind, "import_statement" | "import_from_statement"),
            "JavaScript" | "TypeScript" => kind == "import_statement",
            "Go" => kind == "import_declaration",
            _ => false,
        };

        if is_type {
            metrics.type_count += 1;
        }

        if is_import {
            metrics.import_count += 1;
        }

        // Recurse
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.count_types_and_imports(child, metrics);
            }
        }
    }

    /// Run a custom tree-sitter query and count matches
    pub fn query_count(&mut self, source: &str, query_str: &str) -> Option<usize> {
        let tree = self.parser.parse(source, None)?;
        let lang = tree.language();

        let query = Query::new(&lang, query_str).ok()?;
        let mut cursor = QueryCursor::new();
        let source_bytes = source.as_bytes();

        let matches = cursor.matches(&query, tree.root_node(), source_bytes);
        Some(matches.count())
    }

    /// Get all matches from a custom query with position info
    pub fn query_matches(&mut self, source: &str, query_str: &str) -> Option<Vec<QueryMatch>> {
        let tree = self.parser.parse(source, None)?;
        let lang = tree.language();

        let query = Query::new(&lang, query_str).ok()?;
        let mut cursor = QueryCursor::new();
        let source_bytes = source.as_bytes();

        let mut results = Vec::new();
        let matches = cursor.matches(&query, tree.root_node(), source_bytes);

        for m in matches {
            for capture in m.captures {
                let node = capture.node;
                let text = node.utf8_text(source_bytes).unwrap_or("").to_string();
                let capture_name = query.capture_names()
                    .get(capture.index as usize)
                    .map(|s| s.to_string())
                    .unwrap_or_default();

                results.push(QueryMatch {
                    capture_name,
                    text,
                    start_line: node.start_position().row + 1,
                    start_column: node.start_position().column,
                    end_line: node.end_position().row + 1,
                    end_column: node.end_position().column,
                });
            }
        }

        Some(results)
    }
}

/// A single match from a tree-sitter query
#[derive(Debug, Clone)]
pub struct QueryMatch {
    /// Name of the capture (from @capture in query)
    pub capture_name: String,
    /// Matched text
    pub text: String,
    /// Start line (1-indexed)
    pub start_line: usize,
    /// Start column
    pub start_column: usize,
    /// End line (1-indexed)
    pub end_line: usize,
    /// End column
    pub end_column: usize,
}

/// Common tree-sitter queries for various languages
pub struct CommonQueries;

impl CommonQueries {
    /// Find all function definitions in Rust
    pub const RUST_FUNCTIONS: &'static str = "(function_item name: (identifier) @name)";

    /// Find all public functions in Rust
    pub const RUST_PUBLIC_FUNCTIONS: &'static str = r#"
        (function_item
            (visibility_modifier)
            name: (identifier) @name)
    "#;

    /// Find all unsafe blocks in Rust
    pub const RUST_UNSAFE_BLOCKS: &'static str = "(unsafe_block) @unsafe";

    /// Find all unwrap calls in Rust
    pub const RUST_UNWRAP_CALLS: &'static str = r#"
        (call_expression
            function: (field_expression
                field: (field_identifier) @method
                (#match? @method "unwrap")))
    "#;

    /// Find all TODO comments
    pub const TODO_COMMENTS: &'static str = r#"
        ((line_comment) @comment
         (#match? @comment "TODO"))
    "#;

    /// Find all function definitions in Python
    pub const PYTHON_FUNCTIONS: &'static str = "(function_definition name: (identifier) @name)";

    /// Find all class definitions in Python
    pub const PYTHON_CLASSES: &'static str = "(class_definition name: (identifier) @name)";

    /// Find all function declarations in Go
    pub const GO_FUNCTIONS: &'static str = "(function_declaration name: (identifier) @name)";

    /// Find all defer statements in Go
    pub const GO_DEFER: &'static str = "(defer_statement) @defer";

    /// Find all goroutine calls in Go
    pub const GO_GOROUTINES: &'static str = "(go_statement) @goroutine";

    /// Find all function declarations in JavaScript/TypeScript
    pub const JS_FUNCTIONS: &'static str = r#"
        [
            (function_declaration name: (identifier) @name)
            (arrow_function) @arrow
            (method_definition name: (property_identifier) @name)
        ]
    "#;

    /// Find all console.log calls in JavaScript
    pub const JS_CONSOLE_LOG: &'static str = r#"
        (call_expression
            function: (member_expression
                object: (identifier) @obj
                property: (property_identifier) @prop)
            (#eq? @obj "console")
            (#eq? @prop "log"))
    "#;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_analysis() {
        let mut analyzer = AstAnalyzer::new("Rust").unwrap();

        let code = r#"
fn simple() {
    println!("hello");
}

fn complex(a: i32, b: i32, c: i32) -> i32 {
    if a > 0 {
        if b > 0 {
            return a + b + c;
        }
    }
    0
}

pub fn public_fn() {}
"#;

        let metrics = analyzer.analyze(code).unwrap();

        assert_eq!(metrics.function_count, 3);
        assert!(metrics.cyclomatic_complexity >= 2); // At least 2 if statements
        assert!(metrics.max_nesting_depth >= 2); // Nested ifs
    }

    #[test]
    fn test_query_count() {
        let mut analyzer = AstAnalyzer::new("Rust").unwrap();

        let code = r#"
fn foo() {}
fn bar() {}
fn baz() {}
"#;

        let count = analyzer.query_count(code, CommonQueries::RUST_FUNCTIONS).unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_python_analysis() {
        let mut analyzer = AstAnalyzer::new("Python").unwrap();

        let code = r#"
def hello():
    print("hello")

def complex(a, b, c):
    if a > 0:
        if b > 0:
            return a + b + c
    return 0

class MyClass:
    def method(self):
        pass
"#;

        let metrics = analyzer.analyze(code).unwrap();

        assert!(metrics.function_count >= 2); // hello, complex (method may or may not count)
        assert_eq!(metrics.type_count, 1); // MyClass
    }
}
