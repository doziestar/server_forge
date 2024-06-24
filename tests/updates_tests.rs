// use std::error::Error;
// use std::ffi::OsStr;
// use std::fs;
// use std::process::{Command as StdCommand, Output};
// use tempfile::tempdir;
// use mockall::{predicate::*, mock};
// use log::{info, error};
// use server_forge::config::Config;
// use server_forge::rollback::RollbackManager;
//
// pub trait CommandExecutor {
//     fn new(name: &str) -> Self;
//     fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self;
//     fn args<I, S>(&mut self, args: I) -> &mut Self
//         where
//             I: IntoIterator<Item = S>,
//             S: AsRef<OsStr>;
//     fn output(&mut self) -> std::io::Result<Output>;
// }
//
// impl CommandExecutor for StdCommand {
//     fn new(name: &str) -> Self {
//         StdCommand::new(name)
//     }
//
//     fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
//         StdCommand::arg(self, arg)
//     }
//
//     fn args<I, S>(&mut self, args: I) -> &mut Self
//         where
//             I: IntoIterator<Item = S>,
//             S: AsRef<OsStr>,
//     {
//         StdCommand::args(self, args)
//     }
//
//     fn output(&mut self) -> std::io::Result<Output> {
//         StdCommand::output(self)
//     }
// }
//
// mock! {
//     pub CommandExecutor {
//         fn new(name: &str) -> Self;
//         fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self;
//         fn args<I, S>(&mut self, args: I) -> &mut Self
//         where
//             I: IntoIterator<Item = S>,
//             S: AsRef<OsStr>;
//         fn output(&mut self) -> std::io::Result<Output>;
//     }
// }
//
// impl CommandExecutor for MockCommandExecutor {
//     fn new(name: &str) -> Self {
//         MockCommandExecutor::new(name)
//     }
//
//     fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
//         MockCommandExecutor::arg(self, arg)
//     }
//
//     fn args<I, S>(&mut self, args: I) -> &mut Self
//         where
//             I: IntoIterator<Item = S>,
//             S: AsRef<OsStr>,
//     {
//         MockCommandExecutor::args(self, args)
//     }
//
//     fn output(&mut self) -> std::io::Result<Output> {
//         MockCommandExecutor::output(self)
//     }
// }
//
// pub fn setup_automatic_updates<E: CommandExecutor>(
//     config: &Config,
//     rollback: &RollbackManager,
//     executor: &mut E,
// ) -> Result<(), Box<dyn Error>> {
//     info!("Setting up automatic updates...");
//
//     let snapshot = rollback.create_snapshot()?;
//
//     match config.linux_distro.as_str() {
//         "ubuntu" => setup_ubuntu_updates(config, executor)?,
//         "centos" => setup_centos_updates(config, executor)?,
//         "fedora" => setup_fedora_updates(config, executor)?,
//         _ => return Err("Unsupported Linux distribution".into()),
//     }
//
//     rollback.commit_snapshot(snapshot)?;
//
//     info!("Automatic updates configured");
//     Ok(())
// }
//
// fn setup_ubuntu_updates<E: CommandExecutor>(config: &Config, executor: &mut E) -> Result<(), Box<dyn Error>> {
//     executor.new("apt").args(&["install", "-y", "unattended-upgrades", "apt-listchanges"]).output()?;
//
//     let unattended_upgrades_conf = "/etc/apt/apt.conf.d/50unattended-upgrades";
//     let conf_content = r#"
// Unattended-Upgrade::Allowed-Origins {
//     "${distro_id}:${distro_codename}";
//     "${distro_id}:${distro_codename}-security";
// };
// Unattended-Upgrade::Package-Blacklist {
// };
// Unattended-Upgrade::AutoFixInterruptedDpkg "true";
// Unattended-Upgrade::MinimalSteps "true";
// Unattended-Upgrade::InstallOnShutdown "false";
// Unattended-Upgrade::Mail "root";
// Unattended-Upgrade::MailReport "on-change";
// Unattended-Upgrade::Remove-Unused-Kernel-Packages "true";
// Unattended-Upgrade::Remove-Unused-Dependencies "true";
// Unattended-Upgrade::Automatic-Reboot "false";
// "#;
//     fs::write(unattended_upgrades_conf, conf_content)?;
//
//     let auto_upgrades_conf = "/etc/apt/apt.conf.d/20auto-upgrades";
//     let auto_upgrades_content = match config.update_schedule.as_str() {
//         "daily" => {
//             "APT::Periodic::Update-Package-Lists \"1\";\nAPT::Periodic::Unattended-Upgrade \"1\";\n"
//         }
//         "weekly" => {
//             "APT::Periodic::Update-Package-Lists \"7\";\nAPT::Periodic::Unattended-Upgrade \"7\";\n"
//         }
//         _ => {
//             "APT::Periodic::Update-Package-Lists \"1\";\nAPT::Periodic::Unattended-Upgrade \"1\";\n"
//         }
//     };
//     fs::write(auto_upgrades_conf, auto_upgrades_content)?;
//
//     executor.new("systemctl").args(&["enable", "unattended-upgrades"]).output()?;
//     executor.new("systemctl").args(&["start", "unattended-upgrades"]).output()?;
//
//     Ok(())
// }
//
// fn setup_centos_updates<E: CommandExecutor>(config: &Config, executor: &mut E) -> Result<(), Box<dyn Error>> {
//     executor.new("yum").args(&["install", "-y", "yum-cron"]).output()?;
//
//     let yum_cron_conf = "/etc/yum/yum-cron.conf";
//     let mut conf_content = fs::read_to_string(yum_cron_conf)?;
//     conf_content = conf_content.replace("apply_updates = no", "apply_updates = yes");
//     fs::write(yum_cron_conf, conf_content)?;
//
//     executor.new("systemctl").args(&["enable", "yum-cron"]).output()?;
//     executor.new("systemctl").args(&["start", "yum-cron"]).output()?;
//
//     Ok(())
// }
//
// fn setup_fedora_updates<E: CommandExecutor>(config: &Config, executor: &mut E) -> Result<(), Box<dyn Error>> {
//     executor.new("dnf").args(&["install", "-y", "dnf-automatic"]).output()?;
//
//     let dnf_automatic_conf = "/etc/dnf/automatic.conf";
//     let mut conf_content = fs::read_to_string(dnf_automatic_conf)?;
//     conf_content = conf_content.replace("apply_updates = no", "apply_updates = yes");
//     fs::write(dnf_automatic_conf, conf_content)?;
//
//     executor.new("systemctl").args(&["enable", "dnf-automatic.timer"]).output()?;
//     executor.new("systemctl").args(&["start", "dnf-automatic.timer"]).output()?;
//
//     Ok(())
// }
//
// #[cfg(test)]
// mod updates_tests {
//     use std::os::unix::process::ExitStatusExt;
//     use server_forge::config::Config;
//     use server_forge::rollback::RollbackManager;
//     use super::*;
//
//     #[test]
//     fn test_setup_automatic_updates_ubuntu() -> Result<(), Box<dyn Error>> {
//         let config = Config {
//             linux_distro: "ubuntu".to_string(),
//             update_schedule: "daily".to_string(),
//             ..Config::default()
//         };
//         let rollback = RollbackManager::new()?;
//
//         let mut mock_command = MockCommandExecutor::new("");
//         mock_command
//             .expect_new()
//             .times(1)
//             .return_const(MockCommandExecutor::new(""));
//         mock_command
//             .expect_args()
//             .times(1)
//             .return_const(&mut mock_command);
//         mock_command
//             .expect_output()
//             .times(1)
//             .return_const(Ok(Output {
//                 status: std::process::ExitStatus::from_raw(0),
//                 stdout: Vec::new(),
//                 stderr: Vec::new(),
//             }));
//
//         setup_automatic_updates(&config, &rollback, &mut mock_command)?;
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_setup_automatic_updates_centos() -> Result<(), Box<dyn Error>> {
//         let config = Config {
//             linux_distro: "centos".to_string(),
//             update_schedule: "weekly".to_string(),
//             ..Config::default()
//         };
//         let rollback = RollbackManager::new()?;
//
//         let mut mock_command = MockCommandExecutor::new("");
//         mock_command
//             .expect_new()
//             .times(1)
//             .return_const(MockCommandExecutor::new(""));
//         mock_command
//             .expect_args()
//             .times(1)
//             .return_const(&mut mock_command);
//         mock_command
//             .expect_output()
//             .times(1)
//             .return_const(Ok(Output {
//                 status: std::process::ExitStatus::from_raw(0),
//                 stdout: Vec::new(),
//                 stderr: Vec::new(),
//             }));
//
//         setup_automatic_updates(&config, &rollback, &mut mock_command)?;
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_setup_automatic_updates_fedora() -> Result<(), Box<dyn Error>> {
//         let config = Config {
//             linux_distro: "fedora".to_string(),
//             update_schedule: "monthly".to_string(),
//             ..Config::default()
//         };
//         let rollback = RollbackManager::new()?;
//
//         let mut mock_command = MockCommandExecutor::new("");
//         mock_command
//             .expect_new()
//             .times(1)
//             .return_const(MockCommandExecutor::new(""));
//         mock_command
//             .expect_args()
//             .times(1)
//             .return_const(&mut mock_command);
//         mock_command
//             .expect_output()
//             .times(1)
//             .return_const(Ok(Output {
//                 status: std::process::ExitStatus::from_raw(0),
//                 stdout: Vec::new(),
//                 stderr: Vec::new(),
//             }));
//
//         setup_automatic_updates(&config, &rollback, &mut mock_command)?;
//
//         Ok(())
//     }
//
//     #[test]
//     fn test_setup_automatic_updates_unsupported_distro() {
//         let config = Config {
//             linux_distro: "archlinux".to_string(),
//             ..Config::default()
//         };
//         let rollback = RollbackManager::new().unwrap();
//
//         let mut mock_command = MockCommandExecutor::new("");
//         let result = setup_automatic_updates(&config, &rollback, &mut mock_command);
//         assert!(result.is_err());
//     }
// }
