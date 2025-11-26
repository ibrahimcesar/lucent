//! Event handling for the TUI
//!
//! This module handles keyboard, mouse, and file system events.

use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

/// Application events
#[derive(Debug)]
pub enum Event {
    /// Terminal tick (for animations/updates)
    Tick,
    /// Keyboard event
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Terminal resize
    Resize(u16, u16),
    /// File system change
    FileChange(PathBuf),
}

/// Handles events from various sources
pub struct EventHandler {
    /// Event receiver
    receiver: Receiver<Event>,
    /// Sender for file watcher (kept alive)
    #[allow(dead_code)]
    sender: Sender<Event>,
}

impl EventHandler {
    /// Create a new event handler with the given tick rate in milliseconds
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();

        // Clone sender for the event thread
        let event_sender = sender.clone();

        // Spawn event handling thread
        thread::spawn(move || {
            let mut last_tick = Instant::now();

            loop {
                // Calculate timeout until next tick
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(Duration::ZERO);

                // Poll for crossterm events
                if event::poll(timeout).unwrap_or(false) {
                    match event::read() {
                        Ok(CrosstermEvent::Key(key)) => {
                            if event_sender.send(Event::Key(key)).is_err() {
                                break;
                            }
                        }
                        Ok(CrosstermEvent::Mouse(mouse)) => {
                            if event_sender.send(Event::Mouse(mouse)).is_err() {
                                break;
                            }
                        }
                        Ok(CrosstermEvent::Resize(width, height)) => {
                            if event_sender.send(Event::Resize(width, height)).is_err() {
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                // Send tick event if needed
                if last_tick.elapsed() >= tick_rate {
                    if event_sender.send(Event::Tick).is_err() {
                        break;
                    }
                    last_tick = Instant::now();
                }
            }
        });

        Self { receiver, sender }
    }

    /// Get the next event (blocking)
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }

    /// Create a file watcher that sends events to this handler
    pub fn create_file_watcher(&self, path: &std::path::Path) -> Result<RecommendedWatcher> {
        let sender = self.sender.clone();

        let mut watcher = RecommendedWatcher::new(
            move |res: Result<notify::Event, notify::Error>| {
                if let Ok(event) = res {
                    // Only send events for actual file modifications
                    if event.kind.is_modify() || event.kind.is_create() || event.kind.is_remove() {
                        for path in event.paths {
                            let _ = sender.send(Event::FileChange(path));
                        }
                    }
                }
            },
            Config::default(),
        )?;

        watcher.watch(path, RecursiveMode::Recursive)?;

        Ok(watcher)
    }
}
