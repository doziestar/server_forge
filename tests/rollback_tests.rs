use server_forge::rollback::RollbackManager;
use std::fs;

#[test]
fn test_create_snapshot() {
    let rollback_manager = RollbackManager::new();
    let snapshot_id = rollback_manager.create_snapshot().unwrap();
    assert!(snapshot_id > 0);
}

#[test]
fn test_add_file_change() {
    let rollback_manager = RollbackManager::new();
    let snapshot_id = rollback_manager.create_snapshot().unwrap();

    let test_file = "/tmp/test_rollback.txt";
    fs::write(test_file, "original content").unwrap();

    assert!(rollback_manager
        .add_file_change(snapshot_id, test_file)
        .is_ok());

    // Modify the file
    fs::write(test_file, "modified content").unwrap();

    // Rollback
    assert!(rollback_manager.rollback_to(snapshot_id).is_ok());

    // Verify the file content is back to original
    let content = fs::read_to_string(test_file).unwrap();
    assert_eq!(content, "original content");
}

#[test]
fn test_add_package_installed() {
    let rollback_manager = RollbackManager::new();
    let snapshot_id = rollback_manager.create_snapshot().unwrap();

    let test_package = "htop";

    // Install the package
    std::process::Command::new("apt-get")
        .args(&["install", "-y", test_package])
        .status()
        .unwrap();

    assert!(rollback_manager
        .add_package_installed(snapshot_id, test_package)
        .is_ok());

    // Uninstall the package
    std::process::Command::new("apt-get")
        .args(&["remove", "-y", test_package])
        .status()
        .unwrap();

    // Rollback
    assert!(rollback_manager.rollback_to(snapshot_id).is_ok());

    // Verify the package is installed
    let status = std::process::Command::new("which")
        .arg(test_package)
        .status()
        .unwrap();
    assert!(status.success());
}
