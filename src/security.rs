use crate::config::Config;
use crate::distro::{get_package_manager, PackageManager};
use crate::rollback::RollbackManager;
use crate::utils::run_command;
use log::info;
use std::error::Error;

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

fn configure_fail2ban() -> Result<(), Box<dyn Error>> {
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

fn setup_advanced_security(config: &Config) -> Result<(), Box<dyn Error>> {
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

fn setup_rootkit_detection(config: &Config) -> Result<(), Box<dyn Error>> {
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

fn setup_security_scans() -> Result<(), Box<dyn Error>> {
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
