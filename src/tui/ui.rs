//! UI rendering for the TUI

use super::app::{App, FileEntry, View};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, Gauge, List, ListItem, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table, Tabs, Wrap,
    },
    Frame,
};

/// Main draw function
pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(area);

    // Draw header
    draw_header(frame, app, chunks[0]);

    // Draw main content based on view
    match app.view {
        View::Dashboard => draw_dashboard(frame, app, chunks[1]),
        View::FileList => draw_file_list(frame, app, chunks[1]),
        View::FileDetail => draw_file_detail(frame, app, chunks[1]),
        View::Help => draw_help(frame, app, chunks[1]),
    }

    // Draw status bar
    draw_status_bar(frame, app, chunks[2]);
}

/// Draw the header
fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
    let title = format!(
        " 💎 Lucent - {} ",
        app.project_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| app.project_path.to_string_lossy().to_string())
    );

    // Create tabs for navigation
    let tabs = vec!["Dashboard", "Files", "Help"];
    let selected = match app.view {
        View::Dashboard => 0,
        View::FileList | View::FileDetail => 1,
        View::Help => 2,
    };

    let tabs_widget = Tabs::new(tabs)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .title_style(Style::default().fg(Color::Cyan).bold()),
        )
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).bold());

    frame.render_widget(tabs_widget, area);
}

/// Draw the dashboard view
fn draw_dashboard(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // Summary stats
            Constraint::Length(10), // Hotspots
            Constraint::Min(5),     // Language breakdown
        ])
        .margin(1)
        .split(area);

    // Summary statistics
    draw_summary_stats(frame, app, chunks[0]);

    // Hotspots (highest complexity files)
    draw_hotspots(frame, app, chunks[1]);

    // Language breakdown
    draw_language_breakdown(frame, app, chunks[2]);
}

/// Draw summary statistics
fn draw_summary_stats(frame: &mut Frame, app: &App, area: Rect) {
    let Some(metrics) = &app.metrics else {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" 📊 Summary ");
        let loading = Paragraph::new("Analyzing...").block(block);
        frame.render_widget(loading, area);
        return;
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 📊 Project Summary ")
        .title_style(Style::default().fg(Color::Green).bold());

    // Create stats layout
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let stat_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(inner);

    // Compute aggregate stats from by_language HashMap
    let total_files = metrics.total_files;
    let (total_code_lines, avg_cyclomatic, avg_nesting, avg_maintainability) = {
        let mut code_lines: usize = 0;
        let mut total_cyclomatic: f64 = 0.0;
        let mut total_nesting: f64 = 0.0;
        let mut total_mi: f64 = 0.0;
        let mut file_count: usize = 0;

        for lang_metrics in metrics.by_language.values() {
            code_lines += lang_metrics.code_lines;
            total_cyclomatic += lang_metrics.avg_cyclomatic * lang_metrics.file_count as f64;
            total_nesting += lang_metrics.avg_nesting_depth * lang_metrics.file_count as f64;
            total_mi += lang_metrics.avg_maintainability_index * lang_metrics.file_count as f64;
            file_count += lang_metrics.file_count;
        }

        let avg_cc = if file_count > 0 { total_cyclomatic / file_count as f64 } else { 0.0 };
        let avg_nest = if file_count > 0 { total_nesting / file_count as f64 } else { 0.0 };
        let avg_mi = if file_count > 0 { total_mi / file_count as f64 } else { 0.0 };

        (code_lines, avg_cc, avg_nest, avg_mi)
    };

    // File count
    draw_stat_box(
        frame,
        "Files",
        &total_files.to_string(),
        Color::Cyan,
        stat_chunks[0],
    );

    // Lines of code
    draw_stat_box(
        frame,
        "Code Lines",
        &format_number(total_code_lines),
        Color::Blue,
        stat_chunks[1],
    );

    // Average complexity
    let cc_color = complexity_color(avg_cyclomatic);
    draw_stat_box(
        frame,
        "Avg Complexity",
        &format!("{:.1}", avg_cyclomatic),
        cc_color,
        stat_chunks[2],
    );

    // Average nesting
    let nest_color = nesting_color(avg_nesting);
    draw_stat_box(
        frame,
        "Avg Nesting",
        &format!("{:.1}", avg_nesting),
        nest_color,
        stat_chunks[3],
    );

    // Maintainability grade
    let grade = maintainability_grade(avg_maintainability);
    let mi_color = maintainability_color(avg_maintainability);
    draw_stat_box(
        frame,
        "Maintainability",
        &format!("{} ({:.0})", grade, avg_maintainability),
        mi_color,
        stat_chunks[4],
    );
}

/// Draw a single stat box
fn draw_stat_box(frame: &mut Frame, label: &str, value: &str, color: Color, area: Rect) {
    let content = vec![
        Line::from(Span::styled(
            value,
            Style::default().fg(color).bold().add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(label, Style::default().fg(Color::DarkGray))),
    ];

    let paragraph = Paragraph::new(content).alignment(Alignment::Center);

    // Center vertically
    let vertical_padding = (area.height.saturating_sub(2)) / 2;
    let inner = Rect {
        x: area.x,
        y: area.y + vertical_padding,
        width: area.width,
        height: 2,
    };

    frame.render_widget(paragraph, inner);
}

/// Draw hotspots (files with highest complexity)
fn draw_hotspots(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 🔥 Hotspots (Highest Complexity) ")
        .title_style(Style::default().fg(Color::Red).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Get top 5 files by complexity
    let mut files = app.files.clone();
    files.sort_by(|a, b| {
        b.metrics
            .cyclomatic_complexity
            .cmp(&a.metrics.cyclomatic_complexity)
    });

    let rows: Vec<Row> = files
        .iter()
        .take(5)
        .map(|f| {
            let cc = f.metrics.cyclomatic_complexity;
            let color = complexity_color(cc as f64);
            Row::new(vec![
                Cell::from(truncate_path(&f.relative_path, 40)),
                Cell::from(format!("CC: {}", cc)).style(Style::default().fg(color)),
                Cell::from(format!("MI: {:.0}", f.metrics.maintainability_index)),
                Cell::from(format!("{} lines", f.metrics.code_lines)),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(20),
        ],
    )
    .header(
        Row::new(vec!["File", "Complexity", "MI", "Lines"])
            .style(Style::default().fg(Color::Yellow).bold()),
    );

    frame.render_widget(table, inner);
}

/// Draw language breakdown
fn draw_language_breakdown(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 🗂️ Languages ")
        .title_style(Style::default().fg(Color::Magenta).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let Some(metrics) = &app.metrics else {
        return;
    };

    // Build language stats
    let mut lang_stats: std::collections::HashMap<String, (usize, usize)> =
        std::collections::HashMap::new();

    for file in &app.files {
        let entry = lang_stats
            .entry(file.language.clone())
            .or_insert((0, 0));
        entry.0 += 1;
        entry.1 += file.metrics.code_lines;
    }

    let mut stats: Vec<_> = lang_stats.into_iter().collect();
    stats.sort_by(|a, b| b.1 .1.cmp(&a.1 .1)); // Sort by lines

    // Compute total code lines from by_language
    let total_lines: usize = metrics.by_language.values().map(|m| m.code_lines).sum();

    let items: Vec<ListItem> = stats
        .iter()
        .take(8)
        .map(|(lang, (files, lines))| {
            let pct = if total_lines > 0 {
                (*lines as f64 / total_lines as f64) * 100.0
            } else {
                0.0
            };
            let bar_width = (pct / 100.0 * 20.0) as usize;
            let bar: String = "█".repeat(bar_width) + &"░".repeat(20 - bar_width);

            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{:12}", lang),
                    Style::default().fg(language_color(lang)),
                ),
                Span::raw(" "),
                Span::styled(bar, Style::default().fg(Color::Blue)),
                Span::raw(" "),
                Span::styled(
                    format!("{:>5} files, {:>6} lines ({:.1}%)", files, lines, pct),
                    Style::default().fg(Color::DarkGray),
                ),
            ]))
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}

/// Draw the file list view
fn draw_file_list(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(
            " 📁 Files [Sort: {} {}] ",
            app.sort_by.display_name(),
            if app.sort_ascending { "↑" } else { "↓" }
        ))
        .title_style(Style::default().fg(Color::Cyan).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Filter bar if active
    let (list_area, filter_area) = if app.filter_active || !app.filter.is_empty() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)])
            .split(inner);
        (chunks[1], Some(chunks[0]))
    } else {
        (inner, None)
    };

    // Draw filter bar
    if let Some(filter_area) = filter_area {
        let filter_text = if app.filter_active {
            format!("Filter: {}▌", app.filter)
        } else {
            format!("Filter: {} (press / to edit)", app.filter)
        };
        let filter_style = if app.filter_active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let filter_para = Paragraph::new(filter_text).style(filter_style);
        frame.render_widget(filter_para, filter_area);
    }

    // Get filtered files
    let files = app.filtered_files();

    // Build table rows
    let rows: Vec<Row> = files
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let is_selected = i == app.selected_index;
            let style = if is_selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };

            let cc_color = complexity_color(f.metrics.cyclomatic_complexity as f64);
            let mi_color = maintainability_color(f.metrics.maintainability_index);

            Row::new(vec![
                Cell::from(truncate_path(&f.relative_path, 45)),
                Cell::from(f.language.clone()),
                Cell::from(format!("{}", f.metrics.code_lines)),
                Cell::from(format!("{}", f.metrics.cyclomatic_complexity))
                    .style(Style::default().fg(cc_color)),
                Cell::from(format!("{}", f.metrics.cognitive_complexity)),
                Cell::from(format!("{}", f.metrics.max_nesting_depth)),
                Cell::from(format!("{:.0}", f.metrics.maintainability_index))
                    .style(Style::default().fg(mi_color)),
            ])
            .style(style)
        })
        .collect();

    let header = Row::new(vec!["File", "Lang", "Lines", "CC", "Cog", "Nest", "MI"])
        .style(Style::default().fg(Color::Yellow).bold());

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(40),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ],
    )
    .header(header)
    .row_highlight_style(Style::default().bg(Color::DarkGray));

    frame.render_widget(table, list_area);

    // Draw scrollbar if needed
    if files.len() > list_area.height as usize {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        let mut scrollbar_state =
            ScrollbarState::new(files.len()).position(app.selected_index);

        frame.render_stateful_widget(
            scrollbar,
            list_area.inner(ratatui::layout::Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }
}

/// Draw file detail view
fn draw_file_detail(frame: &mut Frame, app: &App, area: Rect) {
    let Some(file) = &app.selected_file else {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" 📄 File Details ");
        let para = Paragraph::new("No file selected").block(block);
        frame.render_widget(para, area);
        return;
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" 📄 {} ", file.relative_path))
        .title_style(Style::default().fg(Color::Cyan).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Main metrics
            Constraint::Length(6),  // Code markers
            Constraint::Length(4),  // Complexity bar
            Constraint::Min(4),     // Code smells
        ])
        .margin(1)
        .split(inner);

    // Main metrics
    draw_file_metrics(frame, file, chunks[0]);

    // Code markers (TODOs, FIXMEs, etc.)
    draw_code_markers(frame, file, chunks[1]);

    // Complexity gauge
    draw_complexity_gauge(frame, file, chunks[2]);

    // Code smells
    draw_code_smells(frame, file, chunks[3]);
}

/// Draw file metrics
fn draw_file_metrics(frame: &mut Frame, file: &FileEntry, area: Rect) {
    let metrics = &file.metrics;

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left column
    let left_text = vec![
        Line::from(vec![
            Span::raw("Language:     "),
            Span::styled(&file.language, Style::default().fg(Color::Cyan).bold()),
        ]),
        Line::from(vec![
            Span::raw("Total Lines:  "),
            Span::styled(
                metrics.total_lines.to_string(),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::raw("Code Lines:   "),
            Span::styled(
                metrics.code_lines.to_string(),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("Comment Lines:"),
            Span::styled(
                format!(
                    " {} ({:.1}%)",
                    metrics.comment_lines,
                    if metrics.total_lines > 0 {
                        (metrics.comment_lines as f64 / metrics.total_lines as f64) * 100.0
                    } else {
                        0.0
                    }
                ),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("Blank Lines:  "),
            Span::styled(
                metrics.blank_lines.to_string(),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
    ];
    frame.render_widget(Paragraph::new(left_text), chunks[0]);

    // Right column
    let cc_color = complexity_color(metrics.cyclomatic_complexity as f64);
    let mi_color = maintainability_color(metrics.maintainability_index);

    let right_text = vec![
        Line::from(vec![
            Span::raw("Cyclomatic:   "),
            Span::styled(
                metrics.cyclomatic_complexity.to_string(),
                Style::default().fg(cc_color).bold(),
            ),
        ]),
        Line::from(vec![
            Span::raw("Cognitive:    "),
            Span::styled(
                metrics.cognitive_complexity.to_string(),
                Style::default().fg(Color::Magenta),
            ),
        ]),
        Line::from(vec![
            Span::raw("Nesting Depth:"),
            Span::styled(
                format!(" {}", metrics.max_nesting_depth),
                Style::default().fg(nesting_color(metrics.max_nesting_depth as f64)),
            ),
        ]),
        Line::from(vec![
            Span::raw("Maintainability:"),
            Span::styled(
                format!(
                    " {} ({:.1})",
                    maintainability_grade(metrics.maintainability_index),
                    metrics.maintainability_index
                ),
                Style::default().fg(mi_color).bold(),
            ),
        ]),
        Line::from(vec![
            Span::raw("Entropy:      "),
            Span::styled(
                format!("{:.2}", metrics.entropy),
                Style::default().fg(Color::Blue),
            ),
        ]),
    ];
    frame.render_widget(Paragraph::new(right_text), chunks[1]);
}

/// Draw code markers
fn draw_code_markers(frame: &mut Frame, file: &FileEntry, area: Rect) {
    let metrics = &file.metrics;

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 🏷️ Code Markers ")
        .title_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let text = vec![
        Line::from(vec![
            Span::styled("TODO:  ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("{}", metrics.todo_count)),
            Span::raw("  │  "),
            Span::styled("FIXME: ", Style::default().fg(Color::Red)),
            Span::raw(format!("{}", metrics.fixme_count)),
            Span::raw("  │  "),
            Span::styled("HACK:  ", Style::default().fg(Color::Magenta)),
            Span::raw(format!("{}", metrics.hack_count)),
        ]),
        Line::from(vec![
            Span::styled("Functions: ", Style::default().fg(Color::Cyan)),
            Span::raw(format!("{}", metrics.function_count)),
            Span::raw("  │  "),
            Span::styled("Avg Len: ", Style::default().fg(Color::Green)),
            Span::raw(format!("{:.1}", metrics.avg_function_length)),
        ]),
    ];

    frame.render_widget(Paragraph::new(text), inner);
}

/// Draw complexity gauge
fn draw_complexity_gauge(frame: &mut Frame, file: &FileEntry, area: Rect) {
    let cc = file.metrics.cyclomatic_complexity;
    let ratio = (cc as f64 / 50.0).min(1.0); // Max 50 for gauge

    let color = complexity_color(cc as f64);
    let label = format!("Complexity: {} / 50", cc);

    let gauge = Gauge::default()
        .block(Block::default().title(" Complexity "))
        .gauge_style(Style::default().fg(color))
        .percent((ratio * 100.0) as u16)
        .label(label);

    frame.render_widget(gauge, area);
}

/// Draw code smells
fn draw_code_smells(frame: &mut Frame, file: &FileEntry, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" ⚠️ Issues ")
        .title_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let smells = &file.metrics.code_smells;

    if smells.is_empty() {
        let text = Paragraph::new(Span::styled(
            "✓ No code smells detected",
            Style::default().fg(Color::Green),
        ));
        frame.render_widget(text, inner);
        return;
    }

    let items: Vec<ListItem> = smells
        .iter()
        .take(5)
        .map(|smell| {
            let color = match smell.severity {
                crate::metrics::SmellSeverity::Minor => Color::Yellow,
                crate::metrics::SmellSeverity::Moderate => Color::Rgb(255, 165, 0), // Orange
                crate::metrics::SmellSeverity::Major => Color::Red,
            };
            ListItem::new(Line::from(vec![
                Span::styled("• ", Style::default().fg(color)),
                Span::raw(&smell.description),
            ]))
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}

/// Draw help view
fn draw_help(frame: &mut Frame, _app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" ❓ Help ")
        .title_style(Style::default().fg(Color::Cyan).bold());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let help_text = vec![
        Line::from(Span::styled(
            "Navigation",
            Style::default().fg(Color::Yellow).bold(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ↑/k, ↓/j  ", Style::default().fg(Color::Cyan)),
            Span::raw("Move selection up/down"),
        ]),
        Line::from(vec![
            Span::styled("  PgUp/PgDn ", Style::default().fg(Color::Cyan)),
            Span::raw("Page up/down"),
        ]),
        Line::from(vec![
            Span::styled("  g/G       ", Style::default().fg(Color::Cyan)),
            Span::raw("Go to first/last"),
        ]),
        Line::from(vec![
            Span::styled("  Enter     ", Style::default().fg(Color::Cyan)),
            Span::raw("Open file details"),
        ]),
        Line::from(vec![
            Span::styled("  Esc       ", Style::default().fg(Color::Cyan)),
            Span::raw("Go back"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Actions",
            Style::default().fg(Color::Yellow).bold(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  s         ", Style::default().fg(Color::Cyan)),
            Span::raw("Cycle sort field"),
        ]),
        Line::from(vec![
            Span::styled("  S         ", Style::default().fg(Color::Cyan)),
            Span::raw("Toggle sort order"),
        ]),
        Line::from(vec![
            Span::styled("  /         ", Style::default().fg(Color::Cyan)),
            Span::raw("Filter files"),
        ]),
        Line::from(vec![
            Span::styled("  r         ", Style::default().fg(Color::Cyan)),
            Span::raw("Refresh analysis"),
        ]),
        Line::from(vec![
            Span::styled("  d         ", Style::default().fg(Color::Cyan)),
            Span::raw("Go to dashboard"),
        ]),
        Line::from(vec![
            Span::styled("  f         ", Style::default().fg(Color::Cyan)),
            Span::raw("Go to file list"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "General",
            Style::default().fg(Color::Yellow).bold(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ?         ", Style::default().fg(Color::Cyan)),
            Span::raw("Show this help"),
        ]),
        Line::from(vec![
            Span::styled("  q         ", Style::default().fg(Color::Cyan)),
            Span::raw("Quit"),
        ]),
        Line::from(vec![
            Span::styled("  Ctrl+C    ", Style::default().fg(Color::Cyan)),
            Span::raw("Force quit"),
        ]),
    ];

    let paragraph = Paragraph::new(help_text).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, inner);
}

/// Draw status bar
fn draw_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let status = if let Some((msg, _)) = &app.status_message {
        msg.clone()
    } else {
        let shortcuts = match app.view {
            View::Dashboard => "Press [f] Files  [r] Refresh  [?] Help  [q] Quit",
            View::FileList => "[↑↓] Navigate  [s] Sort  [/] Filter  [Enter] Details  [?] Help",
            View::FileDetail => "[Esc] Back  [r] Refresh  [?] Help",
            View::Help => "Press any key to close",
        };
        shortcuts.to_string()
    };

    let status_bar = Paragraph::new(status)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);

    frame.render_widget(status_bar, area);
}

// Helper functions

fn complexity_color(cc: f64) -> Color {
    if cc <= 10.0 {
        Color::Green
    } else if cc <= 20.0 {
        Color::Yellow
    } else if cc <= 30.0 {
        Color::Rgb(255, 165, 0) // Orange
    } else {
        Color::Red
    }
}

fn nesting_color(depth: f64) -> Color {
    if depth <= 3.0 {
        Color::Green
    } else if depth <= 4.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

fn maintainability_color(mi: f64) -> Color {
    if mi >= 85.0 {
        Color::Green
    } else if mi >= 65.0 {
        Color::Yellow
    } else if mi >= 50.0 {
        Color::Rgb(255, 165, 0) // Orange
    } else {
        Color::Red
    }
}

fn maintainability_grade(mi: f64) -> &'static str {
    if mi >= 85.0 {
        "A"
    } else if mi >= 65.0 {
        "B"
    } else if mi >= 50.0 {
        "C"
    } else if mi >= 25.0 {
        "D"
    } else {
        "F"
    }
}

fn language_color(lang: &str) -> Color {
    match lang {
        "Rust" => Color::Rgb(222, 165, 132),     // Rust orange
        "Python" => Color::Rgb(55, 118, 171),    // Python blue
        "JavaScript" => Color::Rgb(247, 223, 30), // JS yellow
        "TypeScript" => Color::Rgb(49, 120, 198), // TS blue
        "Go" => Color::Rgb(0, 173, 216),         // Go cyan
        "Java" => Color::Rgb(176, 114, 25),      // Java orange
        "C++" | "C" => Color::Rgb(0, 89, 156),   // C blue
        "Ruby" => Color::Rgb(204, 52, 45),       // Ruby red
        "PHP" => Color::Rgb(119, 123, 180),      // PHP purple
        _ => Color::White,
    }
}

fn truncate_path(path: &str, max_len: usize) -> String {
    if path.len() <= max_len {
        path.to_string()
    } else {
        format!("...{}", &path[path.len() - max_len + 3..])
    }
}

fn format_number(n: usize) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
