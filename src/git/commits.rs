//! Commit-by-commit analysis
//!
//! Track code metrics across Git commit history to understand
//! how code quality has evolved over time.

use super::repository::{GitRepository, CommitInfo};
use crate::metrics::{ProjectMetrics, FileMetrics};
use crate::scanner::Scanner;
use anyhow::Result;
use std::path::Path;
use serde::{Serialize, Deserialize};

/// Analyzer for commit history
pub struct CommitAnalyzer {
    repo: GitRepository,
}

impl CommitAnalyzer {
    /// Create a new commit analyzer
    pub fn new(path: &Path) -> Result<Self> {
        let repo = GitRepository::open(path)?;
        Ok(Self { repo })
    }

    /// Analyze metrics across commit history
    ///
    /// # Arguments
    /// * `limit` - Maximum number of commits to analyze
    /// * `skip` - Number of commits to skip between analyses (for performance)
    pub fn analyze_history(&self, limit: usize, skip: usize) -> Result<CommitHistory> {
        let commits = self.repo.commit_history(limit * (skip + 1))?;

        let mut analyses = Vec::new();

        for (i, commit) in commits.iter().enumerate() {
            // Skip commits based on skip parameter
            if i % (skip + 1) != 0 {
                continue;
            }

            println!("Analyzing commit {} ({}/{})...", commit.short_hash, i + 1, commits.len());

            match self.analyze_commit(&commit.hash) {
                Ok(metrics) => {
                    analyses.push(CommitAnalysis {
                        commit: commit.clone(),
                        metrics,
                    });
                }
                Err(e) => {
                    eprintln!("Warning: Failed to analyze commit {}: {}", commit.short_hash, e);
                }
            }
        }

        Ok(CommitHistory { analyses })
    }

    /// Analyze metrics for a specific commit
    fn analyze_commit(&self, _commit_hash: &str) -> Result<ProjectMetrics> {
        // For now, we analyze the current working tree state
        // TODO: Implement proper commit checkout or tree-based analysis using git worktree

        let root = self.repo.root_path();
        let scanner = Scanner::new(root)?;
        let files = scanner.scan()?;

        let mut project_metrics = ProjectMetrics::new();
        for source_file in &files {
            if let Ok(metrics) = FileMetrics::analyze(&source_file.path, source_file.language) {
                project_metrics.add_file(source_file.language.name, &metrics);
            }
        }

        Ok(project_metrics)
    }

    /// Get the repository
    pub fn repository(&self) -> &GitRepository {
        &self.repo
    }
}

/// History of commit analyses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitHistory {
    /// List of commit analyses
    pub analyses: Vec<CommitAnalysis>,
}

impl CommitHistory {
    /// Calculate deltas between consecutive commits
    pub fn deltas(&self) -> Vec<CommitDelta> {
        let mut deltas = Vec::new();

        for i in 1..self.analyses.len() {
            let previous = &self.analyses[i];
            let current = &self.analyses[i - 1];

            deltas.push(CommitDelta {
                from_commit: previous.commit.short_hash.clone(),
                to_commit: current.commit.short_hash.clone(),
                files_delta: current.metrics.total_files as i32 - previous.metrics.total_files as i32,
                lines_delta: current.metrics.total_lines() as i32 - previous.metrics.total_lines() as i32,
                complexity_delta: current.metrics.avg_complexity() - previous.metrics.avg_complexity(),
                maintainability_delta: current.metrics.avg_maintainability() - previous.metrics.avg_maintainability(),
            });
        }

        deltas
    }

    /// Get trend direction (improving, stable, degrading)
    pub fn trend(&self) -> Trend {
        if self.analyses.len() < 2 {
            return Trend::Stable;
        }

        let deltas = self.deltas();

        // Look at complexity and maintainability trends
        let complexity_trend: f64 = deltas.iter()
            .map(|d| d.complexity_delta)
            .sum::<f64>() / deltas.len() as f64;

        let maintainability_trend: f64 = deltas.iter()
            .map(|d| d.maintainability_delta)
            .sum::<f64>() / deltas.len() as f64;

        // Complexity increasing is bad, maintainability increasing is good
        let overall_trend = maintainability_trend - complexity_trend;

        if overall_trend > 1.0 {
            Trend::Improving
        } else if overall_trend < -1.0 {
            Trend::Degrading
        } else {
            Trend::Stable
        }
    }
}

/// Analysis of a single commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitAnalysis {
    /// Commit information
    pub commit: CommitInfo,

    /// Metrics at this commit
    pub metrics: ProjectMetrics,
}

/// Delta between two commits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitDelta {
    /// From commit (earlier)
    pub from_commit: String,

    /// To commit (later)
    pub to_commit: String,

    /// Change in number of files
    pub files_delta: i32,

    /// Change in total lines
    pub lines_delta: i32,

    /// Change in average complexity
    pub complexity_delta: f64,

    /// Change in average maintainability
    pub maintainability_delta: f64,
}

/// Overall trend direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trend {
    /// Code quality is improving
    Improving,

    /// Code quality is stable
    Stable,

    /// Code quality is degrading
    Degrading,
}

impl std::fmt::Display for Trend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Trend::Improving => write!(f, "✅ Improving"),
            Trend::Stable => write!(f, "➡️  Stable"),
            Trend::Degrading => write!(f, "❌ Degrading"),
        }
    }
}
