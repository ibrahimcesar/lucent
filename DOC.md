# Project structure

```bash
lucent/
├── .gitignore
├── Cargo.toml
├── LICENSE-MIT
├── LICENSE-APACHE
├── README.md
├── LANGUAGES.md
├── CONTRIBUTING.md (to be created)
├── CHANGELOG.md (to be created)
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── metrics/
│   │   └── mod.rs (to be created)
│   ├── language/
│   │   └── mod.rs (to be created)
│   ├── scanner/
│   │   └── mod.rs (to be created)
│   ├── output/
│   │   └── mod.rs (to be created)
│   └── config/
│       └── mod.rs (to be created)
├── tests/
│   └── integration_test.rs (to be created)
└── examples/
    └── basic_usage.rs (to be created)
```

## Quick start commands

```bash
# Create project
cargo new lucent
cd lucent

# Copy the files above into their respective locations

# Test that it compiles
cargo build

# Run the empty CLI
cargo run

# Output:
# 💎 Lucent - Crystal clear code insights
#
# Analyzing: .
# Format: Pretty
#
# 🚧 Implementation coming soon...

# Run tests
cargo test

# Build release
cargo build --release
```

## Developemnt tips

```bash
# Watch mode during development
cargo install cargo-watch
cargo watch -x run

# Format code
cargo fmt

# Lint
cargo clippy

# Run specific test
cargo test test_name

# Documentation
cargo doc --open
```

## Next steps

- [ ] Set up project structure
- [ ] Implement basic file scanner (walkdir)
- [ ] Add language detection (by extension)
- [ ] Implement simple line counting
- [ ] Add first language analyzer (Rust)
- [ ] Create pretty terminal output
- [ ] Add JSON output format
- [ ] Write integration tests
- [ ] Add GitHub Actions CI
- [ ] Publish v0.1.0-alpha
