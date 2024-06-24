use std::path::PathBuf;
use std::process::Command;

fn get_project_root() -> PathBuf {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .expect("Failed to execute cargo metadata");

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Failed to parse cargo metadata");

    PathBuf::from(metadata["workspace_root"].as_str().unwrap())
}

fn build_docker_image() -> Result<(), String> {
    let status = Command::new("docker")
        .args(&["build", "-t", "server_forge_test_image", "."])
        .status()
        .map_err(|e| format!("Failed to build Docker image: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to build Docker image".to_string())
    }
}

fn run_test_in_container(test_name: &str) -> Result<(), String> {
    println!("Running test: {}", test_name);

    let project_root = get_project_root();

    let output = Command::new("docker")
        .args(&[
            "run",
            "--rm",
            "-v",
            &format!("{}:/app", project_root.to_str().unwrap()),
            "server_forge_test_image",
            "cargo",
            "test",
            "--test",
            test_name,
            "--",
            "--nocapture",
        ])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Test {} failed:\n{}",
            test_name,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

#[test]
fn run_all_tests() {
    if let Err(e) = build_docker_image() {
        panic!("Failed to build Docker image: {}", e);
    }

    let tests = vec![
        // "test_package_management",
        "test_security",
        "test_monitoring",
        "test_deployment",
        "test_rollback",
        "test_integration",
        "test_containerization",
        "test_backup",
        "test_setup",
        "test_updates",
    ];

    for test in tests {
        if let Err(e) = run_test_in_container(test) {
            panic!("{}", e);
        }
    }
}
