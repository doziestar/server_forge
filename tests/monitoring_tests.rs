use server_forge::config::Config;
use server_forge::monitoring;
use server_forge::rollback::RollbackManager;
use std::fs;

#[test]
fn test_install_monitoring_tools() {
    let config = Config {
        monitoring: true,
        ..Default::default()
    };

    assert!(monitoring::install_monitoring_tools(&config).is_ok());

    // Verify Prometheus installation
    let prometheus_status = std::process::Command::new("which")
        .arg("prometheus")
        .status()
        .unwrap();
    assert!(prometheus_status.success());

    // Verify Grafana installation
    let grafana_status = std::process::Command::new("which")
        .arg("grafana-server")
        .status()
        .unwrap();
    assert!(grafana_status.success());
}

#[test]
fn test_configure_prometheus() {
    assert!(monitoring::configure_prometheus().is_ok());

    // Verify Prometheus configuration
    let prometheus_config = fs::read_to_string("/etc/prometheus/prometheus.yml").unwrap();
    assert!(prometheus_config.contains("scrape_configs:"));
    assert!(prometheus_config.contains("job_name: 'node'"));

    // Verify Prometheus service is running
    let status = std::process::Command::new("systemctl")
        .args(&["is-active", "prometheus"])
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn test_setup_grafana() {
    assert!(monitoring::setup_grafana().is_ok());

    // Verify Grafana service is running
    let status = std::process::Command::new("systemctl")
        .args(&["is-active", "grafana-server"])
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn test_setup_node_exporter() {
    assert!(monitoring::setup_node_exporter().is_ok());

    // Verify Node Exporter service is running
    let status = std::process::Command::new("systemctl")
        .args(&["is-active", "node_exporter"])
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn test_setup_monitoring() {
    let config = Config {
        monitoring: true,
        ..Default::default()
    };
    let rollback_manager = RollbackManager::new();

    assert!(monitoring::setup_monitoring(&config, &rollback_manager).is_ok());
}
