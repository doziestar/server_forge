use server_forge::backup;
use server_forge::config::Config;
use server_forge::rollback::RollbackManager;
use std::fs;
use std::os::unix::fs::PermissionsExt;

#[test]
fn test_install_backup_tools() {
    assert!(backup::install_backup_tools().is_ok());

    // Verify restic installation
    let restic_status = std::process::Command::new("restic")
        .arg("version")
        .status()
        .unwrap();
    assert!(restic_status.success());
}

#[test]
fn test_configure_backup_schedule() {
    let config = Config {
        backup_frequency: String::from("daily"),
        ..Default::default()
    };

    assert!(backup::configure_backup_schedule(&config).is_ok());

    // Verify cron job creation
    let cron_content = fs::read_to_string("/etc/cron.d/restic-backup").unwrap();
    assert!(cron_content.contains("0 2 * * * root /usr/bin/restic backup"));
}

#[test]
fn test_setup_backup_locations() {
    let config = Config {
        server_role: String::from("web"),
        ..Default::default()
    };

    assert!(backup::setup_backup_locations(&config).is_ok());

    // Verify backup script creation
    let script_content = fs::read_to_string("/usr/local/bin/run-backup.sh").unwrap();
    assert!(script_content.contains("restic backup"));
    assert!(script_content.contains("/var/www"));
    assert!(script_content.contains("/etc/nginx"));

    // Verify script permissions
    let script_metadata = fs::metadata("/usr/local/bin/run-backup.sh").unwrap();
    assert!(script_metadata.permissions().mode() & 0o111 != 0);
}

#[test]
fn test_setup_backup_system() {
    let config = Config {
        backup_frequency: String::from("daily"),
        server_role: String::from("web"),
        ..Default::default()
    };
    let rollback_manager = RollbackManager::new();

    assert!(backup::setup_backup_system(&config, &rollback_manager).is_ok());

    // Verify restic installation
    assert!(std::process::Command::new("restic")
        .arg("version")
        .status()
        .unwrap()
        .success());

    // Verify cron job creation
    assert!(fs::read_to_string("/etc/cron.d/restic-backup").is_ok());

    // Verify backup script creation
    assert!(fs::read_to_string("/usr/local/bin/run-backup.sh").is_ok());
}
