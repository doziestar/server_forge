use crate::config::Config;
use crate::distro::{get_package_manager, PackageManager};
use crate::rollback::RollbackManager;
use crate::utils::run_command;
use log::info;
use std::error::Error;

pub fn setup_docker(rollback: &RollbackManager) -> Result<(), Box<dyn Error>> {
    info!("Setting up Docker...");

    let snapshot = rollback.create_snapshot()?;

    install_docker()?;
    configure_docker()?;

    rollback.commit_snapshot(snapshot)?;

    info!("Docker setup completed");
    Ok(())
}

pub fn setup_kubernetes(rollback: &RollbackManager) -> Result<(), Box<dyn Error>> {
    info!("Setting up Kubernetes...");

    let snapshot = rollback.create_snapshot()?;

    install_kubernetes()?;
    configure_kubernetes()?;

    rollback.commit_snapshot(snapshot)?;

    info!("Kubernetes setup completed");
    Ok(())
}

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

fn install_docker() -> Result<(), Box<dyn Error>> {
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

fn configure_docker() -> Result<(), Box<dyn Error>> {
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

fn install_kubernetes() -> Result<(), Box<dyn Error>> {
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

fn configure_kubernetes() -> Result<(), Box<dyn Error>> {
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

fn deploy_container(app: &str, use_kubernetes: bool) -> Result<(), Box<dyn Error>> {
    if use_kubernetes {
        deploy_to_kubernetes(app)?;
    } else {
        deploy_to_docker(app)?;
    }
    Ok(())
}

fn deploy_to_kubernetes(app: &str) -> Result<(), Box<dyn Error>> {
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

fn deploy_to_docker(app: &str) -> Result<(), Box<dyn Error>> {
    // Pull the latest image
    run_command("docker", &["pull", app])?;

    // Stop and remove any existing container with the same name
    run_command("docker", &["stop", app]).ok();
    run_command("docker", &["rm", app]).ok();

    // Run the new container
    run_command("docker", &["run", "-d", "--name", app, "-p", "80:80", app])?;

    Ok(())
}
