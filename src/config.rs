//! # Configuration Module
//!
//! This module defines the `Config` struct, which represents the configuration
//! for the server setup and maintenance tool. It includes various settings
//! such as the Linux distribution, server role, security level, and deployment options.
//!
//! The `Config` struct implements `Serialize` and `Deserialize` traits from serde,
//! allowing for easy serialization and deserialization of the configuration.

use serde::{Deserialize, Serialize};

/// Represents the configuration for the server setup and maintenance tool.
///
/// This struct contains all the necessary settings and options for configuring
/// a server, including the operating system, security settings, and deployment options.
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    /// The Linux distribution being used (e.g., "ubuntu", "centos", "fedora")
    pub linux_distro: String,

    /// The role of the server (e.g., "web", "database", "application")
    pub server_role: String,

    /// The desired security level (e.g., "basic", "intermediate", "advanced")
    pub security_level: String,

    /// Whether to enable monitoring on the server
    pub monitoring: bool,

    /// The frequency of backups (e.g., "hourly", "daily", "weekly")
    pub backup_frequency: String,

    /// A list of applications to be deployed on the server
    pub deployed_apps: Vec<String>,

    /// A list of custom firewall rules to be applied
    pub custom_firewall_rules: Vec<String>,

    /// The schedule for automatic updates (e.g., "daily", "weekly", "monthly")
    pub update_schedule: String,

    /// Whether to use containerization for deployments
    pub use_containers: bool,

    /// Whether to use Kubernetes for container orchestration
    pub use_kubernetes: bool,
}

/// Provides default values for the `Config` struct.
impl Default for Config {
    /// Returns a new `Config` instance with default values.
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
