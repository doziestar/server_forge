use crate::distro::{get_package_manager, uninstall_package};
use log::{info};
use std::cell::RefCell;
use std::error::Error;
use std::fs;

pub struct RollbackManager {
    snapshots: RefCell<Vec<Snapshot>>,
}

struct Snapshot {
    files_changed: Vec<(String, Vec<u8>)>, // (file path, original content)
    packages_installed: Vec<String>,
}

impl RollbackManager {
    pub fn new() -> Self {
        RollbackManager {
            snapshots: RefCell::new(Vec::new()),
        }
    }

    pub fn create_snapshot(&self) -> Result<usize, Box<dyn Error>> {
        let snapshot = Snapshot {
            files_changed: Vec::new(),
            packages_installed: Vec::new(),
        };
        self.snapshots.borrow_mut().push(snapshot);
        Ok(self.snapshots.borrow().len() - 1)
    }

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

    pub fn commit_snapshot(&self, _snapshot_id: usize) -> Result<(), Box<dyn Error>> {
        // we might want to compress the snapshot or write it to disk
        Ok(())
    }

    pub fn rollback_all(&self) -> Result<(), Box<dyn Error>> {
        info!("Rolling back all changes...");

        for snapshot in self.snapshots.borrow().iter().rev() {
            self.rollback_snapshot(snapshot)?;
        }

        info!("Rollback completed");
        Ok(())
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_create_snapshot() {
        let manager = RollbackManager::new();
        let snapshot_id = manager.create_snapshot().unwrap();
        assert_eq!(snapshot_id, 0);
    }

    #[test]
    fn test_add_file_change() {
        let manager = RollbackManager::new();
        let snapshot_id = manager.create_snapshot().unwrap();

        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        fs::write(file_path, b"original content").unwrap();
        manager.add_file_change(snapshot_id, file_path).unwrap();

        fs::write(file_path, b"modified content").unwrap();

        manager.rollback_all().unwrap();

        let content = fs::read_to_string(file_path).unwrap();
        assert_eq!(content, "original content");
    }

    #[test]
    fn test_add_package_installed() {
        let manager = RollbackManager::new();
        let snapshot_id = manager.create_snapshot().unwrap();

        manager
            .add_package_installed(snapshot_id, "test-package")
            .unwrap();

        let snapshots = manager.snapshots.borrow();
        assert_eq!(snapshots[0].packages_installed[0], "test-package");
    }
}
