# Supported Languages

This document tracks the implementation status of language support in Lucent.

## Implementation Status

- ✅ Implemented
- 🚧 In Progress  
- 📋 Planned
- ⏳ Backlog

---

## Tier 1: Priority Languages (MVP)

Core languages with the highest priority for initial release.

| Language | Status | Lines | Complexity | Comments | TODOs | Parser |
|----------|--------|-------|------------|----------|-------|--------|
| **Rust** | 📋 | - | - | - | - | tree-sitter |
| **Python** | 📋 | - | - | - | - | tree-sitter |
| **JavaScript** | 📋 | - | - | - | - | tree-sitter |
| **TypeScript** | 📋 | - | - | - | - | tree-sitter |
| **Go** | 📋 | - | - | - | - | tree-sitter |
| **Java** | 📋 | - | - | - | - | tree-sitter |
| **C++** | 📋 | - | - | - | - | tree-sitter |
| **C#** | 📋 | - | - | - | - | tree-sitter |

---

## Tier 2: High-Demand Languages

Popular languages to implement after Tier 1.

| Language | Status | Lines | Complexity | Comments | TODOs | Parser |
|----------|--------|-------|------------|----------|-------|--------|
| **PHP** | 📋 | - | - | - | - | tree-sitter |
| **Ruby** | 📋 | - | - | - | - | tree-sitter |
| **Kotlin** | 📋 | - | - | - | - | tree-sitter |
| **Swift** | 📋 | - | - | - | - | tree-sitter |
| **C** | 📋 | - | - | - | - | tree-sitter |
| **Dart** | 📋 | - | - | - | - | tree-sitter |
| **Scala** | 📋 | - | - | - | - | tree-sitter |
| **Elixir** | 📋 | - | - | - | - | tree-sitter |

---

## Tier 3: Specialized & Loved Languages

Languages with strong communities or specific use cases.

| Language | Status | Lines | Complexity | Comments | TODOs | Parser |
|----------|--------|-------|------------|----------|-------|--------|
| **Haskell** | ⏳ | - | - | - | - | tree-sitter |
| **Clojure** | ⏳ | - | - | - | - | tree-sitter |
| **Lua** | ⏳ | - | - | - | - | tree-sitter |
| **Shell/Bash** | ⏳ | - | - | - | - | tree-sitter |
| **SQL** | ⏳ | - | - | - | - | tree-sitter |
| **Zig** | ⏳ | - | - | - | - | tree-sitter |
| **F#** | ⏳ | - | - | - | - | tree-sitter |
| **Julia** | ⏳ | - | - | - | - | tree-sitter |
| **OCaml** | ⏳ | - | - | - | - | tree-sitter |
| **Nim** | ⏳ | - | - | - | - | tree-sitter |

---

## Additional Languages (Extended Support)

| Language | Status | Lines | Complexity | Comments | TODOs | Parser |
|----------|--------|-------|------------|----------|-------|--------|
| **HTML** | ⏳ | - | - | - | - | tree-sitter |
| **CSS** | ⏳ | - | - | - | - | tree-sitter |
| **SCSS** | ⏳ | - | - | - | - | tree-sitter |
| **JSON** | ⏳ | - | - | - | - | tree-sitter |
| **YAML** | ⏳ | - | - | - | - | tree-sitter |
| **TOML** | ⏳ | - | - | - | - | tree-sitter |
| **Markdown** | ⏳ | - | - | - | - | tree-sitter |
| **XML** | ⏳ | - | - | - | - | tree-sitter |
| **Erlang** | ⏳ | - | - | - | - | tree-sitter |
| **Crystal** | ⏳ | - | - | - | - | tree-sitter |
| **R** | ⏳ | - | - | - | - | tree-sitter |
| **Objective-C** | ⏳ | - | - | - | - | tree-sitter |
| **Perl** | ⏳ | - | - | - | - | tree-sitter |
| **PowerShell** | ⏳ | - | - | - | - | tree-sitter |
| **Assembly** | ⏳ | - | - | - | - | regex |
| **Fortran** | ⏳ | - | - | - | - | regex |

---

## Language Support Roadmap

### Phase 1 (v0.1 - v0.3)
**Focus:** Core languages with tree-sitter

- ✅ Basic file detection and extension mapping
- ✅ Simple line counting (code, comments, blanks)
- 🚧 Implement Tier 1 languages (8 languages)
  - Rust, Python, JavaScript, TypeScript, Go, Java, C++, C#

### Phase 2 (v0.4 - v0.6)
**Focus:** Expand coverage and improve metrics

- 📋 Implement Tier 2 languages (8 languages)
- 📋 Add cyclomatic complexity calculation
- 📋 Add TODO/FIXME detection

### Phase 3 (v0.7 - v0.9)
**Focus:** Specialized languages and plugin system

- 📋 Implement Tier 3 languages (10 languages)
- 📋 Plugin system for community contributions
- 📋 Advanced metrics (cognitive complexity, Halstead)

### Phase 4 (v1.0+)
**Focus:** Long tail and community-driven

- 📋 Additional languages based on demand
- 📋 Community plugins
- 📋 Custom language definitions via config

---

## How Languages Are Prioritized

Languages are prioritized based on:

1. **Usage** - Stack Overflow Developer Survey (Most Used)
2. **Love** - Stack Overflow Developer Survey (Most Loved/Admired)
3. **Tree-sitter availability** - Parser quality and maintenance
4. **Community requests** - GitHub issues and discussions
5. **Complexity needs** - Languages where complexity analysis is valuable

---

## Adding a New Language

To request support for a new language:

1. **Check** if tree-sitter parser exists: https://tree-sitter.github.io/tree-sitter/
2. **Open an issue** with:
   - Language name
   - Use case / why it's important
   - Links to tree-sitter parser (if available)
3. **Optional:** Submit a PR implementing basic support

See [CONTRIBUTING.md](CONTRIBUTING.md) for implementation guidelines.

---

## Language Detection

Lucent detects languages using:

1. **File extensions** (primary method)
2. **Shebang lines** (for scripts: `#!/usr/bin/env python`)
3. **Filename patterns** (e.g., `Makefile`, `Dockerfile`)
4. **Content inspection** (fallback for ambiguous files)

### Example Extension Mapping
```
.rs, .rslib          → Rust
.py, .pyi, .pyw      → Python
.js, .mjs, .cjs      → JavaScript
.ts, .tsx            → TypeScript
.go                  → Go
.java                → Java
.cpp, .cc, .cxx, .hpp → C++
.cs                  → C#
```

---

## Notes

- **Tree-sitter** is used for robust parsing where available
- **Regex-based** fallback for languages without tree-sitter support
- **Plugin system** (planned) will allow community to add any language
- Some languages (HTML, CSS, Markdown) have limited complexity metrics

---

**Last Updated:** 2025-11-18
**Languages Implemented:** 0 / 40+
**Next Target:** Rust, Python, JavaScript (Tier 1 MVP)
