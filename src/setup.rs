//! # Setup Module
//!
//! This module provides functionality for performing initial setup tasks on a Linux server.
//! It includes functions for updating the system, installing essential packages,
//! setting up a firewall, and configuring SSH for improved security.
//!
//! The module is designed to work across different Linux distributions by using
//! distribution-specific commands where necessary.
use crate::config::Config;
use crate::rollback::RollbackManager;
use crate::utils::run_command;
use log::info;
use std::error::Error;
use std::fs;

/// Performs the initial setup of the server based on the provided configuration.
///
/// This function orchestrates the entire initial setup process, including:
/// - Updating the system
/// - Installing essential packages
/// - Setting up the firewall
/// - Configuring SSH
///
/// It creates a snapshot before starting the setup process for potential rollback.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct containing setup configuration
/// * `rollback` - A reference to the `RollbackManager` for creating snapshots
///
/// # Returns
///
/// Returns `Ok(())` if the initial setup is completed successfully, or an error if setup fails.
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

/// Updates the system using the appropriate package manager for the Linux distribution.
///
/// This function runs system update commands specific to Ubuntu, CentOS, or Fedora.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct containing the Linux distribution information
///
/// # Returns
///
/// Returns `Ok(())` if the system is updated successfully, or an error if the update fails.
pub fn update_system(config: &Config) -> Result<(), Box<dyn Error>> {
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

/// Installs essential packages on the system.
///
/// This function installs a predefined list of essential packages using
/// the appropriate package manager for the Linux distribution.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct containing the Linux distribution information
///
/// # Returns
///
/// Returns `Ok(())` if all packages are installed successfully, or an error if installation fails.
pub fn install_essential_packages(config: &Config) -> Result<(), Box<dyn Error>> {
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

/// Sets up the firewall with basic rules and any custom rules specified in the configuration.
///
/// This function configures either UFW (for Ubuntu) or firewalld (for CentOS/Fedora)
/// with default deny incoming, allow outgoing policy, and opens ports for SSH and any custom rules.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct containing firewall configuration and Linux distribution information
///
/// # Returns
///
/// Returns `Ok(())` if the firewall is set up successfully, or an error if setup fails.
pub fn setup_firewall(config: &Config) -> Result<(), Box<dyn Error>> {
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

/// Configures SSH for improved security.
///
/// This function modifies the SSH configuration to:
/// - Disable root login
/// - Disable password authentication (requiring key-based authentication)
/// - Change the default SSH port (TODO: implement this securely)
///
/// After making changes, it restarts the SSH service to apply the new configuration.
///
/// # Returns
///
/// Returns `Ok(())` if SSH is configured successfully, or an error if configuration fails.
pub fn setup_ssh() -> Result<(), Box<dyn Error>> {
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
