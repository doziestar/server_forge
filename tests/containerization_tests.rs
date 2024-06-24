use server_forge::config::Config;
use server_forge::containerization;
use server_forge::rollback::RollbackManager;
use std::fs;

#[test]
fn test_install_docker() {
    assert!(containerization::install_docker().is_ok());

    // Verify Docker installation
    let docker_status = std::process::Command::new("docker")
        .arg("--version")
        .status()
        .unwrap();
    assert!(docker_status.success());

    // Verify Docker service is running
    let service_status = std::process::Command::new("systemctl")
        .args(&["is-active", "docker"])
        .status()
        .unwrap();
    assert!(service_status.success());
}

#[test]
fn test_configure_docker() {
    assert!(containerization::configure_docker().is_ok());

    // Verify Docker daemon configuration
    let daemon_config = fs::read_to_string("/etc/docker/daemon.json").unwrap();
    assert!(daemon_config.contains("log-driver"));
    assert!(daemon_config.contains("max-size"));
    assert!(daemon_config.contains("default-ulimits"));
}

#[test]
fn test_install_kubernetes() {
    assert!(containerization::install_kubernetes().is_ok());

    // Verify kubectl installation
    let kubectl_status = std::process::Command::new("kubectl")
        .arg("version")
        .arg("--client")
        .status()
        .unwrap();
    assert!(kubectl_status.success());

    // Verify minikube installation
    let minikube_status = std::process::Command::new("minikube")
        .arg("version")
        .status()
        .unwrap();
    assert!(minikube_status.success());
}

#[test]
fn test_configure_kubernetes() {
    assert!(containerization::configure_kubernetes().is_ok());

    // Verify minikube is running
    let minikube_status = std::process::Command::new("minikube")
        .arg("status")
        .status()
        .unwrap();
    assert!(minikube_status.success());

    // Verify ingress addon is enabled
    let ingress_status = std::process::Command::new("minikube")
        .args(&["addons", "list"])
        .output()
        .unwrap();
    let ingress_output = String::from_utf8_lossy(&ingress_status.stdout);
    assert!(ingress_output.contains("ingress: enabled"));
}

#[test]
fn test_deploy_to_docker() {
    let test_app = "nginx";
    assert!(containerization::deploy_to_docker(test_app).is_ok());

    // Verify container is running
    let container_status = std::process::Command::new("docker")
        .args(&["ps", "-q", "-f", &format!("name={}", test_app)])
        .output()
        .unwrap();
    assert!(!container_status.stdout.is_empty());
}

#[test]
fn test_deploy_to_kubernetes() {
    let test_app = "nginx";
    assert!(containerization::deploy_to_kubernetes(test_app).is_ok());

    // Verify deployment is created
    let deployment_status = std::process::Command::new("kubectl")
        .args(&["get", "deployment", test_app])
        .status()
        .unwrap();
    assert!(deployment_status.success());

    // Verify service is created
    let service_status = std::process::Command::new("kubectl")
        .args(&["get", "service", test_app])
        .status()
        .unwrap();
    assert!(service_status.success());
}

#[test]
fn test_setup_docker() {
    let rollback_manager = RollbackManager::new();
    assert!(containerization::setup_docker(&rollback_manager).is_ok());

    // Verify Docker is installed and configured
    assert!(std::process::Command::new("docker")
        .arg("info")
        .status()
        .unwrap()
        .success());
}

#[test]
fn test_setup_kubernetes() {
    let rollback_manager = RollbackManager::new();
    assert!(containerization::setup_kubernetes(&rollback_manager).is_ok());

    // Verify Kubernetes is installed and configured
    assert!(std::process::Command::new("kubectl")
        .arg("cluster-info")
        .status()
        .unwrap()
        .success());
}

#[test]
fn test_deploy_containers() {
    let config = Config {
        deployed_apps: vec![String::from("nginx"), String::from("redis")],
        use_kubernetes: true,
        ..Default::default()
    };
    let rollback_manager = RollbackManager::new();

    assert!(containerization::deploy_containers(&config, &rollback_manager).is_ok());

    // Verify deployments are created
    for app in &config.deployed_apps {
        let deployment_status = std::process::Command::new("kubectl")
            .args(&["get", "deployment", app])
            .status()
            .unwrap();
        assert!(deployment_status.success());
    }
}
