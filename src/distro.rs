use std::error::Error;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum PackageManager {
    Apt,
    Yum,
    Dnf,
}

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
