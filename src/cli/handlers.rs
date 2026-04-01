//! CLI handlers - orchestrate bounded contexts for each CLI command.

use crate::container::commands as container_commands;
#[cfg(feature = "testing")]
pub use crate::docker::StubDockerClient;
use crate::docker::{CliDockerClient, DockerClient};
use crate::image::DEFAULT_IMAGE_NAME;
use crate::image::commands as image_commands;
use crate::stack::StackDiscovery;
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;

/// CLI handler that coordinates between bounded contexts.
pub struct CliHandler {
    docker: Arc<dyn DockerClient>,
    stacks: StackDiscovery,
    dockerfile_path: PathBuf,
    build_context: PathBuf,
}

impl CliHandler {
    /// Create a new CLI handler with real Docker CLI backend.
    ///
    /// # Arguments
    /// * `stacks_path` - Directory containing stack `.sh` scripts
    /// * `dockerfile_path` - Path to the Dockerfile
    /// * `build_context` - Docker build context directory
    pub fn new(stacks_path: PathBuf, dockerfile_path: PathBuf, build_context: PathBuf) -> Self {
        Self {
            docker: Arc::new(CliDockerClient::new()),
            stacks: StackDiscovery::new(stacks_path),
            dockerfile_path,
            build_context,
        }
    }

    /// Create a CliHandler with custom dependencies (useful for testing).
    pub fn new_with_deps(
        docker: Arc<dyn DockerClient>,
        stacks: StackDiscovery,
        dockerfile_path: PathBuf,
        build_context: PathBuf,
    ) -> Self {
        Self {
            docker,
            stacks,
            dockerfile_path,
            build_context,
        }
    }

    /// Get available stacks formatted for help text.
    pub fn get_stacks_help(&self) -> String {
        format!("Stack: {} or path to script.sh", self.stacks.format_help())
    }

    /// Handle the build command.
    pub fn build(&self, stack: &str) -> Result<()> {
        image_commands::cmd_build(
            self.docker.clone(),
            stack,
            self.dockerfile_path.clone(),
            self.build_context.clone(),
        )
    }

    /// Handle the run command.
    pub fn run(&self) -> Result<()> {
        container_commands::cmd_run(self.docker.clone())
    }

    /// Handle the ACP server command.
    pub fn acp(&self) -> Result<()> {
        container_commands::cmd_acp(self.docker.clone())
    }

    /// Handle the teardown command.
    pub fn teardown(&self) -> Result<()> {
        container_commands::cmd_teardown(self.docker.clone(), DEFAULT_IMAGE_NAME)
    }

    /// Handle the status command.
    pub fn status(&self) -> Result<()> {
        container_commands::cmd_status(self.docker.clone(), DEFAULT_IMAGE_NAME)
    }

    /// Handle the debug command.
    pub fn debug(&self) -> Result<()> {
        container_commands::cmd_debug()
    }

    /// Handle the web command.
    pub fn web(&self) -> Result<()> {
        container_commands::cmd_web(self.docker.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_handler_new() {
        let temp_dir = TempDir::new().unwrap();
        let handler = CliHandler::new(
            temp_dir.path().to_path_buf(),
            temp_dir.path().join("Dockerfile"),
            temp_dir.path().to_path_buf(),
        );

        // Verify fields are populated correctly
        assert_eq!(handler.dockerfile_path, temp_dir.path().join("Dockerfile"));
        assert_eq!(handler.build_context, temp_dir.path());
    }

    #[test]
    fn test_handler_get_stacks_help_empty() {
        let temp_dir = TempDir::new().unwrap();
        let handler = CliHandler::new(
            temp_dir.path().to_path_buf(),
            temp_dir.path().join("Dockerfile"),
            temp_dir.path().to_path_buf(),
        );

        let help = handler.get_stacks_help();
        assert!(help.contains("none"));
    }

    #[cfg(all(test, feature = "testing"))]
    #[test]
    fn test_handler_new_with_deps() {
        let temp_dir = TempDir::new().unwrap();
        let stacks = StackDiscovery::new(temp_dir.path().to_path_buf());
        let docker: Arc<dyn DockerClient> = Arc::new(StubDockerClient::new());

        let handler = CliHandler::new_with_deps(
            docker.clone(),
            stacks,
            temp_dir.path().join("Dockerfile"),
            temp_dir.path().to_path_buf(),
        );

        assert_eq!(handler.dockerfile_path, temp_dir.path().join("Dockerfile"));
    }

    #[cfg(all(test, feature = "testing"))]
    #[test]
    fn test_handler_build_wires_up() {
        let temp_dir = TempDir::new().unwrap();
        let stacks = StackDiscovery::new(temp_dir.path().to_path_buf());
        // Use stub that will fail to simulate error condition
        let stub = StubDockerClient::new().with_build_error("Simulated build error");
        let docker: Arc<dyn DockerClient> = Arc::new(stub);

        let handler = CliHandler::new_with_deps(
            docker.clone(),
            stacks,
            temp_dir.path().join("Dockerfile"),
            temp_dir.path().to_path_buf(),
        );

        // Should fail with our simulated error
        let result = handler.build("test-stack");
        assert!(result.is_err());
    }
}
