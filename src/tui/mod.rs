//! Interactive TUI (Terminal User Interface) for Lucent
//!
//! TUI stands for **Terminal User Interface** - an interactive, text-based
//! graphical interface that runs directly in your terminal. Unlike a traditional
//! CLI that outputs text and exits, a TUI provides a persistent, navigable
//! interface with real-time updates.
//!
//! This module provides an interactive terminal interface for exploring
//! code metrics in real-time. Features include:
//!
//! - **Dashboard** - Project overview with summary stats, hotspots, and language breakdown
//! - **File List** - Interactive file browser with sorting and filtering
//! - **File Details** - Drill-down view showing detailed metrics for each file
//! - **Real-time Updates** - Optional file watching for live metric updates
//! - **Keyboard Navigation** - Vim-style navigation (j/k) and intuitive shortcuts
//!
//! ## Usage
//!
//! ```bash
//! # Launch TUI mode
//! lucent tui
//!
//! # Or use the 'ui' alias
//! lucent ui
//!
//! # Enable file watching for real-time updates
//! lucent tui --watch
//!
//! # Analyze a specific directory
//! lucent tui /path/to/project
//! ```
//!
//! ## Keyboard Shortcuts
//!
//! | Key | Action |
//! |-----|--------|
//! | `↑/k` | Move selection up |
//! | `↓/j` | Move selection down |
//! | `PgUp/PgDn` | Page up/down |
//! | `g/G` | Go to first/last |
//! | `Enter` | Open file details |
//! | `Esc` | Go back |
//! | `s` | Cycle sort field |
//! | `S` | Toggle sort order |
//! | `/` | Filter files |
//! | `r` | Refresh analysis |
//! | `d` | Dashboard view |
//! | `f` | File list view |
//! | `?` | Show help |
//! | `q` | Quit |

mod app;
mod event;
mod ui;
mod widgets;

pub use app::{App, AppResult};
pub use event::{Event, EventHandler};

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io::stdout;
use std::path::Path;

/// Run the interactive TUI
pub fn run(path: &Path) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new(path)?;

    // Create event handler
    let event_handler = EventHandler::new(250);

    // Run the main loop
    let result = run_app(&mut terminal, &mut app, &event_handler);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Handle any errors from the app
    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

/// Main application loop
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    event_handler: &EventHandler,
) -> Result<()> {
    while app.running {
        // Draw UI
        terminal.draw(|frame| ui::draw(frame, app))?;

        // Handle events
        match event_handler.next()? {
            Event::Tick => {
                app.tick();
            }
            Event::Key(key_event) => {
                app.handle_key_event(key_event)?;
            }
            Event::Mouse(mouse_event) => {
                app.handle_mouse_event(mouse_event)?;
            }
            Event::Resize(width, height) => {
                app.handle_resize(width, height)?;
            }
            Event::FileChange(path) => {
                app.handle_file_change(&path)?;
            }
        }
    }

    Ok(())
}
