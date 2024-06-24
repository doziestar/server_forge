use crate::config::Config;
use crate::rollback::RollbackManager;
use crate::utils::run_command;
use log::info;
use std::error::Error;
use std::fs;

pub fn initial_setup(config: &Config, rollback: &RollbackManager) -> Result<(), Box<dyn Error>> {
    info!("Performing initial setup...");

    let snapshot = rollback.create_snapshot()?;

    update_system(config)?;
    install_essential_packages(config)?;
    setup_firewall(config)?;
    setup_ssh()?;

    rollback.commit_snapshot(snapshot)?;

    info!("Initial setup completed");
    Ok(())
}

fn update_system(config: &Config) -> Result<(), Box<dyn Error>> {
    match config.linux_distro.as_str() {
        "ubuntu" => {
            run_command("apt", &["update"])?;
            run_command("apt", &["upgrade", "-y"])?;
        }
        "centos" => {
            run_command("yum", &["update", "-y"])?;
        }
        "fedora" => {
            run_command("dnf", &["upgrade", "-y"])?;
        }
        _ => return Err("Unsupported Linux distribution".into()),
    }
    Ok(())
}

fn install_essential_packages(config: &Config) -> Result<(), Box<dyn Error>> {
    let essential_packages = [
        "curl",
        "wget",
        "vim",
        "ufw",
        "fail2ban",
        "apt-listchanges",
        "needrestart",
        "debsums",
        "apt-show-versions",
    ];

    match config.linux_distro.as_str() {
        "ubuntu" => {
            for package in &essential_packages {
                run_command("apt", &["install", "-y", package])?;
            }
        }
        "centos" => {
            for package in &essential_packages {
                run_command("yum", &["install", "-y", package])?;
            }
        }
        "fedora" => {
            for package in &essential_packages {
                run_command("dnf", &["install", "-y", package])?;
            }
        }
        _ => return Err("Unsupported Linux distribution".into()),
    }
    Ok(())
}

fn setup_firewall(config: &Config) -> Result<(), Box<dyn Error>> {
    match config.linux_distro.as_str() {
        "ubuntu" => {
            run_command("ufw", &["default", "deny", "incoming"])?;
            run_command("ufw", &["default", "allow", "outgoing"])?;
            run_command("ufw", &["allow", "OpenSSH"])?;
            for rule in &config.custom_firewall_rules {
                run_command("ufw", &["allow", rule])?;
            }
            run_command("ufw", &["enable"])?;
        }
        "centos" | "fedora" => {
            run_command("systemctl", &["start", "firewalld"])?;
            run_command("systemctl", &["enable", "firewalld"])?;
            run_command(
                "firewall-cmd",
                &["--zone=public", "--add-service=ssh", "--permanent"],
            )?;
            for rule in &config.custom_firewall_rules {
                run_command(
                    "firewall-cmd",
                    &["--zone=public", "--add-port=", rule, "--permanent"],
                )?;
            }
            run_command("firewall-cmd", &["--reload"])?;
        }
        _ => return Err("Unsupported Linux distribution".into()),
    }
    Ok(())
}

fn setup_ssh() -> Result<(), Box<dyn Error>> {
    let ssh_config = "/etc/ssh/sshd_config";
    let mut ssh_content = fs::read_to_string(ssh_config)?;
    ssh_content = ssh_content
        .replace("PermitRootLogin yes", "PermitRootLogin no")
        .replace("#PasswordAuthentication yes", "PasswordAuthentication no")
        .replace("#Port 22", "Port 2222"); //TODO: Change SSH port for better security
    fs::write(ssh_config, ssh_content)?;

    run_command("systemctl", &["restart", "sshd"])?;
    Ok(())
}
