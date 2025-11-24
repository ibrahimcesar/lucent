# Lucent - Code Metrics & Quality for VS Code

> Crystal clear code insights - Real-time code metrics, complexity analysis, and quality monitoring directly in VS Code

[![Version](https://img.shields.io/visual-studio-marketplace/v/lucent.lucent-code-metrics)](https://marketplace.visualstudio.com/items?itemName=lucent.lucent-code-metrics)
[![Installs](https://img.shields.io/visual-studio-marketplace/i/lucent.lucent-code-metrics)](https://marketplace.visualstudio.com/items?itemName=lucent.lucent-code-metrics)
[![Rating](https://img.shields.io/visual-studio-marketplace/r/lucent.lucent-code-metrics)](https://marketplace.visualstudio.com/items?itemName=lucent.lucent-code-metrics)

## Features Overview

### 1. 📊 Inline Code Lenses

**What it does**: Displays real-time metrics at the top of each file you're editing

**Metrics shown**:
- **Cyclomatic Complexity**: Measures code branching and decision points
- **Maintainability Index**: 0-100 score (higher is better)
  - 🟢 85-100: Excellent
  - 🟡 65-84: Good
  - 🔴 0-64: Needs improvement
- **Lines of Code**: Total code lines, comments, and blank lines
- **Code Smells**: Number of quality issues detected

**How to use**:
- Metrics appear automatically when you open a file
- Click on any metric to open the detailed metrics panel
- Toggle on/off: `Cmd+Shift+P` → "Lucent: Toggle Inline Diagnostics"

**Screenshot**:
```typescript
// $(symbol-method) Complexity: 8.5  $(beaker) Maintainability: 72 🟡  $(file-code) 145 LOC  $(warning) 2 Code Smells

function analyzeCode(data: string[]) {
    // Your code here
}
```

---

### 2. 🔍 Hover Tooltips

**What it does**: Shows detailed metrics when you hover over code

**Information displayed**:
- **Function-level metrics** (when hovering over function names)
- **File-level summary** (when hovering over imports/exports)
- **Complexity breakdown** with explanation
- **Code quality tips** and suggestions

**How to use**:
- Simply hover your mouse over any function, class, or import
- Wait 500ms for the tooltip to appear
- Tooltip stays visible while hovering

**Example tooltip content**:
```
Metrics for calculateTotal():
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Cyclomatic Complexity: 12
├─ Decision points: 8
├─ Nested conditionals: 4
└─ Recommendation: Consider breaking into smaller functions

Maintainability Index: 58 🔴
├─ Low score indicates refactoring needed
└─ Target: Aim for 65+

Lines of Code: 87
├─ Code: 72
├─ Comments: 10
└─ Blank: 5
```

---

### 3. ⚠️ Real-time Diagnostics

**What it does**: Displays warnings and errors for code quality issues

**Types of diagnostics**:
- **High Complexity**: Functions exceeding complexity threshold
- **Deep Nesting**: Code with excessive nesting levels (>4)
- **Low Maintainability**: Files below maintainability threshold
- **Long Functions**: Functions exceeding line count limits
- **Magic Numbers**: Hard-coded values without constants

**Severity levels**:
- 🔴 **Error**: Critical issues (configurable)
- 🟡 **Warning**: Should be addressed (default)
- 🔵 **Info**: Nice-to-have improvements
- 💡 **Hint**: Suggestions

**How to use**:
- Diagnostics appear automatically as you type (on save)
- View all issues: `View` → `Problems` panel
- Configure severity: Settings → `lucent.diagnosticSeverity`
- Set thresholds: Settings → `lucent.thresholds`

**Configuration example**:
```json
{
  "lucent.thresholds": {
    "maxComplexity": 10,
    "maxNesting": 4,
    "minMaintainability": 65
  },
  "lucent.diagnosticSeverity": "warning"
}
```

---

### 4. 📈 Status Bar Indicators

**What it does**: Shows current file metrics in the VS Code status bar

**Display modes**:
- **Current file metrics** (when file is active)
- **Workspace summary** (when no file is active)
- **Color-coded indicators** based on quality thresholds

**Metrics shown**:
- Complexity score with color indicator
- Maintainability index
- Quick action buttons

**How to use**:
- Automatically updates when switching files
- Click the status bar item to analyze the current file
- Right-click for quick actions menu
- Toggle visibility: Settings → `lucent.showStatusBar`

**Status bar example**:
```
💎 Lucent: Complexity 8.5 | MI 72 🟡 | [Analyze]
```

---

### 5. 🗂️ Metrics Sidebar

**What it does**: Dedicated sidebar panel showing project-wide metrics

**Panels available**:

#### File Metrics View
- **Current file analysis** with drill-down details
- **Function-level metrics** (if supported)
- **Code smell list** with quick fixes
- **Trend indicators** (improving/degrading)

#### Project Overview
- **Language breakdown** (Rust, TypeScript, Python, etc.)
- **Top 10 most complex files** with scores
- **Quality distribution** (excellent/good/poor)
- **Total project statistics**

#### Quality Trends (Git Integration)
- **Commit-by-commit** quality changes
- **Historical trends** visualization
- **Complexity over time** graph
- **Maintainability trajectory**

**How to use**:
- Click the Lucent icon in the Activity Bar (left sidebar)
- Expand/collapse sections to view details
- Click on any file to navigate
- Right-click for context actions

---

### 6. 🎯 Commands & Actions

**Available commands** (Cmd+Shift+P):

#### Analysis Commands
- **`Lucent: Analyze Current File`** - Analyze the active file
- **`Lucent: Analyze Entire Workspace`** - Full project analysis
- **`Lucent: Clear Analysis Cache`** - Force re-analysis

#### View Commands
- **`Lucent: Show Metrics Panel`** - Open detailed metrics webview
- **`Lucent: Show Git Trends`** - View quality trends over time
- **`Lucent: Toggle Inline Diagnostics`** - Show/hide code lenses

#### Configuration
- **`Lucent: Open Settings`** - Quick access to extension settings

**Keyboard shortcuts** (default):
- `Cmd+Shift+L` - Analyze current file
- `Cmd+Shift+M` - Show metrics panel
- `Cmd+Shift+T` - Show Git trends

---

### 7. 📊 Metrics Panel (Webview)

**What it does**: Interactive HTML panel with detailed visualizations

**Features**:
- **Interactive charts** showing metrics distribution
- **Complexity heatmap** of files
- **Function-level breakdown** with drill-down
- **Export to PDF/PNG** for reports
- **Historical comparison** view

**Sections**:
1. **Overview Dashboard**
   - Key metrics at a glance
   - Quality score
   - Trend indicators

2. **Complexity Analysis**
   - Function complexity distribution
   - Hot spots requiring attention
   - Refactoring suggestions

3. **Quality Metrics**
   - Maintainability index breakdown
   - Code smell categorization
   - Technical debt estimation

4. **Trends & History**
   - Git-based trend analysis
   - Quality trajectory
   - Commit impact visualization

**How to use**:
- Command: `Lucent: Show Metrics Panel`
- Click "View Details" in code lens
- Automatically opens for high-complexity files

---

### 8. 🌳 Git Integration Features

**What it does**: Integrates with Git for historical analysis

**Features**:

#### Commit Analysis
- **Analyze specific commits** to see quality at that point
- **Compare commits** to see quality impact
- **Blame integration** showing complexity contributors

#### Trend Tracking
- **Quality over time** visualization
- **Regression detection** (quality declining)
- **Improvement tracking** (quality improving)
- **ASCII charts** in terminal output

#### Diff Mode
- **Analyze only changed files** for faster PR reviews
- **Delta metrics** (before/after comparison)
- **Impact assessment** of changes

**Commands**:
- **`Lucent: Show Git Trends`** - View quality trends
- **`Lucent: Compare Commits`** - Compare two commits
- **`Lucent: Blame Complexity`** - See who wrote complex code

**Git views in sidebar**:
- Commit history with metrics
- Quality regression alerts
- Trend charts

---

### 9. ⚙️ Configuration Options

**All settings** (`settings.json`):

```json
{
  // Enable/disable extension
  "lucent.enabled": true,

  // Path to lucent executable
  "lucent.executablePath": "lucent",

  // Display options
  "lucent.showInlineMetrics": true,
  "lucent.showStatusBar": true,

  // Automatic analysis
  "lucent.analyzeOnSave": true,
  "lucent.analyzeOnOpen": false,

  // Quality thresholds
  "lucent.thresholds": {
    "maxComplexity": 10,
    "maxNesting": 4,
    "minMaintainability": 65,
    "maxFunctionLength": 50
  },

  // Diagnostic severity
  "lucent.diagnosticSeverity": "warning",

  // Performance
  "lucent.enableCache": true,

  // Git integration
  "lucent.git.enabled": true,
  "lucent.git.trendsLimit": 20,

  // Advanced
  "lucent.verbose": false,
  "lucent.timeout": 30000
}
```

**Workspace vs User settings**:
- Set in **User settings** for global defaults
- Override in **Workspace settings** for project-specific thresholds
- Example: stricter thresholds for critical projects

---

### 10. 💡 Best Practices & Tips

#### For Teams
- **Share `.vscode/settings.json`** with team thresholds
- **Set CI/CD gates** matching VS Code thresholds
- **Use consistent** `lucent.thresholds` across projects

#### For Performance
- **Enable caching** for faster analysis
- **Use `analyzeOnSave`** instead of `analyzeOnOpen`
- **Exclude large files** in `.gitignore`

#### For Quality
- **Start with relaxed thresholds**, tighten over time
- **Focus on** high-complexity functions first
- **Review trends** weekly to track progress

#### Keyboard Efficiency
- Learn keyboard shortcuts for common actions
- Use command palette (`Cmd+Shift+P`) for all commands
- Pin metrics panel for quick access

---

## Installation

### From VS Code Marketplace
1. Open VS Code
2. Press `Cmd+Shift+X` (Extensions)
3. Search for "Lucent"
4. Click "Install"

### Manual Installation
1. Download `.vsix` from [Releases](https://github.com/ibrahimcesar/lucent/releases)
2. `Cmd+Shift+P` → "Extensions: Install from VSIX..."
3. Select downloaded `.vsix` file

### Requirements
- **VS Code**: 1.85.0 or higher
- **Lucent CLI**: Must be installed and in PATH
  ```bash
  # Install Lucent CLI
  cargo install lucent-code

  # Verify installation
  lucent --version
  ```

---

## Usage Examples

### Example 1: Daily Development Workflow

```typescript
// 1. Open a file - inline metrics appear automatically
// 2. See complexity warning in file
// 3. Hover over function to see detailed metrics
// 4. Click "View Details" to open metrics panel
// 5. Refactor based on suggestions
// 6. Save file - metrics update automatically
// 7. Verify improvement in status bar
```

### Example 2: Code Review

```bash
# 1. Checkout PR branch
git checkout feature-branch

# 2. Run diff analysis
# Cmd+Shift+P → "Lucent: Analyze Current File"

# 3. Review changed files in metrics panel
# Focus on files with complexity increase

# 4. Leave comments on high-complexity functions
# Use metrics as objective data points
```

### Example 3: Refactoring Session

1. **Identify targets**: Open metrics panel → "Top Complex Files"
2. **Set goal**: Aim to reduce complexity by 20%
3. **Track progress**: Use Git trends to see improvement
4. **Verify**: Re-analyze after each refactoring

---

## Troubleshooting

### "Lucent executable not found"
- **Solution**: Set `lucent.executablePath` in settings
- **Check**: Run `which lucent` in terminal
- **Install**: `cargo install lucent-code`

### "Analysis takes too long"
- **Enable caching**: `lucent.enableCache`: true
- **Reduce scope**: Analyze specific files instead of workspace
- **Exclude directories**: Add to `.gitignore`

### "Metrics not updating"
- **Clear cache**: `Lucent: Clear Analysis Cache`
- **Restart extension**: Reload window (`Cmd+R`)
- **Check configuration**: Verify `lucent.enabled`: true

### "Git features not working"
- **Check Git installation**: `git --version`
- **Verify repository**: Must be in a Git repo
- **Enable in settings**: `lucent.git.enabled`: true

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

---

## License

MIT © [Ibrahim Cesar](https://github.com/ibrahimcesar)

---

## Links

- [📦 Marketplace](https://marketplace.visualstudio.com/items?itemName=lucent.lucent-code-metrics)
- [🐛 Issues](https://github.com/ibrahimcesar/lucent/issues)
- [📖 Documentation](https://ibrahimcesar.github.io/lucent/)
- [💬 Discussions](https://github.com/ibrahimcesar/lucent/discussions)

---

**Made with 💎 by the Lucent team**
