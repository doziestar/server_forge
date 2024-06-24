use crate::common;
use crate::common::{MockConfig, MockRollbackManager};
use mockall::predicate::eq;
use server_forge::deployment::{
    deploy_apache, deploy_app, deploy_applications, deploy_mysql, deploy_nginx, deploy_nodejs,
    deploy_php, deploy_postgresql, deploy_python, setup_database, setup_web_server_config,
};
use server_forge::distro::PackageManager;

#[test]
fn test_deploy_applications() {
    let mut mock = common::MockCommandRunner::new();
    let config = MockConfig {
        deployed_apps: vec!["nginx".to_string(), "mysql".to_string()],
        server_role: "web".to_string(),
        ..Default::default()
    };
    let rollback = MockRollbackManager::new();

    mock.expect_run().times(4).returning(|_, _| Ok(()));

    assert!(deploy_applications(&config, &rollback).is_ok());
}

#[test]
fn test_deploy_app() {
    let mut mock = common::MockCommandRunner::new();
    let server_role = "web";

    // Test nginx deployment
    mock.expect_run().times(3).returning(|_, _| Ok(()));
    assert!(deploy_app("nginx", server_role).is_ok());

    // Test unsupported app
    assert!(deploy_app("unsupported-app", server_role).is_err());
}

#[test]
fn test_deploy_nginx() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run()
        .with(eq("apt"), eq(&["install", "-y", "nginx"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("systemctl"), eq(&["start", "nginx"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("systemctl"), eq(&["enable", "nginx"]))
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(deploy_nginx().is_ok());
}

#[test]
fn test_deploy_apache() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run()
        .with(eq("apt"), eq(&["install", "-y", "apache2"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("systemctl"), eq(&["start", "apache2"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("systemctl"), eq(&["enable", "apache2"]))
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(deploy_apache().is_ok());
}

#[test]
fn test_deploy_mysql() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run()
        .with(eq("apt"), eq(&["install", "-y", "mysql-server"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("systemctl"), eq(&["start", "mysql"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("systemctl"), eq(&["enable", "mysql"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("mysql_secure_installation"), eq(&[]))
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(deploy_mysql().is_ok());
}

#[test]
fn test_deploy_postgresql() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run()
        .with(
            eq("apt"),
            eq(&["install", "-y", "postgresql", "postgresql-contrib"]),
        )
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("systemctl"), eq(&["start", "postgresql"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("systemctl"), eq(&["enable", "postgresql"]))
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(deploy_postgresql().is_ok());
}

#[test]
fn test_deploy_php() {
    let mut mock = common::MockCommandRunner::new();
    let server_role = "web";

    mock.expect_run()
        .with(
            eq("apt"),
            eq(&["install", "-y", "php", "php-fpm", "php-mysql"]),
        )
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("apt"), eq(&["install", "-y", "libapache2-mod-php"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("systemctl"), eq(&["start", "php-fpm"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("systemctl"), eq(&["enable", "php-fpm"]))
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(deploy_php(&PackageManager::Apt).is_ok());
}

#[test]
fn test_deploy_nodejs() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(5).returning(|_, _| Ok(()));

    assert!(deploy_nodejs().is_ok());
}

#[test]
fn test_deploy_python() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run()
        .with(
            eq("apt"),
            eq(&["install", "-y", "python3", "python3-pip", "python3-venv"]),
        )
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("pip3"), eq(&["install", "virtualenv"]))
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(deploy_python().is_ok());
}

#[test]
fn test_setup_web_server_config() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(1).returning(|_, _| Ok(()));

    assert!(setup_web_server_config("nginx").is_ok());
    assert!(setup_web_server_config("apache").is_ok());
    assert!(setup_web_server_config("unsupported").is_err());
}

#[test]
fn test_setup_database() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(3).returning(|_, _| Ok(()));

    assert!(setup_database("mysql").is_ok());
    assert!(setup_database("postgresql").is_ok());
    assert!(setup_database("unsupported").is_err());
}
