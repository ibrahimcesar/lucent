//! Application state and logic for the TUI

use crate::{FileMetrics, ProjectMetrics, Scanner};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use notify::RecommendedWatcher;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Result type for app operations
pub type AppResult<T> = Result<T>;

/// Sort order for file list
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    Name,
    Complexity,
    CognitiveComplexity,
    Nesting,
    Lines,
    Maintainability,
    Entropy,
}

impl SortBy {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            SortBy::Name => "Name",
            SortBy::Complexity => "Complexity",
            SortBy::CognitiveComplexity => "Cognitive",
            SortBy::Nesting => "Nesting",
            SortBy::Lines => "Lines",
            SortBy::Maintainability => "MI",
            SortBy::Entropy => "Entropy",
        }
    }

    /// Cycle to next sort option
    pub fn next(&self) -> Self {
        match self {
            SortBy::Name => SortBy::Complexity,
            SortBy::Complexity => SortBy::CognitiveComplexity,
            SortBy::CognitiveComplexity => SortBy::Nesting,
            SortBy::Nesting => SortBy::Lines,
            SortBy::Lines => SortBy::Maintainability,
            SortBy::Maintainability => SortBy::Entropy,
            SortBy::Entropy => SortBy::Name,
        }
    }
}

/// Current view/screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Dashboard,
    FileList,
    FileDetail,
    Help,
}

/// File entry with metrics for display
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub relative_path: String,
    pub language: String,
    pub metrics: FileMetrics,
}

/// Main application state
pub struct App {
    /// Whether the app is running
    pub running: bool,

    /// Project root path
    pub project_path: PathBuf,

    /// Project metrics
    pub metrics: Option<ProjectMetrics>,

    /// Sorted file entries
    pub files: Vec<FileEntry>,

    /// Current view
    pub view: View,

    /// Previous view (for back navigation)
    pub previous_view: Option<View>,

    /// Selected file index
    pub selected_index: usize,

    /// Scroll offset for file list
    pub scroll_offset: usize,

    /// Current sort order
    pub sort_by: SortBy,

    /// Sort ascending
    pub sort_ascending: bool,

    /// Filter text
    pub filter: String,

    /// Whether filter input is active
    pub filter_active: bool,

    /// Currently selected file for detail view
    pub selected_file: Option<FileEntry>,

    /// Last analysis time
    pub last_analysis: Option<Instant>,

    /// Analysis in progress
    pub analyzing: bool,

    /// File watcher
    #[allow(dead_code)]
    watcher: Option<RecommendedWatcher>,

    /// Status message
    pub status_message: Option<(String, Instant)>,

    /// Terminal size
    pub terminal_size: (u16, u16),
}

impl App {
    /// Create a new app instance
    pub fn new(path: &Path) -> Result<Self> {
        let project_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

        let mut app = Self {
            running: true,
            project_path: project_path.clone(),
            metrics: None,
            files: Vec::new(),
            view: View::Dashboard,
            previous_view: None,
            selected_index: 0,
            scroll_offset: 0,
            sort_by: SortBy::Complexity,
            sort_ascending: false,
            filter: String::new(),
            filter_active: false,
            selected_file: None,
            last_analysis: None,
            analyzing: false,
            watcher: None,
            status_message: None,
            terminal_size: (80, 24),
        };

        // Perform initial analysis
        app.analyze()?;

        Ok(app)
    }

    /// Analyze the project
    pub fn analyze(&mut self) -> Result<()> {
        self.analyzing = true;
        self.set_status("Analyzing...");

        let scanner = Scanner::new(&self.project_path)?;
        let source_files = scanner.scan()?;

        // Build file entries with analyzed metrics
        let mut new_files = Vec::new();
        let mut project_metrics = ProjectMetrics::new();

        for source_file in &source_files {
            if let Ok(metrics) = FileMetrics::analyze(&source_file.path, source_file.language) {
                project_metrics.add_file(source_file.language.name, &metrics);

                new_files.push(FileEntry {
                    path: source_file.path.clone(),
                    relative_path: source_file
                        .path
                        .strip_prefix(&self.project_path)
                        .unwrap_or(&source_file.path)
                        .to_string_lossy()
                        .to_string(),
                    language: source_file.language.name.to_string(),
                    metrics,
                });
            }
        }

        self.files = new_files;
        self.metrics = Some(project_metrics);
        self.last_analysis = Some(Instant::now());
        self.analyzing = false;

        // Sort files
        self.sort_files();

        let file_count = self.files.len();
        self.set_status(&format!("Analyzed {} files", file_count));

        Ok(())
    }

    /// Sort files based on current sort settings
    pub fn sort_files(&mut self) {
        let ascending = self.sort_ascending;

        self.files.sort_by(|a, b| {
            let cmp = match self.sort_by {
                SortBy::Name => a.relative_path.cmp(&b.relative_path),
                SortBy::Complexity => a
                    .metrics
                    .cyclomatic_complexity
                    .cmp(&b.metrics.cyclomatic_complexity),
                SortBy::CognitiveComplexity => a
                    .metrics
                    .cognitive_complexity
                    .cmp(&b.metrics.cognitive_complexity),
                SortBy::Nesting => a.metrics.max_nesting_depth.cmp(&b.metrics.max_nesting_depth),
                SortBy::Lines => a.metrics.code_lines.cmp(&b.metrics.code_lines),
                SortBy::Maintainability => a
                    .metrics
                    .maintainability_index
                    .partial_cmp(&b.metrics.maintainability_index)
                    .unwrap_or(std::cmp::Ordering::Equal),
                SortBy::Entropy => a
                    .metrics
                    .entropy
                    .partial_cmp(&b.metrics.entropy)
                    .unwrap_or(std::cmp::Ordering::Equal),
            };

            if ascending {
                cmp
            } else {
                cmp.reverse()
            }
        });
    }

    /// Get filtered files
    pub fn filtered_files(&self) -> Vec<&FileEntry> {
        if self.filter.is_empty() {
            self.files.iter().collect()
        } else {
            let filter_lower = self.filter.to_lowercase();
            self.files
                .iter()
                .filter(|f| f.relative_path.to_lowercase().contains(&filter_lower))
                .collect()
        }
    }

    /// Handle tick event
    pub fn tick(&mut self) {
        // Clear old status messages
        if let Some((_, instant)) = &self.status_message {
            if instant.elapsed().as_secs() > 3 {
                self.status_message = None;
            }
        }
    }

    /// Handle key events
    pub fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        // Handle filter input mode
        if self.filter_active {
            match key.code {
                KeyCode::Esc => {
                    self.filter_active = false;
                }
                KeyCode::Enter => {
                    self.filter_active = false;
                }
                KeyCode::Backspace => {
                    self.filter.pop();
                }
                KeyCode::Char(c) => {
                    self.filter.push(c);
                }
                _ => {}
            }
            return Ok(());
        }

        // Global keys
        match key.code {
            KeyCode::Char('q') => {
                self.running = false;
                return Ok(());
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.running = false;
                return Ok(());
            }
            KeyCode::Char('?') => {
                self.show_help();
                return Ok(());
            }
            KeyCode::Esc => {
                self.go_back();
                return Ok(());
            }
            _ => {}
        }

        // View-specific keys
        match self.view {
            View::Dashboard => self.handle_dashboard_keys(key),
            View::FileList => self.handle_file_list_keys(key),
            View::FileDetail => self.handle_file_detail_keys(key),
            View::Help => self.handle_help_keys(key),
        }
    }

    fn handle_dashboard_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Enter | KeyCode::Char('f') => {
                self.previous_view = Some(View::Dashboard);
                self.view = View::FileList;
            }
            KeyCode::Char('r') => {
                self.analyze()?;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_file_list_keys(&mut self, key: KeyEvent) -> Result<()> {
        let filtered = self.filtered_files();
        let file_count = filtered.len();

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_index < file_count.saturating_sub(1) {
                    self.selected_index += 1;
                }
            }
            KeyCode::PageUp => {
                self.selected_index = self.selected_index.saturating_sub(10);
            }
            KeyCode::PageDown => {
                self.selected_index = (self.selected_index + 10).min(file_count.saturating_sub(1));
            }
            KeyCode::Home | KeyCode::Char('g') => {
                self.selected_index = 0;
            }
            KeyCode::End | KeyCode::Char('G') => {
                self.selected_index = file_count.saturating_sub(1);
            }
            KeyCode::Enter => {
                if let Some(file) = filtered.get(self.selected_index) {
                    self.selected_file = Some((*file).clone());
                    self.previous_view = Some(View::FileList);
                    self.view = View::FileDetail;
                }
            }
            KeyCode::Char('s') => {
                self.sort_by = self.sort_by.next();
                self.sort_files();
                self.set_status(&format!("Sorted by {}", self.sort_by.display_name()));
            }
            KeyCode::Char('S') => {
                self.sort_ascending = !self.sort_ascending;
                self.sort_files();
                let order = if self.sort_ascending { "↑" } else { "↓" };
                self.set_status(&format!("Sort order: {}", order));
            }
            KeyCode::Char('/') => {
                self.filter_active = true;
                self.filter.clear();
            }
            KeyCode::Char('r') => {
                self.analyze()?;
            }
            KeyCode::Char('d') => {
                self.previous_view = Some(View::FileList);
                self.view = View::Dashboard;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_file_detail_keys(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('r') => {
                self.analyze()?;
                // Update selected file if it still exists
                if let Some(ref selected) = self.selected_file {
                    self.selected_file = self
                        .files
                        .iter()
                        .find(|f| f.path == selected.path)
                        .cloned();
                }
            }
            KeyCode::Char('o') => {
                // Open file in default editor
                if let Some(ref file) = self.selected_file {
                    self.set_status(&format!("Opening {}...", file.relative_path));
                    // We can't actually open the file from TUI without leaving it
                    // but we can store the path for later
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_help_keys(&mut self, _key: KeyEvent) -> Result<()> {
        self.go_back();
        Ok(())
    }

    /// Handle mouse events
    pub fn handle_mouse_event(&mut self, _mouse: MouseEvent) -> Result<()> {
        // Mouse support can be added later
        Ok(())
    }

    /// Handle terminal resize
    pub fn handle_resize(&mut self, width: u16, height: u16) -> Result<()> {
        self.terminal_size = (width, height);
        Ok(())
    }

    /// Handle file change event
    pub fn handle_file_change(&mut self, path: &Path) -> Result<()> {
        // Check if the changed file is in our project
        if path.starts_with(&self.project_path) {
            // Re-analyze after a short delay to batch changes
            self.set_status(&format!(
                "File changed: {}",
                path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default()
            ));
            self.analyze()?;
        }
        Ok(())
    }

    /// Show help view
    fn show_help(&mut self) {
        self.previous_view = Some(self.view);
        self.view = View::Help;
    }

    /// Go back to previous view
    fn go_back(&mut self) {
        if let Some(prev) = self.previous_view {
            self.view = prev;
            self.previous_view = None;
        } else {
            match self.view {
                View::Help | View::FileDetail => self.view = View::FileList,
                View::FileList => self.view = View::Dashboard,
                View::Dashboard => {} // Stay on dashboard
            }
        }
    }

    /// Set status message
    fn set_status(&mut self, message: &str) {
        self.status_message = Some((message.to_string(), Instant::now()));
    }

    /// Enable file watching
    pub fn enable_file_watching(&mut self, event_handler: &super::EventHandler) -> Result<()> {
        let watcher = event_handler.create_file_watcher(&self.project_path)?;
        self.watcher = Some(watcher);
        self.set_status("File watching enabled");
        Ok(())
    }
}
