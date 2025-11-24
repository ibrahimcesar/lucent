# Custom Rules System - Design Document

## Overview

The custom rules system allows users to define their own code quality rules beyond the built-in metrics. This is a Phase 3 feature that extends Lucent's analysis capabilities.

## Architecture

### 1. Rule Definition

Custom rules are defined in `.lucent.toml` under a `[[rules]]` section:

```toml
[[rules]]
name = "no-unwrap-in-production"
description = "Disallow .unwrap() calls in production code"
severity = "error"  # error | warning | info
pattern = '\\.unwrap\\('
file_patterns = ["src/**/*.rs"]
exclude_patterns = ["src/**/tests/**"]

[[rules]]
name = "require-error-context"
description = "Ensure all error returns have context"
severity = "warning"
language = "Rust"
ast_matcher = "Result::Err without .context()"
```

### 2. Rule Types

#### Pattern-Based Rules
Simple regex matching against file content:
```toml
[[rules]]
name = "no-console-log"
pattern = 'console\\.log\\('
language = "JavaScript"
severity = "warning"
```

#### AST-Based Rules (Advanced)
Rules that inspect the Abstract Syntax Tree:
```toml
[[rules]]
name = "max-match-arms"
type = "ast"
language = "Rust"
max_match_arms = 10
severity = "warning"
```

#### Metric-Based Rules
Custom thresholds for specific patterns:
```toml
[[rules]]
name = "test-coverage-threshold"
type = "metric"
metric = "test_coverage"
min_value = 80
per_file = true
severity = "error"
```

### 3. Rule Engine

```rust
pub trait Rule {
    /// Rule unique identifier
    fn name(&self) -> &str;

    /// Rule description
    fn description(&self) -> &str;

    /// Rule severity level
    fn severity(&self) -> Severity;

    /// Check if rule applies to this file
    fn applies_to(&self, file: &Path, language: &Language) -> bool;

    /// Analyze file and return violations
    fn check(&self, file: &Path, content: &str, ast: Option<&Ast>) -> Vec<Violation>;
}

pub struct Violation {
    pub rule: String,
    pub severity: Severity,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub suggestion: Option<String>,
}

pub enum Severity {
    Error,
    Warning,
    Info,
}
```

### 4. Integration Points

#### With Scanner
```rust
// In Scanner::scan()
let files = scanner.scan()?;
let custom_rules = config.load_custom_rules()?;

for file in files {
    // Built-in metrics
    let metrics = FileMetrics::analyze(&file.path, file.language)?;

    // Custom rules
    let violations = custom_rules.check_file(&file)?;

    // Combine results
    metrics.add_violations(violations);
}
```

#### With Quality Gates
```rust
// In threshold validation
if config.quality_gates.enforce_custom_rules {
    for violation in custom_violations {
        if violation.severity == Severity::Error {
            // Fail build
            return Err(QualityGateFailure::CustomRule(violation));
        }
    }
}
```

## Configuration Schema

### Full Example

```toml
[rules_config]
enabled = true
fail_on_error = true
fail_on_warning = false

[[rules]]
name = "no-panic-in-lib"
description = "Disallow panic! in library code"
type = "pattern"
pattern = 'panic!\\('
file_patterns = ["src/lib/**/*.rs", "src/*/mod.rs"]
exclude_patterns = ["**/tests/**"]
severity = "error"

[[rules]]
name = "require-doc-comments"
description = "Public functions must have documentation"
type = "ast"
language = "Rust"
requires = "pub fn without doc comment"
severity = "warning"

[[rules]]
name = "max-dependencies"
description = "Limit number of imports per file"
type = "metric"
metric = "import_count"
max_value = 20
severity = "warning"

[[rules]]
name = "naming-convention"
description = "Enforce snake_case for functions"
type = "lint"
language = "Rust"
check = "function_names"
pattern = "^[a-z][a-z0-9_]*$"
severity = "info"
```

## Implementation Plan

### Phase 3.1: Pattern Rules (Basic)
1. Add `rules` section to config schema
2. Implement `PatternRule` struct
3. Integrate with scanner
4. Add violation reporting

```rust
// src/rules/pattern.rs
pub struct PatternRule {
    name: String,
    description: String,
    pattern: Regex,
    severity: Severity,
    file_patterns: Vec<String>,
    exclude_patterns: Vec<String>,
}

impl Rule for PatternRule {
    fn check(&self, file: &Path, content: &str, _ast: Option<&Ast>) -> Vec<Violation> {
        let mut violations = vec![];

        for (line_num, line) in content.lines().enumerate() {
            if self.pattern.is_match(line) {
                violations.push(Violation {
                    rule: self.name.clone(),
                    severity: self.severity,
                    message: self.description.clone(),
                    line: line_num + 1,
                    column: 0,
                    suggestion: None,
                });
            }
        }

        violations
    }
}
```

### Phase 3.2: AST Rules (Advanced)
1. Add AST parsing per language
2. Implement tree-sitter integration
3. Create AST matcher DSL
4. Add semantic analysis

```rust
// src/rules/ast.rs
pub struct AstRule {
    name: String,
    description: String,
    matcher: AstMatcher,
    severity: Severity,
}

pub enum AstMatcher {
    FunctionWithoutDocs,
    UnhandledResult,
    DeepNesting(usize),
    LongParameterList(usize),
    ComplexCondition(usize),
}
```

### Phase 3.3: Metric Rules
1. Extract metrics from analysis
2. Allow custom thresholds
3. Per-file vs per-project rules
4. Trend-based rules

```rust
// src/rules/metric.rs
pub struct MetricRule {
    name: String,
    metric: MetricType,
    threshold: Threshold,
    scope: Scope,
}

pub enum MetricType {
    Complexity,
    TestCoverage,
    ImportCount,
    FileSize,
    Custom(String),
}

pub enum Threshold {
    Min(f64),
    Max(f64),
    Range(f64, f64),
}

pub enum Scope {
    PerFile,
    PerModule,
    Global,
}
```

## Output Format

### CLI Output
```
⚠️ Custom Rule Violations

  examples/main.rs:42:5
    ✗ no-unwrap-in-production [ERROR]
      Disallow .unwrap() calls in production code

      41 | fn process_data(input: &str) -> Result<Data> {
      42 |     let data = input.parse().unwrap();
         |                             ^^^^^^^^^
      43 |     Ok(data)

      Suggestion: Use `?` operator or provide error context

  src/lib.rs:15:1
    ⚠ require-doc-comments [WARNING]
      Public functions must have documentation

      14 |
      15 | pub fn calculate(x: i32) -> i32 {
         | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      16 |     x * 2

      Suggestion: Add /// documentation comment
```

### JSON Output
```json
{
  "custom_violations": [
    {
      "rule": "no-unwrap-in-production",
      "severity": "error",
      "file": "examples/main.rs",
      "line": 42,
      "column": 5,
      "message": "Disallow .unwrap() calls in production code",
      "suggestion": "Use `?` operator or provide error context"
    }
  ],
  "custom_rules_summary": {
    "total_violations": 12,
    "errors": 3,
    "warnings": 8,
    "info": 1
  }
}
```

## Plugin API

Allow users to write custom rules as plugins:

```rust
// user-rules/src/lib.rs
use lucent_plugin::*;

#[derive(LucentPlugin)]
pub struct NoTodoRule {
    severity: Severity,
}

impl Rule for NoTodoRule {
    fn name(&self) -> &str {
        "no-todo-comments"
    }

    fn description(&self) -> &str {
        "Disallow TODO comments in production code"
    }

    fn check(&self, file: &Path, content: &str, _ast: Option<&Ast>) -> Vec<Violation> {
        content.lines()
            .enumerate()
            .filter_map(|(line_num, line)| {
                if line.to_uppercase().contains("TODO") {
                    Some(Violation {
                        rule: self.name().to_string(),
                        severity: self.severity,
                        message: format!("TODO comment found on line {}", line_num + 1),
                        line: line_num + 1,
                        column: line.find("TODO").unwrap(),
                        suggestion: Some("Complete the TODO or create an issue".to_string()),
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

plugin_entry!(NoTodoRule);
```

Configuration:
```toml
[[plugins]]
name = "user-rules"
path = "./user-rules/target/release/libuser_rules.so"

[plugins.user-rules.config]
severity = "warning"
```

## Testing Custom Rules

```bash
# Test custom rules
lucent src/ --config .lucent.toml --rules-only

# Validate rules configuration
lucent rules validate

# List active rules
lucent rules list

# Test specific rule
lucent rules test no-unwrap-in-production src/main.rs
```

## Future Enhancements

1. **Rule Templates**: Pre-built rule sets
   ```toml
   [rules]
   import_preset = "rust-strict"  # Includes common Rust rules
   ```

2. **Auto-fix Support**:
   ```toml
   [[rules]]
   name = "format-strings"
   autofix = true
   fix_command = "rustfmt --emit=stdout"
   ```

3. **ML-Based Rules**:
   ```toml
   [[rules]]
   name = "detect-code-duplication"
   type = "ml"
   model = "code-similarity"
   threshold = 0.85
   ```

4. **Team Rules Sharing**:
   ```bash
   # Publish rules to registry
   lucent rules publish company/rust-best-practices

   # Use shared rules
   lucent rules install company/rust-best-practices
   ```

## Migration Path

Users can start with simple pattern rules and progressively adopt more complex features:

1. **Phase 1**: Pattern-based rules (regex)
2. **Phase 2**: AST-based rules (syntax)
3. **Phase 3**: Metric-based rules (quantitative)
4. **Phase 4**: Plugin-based rules (custom logic)

## Documentation

Custom rules will be documented at:
- `docs/content/docs/custom-rules.mdx` - User guide
- `docs/content/docs/custom-rules-api.mdx` - Plugin API reference
- `examples/custom-rules/` - Example rules
- `CUSTOM_RULES.md` - Design and architecture

---

**Status**: Design Phase (Phase 3 - Planned)
**Dependencies**: Plugin system (completed), Configuration system (completed)
**Estimated Effort**: 2-3 weeks for full implementation
