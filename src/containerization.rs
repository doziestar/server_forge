//! # Containerization Module
//!
//! This module provides functionality for setting up and managing containerization
//! technologies, specifically Docker and Kubernetes, on a Linux server. It includes
//! functions for installation, configuration, and deployment of containerized applications.
//!
//! The module is designed to work across different Linux distributions by leveraging
//! the appropriate package manager and installation methods for each system.

use crate::config::Config;
use crate::distro::{get_package_manager, PackageManager};
use crate::rollback::RollbackManager;
use crate::utils::run_command;
use log::info;
use std::error::Error;

/// Sets up Docker on the system.
///
/// This function installs Docker, configures it, and ensures it's running and enabled on boot.
/// It creates a snapshot before installation for potential rollback.
///
/// # Arguments
///
/// * `rollback` - A reference to the `RollbackManager` for creating snapshots
///
/// # Returns
///
/// Returns `Ok(())` if Docker is set up successfully, or an error if setup fails.
pub fn setup_docker(rollback: &RollbackManager) -> Result<(), Box<dyn Error>> {
    info!("Setting up Docker...");

    let snapshot = rollback.create_snapshot()?;

    install_docker()?;
    configure_docker()?;

    rollback.commit_snapshot(snapshot)?;

    info!("Docker setup completed");
    Ok(())
}

/// Sets up Kubernetes on the system.
///
/// This function installs Kubernetes tools (kubectl and minikube), configures them,
/// and ensures they're ready for use. It creates a snapshot before installation for potential rollback.
///
/// # Arguments
///
/// * `rollback` - A reference to the `RollbackManager` for creating snapshots
///
/// # Returns
///
/// Returns `Ok(())` if Kubernetes is set up successfully, or an error if setup fails.
pub fn setup_kubernetes(rollback: &RollbackManager) -> Result<(), Box<dyn Error>> {
    info!("Setting up Kubernetes...");

    let snapshot = rollback.create_snapshot()?;

    install_kubernetes()?;
    configure_kubernetes()?;

    rollback.commit_snapshot(snapshot)?;

    info!("Kubernetes setup completed");
    Ok(())
}

/// Deploys containers for all applications specified in the configuration.
///
/// This function iterates through the list of applications in the configuration
/// and deploys each as a container, either using Docker or Kubernetes based on the configuration.
///
/// # Arguments
///
/// * `config` - A reference to the `Config` struct containing deployment information
/// * `rollback` - A reference to the `RollbackManager` for creating snapshots
///
/// # Returns
///
/// Returns `Ok(())` if all containers are deployed successfully, or an error if any deployment fails.
pub fn deploy_containers(
    config: &Config,
    rollback: &RollbackManager,
) -> Result<(), Box<dyn Error>> {
    info!("Deploying containers...");
    let snapshot = rollback.create_snapshot()?;

    for app in &config.deployed_apps {
        deploy_container(app, config.use_kubernetes)?;
    }

    rollback.commit_snapshot(snapshot)?;

    info!("Container deployment completed");
    Ok(())
}

/// Installs Docker on the system.
///
/// This function installs Docker using the appropriate method for the current Linux distribution.
/// It adds the Docker repository, installs necessary dependencies, and installs Docker components.
///
/// # Returns
///
/// Returns `Ok(())` if Docker is installed successfully, or an error if installation fails.
pub fn install_docker() -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;

    match package_manager {
        PackageManager::Apt => {
            run_command("apt", &["update"])?;
            run_command(
                "apt",
                &[
                    "install",
                    "-y",
                    "apt-transport-https",
                    "ca-certificates",
                    "curl",
                    "gnupg",
                    "lsb-release",
                ],
            )?;
            run_command(
                "curl",
                &[
                    "-fsSL",
                    "https://download.docker.com/linux/ubuntu/gpg",
                    "|",
                    "gpg",
                    "--dearmor",
                    "-o",
                    "/usr/share/keyrings/docker-archive-keyring.gpg",
                ],
            )?;
            run_command("echo", &["\"deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable\"", "|", "tee", "/etc/apt/sources.list.d/docker.list", ">", "/dev/null"])?;
            run_command("apt", &["update"])?;
            run_command(
                "apt",
                &[
                    "install",
                    "-y",
                    "docker-ce",
                    "docker-ce-cli",
                    "containerd.io",
                ],
            )?;
        }
        PackageManager::Yum => {
            run_command("yum", &["install", "-y", "yum-utils"])?;
            run_command(
                "yum-config-manager",
                &[
                    "--add-repo",
                    "https://download.docker.com/linux/centos/docker-ce.repo",
                ],
            )?;
            run_command(
                "yum",
                &[
                    "install",
                    "-y",
                    "docker-ce",
                    "docker-ce-cli",
                    "containerd.io",
                ],
            )?;
        }
        PackageManager::Dnf => {
            run_command("dnf", &["install", "-y", "dnf-plugins-core"])?;
            run_command(
                "dnf",
                &[
                    "config-manager",
                    "--add-repo",
                    "https://download.docker.com/linux/fedora/docker-ce.repo",
                ],
            )?;
            run_command(
                "dnf",
                &[
                    "install",
                    "-y",
                    "docker-ce",
                    "docker-ce-cli",
                    "containerd.io",
                ],
            )?;
        }
    }

    run_command("systemctl", &["start", "docker"])?;
    run_command("systemctl", &["enable", "docker"])?;

    Ok(())
}

/// Configures Docker after installation.
///
/// This function sets up the Docker daemon with optimal settings, creates a Docker group,
/// adds the current user to the Docker group, and restarts the Docker service to apply changes.
///
/// # Returns
///
/// Returns `Ok(())` if Docker is configured successfully, or an error if configuration fails.
pub fn configure_docker() -> Result<(), Box<dyn Error>> {
    // Create docker group if it doesn't exist
    run_command("groupadd", &["docker"])?;

    // Add current user to docker group
    run_command("usermod", &["-aG", "docker", "$USER"])?;

    // Set up Docker daemon configuration
    let daemon_config = r#"
{
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "100m",
    "max-file": "3"
  },
  "default-ulimits": {
    "nofile": {
      "Name": "nofile",
      "Hard": 64000,
      "Soft": 64000
    }
  }
}
"#;
    std::fs::write("/etc/docker/daemon.json", daemon_config)?;

    // Restart Docker to apply changes
    run_command("systemctl", &["restart", "docker"])?;

    Ok(())
}

/// Installs Kubernetes tools (kubectl and minikube) on the system.
///
/// This function downloads and installs kubectl and minikube, and installs a virtualization
/// driver (VirtualBox in this implementation) required for running Kubernetes locally.
///
/// # Returns
///
/// Returns `Ok(())` if Kubernetes tools are installed successfully, or an error if installation fails.
pub fn install_kubernetes() -> Result<(), Box<dyn Error>> {
    let package_manager = get_package_manager()?;

    // Install kubectl
    run_command("curl", &["-LO", "https://storage.googleapis.com/kubernetes-release/release/$(curl -s https://storage.googleapis.com/kubernetes-release/release/stable.txt)/bin/linux/amd64/kubectl"])?;
    run_command("chmod", &["+x", "./kubectl"])?;
    run_command("mv", &["./kubectl", "/usr/local/bin/kubectl"])?;

    // Install minikube
    run_command(
        "curl",
        &[
            "-Lo",
            "minikube",
            "https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64",
        ],
    )?;
    run_command("chmod", &["+x", "minikube"])?;
    run_command("mv", &["minikube", "/usr/local/bin/"])?;

    // Install required virtualization driver (using VirtualBox in this example)
    match package_manager {
        PackageManager::Apt => run_command("apt", &["install", "-y", "virtualbox"])?,
        PackageManager::Yum => run_command("yum", &["install", "-y", "VirtualBox"])?,
        PackageManager::Dnf => run_command("dnf", &["install", "-y", "VirtualBox"])?,
    }

    Ok(())
}

/// Configures Kubernetes after installation.
///
/// This function starts minikube, enables necessary addons (ingress and dashboard),
/// and sets up kubectl autocomplete for easier use.
///
/// # Returns
///
/// Returns `Ok(())` if Kubernetes is configured successfully, or an error if configuration fails.
pub fn configure_kubernetes() -> Result<(), Box<dyn Error>> {
    // Start minikube
    run_command("minikube", &["start"])?;

    // Enable necessary addons
    run_command("minikube", &["addons", "enable", "ingress"])?;
    run_command("minikube", &["addons", "enable", "dashboard"])?;

    // Set up kubectl autocomplete
    run_command(
        "kubectl",
        &["completion", "bash", ">", "/etc/bash_completion.d/kubectl"],
    )?;

    Ok(())
}

/// Deploys a single container for the specified application.
///
/// This function deploys the application either to Kubernetes or directly to Docker,
/// based on the `use_kubernetes` flag.
///
/// # Arguments
///
/// * `app` - A string slice representing the application to deploy
/// * `use_kubernetes` - A boolean indicating whether to use Kubernetes for deployment
///
/// # Returns
///
/// Returns `Ok(())` if the container is deployed successfully, or an error if deployment fails.
pub fn deploy_container(app: &str, use_kubernetes: bool) -> Result<(), Box<dyn Error>> {
    if use_kubernetes {
        deploy_to_kubernetes(app)?;
    } else {
        deploy_to_docker(app)?;
    }
    Ok(())
}

/// Deploys a single container for the specified application.
///
/// This function deploys the application either to Kubernetes or directly to Docker,
/// based on the `use_kubernetes` flag.
///
/// # Arguments
///
/// * `app` - A string slice representing the application to deploy
/// * `use_kubernetes` - A boolean indicating whether to use Kubernetes for deployment
///
/// # Returns
///
/// Returns `Ok(())` if the container is deployed successfully, or an error if deployment fails.
pub fn deploy_to_kubernetes(app: &str) -> Result<(), Box<dyn Error>> {
    // Create a basic deployment YAML
    let deployment_yaml = format!(
        r#"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {}
spec:
  replicas: 1
  selector:
    matchLabels:
      app: {}
  template:
    metadata:
      labels:
        app: {}
    spec:
      containers:
      - name: {}
        image: {}:latest
        ports:
        - containerPort: 80
"#,
        app, app, app, app, app
    );

    // Write the deployment YAML to a file
    std::fs::write(format!("{}-deployment.yaml", app), deployment_yaml)?;

    // Apply the deployment
    run_command(
        "kubectl",
        &["apply", "-f", &format!("{}-deployment.yaml", app)],
    )?;

    // Expose the deployment as a service
    run_command(
        "kubectl",
        &[
            "expose",
            "deployment",
            app,
            "--type=LoadBalancer",
            "--port=80",
        ],
    )?;

    Ok(())
}

/// Deploys an application to Kubernetes.
///
/// This function creates a Kubernetes Deployment and Service for the specified application.
/// It generates a basic YAML configuration, applies it to the cluster, and exposes the deployment as a service.
///
/// # Arguments
///
/// * `app` - A string slice representing the application to deploy
///
/// # Returns
///
/// Returns `Ok(())` if the application is deployed to Kubernetes successfully, or an error if deployment fails.
pub fn deploy_to_docker(app: &str) -> Result<(), Box<dyn Error>> {
    // Pull the latest image
    run_command("docker", &["pull", app])?;

    // Stop and remove any existing container with the same name
    run_command("docker", &["stop", app]).ok();
    run_command("docker", &["rm", app]).ok();

    // Run the new container
    run_command("docker", &["run", "-d", "--name", app, "-p", "80:80", app])?;

    Ok(())
}
