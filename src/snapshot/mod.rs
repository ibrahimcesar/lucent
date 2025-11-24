//! Snapshot and history tracking for Lucent
//!
//! This module handles saving, loading, and comparing code quality metrics over time.

mod storage;
mod compare;
mod git;

pub use storage::SnapshotManager;
pub use compare::{SnapshotComparison, MetricDelta, TrendDirection};
pub use git::GitInfo;

use crate::metrics::ProjectMetrics;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Complete snapshot including metrics and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Unique snapshot ID (timestamp-based)
    pub id: String,

    /// When the snapshot was created
    pub timestamp: DateTime<Utc>,

    /// Git information (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git: Option<GitInfo>,

    /// Project metrics at this point in time
    pub metrics: ProjectMetrics,

    /// Optional user-provided description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Metadata about a snapshot (lightweight, for listing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub git_commit: Option<String>,
    pub git_branch: Option<String>,
    pub description: Option<String>,
    pub total_files: usize,
    pub total_lines: usize,
    pub avg_complexity: f64,
    pub avg_maintainability: f64,
}

impl Snapshot {
    /// Create a new snapshot from current metrics
    pub fn new(metrics: ProjectMetrics, description: Option<String>) -> Result<Self> {
        let timestamp = Utc::now();
        let id = Self::generate_id(&timestamp);
        let git = GitInfo::from_current_repo().ok();

        Ok(Self {
            id,
            timestamp,
            git,
            metrics,
            description,
        })
    }

    /// Generate a unique ID from timestamp
    fn generate_id(timestamp: &DateTime<Utc>) -> String {
        // Include microseconds to ensure uniqueness
        timestamp.format("%Y%m%d_%H%M%S_%6f").to_string()
    }

    /// Get lightweight metadata for this snapshot
    pub fn metadata(&self) -> SnapshotMetadata {
        let total_files = self.metrics.by_language.values()
            .map(|lang| lang.file_count)
            .sum();

        let total_lines = self.metrics.by_language.values()
            .map(|lang| lang.total_lines)
            .sum();

        let avg_complexity = if !self.metrics.by_language.is_empty() {
            self.metrics.by_language.values()
                .map(|lang| lang.avg_cyclomatic)
                .sum::<f64>() / self.metrics.by_language.len() as f64
        } else {
            0.0
        };

        let avg_maintainability = if !self.metrics.by_language.is_empty() {
            self.metrics.by_language.values()
                .map(|lang| lang.avg_maintainability_index)
                .sum::<f64>() / self.metrics.by_language.len() as f64
        } else {
            0.0
        };

        SnapshotMetadata {
            id: self.id.clone(),
            timestamp: self.timestamp,
            git_commit: self.git.as_ref().map(|g| g.commit_hash.clone()),
            git_branch: self.git.as_ref().map(|g| g.branch.clone()),
            description: self.description.clone(),
            total_files,
            total_lines,
            avg_complexity,
            avg_maintainability,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_snapshot_creation() {
        let metrics = ProjectMetrics {
            total_files: 0,
            by_language: HashMap::new(),
        };

        let snapshot = Snapshot::new(metrics, Some("Test snapshot".to_string())).unwrap();
        assert!(!snapshot.id.is_empty());
        assert_eq!(snapshot.description, Some("Test snapshot".to_string()));
    }

    #[test]
    fn test_snapshot_metadata() {
        let metrics = ProjectMetrics {
            total_files: 0,
            by_language: HashMap::new(),
        };

        let snapshot = Snapshot::new(metrics, None).unwrap();
        let metadata = snapshot.metadata();

        assert_eq!(metadata.id, snapshot.id);
        assert_eq!(metadata.total_files, 0);
        assert_eq!(metadata.total_lines, 0);
    }
}
