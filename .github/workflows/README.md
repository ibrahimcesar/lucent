# 🤖 GitHub Actions Workflows for Lucent

This directory contains automated workflows for Lucent code analysis and CI/CD.

## 📋 Available Workflows

### 1. 💎 Lucent Code Analysis (`lucent-pr-analysis.yml`)

**Trigger**: Pull requests to `main`, `master`, or `develop`

**What it does**:
- Runs full codebase analysis with Lucent
- Generates markdown and JSON reports
- Posts analysis results as PR comment
- Checks quality gate thresholds
- Uploads analysis artifacts

**Quality Gates**:
- ❌ **Fails** if average complexity > 20.0
- ❌ **Fails** if average nesting depth > 5.0
- ⚠️ **Warns** if comment quality < 30%

**Outputs**:
- PR comment with full Lucent analysis
- `lucent-report.md` - Markdown report
- `lucent-metrics.json` - JSON metrics
- GitHub Actions job summary

**Example PR Comment**:
```markdown
## 💎 Lucent Code Analysis Report

### 📊 Summary
| Metric | Value |
|--------|-------|
| Total Files | 10 |
| Total Lines | 310 |
| Average Complexity | 4.4 |
...
```

---

### 2. 💎 Lucent Diff Analysis (`lucent-diff-analysis.yml`)

**Trigger**: Pull requests to `main`, `master`, or `develop`

**What it does**:
- Analyzes only files changed in the PR
- Provides focused analysis on modifications
- Posts per-file metrics as PR comment
- More efficient for large codebases

**Supported file extensions**:
`.rs`, `.py`, `.js`, `.ts`, `.go`, `.java`, `.cpp`, `.c`, `.cs`, `.rb`, `.php`, `.swift`, `.kt`, `.dart`, `.scala`, `.ex`, `.hs`, `.clj`, `.lua`, `.sh`, `.sql`

**Outputs**:
- PR comment with diff analysis
- `diff-report.md` - Per-file analysis
- `changed_files.txt` - List of analyzed files

---

### 3. 📈 Lucent Trend Comparison (`lucent-trend-comparison.yml`)

**Trigger**: Pull requests to `main`, `master`, or `develop`

**What it does**:
- Compares PR metrics with base branch
- Calculates metric deltas and trends
- Provides quality score based on changes
- Shows improvement or regression indicators
- Tracks long-term code quality evolution

**Metrics Compared**:
- Files, Lines, Functions (size metrics)
- Cyclomatic Complexity (trend tracking)
- Nesting Depth (structure quality)
- Comment Quality (documentation)

**Quality Score**:
- **90-100**: ✅ Excellent - Quality improved or maintained
- **70-89**: ⚠️ Good - Minor concerns
- **< 70**: ❌ Needs improvement - Significant issues

**Trend Indicators**:
- ✅ **Green**: Metric improved
- ➡️ **Neutral**: No significant change (< 0.1)
- ❌ **Red**: Metric regressed

**Example Output**:
```markdown
## 📈 Lucent Trend Analysis

### 📊 Metric Changes
| Metric | Base | Current | Change | Trend |
|--------|------|---------|--------|-------|
| Avg Complexity | 10.5 | 8.2 | -2.3 (-21.9%) | ✅ |
| Avg Nesting | 3.0 | 2.5 | -0.5 (-16.7%) | ✅ |
| Comment Quality | 40% | 52% | +12% (+30.0%) | ✅ |

### Quality Score
**Score: 95/100** ✅ Excellent - Code quality improved
```

**Outputs**:
- PR comment with trend analysis
- `pr-metrics.json` - Current branch metrics
- `base-metrics.json` - Base branch metrics
- `trend-report.md` - Detailed comparison
- 90-day artifact retention for historical tracking

---

### 4. 🧪 CI - Test & Build (`ci.yml`)

**Trigger**:
- Push to `main`, `master`, or `develop`
- All pull requests

**What it does**:
- Runs on Linux, macOS, and Windows
- Checks code formatting (`cargo fmt`)
- Runs Clippy lints
- Executes all tests
- Builds release binary
- Self-analyzes with Lucent
- Generates code coverage
- Builds platform-specific binaries (on main branch)

**Matrix**:
- **OS**: Ubuntu, macOS, Windows
- **Rust**: stable

**Outputs**:
- Test results
- Release binaries (main branch only)
- Self-analysis JSON
- Code coverage report (Codecov)

**Artifacts**:
- `lucent-linux-x86_64.tar.gz`
- `lucent-macos-x86_64.tar.gz`
- `lucent-macos-aarch64.tar.gz`
- `lucent-windows-x86_64.exe.zip`

---

## 🚀 Getting Started

### Enable Workflows

1. **Push workflows to your repository**:
   ```bash
   git add .github/workflows/
   git commit -m "Add Lucent GitHub Actions workflows"
   git push
   ```

2. **Workflows are enabled automatically** for your repository

3. **Create a pull request** to see Lucent in action!

### Customize Thresholds

Edit the quality gate section in `lucent-pr-analysis.yml`:

```yaml
# Default thresholds
COMPLEXITY_THRESHOLD=20.0
NESTING_THRESHOLD=5.0
COMMENT_QUALITY_THRESHOLD=0.3
```

Adjust these values based on your project's requirements.

---

## 📊 Understanding Quality Gates

### Complexity Threshold (20.0)
- **< 5.0**: ✅ Simple, easy to maintain
- **5.0 - 10.0**: ✅ Moderate complexity
- **10.0 - 20.0**: ⚠️ Consider refactoring
- **> 20.0**: ❌ **FAILS** - Urgent refactoring needed

### Nesting Depth Threshold (5.0)
- **< 2.0**: ✅ Flat, readable code
- **2.0 - 4.0**: ✅ Acceptable nesting
- **4.0 - 5.0**: ⚠️ Deep nesting
- **> 5.0**: ❌ **FAILS** - Reduce nesting

### Comment Quality Threshold (0.3 = 30%)
- **< 0.3**: ⚠️ **WARNING** - Improve documentation
- **0.3 - 0.5**: ✅ Adequate documentation
- **> 0.7**: ✅ Excellent documentation

---

## 🔧 Configuration Examples

### Strict Quality Gates (Enterprise)

```yaml
COMPLEXITY_THRESHOLD=10.0
NESTING_THRESHOLD=3.0
COMMENT_QUALITY_THRESHOLD=0.5
```

### Relaxed Quality Gates (Legacy Codebase)

```yaml
COMPLEXITY_THRESHOLD=30.0
NESTING_THRESHOLD=6.0
COMMENT_QUALITY_THRESHOLD=0.2
```

### Open Source Project

```yaml
COMPLEXITY_THRESHOLD=15.0
NESTING_THRESHOLD=4.0
COMMENT_QUALITY_THRESHOLD=0.4
```

---

## 📝 Workflow Permissions

The workflows require the following permissions:

```yaml
permissions:
  contents: read        # Read repository contents
  pull-requests: write  # Post PR comments
  issues: write         # Create issue comments
```

These are automatically granted by GitHub Actions.

---

## 🎯 Best Practices

1. **Use both workflows together**:
   - `lucent-pr-analysis.yml` for full codebase health
   - `lucent-diff-analysis.yml` for focused PR changes

2. **Set appropriate thresholds**:
   - Start with relaxed thresholds
   - Gradually tighten as code quality improves

3. **Review analysis reports**:
   - Check PR comments after each push
   - Address quality issues before merging

4. **Track trends over time**:
   - Download and archive `lucent-metrics.json`
   - Compare metrics across releases

5. **Integrate with branch protection**:
   - Require quality gates to pass before merge
   - Settings → Branches → Add rule

---

## 🐛 Troubleshooting

### Workflow fails to build Lucent

**Solution**: Ensure Rust toolchain is properly cached
```yaml
- uses: actions/cache@v3
  with:
    path: ~/.cargo/
```

### PR comment too large

**Solution**: The diff workflow limits comments to 65,000 characters. For very large PRs, check the uploaded artifacts instead.

### Quality gate too strict

**Solution**: Adjust thresholds in the workflow file or use `--continue-on-error`:
```yaml
- name: 🚦 Quality Gate Check
  continue-on-error: true
```

### Missing `jq` command

**Solution**: Ubuntu runners have `jq` pre-installed. For other OS:
```yaml
- name: Install jq
  run: |
    # macOS
    brew install jq
    # Windows
    choco install jq
```

---

## 📚 Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Lucent Documentation](../../README.md)
- [Quality Metrics Guide](../../docs/METRICS.md)
- [Threshold Configuration](../../docs/THRESHOLDS.md)

---

## 🤝 Contributing

To add new workflows or improve existing ones:

1. Test locally with [act](https://github.com/nektos/act)
2. Document changes in this README
3. Submit a pull request

---

**Last Updated**: 2025-11-23
**Version**: 0.1.0-alpha
