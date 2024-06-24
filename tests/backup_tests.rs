use crate::common;
use crate::common::{MockConfig, MockRollbackManager};
use server_forge::backup::{
    configure_backup_schedule, install_backup_tools, setup_backup_locations, setup_backup_system,
};
use std::ptr::eq;

#[test]
fn test_setup_backup_system() {
    let mut mock = common::CommandRunner::new();
    let config = MockConfig {
        backup_frequency: "daily".to_string(),
        server_role: "web".to_string(),
        ..Default::default()
    };
    let rollback = MockRollbackManager::new();

    mock.expect_run().times(4).returning(|_, _| Ok(()));

    assert!(setup_backup_system(&config, &rollback).is_ok());
}

#[test]
fn test_install_backup_tools() {
    let mut mock = common::CommandRunner::new();

    mock.expect_run()
        .with(eq("apt", ()), eq(&["install", "-y", "restic"], ()))
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(install_backup_tools().is_ok());
}

// #[test]
// fn test_configure_backup_schedule() {
//     let config = MockConfig {
//         backup_frequency: "daily".to_string(),
//         ..Default::default()
//     };
//
//     assert!(configure_backup_schedule(&config).is_ok());
//
//     // Test invalid frequency
//     let invalid_config = MockConfig {
//         backup_frequency: "invalid".to_string(),
//         ..Default::default()
//     };
//     assert!(configure_backup_schedule(&invalid_config).is_err());
// }

#[test]
fn test_setup_backup_locations() {
    let mut mock = common::CommandRunner::new();
    let config = MockConfig {
        server_role: "web".to_string(),
        ..Default::default()
    };

    mock.expect_run().times(2).returning(|_, _| Ok(()));

    assert!(setup_backup_locations(&config).is_ok());
}
