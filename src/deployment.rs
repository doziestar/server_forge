use crate::config::Config;
use crate::distro::{get_package_manager, PackageManager};
use crate::rollback::RollbackManager;
use crate::utils::run_command;
use log::info;
use std::error::Error;

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

pub fn deploy_apache() -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;

    match package_manager {
        PackageManager::Apt => run_command("apt", &["install", "-y", "apache2"])?,
        PackageManager::Yum => run_command("yum", &["install", "-y", "httpd"])?,
        PackageManager::Dnf => run_command("dnf", &["install", "-y", "httpd"])?,
    }

    if let Err(_) = run_command("systemctl", &["start", "apache2"]) {
        run_command("systemctl", &["start", "httpd"])?;
    }
    if let Err(_) = run_command("systemctl", &["enable", "apache2"]) {
        run_command("systemctl", &["enable", "httpd"])?;
    }

    Ok(())
}

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

pub fn setup_web_server_config(app: &str) -> Result<(), Box<dyn Error>> {
    match app {
        "nginx" => setup_nginx_config()?,
        "apache" => setup_apache_config()?,
        _ => return Err(format!("Unsupported web server: {}", app).into()),
    }
    Ok(())
}

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
    if let Err(_) = run_command("systemctl", &["reload", "apache2"]) {
        run_command("systemctl", &["reload", "httpd"])?;
    }
    Ok(())
}

pub fn setup_database(db: &str) -> Result<(), Box<dyn Error>> {
    match db {
        "mysql" => setup_mysql()?,
        "postgresql" => setup_postgresql()?,
        _ => return Err(format!("Unsupported database: {}", db).into()),
    }
    Ok(())
}

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

// Helper function to create a simple web application
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

// Helper function to set up firewall rules
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
