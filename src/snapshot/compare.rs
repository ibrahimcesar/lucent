//! Snapshot comparison and trend analysis

use super::Snapshot;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Comparison result between two snapshots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotComparison {
    /// Base (older) snapshot ID
    pub base_id: String,

    /// Current (newer) snapshot ID
    pub current_id: String,

    /// Overall metrics delta
    pub overall: OverallDelta,

    /// Per-language deltas
    pub languages: HashMap<String, LanguageDelta>,

    /// Quality score (0-100)
    pub quality_score: f64,
}

/// Overall project-level deltas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallDelta {
    pub files: MetricDelta<usize>,
    pub lines: MetricDelta<usize>,
    pub code_lines: MetricDelta<usize>,
    pub avg_complexity: MetricDelta<f64>,
    pub avg_nesting: MetricDelta<f64>,
    pub avg_maintainability: MetricDelta<f64>,
    pub code_smells: MetricDelta<usize>,
}

/// Per-language deltas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDelta {
    pub files: MetricDelta<usize>,
    pub lines: MetricDelta<usize>,
    pub complexity: MetricDelta<f64>,
    pub nesting: MetricDelta<f64>,
    pub maintainability: MetricDelta<f64>,
}

/// Delta for a single metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDelta<T> {
    /// Value in base snapshot
    pub base: T,

    /// Value in current snapshot
    pub current: T,

    /// Absolute change
    pub delta: T,

    /// Percentage change
    pub percent_change: f64,

    /// Trend direction
    pub trend: TrendDirection,
}

/// Trend direction for a metric
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendDirection {
    /// Metric improved
    Improved,

    /// Metric stayed the same (within threshold)
    Neutral,

    /// Metric regressed
    Regressed,
}

impl TrendDirection {
    /// Get emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            TrendDirection::Improved => "✅",
            TrendDirection::Neutral => "➡️",
            TrendDirection::Regressed => "❌",
        }
    }

    /// Get color name for terminal output
    pub fn color(&self) -> &'static str {
        match self {
            TrendDirection::Improved => "green",
            TrendDirection::Neutral => "yellow",
            TrendDirection::Regressed => "red",
        }
    }
}

impl SnapshotComparison {
    /// Compare two snapshots and generate comparison report
    pub fn compare(base: &Snapshot, current: &Snapshot) -> Self {
        let overall = Self::compare_overall(base, current);
        let languages = Self::compare_languages(base, current);
        let quality_score = Self::calculate_quality_score(&overall);

        Self {
            base_id: base.id.clone(),
            current_id: current.id.clone(),
            overall,
            languages,
            quality_score,
        }
    }

    /// Compare overall project metrics
    fn compare_overall(base: &Snapshot, current: &Snapshot) -> OverallDelta {
        // Calculate totals for base
        let base_files: usize = base.metrics.by_language.values().map(|l| l.file_count).sum();
        let base_lines: usize = base.metrics.by_language.values().map(|l| l.total_lines).sum();
        let base_code: usize = base.metrics.by_language.values().map(|l| l.code_lines).sum();
        let base_smells: usize = base.metrics.by_language.values()
            .map(|l| l.long_functions + l.long_parameter_lists +
                     l.deep_nesting_instances + l.magic_numbers)
            .sum();

        let base_avg_complexity = if !base.metrics.by_language.is_empty() {
            base.metrics.by_language.values().map(|l| l.avg_cyclomatic).sum::<f64>()
                / base.metrics.by_language.len() as f64
        } else {
            0.0
        };

        let base_avg_nesting = if !base.metrics.by_language.is_empty() {
            base.metrics.by_language.values().map(|l| l.avg_nesting_depth).sum::<f64>()
                / base.metrics.by_language.len() as f64
        } else {
            0.0
        };

        let base_avg_mi = if !base.metrics.by_language.is_empty() {
            base.metrics.by_language.values().map(|l| l.avg_maintainability_index).sum::<f64>()
                / base.metrics.by_language.len() as f64
        } else {
            0.0
        };

        // Calculate totals for current
        let curr_files: usize = current.metrics.by_language.values().map(|l| l.file_count).sum();
        let curr_lines: usize = current.metrics.by_language.values().map(|l| l.total_lines).sum();
        let curr_code: usize = current.metrics.by_language.values().map(|l| l.code_lines).sum();
        let curr_smells: usize = current.metrics.by_language.values()
            .map(|l| l.long_functions + l.long_parameter_lists +
                     l.deep_nesting_instances + l.magic_numbers)
            .sum();

        let curr_avg_complexity = if !current.metrics.by_language.is_empty() {
            current.metrics.by_language.values().map(|l| l.avg_cyclomatic).sum::<f64>()
                / current.metrics.by_language.len() as f64
        } else {
            0.0
        };

        let curr_avg_nesting = if !current.metrics.by_language.is_empty() {
            current.metrics.by_language.values().map(|l| l.avg_nesting_depth).sum::<f64>()
                / current.metrics.by_language.len() as f64
        } else {
            0.0
        };

        let curr_avg_mi = if !current.metrics.by_language.is_empty() {
            current.metrics.by_language.values().map(|l| l.avg_maintainability_index).sum::<f64>()
                / current.metrics.by_language.len() as f64
        } else {
            0.0
        };

        OverallDelta {
            files: MetricDelta::new_usize(base_files, curr_files, false),
            lines: MetricDelta::new_usize(base_lines, curr_lines, false),
            code_lines: MetricDelta::new_usize(base_code, curr_code, false),
            avg_complexity: MetricDelta::new_f64(base_avg_complexity, curr_avg_complexity, true),
            avg_nesting: MetricDelta::new_f64(base_avg_nesting, curr_avg_nesting, true),
            avg_maintainability: MetricDelta::new_f64(base_avg_mi, curr_avg_mi, false),
            code_smells: MetricDelta::new_usize(base_smells, curr_smells, true),
        }
    }

    /// Compare per-language metrics
    fn compare_languages(base: &Snapshot, current: &Snapshot) -> HashMap<String, LanguageDelta> {
        let mut deltas = HashMap::new();

        // Get all unique languages from both snapshots
        let mut languages: Vec<String> = base.metrics.by_language.keys()
            .chain(current.metrics.by_language.keys())
            .cloned()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        languages.sort();

        for lang in languages {
            let base_lang = base.metrics.by_language.get(&lang);
            let curr_lang = current.metrics.by_language.get(&lang);

            if let (Some(base_l), Some(curr_l)) = (base_lang, curr_lang) {
                deltas.insert(lang.clone(), LanguageDelta {
                    files: MetricDelta::new_usize(base_l.file_count, curr_l.file_count, false),
                    lines: MetricDelta::new_usize(base_l.total_lines, curr_l.total_lines, false),
                    complexity: MetricDelta::new_f64(
                        base_l.avg_cyclomatic,
                        curr_l.avg_cyclomatic,
                        true,
                    ),
                    nesting: MetricDelta::new_f64(
                        base_l.avg_nesting_depth,
                        curr_l.avg_nesting_depth,
                        true,
                    ),
                    maintainability: MetricDelta::new_f64(
                        base_l.avg_maintainability_index,
                        curr_l.avg_maintainability_index,
                        false,
                    ),
                });
            }
        }

        deltas
    }

    /// Calculate quality score based on deltas (0-100)
    fn calculate_quality_score(overall: &OverallDelta) -> f64 {
        let mut score = 100.0;

        // Penalize complexity increases
        if overall.avg_complexity.trend == TrendDirection::Regressed {
            score -= overall.avg_complexity.percent_change.abs().min(20.0);
        } else if overall.avg_complexity.trend == TrendDirection::Improved {
            score += overall.avg_complexity.percent_change.abs().min(10.0);
        }

        // Penalize nesting increases
        if overall.avg_nesting.trend == TrendDirection::Regressed {
            score -= overall.avg_nesting.percent_change.abs().min(15.0);
        } else if overall.avg_nesting.trend == TrendDirection::Improved {
            score += overall.avg_nesting.percent_change.abs().min(10.0);
        }

        // Reward maintainability improvements
        if overall.avg_maintainability.trend == TrendDirection::Improved {
            score += overall.avg_maintainability.percent_change.abs().min(15.0);
        } else if overall.avg_maintainability.trend == TrendDirection::Regressed {
            score -= overall.avg_maintainability.percent_change.abs().min(20.0);
        }

        // Penalize code smell increases
        if overall.code_smells.trend == TrendDirection::Regressed {
            score -= overall.code_smells.percent_change.abs().min(15.0);
        } else if overall.code_smells.trend == TrendDirection::Improved {
            score += overall.code_smells.percent_change.abs().min(10.0);
        }

        score.max(0.0).min(100.0)
    }

    /// Get quality score description
    pub fn quality_description(&self) -> &'static str {
        match self.quality_score {
            s if s >= 90.0 => "Excellent - Code quality improved",
            s if s >= 70.0 => "Good - Minor concerns",
            _ => "Needs improvement - Significant issues",
        }
    }
}

/// Determine trend direction based on percentage change
fn determine_trend(percent_change: f64, lower_is_better: bool) -> TrendDirection {
    const THRESHOLD: f64 = 0.1; // 0.1% threshold for "neutral"

    if percent_change.abs() < THRESHOLD {
        TrendDirection::Neutral
    } else if lower_is_better {
        if percent_change < 0.0 {
            TrendDirection::Improved
        } else {
            TrendDirection::Regressed
        }
    } else {
        if percent_change > 0.0 {
            TrendDirection::Improved
        } else {
            TrendDirection::Regressed
        }
    }
}

impl<T> MetricDelta<T>
where
    T: Copy + std::ops::Sub<Output = T> + Into<f64>,
{
    /// Create new metric delta for numeric types
    fn new(base: T, current: T, lower_is_better: bool) -> Self {
        let delta = current - base;
        let percent_change = if base.into() != 0.0 {
            (delta.into() / base.into()) * 100.0
        } else {
            0.0
        };

        let trend = determine_trend(percent_change, lower_is_better);

        Self {
            base,
            current,
            delta,
            percent_change,
            trend,
        }
    }
}

impl MetricDelta<usize> {
    /// Create new metric delta for usize types
    fn new_usize(base: usize, current: usize, lower_is_better: bool) -> Self {
        let delta = if current >= base {
            current - base
        } else {
            base - current
        };

        let percent_change = if base != 0 {
            if current >= base {
                ((current - base) as f64 / base as f64) * 100.0
            } else {
                -((base - current) as f64 / base as f64) * 100.0
            }
        } else if current != 0 {
            100.0
        } else {
            0.0
        };

        let trend = determine_trend(percent_change, lower_is_better);

        Self {
            base,
            current,
            delta,
            percent_change,
            trend,
        }
    }
}

impl MetricDelta<f64> {
    /// Create new metric delta for f64 types
    fn new_f64(base: f64, current: f64, lower_is_better: bool) -> Self {
        let delta = current - base;
        let percent_change = if base != 0.0 {
            (delta / base) * 100.0
        } else if current != 0.0 {
            100.0 // If base is 0 but current isn't, that's a 100% increase
        } else {
            0.0
        };

        let trend = determine_trend(percent_change, lower_is_better);

        Self {
            base,
            current,
            delta,
            percent_change,
            trend,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::{LanguageMetrics, ProjectMetrics};

    fn create_test_metrics(complexity: f64, mi: f64) -> ProjectMetrics {
        let mut by_language = HashMap::new();
        by_language.insert("Rust".to_string(), LanguageMetrics {
            file_count: 10,
            total_lines: 1000,
            code_lines: 800,
            comment_lines: 100,
            blank_lines: 100,
            avg_entropy: 3.5,
            avg_cyclomatic: complexity,
            avg_cognitive: complexity * 1.5,
            avg_nesting_depth: 2.0,
            total_functions: 40,
            avg_function_length: 20.0,
            todo_count: 0,
            fixme_count: 0,
            hack_count: 0,
            avg_comment_quality: 0.5,
            low_quality_comments: 2,
            high_quality_comments: 8,
            long_functions: 2,
            long_parameter_lists: 1,
            deep_nesting_instances: 1,
            magic_numbers: 5,
            avg_maintainability_index: mi,
        });

        ProjectMetrics {
            total_files: 10,
            by_language,
        }
    }

    #[test]
    fn test_metric_delta_improved() {
        let delta = MetricDelta::new_f64(10.0, 5.0, true); // Lower is better
        assert_eq!(delta.trend, TrendDirection::Improved);
        assert_eq!(delta.delta, -5.0);
    }

    #[test]
    fn test_metric_delta_regressed() {
        let delta = MetricDelta::new_f64(5.0, 10.0, true); // Lower is better
        assert_eq!(delta.trend, TrendDirection::Regressed);
        assert_eq!(delta.delta, 5.0);
    }

    #[test]
    fn test_snapshot_comparison() {
        let base_metrics = create_test_metrics(10.0, 70.0);
        let curr_metrics = create_test_metrics(8.0, 75.0);

        let base = Snapshot::new(base_metrics, None).unwrap();
        let current = Snapshot::new(curr_metrics, None).unwrap();

        let comparison = SnapshotComparison::compare(&base, &current);

        assert_eq!(comparison.overall.avg_complexity.trend, TrendDirection::Improved);
        assert_eq!(comparison.overall.avg_maintainability.trend, TrendDirection::Improved);
        assert!(comparison.quality_score > 90.0);
    }
}
