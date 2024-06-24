use crate::common::{CommandRunner, MockConfig, MockRollbackManager};
use server_forge::monitoring::{
    configure_prometheus, install_monitoring_tools, setup_grafana, setup_monitoring,
    setup_node_exporter,
};

mod common;

#[test]
fn test_setup_monitoring() {
    let mut mock = common::MockCommandRunner::new();
    let config = MockConfig {
        monitoring: true,
        linux_distro: "ubuntu".to_string(),
        ..Default::default()
    };
    let rollback = MockRollbackManager::new();

    mock.expect_run().times(15).returning(|_, _| Ok(()));

    assert!(setup_monitoring(&config, &rollback, &mock).is_ok());

    // Test when monitoring is disabled
    let config_disabled = MockConfig {
        monitoring: false,
        ..Default::default()
    };
    assert!(setup_monitoring(&config_disabled, &rollback, &mock).is_ok());
}

#[test]
fn test_install_monitoring_tools() {
    let mut mock = common::MockCommandRunner::new();
    let config = MockConfig {
        linux_distro: "ubuntu".to_string(),
        ..Default::default()
    };

    mock.expect_run().times(6).returning(|_, _| Ok(()));

    assert!(install_monitoring_tools(&config, &mock).is_ok());
}

#[test]
fn test_configure_prometheus() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(2).returning(|_, _| Ok(()));

    assert!(configure_prometheus(&mock).is_ok());
    assert!(configure_prometheus(&mock).is_ok());
}

#[test]
fn test_setup_grafana() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(2).returning(|_, _| Ok(()));

    assert!(setup_grafana(&mock).is_ok());
}

#[test]
fn test_setup_node_exporter() {
    let mut mock = common::MockCommandRunner::new();
    let config = MockConfig {
        linux_distro: "ubuntu".to_string(),
        ..Default::default()
    };

    mock.expect_run().times(3).returning(|_, _| Ok(()));

    assert!(setup_node_exporter(&config, &mock).is_ok());
}

#[test]
fn test_install_prometheus_from_source() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(12).returning(|_, _| Ok(()));

    assert!(install_prometheus_from_source(&mock).is_ok());
}

#[test]
fn test_install_node_exporter_from_source() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(6).returning(|_, _| Ok(()));

    assert!(install_node_exporter_from_source(&mock).is_ok());
}

#[test]
fn test_monitoring_error_handling() {
    let mut mock = common::MockCommandRunner::new();
    let config = MockConfig {
        monitoring: true,
        linux_distro: "unsupported".to_string(),
        ..Default::default()
    };
    let rollback = MockRollbackManager::new();

    mock.expect_run()
        .returning(|_, _| Err("Command failed".into()));

    assert!(setup_monitoring(&config, &rollback, &mock).is_err());
}
