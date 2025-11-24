# 🗺️ Lucent Roadmap

This document outlines the development plan for Lucent, a fast code metrics analyzer built in Rust.

**Current Version:** 0.0.1 (pre-alpha)
**Status:** 🚧 Foundation Phase
**Last Updated:** 2025-11-23

---

## 🎯 Vision

Build a fast, beautiful, and extensible code metrics analyzer that provides crystal clear insights into code quality across 25+ programming languages.

### Core Principles
- 🎨 **Beautiful by default** - Rich colors and emojis from day one
- ⚡ **Performance first** - Rust-powered speed for massive codebases
- 🔌 **Extensible** - Plugin architecture for custom languages and metrics
- 💎 **Developer-friendly** - Clear, actionable insights

---

## 📍 Current Status

### ✅ Completed (Phase 1)
- [x] Project initialization and structure
- [x] Basic Cargo.toml with dependencies
- [x] MIT license
- [x] Documentation (README, DOC, LANGUAGES, docs/)
- [x] Repository setup
- [x] **Core architecture implemented!** 🎉
- [x] All Phase 1 modules (scanner, metrics, language, output, config)
- [x] 13+ language support
- [x] Shannon entropy calculation
- [x] Beautiful terminal output with colors and emojis
- [x] JSON export
- [x] Integration tests (25 tests passing)
- [x] Example code
- [x] **Metrics documentation** (docs/METRICS.md)
- [x] **Configuration example** (.lucent.toml.example)
- [x] **Snapshot feature documented** (docs/SNAPSHOTS.md)

### 🚧 In Progress
- [ ] Phase 2 planning

---

## 🚀 Phase 1: Foundation (v0.1.0-alpha)

**Goal:** Basic CLI that can scan files and count lines with beautiful output

**Timeline:** Completed 2025-11-23
**Status:** ✅ **COMPLETED!**

### Core Features
- [x] **Project Structure**
  - [x] Create `src/scanner/mod.rs`
  - [x] Create `src/language/mod.rs`
  - [x] Create `src/metrics/mod.rs`
  - [x] Create `src/output/mod.rs`
  - [x] Create `src/config/mod.rs`
  - [x] Set up `lib.rs` with module exports

- [x] **File Scanner** (`scanner` module)
  - [x] Implement directory traversal with `walkdir`
  - [x] Respect `.gitignore` using `ignore` crate
  - [x] Filter by file extensions
  - [x] Handle symbolic links
  - [x] Error handling for permission issues

- [x] **Language Detection** (`language` module)
  - [x] File extension to language mapping
  - [x] Language configuration struct (name, extensions, comment styles)
  - [x] Support for Rust (`.rs`)
  - [x] Support for Python (`.py`)
  - [x] Support for JavaScript (`.js`, `.jsx`)
  - [x] Support for TypeScript (`.ts`, `.tsx`)
  - [x] Language registry system
  - [x] **BONUS:** 13+ languages (Go, Java, C++, C, C#, Ruby, PHP, Swift, Kotlin)
  - [x] **BONUS:** Emoji per language 🦀🐍📜💠

- [x] **Basic Metrics** (`metrics` module)
  - [x] Line counter (total lines)
  - [x] Code lines (non-empty, non-comment)
  - [x] Comment lines detection
  - [x] Blank lines counting
  - [x] File-level metrics aggregation
  - [x] Project-level metrics rollup
  - [x] **BONUS:** Shannon entropy calculation 📊

- [x] **Terminal Output** (`output` module)
  - [x] 🎨 Colorized output with `colored` crate
  - [x] 💎 Emoji integration for visual clarity
  - [x] Table formatting with `tabled`
  - [x] Summary statistics display
  - [x] Per-language breakdown
  - [x] File listing (optional, with `--per-file` flag)
  - [x] **BONUS:** Smart insights (entropy warnings, density analysis)

- [x] **CLI Interface** (`config` module)
  - [x] Argument parsing with `clap`
  - [x] Path argument for target directory
  - [x] `--format` flag (pretty, json)
  - [x] `--verbose` flag for detailed output
  - [x] `--per-file` flag for per-file metrics
  - [x] `--help` documentation

- [x] **Testing**
  - [x] Unit tests for each module (15 tests)
  - [x] Integration test for basic scanning (10 tests)
  - [x] Test fixtures with sample code files
  - [x] `tests/integration_test.rs`
  - [x] **All 25 tests passing** ✅

- [x] **Documentation**
  - [x] Code documentation comments
  - [x] `examples/basic_usage.rs`
  - [x] Update README with usage examples
  - [x] **BONUS:** Comprehensive `docs/` directory
  - [x] **BONUS:** METRICS.md - Detailed metrics explained
  - [x] **BONUS:** SNAPSHOTS.md - Future feature documented
  - [x] **BONUS:** THRESHOLDS.md - Threshold reference
  - [x] **BONUS:** .lucent.toml.example - Config template

**Exit Criteria:** ✅ **ALL MET!**
- ✅ CLI can scan a directory and count lines
- ✅ Beautiful terminal output with colors and emojis
- ✅ JSON export works
- ✅ Tests pass (25/25)
- ✅ Documentation complete

---

## 📊 Phase 2: Core Metrics (v0.2.0-alpha)

**Goal:** Add complexity metrics and expand language support

**Timeline:** Started 2025-11-23
**Status:** 🚧 **IN PROGRESS**

### Core Features
- [x] **Cyclomatic Complexity** ✅ **COMPLETE!**
  - [x] Basic complexity calculation (counts decision points)
  - [x] File-level aggregation
  - [x] Complexity thresholds and warnings (< 5, 10-20, > 20)
  - [x] Terminal output with color-coded insights
  - [x] JSON export support
  - [ ] Function-level complexity (requires AST - deferred to Phase 3)

- [x] **Function Analysis** ✅ **COMPLETE!**
  - [x] Function/method counting (regex-based for 13+ languages)
  - [x] Function length metrics (average lines per function)
  - [x] Terminal output with function insights
  - [x] JSON export support
  - [ ] Parameter count per function (deferred to Phase 3)

- [x] **Comment Analysis** ✅ **COMPLETE!**
  - [x] Comment ratio calculation
  - [x] Documentation coverage estimate
  - [x] TODO/FIXME/HACK detection
  - [x] Comment quality heuristics (detects low/high quality comments)

- [x] **Expanded Language Support** (21+ languages currently supported) ✅
  - [x] Go (`.go`) ✅
  - [x] Java (`.java`) ✅
  - [x] C++ (`.cpp`, `.hpp`, `.cc`, `.h`, `.cxx`, `.hxx`) ✅
  - [x] Ruby (`.rb`) ✅
  - [x] Dart (`.dart`) ✅
  - [x] Scala (`.scala`) ✅
  - [x] Elixir (`.ex`, `.exs`) ✅
  - [x] Haskell (`.hs`, `.lhs`) ✅
  - [x] Clojure (`.clj`, `.cljs`, `.cljc`) ✅
  - [x] Lua (`.lua`) ✅
  - [x] Shell (`.sh`, `.bash`, `.zsh`) ✅
  - [x] SQL (`.sql`) ✅

- [ ] **Enhanced Output**
  - [x] Markdown report generation ✅
  - [x] CSV export ✅
  - [x] Complexity visualization (ASCII charts) ✅
  - [ ] Warning and error highlighting

- [x] **CI/CD Integration** ✅
  - [x] GitHub Actions workflow for PRs (Lucent PR analyzer) ✅
  - [x] Automated PR comments with metrics report ✅
  - [x] Quality gate thresholds (fail PR on high complexity) ✅
  - [x] Diff mode (analyze only changed files) ✅
  - [x] Trend comparison (compare metrics with base branch) ✅
  - [x] Automated testing workflow ✅
  - [x] Coverage reporting ✅
  - [x] Release automation (binary builds) ✅

**Exit Criteria:**
- Cyclomatic complexity working for 5+ languages
- Function-level metrics available
- CI/CD pipeline operational
- v0.2.0-alpha published

---

## 🔬 Phase 3: Advanced Features (v0.3.0-beta)

**Goal:** Advanced metrics, code smells, and reporting

**Timeline:** TBD
**Status:** ⏳ Planned

### Core Features
- [x] **Cognitive Complexity** ✅
  - [x] Implement cognitive complexity algorithm ✅
  - [x] Comparison with cyclomatic complexity ✅
  - [x] Visualization ✅

- [x] **Code Smells Detection** ✅
  - [x] Long functions (>50 lines threshold) ✅
  - [x] Long parameter lists (>5 parameters) ✅
  - [x] Deep nesting (>4 levels) ✅
  - [x] Magic numbers detection (excludes common numbers like 0, 1, 2, 10, 100, 1000) ✅
  - [x] Severity levels (Minor, Moderate, Major) ✅
  - [x] Integration in all output formats ✅
  - [ ] Duplicate code detection (basic)

- [x] **Maintainability Index** ✅
  - [x] Calculate maintainability index using Microsoft formula ✅
  - [x] Letter grade (A-F) assignment (85-100: A, 65-84: B, 50-64: C, 25-49: D, 0-24: F) ✅
  - [x] Integration in all output formats (Terminal, JSON, CSV, Markdown) ✅
  - [ ] Trend tracking (with git integration)

- [x] **HTML Reports** ✅
  - [x] Static HTML report generation (self-contained single file) ✅
  - [x] Interactive charts and graphs (Chart.js integration) ✅
  - [x] Dark/light mode toggle with persistence ✅
  - [x] Export functionality (CSV export, Print/PDF support) ✅
  - [x] Responsive design (mobile, tablet, desktop) ✅
  - [x] Embedded CSS and JavaScript (no external dependencies except Chart.js CDN) ✅
  - [ ] Drill-down capability (expandable per-file details)

- [x] **Configuration System** 📄 **[Example](.lucent.toml.example)** ✅
  - [x] `.lucent.toml` configuration file structure (complete settings.rs implementation) ✅
  - [x] Threshold customization (all metrics configurable: complexity, nesting, function length, MI, density, etc.) ✅
  - [x] Ignore patterns (patterns list + .lucentignore support + .gitignore integration) ✅
  - [x] Per-project settings (hierarchical loading: project → user → system → defaults) ✅
  - [x] Language-specific overrides (per-language threshold customization) ✅
  - [x] Quality gates configuration (fail_on_complexity, fail_on_nesting, fail_on_maintainability, warn_only) ✅
  - [x] Output format defaults (format, colors, per-file display, charts) ✅
  - [ ] Custom rules definition (planned for Phase 3)
  - [ ] Integration with main analysis flow (connect config to scanner)
  - [ ] Quality gate exit codes (implement CI/CD failure logic)

- [x] **Snapshot & History Tracking** 📸 ✅
  - [x] Save analysis snapshots as JSON (`.lucent/history/` directory) ✅
  - [x] Track metrics evolution over time (complete ProjectMetrics serialization) ✅
  - [x] Compare current vs previous snapshots (SnapshotComparison with quality score) ✅
  - [x] Git integration (commit hash, branch, author, dirty state detection) ✅
  - [x] Snapshot management (save, list, delete, cleanup commands) ✅
  - [x] Auto-cleanup old snapshots (retention policy with configurable keep count) ✅
  - [x] Trend calculation (MetricDelta with improved/neutral/regressed detection) ✅
  - [x] CLI commands (`lucent snapshot save/list/compare/delete/cleanup`) ✅
  - [ ] Trend visualization (ASCII sparkline charts for terminal)
  - [ ] Historical comparison reports (markdown/HTML trend reports)

- [x] **Threshold Checking** ✅
  - [x] Exit code based on thresholds
  - [x] `--max-complexity` flag
  - [x] `--max-nesting` flag
  - [x] `--min-maintainability` flag
  - [x] `--max-function-length` flag
  - [x] `--max-code-smells` flag
  - [x] `--max-todos` flag
  - [x] `--fail-on-warning` flag
  - [x] Quality gate enforcement with exit codes
  - [x] Per-language thresholds ✅
  - [ ] Baseline comparison mode
  - [x] Load thresholds from .lucent.toml ✅

**Exit Criteria:**
- Advanced metrics operational
- HTML reports generating
- Configuration system working
- Beta ready for broader testing

---

## 🔌 Phase 4: Extensibility (v1.0.0)

**Goal:** Plugin system and production readiness

**Timeline:** TBD
**Status:** ⏳ Planned

### Core Features
- [ ] **Tree-sitter Integration**
  - [ ] Core tree-sitter support
  - [ ] AST parsing for accuracy
  - [ ] Query-based metric extraction
  - [ ] Performance optimization

- [x] **Plugin System** ✅
  - [x] Plugin API design (MetricProvider trait) ✅
  - [x] Plugin registry and loading ✅
  - [x] Built-in regex plugin ✅
  - [x] Configuration support ✅
  - [x] Priority-based selection ✅
  - [x] Plugin developer guide ✅
  - [ ] Dynamic plugin loading (shared libraries)
  - [ ] Plugin registry/marketplace

- [ ] **Expanded Language Support**
  - [ ] 25+ languages via tree-sitter
  - [ ] Community-contributed languages
  - [ ] Language packs

- [ ] **Interactive TUI Mode**
  - [ ] Real-time file watching
  - [ ] Interactive exploration
  - [ ] Drill-down navigation
  - [ ] Filtering and search

- [x] **Git Integration**
  - [x] Diff mode (analyze changes only)
  - [x] Commit-by-commit analysis
  - [x] Trend tracking
  - [x] Blame integration

- [x] **Performance**
  - [x] Parallel processing optimization (rayon)
  - [x] Caching layer (file-based with mtime validation)
  - [x] Incremental analysis (via Git diff mode)
  - [x] Benchmark suite (criterion-based)

**Exit Criteria:**
- Plugin system stable
- 25+ languages supported
- Performance benchmarks met
- v1.0.0 production ready

---

## 🎯 Release Milestones

| Version | Focus | Status | Completed |
|---------|-------|--------|-----------|
| **v0.1.0-alpha** | Foundation - Basic CLI with line counting + entropy | ✅ **COMPLETE** | 2025-11-23 |
| **v0.2.0-alpha** | Core Metrics - Complexity & expanded languages | ⏳ Planned | TBD |
| **v0.3.0-beta** | Advanced Features - Code smells & HTML reports | ⏳ Planned | TBD |
| **v1.0.0** | Production - Plugin system & 25+ languages | ⏳ Planned | TBD |

---

## 🌟 Future Possibilities (Post-1.0)

These are aspirational features for future consideration:

### Developer Experience
- [x] VSCode extension (v0.1.0 - foundational features complete)
- [ ] JetBrains plugin
- [ ] Web dashboard
- [ ] Real-time collaboration features

### Advanced Analytics
- [ ] Machine learning for code smell prediction
- [ ] Automated refactoring suggestions
- [ ] Security vulnerability hints
- [ ] Performance bottleneck detection

### Integrations
- [ ] GitHub App
- [ ] GitLab integration
- [ ] Bitbucket support
- [ ] SonarQube compatibility
- [ ] IDE language servers

### Enterprise Features
- [ ] Team analytics
- [ ] Historical tracking
- [ ] Multi-repo analysis
- [ ] Custom rule engine
- [ ] API server mode

---

## 📝 Contributing to the Roadmap

This roadmap is a living document. Contributions and feedback are welcome!

**How to contribute:**
1. Open an issue to discuss new features
2. Comment on existing roadmap items
3. Submit PRs with roadmap updates
4. Vote on features via GitHub Discussions

**Priority Criteria:**
- 🔴 Critical - Blocks next release
- 🟡 High - Important for release
- 🟢 Medium - Nice to have
- ⚪ Low - Future consideration

---

## 📊 Progress Tracking

Track overall progress: [GitHub Projects](https://github.com/ibrahimcesar/lucent/projects)

### Phase 1 Progress: 100% ✅
```
████████████████████ 100% COMPLETE!
```

### Overall Project Progress: 25%
```
█████░░░░░░░░░░░░░░░ 25%
```
*(Phase 1 complete, ready for Phase 2)*

---

## 📊 Comprehensive Metrics Catalog

This section provides a complete catalog of all metrics that can be implemented in Lucent, organized by implementation phase and complexity.

### Metric Classification

**Default Behavior:**
- ✅ **ON by default** - Industry standard, widely accepted, fast to calculate
- ⚙️ **OFF by default** - Specialized use case, expensive, or privacy-sensitive
- 🔒 **Requires opt-in** - Privacy/ethics concerns (team/author data)

**Complexity:**
- 🟢 **Simple** - Basic parsing, quick calculation
- 🟡 **Moderate** - Requires AST or git integration
- 🔴 **Complex** - Multi-pass analysis, expensive computation

---

### Phase 2 Metrics (v0.2.0-alpha) - Core Complexity

#### Traditional Metrics
| Metric | Default | Complexity | Priority | Notes |
|--------|---------|------------|----------|-------|
| **Cyclomatic Complexity** | ✅ ON | 🟡 Moderate | High | Count decision points (if, while, for, case) |
| **Function Length** | ✅ ON | 🟢 Simple | High | Lines per function, threshold: 50 lines |
| **Nesting Depth** | ✅ ON | 🟢 Simple | High | Max indentation, threshold: 4 levels |
| **Parameter Count** | ✅ ON | 🟢 Simple | High | Args per function, threshold: 5 params |
| **ABC Metric** | ⚙️ OFF | 🟡 Moderate | Medium | Assignments, Branches, Conditionals |

#### Git-Based Metrics
| Metric | Default | Complexity | Priority | Notes |
|--------|---------|------------|----------|-------|
| **Commit Size** | ✅ ON | 🟢 Simple | High | Lines changed per commit, categorize small/medium/large |
| **Time Between Commits** | ✅ ON | 🟢 Simple | Medium | Avg/median gaps between commits |
| **Uncommitted Changes** | ⚙️ OFF | 🟢 Simple | Low | Files modified without commit |

#### Implementation Notes:
```rust
// Cyclomatic complexity pseudocode
fn calculate_cyclomatic(content: &str) -> usize {
    let mut complexity = 1; // Base complexity
    for token in ["if", "while", "for", "match", "&&", "||"] {
        complexity += content.matches(token).count();
    }
    complexity
}

// Function length
fn count_function_lines(ast_node: &Function) -> usize {
    ast_node.end_line - ast_node.start_line
}
```

---

### Phase 3 Metrics (v0.3.0-beta) - Advanced Analysis

#### Cognitive & Maintainability
| Metric | Default | Complexity | Priority | Notes |
|--------|---------|------------|----------|-------|
| **Cognitive Complexity** | ✅ ON | 🟡 Moderate | High | Weighted by nesting depth |
| **Maintainability Index** | ✅ ON | 🟡 Moderate | High | 0-100 score, combines Halstead+Cyclomatic+LOC |
| **Halstead Metrics** | ⚙️ OFF | 🟡 Moderate | Medium | Volume, Difficulty, Effort |
| **Nesting Complexity** | ✅ ON | 🟢 Simple | Medium | Weighted by depth level |

#### Git Evolution Metrics
| Metric | Default | Complexity | Priority | Notes |
|--------|---------|------------|----------|-------|
| **Code Churn** | ✅ ON | 🟢 Simple | High | Lines added + deleted / time period |
| **Entropy Delta** | ✅ ON | 🟡 Moderate | High | Entropy added/removed per commit |
| **Hotspot Analysis** | ✅ ON | 🟢 Simple | High | Files changed most frequently |
| **Commit Velocity** | ⚙️ OFF | 🟢 Simple | Medium | Commits per week/sprint (configurable) |
| **Commit Reversibility** | ⚙️ OFF | 🟡 Moderate | Medium | Score 0-100 based on size+files+tests |

#### Testing Metrics
| Metric | Default | Complexity | Priority | Notes |
|--------|---------|------------|----------|-------|
| **Test-to-Code Ratio** | ✅ ON | 🟢 Simple | High | Test LOC / Code LOC, threshold: 0.5:1 |
| **Test Complexity** | ✅ ON | 🟡 Moderate | Medium | Avg complexity of test functions |
| **Assert Density** | ⚙️ OFF | 🟡 Moderate | Low | Asserts per test function |

#### Code Quality
| Metric | Default | Complexity | Priority | Notes |
|--------|---------|------------|----------|-------|
| **Magic Numbers** | ✅ ON | 🟢 Simple | Medium | Literal numbers (exclude 0, 1, -1) |
| **Identifier Length** | ⚙️ OFF | 🟢 Simple | Low | Warn on single-letter vars (except i,j,k) |
| **Documentation Ratio** | ✅ ON | 🟢 Simple | Medium | Doc comments / total comments |

#### Implementation: Maintainability Index
```rust
// Microsoft's Maintainability Index formula
fn maintainability_index(metrics: &FileMetrics) -> f64 {
    let halstead_volume = calculate_halstead_volume(metrics);
    let cyclomatic = metrics.cyclomatic_complexity as f64;
    let loc = metrics.code_lines as f64;

    let mi = 171.0
        - 5.2 * halstead_volume.ln()
        - 0.23 * cyclomatic
        - 16.2 * loc.ln();

    // Normalize to 0-100
    (mi.max(0.0) * 100.0 / 171.0).min(100.0)
}
```

---

### Phase 4 Metrics (v1.0.0) - Enterprise & Teams

#### SPACE Metrics (Nicole Forsgren/DORA)
| Metric | Default | Complexity | Priority | Notes |
|--------|---------|------------|----------|-------|
| **Deployment Frequency** | ⚙️ OFF | 🔴 Complex | Medium | Requires CI/CD integration |
| **Lead Time for Changes** | ⚙️ OFF | 🔴 Complex | Medium | Commit to production time |
| **Change Failure Rate** | ⚙️ OFF | 🔴 Complex | Medium | % of deployments causing incidents |
| **Code Review Cycle Time** | ⚙️ OFF | 🟡 Moderate | Medium | PR open to merge (requires GitHub API) |
| **Documentation Drift** | ✅ ON | 🟢 Simple | Low | Docs changes vs code changes |
| **Rework Ratio** | ✅ ON | 🟡 Moderate | Medium | Changes to same lines within N days |

#### Team Topologies Indicators
| Metric | Default | Complexity | Priority | Notes |
|--------|---------|------------|----------|-------|
| **Module Complexity Score** | ✅ ON | 🟡 Moderate | High | Combined: LOC+deps+churn+complexity |
| **Dependency Fan-out** | ✅ ON | 🟡 Moderate | High | Modules this depends on |
| **API Surface Area** | ✅ ON | 🟢 Simple | Medium | Public functions/types count |
| **Domain Violations** | ⚙️ OFF | 🟡 Moderate | Medium | Cross-domain imports (needs config) |
| **Ownership Clarity** | 🔒 Opt-in | 🟡 Moderate | Low | Single vs multi-team ownership |
| **Cross-Team Deps** | 🔒 Opt-in | 🟡 Moderate | Low | Files edited by multiple teams |

#### Change Coupling & Architecture
| Metric | Default | Complexity | Priority | Notes |
|--------|---------|------------|----------|-------|
| **Change Coupling** | ⚙️ OFF | 🔴 Complex | High | Files that change together |
| **Afferent Coupling** | ⚙️ OFF | 🟡 Moderate | Medium | Incoming dependencies |
| **Efferent Coupling** | ⚙️ OFF | 🟡 Moderate | Medium | Outgoing dependencies |
| **Instability** | ⚙️ OFF | 🟡 Moderate | Low | Ce / (Ca + Ce) |
| **Distance from Main** | ⚙️ OFF | 🟡 Moderate | Low | Abstractness + Instability balance |

#### Git Team Dynamics
| Metric | Default | Complexity | Priority | Notes |
|--------|---------|------------|----------|-------|
| **Author Ownership** | 🔒 Opt-in | 🟡 Moderate | Low | % of file by author (privacy!) |
| **Bus Factor** | 🔒 Opt-in | 🟡 Moderate | Medium | Risk if key author leaves |
| **Collaboration Patterns** | 🔒 Opt-in | 🟡 Moderate | Low | Co-authorship frequency |
| **Commit Time Distribution** | 🔒 Opt-in | 🟢 Simple | Low | Hour-of-day patterns (ethics!) |

---

### Future Metrics (Post-1.0) - Specialized

#### Security & Safety
| Metric | Default | Complexity | Priority | Notes |
|--------|---------|------------|----------|-------|
| **Secret Patterns** | ✅ ON | 🟢 Simple | High | Detect AWS keys, tokens, passwords |
| **Unsafe Code Ratio** | ✅ ON | 🟢 Simple | High | `unsafe` blocks in Rust |
| **Dependency Age** | ⚙️ OFF | 🟡 Moderate | Medium | Days since last update |
| **Known Vulnerabilities** | ⚙️ OFF | 🔴 Complex | High | CVE database lookup |

#### Performance (Rust-specific)
| Metric | Default | Complexity | Priority | Notes |
|--------|---------|------------|----------|-------|
| **Allocation Sites** | ⚙️ OFF | 🔴 Complex | Low | Heap allocations in hot paths |
| **Clone Density** | ⚙️ OFF | 🟢 Simple | Medium | `.clone()` calls frequency |
| **Lifetime Complexity** | ⚙️ OFF | 🟡 Moderate | Low | Generic lifetime parameters |
| **Trait Complexity** | ⚙️ OFF | 🟡 Moderate | Low | Trait bounds per impl |

#### Language-Specific Features
**Python:**
- Type Hint Coverage (% functions with types)
- Import Complexity (circular imports)

**JavaScript/TypeScript:**
- Any Type Usage (TypeScript `any` ratio)
- Callback Hell Depth (nested callbacks)

**Go:**
- Goroutine Complexity
- Channel Usage Patterns

---

## 🔧 Configuration: Metric Profiles

Users can enable pre-configured metric bundles:

```toml
# .lucent.toml

[metrics]
# Select a profile (or customize individual metrics)
profile = "standard"  # minimal | standard | comprehensive | team_lead | security

# Or enable specific metrics
[metrics.enabled]
cyclomatic_complexity = true
cognitive_complexity = true
commit_velocity = true

[metrics.thresholds]
# Cyclomatic complexity
cyclomatic_low = 10
cyclomatic_high = 20

# Function length
function_length_warning = 50
function_length_error = 100

# Nesting depth
nesting_max = 4

# Test ratio
test_ratio_min = 0.5

# Commit size
commit_small_max = 50      # lines
commit_medium_max = 200
commit_large_max = 500

[metrics.git]
# Git-based metrics configuration
enabled = true
sprint_days = 14           # Sprint duration
sprint_start_day = "Monday"
velocity_period = "week"   # week | sprint | month

[metrics.space]
# SPACE metrics (requires integrations)
enabled = false
github_token = "${GITHUB_TOKEN}"
ci_integration = "github-actions"

[metrics.team]
# Team Topologies (requires team config)
enabled = false
teams_file = ".lucent/teams.toml"
```

### Metric Profiles

**Minimal** (fast, essential only):
```toml
profile = "minimal"
# Includes: LOC, Entropy, Cyclomatic Complexity
```

**Standard** (balanced, default):
```toml
profile = "standard"
# Includes: minimal + TODOs, Function Length, Nesting, Test Ratio, Commit Size
```

**Comprehensive** (everything except privacy-sensitive):
```toml
profile = "comprehensive"
# Includes: standard + Git metrics, Maintainability, Coupling, Halstead, Code Churn
```

**Team Lead** (focus on team health):
```toml
profile = "team_lead"
# Includes: Commit Velocity, Code Review Time, Hotspots, Module Complexity
```

**Security** (focus on vulnerabilities):
```toml
profile = "security"
# Includes: Secret Patterns, Dependency Age, Unsafe Ratio, Known Vulns
```

**Performance** (Rust performance):
```toml
profile = "performance"
# Includes: Allocation Sites, Clone Density, Complexity, Unsafe Code
```

---

## 📐 Implementation Formulas

### Cyclomatic Complexity (McCabe)
```
CC = E - N + 2P
Where:
  E = edges in control flow graph
  N = nodes in control flow graph
  P = connected components (usually 1)

Simplified:
CC = 1 + (number of decision points)
Decision points: if, while, for, case, &&, ||, ?:
```

### Cognitive Complexity (SonarSource)
```
CC = sum of complexity increments

Increments:
- +1 for: if, while, for, catch, switch, ?: ternary
- +1 for each boolean operator (&&, ||) in condition
- +nested_level for: if, while, for inside nested structures

Example:
if (a && b) {        # +1 (if) +1 (&&) = 2
    if (c) {         # +1 (if) +1 (nesting) = 2
        while (d) {  # +1 (while) +2 (nesting) = 3
        }
    }
}
Total: 7
```

### Maintainability Index (Microsoft)
```
MI = 171 - 5.2 × ln(HV) - 0.23 × CC - 16.2 × ln(LOC)

Where:
  HV = Halstead Volume
  CC = Cyclomatic Complexity
  LOC = Lines of Code

Normalized to 0-100:
MI_normalized = max(0, MI × 100 / 171)

Interpretation:
  85-100: Good (green)
  65-84:  Moderate (yellow)
  0-64:   Poor (red)
```

### Halstead Metrics
```
n1 = number of distinct operators
n2 = number of distinct operands
N1 = total occurrences of operators
N2 = total occurrences of operands

Program Vocabulary: n = n1 + n2
Program Length: N = N1 + N2
Volume: V = N × log2(n)
Difficulty: D = (n1/2) × (N2/n2)
Effort: E = D × V
Time to Program: T = E / 18 seconds
Estimated Bugs: B = V / 3000
```

### Entropy Delta (Per Commit)
```
For each commit:
  entropy_before = calculate_entropy(file_before)
  entropy_after = calculate_entropy(file_after)
  delta = entropy_after - entropy_before

Positive delta = complexity increased
Negative delta = complexity decreased

Track over time for trends.
```

### Code Churn
```
Churn = (Lines Added + Lines Deleted) / Time Period

High churn indicators:
- File churn > 500 lines/week = high instability
- Churn without tests = risky changes
- Churn concentrated in few files = hotspots
```

### Commit Reversibility Score
```
Score = 100 - penalty

Penalties:
- Size: lines_changed > 200 (-20)
- Files: files_changed > 5 (-15)
- No tests: test_changes == 0 (-30)
- Merge conflicts: potential_conflicts (-25)
- Dependencies: changes_dependencies (-20)

Range: 0 (high risk) to 100 (easily reversible)
```

---

## 🤖 GitHub Action: Lucent PR Analyzer

### Overview
A GitHub Action that automatically analyzes pull requests with Lucent and posts a detailed comment with code quality metrics, trends, and recommendations.

### Implementation Plan

#### Phase 2a: Basic PR Analysis (Priority: High 🔴)
**Goal:** Get Lucent running on every PR with basic metrics

```yaml
# .github/workflows/lucent-pr.yml
name: Lucent Code Analysis

on:
  pull_request:
    types: [opened, synchronize, reopened]

jobs:
  analyze:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write

    steps:
      - name: Checkout PR
        uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Install Lucent
        run: cargo install lucent-code

      - name: Run Lucent Analysis
        id: lucent
        run: |
          lucent . --format json > lucent-output.json
          cat lucent-output.json

      - name: Comment PR
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const results = JSON.parse(fs.readFileSync('lucent-output.json', 'utf8'));

            // Format comment
            const comment = `## 💎 Lucent Code Analysis

            ### 📊 Summary
            - **Files Analyzed:** ${results.summary.total_files}
            - **Lines of Code:** ${results.summary.code_lines}
            - **Avg Complexity:** ${results.summary.average_cyclomatic.toFixed(1)}
            - **Avg Nesting:** ${results.summary.average_nesting_depth.toFixed(1)}
            - **Avg Entropy:** ${results.summary.average_entropy.toFixed(2)}

            ### 🔖 Code Markers
            ${results.summary.total_todos > 0 ? `- ⚠️ **TODOs:** ${results.summary.total_todos}` : ''}
            ${results.summary.total_fixmes > 0 ? `- 🔴 **FIXMEs:** ${results.summary.total_fixmes}` : ''}
            ${results.summary.total_hacks > 0 ? `- 🟣 **HACKs:** ${results.summary.total_hacks}` : ''}

            ### 💡 Insights
            ${results.summary.average_cyclomatic > 20 ? '🔴 **Very high complexity** - Urgent refactoring recommended' : ''}
            ${results.summary.average_cyclomatic > 10 && results.summary.average_cyclomatic <= 20 ? '⚠️ **High complexity** - Consider refactoring' : ''}
            ${results.summary.average_nesting_depth > 4 ? '⚠️ **Deep nesting detected** - Consider flattening code structure' : ''}

            ---
            *Analyzed with [Lucent](https://github.com/ibrahimcesar/lucent) v${results.version || '0.2.0'}*
            `;

            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: comment
            });
```

#### Phase 2b: Quality Gates (Priority: Medium 🟡)
**Goal:** Fail PR if complexity thresholds are exceeded

**Features:**
- Configurable thresholds via `.lucent.toml`
- Exit with error code if thresholds exceeded
- Red/yellow/green status indicators
- Actionable recommendations

**Example thresholds:**
```toml
[quality_gates]
max_avg_complexity = 15.0
max_avg_nesting = 4.0
max_file_complexity = 50.0
max_todos_increase = 5
fail_on_fixmes = true
```

**Workflow addition:**
```yaml
      - name: Check Quality Gates
        run: |
          lucent . --check-thresholds
          # Exit code 1 if thresholds exceeded
```

#### Phase 2c: Diff Mode (Priority: High 🔴)
**Goal:** Analyze only files changed in the PR

**Implementation:**
```bash
# Get changed files
git diff --name-only origin/${{ github.base_ref }}...HEAD > changed-files.txt

# Run Lucent on changed files only
lucent --files-from changed-files.txt --format json
```

**Benefits:**
- Faster analysis for large repos
- Focus on PR-specific changes
- Relevant metrics for code review

#### Phase 3a: Trend Comparison (Priority: Low 🟢)
**Goal:** Compare metrics between base branch and PR

**Features:**
- Show complexity delta (↑ ↓ →)
- Highlight improvements or regressions
- Historical trend graphs (ASCII or image)

**Example comment:**
```markdown
## 📈 Metrics Comparison

| Metric | Base | PR | Change |
|--------|------|----|----|
| Avg Complexity | 12.3 | 14.5 | 🔴 +2.2 |
| Avg Nesting | 4.1 | 3.8 | ✅ -0.3 |
| Code Density | 68% | 71% | ↗️ +3% |
| TODOs | 15 | 18 | ⚠️ +3 |
```

#### Phase 3b: Per-File Breakdown (Priority: Low 🟢)
**Goal:** Show metrics for each changed file

**Features:**
- Expandable details section
- Sort by complexity (highest first)
- Link to file in PR
- Inline suggestions

**Example:**
```markdown
### 📁 Changed Files

<details>
<summary>src/metrics/mod.rs (Complexity: 35.2 🔴)</summary>

- **Lines:** 468 (+87)
- **Complexity:** 35.2 (+5.1) 🔴
- **Nesting:** 5.2 (+0.4) ⚠️
- **TODOs:** 3 (+2)

**Recommendations:**
- Consider breaking down large functions
- Reduce nesting depth in analyze_content()
</details>
```

#### Future: Advanced Features
- **Complexity heatmap visualization** (image generation)
- **AI-powered refactoring suggestions** (via LLM API)
- **Integration with code review tools** (GitHub Copilot, ReviewDog)
- **Slack/Discord notifications** for critical regressions
- **Custom webhook support** for enterprise workflows

### Usage for Other Projects

Once implemented, other projects can use Lucent in their PRs:

```yaml
# Add to .github/workflows/code-quality.yml
- uses: ibrahimcesar/lucent-action@v1
  with:
    fail-on-complexity: 20
    fail-on-nesting: 5
    diff-only: true
    comment-pr: true
```

---

## 🚀 Implementation Priority Matrix

| Phase | Metrics | Rationale | Effort |
|-------|---------|-----------|--------|
| **Phase 2** | Cyclomatic, Function Length, Nesting, Commit Size, Parameter Count | Quick wins, regex-based, widely understood | 🟢 Low |
| **Phase 3** | Cognitive, Maintainability, Halstead, Churn, Hotspots, Test Ratio | High value, moderate complexity, requires AST | 🟡 Medium |
| **Phase 4** | Change Coupling, SPACE basics, Module Complexity, Coupling metrics | Advanced, multi-pass analysis | 🔴 High |
| **Future** | Team metrics, DORA, Security scanning, Performance profiling | Specialized, requires integrations | 🔴 Very High |

---

**Last Updated:** 2025-11-23
**Maintained By:** [@ibrahimcesar](https://github.com/ibrahimcesar)
**Questions?** Open an issue or discussion on GitHub
