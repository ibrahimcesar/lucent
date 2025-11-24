//! Git command handlers for main.rs
//!
//! This file contains the implementation of Git subcommands.

use anyhow::{Context, Result};
use colored::Colorize;
use lucent::{
    Config,
    git::{
        DiffAnalyzer, CommitAnalyzer, TrendTracker, BlameAnalyzer,
        diff::DiffMode,
        trends::MetricType,
    },
};

/// Handle Git subcommands
pub fn handle_git_command(command: &lucent::config::GitCommands, config: &Config) -> Result<()> {
    use lucent::config::GitCommands;

    match command {
        GitCommands::Diff { base, uncommitted } => cmd_git_diff(config, base.as_deref(), *uncommitted),
        GitCommands::History { limit, skip } => cmd_git_history(config, *limit, *skip),
        GitCommands::Trends { limit, metric, height } => cmd_git_trends(config, *limit, metric, *height),
        GitCommands::Blame { file, top } => cmd_git_blame(config, file.as_ref().map(|p| p.as_path()), *top),
    }
}

/// Analyze changed files (diff mode)
fn cmd_git_diff(config: &Config, base: Option<&str>, uncommitted: bool) -> Result<()> {
    println!("📊 {}", "Git Diff Analysis".bold());
    println!();

    let analyzer = DiffAnalyzer::new(&config.path)
        .context("Failed to initialize Git analyzer. Is this a Git repository?")?;

    // Determine diff mode
    let mode = if uncommitted {
        DiffMode::WorkingDirectory
    } else if let Some(base_ref) = base {
        DiffMode::Since(base_ref.to_string())
    } else {
        DiffMode::MainBranch
    };

    println!("Mode: {}", format!("{:?}", mode).cyan());
    println!();

    // Analyze
    let analysis = analyzer.analyze(mode)?;
    let summary = analyzer.summary(&analysis);

    // Display summary
    println!("{}", summary);
    println!();

    // Display per-file metrics
    if config.per_file && !analysis.metrics.is_empty() {
        println!("📁 {}", "Changed Files:".bold());
        println!();

        for (path, metrics) in &analysis.metrics {
            println!("  {} {}", "•".bright_black(), path.display().to_string().cyan());
            println!("    Lines: {}, Code: {}, Complexity: {:.1}",
                metrics.total_lines,
                metrics.code_lines,
                metrics.cyclomatic_complexity
            );
        }
        println!();
    }

    Ok(())
}

/// Analyze commit history
fn cmd_git_history(config: &Config, limit: usize, skip: usize) -> Result<()> {
    println!("📜 {}", "Commit History Analysis".bold());
    println!();

    let analyzer = CommitAnalyzer::new(&config.path)
        .context("Failed to initialize Git analyzer. Is this a Git repository?")?;

    println!("Analyzing {} commits (skipping {} between analyses)...", limit, skip);
    println!();

    let history = analyzer.analyze_history(limit, skip)?;

    // Display commit analyses
    for analysis in &history.analyses {
        let commit_short = &analysis.commit.short_hash;
        let message = analysis.commit.message.lines().next().unwrap_or("");

        println!("{} {} {}",
            commit_short.yellow(),
            message.bright_black(),
            format!("({})", analysis.commit.author).bright_black()
        );
        println!("  Files: {}, Lines: {}, Complexity: {:.1}, MI: {:.0}",
            analysis.metrics.total_files,
            analysis.metrics.total_lines(),
            analysis.metrics.avg_complexity(),
            analysis.metrics.avg_maintainability()
        );
        println!();
    }

    // Display trend
    let trend = history.trend();
    println!("📈 Overall Trend: {}", trend);
    println!();

    Ok(())
}

/// Show trend analysis with visualization
fn cmd_git_trends(config: &Config, limit: usize, metric_name: &str, height: usize) -> Result<()> {
    println!("📈 {}", "Trend Analysis".bold());
    println!();

    let analyzer = CommitAnalyzer::new(&config.path)
        .context("Failed to initialize Git analyzer. Is this a Git repository?")?;

    println!("Analyzing {} commits for trends...", limit);
    println!();

    let history = analyzer.analyze_history(limit, 0)?;
    let tracker = TrendTracker::new(history);

    // Parse metric type
    let metric_type = match metric_name.to_lowercase().as_str() {
        "complexity" => MetricType::Complexity,
        "maintainability" => MetricType::Maintainability,
        "lines" => MetricType::Lines,
        _ => {
            eprintln!("{} Invalid metric: {}", "Error:".red().bold(), metric_name);
            eprintln!("Valid options: complexity, maintainability, lines");
            anyhow::bail!("Invalid metric type");
        }
    };

    // Generate and display ASCII chart
    let chart = tracker.ascii_chart(metric_type, height);
    println!("{}", chart);
    println!();

    // Generate and display report
    let report = tracker.generate_report();
    println!("{}", report);
    println!();

    Ok(())
}

/// Show blame analysis (complexity attribution)
fn cmd_git_blame(config: &Config, file: Option<&std::path::Path>, top: usize) -> Result<()> {
    println!("👥 {}", "Blame Analysis - Complexity Attribution".bold());
    println!();

    let analyzer = BlameAnalyzer::new(&config.path)
        .context("Failed to initialize Git analyzer. Is this a Git repository?")?;

    if let Some(file_path) = file {
        // Analyze single file
        let analysis = analyzer.analyze_file(file_path)?;
        println!("{}", analysis);
    } else {
        // Analyze project
        println!("Analyzing project (this may take a while)...");
        println!();

        // Get all source files
        use lucent::Scanner;
        let scanner = Scanner::new(&config.path)?;
        let files = scanner.scan()?;

        let file_paths: Vec<&std::path::Path> = files.iter()
            .map(|f| f.path.as_path())
            .collect();

        let analysis = analyzer.analyze_project(&file_paths)?;

        // Display top contributors
        println!("📊 Top {} Contributors by Complexity:", top);
        println!();

        for (i, contributor) in analysis.contributors.iter().take(top).enumerate() {
            println!("{}. {} {}",
                (i + 1).to_string().yellow(),
                contributor.author.cyan(),
                format!("({:.1} complexity units)", contributor.complexity_contribution).bright_black()
            );
            println!("   {} lines across {} files",
                contributor.total_lines,
                contributor.files_contributed
            );
            println!();
        }
    }

    Ok(())
}
