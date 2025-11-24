//! Git integration for Lucent
//!
//! This module provides Git-based analysis features:
//! - Diff mode: Analyze only changed files
//! - Commit-by-commit analysis: Track metrics across commits
//! - Trend tracking: Visualize metric changes over time
//! - Blame integration: Show who contributed to complexity

pub mod diff;
pub mod commits;
pub mod trends;
pub mod blame;
pub mod repository;

pub use diff::DiffAnalyzer;
pub use commits::CommitAnalyzer;
pub use trends::TrendTracker;
pub use blame::BlameAnalyzer;
pub use repository::GitRepository;

use anyhow::Result;
use std::path::Path;

/// Git integration status
#[derive(Debug, Clone)]
pub struct GitInfo {
    /// Current commit hash
    pub commit: Option<String>,

    /// Current branch name
    pub branch: Option<String>,

    /// Repository root path
    pub root: Option<String>,

    /// Whether repository has uncommitted changes
    pub dirty: bool,
}

/// Check if a directory is a Git repository
pub fn is_git_repo(path: &Path) -> bool {
    path.join(".git").exists()
}

/// Get Git information for the current repository
pub fn get_git_info(path: &Path) -> Result<Option<GitInfo>> {
    if !is_git_repo(path) {
        return Ok(None);
    }

    let repo = GitRepository::open(path)?;

    Ok(Some(GitInfo {
        commit: repo.current_commit()?,
        branch: repo.current_branch()?,
        root: Some(repo.root_path().to_string_lossy().to_string()),
        dirty: repo.is_dirty()?,
    }))
}
