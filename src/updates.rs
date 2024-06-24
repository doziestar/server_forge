use crate::config::Config;
use crate::distro::get_package_manager;
use crate::rollback::RollbackManager;
use crate::utils::run_command;
use log::info;
use std::error::Error;

pub fn setup_automatic_updates(
    config: &Config,
    rollback: &RollbackManager,
) -> Result<(), Box<dyn Error>> {
    info!("Setting up automatic updates...");

    let snapshot = rollback.create_snapshot()?;

    match config.linux_distro.as_str() {
        "ubuntu" => setup_ubuntu_updates(config)?,
        "centos" => setup_centos_updates(config)?,
        "fedora" => setup_fedora_updates(config)?,
        _ => return Err("Unsupported Linux distribution".into()),
    }

    rollback.commit_snapshot(snapshot)?;

    info!("Automatic updates configured");
    Ok(())
}

fn setup_ubuntu_updates(config: &Config) -> Result<(), Box<dyn Error>> {
    run_command(
        "apt",
        &["install", "-y", "unattended-upgrades", "apt-listchanges"],
    )?;

    let unattended_upgrades_conf = "/etc/apt/apt.conf.d/50unattended-upgrades";
    let conf_content = r#"
Unattended-Upgrade::Allowed-Origins {
    "${distro_id}:${distro_codename}";
    "${distro_id}:${distro_codename}-security";
};
Unattended-Upgrade::Package-Blacklist {
};
Unattended-Upgrade::AutoFixInterruptedDpkg "true";
Unattended-Upgrade::MinimalSteps "true";
Unattended-Upgrade::InstallOnShutdown "false";
Unattended-Upgrade::Mail "root";
Unattended-Upgrade::MailReport "on-change";
Unattended-Upgrade::Remove-Unused-Kernel-Packages "true";
Unattended-Upgrade::Remove-Unused-Dependencies "true";
Unattended-Upgrade::Automatic-Reboot "false";
"#;
    std::fs::write(unattended_upgrades_conf, conf_content)?;

    let auto_upgrades_conf = "/etc/apt/apt.conf.d/20auto-upgrades";
    let auto_upgrades_content = match config.update_schedule.as_str() {
        "daily" => {
            "APT::Periodic::Update-Package-Lists \"1\";\nAPT::Periodic::Unattended-Upgrade \"1\";\n"
        }
        "weekly" => {
            "APT::Periodic::Update-Package-Lists \"7\";\nAPT::Periodic::Unattended-Upgrade \"7\";\n"
        }
        _ => {
            "APT::Periodic::Update-Package-Lists \"1\";\nAPT::Periodic::Unattended-Upgrade \"1\";\n"
        }
    };
    std::fs::write(auto_upgrades_conf, auto_upgrades_content)?;

    run_command("systemctl", &["enable", "unattended-upgrades"])?;
    run_command("systemctl", &["start", "unattended-upgrades"])?;

    Ok(())
}

fn setup_centos_updates(config: &Config) -> Result<(), Box<dyn Error>> {
    run_command("yum", &["install", "-y", "yum-cron"])?;

    let yum_cron_conf = "/etc/yum/yum-cron.conf";
    let mut conf_content = std::fs::read_to_string(yum_cron_conf)?;
    conf_content = conf_content.replace("apply_updates = no", "apply_updates = yes");
    std::fs::write(yum_cron_conf, conf_content)?;

    run_command("systemctl", &["enable", "yum-cron"])?;
    run_command("systemctl", &["start", "yum-cron"])?;

    Ok(())
}

fn setup_fedora_updates(config: &Config) -> Result<(), Box<dyn Error>> {
    run_command("dnf", &["install", "-y", "dnf-automatic"])?;

    let dnf_automatic_conf = "/etc/dnf/automatic.conf";
    let mut conf_content = std::fs::read_to_string(dnf_automatic_conf)?;
    conf_content = conf_content.replace("apply_updates = no", "apply_updates = yes");
    std::fs::write(dnf_automatic_conf, conf_content)?;

    run_command("systemctl", &["enable", "dnf-automatic.timer"])?;
    run_command("systemctl", &["start", "dnf-automatic.timer"])?;

    Ok(())
}
