//! CLI application service - orchestrates use cases for CLI commands.

use crate::adapters::driven::CliDockerAdapter;
use crate::application::use_cases;
use crate::domain::entities::DEFAULT_IMAGE_NAME;
use crate::domain::services::StackDiscovery;
use crate::ports::outbound::ContainerPort;
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;

/// CLI application service that coordinates between bounded contexts.
pub struct CliApplicationService {
    docker: Arc<dyn ContainerPort>,
    stacks: StackDiscovery,
    dockerfile_path: PathBuf,
    build_context: PathBuf,
}

impl CliApplicationService {
    /// Create a new CLI application service with real Docker CLI backend.
    pub fn new(stacks_path: PathBuf, dockerfile_path: PathBuf, build_context: PathBuf) -> Self {
        Self {
            docker: Arc::new(CliDockerAdapter::new()),
            stacks: StackDiscovery::new(stacks_path),
            dockerfile_path,
            build_context,
        }
    }

    /// Create a CliApplicationService with custom dependencies (useful for testing).
    pub fn new_with_deps(
        docker: Arc<dyn ContainerPort>,
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
        use_cases::build_image(
            self.docker.clone(),
            stack,
            self.dockerfile_path.clone(),
            self.build_context.clone(),
        )
    }

    /// Handle the run command.
    pub fn run(&self) -> Result<()> {
        use_cases::run_container(self.docker.clone())
    }

    /// Handle the ACP server command.
    pub fn acp(&self) -> Result<()> {
        use_cases::run_acp_server(self.docker.clone())
    }

    /// Handle the teardown command.
    pub fn teardown(&self) -> Result<()> {
        use_cases::teardown(self.docker.clone(), DEFAULT_IMAGE_NAME)
    }

    /// Handle the status command.
    pub fn status(&self) -> Result<()> {
        use_cases::status(self.docker.clone(), DEFAULT_IMAGE_NAME)
    }

    /// Handle the debug command.
    pub fn debug(&self) -> Result<()> {
        use_cases::debug()
    }

    /// Handle the web command.
    pub fn web(&self) -> Result<()> {
        use_cases::run_web_server(self.docker.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_service_new() {
        let temp_dir = TempDir::new().unwrap();
        let service = CliApplicationService::new(
            temp_dir.path().to_path_buf(),
            temp_dir.path().join("Dockerfile"),
            temp_dir.path().to_path_buf(),
        );

        assert_eq!(service.dockerfile_path, temp_dir.path().join("Dockerfile"));
        assert_eq!(service.build_context, temp_dir.path());
    }

    #[test]
    fn test_service_get_stacks_help_empty() {
        let temp_dir = TempDir::new().unwrap();
        let service = CliApplicationService::new(
            temp_dir.path().to_path_buf(),
            temp_dir.path().join("Dockerfile"),
            temp_dir.path().to_path_buf(),
        );

        let help = service.get_stacks_help();
        assert!(help.contains("none"));
    }

    #[cfg(all(test, feature = "testing"))]
    #[test]
    fn test_service_new_with_deps() {
        use crate::adapters::driven::test::StubDockerAdapter;
        use std::sync::Arc;

        let temp_dir = TempDir::new().unwrap();
        let stacks = StackDiscovery::new(temp_dir.path().to_path_buf());
        let docker: Arc<dyn ContainerPort> = Arc::new(StubDockerAdapter::new());

        let service = CliApplicationService::new_with_deps(
            docker.clone(),
            stacks,
            temp_dir.path().join("Dockerfile"),
            temp_dir.path().to_path_buf(),
        );

        assert_eq!(service.dockerfile_path, temp_dir.path().join("Dockerfile"));
    }

    #[cfg(all(test, feature = "testing"))]
    #[test]
    fn test_service_build_wires_up() {
        use crate::adapters::driven::test::StubDockerAdapter;
        use std::sync::Arc;

        let temp_dir = TempDir::new().unwrap();
        let stacks = StackDiscovery::new(temp_dir.path().to_path_buf());
        let stub = StubDockerAdapter::new().with_build_error("Simulated build error");
        let docker: Arc<dyn ContainerPort> = Arc::new(stub);

        let service = CliApplicationService::new_with_deps(
            docker.clone(),
            stacks,
            temp_dir.path().join("Dockerfile"),
            temp_dir.path().to_path_buf(),
        );

        let result = service.build("test-stack");
        assert!(result.is_err());
    }
}
