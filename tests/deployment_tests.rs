use crate::common::{CommandRunner, MockConfig, MockRollbackManager};
use server_forge::deployment::{
    deploy_apache, deploy_app, deploy_applications, deploy_mysql, deploy_nginx, deploy_nodejs,
    deploy_php, deploy_postgresql, deploy_python,
};
use server_forge::distro::PackageManager;

mod common;

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

    assert!(deploy_applications(&config, &rollback, &mock).is_ok());
}

#[test]
fn test_deploy_app() {
    let mut mock = common::MockCommandRunner::new();
    let server_role = "web";

    // Test nginx deployment
    mock.expect_run().times(3).returning(|_, _| Ok(()));
    assert!(deploy_app("nginx", server_role, &mock).is_ok());

    // Test unsupported app
    assert!(deploy_app("unsupported-app", server_role, &mock).is_err());
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

    assert!(deploy_nginx(&PackageManager::Apt, &mock).is_ok());
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

    assert!(deploy_apache(&PackageManager::Apt, &mock).is_ok());
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

    assert!(deploy_mysql(&PackageManager::Apt, &mock).is_ok());
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

    assert!(deploy_postgresql(&PackageManager::Apt, &mock).is_ok());
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

    assert!(deploy_php(&PackageManager::Apt, server_role, &mock).is_ok());
}

#[test]
fn test_deploy_nodejs() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run()
        .times(5)
        .returning(|_, _| Ok(()));

    assert!(deploy_nodejs(&mock).is_ok());
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

    assert!(deploy_python(&PackageManager::Apt, &mock).is_ok());
}

#[test]
fn test_setup_web_server_config() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run()
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(setup_web_server_config("nginx", &mock).is_ok());
    assert!(setup_web_server_config("apache", &mock).is_ok());
    assert!(setup_web_server_config("unsupported", &mock).is_err());
}

#[test]
fn test_setup_database() {
    let mut mock = common::MockCommandRunner::new();

    mock.expect_run().times(3).returning(|_, _| Ok(()));

    assert!(setup_database("mysql", &mock).is_ok());
    assert!(setup_database("postgresql", &mock).is_ok());
    assert!(setup_database("unsupported", &mock).is_err());
}
