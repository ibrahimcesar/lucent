# Lucent Example Plugin

This is an example dynamic plugin for Lucent demonstrating how to create plugins that can be loaded at runtime.

## Building

```bash
cd examples/plugins/example_plugin
cargo build --release
```

This will create a shared library in `target/release/`:

| Platform | File |
|----------|------|
| Linux | `liblucent_example_plugin.so` |
| macOS | `liblucent_example_plugin.dylib` |
| Windows | `lucent_example_plugin.dll` |

## Installing

Copy the shared library to one of Lucent's plugin directories:

```bash
# User plugins (recommended)
mkdir -p ~/.local/share/lucent/plugins
cp target/release/liblucent_example_plugin.* ~/.local/share/lucent/plugins/

# Or current directory
mkdir -p ./plugins
cp target/release/liblucent_example_plugin.* ./plugins/
```

## How It Works

The plugin:

1. Implements the `DynamicPlugin` trait
2. Uses `lucent::declare_plugin!()` macro to export FFI symbols
3. Gets loaded by Lucent's `DynamicPluginManager`

## Creating Your Own Plugin

1. Copy this directory as a template
2. Update `Cargo.toml` with your plugin name
3. Implement `DynamicPlugin` in `src/lib.rs`
4. Build and install

## Plugin API

```rust
pub trait DynamicPlugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;

    /// Get list of languages this plugin supports
    fn supported_languages(&self) -> Vec<String>;

    /// Analyze source code and produce metrics
    fn analyze(&self, path: &Path, source: &str, language: &str) -> Option<FileMetrics>;

    /// Get plugin priority (higher = used first)
    fn priority(&self) -> u32 { 50 }

    /// Called when plugin is being unloaded
    fn on_unload(&self) {}
}
```

## Required Exports

The `declare_plugin!` macro generates these required exports:

- `lucent_plugin_api_version() -> u32`
- `lucent_plugin_create() -> *mut dyn DynamicPlugin`
- `lucent_plugin_destroy(*mut dyn DynamicPlugin)`
