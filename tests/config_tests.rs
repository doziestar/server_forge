use server_forge::config::Config;

#[test]
fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.linux_distro, "ubuntu");
    assert_eq!(config.server_role, "");
    assert_eq!(config.security_level, "");
    assert_eq!(config.monitoring, false);
    assert_eq!(config.backup_frequency, "daily");
    assert!(config.deployed_apps.is_empty());
    assert!(config.custom_firewall_rules.is_empty());
    assert_eq!(config.update_schedule, "weekly");
    assert_eq!(config.use_containers, false);
    assert_eq!(config.use_kubernetes, false);
}

#[test]
fn test_config_serialization() {
    let config = Config {
        linux_distro: "centos".to_string(),
        server_role: "web".to_string(),
        security_level: "high".to_string(),
        monitoring: true,
        backup_frequency: "weekly".to_string(),
        deployed_apps: vec!["nginx".to_string(), "mysql".to_string()],
        custom_firewall_rules: vec!["80/tcp".to_string()],
        update_schedule: "daily".to_string(),
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
    assert_eq!(config.custom_firewall_rules, deserialized.custom_firewall_rules);
    assert_eq!(config.update_schedule, deserialized.update_schedule);
    assert_eq!(config.use_containers, deserialized.use_containers);
    assert_eq!(config.use_kubernetes, deserialized.use_kubernetes);
}