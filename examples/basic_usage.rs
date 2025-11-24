//! Basic usage example for Lucent library
//!
//! This example shows how to use Lucent as a library to analyze code metrics.

use anyhow::Result;
use lucent::{Scanner, FileMetrics, ProjectMetrics, display_metrics, OutputFormat};

fn main() -> Result<()> {
    println!("💎 Lucent Library Example\n");

    // 1. Create a scanner for the current directory
    println!("Step 1: Scanning for source files...");
    let scanner = Scanner::new(".")?;
    let files = scanner.scan()?;

    println!("  Found {} source files\n", files.len());

    // 2. Analyze each file and collect metrics
    println!("Step 2: Analyzing metrics...");
    let mut project_metrics = ProjectMetrics::new();

    for source_file in files.iter().take(5) {
        // Just analyze first 5 for demo
        println!("  {} Analyzing: {}",
            source_file.language.emoji(),
            source_file.path.display()
        );

        match FileMetrics::analyze(&source_file.path, source_file.language) {
            Ok(metrics) => {
                println!("    Lines: {}, Code: {}, Entropy: {:.2}",
                    metrics.total_lines,
                    metrics.code_lines,
                    metrics.entropy
                );
                project_metrics.add_file(source_file.language.name, &metrics);
            }
            Err(e) => {
                eprintln!("    Error: {}", e);
            }
        }
    }

    // 3. Display results
    println!("\nStep 3: Displaying results...\n");
    display_metrics(&project_metrics, OutputFormat::Pretty);

    // 4. Show JSON output
    println!("\n\nStep 4: JSON output example...\n");
    display_metrics(&project_metrics, OutputFormat::Json);

    Ok(())
}
