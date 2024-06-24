//! # Security Module
//!
//! This module provides functions for implementing various security measures on a Linux server.
//! It includes functionality for configuring Fail2Ban, setting up advanced security measures
//! (SELinux or AppArmor), implementing rootkit detection, and scheduling regular security scans.

use crate::config::Config;
use crate::distro::{get_package_manager, PackageManager};
use crate::rollback::RollbackManager;
use crate::utils::run_command;
use log::info;
use std::error::Error;

/// Implements all security measures based on the provided configuration.
///
/// This function orchestrates the implementation of various security measures including:
/// - Configuring Fail2Ban
/// - Setting up advanced security (SELinux or AppArmor)
/// - Setting up rootkit detection
/// - Configuring regular security scans
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct containing user-defined configuration options
/// * `rollback` - A reference to the `RollbackManager` for managing system state
///
/// # Errors
///
/// Returns an error if any of the security measures fail to implement
pub fn implement_security_measures(
    config: &Config,
    rollback: &RollbackManager,
) -> Result<(), Box<dyn Error>> {
    info!("Implementing security measures...");

    let snapshot = rollback.create_snapshot()?;

    configure_fail2ban()?;
    setup_advanced_security(config)?;
    setup_rootkit_detection(config)?;
    setup_security_scans()?;

    rollback.commit_snapshot(snapshot)?;

    info!("Security measures implemented");
    Ok(())
}

/// Configures and starts the Fail2Ban service.
///
/// This function installs Fail2Ban, creates a basic configuration for SSH,
/// and starts the Fail2Ban service.
///
/// # Errors
///
/// Returns an error if Fail2Ban installation or configuration fails
pub fn configure_fail2ban() -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;
    match package_manager {
        PackageManager::Apt => run_command("apt", &["install", "-y", "fail2ban"])?,
        PackageManager::Yum => run_command("yum", &["install", "-y", "fail2ban"])?,
        PackageManager::Dnf => run_command("dnf", &["install", "-y", "fail2ban"])?,
    }

    let fail2ban_config = r#"
[sshd]
enabled = true
port = ssh
filter = sshd
logpath = /var/log/auth.log
maxretry = 3
bantime = 3600
"#;
    std::fs::write("/etc/fail2ban/jail.local", fail2ban_config)?;

    run_command("systemctl", &["enable", "fail2ban"])?;
    run_command("systemctl", &["start", "fail2ban"])?;

    Ok(())
}

/// Sets up advanced security measures based on the Linux distribution.
///
/// For Ubuntu, this function sets up AppArmor.
/// For CentOS or Fedora, this function sets up SELinux.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct containing user-defined configuration options
///
/// # Errors
///
/// Returns an error if the setup fails or if the Linux distribution is not supported
pub fn setup_advanced_security(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.security_level == "advanced" {
        // Enable and configure SELinux or AppArmor based on the distribution
        match config.linux_distro.as_str() {
            "ubuntu" => {
                run_command("apt", &["install", "-y", "apparmor", "apparmor-utils"])?;
                run_command("aa-enforce", &["/etc/apparmor.d/*"])?;
            }
            "centos" | "fedora" => {
                run_command(
                    "yum",
                    &["install", "-y", "selinux-policy", "selinux-policy-targeted"],
                )?;
                std::fs::write(
                    "/etc/selinux/config",
                    "SELINUX=enforcing\nSELINUXTYPE=targeted\n",
                )?;
            }
            _ => return Err("Unsupported Linux distribution for advanced security".into()),
        }
    }
    Ok(())
}

/// Sets up rootkit detection tools (rkhunter and chkrootkit).
///
/// This function installs rkhunter and chkrootkit, then updates the rkhunter database.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct (unused in the current implementation)
///
/// # Errors
///
/// Returns an error if installation or configuration of rootkit detection tools fails
pub fn setup_rootkit_detection(config: &Config) -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;
    match package_manager {
        PackageManager::Apt => run_command("apt", &["install", "-y", "rkhunter", "chkrootkit"])?,
        PackageManager::Yum => run_command("yum", &["install", "-y", "rkhunter", "chkrootkit"])?,
        PackageManager::Dnf => run_command("dnf", &["install", "-y", "rkhunter", "chkrootkit"])?,
    }

    // Update rkhunter database
    run_command("rkhunter", &["--update"])?;
    run_command("rkhunter", &["--propupd"])?;

    Ok(())
}

/// Sets up regular security scans using rkhunter and chkrootkit.
///
/// This function creates a script to run both rkhunter and chkrootkit,
/// then sets up a weekly cron job to execute this script.
///
/// # Errors
///
/// Returns an error if creating the script or setting up the cron job fails
pub fn setup_security_scans() -> Result<(), Box<dyn Error>> {
    let scan_script = r#"#!/bin/bash
rkhunter --check --skip-keypress
chkrootkit
"#;
    std::fs::write("/usr/local/bin/security_scan.sh", scan_script)?;
    run_command("chmod", &["+x", "/usr/local/bin/security_scan.sh"])?;

    // Add weekly cron job for security scans
    let cron_job =
        "0 2 * * 0 root /usr/local/bin/security_scan.sh > /var/log/security_scan.log 2>&1\n";
    std::fs::write("/etc/cron.d/security_scan", cron_job)?;

    Ok(())
}
