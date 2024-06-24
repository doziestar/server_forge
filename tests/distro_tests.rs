use crate::common::CommandRunner;
use mockall::predicate::eq;
use server_forge::distro::{
    get_package_manager, install_package, uninstall_package, update_system, PackageManager,
};
use std::fs;
use tempfile::TempDir;

mod common;

#[test]
fn test_get_package_manager() {
    let temp_dir = TempDir::new().unwrap();
    let bin_dir = temp_dir.path().join("usr").join("bin");
    fs::create_dir_all(&bin_dir).unwrap();

    // Test Apt
    fs::File::create(bin_dir.join("apt")).unwrap();
    assert_eq!(get_package_manager().unwrap(), PackageManager::Apt);
    fs::remove_file(bin_dir.join("apt")).unwrap();

    // Test Yum
    fs::File::create(bin_dir.join("yum")).unwrap();
    assert_eq!(get_package_manager().unwrap(), PackageManager::Yum);
    fs::remove_file(bin_dir.join("yum")).unwrap();

    // Test Dnf
    fs::File::create(bin_dir.join("dnf")).unwrap();
    assert_eq!(get_package_manager().unwrap(), PackageManager::Dnf);
    fs::remove_file(bin_dir.join("dnf")).unwrap();

    // Test unsupported
    assert!(get_package_manager().is_err());
}

#[test]
fn test_update_system() {
    let mut mock = common::MockCommandRunner::new();

    // Test Apt
    mock.expect_run()
        .with(eq("apt"), eq(&["update"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("apt"), eq(&["upgrade", "-y"]))
        .times(1)
        .returning(|_, _| Ok(()));
    assert!(update_system(&PackageManager::Apt).is_ok());

    // Test Yum
    mock.expect_run()
        .with(eq("yum"), eq(&["update", "-y"]))
        .times(1)
        .returning(|_, _| Ok(()));
    assert!(update_system(&PackageManager::Yum).is_ok());

    // Test Dnf
    mock.expect_run()
        .with(eq("dnf"), eq(&["upgrade", "-y"]))
        .times(1)
        .returning(|_, _| Ok(()));
    assert!(update_system(&PackageManager::Dnf).is_ok());
}

#[test]
fn test_install_package() {
    let mut mock = common::MockCommandRunner::new();
    let package = "test-package";

    // Test Apt
    mock.expect_run()
        .with(eq("apt"), eq(&["install", "-y", package]))
        .times(1)
        .returning(|_, _| Ok(()));
    assert!(install_package(&PackageManager::Apt, package).is_ok());

    // Test Yum
    mock.expect_run()
        .with(eq("yum"), eq(&["install", "-y", package]))
        .times(1)
        .returning(|_, _| Ok(()));
    assert!(install_package(&PackageManager::Yum, package).is_ok());

    // Test Dnf
    mock.expect_run()
        .with(eq("dnf"), eq(&["install", "-y", package]))
        .times(1)
        .returning(|_, _| Ok(()));
    assert!(install_package(&PackageManager::Dnf, package).is_ok());
}

#[test]
fn test_uninstall_package() {
    let mut mock = common::MockCommandRunner::new();
    let package = "test-package";

    // Test Apt
    mock.expect_run()
        .with(eq("apt"), eq(&["remove", "-y", package]))
        .times(1)
        .returning(|_, _| Ok(()));
    assert!(uninstall_package(&PackageManager::Apt, package).is_ok());

    // Test Yum
    mock.expect_run()
        .with(eq("yum"), eq(&["remove", "-y", package]))
        .times(1)
        .returning(|_, _| Ok(()));
    assert!(uninstall_package(&PackageManager::Yum, package).is_ok());

    // Test Dnf
    mock.expect_run()
        .with(eq("dnf"), eq(&["remove", "-y", package]))
        .times(1)
        .returning(|_, _| Ok(()));
    assert!(uninstall_package(&PackageManager::Dnf, package).is_ok());
}
