//! Output formatters for displaying metrics
//!
//! This module provides beautiful terminal output with colors and emojis,
//! as well as JSON export functionality.

use colored::Colorize;
use serde::Serialize;
use tabled::{Table, Tabled, settings::Style};

use crate::metrics::ProjectMetrics;

/// Output format options
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    /// Pretty terminal output with colors and emojis
    Pretty,
    /// JSON output
    Json,
    /// Markdown report
    Markdown,
    /// CSV export
    Csv,
    /// Interactive HTML report
    Html,
}

/// Display project metrics in the specified format
pub fn display_metrics(metrics: &ProjectMetrics, format: OutputFormat) {
    display_metrics_with_violations(metrics, &[], format);
}

/// Display project metrics with violations in the specified format
pub fn display_metrics_with_violations(
    metrics: &ProjectMetrics,
    violations: &[crate::Violation],
    format: OutputFormat
) {
    match format {
        OutputFormat::Pretty => display_pretty(metrics),
        OutputFormat::Json => display_json_with_violations(metrics, violations),
        OutputFormat::Markdown => display_markdown(metrics),
        OutputFormat::Csv => display_csv(metrics),
        OutputFormat::Html => display_html(metrics),
    }
}

/// Create a horizontal bar chart for a metric
fn create_bar_chart(value: f64, max_value: f64, width: usize, symbol: char) -> String {
    let filled = if max_value > 0.0 {
        ((value / max_value) * width as f64).round() as usize
    } else {
        0
    };
    let empty = width.saturating_sub(filled);

    format!("{}{}",
        symbol.to_string().repeat(filled),
        "·".repeat(empty)
    )
}

/// Get color for complexity level
fn get_complexity_color(value: f64) -> colored::Color {
    use colored::Color;
    if value > 20.0 {
        Color::Red
    } else if value > 10.0 {
        Color::Yellow
    } else if value > 5.0 {
        Color::Cyan
    } else {
        Color::Green
    }
}

/// Create a visual bar chart with color based on value
fn create_colored_bar(value: f64, max_value: f64, width: usize) -> colored::ColoredString {
    let bar = create_bar_chart(value, max_value, width, '█');
    let color = get_complexity_color(value);
    bar.color(color)
}

/// Display metrics with beautiful terminal formatting
fn display_pretty(metrics: &ProjectMetrics) {
    // Header
    println!();
    println!("{}", "━".repeat(60).bright_cyan());
    println!("  💎 {} {}", "Lucent".bold().bright_white(), "Analysis Results".bright_white());
    println!("{}", "━".repeat(60).bright_cyan());
    println!();

    // Summary
    println!("{}", "📊 Summary".bold().bright_yellow());
    println!("  {} {}", "Total Files:".bright_white(), metrics.total_files.to_string().bright_cyan().bold());
    println!("  {} {}", "Total Lines:".bright_white(), metrics.total_lines().to_string().bright_cyan().bold());
    println!("  {} {}", "Code Lines:".bright_white(), metrics.total_code_lines().to_string().bright_green().bold());
    println!("  {} {}", "Comment Lines:".bright_white(), metrics.total_comment_lines().to_string().bright_blue().bold());
    println!("  {} {:.2}", "Avg Entropy:".bright_white(), metrics.average_entropy().to_string().bright_magenta().bold());
    print!("  {} {:.1} ", "Avg Complexity:".bright_white(), metrics.average_cyclomatic());
    let ratio_val = (metrics.average_cognitive() / metrics.average_cyclomatic().max(1.0)).round() as usize;
    println!("{} {:.1} {}", "Cognitive:".bright_white(), metrics.average_cognitive(), format!("({}x)", ratio_val));
    println!("  {} {:.1}", "Avg Nesting:".bright_white(), metrics.average_nesting_depth().to_string().bright_cyan().bold());

    let total_funcs = metrics.total_functions();
    if total_funcs > 0 {
        println!("  {} {}", "Total Functions:".bright_white(), total_funcs.to_string().bright_green().bold());
        println!("  {} {:.1}", "Avg Func Length:".bright_white(), metrics.average_function_length().to_string().bright_blue().bold());
    }

    // Maintainability Index
    let mi = metrics.average_maintainability_index();
    let mi_grade = crate::metrics::FileMetrics::get_maintainability_grade(mi);
    let mi_color = match mi_grade {
        "A" => colored::Color::BrightGreen,
        "B" => colored::Color::Green,
        "C" => colored::Color::Yellow,
        "D" => colored::Color::Red,
        _ => colored::Color::BrightRed,
    };
    println!("  {} {:.1} {}",
        "Maintainability:".bright_white(),
        mi.to_string().color(mi_color).bold(),
        format!("(Grade: {})", mi_grade).color(mi_color)
    );

    // Code markers
    let total_markers = metrics.total_todos() + metrics.total_fixmes() + metrics.total_hacks();
    if total_markers > 0 {
        println!();
        println!("{}", "🔖 Code Markers".bold().bright_yellow());
        if metrics.total_todos() > 0 {
            println!("  {} {}", "TODOs:".bright_white(), metrics.total_todos().to_string().yellow().bold());
        }
        if metrics.total_fixmes() > 0 {
            println!("  {} {}", "FIXMEs:".bright_white(), metrics.total_fixmes().to_string().red().bold());
        }
        if metrics.total_hacks() > 0 {
            println!("  {} {}", "HACKs:".bright_white(), metrics.total_hacks().to_string().magenta().bold());
        }
    }
    println!();

    // Per-language breakdown
    if !metrics.by_language.is_empty() {
        println!("{}", "🌍 By Language".bold().bright_yellow());
        println!();

        let mut lang_rows: Vec<LanguageRow> = metrics
            .by_language
            .iter()
            .map(|(name, metrics)| {
                let emoji = get_language_emoji(name);
                LanguageRow {
                    language: format!("{} {}", emoji, name),
                    files: metrics.file_count,
                    lines: metrics.total_lines,
                    code: metrics.code_lines,
                    comments: metrics.comment_lines,
                    blanks: metrics.blank_lines,
                    complexity: format!("{:.1}", metrics.avg_cyclomatic),
                    nesting: format!("{:.1}", metrics.avg_nesting_depth),
                    entropy: format!("{:.2}", metrics.avg_entropy),
                }
            })
            .collect();

        // Sort by line count descending
        lang_rows.sort_by(|a, b| b.lines.cmp(&a.lines));

        let table = Table::new(&lang_rows)
            .with(Style::rounded())
            .to_string();

        println!("{}", table);
        println!();

        // Complexity visualization
        println!("{}", "📊 Complexity Visualization".bold().bright_yellow());
        println!();

        // Sort languages by complexity for visualization
        let mut lang_complexity: Vec<_> = metrics.by_language.iter().collect();
        lang_complexity.sort_by(|a, b| {
            b.1.avg_cyclomatic.partial_cmp(&a.1.avg_cyclomatic).unwrap()
        });

        // Find max complexity for scaling
        let max_complexity = lang_complexity
            .iter()
            .map(|(_, m)| m.avg_cyclomatic)
            .fold(0.0f64, f64::max)
            .max(10.0); // At least 10 for better scaling

        println!("  {}", "Cyclomatic Complexity by Language".bright_white().bold());
        for (name, lang_metrics) in lang_complexity.iter() {
            let emoji = get_language_emoji(name);
            let bar = create_colored_bar(lang_metrics.avg_cyclomatic, max_complexity, 30);
            println!(
                "  {} {:12} {:5.1} {}",
                emoji,
                name,
                lang_metrics.avg_cyclomatic,
                bar
            );
        }
        println!();

        // Cognitive complexity visualization
        let mut lang_cognitive: Vec<_> = metrics.by_language.iter().collect();
        lang_cognitive.sort_by(|a, b| {
            b.1.avg_cognitive.partial_cmp(&a.1.avg_cognitive).unwrap()
        });

        let max_cognitive = lang_cognitive
            .iter()
            .map(|(_, m)| m.avg_cognitive)
            .fold(0.0f64, f64::max)
            .max(15.0); // At least 15 for better scaling

        println!("  {}", "Cognitive Complexity by Language".bright_white().bold());
        for (name, lang_metrics) in lang_cognitive.iter() {
            let emoji = get_language_emoji(name);
            let bar = create_colored_bar(lang_metrics.avg_cognitive, max_cognitive, 30);
            let ratio = if lang_metrics.avg_cyclomatic > 0.0 {
                (lang_metrics.avg_cognitive / lang_metrics.avg_cyclomatic).round() as usize
            } else {
                0
            };
            println!("  {} {:12} {:5.1} {} {}",
                emoji,
                name,
                lang_metrics.avg_cognitive,
                bar,
                format!("({}x cyclo)", ratio)
            );
        }
        println!();

        // Nesting depth visualization
        let mut lang_nesting: Vec<_> = metrics.by_language.iter().collect();
        lang_nesting.sort_by(|a, b| {
            b.1.avg_nesting_depth.partial_cmp(&a.1.avg_nesting_depth).unwrap()
        });

        let max_nesting = lang_nesting
            .iter()
            .map(|(_, m)| m.avg_nesting_depth)
            .fold(0.0f64, f64::max)
            .max(5.0); // At least 5 for better scaling

        println!("  {}", "Nesting Depth by Language".bright_white().bold());
        for (name, lang_metrics) in lang_nesting.iter() {
            let emoji = get_language_emoji(name);
            let bar = create_colored_bar(lang_metrics.avg_nesting_depth, max_nesting, 30);
            println!(
                "  {} {:12} {:5.1} {}",
                emoji,
                name,
                lang_metrics.avg_nesting_depth,
                bar
            );
        }
        println!();
    }

    // Code Smells
    let total_smells = metrics.total_code_smells();
    if total_smells > 0 {
        println!("{}", "👃 Code Smells".bold().bright_yellow());

        if metrics.total_long_functions() > 0 {
            println!("  {} {}", "Long Functions:".bright_white(),
                metrics.total_long_functions().to_string().yellow().bold());
        }
        if metrics.total_long_parameter_lists() > 0 {
            println!("  {} {}", "Long Parameter Lists:".bright_white(),
                metrics.total_long_parameter_lists().to_string().yellow().bold());
        }
        if metrics.total_deep_nesting() > 0 {
            println!("  {} {}", "Deep Nesting Instances:".bright_white(),
                metrics.total_deep_nesting().to_string().red().bold());
        }
        if metrics.total_magic_numbers() > 0 {
            println!("  {} {}", "Magic Numbers:".bright_white(),
                metrics.total_magic_numbers().to_string().magenta().bold());
        }

        println!("  {} {}", "Total Code Smells:".bright_white(),
            total_smells.to_string().red().bold());
        println!();
    }

    // Insights
    println!("{}", "💡 Insights".bold().bright_yellow());
    display_insights(metrics);
    println!();

    println!("{}", "━".repeat(60).bright_cyan());
    println!();
}

/// Display actionable insights based on metrics
fn display_insights(metrics: &ProjectMetrics) {
    let total_lines = metrics.total_lines();
    let code_lines = metrics.total_code_lines();
    let comment_lines = metrics.total_comment_lines();

    if total_lines == 0 {
        println!("  {} No code found to analyze", "⚠️".yellow());
        return;
    }

    let comment_ratio = comment_lines as f64 / total_lines as f64;
    let avg_entropy = metrics.average_entropy();
    let avg_complexity = metrics.average_cyclomatic();
    let avg_nesting = metrics.average_nesting_depth();
    let avg_func_length = metrics.average_function_length();

    // Cyclomatic complexity insight
    if avg_complexity > 20.0 {
        println!("  {} Very high complexity: {:.1} (urgent refactoring needed)", "🔴".red(), avg_complexity);
    } else if avg_complexity > 10.0 {
        println!("  {} High complexity: {:.1} (consider refactoring)", "⚠️".yellow(), avg_complexity);
    } else if avg_complexity < 5.0 {
        println!("  {} Low complexity: {:.1} (simple codebase)", "✓".green(), avg_complexity);
    }

    // Nesting depth insight
    if avg_nesting > 4.0 {
        println!("  {} Deep nesting: {:.1} levels (reduce nesting)", "⚠️".yellow(), avg_nesting);
    } else if avg_nesting < 2.0 {
        println!("  {} Shallow nesting: {:.1} levels (flat structure)", "✓".green(), avg_nesting);
    }

    // Function length insight
    if avg_func_length > 50.0 {
        println!("  {} Long functions: {:.1} lines avg (consider breaking down)", "⚠️".yellow(), avg_func_length);
    } else if avg_func_length > 0.0 && avg_func_length < 20.0 {
        println!("  {} Short functions: {:.1} lines avg (good modularity)", "✓".green(), avg_func_length);
    }

    // Comment ratio insight
    if comment_ratio < 0.1 {
        println!("  {} Low documentation: Only {:.1}% comments", "⚠️".yellow(), comment_ratio * 100.0);
    } else if comment_ratio > 0.3 {
        println!("  {} Well documented: {:.1}% comments", "✓".green(), comment_ratio * 100.0);
    }

    // Comment quality insight
    let comment_quality = metrics.average_comment_quality();
    let low_quality = metrics.total_low_quality_comments();
    let high_quality = metrics.total_high_quality_comments();

    if comment_lines > 0 {
        if comment_quality < 0.3 {
            println!("  {} Poor comment quality: {:.0}% (mostly noise/obvious)", "🔴".red(), comment_quality * 100.0);
            if low_quality > 0 {
                println!("    {} {} low-quality comments detected", "↳".dimmed(), low_quality);
            }
        } else if comment_quality < 0.5 {
            println!("  {} Mediocre comment quality: {:.0}% (add more context)", "⚠️".yellow(), comment_quality * 100.0);
        } else if comment_quality >= 0.7 {
            println!("  {} Excellent comment quality: {:.0}% (explanatory & helpful)", "✓".green(), comment_quality * 100.0);
            if high_quality > 0 {
                println!("    {} {} high-quality comments", "↳".dimmed(), high_quality);
            }
        }
    }

    // Code smells insight
    let total_smells = metrics.total_code_smells();
    if total_smells > 0 {
        let smell_density = total_smells as f64 / metrics.total_files as f64;

        if total_smells > 20 || smell_density > 3.0 {
            println!("  {} {} code smells detected ({:.1} per file)", "🔴".red(), total_smells, smell_density);
        } else if total_smells > 5 {
            println!("  {} {} code smells detected ({:.1} per file)", "⚠️".yellow(), total_smells, smell_density);
        }

        if metrics.total_long_functions() > 0 {
            println!("    {} {} long functions (>50 lines)", "↳".dimmed(), metrics.total_long_functions());
        }
        if metrics.total_deep_nesting() > 0 {
            println!("    {} {} deep nesting instances (>4 levels)", "↳".dimmed(), metrics.total_deep_nesting());
        }
        if metrics.total_magic_numbers() > 0 {
            println!("    {} {} magic numbers (use named constants)", "↳".dimmed(), metrics.total_magic_numbers());
        }
    } else if total_lines > 100 {
        println!("  {} No code smells detected!", "✓".green());
    }

    // Entropy insight
    if avg_entropy > 4.5 {
        println!("  {} High entropy: {:.2} (information dense)", "📊".normal(), avg_entropy);
    } else if avg_entropy < 3.0 {
        println!("  {} Low entropy: {:.2} (repetitive patterns)", "✓".green(), avg_entropy);
    }

    // Code distribution
    let code_ratio = code_lines as f64 / total_lines as f64;
    println!("  {} Code density: {:.1}%", "📈".normal(), code_ratio * 100.0);
}

/// Display metrics as JSON
fn display_json(metrics: &ProjectMetrics) {
    display_json_with_violations(metrics, &[]);
}

fn display_json_with_violations(metrics: &ProjectMetrics, violations: &[crate::Violation]) {
    let json_output = JsonOutput::with_violations(metrics, violations);
    match serde_json::to_string_pretty(&json_output) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing JSON: {}", e),
    }
}

/// Display metrics as Markdown report
fn display_markdown(metrics: &ProjectMetrics) {
    println!("# 💎 Lucent Code Analysis Report\n");

    // Summary section
    println!("## 📊 Summary\n");
    println!("| Metric | Value |");
    println!("|--------|-------|");
    println!("| **Total Files** | {} |", metrics.total_files);
    println!("| **Total Lines** | {} |", metrics.total_lines());
    println!("| **Code Lines** | {} |", metrics.total_code_lines());
    println!("| **Comment Lines** | {} |", metrics.total_comment_lines());
    println!("| **Blank Lines** | {} |", metrics.by_language.values().map(|m| m.blank_lines).sum::<usize>());
    println!("| **Average Entropy** | {:.2} |", metrics.average_entropy());
    println!("| **Average Complexity** | {:.1} |", metrics.average_cyclomatic());
    println!("| **Average Nesting Depth** | {:.1} |", metrics.average_nesting_depth());

    let mi = metrics.average_maintainability_index();
    let mi_grade = crate::metrics::FileMetrics::get_maintainability_grade(mi);
    println!("| **Maintainability Index** | {:.1} (Grade: {}) |", mi, mi_grade);

    let total_funcs = metrics.total_functions();
    if total_funcs > 0 {
        println!("| **Total Functions** | {} |", total_funcs);
        println!("| **Average Function Length** | {:.1} lines |", metrics.average_function_length());
    }

    let comment_quality = metrics.average_comment_quality();
    if metrics.total_comment_lines() > 0 {
        println!("| **Comment Quality Score** | {:.0}% |", comment_quality * 100.0);
    }

    println!();

    // Code markers section
    let todos = metrics.total_todos();
    let fixmes = metrics.total_fixmes();
    let hacks = metrics.total_hacks();

    if todos > 0 || fixmes > 0 || hacks > 0 {
        println!("## 🔖 Code Markers\n");
        println!("| Marker | Count |");
        println!("|--------|-------|");
        if todos > 0 {
            println!("| ⚠️ TODOs | {} |", todos);
        }
        if fixmes > 0 {
            println!("| 🔴 FIXMEs | {} |", fixmes);
        }
        if hacks > 0 {
            println!("| 🟣 HACKs | {} |", hacks);
        }
        println!();
    }

    // Language breakdown
    println!("## 🌍 Languages\n");
    println!("| Language | Files | Lines | Code | Comments | Blanks | Complexity | Nesting | Entropy | MI |");
    println!("|----------|-------|-------|------|----------|--------|------------|---------|---------|-----|");

    let mut languages: Vec<_> = metrics.by_language.iter().collect();
    languages.sort_by(|a, b| b.1.total_lines.cmp(&a.1.total_lines));

    for (name, lang_metrics) in languages {
        let emoji = get_language_emoji(name);
        let mi_grade = crate::metrics::FileMetrics::get_maintainability_grade(lang_metrics.avg_maintainability_index);
        println!(
            "| {} {} | {} | {} | {} | {} | {} | {:.1} | {:.1} | {:.2} | {:.1} ({}) |",
            emoji,
            name,
            lang_metrics.file_count,
            lang_metrics.total_lines,
            lang_metrics.code_lines,
            lang_metrics.comment_lines,
            lang_metrics.blank_lines,
            lang_metrics.avg_cyclomatic,
            lang_metrics.avg_nesting_depth,
            lang_metrics.avg_entropy,
            lang_metrics.avg_maintainability_index,
            mi_grade
        );
    }

    println!();

    // Code Smells section
    let total_smells = metrics.total_code_smells();
    if total_smells > 0 {
        println!("## 👃 Code Smells\n");
        println!("| Type | Count |");
        println!("|------|-------|");
        if metrics.total_long_functions() > 0 {
            println!("| ⚠️ Long Functions (>50 lines) | {} |", metrics.total_long_functions());
        }
        if metrics.total_long_parameter_lists() > 0 {
            println!("| ⚠️ Long Parameter Lists (>5 params) | {} |", metrics.total_long_parameter_lists());
        }
        if metrics.total_deep_nesting() > 0 {
            println!("| 🔴 Deep Nesting (>4 levels) | {} |", metrics.total_deep_nesting());
        }
        if metrics.total_magic_numbers() > 0 {
            println!("| 🟣 Magic Numbers | {} |", metrics.total_magic_numbers());
        }
        println!("| **Total** | **{}** |", total_smells);
        println!();
    }

    // Insights section
    println!("## 💡 Insights\n");

    let total_lines = metrics.total_lines();
    let code_lines = metrics.total_code_lines();
    let comment_lines = metrics.total_comment_lines();

    if total_lines == 0 {
        println!("⚠️ No code found to analyze\n");
        return;
    }

    let comment_ratio = comment_lines as f64 / total_lines as f64;
    let avg_entropy = metrics.average_entropy();
    let avg_complexity = metrics.average_cyclomatic();
    let avg_nesting = metrics.average_nesting_depth();
    let avg_func_length = metrics.average_function_length();

    // Complexity insights
    if avg_complexity > 20.0 {
        println!("- 🔴 **Very high complexity: {:.1}** - Urgent refactoring needed", avg_complexity);
    } else if avg_complexity > 10.0 {
        println!("- ⚠️ **High complexity: {:.1}** - Consider refactoring", avg_complexity);
    } else if avg_complexity < 5.0 {
        println!("- ✅ **Low complexity: {:.1}** - Simple codebase", avg_complexity);
    }

    // Nesting depth insights
    if avg_nesting > 4.0 {
        println!("- ⚠️ **Deep nesting: {:.1} levels** - Reduce nesting depth", avg_nesting);
    } else if avg_nesting < 2.0 {
        println!("- ✅ **Shallow nesting: {:.1} levels** - Flat code structure", avg_nesting);
    }

    // Function length insights
    if avg_func_length > 50.0 {
        println!("- ⚠️ **Long functions: {:.1} lines avg** - Consider breaking down", avg_func_length);
    } else if avg_func_length > 0.0 && avg_func_length < 20.0 {
        println!("- ✅ **Short functions: {:.1} lines avg** - Good modularity", avg_func_length);
    }

    // Comment ratio insights
    if comment_ratio < 0.1 {
        println!("- ⚠️ **Low documentation: {:.1}% comments** - Add more documentation", comment_ratio * 100.0);
    } else if comment_ratio > 0.3 {
        println!("- ✅ **Well documented: {:.1}% comments**", comment_ratio * 100.0);
    }

    // Comment quality insights
    if comment_lines > 0 {
        let low_quality = metrics.total_low_quality_comments();
        let high_quality = metrics.total_high_quality_comments();

        if comment_quality < 0.3 {
            println!("- 🔴 **Poor comment quality: {:.0}%** - Mostly noise or obvious comments", comment_quality * 100.0);
            if low_quality > 0 {
                println!("  - {} low-quality comments detected", low_quality);
            }
        } else if comment_quality < 0.5 {
            println!("- ⚠️ **Mediocre comment quality: {:.0}%** - Add more context", comment_quality * 100.0);
        } else if comment_quality >= 0.7 {
            println!("- ✅ **Excellent comment quality: {:.0}%** - Explanatory and helpful", comment_quality * 100.0);
            if high_quality > 0 {
                println!("  - {} high-quality comments", high_quality);
            }
        }
    }

    // Entropy insights
    if avg_entropy > 4.5 {
        println!("- 📊 **High entropy: {:.2}** - Information dense code", avg_entropy);
    } else if avg_entropy < 3.0 {
        println!("- ✅ **Low entropy: {:.2}** - Repetitive patterns detected", avg_entropy);
    }

    // Code density
    let code_ratio = code_lines as f64 / total_lines as f64;
    println!("- 📈 **Code density: {:.1}%**", code_ratio * 100.0);

    println!();
    println!("---");
    println!("*Generated by [Lucent](https://github.com/ibrahimcesar/lucent) v0.1.0-alpha*");
}

/// Display metrics as CSV
fn display_csv(metrics: &ProjectMetrics) {
    // Summary CSV
    println!("# Summary");
    println!("metric,value");
    println!("total_files,{}", metrics.total_files);
    println!("total_lines,{}", metrics.total_lines());
    println!("code_lines,{}", metrics.total_code_lines());
    println!("comment_lines,{}", metrics.total_comment_lines());
    println!("blank_lines,{}", metrics.by_language.values().map(|m| m.blank_lines).sum::<usize>());
    println!("average_entropy,{:.2}", metrics.average_entropy());
    println!("average_complexity,{:.1}", metrics.average_cyclomatic());
    println!("average_nesting_depth,{:.1}", metrics.average_nesting_depth());

    let total_funcs = metrics.total_functions();
    if total_funcs > 0 {
        println!("total_functions,{}", total_funcs);
        println!("average_function_length,{:.1}", metrics.average_function_length());
    }

    let comment_quality = metrics.average_comment_quality();
    if metrics.total_comment_lines() > 0 {
        println!("comment_quality_score,{:.2}", comment_quality);
        println!("low_quality_comments,{}", metrics.total_low_quality_comments());
        println!("high_quality_comments,{}", metrics.total_high_quality_comments());
    }

    let total_lines = metrics.total_lines();
    if total_lines > 0 {
        let code_ratio = metrics.total_code_lines() as f64 / total_lines as f64;
        let comment_ratio = metrics.total_comment_lines() as f64 / total_lines as f64;
        println!("code_density,{:.3}", code_ratio);
        println!("comment_ratio,{:.3}", comment_ratio);
    }

    let mi = metrics.average_maintainability_index();
    let mi_grade = crate::metrics::FileMetrics::get_maintainability_grade(mi);
    println!("maintainability_index,{:.1}", mi);
    println!("maintainability_grade,{}", mi_grade);

    println!();

    // Code markers CSV
    let todos = metrics.total_todos();
    let fixmes = metrics.total_fixmes();
    let hacks = metrics.total_hacks();

    if todos > 0 || fixmes > 0 || hacks > 0 {
        println!("# Code Markers");
        println!("marker,count");
        if todos > 0 {
            println!("TODO,{}", todos);
        }
        if fixmes > 0 {
            println!("FIXME,{}", fixmes);
        }
        if hacks > 0 {
            println!("HACK,{}", hacks);
        }
        println!();
    }

    // Code Smells CSV
    let total_smells = metrics.total_code_smells();
    if total_smells > 0 {
        println!("# Code Smells");
        println!("type,count");
        println!("Long Functions,{}", metrics.total_long_functions());
        println!("Long Parameter Lists,{}", metrics.total_long_parameter_lists());
        println!("Deep Nesting,{}", metrics.total_deep_nesting());
        println!("Magic Numbers,{}", metrics.total_magic_numbers());
        println!("Total,{}", total_smells);
        println!();
    }

    // Language breakdown CSV
    println!("# Languages");
    println!("language,files,lines,code,comments,blanks,complexity,nesting,entropy,functions,avg_function_length,comment_quality,long_functions,long_params,deep_nesting,magic_numbers");

    let mut languages: Vec<_> = metrics.by_language.iter().collect();
    languages.sort_by(|a, b| b.1.total_lines.cmp(&a.1.total_lines));

    for (name, lang_metrics) in languages {
        println!(
            "{},{},{},{},{},{},{:.1},{:.1},{:.2},{},{:.1},{:.2},{},{},{},{}",
            name,
            lang_metrics.file_count,
            lang_metrics.total_lines,
            lang_metrics.code_lines,
            lang_metrics.comment_lines,
            lang_metrics.blank_lines,
            lang_metrics.avg_cyclomatic,
            lang_metrics.avg_nesting_depth,
            lang_metrics.avg_entropy,
            lang_metrics.total_functions,
            lang_metrics.avg_function_length,
            lang_metrics.avg_comment_quality,
            lang_metrics.long_functions,
            lang_metrics.long_parameter_lists,
            lang_metrics.deep_nesting_instances,
            lang_metrics.magic_numbers
        );
    }
}

/// Display metrics as interactive HTML report
fn display_html(metrics: &ProjectMetrics) {
    let html = generate_html_report(metrics);
    println!("{}", html);
}

/// Generate a complete HTML report with embedded CSS and JavaScript
fn generate_html_report(metrics: &ProjectMetrics) -> String {
    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>💎 Lucent Code Analysis Report</title>
    {}
</head>
<body>
    {}
    <script src="https://cdn.jsdelivr.net/npm/chart.js@4.4.0/dist/chart.umd.min.js"></script>
    {}
</body>
</html>"#,
        get_embedded_css(),
        generate_html_body(metrics),
        generate_embedded_js(metrics)
    )
}

/// Get embedded CSS styles for the HTML report
fn get_embedded_css() -> &'static str {
    r#"<style>
        :root {
            --primary: #9333ea;
            --secondary: #60a5fa;
            --success: #10b981;
            --warning: #f59e0b;
            --danger: #ef4444;
            --bg: #ffffff;
            --bg-alt: #f9fafb;
            --text: #111827;
            --text-muted: #6b7280;
            --border: #e5e7eb;
            --shadow: rgba(0, 0, 0, 0.1);
        }

        [data-theme="dark"] {
            --primary: #a855f7;
            --secondary: #3b82f6;
            --success: #34d399;
            --warning: #fbbf24;
            --danger: #f87171;
            --bg: #111827;
            --bg-alt: #1f2937;
            --text: #f9fafb;
            --text-muted: #9ca3af;
            --border: #374151;
            --shadow: rgba(0, 0, 0, 0.3);
        }

        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.6;
            transition: background 0.3s, color 0.3s;
        }

        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem 1rem;
        }

        header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 2rem;
            padding-bottom: 1rem;
            border-bottom: 2px solid var(--primary);
        }

        h1 {
            font-size: 2.5rem;
            color: var(--primary);
            margin: 0;
        }

        h2 {
            font-size: 1.8rem;
            margin: 2rem 0 1rem;
            color: var(--text);
        }

        h3 {
            font-size: 1.3rem;
            margin: 1.5rem 0 0.75rem;
            color: var(--text);
        }

        .toolbar {
            display: flex;
            gap: 0.5rem;
        }

        button {
            padding: 0.5rem 1rem;
            background: var(--primary);
            color: white;
            border: none;
            border-radius: 0.5rem;
            cursor: pointer;
            font-size: 0.9rem;
            transition: opacity 0.2s;
        }

        button:hover {
            opacity: 0.8;
        }

        .timestamp {
            color: var(--text-muted);
            font-size: 0.9rem;
            margin-bottom: 0.5rem;
        }

        .metric-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin: 1.5rem 0;
        }

        .metric-card {
            background: var(--bg-alt);
            padding: 1.5rem;
            border-radius: 0.75rem;
            border: 1px solid var(--border);
            box-shadow: 0 2px 4px var(--shadow);
        }

        .metric-label {
            font-size: 0.9rem;
            color: var(--text-muted);
            margin-bottom: 0.5rem;
        }

        .metric-value {
            font-size: 2rem;
            font-weight: bold;
            color: var(--primary);
        }

        .metric-value.success { color: var(--success); }
        .metric-value.warning { color: var(--warning); }
        .metric-value.danger { color: var(--danger); }

        .progress-bar {
            width: 100%;
            height: 2rem;
            background: var(--border);
            border-radius: 1rem;
            overflow: hidden;
            margin: 1rem 0;
            position: relative;
        }

        .progress-fill {
            height: 100%;
            background: linear-gradient(90deg, var(--success), var(--primary));
            transition: width 0.5s ease;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-weight: bold;
        }

        .grade {
            display: inline-block;
            padding: 0.25rem 0.75rem;
            border-radius: 0.5rem;
            font-weight: bold;
            font-size: 1.2rem;
        }

        .grade-a { background: #10b981; color: white; }
        .grade-b { background: #34d399; color: white; }
        .grade-c { background: #fbbf24; color: #111827; }
        .grade-d { background: #f87171; color: white; }
        .grade-f { background: #dc2626; color: white; }

        table {
            width: 100%;
            border-collapse: collapse;
            margin: 1rem 0;
            background: var(--bg-alt);
            border-radius: 0.5rem;
            overflow: hidden;
        }

        th, td {
            padding: 0.75rem;
            text-align: left;
            border-bottom: 1px solid var(--border);
        }

        th {
            background: var(--primary);
            color: white;
            font-weight: 600;
        }

        tr:hover {
            background: var(--border);
        }

        .chart-container {
            position: relative;
            height: 400px;
            margin: 2rem 0;
            background: var(--bg-alt);
            padding: 1.5rem;
            border-radius: 0.75rem;
            border: 1px solid var(--border);
        }

        .expandable {
            cursor: pointer;
            user-select: none;
        }

        .expandable:hover {
            opacity: 0.8;
        }

        .expandable-content {
            max-height: 0;
            overflow: hidden;
            transition: max-height 0.3s ease;
        }

        .expandable-content.open {
            max-height: 5000px;
        }

        .badge {
            display: inline-block;
            padding: 0.25rem 0.5rem;
            border-radius: 0.25rem;
            font-size: 0.8rem;
            font-weight: 600;
        }

        .badge-success { background: var(--success); color: white; }
        .badge-warning { background: var(--warning); color: white; }
        .badge-danger { background: var(--danger); color: white; }
        .badge-info { background: var(--secondary); color: white; }

        footer {
            margin-top: 3rem;
            padding-top: 1rem;
            border-top: 1px solid var(--border);
            text-align: center;
            color: var(--text-muted);
            font-size: 0.9rem;
        }

        @media print {
            .toolbar {
                display: none;
            }
            .expandable-content {
                max-height: none !important;
            }
            .chart-container {
                page-break-inside: avoid;
            }
        }

        @media (max-width: 768px) {
            .metric-grid {
                grid-template-columns: 1fr;
            }
            h1 {
                font-size: 1.8rem;
            }
            header {
                flex-direction: column;
                gap: 1rem;
            }
        }
    </style>"#
}

/// Generate the HTML body content
fn generate_html_body(metrics: &ProjectMetrics) -> String {
    format!(r#"
    <div class="container">
        <header>
            <div>
                <h1>💎 Lucent Code Analysis Report</h1>
                <div class="timestamp">Generated: {}</div>
            </div>
            <div class="toolbar">
                <button onclick="toggleDarkMode()">🌙 Dark Mode</button>
                <button onclick="window.print()">🖨️ Print/PDF</button>
                <button onclick="exportCSV()">📊 Export CSV</button>
            </div>
        </header>

        {}
        {}
        {}
        {}
        {}

        <footer>
            <p>Generated by <strong>Lucent v0.1.0-alpha</strong> | <a href="https://github.com/ibrahimcesar/lucent" style="color: var(--primary);">GitHub</a></p>
        </footer>
    </div>
    "#,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        generate_summary_section(metrics),
        generate_charts_section(metrics),
        generate_languages_section(metrics),
        generate_code_smells_section(metrics),
        generate_insights_section(metrics)
    )
}

/// Generate summary dashboard section
fn generate_summary_section(metrics: &ProjectMetrics) -> String {
    let mi = metrics.average_maintainability_index();
    let mi_grade = crate::metrics::FileMetrics::get_maintainability_grade(mi);
    let grade_class = match mi_grade {
        "A" => "grade-a",
        "B" => "grade-b",
        "C" => "grade-c",
        "D" => "grade-d",
        _ => "grade-f",
    };

    format!(r#"
        <section>
            <h2>📊 Summary Dashboard</h2>
            <div class="metric-grid">
                <div class="metric-card">
                    <div class="metric-label">Total Files</div>
                    <div class="metric-value">{}</div>
                </div>
                <div class="metric-card">
                    <div class="metric-label">Lines of Code</div>
                    <div class="metric-value">{}</div>
                </div>
                <div class="metric-card">
                    <div class="metric-label">Avg Complexity</div>
                    <div class="metric-value {}">{:.1}</div>
                </div>
                <div class="metric-card">
                    <div class="metric-label">Code Smells</div>
                    <div class="metric-value {}">{}</div>
                </div>
            </div>

            <h3>📈 Maintainability Index</h3>
            <div class="progress-bar">
                <div class="progress-fill" style="width: {}%">
                    {:.1}/100 <span class="grade {}">Grade: {}</span>
                </div>
            </div>
        </section>
    "#,
        metrics.total_files,
        metrics.total_code_lines(),
        if metrics.average_cyclomatic() > 20.0 { "danger" } else if metrics.average_cyclomatic() > 10.0 { "warning" } else { "success" },
        metrics.average_cyclomatic(),
        if metrics.total_code_smells() > 20 { "danger" } else if metrics.total_code_smells() > 10 { "warning" } else { "success" },
        metrics.total_code_smells(),
        mi,
        mi,
        grade_class,
        mi_grade
    )
}

/// Generate charts section
fn generate_charts_section(_metrics: &ProjectMetrics) -> String {
    format!(r#"
        <section>
            <h2>📊 Visual Analytics</h2>
            <div class="chart-container">
                <canvas id="complexityChart"></canvas>
            </div>
            <div class="chart-container">
                <canvas id="codeCompositionChart"></canvas>
            </div>
        </section>
    "#)
}

/// Generate languages section
fn generate_languages_section(metrics: &ProjectMetrics) -> String {
    let mut languages: Vec<_> = metrics.by_language.iter().collect();
    languages.sort_by(|a, b| b.1.total_lines.cmp(&a.1.total_lines));

    let rows: Vec<String> = languages.iter().map(|(name, lang_metrics)| {
        let emoji = get_language_emoji(name);
        let mi_grade = crate::metrics::FileMetrics::get_maintainability_grade(lang_metrics.avg_maintainability_index);

        format!(r#"
                <tr>
                    <td>{} {}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{:.1}</td>
                    <td>{:.1}</td>
                    <td>{:.1} ({})</td>
                </tr>
        "#,
            emoji, name,
            lang_metrics.file_count,
            lang_metrics.total_lines,
            lang_metrics.code_lines,
            lang_metrics.avg_cyclomatic,
            lang_metrics.avg_nesting_depth,
            lang_metrics.avg_maintainability_index,
            mi_grade
        )
    }).collect();

    format!(r#"
        <section>
            <h2>🌍 Languages Breakdown</h2>
            <table>
                <thead>
                    <tr>
                        <th>Language</th>
                        <th>Files</th>
                        <th>Lines</th>
                        <th>Code</th>
                        <th>Complexity</th>
                        <th>Nesting</th>
                        <th>MI (Grade)</th>
                    </tr>
                </thead>
                <tbody>
                    {}
                </tbody>
            </table>
        </section>
    "#, rows.join(""))
}

/// Generate code smells section
fn generate_code_smells_section(metrics: &ProjectMetrics) -> String {
    let total_smells = metrics.total_code_smells();
    if total_smells == 0 {
        return String::from(r#"
        <section>
            <h2>👃 Code Smells</h2>
            <div class="metric-card" style="text-align: center; padding: 2rem;">
                <div class="metric-value success">✨ No code smells detected!</div>
                <div class="metric-label">Your code is clean</div>
            </div>
        </section>
        "#);
    }

    format!(r#"
        <section>
            <h2>👃 Code Smells</h2>
            <div class="metric-grid">
                <div class="metric-card">
                    <div class="metric-label">Long Functions (&gt;50 lines)</div>
                    <div class="metric-value warning">{}</div>
                </div>
                <div class="metric-card">
                    <div class="metric-label">Long Parameter Lists (&gt;5)</div>
                    <div class="metric-value warning">{}</div>
                </div>
                <div class="metric-card">
                    <div class="metric-label">Deep Nesting (&gt;4 levels)</div>
                    <div class="metric-value danger">{}</div>
                </div>
                <div class="metric-card">
                    <div class="metric-label">Magic Numbers</div>
                    <div class="metric-value warning">{}</div>
                </div>
            </div>
            <p style="text-align: center; margin-top: 1rem;">
                <span class="badge badge-warning">Total: {} code smells detected</span>
            </p>
        </section>
    "#,
        metrics.total_long_functions(),
        metrics.total_long_parameter_lists(),
        metrics.total_deep_nesting(),
        metrics.total_magic_numbers(),
        total_smells
    )
}

/// Generate insights section
fn generate_insights_section(metrics: &ProjectMetrics) -> String {
    let mut insights = Vec::new();
    let avg_complexity = metrics.average_cyclomatic();
    let avg_nesting = metrics.average_nesting_depth();
    let comment_quality = metrics.average_comment_quality();
    let total_smells = metrics.total_code_smells();

    if avg_complexity < 5.0 {
        insights.push(r#"<li class="badge badge-success">✅ Low complexity - Simple codebase</li>"#);
    } else if avg_complexity > 20.0 {
        insights.push(r#"<li class="badge badge-danger">❌ Very high complexity - Urgent refactoring needed</li>"#);
    } else if avg_complexity > 10.0 {
        insights.push(r#"<li class="badge badge-warning">⚠️ High complexity - Consider refactoring</li>"#);
    }

    if avg_nesting < 2.0 {
        insights.push(r#"<li class="badge badge-success">✅ Shallow nesting - Flat structure</li>"#);
    } else if avg_nesting > 4.0 {
        insights.push(r#"<li class="badge badge-warning">⚠️ Deep nesting - Reduce complexity</li>"#);
    }

    if comment_quality > 0.5 {
        insights.push(r#"<li class="badge badge-success">✅ Good documentation</li>"#);
    } else if comment_quality < 0.3 {
        insights.push(r#"<li class="badge badge-warning">⚠️ Improve documentation</li>"#);
    }

    if total_smells == 0 {
        insights.push(r#"<li class="badge badge-success">✨ No code smells detected</li>"#);
    } else if total_smells > 20 {
        insights.push(r#"<li class="badge badge-danger">⚠️ Many code smells - Review needed</li>"#);
    }

    format!(r#"
        <section>
            <h2>💡 Insights & Recommendations</h2>
            <ul style="list-style: none; display: flex; flex-wrap: wrap; gap: 0.5rem;">
                {}
            </ul>
        </section>
    "#, insights.join("\n"))
}

/// Generate embedded JavaScript for interactivity
fn generate_embedded_js(metrics: &ProjectMetrics) -> String {
    let mut languages: Vec<_> = metrics.by_language.iter().collect();
    languages.sort_by(|a, b| b.1.total_lines.cmp(&a.1.total_lines));

    let labels: Vec<String> = languages.iter().map(|(name, _)| format!("'{}'", name)).collect();
    let complexity_data: Vec<String> = languages.iter().map(|(_, m)| format!("{:.1}", m.avg_cyclomatic)).collect();
    let nesting_data: Vec<String> = languages.iter().map(|(_, m)| format!("{:.1}", m.avg_nesting_depth)).collect();

    let code_lines = metrics.total_code_lines();
    let comment_lines = metrics.total_comment_lines();
    let blank_lines = metrics.by_language.values().map(|m| m.blank_lines).sum::<usize>();

    format!(r#"<script>
        // Dark mode toggle
        function toggleDarkMode() {{
            const html = document.documentElement;
            const currentTheme = html.getAttribute('data-theme');
            const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
            html.setAttribute('data-theme', newTheme);
            localStorage.setItem('theme', newTheme);
        }}

        // Load saved theme
        const savedTheme = localStorage.getItem('theme') || 'light';
        document.documentElement.setAttribute('data-theme', savedTheme);

        // Export CSV functionality
        function exportCSV() {{
            const tables = document.querySelectorAll('table');
            if (tables.length === 0) return;

            let csv = '';
            const table = tables[0];
            const rows = table.querySelectorAll('tr');

            rows.forEach(row => {{
                const cells = row.querySelectorAll('th, td');
                const rowData = Array.from(cells).map(cell => {{
                    return '"' + cell.textContent.replace(/"/g, '""') + '"';
                }}).join(',');
                csv += rowData + '\n';
            }});

            const blob = new Blob([csv], {{ type: 'text/csv' }});
            const url = window.URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = 'lucent-metrics.csv';
            a.click();
        }}

        // Initialize charts when Chart.js loads
        window.addEventListener('load', function() {{
            if (typeof Chart === 'undefined') {{
                console.warn('Chart.js failed to load');
                return;
            }}

            // Complexity chart
            const complexityCtx = document.getElementById('complexityChart');
            if (complexityCtx) {{
                new Chart(complexityCtx, {{
                    type: 'bar',
                    data: {{
                        labels: [{}],
                        datasets: [{{
                            label: 'Cyclomatic Complexity',
                            data: [{}],
                            backgroundColor: 'rgba(147, 51, 234, 0.7)',
                            borderColor: 'rgba(147, 51, 234, 1)',
                            borderWidth: 2
                        }}, {{
                            label: 'Nesting Depth',
                            data: [{}],
                            backgroundColor: 'rgba(96, 165, 250, 0.7)',
                            borderColor: 'rgba(96, 165, 250, 1)',
                            borderWidth: 2
                        }}]
                    }},
                    options: {{
                        responsive: true,
                        maintainAspectRatio: false,
                        plugins: {{
                            title: {{
                                display: true,
                                text: 'Complexity & Nesting by Language',
                                font: {{ size: 16, weight: 'bold' }}
                            }},
                            legend: {{
                                position: 'top'
                            }}
                        }},
                        scales: {{
                            y: {{
                                beginAtZero: true,
                                title: {{
                                    display: true,
                                    text: 'Value'
                                }}
                            }}
                        }}
                    }}
                }});
            }}

            // Code composition pie chart
            const compositionCtx = document.getElementById('codeCompositionChart');
            if (compositionCtx) {{
                new Chart(compositionCtx, {{
                    type: 'doughnut',
                    data: {{
                        labels: ['Code Lines', 'Comment Lines', 'Blank Lines'],
                        datasets: [{{
                            data: [{}, {}, {}],
                            backgroundColor: [
                                'rgba(147, 51, 234, 0.7)',
                                'rgba(16, 185, 129, 0.7)',
                                'rgba(156, 163, 175, 0.7)'
                            ],
                            borderColor: [
                                'rgba(147, 51, 234, 1)',
                                'rgba(16, 185, 129, 1)',
                                'rgba(156, 163, 175, 1)'
                            ],
                            borderWidth: 2
                        }}]
                    }},
                    options: {{
                        responsive: true,
                        maintainAspectRatio: false,
                        plugins: {{
                            title: {{
                                display: true,
                                text: 'Code Composition',
                                font: {{ size: 16, weight: 'bold' }}
                            }},
                            legend: {{
                                position: 'right'
                            }}
                        }}
                    }}
                }});
            }}
        }});
    </script>"#,
        labels.join(", "),
        complexity_data.join(", "),
        nesting_data.join(", "),
        code_lines,
        comment_lines,
        blank_lines
    )
}

/// Get emoji for a language name
fn get_language_emoji(name: &str) -> &'static str {
    match name {
        "Rust" => "🦀",
        "Python" => "🐍",
        "JavaScript" => "📜",
        "TypeScript" => "💠",
        "Go" => "🐹",
        "Java" => "☕",
        "C++" => "⚙️",
        "C" => "🔧",
        "Ruby" => "💎",
        "PHP" => "🐘",
        "C#" => "#️⃣",
        "Swift" => "🦅",
        "Kotlin" => "🎯",
        "Dart" => "🎯",
        "Scala" => "🔴",
        "Elixir" => "💧",
        "Haskell" => "λ",
        "Clojure" => "🌀",
        "Lua" => "🌙",
        "Shell" => "🐚",
        "SQL" => "🗄️",
        _ => "📄",
    }
}

/// Row for language table
#[derive(Tabled)]
struct LanguageRow {
    #[tabled(rename = "Language")]
    language: String,
    #[tabled(rename = "Files")]
    files: usize,
    #[tabled(rename = "Lines")]
    lines: usize,
    #[tabled(rename = "Code")]
    code: usize,
    #[tabled(rename = "Comments")]
    comments: usize,
    #[tabled(rename = "Blanks")]
    blanks: usize,
    #[tabled(rename = "Complexity")]
    complexity: String,
    #[tabled(rename = "Nesting")]
    nesting: String,
    #[tabled(rename = "Entropy")]
    entropy: String,
}

/// JSON output structure
#[derive(Serialize)]
struct JsonOutput {
    summary: JsonSummary,
    languages: Vec<JsonLanguage>,
    violations: Vec<crate::Violation>,
}

#[derive(Serialize)]
struct JsonSummary {
    total_files: usize,
    total_lines: usize,
    code_lines: usize,
    comment_lines: usize,
    blank_lines: usize,
    average_entropy: f64,
    average_complexity: f64, // cyclomatic
    average_cognitive: f64,  // cognitive complexity
    average_nesting_depth: f64,
    total_functions: usize,
    average_function_length: f64,
    total_todos: usize,
    total_fixmes: usize,
    total_hacks: usize,
    average_comment_quality: f64,
    low_quality_comments: usize,
    high_quality_comments: usize,
    code_smells: JsonCodeSmells,
    maintainability_index: f64,
    maintainability_grade: String,
}

#[derive(Serialize)]
struct JsonCodeSmells {
    total: usize,
    long_functions: usize,
    long_parameter_lists: usize,
    deep_nesting: usize,
    magic_numbers: usize,
}

#[derive(Serialize)]
struct JsonLanguage {
    name: String,
    file_count: usize,
    total_lines: usize,
    code_lines: usize,
    comment_lines: usize,
    blank_lines: usize,
    average_entropy: f64,
    average_cyclomatic: f64,
    average_cognitive: f64,
    average_nesting_depth: f64,
    total_functions: usize,
    average_function_length: f64,
    todo_count: usize,
    fixme_count: usize,
    hack_count: usize,
    average_comment_quality: f64,
    low_quality_comments: usize,
    high_quality_comments: usize,
    long_functions: usize,
    long_parameter_lists: usize,
    deep_nesting: usize,
    magic_numbers: usize,
    maintainability_index: f64,
    maintainability_grade: String,
}

impl JsonOutput {
    /// Create JSON output with violations
    fn with_violations(metrics: &ProjectMetrics, violations: &[crate::Violation]) -> Self {
        let mut output = Self::from(metrics);
        output.violations = violations.to_vec();
        output
    }
}

impl From<&ProjectMetrics> for JsonOutput {
    fn from(metrics: &ProjectMetrics) -> Self {
        let languages = metrics
            .by_language
            .iter()
            .map(|(name, lang_metrics)| JsonLanguage {
                name: name.clone(),
                file_count: lang_metrics.file_count,
                total_lines: lang_metrics.total_lines,
                code_lines: lang_metrics.code_lines,
                comment_lines: lang_metrics.comment_lines,
                blank_lines: lang_metrics.blank_lines,
                average_entropy: lang_metrics.avg_entropy,
                average_cyclomatic: lang_metrics.avg_cyclomatic,
                average_cognitive: lang_metrics.avg_cognitive,
                average_nesting_depth: lang_metrics.avg_nesting_depth,
                total_functions: lang_metrics.total_functions,
                average_function_length: lang_metrics.avg_function_length,
                todo_count: lang_metrics.todo_count,
                fixme_count: lang_metrics.fixme_count,
                hack_count: lang_metrics.hack_count,
                average_comment_quality: lang_metrics.avg_comment_quality,
                low_quality_comments: lang_metrics.low_quality_comments,
                high_quality_comments: lang_metrics.high_quality_comments,
                long_functions: lang_metrics.long_functions,
                long_parameter_lists: lang_metrics.long_parameter_lists,
                deep_nesting: lang_metrics.deep_nesting_instances,
                magic_numbers: lang_metrics.magic_numbers,
                maintainability_index: lang_metrics.avg_maintainability_index,
                maintainability_grade: crate::metrics::FileMetrics::get_maintainability_grade(
                    lang_metrics.avg_maintainability_index
                ).to_string(),
            })
            .collect();

        Self {
            summary: JsonSummary {
                total_files: metrics.total_files,
                total_lines: metrics.total_lines(),
                code_lines: metrics.total_code_lines(),
                comment_lines: metrics.total_comment_lines(),
                blank_lines: metrics
                    .by_language
                    .values()
                    .map(|m| m.blank_lines)
                    .sum(),
                average_entropy: metrics.average_entropy(),
                average_complexity: metrics.average_cyclomatic(),
                average_cognitive: metrics.average_cognitive(),
                average_nesting_depth: metrics.average_nesting_depth(),
                total_functions: metrics.total_functions(),
                average_function_length: metrics.average_function_length(),
                total_todos: metrics.total_todos(),
                total_fixmes: metrics.total_fixmes(),
                total_hacks: metrics.total_hacks(),
                average_comment_quality: metrics.average_comment_quality(),
                low_quality_comments: metrics.total_low_quality_comments(),
                high_quality_comments: metrics.total_high_quality_comments(),
                code_smells: JsonCodeSmells {
                    total: metrics.total_code_smells(),
                    long_functions: metrics.total_long_functions(),
                    long_parameter_lists: metrics.total_long_parameter_lists(),
                    deep_nesting: metrics.total_deep_nesting(),
                    magic_numbers: metrics.total_magic_numbers(),
                },
                maintainability_index: metrics.average_maintainability_index(),
                maintainability_grade: crate::metrics::FileMetrics::get_maintainability_grade(
                    metrics.average_maintainability_index()
                ).to_string(),
            },
            languages,
            violations: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::LanguageMetrics;
    use std::collections::HashMap;

    #[test]
    fn test_json_output() {
        let mut metrics = ProjectMetrics::new();
        metrics.total_files = 1;

        let mut by_lang = HashMap::new();
        by_lang.insert(
            "Rust".to_string(),
            LanguageMetrics {
                file_count: 1,
                total_lines: 100,
                code_lines: 80,
                comment_lines: 10,
                blank_lines: 10,
                avg_entropy: 4.2,
                avg_cyclomatic: 5.0,
                avg_cognitive: 8.0,
                avg_nesting_depth: 3.0,
                total_functions: 5,
                avg_function_length: 15.0,
                todo_count: 0,
                fixme_count: 0,
                hack_count: 0,
                avg_comment_quality: 0.6,
                low_quality_comments: 1,
                high_quality_comments: 2,
                long_functions: 0,
                long_parameter_lists: 0,
                deep_nesting_instances: 0,
                magic_numbers: 0,
                avg_maintainability_index: 75.0,
            },
        );
        metrics.by_language = by_lang;

        let json = JsonOutput::from(&metrics);
        assert_eq!(json.summary.total_files, 1);
        assert_eq!(json.languages.len(), 1);
    }
}
