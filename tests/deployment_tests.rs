use server_forge::config::Config;
use server_forge::deployment;
use server_forge::rollback::RollbackManager;

#[test]
fn test_deploy_nginx() {
    assert!(deployment::deploy_nginx().is_ok());

    // Verify Nginx installation
    let nginx_status = std::process::Command::new("which")
        .arg("nginx")
        .status()
        .unwrap();
    assert!(nginx_status.success());

    // Verify Nginx service is running
    let service_status = std::process::Command::new("systemctl")
        .args(&["is-active", "nginx"])
        .status()
        .unwrap();
    assert!(service_status.success());
}

#[test]
fn test_deploy_apache() {
    assert!(deployment::deploy_apache().is_ok());

    // Verify Apache installation (either apache2 or httpd)
    let apache_status = std::process::Command::new("which")
        .arg("apache2")
        .status()
        .unwrap_or_else(|_| {
            std::process::Command::new("which")
                .arg("httpd")
                .status()
                .unwrap()
        });
    assert!(apache_status.success());

    // Verify Apache service is running
    let service_status = std::process::Command::new("systemctl")
        .args(&["is-active", "apache2"])
        .status()
        .unwrap_or_else(|_| {
            std::process::Command::new("systemctl")
                .args(&["is-active", "httpd"])
                .status()
                .unwrap()
        });
    assert!(service_status.success());
}

#[test]
fn test_deploy_mysql() {
    assert!(deployment::deploy_mysql().is_ok());

    // Verify MySQL installation
    let mysql_status = std::process::Command::new("which")
        .arg("mysql")
        .status()
        .unwrap();
    assert!(mysql_status.success());

    // Verify MySQL service is running
    let service_status = std::process::Command::new("systemctl")
        .args(&["is-active", "mysql"])
        .status()
        .unwrap();
    assert!(service_status.success());
}

#[test]
fn test_deploy_postgresql() {
    assert!(deployment::deploy_postgresql().is_ok());

    // Verify PostgreSQL installation
    let psql_status = std::process::Command::new("which")
        .arg("psql")
        .status()
        .unwrap();
    assert!(psql_status.success());

    // Verify PostgreSQL service is running
    let service_status = std::process::Command::new("systemctl")
        .args(&["is-active", "postgresql"])
        .status()
        .unwrap();
    assert!(service_status.success());
}

#[test]
fn test_deploy_php() {
    let server_role = "web";
    assert!(deployment::deploy_php(server_role).is_ok());

    // Verify PHP installation
    let php_status = std::process::Command::new("which")
        .arg("php")
        .status()
        .unwrap();
    assert!(php_status.success());

    // Verify PHP-FPM service is running
    let service_status = std::process::Command::new("systemctl")
        .args(&["is-active", "php-fpm"])
        .status()
        .unwrap();
    assert!(service_status.success());
}

#[test]
fn test_deploy_applications() {
    let config = Config {
        deployed_apps: vec![
            String::from("nginx"),
            String::from("mysql"),
            String::from("php"),
        ],
        ..Default::default()
    };
    let rollback_manager = RollbackManager::new();

    assert!(deployment::deploy_applications(&config, &rollback_manager).is_ok());
}
