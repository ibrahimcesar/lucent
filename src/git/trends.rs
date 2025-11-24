//! Trend tracking and visualization
//!
//! Track and visualize how metrics change over time across commits.

use super::commits::{CommitHistory, Trend};
use serde::{Serialize, Deserialize};

/// Trend tracker for visualizing metric changes
pub struct TrendTracker {
    history: CommitHistory,
}

impl TrendTracker {
    /// Create a new trend tracker from commit history
    pub fn new(history: CommitHistory) -> Self {
        Self { history }
    }

    /// Generate trend report
    pub fn generate_report(&self) -> TrendReport {
        let analyses = &self.history.analyses;

        if analyses.is_empty() {
            return TrendReport::default();
        }

        // Extract time series data
        let complexity_series: Vec<(i64, f64)> = analyses.iter()
            .map(|a| (a.commit.timestamp, a.metrics.avg_complexity()))
            .collect();

        let maintainability_series: Vec<(i64, f64)> = analyses.iter()
            .map(|a| (a.commit.timestamp, a.metrics.avg_maintainability()))
            .collect();

        let lines_series: Vec<(i64, usize)> = analyses.iter()
            .map(|a| (a.commit.timestamp, a.metrics.total_lines()))
            .collect();

        let files_series: Vec<(i64, usize)> = analyses.iter()
            .map(|a| (a.commit.timestamp, a.metrics.total_files))
            .collect();

        // Calculate statistics
        let complexity_stats = Self::calculate_stats(&complexity_series);
        let maintainability_stats = Self::calculate_stats(&maintainability_series);

        // Determine overall trend
        let overall_trend = self.history.trend();

        TrendReport {
            overall_trend,
            complexity_series,
            maintainability_series,
            lines_series,
            files_series,
            complexity_stats,
            maintainability_stats,
            commit_count: analyses.len(),
        }
    }

    /// Generate ASCII chart for terminal display
    pub fn ascii_chart(&self, metric: MetricType, height: usize) -> String {
        let series: Vec<f64> = match metric {
            MetricType::Complexity => {
                self.history.analyses.iter()
                    .map(|a| a.metrics.avg_complexity())
                    .collect()
            }
            MetricType::Maintainability => {
                self.history.analyses.iter()
                    .map(|a| a.metrics.avg_maintainability())
                    .collect()
            }
            MetricType::Lines => {
                self.history.analyses.iter()
                    .map(|a| a.metrics.total_lines() as f64)
                    .collect()
            }
        };

        if series.is_empty() {
            return String::from("No data available");
        }

        let max = series.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min = series.iter().cloned().fold(f64::INFINITY, f64::min);

        let mut chart = String::new();

        // Title
        chart.push_str(&format!("{} Over Time\n", metric));
        chart.push_str(&format!("Max: {:.1} | Min: {:.1}\n", max, min));
        chart.push_str(&"─".repeat(series.len().max(40)));
        chart.push('\n');

        // Draw chart rows
        for row in (0..height).rev() {
            let threshold = min + (max - min) * (row as f64 / height as f64);

            for value in &series {
                if *value >= threshold {
                    chart.push('█');
                } else {
                    chart.push(' ');
                }
            }
            chart.push('\n');
        }

        // X-axis
        chart.push_str(&"─".repeat(series.len().max(40)));
        chart.push('\n');
        chart.push_str(&format!("← {} commits →", series.len()));

        chart
    }

    /// Calculate statistics for a time series
    fn calculate_stats(series: &[(i64, f64)]) -> MetricStats {
        if series.is_empty() {
            return MetricStats::default();
        }

        let values: Vec<f64> = series.iter().map(|(_, v)| *v).collect();

        let sum: f64 = values.iter().sum();
        let mean = sum / values.len() as f64;

        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);

        // Calculate trend (linear regression slope)
        let n = series.len() as f64;
        let sum_x: i64 = series.iter().map(|(t, _)| *t).sum();
        let sum_y: f64 = values.iter().sum();
        let sum_xy: f64 = series.iter().map(|(t, v)| *t as f64 * v).sum();
        let sum_x2: f64 = series.iter().map(|(t, _)| (*t as f64).powi(2)).sum();

        let slope = (n * sum_xy - sum_x as f64 * sum_y) / (n * sum_x2 - (sum_x as f64).powi(2));

        MetricStats {
            mean,
            min,
            max,
            trend_slope: slope,
            data_points: values.len(),
        }
    }
}

/// Type of metric to track
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    /// Cyclomatic complexity
    Complexity,

    /// Maintainability index
    Maintainability,

    /// Total lines of code
    Lines,
}

impl std::fmt::Display for MetricType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricType::Complexity => write!(f, "Complexity"),
            MetricType::Maintainability => write!(f, "Maintainability"),
            MetricType::Lines => write!(f, "Lines of Code"),
        }
    }
}

/// Trend analysis report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrendReport {
    /// Overall trend direction
    pub overall_trend: Trend,

    /// Complexity over time (timestamp, value)
    pub complexity_series: Vec<(i64, f64)>,

    /// Maintainability over time
    pub maintainability_series: Vec<(i64, f64)>,

    /// Lines of code over time
    pub lines_series: Vec<(i64, usize)>,

    /// Number of files over time
    pub files_series: Vec<(i64, usize)>,

    /// Complexity statistics
    pub complexity_stats: MetricStats,

    /// Maintainability statistics
    pub maintainability_stats: MetricStats,

    /// Number of commits analyzed
    pub commit_count: usize,
}

impl Default for Trend {
    fn default() -> Self {
        Trend::Stable
    }
}

/// Statistics for a metric
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricStats {
    /// Mean value
    pub mean: f64,

    /// Minimum value
    pub min: f64,

    /// Maximum value
    pub max: f64,

    /// Trend slope (positive = increasing, negative = decreasing)
    pub trend_slope: f64,

    /// Number of data points
    pub data_points: usize,
}

impl std::fmt::Display for TrendReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "📊 Trend Analysis Report")?;
        writeln!(f, "Commits Analyzed: {}", self.commit_count)?;
        writeln!(f, "Overall Trend: {}", self.overall_trend)?;
        writeln!(f)?;

        writeln!(f, "Complexity Statistics:")?;
        writeln!(f, "  Mean: {:.2}", self.complexity_stats.mean)?;
        writeln!(f, "  Min: {:.2}", self.complexity_stats.min)?;
        writeln!(f, "  Max: {:.2}", self.complexity_stats.max)?;
        writeln!(f, "  Trend: {}", if self.complexity_stats.trend_slope > 0.0 { "↗ Increasing" } else if self.complexity_stats.trend_slope < 0.0 { "↘ Decreasing" } else { "→ Stable" })?;
        writeln!(f)?;

        writeln!(f, "Maintainability Statistics:")?;
        writeln!(f, "  Mean: {:.2}", self.maintainability_stats.mean)?;
        writeln!(f, "  Min: {:.2}", self.maintainability_stats.min)?;
        writeln!(f, "  Max: {:.2}", self.maintainability_stats.max)?;
        write!(f, "  Trend: {}", if self.maintainability_stats.trend_slope > 0.0 { "↗ Increasing" } else if self.maintainability_stats.trend_slope < 0.0 { "↘ Decreasing" } else { "→ Stable" })
    }
}
