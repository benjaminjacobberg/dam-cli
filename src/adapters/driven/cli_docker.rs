//! CLI Docker adapter - Docker implementation using the Docker CLI.

use crate::ports::outbound::docker::{ContainerInfo, ContainerPort};
use std::process::{Command, ExitStatus, Output};

/// Docker client implementation using the Docker CLI.
pub struct CliDockerAdapter;

impl CliDockerAdapter {
    pub fn new() -> Self {
        Self
    }

    fn docker_cmd(&self) -> Command {
        Command::new("docker")
    }

    fn run_output(&self, output: Output) -> anyhow::Result<()> {
        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Docker command failed: {}", stderr)
        }
    }
}

impl Default for CliDockerAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerPort for CliDockerAdapter {
    fn build_image(
        &self,
        image_name: &str,
        dockerfile_path: &str,
        build_context: &str,
        build_args: &[(&str, &str)],
    ) -> anyhow::Result<()> {
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
    ) -> anyhow::Result<ExitStatus> {
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
    ) -> anyhow::Result<()> {
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

    fn stop_containers(&self, ids: &[&str]) -> anyhow::Result<()> {
        for id in ids {
            let output = self.docker_cmd().args(["kill", id]).output()?;
            self.run_output(output)?;
        }
        Ok(())
    }

    fn list_containers(&self, image_filter: &str) -> anyhow::Result<Vec<ContainerInfo>> {
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

    fn get_logs(&self, container_name: &str) -> anyhow::Result<String> {
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
    fn test_cli_docker_adapter_can_be_created() {
        let _client = CliDockerAdapter::new();
    }
}
