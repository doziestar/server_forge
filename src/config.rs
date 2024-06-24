use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
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

impl Default for Config {
    fn default() -> Self {
        Config {
            linux_distro: String::from("ubuntu"),
            server_role: String::new(),
            security_level: String::new(),
            monitoring: false,
            backup_frequency: String::from("daily"),
            deployed_apps: Vec::new(),
            custom_firewall_rules: Vec::new(),
            update_schedule: String::from("weekly"),
            use_containers: false,
            use_kubernetes: false,
        }
    }
}
