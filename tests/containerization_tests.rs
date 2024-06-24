use common::{CommandRunner, MockConfig, MockRollbackManager};
use server_forge::containerization::{deploy_containers, setup_docker, setup_kubernetes};
use crate::common;


#[test]
fn test_setup_docker() {
    let mut mock = common::CommandRunner::new();
    let rollback = MockRollbackManager::new();

    mock.expect_run().times(7).returning(|_, _| Ok(()));

    assert!(setup_docker(&rollback, &mock).is_ok());
}

#[test]
fn test_setup_kubernetes() {
    let mut mock = common::MockCommandRunner::new();
    let rollback = MockRollbackManager::new();

    mock.expect_run().times(6).returning(|_, _| Ok(()));

    assert!(setup_kubernetes(&rollback, &mock).is_ok());
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

    assert!(deploy_containers(&config, &rollback, &mock).is_ok());
}

#[test]
fn test_install_docker() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(7).returning(|_, _| Ok(()));

    assert!(install_docker(&PackageManager::Apt, &mock).is_ok());
}

#[test]
fn test_configure_docker() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(4).returning(|_, _| Ok(()));

    assert!(configure_docker(&mock).is_ok());
}

#[test]
fn test_install_kubernetes() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(5).returning(|_, _| Ok(()));

    assert!(install_kubernetes(&PackageManager::Apt, &mock).is_ok());
}

#[test]
fn test_configure_kubernetes() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(3).returning(|_, _| Ok(()));

    assert!(configure_kubernetes(&mock).is_ok());
}

#[test]
fn test_deploy_container() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(3).returning(|_, _| Ok(()));

    assert!(deploy_container("test-app", true, &mock).is_ok());

    mock.expect_run().times(4).returning(|_, _| Ok(()));

    assert!(deploy_container("test-app", false, &mock).is_ok());
}

#[test]
fn test_deploy_to_kubernetes() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(3).returning(|_, _| Ok(()));

    assert!(deploy_to_kubernetes("test-app", &mock).is_ok());
}

#[test]
fn test_deploy_to_docker() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(4).returning(|_, _| Ok(()));

    assert!(deploy_to_docker("test-app", &mock).is_ok());
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

    assert!(setup_docker(&rollback, &mock).is_err());
    assert!(setup_kubernetes(&rollback, &mock).is_err());
    assert!(deploy_containers(&config, &rollback, &mock).is_err());
}
