//! Container entity representing a Docker container.
//!
//! # Example
//!
//! ```
//! use dam_cli::container::Container;
//! use std::path::PathBuf;
//!
//! let container = Container::new(
//!     "my-image",
//!     PathBuf::from("/host/path"),
//!     "",
//! ).with_name("my-container")
//!  .with_command(vec!["bash".to_string(), "-c".to_string(), "echo hello".to_string()]);
//!
//! assert_eq!(container.image, "my-image");
//! assert_eq!(container.name, Some("my-container".to_string()));
//! ```

use std::path::PathBuf;

/// Represents a configured container to run.
#[derive(Debug, Clone)]
pub struct Container {
    /// Name of the container.
    pub name: Option<String>,
    /// Image to run.
    pub image: String,
    /// Host path to mount.
    pub volume_host_path: PathBuf,
    /// Container path to mount at.
    pub volume_container_path: PathBuf,
    /// Working directory inside the container.
    pub workdir: PathBuf,
    /// Command to execute.
    pub cmd: Vec<String>,
}

impl Container {
    /// Create a new container configuration.
    pub fn new(
        image: impl Into<String>,
        volume_host_path: PathBuf,
        _workdir: impl Into<String>,
    ) -> Self {
        let container_dir = volume_host_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("app")
            .to_string();

        Self {
            name: None,
            image: image.into(),
            volume_host_path,
            volume_container_path: PathBuf::from(format!("/{}", container_dir)),
            workdir: PathBuf::from(format!("/{}", container_dir)),
            cmd: Vec::new(),
        }
    }

    /// Set the container name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the command to run.
    pub fn with_command(mut self, cmd: Vec<String>) -> Self {
        self.cmd = cmd;
        self
    }

    /// Get volume mounts as tuples.
    pub fn volume_mounts(&self) -> Vec<(&str, &str)> {
        vec![(
            self.volume_host_path.to_str().unwrap_or("."),
            self.volume_container_path.to_str().unwrap_or("/app"),
        )]
    }

    /// Get command as string slices.
    pub fn cmd_slice(&self) -> Vec<&str> {
        self.cmd.iter().map(|s| s.as_str()).collect()
    }
}

/// Constants for container configuration.
pub const DEFAULT_CONTAINER_IMAGE: &str = "dam-container";
pub const OPENCODE_BIN: &str = "/root/.opencode/bin/opencode";

/// Predefined container configurations.
pub mod presets {
    use super::*;

    /// Create an interactive run container.
    pub fn interactive() -> Container {
        Container::new(
            DEFAULT_CONTAINER_IMAGE,
            std::env::current_dir().ok().unwrap_or_default(),
            "",
        )
        .with_command(vec![OPENCODE_BIN.to_string()])
    }

    /// Create an ACP server container.
    pub fn acp_server() -> Container {
        Container::new(
            DEFAULT_CONTAINER_IMAGE,
            std::env::current_dir().ok().unwrap_or_default(),
            "",
        )
        .with_name("dam-acp")
        .with_command(vec![OPENCODE_BIN.to_string(), "acp".to_string()])
    }

    /// Create a web server container.
    pub fn web_server() -> Container {
        Container::new(
            DEFAULT_CONTAINER_IMAGE,
            std::env::current_dir().ok().unwrap_or_default(),
            "",
        )
        .with_name("dam-web")
        .with_command(vec![
            OPENCODE_BIN.to_string(),
            "web".to_string(),
            "--port".to_string(),
            "4096".to_string(),
            "--hostname".to_string(),
            "0.0.0.0".to_string(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_container_new() {
        let container = Container::new("test-image", PathBuf::from("/host/path"), "");

        assert_eq!(container.image, "test-image");
        assert_eq!(container.volume_host_path, PathBuf::from("/host/path"));
        assert!(container.name.is_none());
    }

    #[test]
    fn test_container_with_name() {
        let container = Container::new("img", PathBuf::from("/path"), "").with_name("my-container");

        assert_eq!(container.name, Some("my-container".to_string()));
    }

    #[test]
    fn test_container_with_command() {
        let container = Container::new("img", PathBuf::from("/path"), "")
            .with_command(vec!["bash".to_string(), "-c".to_string()]);

        assert_eq!(container.cmd.len(), 2);
        assert_eq!(container.cmd[0], "bash");
    }

    #[test]
    fn test_volume_mounts() {
        let container = Container::new("img", PathBuf::from("/host/app"), "");
        let mounts = container.volume_mounts();

        assert_eq!(mounts.len(), 1);
        assert_eq!(mounts[0].0, "/host/app");
    }

    #[test]
    fn test_volume_container_path_is_absolute() {
        let container = Container::new("img", PathBuf::from("/host/path"), "");
        let mounts = container.volume_mounts();

        assert!(
            mounts[0].1.starts_with('/'),
            "container path '{}' must be absolute",
            mounts[0].1
        );
    }

    #[test]
    fn test_cmd_slice() {
        let container = Container::new("img", PathBuf::from("/path"), "")
            .with_command(vec!["ls".to_string(), "-la".to_string()]);

        let slice = container.cmd_slice();
        assert_eq!(slice, vec!["ls", "-la"]);
    }

    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_CONTAINER_IMAGE, "dam-container");
        assert_eq!(OPENCODE_BIN, "/root/.opencode/bin/opencode");
    }
}
