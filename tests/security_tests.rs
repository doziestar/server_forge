use server_forge::config::Config;
use server_forge::rollback::RollbackManager;
use server_forge::security;
use std::fs;

#[test]
fn test_configure_fail2ban() {
    assert!(security::configure_fail2ban().is_ok());

    // Verify fail2ban configuration
    let fail2ban_config = fs::read_to_string("/etc/fail2ban/jail.local").unwrap();
    assert!(fail2ban_config.contains("[sshd]"));
    assert!(fail2ban_config.contains("maxretry = 3"));

    // Verify fail2ban service is running
    let status = std::process::Command::new("systemctl")
        .args(&["is-active", "fail2ban"])
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn test_setup_advanced_security() {
    let config = Config {
        linux_distro: String::from("ubuntu"),
        security_level: String::from("advanced"),
        ..Default::default()
    };

    assert!(security::setup_advanced_security(&config).is_ok());

    // Verify AppArmor is enforcing (for Ubuntu)
    if config.linux_distro == "ubuntu" {
        let status = std::process::Command::new("aa-status").status().unwrap();
        assert!(status.success());
    }
}

#[test]
fn test_setup_rootkit_detection() {
    let config = Config::default();
    assert!(security::setup_rootkit_detection(&config).is_ok());

    // Verify rkhunter and chkrootkit are installed
    let rkhunter_status = std::process::Command::new("which")
        .arg("rkhunter")
        .status()
        .unwrap();
    assert!(rkhunter_status.success());

    let chkrootkit_status = std::process::Command::new("which")
        .arg("chkrootkit")
        .status()
        .unwrap();
    assert!(chkrootkit_status.success());
}

#[test]
fn test_setup_security_scans() {
    assert!(security::setup_security_scans().is_ok());

    // Verify security scan script
    assert!(fs::metadata("/usr/local/bin/security_scan.sh").is_ok());

    // Verify cron job
    let cron_config = fs::read_to_string("/etc/cron.d/security_scan").unwrap();
    assert!(cron_config.contains("security_scan.sh"));
}

#[test]
fn test_implement_security_measures() {
    let config = Config {
        linux_distro: String::from("ubuntu"),
        security_level: String::from("advanced"),
        ..Default::default()
    };
    let rollback_manager = RollbackManager::new();

    assert!(security::implement_security_measures(&config, &rollback_manager).is_ok());
}
