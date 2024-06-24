use crate::config::Config;
use chrono::Local;
use log::{error, info};
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

pub(crate) fn setup_logging() -> Result<(), Box<dyn Error>> {
    let log_file = format!(
        "/var/log/server_setup_{}.log",
        Local::now().format("%Y%m%d_%H%M%S")
    );
    let file_appender = log4rs::append::file::FileAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
            "{d} - {l} - {m}\n",
        )))
        .build(log_file)?;

    let config = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build("file", Box::new(file_appender)))
        .build(
            log4rs::config::Root::builder()
                .appender("file")
                .build(log::LevelFilter::Info),
        )?;

    log4rs::init_config(config)?;
    Ok(())
}

pub(crate) fn get_user_input() -> Result<Config, Box<dyn Error>> {
    let mut config = Config::default();

    config.linux_distro = prompt("Enter Linux distribution (ubuntu/centos/fedora): ")?;
    config.server_role = prompt("Enter server role (web/database/application): ")?;
    config.security_level = prompt("Enter desired security level (basic/intermediate/advanced): ")?;
    config.monitoring = prompt("Enable monitoring? (y/n): ")?.to_lowercase() == "y";
    config.backup_frequency = prompt("Enter backup frequency (hourly/daily/weekly): ")?;
    config.update_schedule = prompt("Enter update schedule (daily/weekly/monthly): ")?;
    config.use_containers = prompt("Use containerization? (y/n): ")?.to_lowercase() == "y";

    if config.use_containers {
        config.use_kubernetes = prompt("Use Kubernetes? (y/n): ")?.to_lowercase() == "y";
    }

    let num_apps: usize = prompt("How many applications to deploy? ")?.parse()?;
    for i in 0..num_apps {
        let app = prompt(&format!("Enter application #{} to deploy: ", i + 1))?;
        config.deployed_apps.push(app);
    }

    let num_rules: usize = prompt("How many custom firewall rules to add? ")?.parse()?;
    for i in 0..num_rules {
        let rule = prompt(&format!("Enter custom firewall rule #{}: ", i + 1))?;
        config.custom_firewall_rules.push(rule);
    }

    Ok(config)
}

fn prompt(question: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", question);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub(crate) fn save_config(config: &Config) -> Result<(), Box<dyn Error>> {
    let config_path = "/etc/server_setup_config.json";
    let config_json = serde_json::to_string_pretty(config)?;
    fs::write(config_path, config_json)?;
    info!("Configuration saved to {}", config_path);
    Ok(())
}

pub(crate) fn run_command(command: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    info!("Running command: {} {:?}", command, args);
    let output = Command::new(command).args(args).output()?;
    if !output.status.success() {
        let error_message = format!(
            "Command failed: {} {:?}\nError: {}",
            command,
            args,
            String::from_utf8_lossy(&output.stderr)
        );
        error!("{}", error_message);
        return Err(error_message.into());
    }
    Ok(())
}

pub(crate) fn generate_report(config: &Config) -> Result<(), Box<dyn Error>> {
    let report_path = "/root/server_setup_report.txt";
    let mut report = String::new();

    report.push_str("Server Setup Report\n");
    report.push_str("===================\n\n");

    report.push_str(&format!("Linux Distribution: {}\n", config.linux_distro));
    report.push_str(&format!("Server Role: {}\n", config.server_role));
    report.push_str(&format!("Security Level: {}\n", config.security_level));
    report.push_str(&format!("Monitoring Enabled: {}\n", config.monitoring));
    report.push_str(&format!("Backup Frequency: {}\n", config.backup_frequency));
    report.push_str(&format!("Update Schedule: {}\n", config.update_schedule));
    report.push_str(&format!("Containerization: {}\n", config.use_containers));
    report.push_str(&format!("Kubernetes: {}\n", config.use_kubernetes));

    report.push_str("\nDeployed Applications:\n");
    for app in &config.deployed_apps {
        report.push_str(&format!("- {}\n", app));
    }

    report.push_str("\nCustom Firewall Rules:\n");
    for rule in &config.custom_firewall_rules {
        report.push_str(&format!("- {}\n", rule));
    }

    // Add system information
    report.push_str("\nSystem Information:\n");
    if let Ok(output) = Command::new("uname").arg("-a").output() {
        report.push_str(&format!(
            "OS: {}\n",
            String::from_utf8_lossy(&output.stdout).trim()
        ));
    }
    if let Ok(output) = Command::new("lscpu").output() {
        report.push_str(&format!(
            "CPU: {}\n",
            String::from_utf8_lossy(&output.stdout).trim()
        ));
    }
    if let Ok(output) = Command::new("free").arg("-h").output() {
        report.push_str(&format!(
            "Memory: {}\n",
            String::from_utf8_lossy(&output.stdout).trim()
        ));
    }

    fs::write(report_path, report)?;
    info!("Setup report generated at {}", report_path);
    Ok(())
}
