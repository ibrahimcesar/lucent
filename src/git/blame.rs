//! Git blame integration for complexity attribution
//!
//! Show which developers contributed to code complexity by integrating
//! Git blame information with complexity metrics.

use super::repository::{GitRepository, BlameLine};
use crate::metrics::FileMetrics;
use crate::language::Language;
use anyhow::{Context, Result};
use std::path::Path;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Analyzer for Git blame information
pub struct BlameAnalyzer {
    repo: GitRepository,
}

impl BlameAnalyzer {
    /// Create a new blame analyzer
    pub fn new(path: &Path) -> Result<Self> {
        let repo = GitRepository::open(path)?;
        Ok(Self { repo })
    }

    /// Analyze a file with blame information
    pub fn analyze_file(&self, file_path: &Path) -> Result<BlameAnalysis> {
        // Get blame information
        let blame_lines = self.repo.blame(file_path)
            .context("Failed to get blame information")?;

        // Get file metrics
        let lang = Language::from_path(file_path)
            .context("Unknown language for file")?;

        let full_path = self.repo.root_path().join(file_path);
        let metrics = FileMetrics::analyze(&full_path, lang)?;

        // Map contributors to their contributions
        let mut contributions: HashMap<String, AuthorContribution> = HashMap::new();

        for blame_line in &blame_lines {
            let contribution = contributions.entry(blame_line.author.clone())
                .or_insert_with(|| AuthorContribution {
                    author: blame_line.author.clone(),
                    lines: 0,
                    commits: std::collections::HashSet::new(),
                });

            contribution.lines += 1;
            contribution.commits.insert(blame_line.commit_hash.clone());
        }

        // Convert HashSet to Vec for serialization
        let mut author_contributions: Vec<AuthorContributionSummary> = contributions.values()
            .map(|c| AuthorContributionSummary {
                author: c.author.clone(),
                lines: c.lines,
                commit_count: c.commits.len(),
                percentage: (c.lines as f64 / blame_lines.len() as f64) * 100.0,
            })
            .collect();

        // Sort by line count descending
        author_contributions.sort_by(|a, b| b.lines.cmp(&a.lines));

        Ok(BlameAnalysis {
            file_path: file_path.to_path_buf(),
            metrics,
            total_lines: blame_lines.len(),
            contributors: author_contributions,
            blame_lines,
        })
    }

    /// Analyze complexity attribution across all files
    pub fn analyze_project(&self, paths: &[&Path]) -> Result<ProjectBlameAnalysis> {
        let mut file_analyses = Vec::new();
        let mut total_contributors: HashMap<String, ProjectContribution> = HashMap::new();

        for path in paths {
            match self.analyze_file(path) {
                Ok(analysis) => {
                    // Aggregate contributor statistics
                    for contributor in &analysis.contributors {
                        let total_contrib = total_contributors.entry(contributor.author.clone())
                            .or_insert_with(|| ProjectContribution {
                                author: contributor.author.clone(),
                                total_lines: 0,
                                files_contributed: 0,
                                complexity_contribution: 0.0,
                            });

                        total_contrib.total_lines += contributor.lines;
                        total_contrib.files_contributed += 1;

                        // Weight complexity by percentage of contribution
                        let complexity_share = analysis.metrics.cyclomatic_complexity as f64
                            * (contributor.percentage / 100.0);
                        total_contrib.complexity_contribution += complexity_share;
                    }

                    file_analyses.push(analysis);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to analyze {}: {}", path.display(), e);
                }
            }
        }

        // Sort by complexity contribution
        let mut contributors: Vec<ProjectContribution> = total_contributors.into_values().collect();
        contributors.sort_by(|a, b| b.complexity_contribution.partial_cmp(&a.complexity_contribution).unwrap());

        Ok(ProjectBlameAnalysis {
            file_analyses,
            contributors,
        })
    }

    /// Get the repository
    pub fn repository(&self) -> &GitRepository {
        &self.repo
    }
}

/// Internal structure for tracking contributions
struct AuthorContribution {
    author: String,
    lines: usize,
    commits: std::collections::HashSet<String>,
}

/// Blame analysis for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlameAnalysis {
    /// Path to the file
    pub file_path: std::path::PathBuf,

    /// File metrics
    pub metrics: FileMetrics,

    /// Total lines in file
    pub total_lines: usize,

    /// Contributors to this file
    pub contributors: Vec<AuthorContributionSummary>,

    /// Blame information for each line
    #[serde(skip)]
    pub blame_lines: Vec<BlameLine>,
}

/// Summary of an author's contribution to a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorContributionSummary {
    /// Author name
    pub author: String,

    /// Number of lines contributed
    pub lines: usize,

    /// Number of commits from this author
    pub commit_count: usize,

    /// Percentage of file
    pub percentage: f64,
}

/// Project-wide blame analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectBlameAnalysis {
    /// Per-file analyses
    pub file_analyses: Vec<BlameAnalysis>,

    /// Project-wide contributor statistics
    pub contributors: Vec<ProjectContribution>,
}

/// Project-level contribution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContribution {
    /// Author name
    pub author: String,

    /// Total lines across all files
    pub total_lines: usize,

    /// Number of files contributed to
    pub files_contributed: usize,

    /// Weighted complexity contribution
    /// (sum of complexity * contribution percentage across files)
    pub complexity_contribution: f64,
}

impl std::fmt::Display for BlameAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "File: {}", self.file_path.display())?;
        writeln!(f, "Complexity: {}", self.metrics.cyclomatic_complexity)?;
        writeln!(f, "Total Lines: {}", self.total_lines)?;
        writeln!(f)?;
        writeln!(f, "Contributors:")?;

        for contributor in &self.contributors {
            writeln!(
                f,
                "  {} - {} lines ({:.1}%) - {} commits",
                contributor.author,
                contributor.lines,
                contributor.percentage,
                contributor.commit_count
            )?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ProjectBlameAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "📊 Project Complexity Attribution")?;
        writeln!(f, "Files Analyzed: {}", self.file_analyses.len())?;
        writeln!(f)?;
        writeln!(f, "Top Contributors by Complexity:")?;

        for (i, contributor) in self.contributors.iter().take(10).enumerate() {
            writeln!(
                f,
                "{}. {} - {:.1} complexity units ({} lines, {} files)",
                i + 1,
                contributor.author,
                contributor.complexity_contribution,
                contributor.total_lines,
                contributor.files_contributed
            )?;
        }

        Ok(())
    }
}
