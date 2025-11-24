//! 💎 Lucent CLI entry point
//!
//! Crystal clear code insights - Fast code metrics analyzer

use anyhow::{Context, Result};
use colored::Colorize;
use rayon::prelude::*;

use lucent::{
    Config, Scanner, FileMetrics, ProjectMetrics, display_metrics,
    Snapshot, SnapshotManager, SnapshotComparison,
    ValidationResult, ThresholdViolation, Severity,
    cache::AnalysisCache,
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

    // Run analysis (we don't need violations for snapshots)
    let (metrics, _violations, _rules_config) = analyze_project(config)?;

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

    let (metrics, violations, rules_config_opt) = analyze_project(config)?;

    // Display results
    let output_format = config.format.into();
    display_metrics(&metrics, output_format);

    // Display custom rule violations if any
    let mut rules_exit_code = 0;
    if !violations.is_empty() {
        println!();
        rules_exit_code = display_custom_rule_violations(&violations, rules_config_opt.as_ref());
    }

    // Threshold validation
    let threshold_config = config.threshold_config();
    if threshold_config.has_thresholds() {
        println!();
        let validation_result = threshold_config.validate(&metrics);
        let threshold_exit_code = display_validation_result(&validation_result);

        // Return non-zero if either thresholds or rules failed
        return Ok(std::cmp::max(threshold_exit_code, rules_exit_code));
    }

    Ok(rules_exit_code)
}

/// Analyze project and return metrics along with custom rule violations
fn analyze_project(config: &Config) -> Result<(ProjectMetrics, Vec<lucent::Violation>, Option<lucent::config::RulesGlobalConfig>)> {
    // Scan for source files
    if config.verbose {
        println!("{} {}", "📂 Scanning:".bright_cyan(), config.path.display());
    }

    // Load custom rules from config file if available
    let mut rules_config_opt = None;
    let rule_engine = if let Some(config_path) = &config.config_file {
        match lucent::config::LucentConfig::from_file(config_path) {
            Ok(file_config) => {
                rules_config_opt = Some(file_config.rules_config.clone());
                if file_config.rules_config.enabled && !file_config.rules.is_empty() {
                let mut engine = lucent::RuleEngine::new();

                for rule_config in &file_config.rules {
                    // Validate rule config
                    if let Err(e) = rule_config.validate() {
                        eprintln!("{} Invalid rule '{}': {}", "⚠️".yellow(), rule_config.name, e);
                        continue;
                    }

                    // Handle pattern rules
                    if rule_config.rule_type == "pattern" {
                        if let Some(pattern) = &rule_config.pattern {
                            match lucent::rules::Severity::from_str(&rule_config.severity) {
                                Some(severity) => {
                                    match lucent::PatternRule::new(
                                        rule_config.name.clone(),
                                        rule_config.description.clone(),
                                        pattern.clone(),
                                        severity,
                                    ) {
                                        Ok(mut rule) => {
                                            if !rule_config.file_patterns.is_empty() {
                                                rule = rule.with_file_patterns(rule_config.file_patterns.clone());
                                            }
                                            if !rule_config.exclude_patterns.is_empty() {
                                                rule = rule.with_exclude_patterns(rule_config.exclude_patterns.clone());
                                            }
                                            if let Some(lang) = &rule_config.language {
                                                rule = rule.with_language(lang.clone());
                                            }
                                            if let Some(suggestion) = &rule_config.suggestion {
                                                rule = rule.with_suggestion(suggestion.clone());
                                            }
                                            engine.add_rule(Box::new(rule));
                                        }
                                        Err(e) => {
                                            eprintln!("{} Failed to compile pattern rule '{}': {}",
                                                "⚠️".yellow(), rule_config.name, e);
                                        }
                                    }
                                }
                                None => {
                                    eprintln!("{} Invalid severity for rule '{}': {}",
                                        "⚠️".yellow(), rule_config.name, rule_config.severity);
                                }
                            }
                        }
                    }

                    // Handle AST rules
                    else if rule_config.rule_type == "ast" {
                        match lucent::rules::Severity::from_str(&rule_config.severity) {
                            Some(severity) => {
                                // Determine the AST matcher type
                                let matcher = if let Some(check) = &rule_config.check {
                                    match check.as_str() {
                                        "pub_fn_without_docs" | "public_function_without_docs" => {
                                            lucent::AstMatcher::PublicFunctionWithoutDocs
                                        }
                                        "fn_without_docs" | "function_without_docs" => {
                                            lucent::AstMatcher::FunctionWithoutDocs
                                        }
                                        "unhandled_result" | "missing_error_handling" => {
                                            lucent::AstMatcher::UnhandledResult
                                        }
                                        _ => {
                                            eprintln!("{} Unknown AST check type '{}' for rule '{}'",
                                                "⚠️".yellow(), check, rule_config.name);
                                            continue;
                                        }
                                    }
                                } else if let Some(max_params) = rule_config.max_params {
                                    lucent::AstMatcher::TooManyParameters { max: max_params }
                                } else if let Some(max_arms) = rule_config.max_match_arms {
                                    lucent::AstMatcher::TooManyMatchArms { max: max_arms }
                                } else if let Some(max_depth) = rule_config.max_nesting_depth {
                                    lucent::AstMatcher::DeepNesting { max_depth }
                                } else {
                                    eprintln!("{} AST rule '{}' must specify check, max_params, max_match_arms, or max_nesting_depth",
                                        "⚠️".yellow(), rule_config.name);
                                    continue;
                                };

                                let mut rule = lucent::AstRule::new(
                                    rule_config.name.clone(),
                                    rule_config.description.clone(),
                                    severity,
                                    matcher,
                                );

                                if !rule_config.file_patterns.is_empty() {
                                    rule = rule.with_file_patterns(rule_config.file_patterns.clone());
                                }
                                if !rule_config.exclude_patterns.is_empty() {
                                    rule = rule.with_exclude_patterns(rule_config.exclude_patterns.clone());
                                }
                                if let Some(lang) = &rule_config.language {
                                    rule = rule.with_language(lang.clone());
                                }
                                if let Some(suggestion) = &rule_config.suggestion {
                                    rule = rule.with_suggestion(suggestion.clone());
                                }

                                engine.add_rule(Box::new(rule));
                            }
                            None => {
                                eprintln!("{} Invalid severity for rule '{}': {}",
                                    "⚠️".yellow(), rule_config.name, rule_config.severity);
                            }
                        }
                    }

                    // Handle metric rules
                    else if rule_config.rule_type == "metric" {
                        match lucent::rules::Severity::from_str(&rule_config.severity) {
                            Some(severity) => {
                                if let Some(metric) = &rule_config.metric {
                                    let metric_type = match metric.as_str() {
                                        "complexity" | "cyclomatic_complexity" => {
                                            if let Some(max) = rule_config.max_value {
                                                lucent::MetricType::CyclomaticComplexity { max }
                                            } else {
                                                eprintln!("{} Metric rule '{}' for complexity must have max_value",
                                                    "⚠️".yellow(), rule_config.name);
                                                continue;
                                            }
                                        }
                                        "cognitive_complexity" => {
                                            if let Some(max) = rule_config.max_value {
                                                lucent::MetricType::CognitiveComplexity { max }
                                            } else {
                                                eprintln!("{} Metric rule '{}' for cognitive_complexity must have max_value",
                                                    "⚠️".yellow(), rule_config.name);
                                                continue;
                                            }
                                        }
                                        "nesting_depth" | "max_nesting_depth" => {
                                            if let Some(max) = rule_config.max_value {
                                                lucent::MetricType::NestingDepth { max: max as usize }
                                            } else {
                                                eprintln!("{} Metric rule '{}' for nesting_depth must have max_value",
                                                    "⚠️".yellow(), rule_config.name);
                                                continue;
                                            }
                                        }
                                        "function_length" | "avg_function_length" => {
                                            if let Some(max) = rule_config.max_value {
                                                lucent::MetricType::FunctionLength { max: max as usize }
                                            } else {
                                                eprintln!("{} Metric rule '{}' for function_length must have max_value",
                                                    "⚠️".yellow(), rule_config.name);
                                                continue;
                                            }
                                        }
                                        "file_size" | "total_lines" => {
                                            if let Some(max) = rule_config.max_value {
                                                lucent::MetricType::FileSize { max: max as usize }
                                            } else {
                                                eprintln!("{} Metric rule '{}' for file_size must have max_value",
                                                    "⚠️".yellow(), rule_config.name);
                                                continue;
                                            }
                                        }
                                        "comment_ratio" => {
                                            let min = rule_config.min_value.unwrap_or(0.0);
                                            let max = rule_config.max_value.unwrap_or(100.0);
                                            lucent::MetricType::CommentRatio { min, max }
                                        }
                                        "code_density" => {
                                            let min = rule_config.min_value.unwrap_or(0.0);
                                            let max = rule_config.max_value.unwrap_or(100.0);
                                            lucent::MetricType::CodeDensity { min, max }
                                        }
                                        "maintainability_index" | "maintainability" => {
                                            if let Some(min) = rule_config.min_value {
                                                lucent::MetricType::MaintainabilityIndex { min }
                                            } else {
                                                eprintln!("{} Metric rule '{}' for maintainability_index must have min_value",
                                                    "⚠️".yellow(), rule_config.name);
                                                continue;
                                            }
                                        }
                                        "halstead_volume" => {
                                            if let Some(max) = rule_config.max_value {
                                                lucent::MetricType::HalsteadVolume { max }
                                            } else {
                                                eprintln!("{} Metric rule '{}' for halstead_volume must have max_value",
                                                    "⚠️".yellow(), rule_config.name);
                                                continue;
                                            }
                                        }
                                        "halstead_difficulty" => {
                                            if let Some(max) = rule_config.max_value {
                                                lucent::MetricType::HalsteadDifficulty { max }
                                            } else {
                                                eprintln!("{} Metric rule '{}' for halstead_difficulty must have max_value",
                                                    "⚠️".yellow(), rule_config.name);
                                                continue;
                                            }
                                        }
                                        "function_count" => {
                                            if let Some(max) = rule_config.max_value {
                                                lucent::MetricType::FunctionCount { max: max as usize }
                                            } else {
                                                eprintln!("{} Metric rule '{}' for function_count must have max_value",
                                                    "⚠️".yellow(), rule_config.name);
                                                continue;
                                            }
                                        }
                                        "todo_markers" | "todos" => {
                                            if let Some(max) = rule_config.max_value {
                                                lucent::MetricType::TodoMarkers { max: max as usize }
                                            } else {
                                                eprintln!("{} Metric rule '{}' for todo_markers must have max_value",
                                                    "⚠️".yellow(), rule_config.name);
                                                continue;
                                            }
                                        }
                                        _ => {
                                            eprintln!("{} Unknown metric type '{}' for rule '{}'",
                                                "⚠️".yellow(), metric, rule_config.name);
                                            continue;
                                        }
                                    };

                                    let mut rule = lucent::MetricRule::new(
                                        rule_config.name.clone(),
                                        rule_config.description.clone(),
                                        severity,
                                        metric_type,
                                    );

                                    if !rule_config.file_patterns.is_empty() {
                                        rule = rule.with_file_patterns(rule_config.file_patterns.clone());
                                    }
                                    if !rule_config.exclude_patterns.is_empty() {
                                        rule = rule.with_exclude_patterns(rule_config.exclude_patterns.clone());
                                    }
                                    if let Some(lang) = &rule_config.language {
                                        rule = rule.with_language(lang.clone());
                                    }
                                    if let Some(suggestion) = &rule_config.suggestion {
                                        rule = rule.with_suggestion(suggestion.clone());
                                    }

                                    engine.add_rule(Box::new(rule));
                                } else {
                                    eprintln!("{} Metric rule '{}' must specify a metric type",
                                        "⚠️".yellow(), rule_config.name);
                                }
                            }
                            None => {
                                eprintln!("{} Invalid severity for rule '{}': {}",
                                    "⚠️".yellow(), rule_config.name, rule_config.severity);
                            }
                        }
                    }
                }

                if config.verbose && engine.rule_count() > 0 {
                    println!("{} Loaded {} custom rules", "📋".bright_cyan(), engine.rule_count());
                }

                Some(engine)
                } else {
                    None
                }
            }
            Err(e) => {
                if config.verbose {
                    eprintln!("{} Failed to load config file: {}", "⚠️".yellow(), e);
                }
                None
            }
        }
    } else {
        None
    };

    // Get ignore patterns from config file if available
    let scanner = if let Some(config_path) = &config.config_file {
        // Load config file to get ignore patterns
        if let Ok(file_config) = lucent::config::LucentConfig::from_file(config_path) {
            let mut ignore_patterns = vec![];

            // Add .lucentignore if enabled
            if file_config.ignore.use_lucentignore {
                ignore_patterns.push(".lucentignore".to_string());
            }

            // Add custom patterns from config
            ignore_patterns.extend(file_config.ignore.patterns);

            if config.verbose && !ignore_patterns.is_empty() {
                println!("{} Using {} ignore patterns", "ℹ️".blue(), ignore_patterns.len());
            }

            Scanner::with_ignores(&config.path, ignore_patterns)
                .context("Failed to create scanner")?
        } else {
            Scanner::new(&config.path)
                .context("Failed to create scanner")?
        }
    } else {
        Scanner::new(&config.path)
            .context("Failed to create scanner")?
    };

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

    // Initialize cache
    let mut cache = AnalysisCache::new().unwrap_or_default();
    let mut cache_hits = 0;
    let mut cache_misses = 0;

    // Collect rule violations
    let mut all_violations: Vec<lucent::Violation> = Vec::new();

    // Analyze metrics in parallel with caching
    let results: Vec<_> = files.par_iter()
        .filter_map(|source_file| {
            if config.verbose && !config.per_file {
                println!("{} {}", "  Analyzing:".bright_black(), source_file.path.display());
            }

            // Try to get from cache first (read-only check)
            let metrics = if let Some(cached_metrics) = cache.get(&source_file.path) {
                if config.verbose {
                    println!("    {} Cache hit", "💨".bright_green());
                }
                cached_metrics.clone()
            } else {
                match FileMetrics::analyze(&source_file.path, source_file.language) {
                    Ok(metrics) => metrics,
                    Err(e) => {
                        if config.verbose {
                            eprintln!("    {} Failed to analyze {}: {}", "✗".red(), source_file.path.display(), e);
                        }
                        return None;
                    }
                }
            };

            // Show per-file metrics if requested
            if config.per_file && !matches!(config.format, lucent::config::OutputFormat::Json) {
                println!("    {} {} Lines: {}, Code: {}, Entropy: {:.2}",
                    source_file.language.emoji(),
                    source_file.path.display().to_string().bright_black(),
                    metrics.total_lines,
                    metrics.code_lines,
                    metrics.entropy
                );
            }

            Some((source_file.path.clone(), source_file.language.name, metrics))
        })
        .collect();

    // Update cache and aggregate results
    let mut project_metrics = ProjectMetrics::new();
    for (path, language_name, metrics) in results {
        // Update cache
        if cache.get(&path).is_none() {
            let _ = cache.insert(path.clone(), metrics.clone());
            cache_misses += 1;
        } else {
            cache_hits += 1;
        }

        project_metrics.add_file(language_name, &metrics);

        // Check custom rules if engine is available
        if let Some(ref engine) = rule_engine {
            // Find the source file to get language info
            if let Some(source_file) = files.iter().find(|f| f.path == path) {
                // Read file content for rule checking
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let violations = engine.check_file(&path, &content, &source_file.language);
                    all_violations.extend(violations);
                }
            }
        }
    }

    // Save cache
    if let Err(e) = cache.save() {
        if config.verbose {
            eprintln!("Warning: Failed to save cache: {}", e);
        }
    }

    if config.verbose {
        println!();
        println!("{} Cache: {} hits, {} misses ({:.1}% hit rate)",
            "📊".bright_cyan(),
            cache_hits,
            cache_misses,
            if cache_hits + cache_misses > 0 {
                (cache_hits as f64 / (cache_hits + cache_misses) as f64) * 100.0
            } else {
                0.0
            }
        );
    }

    Ok((project_metrics, all_violations, rules_config_opt))
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

/// Display custom rule violations and return exit code
fn display_custom_rule_violations(
    violations: &[lucent::Violation],
    rules_config: Option<&lucent::config::RulesGlobalConfig>
) -> i32 {
    use colored::Colorize;

    println!("{} Custom Rule Violations", "⚠️".yellow().bold());
    println!();

    // Group violations by severity
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut infos = Vec::new();

    for violation in violations {
        match violation.severity {
            lucent::rules::Severity::Error => errors.push(violation),
            lucent::rules::Severity::Warning => warnings.push(violation),
            lucent::rules::Severity::Info => infos.push(violation),
        }
    }

    // Display errors first
    for violation in &errors {
        display_single_violation(violation, "✗", |s: String| s.red());
    }

    // Then warnings
    for violation in &warnings {
        display_single_violation(violation, "⚠", |s: String| s.yellow());
    }

    // Finally info
    for violation in &infos {
        display_single_violation(violation, "ℹ", |s: String| s.bright_blue());
    }

    // Summary
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("{} Custom Rules Summary", "📊".bright_cyan());
    println!("  Total Violations: {}", violations.len().to_string().cyan());
    if !errors.is_empty() {
        println!("  {} Errors: {}", "✗".red(), errors.len().to_string().red());
    }
    if !warnings.is_empty() {
        println!("  {} Warnings: {}", "⚠".yellow(), warnings.len().to_string().yellow());
    }
    if !infos.is_empty() {
        println!("  {} Info: {}", "ℹ".bright_blue(), infos.len().to_string().bright_blue());
    }

    // Determine exit code based on rules config
    let default_config = lucent::config::RulesGlobalConfig::default();
    let config = rules_config.unwrap_or(&default_config);

    if config.fail_on_error && !errors.is_empty() {
        return 1;
    }

    if config.fail_on_warning && !warnings.is_empty() {
        return 1;
    }

    0
}

/// Display a single violation
fn display_single_violation(
    violation: &lucent::Violation,
    emoji: &str,
    color_fn: fn(String) -> colored::ColoredString
) {
    use colored::Colorize;

    println!(
        "  {}:{}:{}",
        violation.file.bright_black(),
        violation.line.to_string().cyan(),
        violation.column.to_string().cyan()
    );

    println!(
        "    {} {} [{}]",
        emoji,
        color_fn(violation.rule.clone()),
        violation.severity.to_string().bold()
    );

    println!(
        "      {}",
        violation.message.bright_white()
    );

    if let Some(ref snippet) = violation.code_snippet {
        println!();
        println!(
            "      {} | {}",
            violation.line.to_string().bright_black(),
            snippet.bright_black()
        );
    }

    if let Some(ref suggestion) = violation.suggestion {
        println!();
        println!(
            "      Suggestion: {}",
            suggestion.bright_green()
        );
    }

    println!();
}
