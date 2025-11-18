//! Lucent CLI entry point

use anyhow::Result;
use clap::Parser;

/// Crystal clear code insights
#[derive(Parser, Debug)]
#[command(name = "lucent")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to analyze
    #[arg(default_value = ".")]
    path: String,

    /// Output format
    #[arg(short, long, value_enum, default_value = "pretty")]
    format: OutputFormat,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum OutputFormat {
    Pretty,
    Json,
    Markdown,
    Csv,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("💎 Lucent - Crystal clear code insights");
    println!();
    println!("Analyzing: {}", cli.path);
    println!("Format: {:?}", cli.format);
    println!();
    println!("🚧 Implementation coming soon...");

    Ok(())
}
