use crate::common;
use crate::common::{MockConfig, MockRollbackManager};
use server_forge::containerization::{
    configure_docker, configure_kubernetes, deploy_container, deploy_containers, deploy_to_docker,
    deploy_to_kubernetes, install_docker, install_kubernetes, setup_docker, setup_kubernetes,
};

#[test]
fn test_setup_docker() {
    let mut mock = common::CommandRunner::new();
    let rollback = MockRollbackManager::new();

    mock.expect_run().times(7).returning(|_, _| Ok(()));

    assert!(setup_docker(&rollback).is_ok());
}

#[test]
fn test_setup_kubernetes() {
    let mut mock = common::MockCommandRunner::new();
    let rollback = MockRollbackManager::new();

    mock.expect_run().times(6).returning(|_, _| Ok(()));

    assert!(setup_kubernetes(&rollback).is_ok());
}

#[test]
fn test_deploy_containers() {
    let mut mock = common::MockCommandRunner::new();
    let config = MockConfig {
        deployed_apps: vec!["app1".to_string(), "app2".to_string()],
        use_kubernetes: true,
        ..Default::default()
    };
    let rollback = MockRollbackManager::new();

    mock.expect_run().times(6).returning(|_, _| Ok(()));

    assert!(deploy_containers(&config, &rollback).is_ok());
}

#[test]
fn test_install_docker() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(7).returning(|_, _| Ok(()));

    assert!(install_docker().is_ok());
}

#[test]
fn test_configure_docker() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(4).returning(|_, _| Ok(()));

    assert!(configure_docker().is_ok());
}

#[test]
fn test_install_kubernetes() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(5).returning(|_, _| Ok(()));

    assert!(install_kubernetes().is_ok());
}

#[test]
fn test_configure_kubernetes() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(3).returning(|_, _| Ok(()));

    assert!(configure_kubernetes().is_ok());
}

#[test]
fn test_deploy_container() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(3).returning(|_, _| Ok(()));

    assert!(deploy_container("test-app", true).is_ok());

    mock.expect_run().times(4).returning(|_, _| Ok(()));

    assert!(deploy_container("test-app", false).is_ok());
}

#[test]
fn test_deploy_to_kubernetes() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(3).returning(|_, _| Ok(()));

    assert!(deploy_to_kubernetes("test-app").is_ok());
}

#[test]
fn test_deploy_to_docker() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(4).returning(|_, _| Ok(()));

    assert!(deploy_to_docker("test-app").is_ok());
}

#[test]
fn test_containerization_error_handling() {
    let mut mock = common::MockCommandRunner::new();
    let config = MockConfig {
        use_containers: true,
        use_kubernetes: true,
        deployed_apps: vec!["test-app".to_string()],
        ..Default::default()
    };
    let rollback = MockRollbackManager::new();

    mock.expect_run()
        .returning(|_, _| Err("Command failed".into()));

    assert!(setup_docker(&rollback).is_err());
    assert!(setup_kubernetes(&rollback).is_err());
    assert!(deploy_containers(&config, &rollback).is_err());
}
