use server_forge::config::Config;
use server_forge::rollback::RollbackManager;
use server_forge::setup;
use std::fs;

#[test]
fn test_update_system() {
    let config = Config {
        linux_distro: String::from("ubuntu"),
        ..Default::default()
    };

    assert!(setup::update_system(&config).is_ok());

    // Verify system is up to date
    let update_status = std::process::Command::new("apt")
        .args(&["list", "--upgradable"])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&update_status.stdout)
        .trim()
        .is_empty());
}

#[test]
fn test_install_essential_packages() {
    let config = Config {
        linux_distro: String::from("ubuntu"),
        ..Default::default()
    };

    assert!(setup::install_essential_packages(&config).is_ok());

    // Verify essential packages are installed
    let essential_packages = ["curl", "wget", "vim", "ufw", "fail2ban"];
    for package in &essential_packages {
        let package_status = std::process::Command::new("dpkg")
            .args(&["-s", package])
            .status()
            .unwrap();
        assert!(package_status.success());
    }
}

#[test]
fn test_setup_firewall() {
    let config = Config {
        linux_distro: String::from("ubuntu"),
        custom_firewall_rules: vec![String::from("80/tcp"), String::from("443/tcp")],
        ..Default::default()
    };

    assert!(setup::setup_firewall(&config).is_ok());

    // Verify firewall is enabled and rules are applied
    let firewall_status = std::process::Command::new("ufw")
        .arg("status")
        .output()
        .unwrap();
    let status_output = String::from_utf8_lossy(&firewall_status.stdout);
    assert!(status_output.contains("Status: active"));
    assert!(status_output.contains("80/tcp"));
    assert!(status_output.contains("443/tcp"));
}

#[test]
fn test_setup_ssh() {
    assert!(setup::setup_ssh().is_ok());

    // Verify SSH configuration
    let ssh_config = fs::read_to_string("/etc/ssh/sshd_config").unwrap();
    assert!(ssh_config.contains("PermitRootLogin no"));
    assert!(ssh_config.contains("PasswordAuthentication no"));
    assert!(ssh_config.contains("Port 2222"));

    // Verify SSH service is running
    let ssh_status = std::process::Command::new("systemctl")
        .args(&["is-active", "sshd"])
        .status()
        .unwrap();
    assert!(ssh_status.success());
}

#[test]
fn test_initial_setup() {
    let config = Config {
        linux_distro: String::from("ubuntu"),
        custom_firewall_rules: vec![String::from("80/tcp"), String::from("443/tcp")],
        ..Default::default()
    };
    let rollback_manager = RollbackManager::new();

    assert!(setup::initial_setup(&config, &rollback_manager).is_ok());

    // Verify system is updated
    assert!(std::process::Command::new("apt")
        .args(&["list", "--upgradable"])
        .output()
        .unwrap()
        .stdout
        .is_empty());

    // Verify essential packages are installed
    assert!(std::process::Command::new("dpkg")
        .args(&["-s", "fail2ban"])
        .status()
        .unwrap()
        .success());

    // Verify firewall is set up
    assert!(std::process::Command::new("ufw")
        .arg("status")
        .output()
        .unwrap()
        .stdout
        .contains(&1));

    // Verify SSH is configured
    assert!(fs::read_to_string("/etc/ssh/sshd_config")
        .unwrap()
        .contains("PermitRootLogin no"));
}
