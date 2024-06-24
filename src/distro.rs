//! # Distribution Module
//!
//! This module provides functionality for interacting with different Linux distributions
//! and their package managers. It includes functions for detecting the package manager,
//! updating the system, and installing or uninstalling packages.

use std::error::Error;
use std::path::Path;

/// Represents the different package managers supported by the application.
#[derive(Debug, PartialEq)]
pub enum PackageManager {
    Apt, // For Debian-based distributions (e.g., Ubuntu)
    Yum, // For older Red Hat-based distributions
    Dnf, // For newer Red Hat-based distributions (e.g., Fedora)
}

/// Detects the package manager used by the current system.
///
/// This function checks for the existence of specific package manager
/// executables to determine which one is available on the system.
///
/// # Returns
///
/// Returns a `Result` containing the detected `PackageManager` or an error
/// if no supported package manager is found.
pub fn get_package_manager() -> Result<PackageManager, Box<dyn Error>> {
    if Path::new("/usr/bin/apt").exists() {
        Ok(PackageManager::Apt)
    } else if Path::new("/usr/bin/yum").exists() {
        Ok(PackageManager::Yum)
    } else if Path::new("/usr/bin/dnf").exists() {
        Ok(PackageManager::Dnf)
    } else {
        Err("Unsupported package manager".into())
    }
}

/// Updates the system using the specified package manager.
///
/// This function runs the appropriate update commands for the given package manager.
///
/// # Arguments
///
/// * `package_manager` - A reference to the `PackageManager` enum representing the system's package manager.
///
/// # Returns
///
/// Returns a `Result` indicating success or an error if the update process fails.
pub fn update_system(package_manager: &PackageManager) -> Result<(), Box<dyn Error>> {
    match package_manager {
        PackageManager::Apt => {
            crate::utils::run_command("apt", &["update"])?;
            crate::utils::run_command("apt", &["upgrade", "-y"])?;
        }
        PackageManager::Yum => {
            crate::utils::run_command("yum", &["update", "-y"])?;
        }
        PackageManager::Dnf => {
            crate::utils::run_command("dnf", &["upgrade", "-y"])?;
        }
    }
    Ok(())
}

/// Installs a package using the specified package manager.
///
/// This function runs the appropriate install command for the given package manager.
///
/// # Arguments
///
/// * `package_manager` - A reference to the `PackageManager` enum representing the system's package manager.
/// * `package` - A string slice containing the name of the package to install.
///
/// # Returns
///
/// Returns a `Result` indicating success or an error if the installation process fails.
pub fn install_package(
    package_manager: &PackageManager,
    package: &str,
) -> Result<(), Box<dyn Error>> {
    match package_manager {
        PackageManager::Apt => crate::utils::run_command("apt", &["install", "-y", package])?,
        PackageManager::Yum => crate::utils::run_command("yum", &["install", "-y", package])?,
        PackageManager::Dnf => crate::utils::run_command("dnf", &["install", "-y", package])?,
    }
    Ok(())
}

/// Uninstalls a package using the specified package manager.
///
/// This function runs the appropriate remove command for the given package manager.
///
/// # Arguments
///
/// * `package_manager` - A reference to the `PackageManager` enum representing the system's package manager.
/// * `package` - A string slice containing the name of the package to uninstall.
///
/// # Returns
///
/// Returns a `Result` indicating success or an error if the uninstallation process fails.
pub fn uninstall_package(
    package_manager: &PackageManager,
    package: &str,
) -> Result<(), Box<dyn Error>> {
    match package_manager {
        PackageManager::Apt => crate::utils::run_command("apt", &["remove", "-y", package])?,
        PackageManager::Yum => crate::utils::run_command("yum", &["remove", "-y", package])?,
        PackageManager::Dnf => crate::utils::run_command("dnf", &["remove", "-y", package])?,
    }
    Ok(())
}
