//! Diff mode - Analyze only changed files
//!
//! This module provides functionality to analyze only files that have changed
//! between Git revisions or in the working directory.

use super::repository::GitRepository;
use crate::metrics::FileMetrics;
use crate::language::Language;
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Diff analysis modes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffMode {
    /// Compare working directory with HEAD
    WorkingDirectory,

    /// Compare two commits/branches
    Commits { from: String, to: String },

    /// Compare with a specific commit/branch
    Since(String),

    /// Compare with main/master branch
    MainBranch,
}

/// Result of diff analysis
#[derive(Debug, Clone)]
pub struct DiffAnalysis {
    /// Mode used for analysis
    pub mode: DiffMode,

    /// Files that were changed
    pub changed_files: Vec<PathBuf>,

    /// Files that were analyzed (subset of changed_files)
    pub analyzed_files: Vec<PathBuf>,

    /// Metrics for analyzed files
    pub metrics: Vec<(PathBuf, FileMetrics)>,
}

/// Analyzer for Git diffs
pub struct DiffAnalyzer {
    repo: GitRepository,
}

impl DiffAnalyzer {
    /// Create a new diff analyzer for the given repository
    pub fn new(path: &Path) -> Result<Self> {
        let repo = GitRepository::open(path)?;
        Ok(Self { repo })
    }

    /// Analyze changed files in the given mode
    pub fn analyze(&self, mode: DiffMode) -> Result<DiffAnalysis> {
        let changed_files = self.get_changed_files(&mode)?;

        // Analyze each file
        let mut metrics = Vec::new();
        let mut analyzed_files = Vec::new();

        for file_path in &changed_files {
            let full_path = self.repo.root_path().join(file_path);

            if let Some(lang) = Language::from_path(&full_path) {
                analyzed_files.push(file_path.clone());
                match FileMetrics::analyze(&full_path, lang) {
                    Ok(file_metrics) => {
                        metrics.push((file_path.clone(), file_metrics));
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to analyze {}: {}", file_path.display(), e);
                    }
                }
            }
        }

        Ok(DiffAnalysis {
            mode,
            changed_files,
            analyzed_files,
            metrics,
        })
    }

    /// Get list of changed files based on diff mode
    fn get_changed_files(&self, mode: &DiffMode) -> Result<Vec<PathBuf>> {
        match mode {
            DiffMode::WorkingDirectory => {
                self.repo.uncommitted_changes()
            }

            DiffMode::Commits { from, to } => {
                self.repo.changed_files(from, to)
            }

            DiffMode::Since(reference) => {
                let current = self.repo.current_commit()?
                    .unwrap_or_else(|| "HEAD".to_string());
                self.repo.changed_files(reference, &current)
            }

            DiffMode::MainBranch => {
                // Try to find main branch (main or master)
                let main_branch = self.find_main_branch()?;
                let current = self.repo.current_commit()?
                    .unwrap_or_else(|| "HEAD".to_string());
                self.repo.changed_files(&main_branch, &current)
            }
        }
    }

    /// Find the main branch (main or master)
    fn find_main_branch(&self) -> Result<String> {
        let repo = self.repo.inner();

        // Check for 'main' branch
        if repo.find_branch("main", git2::BranchType::Local).is_ok() {
            return Ok("main".to_string());
        }

        // Check for 'master' branch
        if repo.find_branch("master", git2::BranchType::Local).is_ok() {
            return Ok("master".to_string());
        }

        // Check remote branches
        if repo.find_branch("origin/main", git2::BranchType::Remote).is_ok() {
            return Ok("origin/main".to_string());
        }

        if repo.find_branch("origin/master", git2::BranchType::Remote).is_ok() {
            return Ok("origin/master".to_string());
        }

        anyhow::bail!("Could not find main branch (tried: main, master, origin/main, origin/master)")
    }

    /// Get summary statistics for the diff
    pub fn summary(&self, analysis: &DiffAnalysis) -> DiffSummary {
        let total_lines: usize = analysis.metrics.iter()
            .map(|(_, m)| m.total_lines)
            .sum();

        let total_code_lines: usize = analysis.metrics.iter()
            .map(|(_, m)| m.code_lines)
            .sum();

        let avg_complexity = if !analysis.metrics.is_empty() {
            analysis.metrics.iter()
                .map(|(_, m)| m.cyclomatic_complexity as f64)
                .sum::<f64>() / analysis.metrics.len() as f64
        } else {
            0.0
        };

        DiffSummary {
            total_changed_files: analysis.changed_files.len(),
            analyzed_files: analysis.analyzed_files.len(),
            total_lines,
            total_code_lines,
            avg_complexity,
        }
    }
}

/// Summary of diff analysis
#[derive(Debug, Clone)]
pub struct DiffSummary {
    /// Total number of changed files (including non-source)
    pub total_changed_files: usize,

    /// Number of source files analyzed
    pub analyzed_files: usize,

    /// Total lines in analyzed files
    pub total_lines: usize,

    /// Total code lines in analyzed files
    pub total_code_lines: usize,

    /// Average cyclomatic complexity
    pub avg_complexity: f64,
}

impl std::fmt::Display for DiffSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Diff Analysis Summary:")?;
        writeln!(f, "  Changed Files: {}", self.total_changed_files)?;
        writeln!(f, "  Analyzed Files: {}", self.analyzed_files)?;
        writeln!(f, "  Total Lines: {}", self.total_lines)?;
        writeln!(f, "  Code Lines: {}", self.total_code_lines)?;
        write!(f, "  Avg Complexity: {:.1}", self.avg_complexity)
    }
}
