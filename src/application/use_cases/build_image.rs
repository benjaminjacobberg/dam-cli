//! Build image use case.

use crate::domain::entities::DEFAULT_IMAGE_NAME;
use crate::domain::entities::Image;
use crate::domain::value_objects::ImageBuilder;
use crate::ports::outbound::ContainerPort;
use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;
use std::sync::Arc;

/// Handle the build command.
pub fn build_image(
    docker: Arc<dyn ContainerPort>,
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
    use crate::adapters::driven::test::StubDockerAdapter;
    use std::sync::Arc;

    #[test]
    fn test_build_image_wires_up_correctly() {
        let stub = StubDockerAdapter::new().with_build_error("Simulated build failure");
        let docker: Arc<dyn ContainerPort> = Arc::new(stub);

        let temp_dir = std::env::temp_dir();
        let result = build_image(
            docker,
            "test-stack",
            temp_dir.join("nonexistent/Dockerfile"),
            temp_dir,
        );

        assert!(result.is_err());
    }
}
