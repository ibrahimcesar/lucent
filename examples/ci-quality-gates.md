# Quality Gates for CI/CD

This example demonstrates how to use Lucent's threshold checking in CI/CD pipelines to enforce code quality standards.

## Overview

Lucent can act as a quality gate by exiting with code 1 when metrics exceed configured thresholds. This makes it ideal for CI/CD integration where you want to fail builds that don't meet quality standards.

## Available Thresholds

### Error Thresholds (Always Fail)
- `--max-complexity <N>` - Maximum average cyclomatic complexity
- `--max-nesting <N>` - Maximum average nesting depth
- `--min-maintainability <N>` - Minimum average maintainability index (0-100)

### Warning Thresholds (Optional Fail)
- `--max-function-length <N>` - Maximum average function length in lines
- `--max-code-smells <N>` - Maximum total code smells
- `--max-todos <N>` - Maximum number of TODO comments

### Severity Control
- `--fail-on-warning` - Treat warnings as errors (fail build on warnings)

## Exit Codes

- **0**: All thresholds passed
- **1**: One or more thresholds violated

## GitHub Actions Example

```yaml
name: Code Quality Gate

on: [push, pull_request]

jobs:
  quality-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install Lucent
        run: cargo install --path .

      - name: Run Quality Gate
        run: |
          lucent src/ \
            --max-complexity 15.0 \
            --max-nesting 4.0 \
            --min-maintainability 65.0 \
            --max-function-length 50.0 \
            --max-todos 20
```

## GitLab CI Example

```yaml
code_quality:
  stage: test
  image: rust:latest
  script:
    - cargo install --path .
    - |
      lucent src/ \
        --max-complexity 15.0 \
        --max-nesting 4.0 \
        --min-maintainability 65.0
  only:
    - merge_requests
    - main
```

## Jenkins Pipeline Example

```groovy
pipeline {
    agent any

    stages {
        stage('Quality Gate') {
            steps {
                sh '''
                    cargo install --path .
                    lucent src/ \
                        --max-complexity 15.0 \
                        --max-nesting 4.0 \
                        --min-maintainability 65.0 \
                        --fail-on-warning
                '''
            }
        }
    }
}
```

## CircleCI Example

```yaml
version: 2.1

jobs:
  quality-gate:
    docker:
      - image: cimg/rust:1.70
    steps:
      - checkout
      - run:
          name: Install Lucent
          command: cargo install --path .
      - run:
          name: Run Quality Checks
          command: |
            lucent src/ \
              --max-complexity 15.0 \
              --max-nesting 4.0 \
              --min-maintainability 65.0

workflows:
  build-and-test:
    jobs:
      - quality-gate
```

## Pre-commit Hook Example

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/bash

echo "Running code quality checks..."

lucent src/ \
  --max-complexity 20.0 \
  --max-nesting 5.0 \
  --min-maintainability 60.0 \
  --fail-on-warning

if [ $? -ne 0 ]; then
  echo "❌ Quality gate failed! Fix the issues above before committing."
  exit 1
fi

echo "✅ Quality checks passed!"
exit 0
```

## Make Target Example

Add to your `Makefile`:

```makefile
.PHONY: quality-gate
quality-gate:
	@echo "Running quality gate..."
	@lucent src/ \
		--max-complexity 15.0 \
		--max-nesting 4.0 \
		--min-maintainability 65.0 \
		--max-function-length 50.0 \
		--max-code-smells 100 \
		--max-todos 20 \
		--fail-on-warning
	@echo "✅ All quality thresholds passed"

.PHONY: ci
ci: test quality-gate
	@echo "CI pipeline complete"
```

## Recommended Thresholds by Project Size

### Small Projects (<10k LOC)
```bash
lucent src/ \
  --max-complexity 10.0 \
  --max-nesting 3.0 \
  --min-maintainability 75.0 \
  --max-function-length 40.0
```

### Medium Projects (10k-100k LOC)
```bash
lucent src/ \
  --max-complexity 15.0 \
  --max-nesting 4.0 \
  --min-maintainability 65.0 \
  --max-function-length 50.0
```

### Large Projects (>100k LOC)
```bash
lucent src/ \
  --max-complexity 20.0 \
  --max-nesting 5.0 \
  --min-maintainability 60.0 \
  --max-function-length 60.0
```

## Progressive Quality Improvement

Start with lenient thresholds and tighten them over time:

### Week 1: Baseline
```bash
lucent src/ --max-complexity 30.0 --max-nesting 6.0
```

### Week 4: Improvement
```bash
lucent src/ --max-complexity 25.0 --max-nesting 5.0
```

### Week 8: Target
```bash
lucent src/ --max-complexity 20.0 --max-nesting 4.0
```

## Output Examples

### ✅ Pass
```
✓ All quality thresholds passed
```
Exit code: 0

### ⚠️ Warning
```
⚠ Quality warnings detected

  ⚠ Average Function Length WARNING (expected <= 50 lines, got 52 lines)
```
Exit code: 0 (unless `--fail-on-warning`)

### ❌ Failure
```
✗ Quality thresholds violated

  ✗ Average Complexity FAILED (expected <= 15.0, got 22.3)
  ✗ Average Nesting Depth FAILED (expected <= 4.0, got 5.2)
```
Exit code: 1

## Combining with Snapshots

Compare against a baseline snapshot:

```bash
# Save baseline
lucent snapshot save --description "Baseline before refactor"

# Make changes...

# Check against baseline AND thresholds
lucent src/ --max-complexity 15.0

# Compare progress
lucent snapshot compare <baseline-id> <current-id>
```

## Per-Language Thresholds

Lucent supports setting different quality standards for different languages. This is useful for:

- **Microservices**: Stricter thresholds for backend Rust, lenient for frontend JS
- **Legacy Migration**: Allow higher complexity in legacy code during refactoring
- **Language Differences**: Account for natural complexity differences between languages

### Configuration File Approach (Recommended)

Create `.lucent.toml` with per-language overrides:

```toml
[thresholds]
# Global defaults
max_cyclomatic_complexity = 15.0
max_nesting_depth = 4.0
min_maintainability_index = 65.0

[languages.rust]
# Stricter for production backend
max_cyclomatic_complexity = 10.0
max_nesting_depth = 3.0
min_maintainability_index = 75.0

[languages.javascript]
# More lenient for UI code
max_cyclomatic_complexity = 20.0
max_nesting_depth = 5.0

[languages.python]
# Data processing scripts
max_cyclomatic_complexity = 18.0
max_function_length = 60
```

### GitHub Actions with Per-Language Thresholds

```yaml
- name: Code Quality Gate
  run: |
    lucent src/  # Uses .lucent.toml automatically
```

### Example Output

Violations show which language failed:

```
✗ Quality thresholds violated

  ✗ Rust - Average Complexity FAILED (expected <= 10.0, got 12.5)
  ⚠ JavaScript - Average Function Length WARNING (expected <= 50 lines, got 58 lines)
```

Each language is checked independently against its specific thresholds (or global defaults).

## Tips

1. **Start Lenient**: Begin with thresholds based on current metrics, then tighten gradually
2. **Use Warnings First**: Test new thresholds as warnings before enforcing as errors
3. **Per-Language Standards**: Use `.lucent.toml` to set different standards per language
4. **Document Standards**: Keep threshold values in README or contributing guide
5. **Team Agreement**: Discuss and agree on thresholds with the whole team
6. **Monitor Trends**: Use snapshots to track quality improvements over time

## Troubleshooting

**Q: Thresholds keep failing after refactoring**
A: Your improvements might not be reflected in averages yet. Consider using `--per-file` to identify specific files.

**Q: Different developers get different results**
A: Ensure everyone is using the same Lucent version and running from the same directory.

**Q: CI passes locally but fails in CI**
A: Check if `.gitignore` differences affect which files are analyzed.

## Future Features

Coming soon:
- Per-language thresholds
- Baseline comparison mode
- Load thresholds from `.lucent.toml`
- Threshold profiles (strict/standard/lenient)
