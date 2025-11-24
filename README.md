<div align="center">
  
# Lucent 💎

_Crystal clear code insights_ 

**Lucent** is a fast, extensible code metrics analyzer built in Rust. It helps you understand code quality, complexity, and maintainability across multiple programming languages.

</div>

## ✨ Features

- 🚀 **Fast** - Rust-powered analysis for large codebases
- 🌍 **Multi-language** - 21+ languages supported (Rust, Python, JS/TS, Go, Java, C/C++, C#, Ruby, PHP, Swift, Kotlin, Dart, Scala, Elixir, Haskell, Clojure, Lua, Shell, SQL)
- 📊 **70+ Metrics Catalog** - Comprehensive roadmap from basic LOC to advanced SPACE metrics
- 💎 **Beautiful Output** - Rich colors, emojis, and ASCII charts in terminal, JSON, Markdown, and CSV exports
- 🔍 **Code Quality** - Shannon entropy, code density, comment ratio & quality analysis, TODO/FIXME/HACK detection
- 👃 **Code Smells** - Automatic detection of long functions, long parameter lists, deep nesting, and magic numbers
- 📈 **Maintainability Index** - Microsoft MI formula with A-F letter grades for code maintainability assessment
- 📊 **Visual Analytics** - ASCII bar charts for complexity and nesting depth visualization
- 🎨 **Multiple Outputs** - Pretty terminal, JSON, Markdown, CSV, and interactive HTML reports with charts
- 🌐 **Interactive HTML Reports** - Beautiful, self-contained reports with Chart.js visualizations, dark mode, and export capabilities
- ⚙️ **Configurable** - Metric profiles and thresholds via `.lucent.toml`
- ⚡ **CI/CD Ready** - GitHub Actions workflows for PR analysis, quality gates, and automated testing

## 🚧 Status

**Phase 1 Complete!** ✅ - Core architecture implemented and working.

Current version: `0.1.0-alpha`

### What's Working Now:
- ✅ File scanning with `.gitignore` support
- ✅ 21+ language detection
- ✅ Line counting (code, comments, blanks)
- ✅ Shannon entropy calculation
- ✅ **Cyclomatic complexity**
- ✅ **Cognitive complexity** (NEW!)
- ✅ **Nesting depth detection**
- ✅ **Function counting & length metrics**
- ✅ **Comment quality analysis** (NEW!)
- ✅ **Code smells detection** (NEW!)
  - Long functions (>50 lines)
  - Long parameter lists (>5 params)
  - Deep nesting (>4 levels)
  - Magic numbers
- ✅ **Maintainability Index** (NEW!)
  - Microsoft MI formula (0-100 scale)
  - Letter grades (A-F)
  - Per-language and overall metrics
- ✅ TODO/FIXME/HACK detection
- ✅ Beautiful terminal output with colors & emojis
- ✅ **ASCII complexity visualizations** (NEW!)
- ✅ JSON export
- ✅ **Markdown report generation** (NEW!)
- ✅ **CSV export** (NEW!)
- ✅ **Interactive HTML Reports** (NEW!)
  - Chart.js visualizations
  - Dark/light mode toggle
  - Print/PDF export
  - CSV export from browser
  - Responsive design
- ✅ **Configuration System** (NEW!)
  - `.lucent.toml` file support
  - Hierarchical config loading (project → user → defaults)
  - Threshold customization for all metrics
  - Ignore patterns (.lucentignore + .gitignore)
  - Language-specific overrides
  - Quality gates configuration
- ✅ **Snapshot & History Tracking** (NEW!)
  - Save snapshots with `lucent snapshot save`
  - List all snapshots with `lucent snapshot list`
  - Compare snapshots with trend analysis and quality score
  - Git integration (commit hash, branch, author, dirty state)
  - Automatic cleanup with retention policy
  - JSON storage in `.lucent/history/`
- ✅ **Threshold Checking & Quality Gates** (NEW!)
  - Exit code enforcement for CI/CD pipelines
  - `--max-complexity`, `--max-nesting`, `--min-maintainability`
  - `--max-function-length`, `--max-code-smells`, `--max-todos`
  - `--fail-on-warning` flag to treat warnings as errors
  - Warning vs Error severity levels
  - **Per-language thresholds** - Set different standards for each language
- ✅ **GitHub Actions workflows** (NEW!)
  - PR analysis with automated comments
  - Quality gate enforcement
  - Diff-mode for changed files only
  - Multi-platform CI/CD pipeline
- ✅ **Plugin System** (NEW!)
  - Extensible architecture for custom analyzers
  - Built-in regex plugin (default)
  - Plugin API with priority-based selection
  - Ready for tree-sitter integration
  - See [Plugin Developer Guide](docs/content/docs/plugin-guide.mdx)
- ✅ 65 tests passing (50 unit + 14 integration + 1 doc)

## 🎯 Roadmap

### Phase 1: Foundation ✅ **COMPLETE**
- [x] Basic file traversal and language detection
- [x] Lines of code counting (code, comments, blanks)
- [x] Shannon entropy calculation
- [x] Support for 13+ languages
- [x] Beautiful terminal output
- [x] TODO/FIXME/HACK tracking

### Phase 2: Core Metrics 🚧 **IN PROGRESS**
- [x] **Cyclomatic complexity** ✅
- [x] **Nesting depth detection** ✅
- [x] **Function/method counting & length** ✅
- [x] **Comment quality analysis** ✅
- [ ] Git-based metrics (commit size, velocity, churn)
- [ ] GitHub Action for PR analysis (automated comments)
- [x] **Expanded language support (21+ languages)** ✅

### Phase 3: Advanced Features 🚧 **IN PROGRESS**
- [x] **Cognitive complexity** ✅
- [x] **Code smell detection** ✅
  - [x] Long functions
  - [x] Long parameter lists
  - [x] Deep nesting
  - [x] Magic numbers
- [x] **Maintainability index** ✅
- [ ] Halstead metrics
- [x] **HTML reports** ✅
- [x] **Configuration system** (`.lucent.toml`) ✅
  - [x] Threshold customization
  - [x] Ignore patterns
  - [x] Language-specific overrides
  - [x] Quality gates configuration
- [x] **Snapshot & history tracking** ✅
  - [x] Save/load snapshots
  - [x] Compare snapshots
  - [x] Git integration
  - [x] Trend analysis
- [x] **Threshold checking & quality gates** ✅
  - [x] Exit code enforcement
  - [x] CLI threshold flags
  - [x] Warning vs Error severity
  - [x] Fail-on-warning mode

### Phase 4: Enterprise & Extensibility ⏳ **PLANNED**
- [ ] SPACE metrics (DORA)
- [ ] Team Topologies indicators
- [ ] Plugin system (tree-sitter based)
- [ ] Interactive TUI mode
- [ ] Change coupling analysis

### 📊 Full Metrics Catalog
See [ROADMAP.md](ROADMAP.md#-comprehensive-metrics-catalog) for the complete catalog of **70+ metrics** organized by phase, including:
- Traditional metrics (Cyclomatic, Cognitive, Halstead, Maintainability Index)
- Git-based metrics (Commit velocity, Code churn, Entropy delta, Hotspot analysis)
- SPACE metrics (Nicole Forsgren/DORA)
- Team Topologies indicators (Cognitive load, Module complexity)
- Security metrics (Secret patterns, Unsafe code detection)
- **6 metric profiles**: minimal, standard, comprehensive, team_lead, security, performance

## 🎓 Supported Languages

### Currently Supported (21+) ✅
**Tier 1:** 🦀 Rust • 🐍 Python • 📜 JavaScript • 💠 TypeScript • 🐹 Go • ☕ Java • ⚙️ C++ • 🔧 C • #️⃣ C# • 💎 Ruby • 🐘 PHP • 🦅 Swift • 🎯 Kotlin

**Tier 2:** 🎯 Dart • 🔴 Scala • 💧 Elixir • λ Haskell • 🌀 Clojure • 🌙 Lua • 🐚 Shell • 🗄️ SQL

### Planned Expansion (Phase 3+)
**Tier 3:** Zig, F#, Julia, OCaml, Nim, R, MATLAB, Perl, Objective-C

See [LANGUAGES.md](LANGUAGES.md) for details and [ROADMAP.md](ROADMAP.md) for language expansion plans.

## 🔧 Installation
```bash
# Not yet published to crates.io
# For now, build from source:
git clone https://github.com/ibrahimcesar/lucent
cd lucent
cargo build --release

# Run from project directory
cargo run -- .

# Or install locally
cargo install --path .
lucent .
```

## 🚀 Usage

```bash
# Analyze current directory
lucent .

# Analyze specific path
lucent src/

# Output as JSON
lucent --format json

# Output as interactive HTML report
lucent --format html > report.html

# Output as Markdown
lucent --format markdown > report.md

# Output as CSV
lucent --format csv > metrics.csv

# Verbose output with per-file details
lucent --verbose --per-file

# Get help
lucent --help

# Quality gates for CI/CD (exit code 1 if thresholds violated)
lucent --max-complexity 10.0 --min-maintainability 70.0

# Combine multiple thresholds
lucent --max-complexity 15.0 --max-nesting 4.0 --max-todos 10

# Fail on warnings (treat warnings as errors)
lucent --max-function-length 50.0 --fail-on-warning

# Load thresholds from .lucent.toml configuration file
lucent  # Auto-discovers .lucent.toml in current/parent directories

# Use custom config file
lucent --config-file path/to/my-config.toml

# CLI flags override config file settings
lucent --max-complexity 20.0  # Overrides value from .lucent.toml
```

### Configuration File

Create a `.lucent.toml` file in your project root to set default thresholds:

```toml
[thresholds]
max_cyclomatic_complexity = 15.0
max_nesting_depth = 4.0
min_maintainability_index = 65.0
max_function_length = 50

[quality_gates]
fail_on_complexity = true
fail_on_nesting = true
warn_only = false

# Per-language overrides
[languages.rust]
max_cyclomatic_complexity = 12.0
max_nesting_depth = 3.0
```

See [examples/lucent-thresholds.toml](examples/lucent-thresholds.toml) for a complete example.
```

### Example Output

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  💎 Lucent Analysis Results
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 Summary
  Total Files:  9
  Total Lines:  1342
  Code Lines:   939
  Comment Lines: 189
  Avg Entropy:  4.68
  Avg Complexity: 12.3
  Avg Nesting: 4.2
  Total Functions: 72
  Avg Func Length: 14.1

🔖 Code Markers
  TODOs:  3
  FIXMEs: 1

🌍 By Language

╭──────────┬───────┬───────┬──────┬──────────┬────────┬────────────┬─────────┬─────────╮
│ Language │ Files │ Lines │ Code │ Comments │ Blanks │ Complexity │ Nesting │ Entropy │
├──────────┼───────┼───────┼──────┼──────────┼────────┼────────────┼─────────┼─────────┤
│ 🦀 Rust  │     9 │  1342 │  939 │      189 │    214 │       12.3 │     4.2 │    4.68 │
╰──────────┴───────┴───────┴──────┴──────────┴────────┬────────────┴─────────┴─────────╯

💡 Insights
  ⚠️ High complexity: 12.3 (consider refactoring)
  ⚠️ Deep nesting: 4.2 levels (reduce nesting)
  ✓ Short functions: 14.1 lines avg (good modularity)
  ✓ Well documented: 14.1% comments
  ⚠️ Mediocre comment quality: 40% (add more context)
  📊 High entropy: 4.68 (information dense)
  📈 Code density: 70.0%

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## 🤖 CI/CD Integration

Lucent comes with production-ready GitHub Actions workflows for automated code analysis.

### Quick Start

1. **Copy workflows to your repository**:
   ```bash
   mkdir -p .github/workflows
   cp .github/workflows/lucent-*.yml .github/workflows/
   ```

2. **Push and create a PR** - Lucent will automatically analyze your code!

### Available Workflows

#### 💎 Full PR Analysis
Analyzes entire codebase on every pull request:
- Posts detailed metrics as PR comment
- Enforces quality gate thresholds
- Generates markdown and JSON reports

**Quality Gates** (configurable):
- ❌ Fails if complexity > 20.0
- ❌ Fails if nesting depth > 5.0
- ⚠️ Warns if comment quality < 30%

#### 💎 Diff Analysis (Recommended)
Analyzes only changed files for faster feedback:
- Focused analysis on your modifications
- Per-file metrics in PR comment
- Ideal for large codebases

#### 📈 Trend Comparison
Compares PR metrics with base branch:
- Tracks quality improvements/regressions
- Calculates metric deltas with percentages
- Provides quality score (0-100)
- Visual trend indicators (✅ ➡️ ❌)
- 90-day historical tracking

#### 🧪 CI Pipeline
Comprehensive testing and building:
- Multi-platform (Linux, macOS, Windows)
- Automated testing and linting
- Code coverage reporting
- Release binary builds

### Example PR Comment

<img width="800" alt="Lucent PR Comment" src="https://via.placeholder.com/800x400?text=Lucent+PR+Analysis">

```markdown
## 💎 Lucent Code Analysis Report

### 📊 Summary
| Metric | Value |
|--------|-------|
| Total Files | 15 |
| Average Complexity | 8.2 |
| Quality Score | ✅ PASS |

### 📊 Complexity Visualization
  🦀 Rust     12.5 ████████████████████░░░░░░░░░░
  🐍 Python    6.3 ██████████░░░░░░░░░░░░░░░░░░░░
```

### Customize Thresholds

Edit `.github/workflows/lucent-pr-analysis.yml`:

```yaml
# Strict quality gates
COMPLEXITY_THRESHOLD=10.0
NESTING_THRESHOLD=3.0
COMMENT_QUALITY_THRESHOLD=0.5
```

See [.github/workflows/README.md](.github/workflows/README.md) for complete documentation.

## 📚 Documentation

### Comprehensive Guides

- **[📈 Maintainability Index Guide](docs/content/docs/maintainability-index.mdx)** - NEW!
  - Understand the MI formula and scoring
  - Learn how to interpret A-F letter grades
  - Fine-tune thresholds for your organization
  - Practical strategies for improving your scores
  - Real-world examples and case studies

- **[📊 Metrics Explained](docs/content/docs/metrics.mdx)**
  - Deep dive into all metrics
  - Shannon Entropy explained
  - Complexity metrics guide
  - Comment quality analysis

- **[🔧 Configuration](docs/content/docs/config-file-guide.mdx)**
  - Quality gate customization
  - Threshold recommendations by project type
  - CI/CD integration patterns

- **[.github/workflows/README.md](.github/workflows/README.md)**
  - Complete GitHub Actions documentation
  - Workflow customization examples
  - Quality gate configuration

## 🤝 Contributing

Contributions are welcome! This project is in early stages, and we're building the foundation.

**How to contribute:**
- 🐛 Report bugs or suggest features via Issues
- 💻 Submit PRs for bug fixes or new features
- 📝 Improve documentation
- 🔌 Create language plugins (once plugin system is ready)

## 📝 License

MIT

## 🙏 Acknowledgments

Inspired by:
- [tokei](https://github.com/XAMPPRocky/tokei) - Fast line counting
- [scc](https://github.com/boyter/scc) - Complexity metrics
- [tree-sitter](https://tree-sitter.github.io/) - Universal parsing
- [SonarQube](https://www.sonarqube.org/) - Comprehensive quality analysis

---

**Status:** 🚧 Pre-alpha - Core architecture in development

**Star** ⭐ this repo to follow development!
