//! Code metrics calculations
//!
//! This module provides various code quality metrics including line counts,
//! entropy, and complexity measures.

use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::language::Language;

/// A detected code smell
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodeSmell {
    /// Type of code smell
    pub smell_type: CodeSmellType,
    /// Location in the file (line number)
    pub line: usize,
    /// Description of the smell
    pub description: String,
    /// Severity level
    pub severity: SmellSeverity,
}

/// Types of code smells
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CodeSmellType {
    /// Function exceeds recommended length threshold
    LongFunction,
    /// Function has too many parameters
    LongParameterList,
    /// Code has excessive nesting levels
    DeepNesting,
    /// Literal number without named constant
    MagicNumber,
}

/// Severity levels for code smells
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SmellSeverity {
    /// Minor issue, low priority
    Minor,
    /// Moderate issue, should be addressed
    Moderate,
    /// Major issue, high priority
    Major,
}

/// Metrics for a single file
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct FileMetrics {
    /// Total lines in the file
    pub total_lines: usize,
    /// Lines of code (non-empty, non-comment)
    pub code_lines: usize,
    /// Comment lines
    pub comment_lines: usize,
    /// Blank lines
    pub blank_lines: usize,
    /// Shannon entropy (information density)
    pub entropy: f64,
    /// Cyclomatic complexity
    pub cyclomatic_complexity: usize,
    /// Cognitive complexity (understandability measure)
    pub cognitive_complexity: usize,
    /// Maximum nesting depth
    pub max_nesting_depth: usize,
    /// Number of functions/methods
    pub function_count: usize,
    /// Average function length (lines per function)
    pub avg_function_length: f64,
    /// TODO markers found
    pub todo_count: usize,
    /// FIXME markers found
    pub fixme_count: usize,
    /// HACK markers found
    pub hack_count: usize,
    /// Comment quality score (0.0 to 1.0)
    pub comment_quality_score: f64,
    /// Number of low-quality comments (obvious, noise, or commented-out code)
    pub low_quality_comments: usize,
    /// Number of high-quality comments (explanatory, documentation)
    pub high_quality_comments: usize,
    /// Detected code smells
    pub code_smells: Vec<CodeSmell>,
    /// Maintainability Index (0-100, higher is better)
    pub maintainability_index: f64,
    /// Halstead Volume (program size)
    pub halstead_volume: f64,
    /// Halstead Difficulty (comprehension difficulty)
    pub halstead_difficulty: f64,
    /// Halstead Effort (time to understand)
    pub halstead_effort: f64,
}

/// Aggregated metrics for a project
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProjectMetrics {
    /// Total files analyzed
    pub total_files: usize,
    /// Metrics per language
    pub by_language: HashMap<String, LanguageMetrics>,
}

/// Metrics aggregated by language
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LanguageMetrics {
    /// Number of files
    pub file_count: usize,
    /// Total lines
    pub total_lines: usize,
    /// Code lines
    pub code_lines: usize,
    /// Comment lines
    pub comment_lines: usize,
    /// Blank lines
    pub blank_lines: usize,
    /// Average entropy
    pub avg_entropy: f64,
    /// Average cyclomatic complexity
    pub avg_cyclomatic: f64,
    /// Average cognitive complexity
    pub avg_cognitive: f64,
    /// Average maximum nesting depth
    pub avg_nesting_depth: f64,
    /// Total function count
    pub total_functions: usize,
    /// Average function length across all files
    pub avg_function_length: f64,
    /// Total TODO markers
    pub todo_count: usize,
    /// Total FIXME markers
    pub fixme_count: usize,
    /// Total HACK markers
    pub hack_count: usize,
    /// Average comment quality score
    pub avg_comment_quality: f64,
    /// Total low-quality comments
    pub low_quality_comments: usize,
    /// Total high-quality comments
    pub high_quality_comments: usize,
    /// Code smell counts by type
    /// Count of functions exceeding length threshold
    pub long_functions: usize,
    /// Count of functions with too many parameters
    pub long_parameter_lists: usize,
    /// Count of deep nesting instances
    pub deep_nesting_instances: usize,
    /// Count of magic number occurrences
    pub magic_numbers: usize,
    /// Average Maintainability Index
    pub avg_maintainability_index: f64,
}

impl FileMetrics {
    /// Analyze a file and calculate metrics
    pub fn analyze(path: &Path, language: &Language) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::analyze_content(&content, language)
    }

    /// Analyze content string and calculate metrics
    pub fn analyze_content(content: &str, language: &Language) -> Result<Self> {
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        let mut code_lines = 0;
        let mut comment_lines = 0;
        let mut blank_lines = 0;
        let mut todo_count = 0;
        let mut fixme_count = 0;
        let mut hack_count = 0;

        let mut in_block_comment = false;

        for line in &lines {
            let trimmed = line.trim();

            // Blank line
            if trimmed.is_empty() {
                blank_lines += 1;
                continue;
            }

            // Check for block comment start/end
            if let Some((start, end)) = language.block_comment {
                if trimmed.starts_with(start) && !in_block_comment {
                    in_block_comment = true;
                }

                if in_block_comment {
                    comment_lines += 1;

                    if trimmed.ends_with(end) || trimmed.contains(end) {
                        in_block_comment = false;
                    }
                    continue;
                }
            }

            // Check for line comment
            if let Some(prefix) = language.line_comment {
                if trimmed.starts_with(prefix) {
                    comment_lines += 1;
                    continue;
                }
            }

            // Otherwise it's code
            code_lines += 1;
        }

        // Detect code markers (TODO, FIXME, HACK) in all lines
        for line in &lines {
            let upper = line.to_uppercase();
            if upper.contains("TODO") {
                todo_count += 1;
            }
            if upper.contains("FIXME") {
                fixme_count += 1;
            }
            if upper.contains("HACK") {
                hack_count += 1;
            }
        }

        // Calculate Shannon entropy
        let entropy = Self::calculate_entropy(content);

        // Calculate cyclomatic complexity
        let cyclomatic_complexity = Self::calculate_cyclomatic_complexity(content);

        // Calculate cognitive complexity
        let cognitive_complexity = Self::calculate_cognitive_complexity(content);

        // Calculate maximum nesting depth
        let max_nesting_depth = Self::calculate_max_nesting_depth(content);

        // Calculate function metrics
        let (function_count, avg_function_length) = Self::calculate_function_metrics(content, language);

        // Calculate comment quality
        let (comment_quality_score, low_quality_comments, high_quality_comments) =
            Self::calculate_comment_quality(content, language);

        // Detect code smells
        let code_smells = Self::detect_code_smells(
            content,
            language,
            avg_function_length,
            max_nesting_depth,
        );

        // Calculate comment ratio for Maintainability Index
        let comment_ratio = if total_lines > 0 {
            comment_lines as f64 / total_lines as f64
        } else {
            0.0
        };

        // Calculate Halstead metrics
        let (halstead_volume, halstead_difficulty, halstead_effort) =
            Self::calculate_halstead_metrics(content, &language);

        // Calculate Maintainability Index using real Halstead volume
        let maintainability_index = Self::calculate_maintainability_index(
            code_lines,
            cyclomatic_complexity,
            halstead_volume,
            comment_ratio,
        );

        Ok(Self {
            total_lines,
            code_lines,
            comment_lines,
            blank_lines,
            entropy,
            cyclomatic_complexity,
            cognitive_complexity,
            max_nesting_depth,
            function_count,
            avg_function_length,
            todo_count,
            fixme_count,
            hack_count,
            comment_quality_score,
            low_quality_comments,
            high_quality_comments,
            code_smells,
            maintainability_index,
            halstead_volume,
            halstead_difficulty,
            halstead_effort,
        })
    }

    /// Calculate Shannon entropy for the content
    /// Higher entropy = more complex/random, Lower entropy = more repetitive/simple
    fn calculate_entropy(content: &str) -> f64 {
        if content.is_empty() {
            return 0.0;
        }

        let mut char_counts: HashMap<char, usize> = HashMap::new();
        let total_chars = content.len();

        // Count character frequencies
        for ch in content.chars() {
            *char_counts.entry(ch).or_insert(0) += 1;
        }

        // Calculate Shannon entropy: H = -Σ(p(x) * log2(p(x)))
        let mut entropy = 0.0;
        for count in char_counts.values() {
            let probability = *count as f64 / total_chars as f64;
            entropy -= probability * probability.log2();
        }

        entropy
    }

    /// Calculate cyclomatic complexity
    /// CC = 1 + (number of decision points)
    /// Decision points: if, else if, while, for, match, case, &&, ||, ?, catch
    fn calculate_cyclomatic_complexity(content: &str) -> usize {
        let mut complexity = 1; // Base complexity

        // Decision point keywords
        let keywords = [
            "if", "else if", "while", "for", "loop",
            "match", "case", "switch", "catch",
            "&&", "||", "?", "and", "or",
        ];

        // Count occurrences of decision points
        for keyword in &keywords {
            // Use word boundaries for keywords to avoid false positives
            if keyword.chars().all(|c| c.is_alphabetic()) {
                // For alphabetic keywords, match whole words
                for word in content.split_whitespace() {
                    let word_clean = word.trim_matches(|c: char| !c.is_alphabetic());
                    if word_clean == *keyword {
                        complexity += 1;
                    }
                }
            } else {
                // For operators, simple substring match
                complexity += content.matches(keyword).count();
            }
        }

        complexity
    }

    /// Calculate cognitive complexity - measures understandability
    /// Unlike cyclomatic complexity, cognitive complexity:
    /// - Increments for each nesting level (nested code is harder to understand)
    /// - Doesn't increment for linear sequences
    /// - Penalizes deeply nested structures more heavily
    fn calculate_cognitive_complexity(content: &str) -> usize {
        let mut complexity = 0;
        let mut nesting_level = 0;
        let mut in_function = false;

        // Control flow keywords that increment complexity
        let control_keywords = [
            "if", "else if", "elif", "else",
            "while", "for", "loop", "do",
            "match", "case", "switch",
            "catch", "except", "rescue",
            "try", "finally",
        ];

        // Binary logical operators (only increment at same level, not nesting)
        let logical_operators = ["&&", "||", "and", "or"];

        let lines: Vec<&str> = content.lines().collect();

        for line in &lines {
            let trimmed = line.trim();

            // Track nesting level based on braces
            let open_braces = line.matches('{').count();
            let close_braces = line.matches('}').count();

            // Check for function starts
            if trimmed.contains("fn ") || trimmed.contains("function ") || trimmed.contains("def ") {
                in_function = true;
            }

            // Check for control flow keywords
            for keyword in &control_keywords {
                if trimmed.starts_with(keyword) || trimmed.contains(&format!(" {} ", keyword)) {
                    // Cognitive complexity increments by (1 + nesting level)
                    complexity += 1 + nesting_level;
                    break; // Only count once per line
                }
            }

            // Check for logical operators (only increment by 1, not affected by nesting)
            for op in &logical_operators {
                if trimmed.contains(op) {
                    // Each logical operator adds 1 to cognitive complexity
                    complexity += trimmed.matches(op).count();
                }
            }

            // Check for goto, break, continue (each adds 1)
            if trimmed.contains("goto ") || trimmed.contains("break") || trimmed.contains("continue") {
                complexity += 1;
            }

            // Recursion detection (simplified)
            if in_function {
                // Look for common recursion patterns
                if trimmed.contains("return ") && (
                    trimmed.contains("fn(") ||
                    trimmed.contains("function(") ||
                    trimmed.contains("self(")
                ) {
                    complexity += 1;
                }
            }

            // Update nesting level
            if open_braces > close_braces {
                nesting_level += open_braces - close_braces;
            } else if close_braces > open_braces {
                nesting_level = nesting_level.saturating_sub(close_braces - open_braces);
            }
        }

        complexity.max(1) // Minimum complexity is 1
    }

    /// Calculate maximum nesting depth by tracking opening and closing braces
    /// This works for C-style languages (Rust, C, C++, Java, JavaScript, etc.)
    fn calculate_max_nesting_depth(content: &str) -> usize {
        let mut current_depth = 0;
        let mut max_depth = 0;

        for ch in content.chars() {
            match ch {
                '{' => {
                    current_depth += 1;
                    max_depth = max_depth.max(current_depth);
                }
                '}' => {
                    if current_depth > 0 {
                        current_depth -= 1;
                    }
                }
                _ => {}
            }
        }

        max_depth
    }

    /// Calculate function metrics: count and average length
    /// Returns (function_count, avg_function_length)
    fn calculate_function_metrics(content: &str, language: &Language) -> (usize, f64) {
        let patterns = match language.name {
            "Rust" => vec![r"\bfn\s+\w+"],
            "Python" => vec![r"\bdef\s+\w+", r"\basync\s+def\s+\w+"],
            "JavaScript" | "TypeScript" => vec![
                r"\bfunction\s+\w+",
                r"\b\w+\s*=\s*function",
                r"\b\w+\s*=\s*\([^)]*\)\s*=>",
                r"\b\w+\s*=\s*async\s*\([^)]*\)\s*=>",
            ],
            "Java" | "C" | "C++" | "C#" => vec![
                r"\b(public|private|protected|static|void|int|double|float|bool|char|String)\s+\w+\s*\(",
            ],
            "Go" => vec![r"\bfunc\s+\w+"],
            "Ruby" => vec![r"\bdef\s+\w+"],
            "PHP" => vec![r"\bfunction\s+\w+", r"\bpublic\s+function\s+\w+", r"\bprivate\s+function\s+\w+"],
            "Swift" => vec![r"\bfunc\s+\w+"],
            "Kotlin" => vec![r"\bfun\s+\w+"],
            "Dart" => vec![r"\b\w+\s+\w+\s*\(", r"\bvoid\s+\w+\s*\("],
            "Scala" => vec![r"\bdef\s+\w+"],
            "Elixir" => vec![r"\bdef\s+\w+", r"\bdefp\s+\w+"],
            "Haskell" => vec![r"^\w+\s*::"],
            "Clojure" => vec![r"\(defn\s+\w+", r"\(defn-\s+\w+"],
            "Lua" => vec![r"\bfunction\s+\w+", r"\blocal\s+function\s+\w+"],
            "Shell" => vec![r"^\w+\s*\(\s*\)\s*\{", r"\bfunction\s+\w+"],
            "SQL" => vec![r"\bCREATE\s+(OR\s+REPLACE\s+)?FUNCTION\s+\w+", r"\bCREATE\s+(OR\s+REPLACE\s+)?PROCEDURE\s+\w+"],
            _ => vec![],
        };

        if patterns.is_empty() {
            return (0, 0.0);
        }

        let mut function_count = 0;
        let mut function_lengths = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Find all function definitions
        for (i, line) in lines.iter().enumerate() {
            for pattern in &patterns {
                if let Ok(re) = regex::Regex::new(pattern) {
                    if re.is_match(line) {
                        function_count += 1;

                        // Estimate function length by finding the closing brace
                        let func_length = Self::estimate_function_length(&lines, i);
                        function_lengths.push(func_length);
                        break;
                    }
                }
            }
        }

        let avg_length = if function_count > 0 {
            function_lengths.iter().sum::<usize>() as f64 / function_count as f64
        } else {
            0.0
        };

        (function_count, avg_length)
    }

    /// Estimate function length by finding matching braces
    fn estimate_function_length(lines: &[&str], start_line: usize) -> usize {
        let mut brace_count = 0;
        let mut found_opening = false;
        let mut length = 0;

        for line in lines.iter().skip(start_line) {
            length += 1;

            for ch in line.chars() {
                match ch {
                    '{' => {
                        found_opening = true;
                        brace_count += 1;
                    }
                    '}' => {
                        if found_opening {
                            brace_count -= 1;
                            if brace_count == 0 {
                                return length;
                            }
                        }
                    }
                    _ => {}
                }
            }

            // Safety limit: don't search more than 500 lines
            if length > 500 {
                break;
            }
        }

        length
    }

    /// Calculate comment quality heuristics
    /// Returns (quality_score, low_quality_count, high_quality_count)
    /// Quality score ranges from 0.0 (poor) to 1.0 (excellent)
    fn calculate_comment_quality(content: &str, language: &Language) -> (f64, usize, usize) {
        let mut comments = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut in_block_comment = false;
        let mut current_block_comment = String::new();

        // Extract all comments
        for line in &lines {
            let trimmed = line.trim();

            // Handle block comments
            if let Some((start, end)) = language.block_comment {
                if trimmed.starts_with(start) && !in_block_comment {
                    in_block_comment = true;
                    current_block_comment.clear();
                }

                if in_block_comment {
                    // Remove comment markers
                    let mut comment_text = trimmed
                        .trim_start_matches(start)
                        .trim_end_matches(end)
                        .trim()
                        .to_string();

                    // Remove common doc comment markers (///, //!, ***, etc.)
                    comment_text = comment_text
                        .trim_start_matches('*')
                        .trim_start_matches('!')
                        .trim_start_matches('/')
                        .trim()
                        .to_string();

                    if !comment_text.is_empty() {
                        current_block_comment.push_str(&comment_text);
                        current_block_comment.push(' ');
                    }

                    if trimmed.ends_with(end) || trimmed.contains(end) {
                        in_block_comment = false;
                        if !current_block_comment.is_empty() {
                            comments.push(current_block_comment.trim().to_string());
                            current_block_comment.clear();
                        }
                    }
                    continue;
                }
            }

            // Handle line comments
            if let Some(prefix) = language.line_comment {
                if trimmed.starts_with(prefix) {
                    let comment_text = trimmed
                        .trim_start_matches(prefix)
                        .trim_start_matches('/')
                        .trim_start_matches('!')
                        .trim()
                        .to_string();

                    if !comment_text.is_empty() {
                        comments.push(comment_text);
                    }
                }
            }
        }

        if comments.is_empty() {
            return (0.0, 0, 0);
        }

        let mut low_quality_count = 0;
        let mut high_quality_count = 0;

        // Keywords that indicate high-quality explanatory comments
        let high_quality_keywords = [
            "why", "because", "reason", "purpose", "intent", "rationale",
            "workaround", "optimization", "performance", "algorithm",
            "note:", "important:", "warning:", "see:", "reference:",
            "http://", "https://", "issue", "ticket", "jira", "#",
            "example:", "usage:", "parameter", "returns", "throws",
        ];

        // Patterns that indicate low-quality comments
        let noise_patterns = ["====", "----", "****", "////", "####"];
        let obvious_patterns = [
            "increment", "decrement", "set ", "get ", "return",
            "initialize", "create", "declare", "end of", "start of",
        ];

        // Code-like patterns (commented-out code detection)
        let code_patterns = [
            " = ", " == ", " != ", "if ", "else ", "for ", "while ",
            "function ", "def ", "fn ", "class ", "struct ",
            "import ", "require(", "include ", "use ", "package ",
            "const ", "let ", "var ", "int ", "string ", "bool ",
            "{", "}", "()", "[]", "++;", "--;",
        ];

        for comment in &comments {
            let comment_lower = comment.to_lowercase();
            let word_count = comment.split_whitespace().count();

            // Skip very short comments (likely low quality)
            if word_count < 3 {
                low_quality_count += 1;
                continue;
            }

            // Check for noise patterns (borders, dividers)
            if noise_patterns.iter().any(|p| comment.contains(p)) {
                low_quality_count += 1;
                continue;
            }

            // Check for commented-out code
            let code_pattern_matches = code_patterns.iter()
                .filter(|p| comment.contains(*p))
                .count();

            if code_pattern_matches >= 2 {
                low_quality_count += 1;
                continue;
            }

            // Check for obvious comments (states the obvious from code)
            let has_obvious = obvious_patterns.iter()
                .any(|p| comment_lower.contains(p) && word_count < 6);

            if has_obvious {
                low_quality_count += 1;
                continue;
            }

            // Check for high-quality indicators
            let has_high_quality = high_quality_keywords.iter()
                .any(|k| comment_lower.contains(k));

            // Long, explanatory comments (> 10 words) are usually good
            if has_high_quality || word_count > 10 {
                high_quality_count += 1;
            }
        }

        // Calculate quality score
        let total_comments = comments.len();
        let neutral_comments = total_comments - low_quality_count - high_quality_count;

        // Score formula:
        // - High quality: +1.0
        // - Neutral: +0.5
        // - Low quality: +0.0
        let score = if total_comments > 0 {
            (high_quality_count as f64 + neutral_comments as f64 * 0.5) / total_comments as f64
        } else {
            0.0
        };

        (score, low_quality_count, high_quality_count)
    }

    /// Detect code smells in the content
    /// Configurable thresholds for different smell types
    fn detect_code_smells(
        content: &str,
        language: &Language,
        _avg_function_length: f64,
        _max_nesting_depth: usize,
    ) -> Vec<CodeSmell> {
        let mut smells = Vec::new();

        // Thresholds (configurable in future)
        const LONG_FUNCTION_THRESHOLD: usize = 50;  // lines
        const LONG_PARAM_LIST_THRESHOLD: usize = 5; // parameters
        const DEEP_NESTING_THRESHOLD: usize = 4;    // nesting levels

        let lines: Vec<&str> = content.lines().collect();

        // Track current function for multi-line analysis
        let mut current_function_start: Option<usize> = None;
        let mut current_function_name = String::new();
        let mut current_function_lines = 0;
        let mut brace_depth = 0;
        let mut nesting_stack: Vec<usize> = Vec::new();

        // Track multi-line parameter lists
        let mut in_param_list = false;
        let mut param_list_start_line: Option<usize> = None;
        let mut param_accumulator = String::new();

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let line_number = line_num + 1;

            // Skip comments and blank lines
            if trimmed.is_empty() {
                if current_function_start.is_some() {
                    current_function_lines += 1;
                }
                continue;
            }

            // Check for block comments (simple check)
            if let Some((start, _)) = language.block_comment {
                if trimmed.starts_with(start) {
                    continue;
                }
            }
            if let Some(prefix) = language.line_comment {
                if trimmed.starts_with(prefix) {
                    continue;
                }
            }

            // 1. DETECT LONG FUNCTIONS
            // Look for function declarations
            let function_keywords = ["fn ", "func ", "function ", "def ", "sub ", "proc ", "fun "];
            let is_function_start = function_keywords.iter().any(|kw| {
                trimmed.starts_with(kw) || trimmed.contains(&format!(" {}", kw))
            });

            if is_function_start {
                // Check if previous function was long
                if let Some(start_line) = current_function_start {
                    if current_function_lines > LONG_FUNCTION_THRESHOLD {
                        smells.push(CodeSmell {
                            smell_type: CodeSmellType::LongFunction,
                            line: start_line,
                            description: format!(
                                "Function '{}' is {} lines long (threshold: {})",
                                current_function_name, current_function_lines, LONG_FUNCTION_THRESHOLD
                            ),
                            severity: if current_function_lines > LONG_FUNCTION_THRESHOLD * 2 {
                                SmellSeverity::Major
                            } else if current_function_lines > LONG_FUNCTION_THRESHOLD + 20 {
                                SmellSeverity::Moderate
                            } else {
                                SmellSeverity::Minor
                            },
                        });
                    }
                }

                // Start tracking new function
                current_function_start = Some(line_number);
                current_function_lines = 1;

                // Extract function name (simplified)
                current_function_name = trimmed
                    .split(|c: char| c == '(' || c == ' ' || c == '{')
                    .nth(1)
                    .unwrap_or("unknown")
                    .trim()
                    .to_string();

                // 2. DETECT LONG PARAMETER LISTS
                // Start tracking parameters
                if let Some(param_start) = trimmed.find('(') {
                    in_param_list = true;
                    param_list_start_line = Some(line_number);

                    // Check if it closes on the same line
                    if let Some(param_end) = trimmed.find(')') {
                        let params = &trimmed[param_start + 1..param_end];
                        let param_count = if params.trim().is_empty() {
                            0
                        } else {
                            params.matches(',').count() + 1
                        };

                        if param_count > LONG_PARAM_LIST_THRESHOLD {
                            smells.push(CodeSmell {
                                smell_type: CodeSmellType::LongParameterList,
                                line: line_number,
                                description: format!(
                                    "Function has {} parameters (threshold: {})",
                                    param_count, LONG_PARAM_LIST_THRESHOLD
                                ),
                                severity: if param_count > LONG_PARAM_LIST_THRESHOLD * 2 {
                                    SmellSeverity::Major
                                } else if param_count > LONG_PARAM_LIST_THRESHOLD + 2 {
                                    SmellSeverity::Moderate
                                } else {
                                    SmellSeverity::Minor
                                },
                            });
                        }
                        in_param_list = false;
                        param_list_start_line = None;
                        param_accumulator.clear();
                    } else {
                        // Multi-line parameter list - start accumulating
                        param_accumulator = trimmed[param_start + 1..].to_string();
                    }
                }
            } else {
                // Continue tracking multi-line parameter lists
                if in_param_list {
                    if let Some(param_end) = trimmed.find(')') {
                        // Parameter list ends on this line
                        param_accumulator.push(' ');
                        param_accumulator.push_str(&trimmed[..param_end]);

                        // Count parameters
                        let param_count = if param_accumulator.trim().is_empty() {
                            0
                        } else {
                            param_accumulator.matches(',').count() + 1
                        };

                        if param_count > LONG_PARAM_LIST_THRESHOLD {
                            smells.push(CodeSmell {
                                smell_type: CodeSmellType::LongParameterList,
                                line: param_list_start_line.unwrap_or(line_number),
                                description: format!(
                                    "Function has {} parameters (threshold: {})",
                                    param_count, LONG_PARAM_LIST_THRESHOLD
                                ),
                                severity: if param_count > LONG_PARAM_LIST_THRESHOLD * 2 {
                                    SmellSeverity::Major
                                } else if param_count > LONG_PARAM_LIST_THRESHOLD + 2 {
                                    SmellSeverity::Moderate
                                } else {
                                    SmellSeverity::Minor
                                },
                            });
                        }

                        in_param_list = false;
                        param_list_start_line = None;
                        param_accumulator.clear();
                    } else {
                        // Still in parameter list, keep accumulating
                        param_accumulator.push(' ');
                        param_accumulator.push_str(trimmed);
                    }
                }

                if current_function_start.is_some() {
                    current_function_lines += 1;
                }
            }

            // Track braces for function end detection
            let open_braces = trimmed.matches('{').count();
            let close_braces = trimmed.matches('}').count();

            if open_braces > 0 {
                brace_depth += open_braces;
            }
            if close_braces > 0 {
                brace_depth = brace_depth.saturating_sub(close_braces);

                // Function ended
                if brace_depth == 0 && current_function_start.is_some() {
                    if current_function_lines > LONG_FUNCTION_THRESHOLD {
                        smells.push(CodeSmell {
                            smell_type: CodeSmellType::LongFunction,
                            line: current_function_start.unwrap(),
                            description: format!(
                                "Function '{}' is {} lines long (threshold: {})",
                                current_function_name, current_function_lines, LONG_FUNCTION_THRESHOLD
                            ),
                            severity: if current_function_lines > LONG_FUNCTION_THRESHOLD * 2 {
                                SmellSeverity::Major
                            } else if current_function_lines > LONG_FUNCTION_THRESHOLD + 20 {
                                SmellSeverity::Moderate
                            } else {
                                SmellSeverity::Minor
                            },
                        });
                    }
                    current_function_start = None;
                    current_function_name.clear();
                    current_function_lines = 0;
                }
            }

            // 3. DETECT DEEP NESTING
            // Count current nesting level
            let mut current_nesting: usize = 0;
            for c in trimmed.chars() {
                if c == '{' {
                    current_nesting += 1;
                    nesting_stack.push(line_number);
                } else if c == '}' && !nesting_stack.is_empty() {
                    current_nesting = current_nesting.saturating_sub(1);
                    nesting_stack.pop();
                }
            }

            if nesting_stack.len() > DEEP_NESTING_THRESHOLD {
                smells.push(CodeSmell {
                    smell_type: CodeSmellType::DeepNesting,
                    line: line_number,
                    description: format!(
                        "Deep nesting detected: {} levels (threshold: {})",
                        nesting_stack.len(), DEEP_NESTING_THRESHOLD
                    ),
                    severity: if nesting_stack.len() > DEEP_NESTING_THRESHOLD + 3 {
                        SmellSeverity::Major
                    } else if nesting_stack.len() > DEEP_NESTING_THRESHOLD + 1 {
                        SmellSeverity::Moderate
                    } else {
                        SmellSeverity::Minor
                    },
                });
            }

            // 4. DETECT MAGIC NUMBERS
            // Find numeric literals that aren't 0, 1, -1, 2, 10, 100, 1000 (common non-magic)
            let common_numbers = ["0", "1", "-1", "2", "10", "100", "1000"];

            // Simple regex-like pattern: look for standalone numbers
            let tokens: Vec<&str> = trimmed.split(|c: char| !c.is_numeric() && c != '-' && c != '.').collect();

            for token in tokens {
                if token.is_empty() {
                    continue;
                }

                // Check if it's a number
                if let Ok(_num) = token.parse::<f64>() {
                    // Skip common non-magic numbers
                    if common_numbers.contains(&token) {
                        continue;
                    }

                    // Skip if it's part of a constant declaration (simple heuristic)
                    let upper_line = line.to_uppercase();
                    if upper_line.contains("CONST") || upper_line.contains("FINAL") ||
                       upper_line.contains("STATIC") || upper_line.contains("LET ") ||
                       trimmed.chars().next().map_or(false, |c| c.is_uppercase()) {
                        continue;
                    }

                    smells.push(CodeSmell {
                        smell_type: CodeSmellType::MagicNumber,
                        line: line_number,
                        description: format!(
                            "Magic number '{}' should be replaced with a named constant",
                            token
                        ),
                        severity: SmellSeverity::Minor,
                    });
                }
            }
        }

        // Check last function if file ends
        if let Some(start_line) = current_function_start {
            if current_function_lines > LONG_FUNCTION_THRESHOLD {
                smells.push(CodeSmell {
                    smell_type: CodeSmellType::LongFunction,
                    line: start_line,
                    description: format!(
                        "Function '{}' is {} lines long (threshold: {})",
                        current_function_name, current_function_lines, LONG_FUNCTION_THRESHOLD
                    ),
                    severity: if current_function_lines > LONG_FUNCTION_THRESHOLD * 2 {
                        SmellSeverity::Major
                    } else if current_function_lines > LONG_FUNCTION_THRESHOLD + 20 {
                        SmellSeverity::Moderate
                    } else {
                        SmellSeverity::Minor
                    },
                });
            }
        }

        smells
    }

    /// Calculate Halstead complexity metrics
    ///
    /// Halstead metrics measure program complexity based on operators and operands.
    ///
    /// Returns: (volume, difficulty, effort)
    /// - Volume: Program size (bits needed to represent the program)
    /// - Difficulty: How difficult the program is to write/understand
    /// - Effort: Mental effort required to develop/understand
    fn calculate_halstead_metrics(content: &str, language: &Language) -> (f64, f64, f64) {
        use std::collections::HashSet;

        let mut operators = HashSet::new();
        let mut operands = HashSet::new();
        let mut total_operators = 0;
        let mut total_operands = 0;

        // Language-specific operator patterns based on language name
        let op_patterns = match language.name {
            "Rust" => vec![
                "=>", "->", "::", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=",
                "==", "!=", "<=", ">=", "&&", "||", "<<", ">>",
                "+", "-", "*", "/", "%", "&", "|", "^", "!", "=", "<", ">",
                "?", ":", ";", ",", ".", "(", ")", "[", "]", "{", "}",
                "fn", "let", "mut", "if", "else", "for", "while", "loop", "match", "return",
            ],
            "Python" => vec![
                "+=", "-=", "*=", "/=", "%=", "**=", "//=", "&=", "|=", "^=", "<<=", ">>=",
                "==", "!=", "<=", ">=", "and", "or", "not", "in", "is",
                "+", "-", "*", "/", "%", "**", "//", "&", "|", "^", "~", "=", "<", ">",
                ":", ";", ",", ".", "(", ")", "[", "]", "{", "}",
                "def", "class", "if", "elif", "else", "for", "while", "return", "yield",
            ],
            "JavaScript" | "TypeScript" => vec![
                "===", "!==", "==", "!=", "<=", ">=", "&&", "||", "++", "--",
                "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", ">>>=",
                "+", "-", "*", "/", "%", "&", "|", "^", "!", "=", "<", ">", "?",
                ":", ";", ",", ".", "(", ")", "[", "]", "{", "}",
                "function", "const", "let", "var", "if", "else", "for", "while", "return",
            ],
            "Go" => vec![
                ":=", "==", "!=", "<=", ">=", "&&", "||", "++", "--", "<-",
                "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", "&^=",
                "+", "-", "*", "/", "%", "&", "|", "^", "!", "=", "<", ">",
                ":", ";", ",", ".", "(", ")", "[", "]", "{", "}",
                "func", "if", "else", "for", "switch", "case", "return", "go", "defer",
            ],
            "Java" | "Kotlin" | "Scala" => vec![
                "==", "!=", "<=", ">=", "&&", "||", "++", "--",
                "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", ">>>=",
                "+", "-", "*", "/", "%", "&", "|", "^", "!", "=", "<", ">", "?",
                ":", ";", ",", ".", "(", ")", "[", "]", "{", "}",
                "class", "if", "else", "for", "while", "return", "new", "void",
            ],
            _ => vec![
                "==", "!=", "<=", ">=", "&&", "||",
                "+", "-", "*", "/", "%", "&", "|", "^", "!", "=", "<", ">",
                ":", ";", ",", ".", "(", ")", "[", "]", "{", "}",
                "if", "else", "for", "while", "return",
            ],
        };

        // Simple tokenization
        let tokens: Vec<&str> = content.split_whitespace().collect();

        for token in tokens {
            let mut found_op = false;

            // Check for operators
            for op in &op_patterns {
                if token.contains(op) {
                    operators.insert(op.to_string());
                    total_operators += 1;
                    found_op = true;
                    break;
                }
            }

            // If not an operator, count as operand (identifier, number, string)
            if !found_op && !token.is_empty() {
                // Filter out pure punctuation and very short tokens
                if token.chars().any(|c| c.is_alphanumeric()) && token.len() > 1 {
                    operands.insert(token.to_string());
                    total_operands += 1;
                }
            }
        }

        // Halstead metrics formulas
        let n1 = operators.len() as f64; // Unique operators
        let n2 = operands.len() as f64;  // Unique operands
        let N1 = total_operators as f64; // Total operators
        let N2 = total_operands as f64;  // Total operands

        // Avoid division by zero
        if n1 == 0.0 || n2 == 0.0 || N1 == 0.0 || N2 == 0.0 {
            return (0.0, 0.0, 0.0);
        }

        // Program vocabulary
        let vocabulary = n1 + n2;

        // Program length
        let length = N1 + N2;

        // Volume = length * log2(vocabulary)
        let volume = length * vocabulary.log2();

        // Difficulty = (n1 / 2) * (N2 / n2)
        let difficulty = (n1 / 2.0) * (N2 / n2);

        // Effort = Difficulty * Volume
        let effort = difficulty * volume;

        (volume, difficulty, effort)
    }

    /// Calculate Maintainability Index
    ///
    /// The Maintainability Index (MI) is a metric that estimates how maintainable the code is.
    /// Formula: MI = max(0, (171 - 5.2 * ln(HV) - 0.23 * CC - 16.2 * ln(LOC)) * 100 / 171)
    ///
    /// Where:
    /// - HV is Halstead Volume
    /// - CC is Cyclomatic Complexity
    /// - LOC is Lines of Code
    ///
    /// Returns a value from 0-100 where:
    /// - 85-100: Highly maintainable (Grade A)
    /// - 65-84:  Moderately maintainable (Grade B)
    /// - 50-64:  Acceptable maintainability (Grade C)
    /// - 25-49:  Difficult to maintain (Grade D)
    /// - 0-24:   Very difficult to maintain (Grade F)
    fn calculate_maintainability_index(
        code_lines: usize,
        cyclomatic_complexity: usize,
        halstead_volume: f64,
        comment_ratio: f64,
    ) -> f64 {
        // Avoid log of zero or negative
        let loc = code_lines.max(1) as f64;
        let cc = cyclomatic_complexity.max(1) as f64;
        let hv = halstead_volume.max(1.0);

        // Calculate raw MI using the standard formula
        // MI = 171 - 5.2 * ln(HV) - 0.23 * CC - 16.2 * ln(LOC)
        let raw_mi = 171.0
            - 5.2 * hv.ln()
            - 0.23 * cc
            - 16.2 * loc.ln();

        // Apply comment ratio bonus (up to +10 points for well-commented code)
        let comment_bonus = comment_ratio * 10.0;

        // Normalize to 0-100 scale and ensure it's positive
        let mi = (raw_mi + comment_bonus).max(0.0).min(171.0);

        // Scale to 0-100
        (mi * 100.0 / 171.0).min(100.0).max(0.0)
    }

    /// Get letter grade for Maintainability Index
    pub fn get_maintainability_grade(mi: f64) -> &'static str {
        match mi {
            mi if mi >= 85.0 => "A",
            mi if mi >= 65.0 => "B",
            mi if mi >= 50.0 => "C",
            mi if mi >= 25.0 => "D",
            _ => "F",
        }
    }

    /// Get comment ratio (0.0 to 1.0)
    pub fn comment_ratio(&self) -> f64 {
        if self.total_lines == 0 {
            return 0.0;
        }
        self.comment_lines as f64 / self.total_lines as f64
    }

    /// Get code ratio (0.0 to 1.0)
    pub fn code_ratio(&self) -> f64 {
        if self.total_lines == 0 {
            return 0.0;
        }
        self.code_lines as f64 / self.total_lines as f64
    }
}

impl ProjectMetrics {
    /// Create a new empty project metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Add file metrics to the project
    pub fn add_file(&mut self, language_name: &str, metrics: &FileMetrics) {
        self.total_files += 1;

        let lang_metrics = self.by_language
            .entry(language_name.to_string())
            .or_insert_with(LanguageMetrics::default);

        lang_metrics.file_count += 1;
        lang_metrics.total_lines += metrics.total_lines;
        lang_metrics.code_lines += metrics.code_lines;
        lang_metrics.comment_lines += metrics.comment_lines;
        lang_metrics.blank_lines += metrics.blank_lines;
        lang_metrics.todo_count += metrics.todo_count;
        lang_metrics.fixme_count += metrics.fixme_count;
        lang_metrics.hack_count += metrics.hack_count;
        lang_metrics.total_functions += metrics.function_count;
        lang_metrics.low_quality_comments += metrics.low_quality_comments;
        lang_metrics.high_quality_comments += metrics.high_quality_comments;

        // Aggregate code smells by type
        for smell in &metrics.code_smells {
            match smell.smell_type {
                CodeSmellType::LongFunction => lang_metrics.long_functions += 1,
                CodeSmellType::LongParameterList => lang_metrics.long_parameter_lists += 1,
                CodeSmellType::DeepNesting => lang_metrics.deep_nesting_instances += 1,
                CodeSmellType::MagicNumber => lang_metrics.magic_numbers += 1,
            }
        }

        // Calculate running averages
        let prev_total = (lang_metrics.file_count - 1) as f64;
        lang_metrics.avg_entropy =
            (lang_metrics.avg_entropy * prev_total + metrics.entropy) / lang_metrics.file_count as f64;
        lang_metrics.avg_cyclomatic =
            (lang_metrics.avg_cyclomatic * prev_total + metrics.cyclomatic_complexity as f64) / lang_metrics.file_count as f64;
        lang_metrics.avg_cognitive =
            (lang_metrics.avg_cognitive * prev_total + metrics.cognitive_complexity as f64) / lang_metrics.file_count as f64;
        lang_metrics.avg_nesting_depth =
            (lang_metrics.avg_nesting_depth * prev_total + metrics.max_nesting_depth as f64) / lang_metrics.file_count as f64;
        lang_metrics.avg_comment_quality =
            (lang_metrics.avg_comment_quality * prev_total + metrics.comment_quality_score) / lang_metrics.file_count as f64;
        lang_metrics.avg_maintainability_index =
            (lang_metrics.avg_maintainability_index * prev_total + metrics.maintainability_index) / lang_metrics.file_count as f64;

        // For function length, weight by number of functions
        let prev_func_count = lang_metrics.total_functions - metrics.function_count;
        if lang_metrics.total_functions > 0 {
            lang_metrics.avg_function_length =
                (lang_metrics.avg_function_length * prev_func_count as f64 + metrics.avg_function_length * metrics.function_count as f64) / lang_metrics.total_functions as f64;
        }
    }

    /// Get total lines across all files
    pub fn total_lines(&self) -> usize {
        self.by_language.values().map(|m| m.total_lines).sum()
    }

    /// Get total code lines across all files
    pub fn total_code_lines(&self) -> usize {
        self.by_language.values().map(|m| m.code_lines).sum()
    }

    /// Get total comment lines across all files
    pub fn total_comment_lines(&self) -> usize {
        self.by_language.values().map(|m| m.comment_lines).sum()
    }

    /// Get average entropy across all files
    pub fn average_entropy(&self) -> f64 {
        if self.by_language.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.by_language.values()
            .map(|m| m.avg_entropy * m.file_count as f64)
            .sum();

        sum / self.total_files as f64
    }

    /// Get total TODO markers across all files
    pub fn total_todos(&self) -> usize {
        self.by_language.values().map(|m| m.todo_count).sum()
    }

    /// Get total FIXME markers across all files
    pub fn total_fixmes(&self) -> usize {
        self.by_language.values().map(|m| m.fixme_count).sum()
    }

    /// Get total HACK markers across all files
    pub fn total_hacks(&self) -> usize {
        self.by_language.values().map(|m| m.hack_count).sum()
    }

    /// Get average cyclomatic complexity across all files
    pub fn avg_complexity(&self) -> f64 {
        if self.by_language.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.by_language.values()
            .map(|m| m.avg_cyclomatic * m.file_count as f64)
            .sum();

        sum / self.total_files as f64
    }

    /// Get average maintainability index across all files
    pub fn avg_maintainability(&self) -> f64 {
        if self.by_language.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.by_language.values()
            .map(|m| m.avg_maintainability_index * m.file_count as f64)
            .sum();

        sum / self.total_files as f64
    }

    /// Get average cyclomatic complexity across all files
    pub fn average_cyclomatic(&self) -> f64 {
        if self.by_language.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.by_language.values()
            .map(|m| m.avg_cyclomatic * m.file_count as f64)
            .sum();

        sum / self.total_files as f64
    }

    /// Get average cognitive complexity across all files
    pub fn average_cognitive(&self) -> f64 {
        if self.by_language.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.by_language.values()
            .map(|m| m.avg_cognitive * m.file_count as f64)
            .sum();

        sum / self.total_files as f64
    }

    /// Get average nesting depth across all files
    pub fn average_nesting_depth(&self) -> f64 {
        if self.by_language.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.by_language.values()
            .map(|m| m.avg_nesting_depth * m.file_count as f64)
            .sum();

        sum / self.total_files as f64
    }

    /// Get total function count across all files
    pub fn total_functions(&self) -> usize {
        self.by_language.values().map(|m| m.total_functions).sum()
    }

    /// Get average function length across all files
    pub fn average_function_length(&self) -> f64 {
        let total_funcs = self.total_functions();
        if total_funcs == 0 {
            return 0.0;
        }

        let sum: f64 = self.by_language.values()
            .map(|m| m.avg_function_length * m.total_functions as f64)
            .sum();

        sum / total_funcs as f64
    }

    /// Get average comment quality across all files
    pub fn average_comment_quality(&self) -> f64 {
        if self.by_language.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.by_language.values()
            .map(|m| m.avg_comment_quality * m.file_count as f64)
            .sum();

        sum / self.total_files as f64
    }

    /// Get total low-quality comments across all files
    pub fn total_low_quality_comments(&self) -> usize {
        self.by_language.values().map(|m| m.low_quality_comments).sum()
    }

    /// Get total high-quality comments across all files
    pub fn total_high_quality_comments(&self) -> usize {
        self.by_language.values().map(|m| m.high_quality_comments).sum()
    }

    /// Get total long functions across all files
    pub fn total_long_functions(&self) -> usize {
        self.by_language.values().map(|m| m.long_functions).sum()
    }

    /// Get total long parameter lists across all files
    pub fn total_long_parameter_lists(&self) -> usize {
        self.by_language.values().map(|m| m.long_parameter_lists).sum()
    }

    /// Get total deep nesting instances across all files
    pub fn total_deep_nesting(&self) -> usize {
        self.by_language.values().map(|m| m.deep_nesting_instances).sum()
    }

    /// Get total magic numbers across all files
    pub fn total_magic_numbers(&self) -> usize {
        self.by_language.values().map(|m| m.magic_numbers).sum()
    }

    /// Get total code smells across all files
    pub fn total_code_smells(&self) -> usize {
        self.total_long_functions() +
        self.total_long_parameter_lists() +
        self.total_deep_nesting() +
        self.total_magic_numbers()
    }

    /// Get average Maintainability Index across all files
    pub fn average_maintainability_index(&self) -> f64 {
        if self.by_language.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.by_language.values()
            .map(|m| m.avg_maintainability_index * m.file_count as f64)
            .sum();

        sum / self.total_files as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_calculation() {
        // Highly repetitive content = low entropy
        let low_entropy = FileMetrics::calculate_entropy("aaaaaaaaaa");

        // More varied content = higher entropy
        let high_entropy = FileMetrics::calculate_entropy("abcdefghij");

        assert!(high_entropy > low_entropy);
    }

    #[test]
    fn test_rust_metrics() -> Result<()> {
        let rust_lang = crate::language::Language::from_extension("rs").unwrap();
        let code = r#"
// This is a comment
fn main() {
    println!("Hello");
}

/* Block comment
   spanning lines */
"#;

        let metrics = FileMetrics::analyze_content(code, rust_lang)?;

        assert!(metrics.code_lines > 0);
        assert!(metrics.comment_lines > 0);
        assert!(metrics.blank_lines > 0);
        assert!(metrics.entropy > 0.0);

        Ok(())
    }

    #[test]
    fn test_project_metrics() -> Result<()> {
        let mut project = ProjectMetrics::new();
        let rust_lang = crate::language::Language::from_extension("rs").unwrap();

        let metrics1 = FileMetrics::analyze_content("fn test() {}", rust_lang)?;
        let metrics2 = FileMetrics::analyze_content("fn test2() {}", rust_lang)?;

        project.add_file("Rust", &metrics1);
        project.add_file("Rust", &metrics2);

        assert_eq!(project.total_files, 2);
        assert_eq!(project.by_language.get("Rust").unwrap().file_count, 2);

        Ok(())
    }

    #[test]
    fn test_cyclomatic_complexity() {
        // Simple function with no branches = complexity 1
        let simple = "fn test() { return 42; }";
        assert_eq!(FileMetrics::calculate_cyclomatic_complexity(simple), 1);

        // Function with one if statement = complexity 2
        let with_if = "fn test() { if x > 0 { return 1; } return 0; }";
        assert!(FileMetrics::calculate_cyclomatic_complexity(with_if) >= 2);

        // Function with if and while = complexity 3+
        let with_multiple = "fn test() { if x > 0 { while y < 10 { y += 1; } } }";
        assert!(FileMetrics::calculate_cyclomatic_complexity(with_multiple) >= 3);

        // Function with boolean operators
        let with_operators = "fn test() { if x > 0 && y < 10 || z == 5 { return 1; } }";
        assert!(FileMetrics::calculate_cyclomatic_complexity(with_operators) >= 4);
    }

    #[test]
    fn test_nesting_depth() {
        // No nesting = depth 0
        let no_nesting = "fn test() return 42;";
        assert_eq!(FileMetrics::calculate_max_nesting_depth(no_nesting), 0);

        // Single level = depth 1
        let single_level = "fn test() { return 42; }";
        assert_eq!(FileMetrics::calculate_max_nesting_depth(single_level), 1);

        // Two levels = depth 2
        let two_levels = "fn test() { if x > 0 { return 1; } }";
        assert_eq!(FileMetrics::calculate_max_nesting_depth(two_levels), 2);

        // Three levels = depth 3
        let three_levels = "fn test() { if x > 0 { while y < 10 { y += 1; } } }";
        assert_eq!(FileMetrics::calculate_max_nesting_depth(three_levels), 3);

        // Complex nesting
        let complex = "fn test() { if a { if b { if c { x; } } } }";
        assert_eq!(FileMetrics::calculate_max_nesting_depth(complex), 4);
    }
}
