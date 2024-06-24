use server_forge::config::Config;
use server_forge::security::{implement_security_measures, configure_fail2ban, setup_advanced_security, setup_rootkit_detection, setup_security_scans};
use server_forge::rollback::RollbackManager;
use std::error::Error;
use mockall::predicate::*;
use mockall::mock;

mock! {
    CommandRunner {}
    impl CommandRunner {
        fn run(&self, command: &str, args: &[&str]) -> Result<(), Box<dyn Error>>;
    }
}

#[test]
fn test_implement_security_measures() {
    let mut mock = MockCommandRunner::new();
    mock.expect_run().returning(|_, _| Ok(()));

    let config = Config::default();
    let rollback = RollbackManager::new();

    assert!(implement_security_measures(&config, &rollback, &mock).is_ok());
}

#[test]
fn test_configure_fail2ban() {
    let mut mock = MockCommandRunner::new();
    mock.expect_run()
        .with(eq("apt"), eq(&["install", "-y", "fail2ban"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("systemctl"), eq(&["enable", "fail2ban"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("systemctl"), eq(&["start", "fail2ban"]))
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(configure_fail2ban(&mock).is_ok());
}

#[test]
fn test_setup_advanced_security() {
    let mut mock = MockCommandRunner::new();
    let mut config = Config::default();
    config.security_level = "advanced".to_string();
    config.linux_distro = "ubuntu".to_string();

    mock.expect_run()
        .with(eq("apt"), eq(&["install", "-y", "apparmor", "apparmor-utils"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("aa-enforce"), eq(&["/etc/apparmor.d/*"]))
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(setup_advanced_security(&config, &mock).is_ok());
}

#[test]
fn test_setup_rootkit_detection() {
    let mut mock = MockCommandRunner::new();
    let config = Config::default();

    mock.expect_run()
        .with(eq("apt"), eq(&["install", "-y", "rkhunter", "chkrootkit"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("rkhunter"), eq(&["--update"]))
        .times(1)
        .returning(|_, _| Ok(()));
    mock.expect_run()
        .with(eq("rkhunter"), eq(&["--propupd"]))
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(setup_rootkit_detection(&config, &mock).is_ok());
}

#[test]
fn test_setup_security_scans() {
    let mut mock = MockCommandRunner::new();

    mock.expect_run()
        .with(eq("chmod"), eq(&["+x", "/usr/local/bin/security_scan.sh"]))
        .times(1)
        .returning(|_, _| Ok(()));

    assert!(setup_security_scans(&mock).is_ok());
}