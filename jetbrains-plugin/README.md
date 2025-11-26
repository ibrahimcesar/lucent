# Lucent JetBrains Plugin

A JetBrains IDE plugin that integrates [Lucent](https://github.com/ibrahimcesar/lucent) code metrics analysis directly into your IDE.

## Features

- **Real-time Metrics** - View complexity, maintainability, and other metrics for the current file
- **Code Inspections** - Warnings for high complexity, low maintainability, and deep nesting
- **Gutter Icons** - Visual indicators showing file health at a glance
- **Project Analysis** - Analyze entire project and identify problematic files
- **Tool Window** - Dedicated panel showing metrics dashboard

## Requirements

- **JetBrains IDE** - IntelliJ IDEA, WebStorm, PyCharm, GoLand, or any other JetBrains IDE (2023.3+)
- **Lucent CLI** - Must be installed and available in PATH

### Installing Lucent CLI

```bash
cargo install lucent-code
```

## Installation

### From JetBrains Marketplace

1. Open your JetBrains IDE
2. Go to **Settings/Preferences** > **Plugins**
3. Search for "Lucent"
4. Click **Install**

### Manual Installation

1. Download the latest release from [GitHub Releases](https://github.com/ibrahimcesar/lucent/releases)
2. In your IDE, go to **Settings/Preferences** > **Plugins**
3. Click the gear icon and select **Install Plugin from Disk...**
4. Select the downloaded `.zip` file

## Usage

### Analyze Current File

- Use the keyboard shortcut: `Ctrl+Alt+L` (Windows/Linux) or `Cmd+Alt+L` (macOS)
- Right-click in the editor and select **Analyze with Lucent**
- Go to **Analyze** > **Analyze with Lucent**

### Analyze Entire Project

- Go to **Analyze** > **Analyze Project with Lucent**

### View Metrics

- Open the **Lucent Metrics** tool window (on the right sidebar)
- The "Current File" tab shows metrics for the active file
- The "Project" tab shows overall project statistics

## Configuration

Go to **Settings/Preferences** > **Tools** > **Lucent** to configure:

- **Lucent executable path** - Custom path to lucent CLI (leave empty to use PATH)
- **Feature toggles** - Enable/disable inspections, gutter icons, auto-analysis
- **Thresholds** - Customize when warnings are shown

### Threshold Settings

| Setting | Default | Description |
|---------|---------|-------------|
| Cyclomatic Complexity | 20 | Warning shown above this value |
| Cognitive Complexity | 15 | Warning shown above this value |
| Nesting Depth | 4 | Warning shown above this depth |
| Maintainability Index | 50 | Warning shown below this value |
| Function Length | 50 | Warning shown above this many lines |

## Metrics Explained

### Cyclomatic Complexity
Measures the number of independent paths through code. Lower is better.
- 1-10: Simple, low risk
- 11-20: Moderate complexity
- 21-30: High complexity
- 31+: Very high risk

### Maintainability Index
Composite score (0-100) indicating how maintainable the code is.
- 85-100: Grade A - Excellent
- 65-84: Grade B - Good
- 50-64: Grade C - Moderate
- 25-49: Grade D - Poor
- 0-24: Grade F - Unmaintainable

### Nesting Depth
Maximum level of nested control structures.
- 1-3: Good
- 4: Acceptable
- 5+: Consider refactoring

## Development

### Building from Source

```bash
cd jetbrains-plugin
./gradlew build
```

### Running in Development IDE

```bash
./gradlew runIde
```

### Creating Distribution

```bash
./gradlew buildPlugin
```

The plugin zip will be in `build/distributions/`.

## Supported Languages

The plugin supports all languages that Lucent analyzes:
- Rust
- Python
- JavaScript/TypeScript
- Go
- Java
- And more...

## Troubleshooting

### "Lucent not found" error

1. Verify Lucent is installed: `lucent --version`
2. If using a custom path, configure it in Settings > Tools > Lucent
3. Restart your IDE

### Metrics not updating

1. Click the refresh button in the Lucent tool window
2. Or use **Analyze** > **Analyze with Lucent**

### Performance issues

- Disable "Analyze files on save" in settings
- Use project analysis sparingly on large codebases

## Contributing

Contributions are welcome! Please see the [main Lucent repository](https://github.com/ibrahimcesar/lucent) for contribution guidelines.

## License

MIT License - see [LICENSE](../LICENSE) for details.
