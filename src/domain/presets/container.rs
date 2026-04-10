//! Container presets - predefined container configurations.

use crate::domain::entities::{Container, DEFAULT_CONTAINER_IMAGE, OPENCODE_BIN};
use std::path::PathBuf;

/// Get the OpenCode config directory path (~/.config/opencode).
fn opencode_config_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".config/opencode"))
}

/// Create an interactive run container.
pub fn interactive() -> Container {
    let mut container = Container::new(
        DEFAULT_CONTAINER_IMAGE,
        std::env::current_dir().ok().unwrap_or_default(),
        "",
    )
    .with_command(vec![OPENCODE_BIN.to_string()]);

    // Auto-mount ~/.config/opencode if it exists on the host
    if let Some(config_dir) = opencode_config_dir() {
        if config_dir.exists() {
            container =
                container.with_volume_mount(config_dir, PathBuf::from("/root/.config/opencode"));
        }
    }

    container
}

/// Create an ACP server container.
pub fn acp_server() -> Container {
    let mut container = Container::new(
        DEFAULT_CONTAINER_IMAGE,
        std::env::current_dir().ok().unwrap_or_default(),
        "",
    )
    .with_name("dam-acp")
    .with_command(vec![OPENCODE_BIN.to_string(), "acp".to_string()]);

    // Auto-mount ~/.config/opencode if it exists on the host
    if let Some(config_dir) = opencode_config_dir() {
        if config_dir.exists() {
            container =
                container.with_volume_mount(config_dir, PathBuf::from("/root/.config/opencode"));
        }
    }

    container
}

/// Create a web server container.
pub fn web_server() -> Container {
    let mut container = Container::new(
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
    ]);

    // Auto-mount ~/.config/opencode if it exists on the host
    if let Some(config_dir) = opencode_config_dir() {
        if config_dir.exists() {
            container =
                container.with_volume_mount(config_dir, PathBuf::from("/root/.config/opencode"));
        }
    }

    container
}
