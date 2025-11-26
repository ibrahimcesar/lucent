//! Custom widgets for the TUI
//!
//! This module contains custom ratatui widgets for Lucent.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

/// A simple spark line widget for showing trends
pub struct SparkLine<'a> {
    data: &'a [f64],
    max: Option<f64>,
    style: Style,
}

impl<'a> SparkLine<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        Self {
            data,
            max: None,
            style: Style::default(),
        }
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for SparkLine<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.data.is_empty() || area.width == 0 || area.height == 0 {
            return;
        }

        let max = self
            .max
            .unwrap_or_else(|| self.data.iter().cloned().fold(0.0, f64::max));

        let spark_chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

        for (i, &value) in self.data.iter().enumerate() {
            if i >= area.width as usize {
                break;
            }

            let normalized = if max > 0.0 { value / max } else { 0.0 };
            let char_index = ((normalized * 7.0).round() as usize).min(7);
            let spark_char = spark_chars[char_index];

            if let Some(cell) = buf.cell_mut((area.x + i as u16, area.y)) {
                cell.set_char(spark_char).set_style(self.style);
            }
        }
    }
}

/// A complexity meter widget
pub struct ComplexityMeter {
    value: usize,
    max: usize,
    show_label: bool,
}

impl ComplexityMeter {
    pub fn new(value: usize) -> Self {
        Self {
            value,
            max: 50,
            show_label: true,
        }
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max = max;
        self
    }

    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    fn color_for_value(&self) -> Color {
        let ratio = self.value as f64 / self.max as f64;
        if ratio <= 0.2 {
            Color::Green
        } else if ratio <= 0.4 {
            Color::Yellow
        } else if ratio <= 0.6 {
            Color::Rgb(255, 165, 0) // Orange
        } else {
            Color::Red
        }
    }
}

impl Widget for ComplexityMeter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let label_width = if self.show_label { 4 } else { 0 };
        let bar_width = area.width.saturating_sub(label_width);

        let filled = ((self.value as f64 / self.max as f64) * bar_width as f64)
            .round()
            .min(bar_width as f64) as u16;

        let color = self.color_for_value();

        // Draw filled portion
        for x in 0..filled {
            if let Some(cell) = buf.cell_mut((area.x + x, area.y)) {
                cell.set_char('█').set_fg(color);
            }
        }

        // Draw empty portion
        for x in filled..bar_width {
            if let Some(cell) = buf.cell_mut((area.x + x, area.y)) {
                cell.set_char('░').set_fg(Color::DarkGray);
            }
        }

        // Draw label
        if self.show_label && area.width > 4 {
            let label = format!("{:>3}", self.value);
            for (i, c) in label.chars().enumerate() {
                if bar_width + (i as u16) < area.width {
                    if let Some(cell) = buf.cell_mut((area.x + bar_width + i as u16, area.y)) {
                        cell.set_char(c).set_fg(color);
                    }
                }
            }
        }
    }
}

/// A grade badge widget (A, B, C, D, F)
pub struct GradeBadge {
    grade: char,
    score: f64,
}

impl GradeBadge {
    pub fn new(score: f64) -> Self {
        let grade = if score >= 85.0 {
            'A'
        } else if score >= 65.0 {
            'B'
        } else if score >= 50.0 {
            'C'
        } else if score >= 25.0 {
            'D'
        } else {
            'F'
        };
        Self { grade, score }
    }

    fn color(&self) -> Color {
        match self.grade {
            'A' => Color::Green,
            'B' => Color::Cyan,
            'C' => Color::Yellow,
            'D' => Color::Rgb(255, 165, 0),
            _ => Color::Red,
        }
    }
}

impl Widget for GradeBadge {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 3 || area.height == 0 {
            return;
        }

        let color = self.color();

        // Draw badge: [A]
        if let Some(cell) = buf.cell_mut((area.x, area.y)) {
            cell.set_char('[').set_fg(color);
        }
        if let Some(cell) = buf.cell_mut((area.x + 1, area.y)) {
            cell.set_char(self.grade).set_fg(color);
        }
        if let Some(cell) = buf.cell_mut((area.x + 2, area.y)) {
            cell.set_char(']').set_fg(color);
        }

        // Draw score if there's room
        if area.width >= 10 {
            let score_str = format!(" {:.0}", self.score);
            for (i, c) in score_str.chars().enumerate() {
                if area.x + 3 + (i as u16) < area.x + area.width {
                    if let Some(cell) = buf.cell_mut((area.x + 3 + i as u16, area.y)) {
                        cell.set_char(c).set_fg(Color::DarkGray);
                    }
                }
            }
        }
    }
}

/// A file type icon widget
pub struct FileTypeIcon {
    language: String,
}

impl FileTypeIcon {
    pub fn new(language: &str) -> Self {
        Self {
            language: language.to_string(),
        }
    }

    fn icon(&self) -> &'static str {
        match self.language.as_str() {
            "Rust" => "🦀",
            "Python" => "🐍",
            "JavaScript" => "📜",
            "TypeScript" => "💠",
            "Go" => "🐹",
            "Java" => "☕",
            "C++" | "C" => "⚙️",
            "Ruby" => "💎",
            "PHP" => "🐘",
            "Swift" => "🦅",
            "Kotlin" => "🎯",
            "Scala" => "🔴",
            "Elixir" => "💧",
            "Haskell" => "λ",
            "Shell" => "🐚",
            _ => "📄",
        }
    }
}

impl Widget for FileTypeIcon {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let icon = self.icon();
        // Icons are typically 2 cells wide
        for (i, c) in icon.chars().enumerate() {
            if area.x + (i as u16) < area.x + area.width {
                if let Some(cell) = buf.cell_mut((area.x + i as u16, area.y)) {
                    cell.set_char(c);
                }
            }
        }
    }
}
