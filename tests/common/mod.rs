use mockall::mock;
use std::error::Error;

mock! {
    pub CommandRunner {}
    impl CommandRunner {
        pub fn run(&self, command: &str, args: &[&str]) -> Result<(), Box<dyn Error>>;
    }
}

#[derive(Clone, Default)]
pub struct MockConfig {
    pub linux_distro: String,
    pub server_role: String,
    pub security_level: String,
    pub monitoring: bool,
    pub backup_frequency: String,
    pub deployed_apps: Vec<String>,
    pub custom_firewall_rules: Vec<String>,
    pub update_schedule: String,
    pub use_containers: bool,
    pub use_kubernetes: bool,
}

pub struct MockRollbackManager;

impl MockRollbackManager {
    pub fn new() -> Self {
        MockRollbackManager
    }

    pub fn create_snapshot(&self) -> Result<usize, Box<dyn Error>> {
        Ok(0)
    }

    pub fn commit_snapshot(&self, _snapshot_id: usize) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
