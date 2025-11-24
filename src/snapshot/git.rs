//! Git integration for snapshot metadata

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Git repository information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitInfo {
    /// Current commit hash (short)
    pub commit_hash: String,

    /// Current branch name
    pub branch: String,

    /// Commit author name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Commit message (first line)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,

    /// Whether there are uncommitted changes
    pub dirty: bool,
}

impl GitInfo {
    /// Extract git information from current repository
    pub fn from_current_repo() -> Result<Self> {
        // Check if we're in a git repo
        let is_git_repo = Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        if !is_git_repo {
            anyhow::bail!("Not a git repository");
        }

        let commit_hash = Self::get_commit_hash()?;
        let branch = Self::get_branch()?;
        let author = Self::get_author().ok();
        let commit_message = Self::get_commit_message().ok();
        let dirty = Self::is_dirty()?;

        Ok(Self {
            commit_hash,
            branch,
            author,
            commit_message,
            dirty,
        })
    }

    /// Get current commit hash (short form)
    fn get_commit_hash() -> Result<String> {
        let output = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .context("Failed to get git commit hash")?;

        if !output.status.success() {
            anyhow::bail!("Git command failed");
        }

        let hash = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 in git output")?
            .trim()
            .to_string();

        Ok(hash)
    }

    /// Get current branch name
    fn get_branch() -> Result<String> {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .context("Failed to get git branch")?;

        if !output.status.success() {
            anyhow::bail!("Git command failed");
        }

        let branch = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 in git output")?
            .trim()
            .to_string();

        Ok(branch)
    }

    /// Get commit author name
    fn get_author() -> Result<String> {
        let output = Command::new("git")
            .args(["log", "-1", "--format=%an"])
            .output()
            .context("Failed to get git author")?;

        if !output.status.success() {
            anyhow::bail!("Git command failed");
        }

        let author = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 in git output")?
            .trim()
            .to_string();

        Ok(author)
    }

    /// Get commit message (first line)
    fn get_commit_message() -> Result<String> {
        let output = Command::new("git")
            .args(["log", "-1", "--format=%s"])
            .output()
            .context("Failed to get git commit message")?;

        if !output.status.success() {
            anyhow::bail!("Git command failed");
        }

        let message = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 in git output")?
            .trim()
            .to_string();

        Ok(message)
    }

    /// Check if repository has uncommitted changes
    fn is_dirty() -> Result<bool> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .context("Failed to check git status")?;

        if !output.status.success() {
            anyhow::bail!("Git command failed");
        }

        // If output is not empty, there are uncommitted changes
        Ok(!output.stdout.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_info_from_current_repo() {
        // This test will only pass in a git repository
        match GitInfo::from_current_repo() {
            Ok(info) => {
                assert!(!info.commit_hash.is_empty());
                assert!(!info.branch.is_empty());
            }
            Err(_) => {
                // Not in a git repo, which is fine for tests
            }
        }
    }
}
