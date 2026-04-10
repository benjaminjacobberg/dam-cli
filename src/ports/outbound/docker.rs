//! Container port - outbound port for container operations.
//!
//! This trait defines the contract for any container runtime implementation
//! (Docker, containerd, etc.).

use std::process::ExitStatus;

/// Represents information about a running container.
#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub status: String,
}

/// Outbound port for container operations.
/// Any container runtime implementation must implement this.
pub trait ContainerPort: Send + Sync {
    /// Build a Docker image.
    fn build_image(
        &self,
        image_name: &str,
        dockerfile_path: &str,
        build_context: &str,
        build_args: &[(&str, &str)],
    ) -> anyhow::Result<()>;

    /// Run a container interactively and wait for completion.
    fn run_interactive(
        &self,
        image: &str,
        volume_mounts: &[(&str, &str)],
        workdir: &str,
        cmd: &[&str],
    ) -> anyhow::Result<ExitStatus>;

    /// Run a container in detached mode.
    fn run_detached(
        &self,
        image: &str,
        name: Option<&str>,
        ports: &[(&str, &str)],
        volume_mounts: &[(&str, &str)],
        workdir: &str,
        cmd: &[&str],
    ) -> anyhow::Result<()>;

    /// Stop and remove containers by IDs.
    fn stop_containers(&self, ids: &[&str]) -> anyhow::Result<()>;

    /// List running containers filtered by image.
    fn list_containers(&self, image_filter: &str) -> anyhow::Result<Vec<ContainerInfo>>;

    /// Get container logs.
    fn get_logs(&self, container_name: &str) -> anyhow::Result<String>;
}
