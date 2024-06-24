use chrono::Local;
use std::error::Error;

// pub fn setup_logging() -> Result<(), Box<dyn Error>> {
//     let log_file = format!(
//         "./test_log_{}.log",
//         Local::now().format("%Y%m%d_%H%M%S")
//     );
//     let file_appender = log4rs::append::file::FileAppender::builder()
//         .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
//             "{d} - {l} - {m}\n",
//         )))
//         .build(log_file)?;
//
//     let config = log4rs::config::Config::builder()
//         .appender(log4rs::config::Appender::builder().build("file", Box::new(file_appender)))
//         .build(
//             log4rs::config::Root::builder()
//                 .appender("file")
//                 .build(log::LevelFilter::Info),
//         )?;
//
//     log4rs::init_config(config)?;
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use server_forge::config::Config;
    use server_forge::utils::{generate_report, get_user_input, run_command, save_config};
    use std::error::Error;
    use std::fs;
    use std::io::Cursor;
    use tempfile::tempdir;

    // #[test]
    // fn test_setup_logging() -> Result<(), Box<dyn Error>> {
    //     let temp_dir = tempdir()?;
    //     let log_path = temp_dir.path().join("test.log");
    //
    //     setup_logging()?;
    //
    //     log::info!("Test log message");
    //
    //     // Check if the log file was created and contains the message
    //     let log_content = fs::read_to_string(log_path)?;
    //     assert!(log_content.contains("Test log message"));
    //
    //     Ok(())
    // }

    // #[test]
    // fn test_get_user_input() -> Result<(), Box<dyn Error>> {
    //     let input = b"ubuntu\nweb\nbasic\ny\ndaily\nweekly\ny\nn\n2\napp1\napp2\n1\nrule1\n";
    //     let mut cursor = Cursor::new(input);
    //
    //     let config = get_user_input()?;
    //
    //     assert_eq!(config.linux_distro, "ubuntu");
    //     assert_eq!(config.server_role, "web");
    //     assert_eq!(config.security_level, "basic");
    //     assert_eq!(config.monitoring, true);
    //     assert_eq!(config.backup_frequency, "daily");
    //     assert_eq!(config.update_schedule, "weekly");
    //     assert_eq!(config.use_containers, true);
    //     assert_eq!(config.use_kubernetes, false);
    //     assert_eq!(config.deployed_apps, vec!["app1", "app2"]);
    //     assert_eq!(config.custom_firewall_rules, vec!["rule1"]);
    //
    //     Ok(())
    // }

    // #[test]
    // fn test_save_config() -> Result<(), Box<dyn Error>> {
    //     let temp_dir = tempdir()?;
    //     let config_path = temp_dir.path().join("config.json");
    //
    //     let config = Config {
    //         linux_distro: "fedora".to_string(),
    //         server_role: "database".to_string(),
    //         ..Config::default()
    //     };
    //
    //     save_config(&config)?;
    //
    //     // Check if the config file was created and contains the correct data
    //     let saved_config: Config = serde_json::from_str(&fs::read_to_string(config_path)?)?;
    //     assert_eq!(saved_config.linux_distro, "fedora");
    //     assert_eq!(saved_config.server_role, "database");
    //
    //     Ok(())
    // }

    #[test]
    fn test_run_command() -> Result<(), Box<dyn Error>> {
        let output = run_command("echo", &["Hello, world!"]);
        assert!(output.is_ok());
        Ok(())
    }

    // #[test]
    // fn test_generate_report() -> Result<(), Box<dyn Error>> {
    //     let temp_dir = tempdir()?;
    //     let report_path = temp_dir.path().join("report.txt");
    //
    //     let config = Config {
    //         linux_distro: "ubuntu".to_string(),
    //         server_role: "web".to_string(),
    //         deployed_apps: vec!["nginx".to_string()],
    //         custom_firewall_rules: vec!["80/tcp".to_string()],
    //         ..Config::default()
    //     };
    //
    //     generate_report(&config)?;
    //
    //     // Check if the report file was created and contains the correct data
    //     let report_content = fs::read_to_string(report_path)?;
    //     assert!(report_content.contains("Linux Distribution: ubuntu"));
    //     assert!(report_content.contains("Server Role: web"));
    //     assert!(report_content.contains("Deployed Applications:"));
    //     assert!(report_content.contains("- nginx"));
    //     assert!(report_content.contains("Custom Firewall Rules:"));
    //     assert!(report_content.contains("- 80/tcp"));
    //
    //     Ok(())
    // }
}
