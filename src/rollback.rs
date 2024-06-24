//! # Rollback Module
//!
//! This module provides functionality for creating system snapshots and rolling back changes.
//! It allows the application to revert the system state in case of failures during the setup process.

use crate::distro::{get_package_manager, uninstall_package};
use log::info;
use std::cell::RefCell;
use std::error::Error;
use std::fs;

/// Manages the creation of snapshots and rollback operations.
pub struct RollbackManager {
    snapshots: RefCell<Vec<Snapshot>>,
}

/// Represents a system snapshot, containing information about changed files and installed packages.
struct Snapshot {
    files_changed: Vec<(String, Vec<u8>)>, // (file path, original content)
    packages_installed: Vec<String>,
}

impl RollbackManager {
    /// Creates a new `RollbackManager` instance.
    pub fn new() -> Self {
        RollbackManager {
            snapshots: RefCell::new(Vec::new()),
        }
    }

    /// Creates a new snapshot and returns its ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the snapshot creation fails.
    pub fn create_snapshot(&self) -> Result<usize, Box<dyn Error>> {
        let snapshot = Snapshot {
            files_changed: Vec::new(),
            packages_installed: Vec::new(),
        };
        self.snapshots.borrow_mut().push(snapshot);
        Ok(self.snapshots.borrow().len() - 1)
    }

    /// Adds a file change to a specific snapshot.
    ///
    /// # Arguments
    ///
    /// * `snapshot_id` - The ID of the snapshot to add the file change to
    /// * `file_path` - The path of the changed file
    ///
    /// # Errors
    ///
    /// Returns an error if reading the file fails or if the snapshot ID is invalid.
    pub fn add_file_change(
        &self,
        snapshot_id: usize,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        let original_content = fs::read(file_path)?;
        self.snapshots.borrow_mut()[snapshot_id]
            .files_changed
            .push((file_path.to_string(), original_content));
        Ok(())
    }

    /// Adds an installed package to a specific snapshot.
    ///
    /// # Arguments
    ///
    /// * `snapshot_id` - The ID of the snapshot to add the package to
    /// * `package` - The name of the installed package
    ///
    /// # Errors
    ///
    /// Returns an error if the snapshot ID is invalid.
    pub fn add_package_installed(
        &self,
        snapshot_id: usize,
        package: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.snapshots.borrow_mut()[snapshot_id]
            .packages_installed
            .push(package.to_string());
        Ok(())
    }

    /// Commits a snapshot, finalizing its state.
    ///
    /// This method is a placeholder and currently does nothing.
    /// It could be expanded to compress the snapshot or write it to disk.
    ///
    /// # Arguments
    ///
    /// * `_snapshot_id` - The ID of the snapshot to commit
    pub fn commit_snapshot(&self, _snapshot_id: usize) -> Result<(), Box<dyn Error>> {
        // we could compress the snapshot or write it to disk here
        Ok(())
    }

    /// Rolls back all changes made since the first snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error if any part of the rollback process fails.
    pub fn rollback_all(&self) -> Result<(), Box<dyn Error>> {
        info!("Rolling back all changes...");

        for snapshot in self.snapshots.borrow().iter().rev() {
            self.rollback_snapshot(snapshot)?;
        }

        info!("Rollback completed");
        Ok(())
    }

    /// Rolls back changes made in a specific snapshot.
    ///
    /// # Arguments
    ///
    /// * `snapshot` - A reference to the `Snapshot` to roll back
    ///
    /// # Errors
    ///
    /// Returns an error if any part of the rollback process fails.
    fn rollback_snapshot(&self, snapshot: &Snapshot) -> Result<(), Box<dyn Error>> {
        // Rollback file changes
        for (file_path, original_content) in &snapshot.files_changed {
            info!("Rolling back changes to file: {}", file_path);
            fs::write(file_path, original_content)?;
        }

        // Uninstall packages
        let package_manager = get_package_manager()?;
        for package in &snapshot.packages_installed {
            info!("Uninstalling package: {}", package);
            uninstall_package(&package_manager, package)?;
        }

        Ok(())
    }

    /// Rolls back changes to a specific snapshot.
    ///
    /// # Arguments
    ///
    /// * `snapshot_id` - The ID of the snapshot to roll back to
    ///
    /// # Errors
    ///
    /// Returns an error if the snapshot ID is invalid or if any part of the rollback process fails.
    pub fn rollback_to(&self, snapshot_id: usize) -> Result<(), Box<dyn Error>> {
        info!("Rolling back to snapshot {}", snapshot_id);

        let snapshots = self.snapshots.borrow();
        if snapshot_id >= snapshots.len() {
            return Err("Invalid snapshot ID".into());
        }

        for snapshot in snapshots.iter().skip(snapshot_id).rev() {
            self.rollback_snapshot(snapshot)?;
        }

        info!("Rollback to snapshot {} completed", snapshot_id);
        Ok(())
    }
}
