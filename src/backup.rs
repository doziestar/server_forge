//! # Backup Module
//!
//! This module provides functionality for setting up and configuring an automated backup system
//! for a Linux server. It uses `restic` as the backup tool, which provides efficient, encrypted,
//! and incremental backups.
//!
//! The module includes functions for installing backup tools, configuring backup schedules,
//! and setting up backup locations based on the server's role.

use crate::config::Config;
use crate::distro::{get_package_manager, PackageManager};
use crate::rollback::RollbackManager;
use crate::utils::run_command;
use log::info;
use std::error::Error;

/// Sets up the backup system based on the provided configuration.
///
/// This function orchestrates the entire backup setup process, including:
/// - Installing necessary backup tools
/// - Configuring the backup schedule
/// - Setting up backup locations
///
/// It creates a snapshot before starting the setup process for potential rollback.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct containing backup configuration
/// * `rollback` - A reference to the `RollbackManager` for creating snapshots
///
/// # Returns
///
/// Returns `Ok(())` if the backup system is set up successfully, or an error if setup fails.
pub fn setup_backup_system(
    config: &Config,
    rollback: &RollbackManager,
) -> Result<(), Box<dyn Error>> {
    info!("Setting up backup system...");

    let snapshot = rollback.create_snapshot()?;

    install_backup_tools()?;
    configure_backup_schedule(config)?;
    setup_backup_locations(config)?;

    rollback.commit_snapshot(snapshot)?;

    info!("Backup system setup completed");
    Ok(())
}

/// Installs the necessary backup tools (restic) on the system.
///
/// This function uses the appropriate package manager for the current Linux distribution
/// to install restic.
///
/// # Returns
///
/// Returns `Ok(())` if restic is installed successfully, or an error if installation fails.
pub fn install_backup_tools() -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;
    match package_manager {
        PackageManager::Apt => run_command("apt", &["install", "-y", "restic"])?,
        PackageManager::Yum => run_command("yum", &["install", "-y", "restic"])?,
        PackageManager::Dnf => run_command("dnf", &["install", "-y", "restic"])?,
    }
    Ok(())
}

/// Configures the backup schedule based on the provided configuration.
///
/// This function creates a cron job for running backups at the specified frequency
/// (hourly, daily, or weekly).
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct containing the backup frequency
///
/// # Returns
///
/// Returns `Ok(())` if the backup schedule is configured successfully, or an error if configuration fails.
pub fn configure_backup_schedule(config: &Config) -> Result<(), Box<dyn Error>> {
    let cron_job = match config.backup_frequency.as_str() {
        "hourly" => {
            "0 * * * * root /usr/bin/restic backup /path/to/backup >> /var/log/restic.log 2>&1\n"
        }
        "daily" => {
            "0 2 * * * root /usr/bin/restic backup /path/to/backup >> /var/log/restic.log 2>&1\n"
        }
        "weekly" => {
            "0 2 * * 0 root /usr/bin/restic backup /path/to/backup >> /var/log/restic.log 2>&1\n"
        }
        _ => return Err("Invalid backup frequency".into()),
    };

    std::fs::write("/etc/cron.d/restic-backup", cron_job)?;
    Ok(())
}

/// Sets up backup locations based on the server's role.
///
/// This function determines which directories to back up based on the server's role
/// (web, database, or application server). It then initializes a restic repository
/// and creates a backup script that includes these locations.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct containing the server role
///
/// # Returns
///
/// Returns `Ok(())` if backup locations are set up successfully, or an error if setup fails.
pub fn setup_backup_locations(config: &Config) -> Result<(), Box<dyn Error>> {
    // Define backup locations based on server role
    let backup_dirs = match config.server_role.as_str() {
        "web" => vec!["/var/www", "/etc/nginx", "/etc/apache2"],
        "database" => vec!["/var/lib/mysql", "/var/lib/postgresql"],
        "application" => vec!["/opt/myapp", "/etc/myapp"],
        _ => vec![],
    };

    // Create restic repository
    run_command("restic", &["init", "--repo", "/path/to/backup/repository"])?;

    // Create backup script
    let mut backup_script = String::from("#!/bin/bash\n\n");
    backup_script.push_str("export RESTIC_PASSWORD='your_restic_password'\n\n");
    backup_script.push_str("restic backup");
    for dir in backup_dirs {
        backup_script.push_str(&format!(" {}", dir));
    }
    backup_script.push_str(" --tag serverforge\n");

    std::fs::write("/usr/local/bin/run-backup.sh", backup_script)?;
    run_command("chmod", &["+x", "/usr/local/bin/run-backup.sh"])?;

    Ok(())
}
