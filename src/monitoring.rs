//! # Monitoring Module
//!
//! This module provides functionality for setting up a comprehensive monitoring system
//! using Prometheus, Grafana, and Node Exporter. It handles the installation, configuration,
//! and deployment of these tools across different Linux distributions.

use crate::config::Config;
use crate::distro::{get_package_manager, PackageManager};
use crate::rollback::RollbackManager;
use crate::utils::run_command;
use log::info;
use std::error::Error;

/// Sets up the monitoring system based on the provided configuration.
///
/// This function orchestrates the installation and configuration of Prometheus, Grafana,
/// and Node Exporter. If monitoring is disabled in the configuration, it skips the setup.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct containing user-defined configuration options
/// * `rollback` - A reference to the `RollbackManager` for managing system state
///
/// # Errors
///
/// Returns an error if any part of the monitoring setup process fails.
pub fn setup_monitoring(config: &Config, rollback: &RollbackManager) -> Result<(), Box<dyn Error>> {
    if config.monitoring {
        info!("Setting up monitoring...");

        let snapshot = rollback.create_snapshot()?;

        install_monitoring_tools(config)?;
        configure_prometheus()?;
        setup_grafana()?;
        setup_node_exporter()?;

        rollback.commit_snapshot(snapshot)?;

        info!("Monitoring setup completed");
    } else {
        info!("Monitoring setup skipped as per user preference");
    }
    Ok(())
}

/// Installs Prometheus and Grafana based on the system's package manager.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct (unused in the current implementation)
///
/// # Errors
///
/// Returns an error if the installation of either Prometheus or Grafana fails.
pub fn install_monitoring_tools(config: &Config) -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;

    // Install Prometheus
    match package_manager {
        PackageManager::Apt => {
            run_command("apt", &["update"])?;
            run_command("apt", &["install", "-y", "prometheus"])?;
        }
        PackageManager::Yum | PackageManager::Dnf => {
            // For CentOS/Fedora, we need to install from source
            install_prometheus_from_source()?;
        }
    }

    // Install Grafana
    match package_manager {
        PackageManager::Apt => {
            run_command(
                "apt",
                &[
                    "install",
                    "-y",
                    "apt-transport-https",
                    "software-properties-common",
                    "wget",
                ],
            )?;
            run_command(
                "wget",
                &[
                    "-q",
                    "-O",
                    "/usr/share/keyrings/grafana.key",
                    "https://packages.grafana.com/gpg.key",
                ],
            )?;
            run_command("echo", &["deb [signed-by=/usr/share/keyrings/grafana.key] https://packages.grafana.com/oss/deb stable main", ">", "/etc/apt/sources.list.d/grafana.list"])?;
            run_command("apt", &["update"])?;
            run_command("apt", &["install", "-y", "grafana"])?;
        }
        PackageManager::Yum | PackageManager::Dnf => {
            run_command(
                "wget",
                &[
                    "-q",
                    "-O",
                    "/etc/yum.repos.d/grafana.repo",
                    "https://packages.grafana.com/oss/rpm/grafana.repo",
                ],
            )?;
            match package_manager {
                PackageManager::Yum => run_command("yum", &["install", "-y", "grafana"])?,
                PackageManager::Dnf => run_command("dnf", &["install", "-y", "grafana"])?,
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}

/// Configures Prometheus with a basic scrape configuration.
///
/// This function creates a basic Prometheus configuration file and
/// restarts the Prometheus service.
///
/// # Errors
///
/// Returns an error if writing the configuration file or restarting the service fails.
pub fn configure_prometheus() -> Result<(), Box<dyn Error>> {
    let prometheus_config = r#"
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'node'
    static_configs:
      - targets: ['localhost:9100']
"#;
    std::fs::write("/etc/prometheus/prometheus.yml", prometheus_config)?;

    run_command("systemctl", &["restart", "prometheus"])?;
    run_command("systemctl", &["enable", "prometheus"])?;

    Ok(())
}

/// Sets up and starts the Grafana server.
///
/// This function starts the Grafana server and enables it to start on boot.
/// Additional configuration (like adding data sources or creating dashboards)
/// could be added here in the future.
///
/// # Errors
///
/// Returns an error if starting or enabling the Grafana service fails.
pub fn setup_grafana() -> Result<(), Box<dyn Error>> {
    run_command("systemctl", &["start", "grafana-server"])?;
    run_command("systemctl", &["enable", "grafana-server"])?;

    // Here we will add code to configure Grafana via its API
    // For example, adding data sources, creating dashboards, etc.

    Ok(())
}

/// Sets up and starts the Node Exporter.
///
/// This function installs Node Exporter (either via package manager or from source),
/// starts the Node Exporter service, and enables it to start on boot.
///
/// # Errors
///
/// Returns an error if installation, starting, or enabling the Node Exporter service fails.
pub fn setup_node_exporter() -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;

    match package_manager {
        PackageManager::Apt => {
            run_command("apt", &["install", "-y", "prometheus-node-exporter"])?;
        }
        PackageManager::Yum | PackageManager::Dnf => {
            // For CentOS/Fedora, we need to install from source
            install_node_exporter_from_source()?;
        }
    }

    run_command("systemctl", &["start", "node_exporter"])?;
    run_command("systemctl", &["enable", "node_exporter"])?;

    Ok(())
}

/// Installs Prometheus from source.
///
/// This function is used for systems where Prometheus is not available
/// through the package manager (e.g., CentOS, Fedora).
///
/// # Errors
///
/// Returns an error if any step of the source installation process fails.
pub fn install_prometheus_from_source() -> Result<(), Box<dyn Error>> {
    run_command("wget", &["https://github.com/prometheus/prometheus/releases/download/v2.30.3/prometheus-2.30.3.linux-amd64.tar.gz"])?;
    run_command("tar", &["xvfz", "prometheus-2.30.3.linux-amd64.tar.gz"])?;
    run_command("mv", &["prometheus-2.30.3.linux-amd64", "prometheus"])?;

    // Create Prometheus user
    run_command(
        "useradd",
        &["--no-create-home", "--shell", "/bin/false", "prometheus"],
    )?;

    // Create directories and set ownership
    run_command("mkdir", &["/etc/prometheus", "/var/lib/prometheus"])?;
    run_command(
        "chown",
        &[
            "prometheus:prometheus",
            "/etc/prometheus",
            "/var/lib/prometheus",
        ],
    )?;

    // Move binaries and set ownership
    run_command(
        "mv",
        &[
            "prometheus/prometheus",
            "prometheus/promtool",
            "/usr/local/bin/",
        ],
    )?;
    run_command(
        "chown",
        &[
            "prometheus:prometheus",
            "/usr/local/bin/prometheus",
            "/usr/local/bin/promtool",
        ],
    )?;

    // Move config files and set ownership
    run_command(
        "mv",
        &[
            "prometheus/consoles",
            "prometheus/console_libraries",
            "/etc/prometheus/",
        ],
    )?;
    run_command(
        "mv",
        &[
            "prometheus/prometheus.yml",
            "/etc/prometheus/prometheus.yml",
        ],
    )?;
    run_command("chown", &["-R", "prometheus:prometheus", "/etc/prometheus"])?;

    // Create systemd service file
    let service_file = r#"[Unit]
Description=Prometheus
Wants=network-online.target
After=network-online.target

[Service]
User=prometheus
Group=prometheus
Type=simple
ExecStart=/usr/local/bin/prometheus \
    --config.file /etc/prometheus/prometheus.yml \
    --storage.tsdb.path /var/lib/prometheus/ \
    --web.console.templates=/etc/prometheus/consoles \
    --web.console.libraries=/etc/prometheus/console_libraries

[Install]
WantedBy=multi-user.target
"#;
    std::fs::write("/etc/systemd/system/prometheus.service", service_file)?;

    run_command("systemctl", &["daemon-reload"])?;

    Ok(())
}

/// Installs Node Exporter from source.
///
/// This function is used for systems where Node Exporter is not available
/// through the package manager (e.g., CentOS, Fedora).
///
/// # Errors
///
/// Returns an error if any step of the source installation process fails.
pub fn install_node_exporter_from_source() -> Result<(), Box<dyn Error>> {
    run_command("wget", &["https://github.com/prometheus/node_exporter/releases/download/v1.2.2/node_exporter-1.2.2.linux-amd64.tar.gz"])?;
    run_command("tar", &["xvfz", "node_exporter-1.2.2.linux-amd64.tar.gz"])?;

    // Create Node Exporter user
    run_command(
        "useradd",
        &["--no-create-home", "--shell", "/bin/false", "node_exporter"],
    )?;

    // Move binary and set ownership
    run_command(
        "mv",
        &[
            "node_exporter-1.2.2.linux-amd64/node_exporter",
            "/usr/local/bin/",
        ],
    )?;
    run_command(
        "chown",
        &[
            "node_exporter:node_exporter",
            "/usr/local/bin/node_exporter",
        ],
    )?;

    // Create systemd service file
    let service_file = r#"[Unit]
Description=Node Exporter
Wants=network-online.target
After=network-online.target

[Service]
User=node_exporter
Group=node_exporter
Type=simple
ExecStart=/usr/local/bin/node_exporter

[Install]
WantedBy=multi-user.target
"#;
    std::fs::write("/etc/systemd/system/node_exporter.service", service_file)?;

    run_command("systemctl", &["daemon-reload"])?;

    Ok(())
}
