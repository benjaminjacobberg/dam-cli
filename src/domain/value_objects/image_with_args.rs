//! Image builder - constructs build configurations.
//!
//! # Example
//!
//! ```
//! use dam_cli::domain::entities::Image;
//! use dam_cli::domain::value_objects::ImageBuilder;
//! use std::path::PathBuf;
//!
//! let image = Image::new("test", PathBuf::from("Dockerfile"), PathBuf::from("."));
//! let builder = ImageBuilder::with_stack(&image, "python");
//!
//! let args = builder.build_args();
//! assert_eq!(args, vec![("STACK_SCRIPT", "python")]);
//! ```

use crate::domain::entities::Image;

/// Builds Image configurations with sensible defaults.
pub struct ImageBuilder;

impl ImageBuilder {
    /// Build an image with STACK_SCRIPT build arg.
    pub fn with_stack(image: &Image, stack: &str) -> ImageWithBuildArgs {
        ImageWithBuildArgs {
            image: image.clone(),
            stack: stack.to_string(),
        }
    }
}

/// An image prepared for building with build arguments.
pub struct ImageWithBuildArgs {
    pub image: Image,
    pub stack: String,
}

impl ImageWithBuildArgs {
    /// Convert to build args tuple.
    pub fn build_args(&self) -> Vec<(&str, &str)> {
        vec![("STACK_SCRIPT", self.stack.as_str())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_image_builder_with_stack() {
        let image = Image::new("test", PathBuf::from("/Dockerfile"), PathBuf::from("/ctx"));
        let builder = ImageBuilder::with_stack(&image, "python");

        assert_eq!(builder.image.name, "test");
        assert_eq!(builder.stack, "python");
    }

    #[test]
    fn test_image_with_build_args_build_args() {
        let image = Image::new("test", PathBuf::from("/Dockerfile"), PathBuf::from("/ctx"));
        let builder = ImageBuilder::with_stack(&image, "nodejs");

        let args = builder.build_args();
        assert_eq!(args.len(), 1);
        assert_eq!(args[0], ("STACK_SCRIPT", "nodejs"));
    }
}
