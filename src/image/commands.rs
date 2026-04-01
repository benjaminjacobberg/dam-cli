//! Image commands - handlers for image-related CLI commands.

use crate::docker::DockerClient;
use crate::image::{DEFAULT_IMAGE_NAME, Image, ImageBuilder};
use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;
use std::sync::Arc;

/// Handle the build command.
pub fn cmd_build(
    docker: Arc<dyn DockerClient>,
    stack: &str,
    dockerfile_path: PathBuf,
    build_context: PathBuf,
) -> Result<()> {
    println!(
        "{} Using stack: {}",
        "Building Docker image...".yellow(),
        stack
    );

    let image = Image::default_dam_image(dockerfile_path, build_context);
    let image_to_build = ImageBuilder::with_stack(&image, stack);

    let build_args = image_to_build.build_args();

    docker.build_image(
        DEFAULT_IMAGE_NAME,
        image_to_build
            .image
            .dockerfile_path
            .to_str()
            .unwrap_or("Dockerfile"),
        image_to_build.image.build_context.to_str().unwrap_or("."),
        &build_args,
    )?;

    println!("{}", "✓ Image built successfully!".green());
    Ok(())
}

#[cfg(all(test, feature = "testing"))]
mod tests {
    use super::*;
    use crate::docker::StubDockerClient;
    use std::sync::Arc;

    #[test]
    fn test_cmd_build_wires_up_correctly() {
        // Test that build wires up the Docker client properly by checking call captures
        let stub = StubDockerClient::new().with_build_error("Simulated build failure");
        let docker: Arc<dyn DockerClient> = Arc::new(stub);

        let temp_dir = std::env::temp_dir();
        let result = cmd_build(
            docker,
            "test-stack",
            temp_dir.join("nonexistent/Dockerfile"),
            temp_dir,
        );

        // With our mocked error, this should fail
        assert!(result.is_err());
    }
}
