//! 💎 Lucent CLI entry point
//!
//! Crystal clear code insights - Fast code metrics analyzer

use anyhow::{Context, Result};
use colored::Colorize;

use lucent::{
    Config, Scanner, FileMetrics, ProjectMetrics, display_metrics,
    Snapshot, SnapshotManager, SnapshotComparison,
    ValidationResult, ThresholdViolation, Severity,
};

mod main_git;

fn main() {
    // Parse CLI arguments
    let config = Config::parse_args();

    // Handle subcommands
    if let Some(command) = &config.command {
        match handle_command(command, &config) {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                eprintln!("{} {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
    }

    // Default behavior: analyze metrics
    match run_analysis(&config) {
        Ok(exit_code) => std::process::exit(exit_code),
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}

/// Handle subcommands
fn handle_command(command: &lucent::config::Commands, config: &Config) -> Result<()> {
    match command {
        lucent::config::Commands::Snapshot { command } => handle_snapshot_command(command, config),
        lucent::config::Commands::Git { command } => main_git::handle_git_command(command, config),
    }
}

/// Handle snapshot subcommands
fn handle_snapshot_command(command: &lucent::config::SnapshotCommands, config: &Config) -> Result<()> {
    use lucent::config::SnapshotCommands;

    match command {
        SnapshotCommands::Save { description } => cmd_snapshot_save(config, description.clone()),
        SnapshotCommands::List { limit } => cmd_snapshot_list(*limit),
        SnapshotCommands::Compare { base, current } => cmd_snapshot_compare(base, current),
        SnapshotCommands::Delete { id } => cmd_snapshot_delete(id),
        SnapshotCommands::Cleanup { keep } => cmd_snapshot_cleanup(*keep),
    }
}

/// Save a snapshot of current metrics
fn cmd_snapshot_save(config: &Config, description: Option<String>) -> Result<()> {
    println!("📸 {}", "Saving snapshot...".bright_cyan());
    println!();

    // Run analysis
    let metrics = analyze_project(config)?;

    // Create snapshot
    let snapshot = Snapshot::new(metrics, description.clone())
        .context("Failed to create snapshot")?;

    // Save snapshot
    let manager = SnapshotManager::new()?;
    let path = manager.save(&snapshot)?;

    println!("{} Snapshot saved", "✓".green().bold());
    println!("  {} {}", "ID:".bright_black(), snapshot.id.cyan());
    println!("  {} {}", "Path:".bright_black(), path.display().to_string().bright_black());

    if let Some(git) = &snapshot.git {
        println!("  {} {} ({})",
            "Git:".bright_black(),
            git.branch.yellow(),
            git.commit_hash.bright_black()
        );
        if git.dirty {
            println!("  {} {}", "Status:".bright_black(), "dirty (uncommitted changes)".yellow());
        }
    }

    if let Some(desc) = &description {
        println!("  {} {}", "Description:".bright_black(), desc);
    }

    let metadata = snapshot.metadata();
    println!("  {} {} files, {} lines, complexity {:.1}, MI {:.0}",
        "Metrics:".bright_black(),
        metadata.total_files,
        metadata.total_lines,
        metadata.avg_complexity,
        metadata.avg_maintainability
    );

    println!();
    Ok(())
}

/// List all snapshots
fn cmd_snapshot_list(limit: Option<usize>) -> Result<()> {
    let manager = SnapshotManager::new()?;
    let snapshots = manager.list()?;

    if snapshots.is_empty() {
        println!("{} No snapshots found", "ℹ".bright_blue());
        println!();
        println!("  Create your first snapshot with: {}", "lucent snapshot save".cyan());
        println!();
        return Ok(());
    }

    println!("📸 {} ({} total)", "Snapshots".bold(), snapshots.len());
    println!();

    let to_show = if let Some(n) = limit {
        snapshots.iter().take(n).collect::<Vec<_>>()
    } else {
        snapshots.iter().collect::<Vec<_>>()
    };

    for snapshot in to_show {
        println!("  {} {}", "ID:".bright_black(), snapshot.id.cyan());
        println!("    {} {}",
            "Date:".bright_black(),
            snapshot.timestamp.format("%Y-%m-%d %H:%M:%S").to_string().bright_black()
        );

        if let Some(branch) = &snapshot.git_branch {
            print!("    {} {}", "Git:".bright_black(), branch.yellow());
            if let Some(commit) = &snapshot.git_commit {
                print!(" ({})", commit.bright_black());
            }
            println!();
        }

        if let Some(desc) = &snapshot.description {
            println!("    {} {}", "Description:".bright_black(), desc);
        }

        println!("    {} {} files, {} lines, complexity {:.1}, MI {:.0}",
            "Metrics:".bright_black(),
            snapshot.total_files,
            snapshot.total_lines,
            snapshot.avg_complexity,
            snapshot.avg_maintainability
        );

        println!();
    }

    if limit.is_some() && snapshots.len() > limit.unwrap() {
        println!("  {} Showing {} of {} snapshots",
            "ℹ".bright_blue(),
            limit.unwrap(),
            snapshots.len()
        );
        println!("  Use {} to see all", "lucent snapshot list".cyan());
        println!();
    }

    Ok(())
}

/// Compare two snapshots
fn cmd_snapshot_compare(base_id: &str, current_id: &str) -> Result<()> {
    let manager = SnapshotManager::new()?;

    println!("📊 {}", "Comparing snapshots...".bright_cyan());
    println!();

    // Load snapshots
    let base = manager.load(base_id)
        .with_context(|| format!("Failed to load base snapshot: {}", base_id))?;
    let current = manager.load(current_id)
        .with_context(|| format!("Failed to load current snapshot: {}", current_id))?;

    // Compare
    let comparison = SnapshotComparison::compare(&base, &current);

    // Display comparison
    println!("  {} {} → {}",
        "Snapshots:".bright_black(),
        base_id.yellow(),
        current_id.cyan()
    );
    println!("  {} {} → {}",
        "Dates:".bright_black(),
        base.timestamp.format("%Y-%m-%d %H:%M:%S").to_string().bright_black(),
        current.timestamp.format("%Y-%m-%d %H:%M:%S").to_string().bright_black()
    );
    println!();

    // Overall metrics
    println!("📈 {}", "Overall Changes".bold());
    println!();

    display_metric_delta("Files", &comparison.overall.files);
    display_metric_delta("Lines", &comparison.overall.lines);
    display_metric_delta("Code Lines", &comparison.overall.code_lines);
    display_metric_delta("Avg Complexity", &comparison.overall.avg_complexity);
    display_metric_delta("Avg Nesting", &comparison.overall.avg_nesting);
    display_metric_delta("Maintainability", &comparison.overall.avg_maintainability);
    display_metric_delta("Code Smells", &comparison.overall.code_smells);

    println!();

    // Quality score
    let score_color = if comparison.quality_score >= 90.0 {
        "green"
    } else if comparison.quality_score >= 70.0 {
        "yellow"
    } else {
        "red"
    };

    println!("⭐ {} {:.1}/100 - {}",
        "Quality Score:".bold(),
        comparison.quality_score,
        match score_color {
            "green" => comparison.quality_description().green(),
            "yellow" => comparison.quality_description().yellow(),
            _ => comparison.quality_description().red(),
        }
    );
    println!();

    Ok(())
}

/// Display a metric delta
fn display_metric_delta<T: std::fmt::Display>(label: &str, delta: &lucent::snapshot::MetricDelta<T>) {
    let trend_emoji = delta.trend.emoji();
    let change_str = if delta.percent_change >= 0.0 {
        format!("+{:.1}%", delta.percent_change)
    } else {
        format!("{:.1}%", delta.percent_change)
    };

    let change_colored = match delta.trend {
        lucent::snapshot::TrendDirection::Improved => change_str.green(),
        lucent::snapshot::TrendDirection::Neutral => change_str.yellow(),
        lucent::snapshot::TrendDirection::Regressed => change_str.red(),
    };

    println!("  {} {} {} → {} ({})",
        trend_emoji,
        format!("{:20}", label).bright_black(),
        delta.base.to_string().bright_black(),
        delta.current.to_string().cyan(),
        change_colored
    );
}

/// Delete a snapshot
fn cmd_snapshot_delete(id: &str) -> Result<()> {
    let manager = SnapshotManager::new()?;

    // Verify snapshot exists
    manager.load(id)
        .with_context(|| format!("Snapshot not found: {}", id))?;

    // Delete
    manager.delete(id)?;

    println!("{} Snapshot deleted: {}", "✓".green().bold(), id.yellow());
    println!();

    Ok(())
}

/// Clean up old snapshots
fn cmd_snapshot_cleanup(keep: usize) -> Result<()> {
    let manager = SnapshotManager::new()?;

    let total_before = manager.count()?;
    let deleted = manager.cleanup(keep)?;

    println!("{} Cleaned up {} snapshots (kept {})",
        "✓".green().bold(),
        deleted,
        total_before - deleted
    );
    println!();

    Ok(())
}

/// Run the analysis
fn run_analysis(config: &Config) -> Result<i32> {
    // Validate configuration
    if let Err(e) = config.validate() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        return Ok(1);
    }

    // Show header
    if !matches!(config.format, lucent::config::OutputFormat::Json) {
        println!();
        println!("💎 {} {}", "Lucent".bold().bright_white(), "v0.1.0-alpha".bright_black());
        println!("{}", "Crystal clear code insights".bright_black());
        println!();
    }

    let metrics = analyze_project(config)?;

    // Display results
    let output_format = config.format.into();
    display_metrics(&metrics, output_format);

    // Threshold validation
    let threshold_config = config.threshold_config();
    if threshold_config.has_thresholds() {
        println!();
        let validation_result = threshold_config.validate(&metrics);
        let exit_code = display_validation_result(&validation_result);
        return Ok(exit_code);
    }

    Ok(0)
}

/// Analyze project and return metrics
fn analyze_project(config: &Config) -> Result<ProjectMetrics> {
    // Scan for source files
    if config.verbose {
        println!("{} {}", "📂 Scanning:".bright_cyan(), config.path.display());
    }

    let scanner = Scanner::new(&config.path)
        .context("Failed to create scanner")?;

    let files = scanner.scan()
        .context("Failed to scan directory")?;

    if files.is_empty() {
        eprintln!();
        eprintln!("{} No source files found in {}",
            "⚠️".yellow(),
            config.path.display().to_string().yellow()
        );
        eprintln!();
        anyhow::bail!("No source files found");
    }

    if config.verbose {
        println!("{} Found {} source files", "✓".green(), files.len());
        println!();
    }

    // Analyze metrics
    let mut project_metrics = ProjectMetrics::new();

    for source_file in &files {
        if config.verbose {
            println!("{} {}", "  Analyzing:".bright_black(), source_file.path.display());
        }

        match FileMetrics::analyze(&source_file.path, source_file.language) {
            Ok(metrics) => {
                project_metrics.add_file(source_file.language.name, &metrics);

                // Show per-file metrics if requested
                if config.per_file && !matches!(config.format, lucent::config::OutputFormat::Json) {
                    println!("    {} Lines: {}, Code: {}, Entropy: {:.2}",
                        source_file.language.emoji(),
                        metrics.total_lines,
                        metrics.code_lines,
                        metrics.entropy
                    );
                }
            }
            Err(e) => {
                if config.verbose {
                    eprintln!("    {} Failed to analyze: {}", "✗".red(), e);
                }
            }
        }
    }

    Ok(project_metrics)
}

/// Display threshold validation results and return exit code
fn display_validation_result(result: &ValidationResult) -> i32 {
    match result {
        ValidationResult::Passed => {
            println!("{} All quality thresholds passed", "✓".green().bold());
            println!();
            0
        }
        ValidationResult::Warning(violations) => {
            println!("{} Quality warnings detected", "⚠".yellow().bold());
            println!();
            display_violations(violations);
            println!();
            0
        }
        ValidationResult::Failed(violations) => {
            println!("{} Quality thresholds violated", "✗".red().bold());
            println!();
            display_violations(violations);
            println!();
            1
        }
    }
}

/// Display threshold violations
fn display_violations(violations: &[ThresholdViolation]) {
    for violation in violations {
        let (emoji, color_fn): (&str, fn(String) -> colored::ColoredString) = match violation.severity {
            Severity::Error => ("✗", |s: String| s.red()),
            Severity::Warning => ("⚠", |s: String| s.yellow()),
        };

        println!(
            "  {} {} {} (expected {}, got {})",
            emoji,
            color_fn(violation.metric.clone()),
            match violation.severity {
                Severity::Error => "FAILED".red().bold(),
                Severity::Warning => "WARNING".yellow().bold(),
            },
            violation.expected.bright_black(),
            violation.actual.cyan()
        );
    }
}
