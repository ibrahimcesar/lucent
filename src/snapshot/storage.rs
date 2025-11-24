//! Snapshot storage and management

use super::{Snapshot, SnapshotMetadata};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Default snapshot directory
const DEFAULT_SNAPSHOT_DIR: &str = ".lucent/history";

/// Manages snapshot storage and retrieval
pub struct SnapshotManager {
    /// Directory where snapshots are stored
    snapshot_dir: PathBuf,
}

impl SnapshotManager {
    /// Create a new snapshot manager with default directory
    pub fn new() -> Result<Self> {
        Self::with_dir(DEFAULT_SNAPSHOT_DIR)
    }

    /// Create a new snapshot manager with custom directory
    pub fn with_dir<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let snapshot_dir = dir.as_ref().to_path_buf();

        // Create directory if it doesn't exist
        if !snapshot_dir.exists() {
            fs::create_dir_all(&snapshot_dir)
                .with_context(|| format!("Failed to create snapshot directory: {}", snapshot_dir.display()))?;
        }

        Ok(Self { snapshot_dir })
    }

    /// Save a snapshot to disk
    pub fn save(&self, snapshot: &Snapshot) -> Result<PathBuf> {
        let filename = format!("{}.json", snapshot.id);
        let path = self.snapshot_dir.join(&filename);

        let json = serde_json::to_string_pretty(snapshot)
            .context("Failed to serialize snapshot")?;

        fs::write(&path, json)
            .with_context(|| format!("Failed to write snapshot to {}", path.display()))?;

        Ok(path)
    }

    /// Load a snapshot by ID
    pub fn load(&self, id: &str) -> Result<Snapshot> {
        let filename = format!("{}.json", id);
        let path = self.snapshot_dir.join(&filename);

        let json = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read snapshot from {}", path.display()))?;

        let snapshot: Snapshot = serde_json::from_str(&json)
            .context("Failed to deserialize snapshot")?;

        Ok(snapshot)
    }

    /// List all available snapshots (sorted by timestamp, newest first)
    pub fn list(&self) -> Result<Vec<SnapshotMetadata>> {
        let mut snapshots = Vec::new();

        if !self.snapshot_dir.exists() {
            return Ok(snapshots);
        }

        for entry in fs::read_dir(&self.snapshot_dir)
            .with_context(|| format!("Failed to read snapshot directory: {}", self.snapshot_dir.display()))? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                match self.load_metadata_from_file(&path) {
                    Ok(metadata) => snapshots.push(metadata),
                    Err(e) => {
                        eprintln!("Warning: Failed to load snapshot {}: {}", path.display(), e);
                    }
                }
            }
        }

        // Sort by timestamp, newest first
        snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(snapshots)
    }

    /// Load metadata from a snapshot file
    fn load_metadata_from_file(&self, path: &Path) -> Result<SnapshotMetadata> {
        let json = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let snapshot: Snapshot = serde_json::from_str(&json)
            .context("Failed to deserialize snapshot")?;

        Ok(snapshot.metadata())
    }

    /// Delete a snapshot by ID
    pub fn delete(&self, id: &str) -> Result<()> {
        let filename = format!("{}.json", id);
        let path = self.snapshot_dir.join(&filename);

        fs::remove_file(&path)
            .with_context(|| format!("Failed to delete snapshot {}", path.display()))?;

        Ok(())
    }

    /// Delete old snapshots, keeping only the specified number
    pub fn cleanup(&self, keep_count: usize) -> Result<usize> {
        let snapshots = self.list()?;

        if snapshots.len() <= keep_count {
            return Ok(0);
        }

        let to_delete = &snapshots[keep_count..];
        let mut deleted = 0;

        for snapshot in to_delete {
            match self.delete(&snapshot.id) {
                Ok(_) => deleted += 1,
                Err(e) => {
                    eprintln!("Warning: Failed to delete snapshot {}: {}", snapshot.id, e);
                }
            }
        }

        Ok(deleted)
    }

    /// Get the most recent snapshot
    pub fn latest(&self) -> Result<Option<Snapshot>> {
        let snapshots = self.list()?;

        if let Some(metadata) = snapshots.first() {
            Ok(Some(self.load(&metadata.id)?))
        } else {
            Ok(None)
        }
    }

    /// Count total snapshots
    pub fn count(&self) -> Result<usize> {
        Ok(self.list()?.len())
    }
}

impl Default for SnapshotManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default SnapshotManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::ProjectMetrics;
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    fn test_snapshot_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = SnapshotManager::with_dir(temp_dir.path()).unwrap();
        assert!(temp_dir.path().exists());
    }

    #[test]
    fn test_save_and_load_snapshot() {
        let temp_dir = TempDir::new().unwrap();
        let manager = SnapshotManager::with_dir(temp_dir.path()).unwrap();

        let metrics = ProjectMetrics {
            total_files: 0,
            by_language: HashMap::new(),
        };

        let snapshot = Snapshot::new(metrics, Some("Test".to_string())).unwrap();
        let id = snapshot.id.clone();

        // Save
        manager.save(&snapshot).unwrap();

        // Load
        let loaded = manager.load(&id).unwrap();
        assert_eq!(loaded.id, id);
        assert_eq!(loaded.description, Some("Test".to_string()));
    }

    #[test]
    fn test_list_snapshots() {
        let temp_dir = TempDir::new().unwrap();
        let manager = SnapshotManager::with_dir(temp_dir.path()).unwrap();

        // Create multiple snapshots
        for i in 0..3 {
            let metrics = ProjectMetrics {
                total_files: 0,
                by_language: HashMap::new(),
            };
            let snapshot = Snapshot::new(metrics, Some(format!("Test {}", i))).unwrap();
            manager.save(&snapshot).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(10)); // Ensure unique timestamps
        }

        let list = manager.list().unwrap();
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_delete_snapshot() {
        let temp_dir = TempDir::new().unwrap();
        let manager = SnapshotManager::with_dir(temp_dir.path()).unwrap();

        let metrics = ProjectMetrics {
            total_files: 0,
            by_language: HashMap::new(),
        };
        let snapshot = Snapshot::new(metrics, None).unwrap();
        let id = snapshot.id.clone();

        manager.save(&snapshot).unwrap();
        assert_eq!(manager.count().unwrap(), 1);

        manager.delete(&id).unwrap();
        assert_eq!(manager.count().unwrap(), 0);
    }

    #[test]
    fn test_cleanup_old_snapshots() {
        let temp_dir = TempDir::new().unwrap();
        let manager = SnapshotManager::with_dir(temp_dir.path()).unwrap();

        // Create 5 snapshots
        for i in 0..5 {
            let metrics = ProjectMetrics {
                total_files: 0,
                by_language: HashMap::new(),
            };
            let snapshot = Snapshot::new(metrics, Some(format!("Test {}", i))).unwrap();
            manager.save(&snapshot).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(10)); // Ensure unique timestamps
        }

        assert_eq!(manager.count().unwrap(), 5);

        // Keep only 3 most recent
        let deleted = manager.cleanup(3).unwrap();
        assert_eq!(deleted, 2);
        assert_eq!(manager.count().unwrap(), 3);
    }
}
