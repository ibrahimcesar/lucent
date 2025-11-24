//! Git repository operations
//!
//! Provides a wrapper around Git operations using the git2 crate.

use anyhow::{Context, Result};
use git2::{Repository, Oid, StatusOptions};
use std::path::{Path, PathBuf};

/// Wrapper around git2::Repository with Lucent-specific operations
pub struct GitRepository {
    repo: Repository,
}

impl GitRepository {
    /// Open a Git repository at the given path
    pub fn open(path: &Path) -> Result<Self> {
        let repo = Repository::discover(path)
            .context("Failed to open Git repository")?;

        Ok(Self { repo })
    }

    /// Get the repository root path
    pub fn root_path(&self) -> &Path {
        self.repo.workdir().unwrap_or_else(|| Path::new("."))
    }

    /// Get the current commit hash (HEAD)
    pub fn current_commit(&self) -> Result<Option<String>> {
        match self.repo.head() {
            Ok(head) => {
                let commit = head.peel_to_commit()
                    .context("Failed to get HEAD commit")?;
                Ok(Some(commit.id().to_string()))
            }
            Err(_) => Ok(None), // No commits yet
        }
    }

    /// Get the current branch name
    pub fn current_branch(&self) -> Result<Option<String>> {
        match self.repo.head() {
            Ok(head) => {
                if head.is_branch() {
                    let shorthand = head.shorthand()
                        .map(|s| s.to_string());
                    Ok(shorthand)
                } else {
                    Ok(None) // Detached HEAD
                }
            }
            Err(_) => Ok(None),
        }
    }

    /// Check if the repository has uncommitted changes
    pub fn is_dirty(&self) -> Result<bool> {
        let statuses = self.repo.statuses(None)
            .context("Failed to get repository status")?;

        Ok(!statuses.is_empty())
    }

    /// Get list of changed files between two commits
    pub fn changed_files(&self, from: &str, to: &str) -> Result<Vec<PathBuf>> {
        let from_oid = self.parse_oid(from)?;
        let to_oid = self.parse_oid(to)?;

        let from_tree = self.repo.find_commit(from_oid)?
            .tree()
            .context("Failed to get tree for 'from' commit")?;

        let to_tree = self.repo.find_commit(to_oid)?
            .tree()
            .context("Failed to get tree for 'to' commit")?;

        let diff = self.repo.diff_tree_to_tree(
            Some(&from_tree),
            Some(&to_tree),
            None,
        )?;

        let mut files = Vec::new();
        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    files.push(path.to_path_buf());
                }
                true
            },
            None,
            None,
            None,
        )?;

        Ok(files)
    }

    /// Get list of changed files in working directory (uncommitted)
    pub fn uncommitted_changes(&self) -> Result<Vec<PathBuf>> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);

        let statuses = self.repo.statuses(Some(&mut opts))?;

        let mut files = Vec::new();
        for entry in statuses.iter() {
            if let Some(path) = entry.path() {
                files.push(PathBuf::from(path));
            }
        }

        Ok(files)
    }

    /// Get commit history
    pub fn commit_history(&self, limit: usize) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        let mut commits = Vec::new();
        for (i, oid) in revwalk.enumerate() {
            if i >= limit {
                break;
            }

            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;

            commits.push(CommitInfo {
                hash: commit.id().to_string(),
                short_hash: format!("{:.7}", commit.id()),
                message: commit.message().unwrap_or("").to_string(),
                author: commit.author().name().unwrap_or("Unknown").to_string(),
                timestamp: commit.time().seconds(),
            });
        }

        Ok(commits)
    }

    /// Get file blame information
    pub fn blame(&self, path: &Path) -> Result<Vec<BlameLine>> {
        let blame = self.repo.blame_file(path, None)
            .context("Failed to get blame information")?;

        let mut lines = Vec::new();
        for hunk in blame.iter() {
            let commit = self.repo.find_commit(hunk.final_commit_id())?;

            for line_num in hunk.final_start_line()..hunk.final_start_line() + hunk.lines_in_hunk() {
                lines.push(BlameLine {
                    line_number: line_num,
                    commit_hash: commit.id().to_string(),
                    author: commit.author().name().unwrap_or("Unknown").to_string(),
                    timestamp: commit.time().seconds(),
                });
            }
        }

        Ok(lines)
    }

    /// Parse a Git reference (commit, branch, tag) to an Oid
    fn parse_oid(&self, reference: &str) -> Result<Oid> {
        if let Ok(oid) = Oid::from_str(reference) {
            return Ok(oid);
        }

        let obj = self.repo.revparse_single(reference)
            .with_context(|| format!("Failed to parse Git reference: {}", reference))?;

        Ok(obj.id())
    }

    /// Get the inner git2::Repository
    pub fn inner(&self) -> &Repository {
        &self.repo
    }
}

/// Information about a Git commit
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommitInfo {
    /// Full commit hash
    pub hash: String,

    /// Short commit hash (7 chars)
    pub short_hash: String,

    /// Commit message
    pub message: String,

    /// Author name
    pub author: String,

    /// Unix timestamp
    pub timestamp: i64,
}

/// Information about a line from git blame
#[derive(Debug, Clone)]
pub struct BlameLine {
    /// Line number (1-indexed)
    pub line_number: usize,

    /// Commit hash that last modified this line
    pub commit_hash: String,

    /// Author of the commit
    pub author: String,

    /// Unix timestamp of the commit
    pub timestamp: i64,
}
