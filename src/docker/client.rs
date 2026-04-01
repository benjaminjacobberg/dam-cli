//! Docker client abstraction.
//!
//! Provides a trait for Docker operations that can be implemented
//! by different backends (CLI, API, etc.).

use anyhow::Result;
use std::process::{Command, ExitStatus, Output};

/// Represents information about a running container.
#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub status: String,
}

/// Trait for Docker operations.
/// Implement this to support different Docker backends.
pub trait DockerClient: Send + Sync {
    /// Build a Docker image.
    fn build_image(
        &self,
        image_name: &str,
        dockerfile_path: &str,
        build_context: &str,
        build_args: &[(&str, &str)],
    ) -> Result<()>;

    /// Run a container interactively and wait for completion.
    fn run_interactive(
        &self,
        image: &str,
        volume_mounts: &[(&str, &str)],
        workdir: &str,
        cmd: &[&str],
    ) -> Result<ExitStatus>;

    /// Run a container in detached mode.
    fn run_detached(
        &self,
        image: &str,
        name: Option<&str>,
        ports: &[(&str, &str)],
        volume_mounts: &[(&str, &str)],
        workdir: &str,
        cmd: &[&str],
    ) -> Result<()>;

    /// Stop and remove containers by IDs.
    fn stop_containers(&self, ids: &[&str]) -> Result<()>;

    /// List running containers filtered by image.
    fn list_containers(&self, image_filter: &str) -> Result<Vec<ContainerInfo>>;

    /// Get container logs.
    fn get_logs(&self, container_name: &str) -> Result<String>;
}

/// Docker client implementation using the Docker CLI.
pub struct CliDockerClient;

impl CliDockerClient {
    pub fn new() -> Self {
        Self
    }

    fn docker_cmd(&self) -> Command {
        Command::new("docker")
    }

    fn run_output(&self, output: Output) -> Result<()> {
        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Docker command failed: {}", stderr)
        }
    }
}

impl Default for CliDockerClient {
    fn default() -> Self {
        Self::new()
    }
}

impl DockerClient for CliDockerClient {
    fn build_image(
        &self,
        image_name: &str,
        dockerfile_path: &str,
        build_context: &str,
        build_args: &[(&str, &str)],
    ) -> Result<()> {
        let mut cmd = self.docker_cmd();
        cmd.args(["build", "-t", image_name, "-f", dockerfile_path]);

        for (key, value) in build_args {
            cmd.arg("--build-arg").arg(format!("{}={}", key, value));
        }

        cmd.arg(build_context);

        let output = cmd.output()?;
        self.run_output(output)
    }

    fn run_interactive(
        &self,
        image: &str,
        volume_mounts: &[(&str, &str)],
        workdir: &str,
        args: &[&str],
    ) -> Result<ExitStatus> {
        let mut docker_cmd = self.docker_cmd();
        docker_cmd.arg("run").arg("-it").arg("--rm");

        for (host_path, container_path) in volume_mounts {
            docker_cmd
                .arg("-v")
                .arg(format!("{}:{}", host_path, container_path));
        }

        docker_cmd.arg("-w").arg(workdir);
        docker_cmd.arg(image);
        docker_cmd.args(args);

        let status = docker_cmd.status()?;
        Ok(status)
    }

    fn run_detached(
        &self,
        image: &str,
        name: Option<&str>,
        ports: &[(&str, &str)],
        volume_mounts: &[(&str, &str)],
        workdir: &str,
        args: &[&str],
    ) -> Result<()> {
        let mut docker_cmd = self.docker_cmd();
        docker_cmd.arg("run").arg("-d");

        if let Some(name) = name {
            docker_cmd.arg("--name").arg(name);
        }

        for (host_port, container_port) in ports {
            docker_cmd
                .arg("-p")
                .arg(format!("{}:{}", host_port, container_port));
        }

        for (host_path, container_path) in volume_mounts {
            docker_cmd
                .arg("-v")
                .arg(format!("{}:{}", host_path, container_path));
        }

        docker_cmd.arg("-w").arg(workdir);
        docker_cmd.arg(image);
        docker_cmd.args(args);

        let output = docker_cmd.output()?;
        self.run_output(output)
    }

    fn stop_containers(&self, ids: &[&str]) -> Result<()> {
        for id in ids {
            let output = self.docker_cmd().args(["kill", id]).output()?;
            self.run_output(output)?;
        }
        Ok(())
    }

    fn list_containers(&self, image_filter: &str) -> Result<Vec<ContainerInfo>> {
        let output = self
            .docker_cmd()
            .args([
                "ps",
                "--filter",
                &format!("ancestor={}", image_filter),
                "--format",
                "{{.ID}} {{.Status}}",
            ])
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let containers = stdout
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let parts: Vec<&str> = line.splitn(2, ' ').collect();
                ContainerInfo {
                    id: parts.first().unwrap_or(&"").to_string(),
                    status: parts.get(1).unwrap_or(&"").to_string(),
                }
            })
            .collect();

        Ok(containers)
    }

    fn get_logs(&self, container_name: &str) -> Result<String> {
        let output = self.docker_cmd().args(["logs", container_name]).output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_info_default() {
        let info = ContainerInfo {
            id: String::new(),
            status: String::new(),
        };
        assert!(info.id.is_empty());
        assert!(info.status.is_empty());
    }

    #[test]
    fn test_cli_docker_client_can_be_created() {
        let _client = CliDockerClient::new();
        // Basic instantiation test - just ensure it can be created
    }
}
