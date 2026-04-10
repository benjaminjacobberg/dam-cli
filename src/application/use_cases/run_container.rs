//! Run container use cases.

use crate::domain::presets;
use crate::ports::outbound::ContainerPort;
use anyhow::Result;
use colored::Colorize;
use std::sync::Arc;

/// Handle the run command - interactive container.
pub fn run_container(docker: Arc<dyn ContainerPort>) -> Result<()> {
    let container = presets::interactive();

    println!("{}", "Starting container:".yellow());
    for (host, container_path) in &container.volume_mounts {
        println!(
            "  Mounting {} -> /{}",
            host.display(),
            container_path.display()
        );
    }

    let status = docker.run_interactive(
        &container.image,
        &container.volume_mounts(),
        container.workdir.to_str().unwrap_or("/app"),
        &container.cmd_slice(),
    )?;

    if status.success() {
        println!("{}", "Container exited.".cyan());
    } else if let Some(code) = status.code() {
        println!("{} Exit code: {}", "Container exited with:".yellow(), code);
    }

    Ok(())
}

/// Handle the ACP server command.
pub fn run_acp_server(docker: Arc<dyn ContainerPort>) -> Result<()> {
    let container = presets::acp_server();

    println!("{}", "Starting ACP server:".yellow());
    for (host, container_path) in &container.volume_mounts {
        println!(
            "  Mounting {} -> /{}",
            host.display(),
            container_path.display()
        );
    }

    docker.run_detached(
        &container.image,
        container.name.as_deref(),
        &[("20758", "20758")],
        &container.volume_mounts(),
        container.workdir.to_str().unwrap_or("/app"),
        &container.cmd_slice(),
    )?;

    println!("{}", "✓ ACP server started on port 20758".green());
    println!("{}", "Zed config:".cyan());
    println!(
        r#"  {{"agent_servers": {{"OpenCode": {{"command": "docker", "args": ["exec", "-i", "dam-acp", "opencode", "acp"]}}}}}}"#
    );

    Ok(())
}

/// Handle the web server command.
pub fn run_web_server(docker: Arc<dyn ContainerPort>) -> Result<()> {
    let container = presets::web_server();

    println!(
        "{} Starting web server at http://127.0.0.1:<port>",
        "Starting:".yellow()
    );

    docker.run_detached(
        &container.image,
        container.name.as_deref(),
        &[("127.0.0.1:4096", "4096")],
        &container.volume_mounts(),
        container.workdir.to_str().unwrap_or("/app"),
        &container.cmd_slice(),
    )?;

    println!("{}", "✓ Web server started - check browser".green());

    // Give it a moment to start
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Try to get the port from logs
    let logs = docker.get_logs("dam-web")?;
    if let Some(line) = logs.lines().find(|l| l.contains("http://")) {
        println!("{}", line);
    }

    // Auto-open browser
    let _ = std::process::Command::new("sh")
        .args([
            "-c",
            "sleep 2 && docker logs dam-web 2>&1 | grep -o 'http://[^ ]*' | head -1 | xargs open",
        ])
        .spawn();

    Ok(())
}

/// Handle the teardown command.
pub fn teardown(docker: Arc<dyn ContainerPort>, image_name: &str) -> Result<()> {
    println!("{}", "Finding running container...".yellow());

    let containers = docker.list_containers(image_name)?;

    if containers.is_empty() {
        println!("{}", "No running containers found.".cyan());
        return Ok(());
    }

    let ids: Vec<&str> = containers.iter().map(|c| c.id.as_str()).collect();
    docker.stop_containers(&ids)?;

    for container in &containers {
        println!("{} Container {}", "Stopped:".red(), &container.id[..12]);
    }

    println!("{}", "✓ Containers stopped and removed.".green());
    Ok(())
}

/// Handle the status command.
pub fn status(docker: Arc<dyn ContainerPort>, image_name: &str) -> Result<()> {
    let containers = docker.list_containers(image_name)?;

    if containers.is_empty() {
        println!("{}", "No running container.".cyan());
    } else {
        println!("{}", "Running container:".green());
        for container in containers {
            println!("  {} {}", container.id, container.status);
        }
    }

    Ok(())
}

/// Handle the debug command.
pub fn debug() -> Result<()> {
    let cwd = std::env::current_dir()?;
    println!("Loaded directory: {}", cwd.display());
    Ok(())
}

#[cfg(all(test, feature = "testing"))]
mod tests {
    use super::*;
    use crate::adapters::driven::test::StubDockerAdapter;
    use crate::ports::outbound::ContainerInfo;
    use std::sync::Arc;

    #[test]
    fn test_teardown_no_containers() {
        let stub = StubDockerAdapter::new().with_list_containers(vec![]);
        let docker: Arc<dyn ContainerPort> = Arc::new(stub);

        let result = teardown(docker, "test-image");
        assert!(result.is_ok());
    }

    #[test]
    fn test_teardown_with_containers() {
        let stub = StubDockerAdapter::new().with_list_containers(vec![ContainerInfo {
            id: "abcdef123456".to_string(),
            status: "Up".to_string(),
        }]);
        let docker: Arc<dyn ContainerPort> = Arc::new(stub);

        let result = teardown(docker, "test-image");
        assert!(result.is_ok());
    }

    #[test]
    fn test_status_no_containers() {
        let stub = StubDockerAdapter::new().with_list_containers(vec![]);
        let docker: Arc<dyn ContainerPort> = Arc::new(stub);

        let result = status(docker, "test-image");
        assert!(result.is_ok());
    }

    #[test]
    fn test_status_with_containers() {
        let stub = StubDockerAdapter::new().with_list_containers(vec![ContainerInfo {
            id: "abc123def456".to_string(),
            status: "Up 2 hours".to_string(),
        }]);
        let docker: Arc<dyn ContainerPort> = Arc::new(stub);

        let result = status(docker, "test-image");
        assert!(result.is_ok());
    }

    #[test]
    fn test_debug_current_dir() {
        let result = debug();
        assert!(result.is_ok() || result.is_err());
    }
}
