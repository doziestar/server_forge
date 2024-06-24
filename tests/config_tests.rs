#[cfg(test)]
mod config_tests {
    use super::*;
    use server_forge::config::Config;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.linux_distro, "ubuntu");
        assert_eq!(config.server_role, "");
        assert_eq!(config.security_level, "");
        assert_eq!(config.monitoring, false);
        assert_eq!(config.backup_frequency, "daily");
        assert_eq!(config.deployed_apps, Vec::<String>::new());
        assert_eq!(config.custom_firewall_rules, Vec::<String>::new());
        assert_eq!(config.update_schedule, "weekly");
        assert_eq!(config.use_containers, false);
        assert_eq!(config.use_kubernetes, false);
    }

    #[test]
    fn test_config_custom() {
        let config = Config {
            linux_distro: "centos".to_string(),
            server_role: "web".to_string(),
            security_level: "high".to_string(),
            monitoring: true,
            backup_frequency: "hourly".to_string(),
            deployed_apps: vec!["nginx".to_string(), "mysql".to_string()],
            custom_firewall_rules: vec!["80/tcp".to_string(), "443/tcp".to_string()],
            update_schedule: "daily".to_string(),
            use_containers: true,
            use_kubernetes: true,
        };

        assert_eq!(config.linux_distro, "centos");
        assert_eq!(config.server_role, "web");
        assert_eq!(config.security_level, "high");
        assert_eq!(config.monitoring, true);
        assert_eq!(config.backup_frequency, "hourly");
        assert_eq!(config.deployed_apps, vec!["nginx", "mysql"]);
        assert_eq!(config.custom_firewall_rules, vec!["80/tcp", "443/tcp"]);
        assert_eq!(config.update_schedule, "daily");
        assert_eq!(config.use_containers, true);
        assert_eq!(config.use_kubernetes, true);
    }

    #[test]
    fn test_config_clone() {
        let config1 = Config {
            linux_distro: "fedora".to_string(),
            server_role: "database".to_string(),
            ..Config::default()
        };

        let config2 = config1.clone();

        assert_eq!(config1.linux_distro, config2.linux_distro);
        assert_eq!(config1.server_role, config2.server_role);
        assert_eq!(config1.security_level, config2.security_level);
        assert_eq!(config1.monitoring, config2.monitoring);
        assert_eq!(config1.backup_frequency, config2.backup_frequency);
        assert_eq!(config1.deployed_apps, config2.deployed_apps);
        assert_eq!(config1.custom_firewall_rules, config2.custom_firewall_rules);
        assert_eq!(config1.update_schedule, config2.update_schedule);
        assert_eq!(config1.use_containers, config2.use_containers);
        assert_eq!(config1.use_kubernetes, config2.use_kubernetes);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config {
            linux_distro: "debian".to_string(),
            server_role: "application".to_string(),
            security_level: "medium".to_string(),
            monitoring: true,
            backup_frequency: "weekly".to_string(),
            deployed_apps: vec!["tomcat".to_string()],
            custom_firewall_rules: vec!["8080/tcp".to_string()],
            update_schedule: "monthly".to_string(),
            use_containers: true,
            use_kubernetes: false,
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&serialized).unwrap();

        assert_eq!(config.linux_distro, deserialized.linux_distro);
        assert_eq!(config.server_role, deserialized.server_role);
        assert_eq!(config.security_level, deserialized.security_level);
        assert_eq!(config.monitoring, deserialized.monitoring);
        assert_eq!(config.backup_frequency, deserialized.backup_frequency);
        assert_eq!(config.deployed_apps, deserialized.deployed_apps);
        assert_eq!(
            config.custom_firewall_rules,
            deserialized.custom_firewall_rules
        );
        assert_eq!(config.update_schedule, deserialized.update_schedule);
        assert_eq!(config.use_containers, deserialized.use_containers);
        assert_eq!(config.use_kubernetes, deserialized.use_kubernetes);
    }
}
