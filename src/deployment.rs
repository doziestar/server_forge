//! # Deployment Module
//!
//! This module provides functionality for deploying various applications and services
//! on a Linux server. It supports deployment of web servers (Nginx, Apache), databases
//! (MySQL, PostgreSQL), programming languages and runtimes (PHP, Node.js, Python),
//! and configures them according to best practices.
//!
//! The module is designed to work across different Linux distributions by leveraging
//! the appropriate package manager for each system.

use crate::config::Config;
use crate::distro::{get_package_manager, PackageManager};
use crate::rollback::RollbackManager;
use crate::utils::run_command;
use log::info;
use std::error::Error;

/// Deploys all applications specified in the configuration.
///
/// This function iterates through the list of applications specified in the configuration
/// and deploys each one. It creates a snapshot before deployment for potential rollback.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct containing deployment information
/// * `rollback` - A reference to the `RollbackManager` for creating snapshots
///
/// # Returns
///
/// Returns `Ok(())` if all applications are deployed successfully, or an error if any deployment fails.
pub fn deploy_applications(
    config: &Config,
    rollback: &RollbackManager,
) -> Result<(), Box<dyn Error>> {
    info!("Deploying applications...");

    let snapshot = rollback.create_snapshot()?;

    for app in &config.deployed_apps {
        deploy_app(app, &config.server_role)?;
    }

    rollback.commit_snapshot(snapshot)?;

    info!("Application deployment completed");
    Ok(())
}

/// Deploys a single application based on its type and the server role.
///
/// # Arguments
///
/// * `app` - A string slice representing the application to deploy
/// * `server_role` - A string slice representing the role of the server (e.g., "web", "database")
///
/// # Returns
///
/// Returns `Ok(())` if the application is deployed successfully, or an error if deployment fails.
pub fn deploy_app(app: &str, server_role: &str) -> Result<(), Box<dyn Error>> {
    match app {
        "nginx" => deploy_nginx()?,
        "apache" => deploy_apache()?,
        "mysql" => deploy_mysql()?,
        "postgresql" => deploy_postgresql()?,
        "php" => deploy_php(server_role)?,
        "nodejs" => deploy_nodejs()?,
        "python" => deploy_python()?,
        _ => return Err(format!("Unsupported application: {}", app).into()),
    }
    Ok(())
}

/// Deploys and configures the Nginx web server.
///
/// This function installs Nginx using the appropriate package manager,
/// starts the Nginx service, and enables it to start on boot.
///
/// # Returns
///
/// Returns `Ok(())` if Nginx is deployed successfully, or an error if deployment fails.
pub fn deploy_nginx() -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;

    match package_manager {
        PackageManager::Apt => run_command("apt", &["install", "-y", "nginx"])?,
        PackageManager::Yum => run_command("yum", &["install", "-y", "nginx"])?,
        PackageManager::Dnf => run_command("dnf", &["install", "-y", "nginx"])?,
    }

    run_command("systemctl", &["start", "nginx"])?;
    run_command("systemctl", &["enable", "nginx"])?;

    Ok(())
}

/// Deploys and configures the Apache web server.
///
/// This function installs Apache (httpd) using the appropriate package manager,
/// starts the Apache service, and enables it to start on boot.
///
/// # Returns
///
/// Returns `Ok(())` if Apache is deployed successfully, or an error if deployment fails.
pub fn deploy_apache() -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;

    match package_manager {
        PackageManager::Apt => run_command("apt", &["install", "-y", "apache2"])?,
        PackageManager::Yum => run_command("yum", &["install", "-y", "httpd"])?,
        PackageManager::Dnf => run_command("dnf", &["install", "-y", "httpd"])?,
    }

    if run_command("systemctl", &["start", "apache2"]).is_err() {
        run_command("systemctl", &["start", "httpd"])?;
    }

    if run_command("systemctl", &["enable", "apache2"]).is_err() {
        run_command("systemctl", &["enable", "httpd"])?;
    }

    Ok(())
}

/// Deploys and configures the MySQL database server.
///
/// This function installs MySQL using the appropriate package manager,
/// starts the MySQL service, enables it to start on boot, and runs the
/// mysql_secure_installation script to set up basic security measures.
///
/// # Returns
///
/// Returns `Ok(())` if MySQL is deployed successfully, or an error if deployment fails.
pub fn deploy_mysql() -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;

    match package_manager {
        PackageManager::Apt => run_command("apt", &["install", "-y", "mysql-server"])?,
        PackageManager::Yum => run_command("yum", &["install", "-y", "mysql-server"])?,
        PackageManager::Dnf => run_command("dnf", &["install", "-y", "mysql-server"])?,
    }

    run_command("systemctl", &["start", "mysql"])?;
    run_command("systemctl", &["enable", "mysql"])?;

    // Secure MySQL installation
    run_command("mysql_secure_installation", &[])?;

    Ok(())
}

/// Deploys and configures the PostgreSQL database server.
///
/// This function installs PostgreSQL using the appropriate package manager,
/// initializes the database if necessary (for CentOS/Fedora), starts the
/// PostgreSQL service, and enables it to start on boot.
///
/// # Returns
///
/// Returns `Ok(())` if PostgreSQL is deployed successfully, or an error if deployment fails.
pub fn deploy_postgresql() -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;

    match package_manager {
        PackageManager::Apt => run_command(
            "apt",
            &["install", "-y", "postgresql", "postgresql-contrib"],
        )?,
        PackageManager::Yum => run_command(
            "yum",
            &["install", "-y", "postgresql-server", "postgresql-contrib"],
        )?,
        PackageManager::Dnf => run_command(
            "dnf",
            &["install", "-y", "postgresql-server", "postgresql-contrib"],
        )?,
    }

    // Initialize the database (for CentOS/Fedora)
    if package_manager != PackageManager::Apt {
        run_command("postgresql-setup", &["--initdb"])?;
    }

    run_command("systemctl", &["start", "postgresql"])?;
    run_command("systemctl", &["enable", "postgresql"])?;

    Ok(())
}

/// Deploys and configures PHP.
///
/// This function installs PHP and related packages using the appropriate package manager.
/// It also installs additional modules based on the server role (e.g., libapache2-mod-php for web servers).
///
/// # Arguments
///
/// * `server_role` - A string slice representing the role of the server (e.g., "web")
///
/// # Returns
///
/// Returns `Ok(())` if PHP is deployed successfully, or an error if deployment fails.
pub fn deploy_php(server_role: &str) -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;

    match package_manager {
        PackageManager::Apt => {
            run_command("apt", &["install", "-y", "php", "php-fpm", "php-mysql"])?;
            if server_role == "web" {
                run_command("apt", &["install", "-y", "libapache2-mod-php"])?;
            }
        }
        PackageManager::Yum | PackageManager::Dnf => {
            let install_cmd = if package_manager == PackageManager::Yum {
                "yum"
            } else {
                "dnf"
            };
            run_command(
                install_cmd,
                &["install", "-y", "php", "php-fpm", "php-mysqlnd"],
            )?;
            if server_role == "web" {
                run_command(install_cmd, &["install", "-y", "php-apache"])?;
            }
        }
    }

    run_command("systemctl", &["start", "php-fpm"])?;
    run_command("systemctl", &["enable", "php-fpm"])?;

    Ok(())
}

/// Deploys and configures Node.js.
///
/// This function installs Node.js using NVM (Node Version Manager), installs the latest LTS version,
/// and sets it as the default. It also installs the PM2 process manager for running Node.js applications.
///
/// # Returns
///
/// Returns `Ok(())` if Node.js is deployed successfully, or an error if deployment fails.
pub fn deploy_nodejs() -> Result<(), Box<dyn Error>> {
    // Install Node.js using NVM (Node Version Manager)
    run_command(
        "curl",
        &[
            "-o-",
            "https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh",
            "|",
            "bash",
        ],
    )?;
    run_command("source", &["~/.nvm/nvm.sh"])?;
    run_command("nvm", &["install", "node"])?;
    run_command("nvm", &["use", "node"])?;

    // Install PM2 process manager
    run_command("npm", &["install", "-g", "pm2"])?;

    Ok(())
}

/// Deploys and configures Python.
///
/// This function installs Python 3 and pip using the appropriate package manager.
/// It also installs virtualenv for creating isolated Python environments.
///
/// # Returns
///
/// Returns `Ok(())` if Python is deployed successfully, or an error if deployment fails.
pub fn deploy_python() -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;

    match package_manager {
        PackageManager::Apt => run_command(
            "apt",
            &["install", "-y", "python3", "python3-pip", "python3-venv"],
        )?,
        PackageManager::Yum => run_command("yum", &["install", "-y", "python3", "python3-pip"])?,
        PackageManager::Dnf => run_command("dnf", &["install", "-y", "python3", "python3-pip"])?,
    }

    // Install virtualenv
    run_command("pip3", &["install", "virtualenv"])?;

    Ok(())
}

/// Sets up the web server configuration based on the specified application.
/// This function configures the default web server configuration for Nginx or Apache.
/// It creates a basic "Hello, World!" index page in the web root directory.
///
/// # Arguments
///
/// * `app` - The name of the application (e.g., "nginx" or "apache").
///
/// # Returns
///
/// Returns `Ok(())` if the web server configuration is set up successfully, or an error if configuration fails.
pub fn setup_web_server_config(app: &str) -> Result<(), Box<dyn Error>> {
    match app {
        "nginx" => setup_nginx_config()?,
        "apache" => setup_apache_config()?,
        _ => return Err(format!("Unsupported web server: {}", app).into()),
    }
    Ok(())
}

/// Creates a sample web application based on the specified application type.
/// This function creates a basic "Hello, World!" application for PHP, Node.js, or Python,
/// demonstrating how to set up a simple web server for each technology.
/// # Arguments
///
/// * `app_type` - A string slice representing the type of application to create ("php", "nodejs", or "python")
/// # Returns
///
/// Returns `Ok(())` if the sample application is created successfully, or an error if creation fails.
fn setup_nginx_config() -> Result<(), Box<dyn Error>> {
    let nginx_config = r#"
server {
    listen 80 default_server;
    listen [::]:80 default_server;
    root /var/www/html;
    index index.html index.htm index.nginx-debian.html;
    server_name _;
    location / {
        try_files $uri $uri/ =404;
    }
}
"#;
    std::fs::write("/etc/nginx/sites-available/default", nginx_config)?;
    run_command("systemctl", &["reload", "nginx"])?;
    Ok(())
}

/// Sets up the Apache web server configuration.
/// This function configures the default Apache virtual host configuration.
///
/// # Returns
///
/// Returns `Ok(())` if the Apache configuration is set up successfully, or an error if configuration fails.
fn setup_apache_config() -> Result<(), Box<dyn Error>> {
    let apache_config = r#"
<VirtualHost *:80>
    ServerAdmin webmaster@localhost
    DocumentRoot /var/www/html
    ErrorLog ${APACHE_LOG_DIR}/error.log
    CustomLog ${APACHE_LOG_DIR}/access.log combined
</VirtualHost>
"#;
    std::fs::write(
        "/etc/apache2/sites-available/000-default.conf",
        apache_config,
    )?;

    if run_command("systemctl", &["reload", "apache2"]).is_err() {
        run_command("systemctl", &["reload", "httpd"])?;
    }
    Ok(())
}

/// Sets up the database based on the specified database type.
/// This function sets up the MySQL or PostgreSQL database server by running the necessary
///
/// # Arguments
///
/// * `db` - A string slice representing the type of database to set up ("mysql" or "postgresql")
///
/// # Returns
///
/// Returns `Ok(())` if the database is set up successfully, or an error if setting up fails.
pub fn setup_database(db: &str) -> Result<(), Box<dyn Error>> {
    match db {
        "mysql" => setup_mysql()?,
        "postgresql" => setup_postgresql()?,
        _ => return Err(format!("Unsupported database: {}", db).into()),
    }
    Ok(())
}

/// Sets up the MySQL database server.
/// This function sets the root password, removes anonymous users, and flushes privileges.
///
/// # Returns
///
/// Returns `Ok(())` if the MySQL server is set up successfully, or an error if setting up fails.
fn setup_mysql() -> Result<(), Box<dyn Error>> {
    // Generate a secure random password
    let password = generate_secure_password();

    // Set root password and remove anonymous users
    run_command(
        "mysql",
        &[
            "-e",
            &format!(
                "ALTER USER 'root'@'localhost' IDENTIFIED BY '{}';",
                password
            ),
        ],
    )?;
    run_command("mysql", &["-e", "DELETE FROM mysql.user WHERE User='';"])?;
    run_command("mysql", &["-e", "FLUSH PRIVILEGES;"])?;

    // Save the password securely (this is a placeholder - in a real-world scenario,
    // you'd want to use a more secure method to store this password)
    std::fs::write("/root/.mysql_root_password", &password)?;

    Ok(())
}

/// Sets up the PostgreSQL database server.
/// This function sets the password for the postgres user and saves it securely.
///
/// # Returns
///
/// Returns `Ok(())` if the PostgreSQL server is set up successfully, or an error if setting up fails.
fn setup_postgresql() -> Result<(), Box<dyn Error>> {
    // Generate a secure random password
    let password = generate_secure_password();

    // Set postgres user password
    run_command(
        "sudo",
        &[
            "-u",
            "postgres",
            "psql",
            "-c",
            &format!("ALTER USER postgres PASSWORD '{}';", password),
        ],
    )?;

    // Save the password securely
    // you'd want to use a more secure method to store this password)
    std::fs::write("/root/.postgres_password", &password)?;

    Ok(())
}

/// Generates a secure random password.
///
/// This function creates a random password of 20 characters, including uppercase and lowercase
/// letters, numbers, and special characters.
///
/// # Returns
///
/// Returns a `String` containing the generated password.
fn generate_secure_password() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 20;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    password
}

/// Creates a sample web application based on the specified application type.
///
/// This function creates a basic "Hello, World!" application for PHP, Node.js, or Python,
/// demonstrating how to set up a simple web server for each technology.
///
/// # Arguments
///
/// * `app_type` - A string slice representing the type of application to create ("php", "nodejs", or "python")
///
/// # Returns
///
/// Returns `Ok(())` if the sample application is created successfully, or an error if creation fails.
fn create_sample_web_app(app_type: &str) -> Result<(), Box<dyn Error>> {
    match app_type {
        "php" => {
            let php_content = r#"
<?php
echo "Hello, World! This is a sample PHP application.";
?>
"#;
            std::fs::write("/var/www/html/index.php", php_content)?;
        }
        "nodejs" => {
            let node_content = r#"
const http = require('http');
const server = http.createServer((req, res) => {
  res.statusCode = 200;
  res.setHeader('Content-Type', 'text/plain');
  res.end('Hello, World! This is a sample Node.js application.');
});
server.listen(3000, '127.0.0.1', () => {
  console.log('Server running on http://127.0.0.1:3000/');
});
"#;
            std::fs::write("/root/app.js", node_content)?;
            run_command("pm2", &["start", "/root/app.js"])?;
        }
        "python" => {
            let python_content = r#"
from flask import Flask
app = Flask(__name__)

@app.route('/')
def hello_world():
    return 'Hello, World! This is a sample Python Flask application.'

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
"#;
            std::fs::write("/root/app.py", python_content)?;
            run_command("pip3", &["install", "flask"])?;
            run_command("python3", &["/root/app.py", "&"])?;
        }
        _ => return Err(format!("Unsupported application type: {}", app_type).into()),
    }
    Ok(())
}

/// Sets up firewall rules based on the configuration.
///
/// This function configures the firewall (ufw for Ubuntu, firewalld for CentOS/Fedora)
/// with basic rules for SSH, HTTP, and HTTPS, as well as any custom rules specified in the configuration.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct containing firewall configuration
///
/// # Returns
///
/// Returns `Ok(())` if firewall rules are set up successfully, or an error if setup fails.
fn setup_firewall_rules(config: &Config) -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;

    match package_manager {
        PackageManager::Apt => {
            run_command("ufw", &["allow", "OpenSSH"])?;
            run_command("ufw", &["allow", "80/tcp"])?;
            run_command("ufw", &["allow", "443/tcp"])?;
            for rule in &config.custom_firewall_rules {
                run_command("ufw", &["allow", rule])?;
            }
            run_command("ufw", &["enable"])?;
        }
        PackageManager::Yum | PackageManager::Dnf => {
            run_command("firewall-cmd", &["--permanent", "--add-service=ssh"])?;
            run_command("firewall-cmd", &["--permanent", "--add-service=http"])?;
            run_command("firewall-cmd", &["--permanent", "--add-service=https"])?;
            for rule in &config.custom_firewall_rules {
                run_command("firewall-cmd", &["--permanent", "--add-port=", rule])?;
            }
            run_command("firewall-cmd", &["--reload"])?;
        }
    }
    Ok(())
}
