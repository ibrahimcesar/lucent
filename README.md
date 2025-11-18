<div align="center">
  
# Lucent 💎

_Crystal clear code insights_ 

**Lucent** is a fast, extensible code metrics analyzer built in Rust. It helps you understand code quality, complexity, and maintainability across multiple programming languages.

</div>

## ✨ Features (Planned)

- 🚀 **Fast** - Analyze large codebases in seconds
- 🌍 **Multi-language** - Support for 25+ programming languages
- 📊 **Rich Metrics** - Cyclomatic complexity, cognitive complexity, maintainability index
- 🔍 **Code Smells** - Detect TODOs, FIXMEs, long functions, deep nesting
- 🎨 **Multiple Outputs** - JSON, Markdown, HTML, CSV, and pretty terminal output
- 🔌 **Extensible** - Plugin system for custom languages and metrics
- ⚡ **CI/CD Ready** - Perfect for quality gates and code review automation

## 🚧 Status

**Work in Progress** - This project is in early development.

Current version: `0.1.0-alpha`

## 🎯 Roadmap

### Phase 1: Core (Current)
- [ ] Basic file traversal and language detection
- [ ] Lines of code counting (code, comments, blanks)
- [ ] Simple complexity metrics (cyclomatic)
- [ ] Support for 5 languages (Rust, Python, JavaScript, Go, TypeScript)

### Phase 2: Metrics
- [ ] Cognitive complexity
- [ ] Halstead complexity
- [ ] Maintainability index
- [ ] Code duplication detection
- [ ] TODO/FIXME/HACK tracking

### Phase 3: Extensibility
- [ ] Plugin system (tree-sitter based)
- [ ] Support for 25+ languages
- [ ] Custom metric definitions
- [ ] Community plugin repository

### Phase 4: Developer Experience
- [ ] Interactive TUI mode
- [ ] HTML report generation
- [ ] Git integration (diff mode)
- [ ] Watch mode
- [ ] CI/CD integrations

## 🎓 Supported Languages (Target)

See [LANGUAGES.md](LANGUAGES.md) for the full list and implementation status.

**Tier 1 (Priority):** Rust, Python, JavaScript, TypeScript, Go, Java, C++, C#

**Tier 2:** PHP, Ruby, Kotlin, Swift, C, Dart, Scala, Elixir

**Tier 3:** Haskell, Clojure, Lua, Shell, SQL, Zig, F#, Julia, OCaml, Nim

## 🔧 Installation
```bash
# Not yet published to crates.io
# For now, build from source:
git clone https://github.com/yourusername/lucent
cd lucent
cargo build --release
```

## 🚀 Usage (Planned)
```bash
# Analyze a directory
lucent analyze src/

# Check against thresholds (CI/CD)
lucent check --max-complexity 10

# Output as JSON
lucent analyze src/ --format json

# Watch mode
lucent watch src/
```

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
