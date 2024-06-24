use crate::common::MockCommandRunner;
use mockall::predicate::*;
use server_forge::config::Config;
use server_forge::rollback::RollbackManager;
use server_forge::setup::{
    initial_setup, install_essential_packages, setup_firewall, setup_ssh, update_system,
};

#[test]
fn test_initial_setup() {
    let mut mock = MockCommandRunner::new();
    mock.expect_run().returning(|_, _| Ok(()));

    let config = Config::default();
    let rollback = RollbackManager::new();

    assert!(initial_setup(&config, &rollback).is_ok());
}

#[test]
fn test_update_system() {
    let mut mock = MockCommandRunner::new();
    let config = Config::default();

    mock.expect_run()
        .with(eq("apt"), eq(&["update"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("apt"), eq(&["upgrade", "-y"]))
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(update_system(&config).is_ok());
}

// #[test]
// fn test_install_essential_packages() {
//     let mut mock = MockCommandRunner::new();
//     let config = Config::default();
//
//     mock.expect_run()
//         .with(eq("apt"), eq(&["install", "-y", any::<&str>()]))
//         .times(9) // Number of essential packages
//         .returning(|_, _| Ok(()));
//
//     assert!(install_essential_packages(&config, &mock).is_ok());
// }

// #[test]
// fn test_setup_firewall() {
//     let mut mock = MockCommandRunner::new();
//     let mut config = Config::default();
//     config.custom_firewall_rules = vec!["80/tcp".to_string()];
//
//     mock.expect_run()
//         .with(eq("ufw"), any::<&[&str]>())
//         .times(5) // Number of ufw commands
//         .returning(|_, _| Ok(()));
//
//     assert!(setup_firewall(&config, &mock).is_ok());
// }

#[test]
fn test_setup_ssh() {
    let mut mock = MockCommandRunner::new();

    mock.expect_run()
        .with(eq("systemctl"), eq(&["restart", "sshd"]))
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(setup_ssh().is_ok());
}
