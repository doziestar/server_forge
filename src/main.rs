//! # Server Forge
//!
//! `server_forge` is a comprehensive server setup and maintenance tool designed to automate
//! the process of configuring, securing, and managing Linux servers.
//!
//! This module contains the main entry point for the application and orchestrates the
//! various setup and configuration processes.

use log::{error, info};
use std::error::Error;

mod backup;
mod config;
mod containerization;
mod deployment;
mod monitoring;
mod rollback;
mod security;
mod setup;
mod updates;
mod utils;

mod distro;

use rollback::RollbackManager;
use utils::{generate_report, get_user_input, save_config, setup_logging};

/// The main entry point for the Server Forge application.
///
/// This function orchestrates the entire server setup process, including:
/// - Initial setup
/// - Security measures implementation
/// - Automatic updates configuration
/// - Monitoring setup
/// - Backup system configuration
/// - Container or application deployment
///
/// If any step fails, it attempts to rollback all changes made.
///
/// # Errors
///
/// Returns an error if any step in the process fails.
fn main() -> Result<(), Box<dyn Error>> {
    // Set up logging for the application
    setup_logging()?;
    info!("Server Setup and Maintenance Script started");

    // Get user input for configuration
    let config = get_user_input()?;
    save_config(&config)?;

    // Initialize the rollback manager
    let rollback = RollbackManager::new();

    // Perform initial setup
    if let Err(e) = setup::initial_setup(&config, &rollback) {
        error!("Error during initial setup: {}", e);
        rollback.rollback_all()?;
        return Err("Setup failed".into());
    }

    // Implement security measures
    if let Err(e) = security::implement_security_measures(&config, &rollback) {
        error!("Error implementing security measures: {}", e);
        rollback.rollback_all()?;
        return Err("Security implementation failed".into());
    }

    // Set up automatic updates
    if let Err(e) = updates::setup_automatic_updates(&config, &rollback) {
        error!("Error setting up automatic updates: {}", e);
        rollback.rollback_all()?;
        return Err("Update setup failed".into());
    }

    // Set up monitoring
    if let Err(e) = monitoring::setup_monitoring(&config, &rollback) {
        error!("Error setting up monitoring: {}", e);
        rollback.rollback_all()?;
        return Err("Monitoring setup failed".into());
    }

    // Set up backup system
    if let Err(e) = backup::setup_backup_system(&config, &rollback) {
        error!("Error setting up backup system: {}", e);
        rollback.rollback_all()?;
        return Err("Backup setup failed".into());
    }

    // Deploy containers or applications based on configuration
    if config.use_containers {
        if let Err(e) = containerization::setup_docker(&rollback) {
            error!("Error setting up Docker: {}", e);
            rollback.rollback_all()?;
            return Err("Docker setup failed".into());
        }

        if config.use_kubernetes {
            if let Err(e) = containerization::setup_kubernetes(&rollback) {
                error!("Error setting up Kubernetes: {}", e);
                rollback.rollback_all()?;
                return Err("Kubernetes setup failed".into());
            }
        }

        if let Err(e) = containerization::deploy_containers(&config, &rollback) {
            error!("Error deploying containers: {}", e);
            rollback.rollback_all()?;
            return Err("Container deployment failed".into());
        }
    } else if let Err(e) = deployment::deploy_applications(&config, &rollback) {
        error!("Error deploying applications: {}", e);
        rollback.rollback_all()?;
        return Err("Application deployment failed".into());
    }

    info!("Server setup completed successfully");
    generate_report(&config)?;
    Ok(())
}
