//! Image entity representing a Docker image configuration.
//!
//! # Example
//!
//! ```
//! use dam_cli::image::Image;
//! use std::path::PathBuf;
//!
//! let image = Image::new(
//!     "my-image",
//!     PathBuf::from("./Dockerfile"),
//!     PathBuf::from("."),
//! );
//! assert_eq!(image.name, "my-image");
//! ```

use std::path::PathBuf;

/// Represents a Docker image configuration for building.
#[derive(Debug, Clone)]
pub struct Image {
    /// The name/tag of the image.
    pub name: String,
    /// Path to the Dockerfile.
    pub dockerfile_path: PathBuf,
    /// Build context directory.
    pub build_context: PathBuf,
}

impl Image {
    pub fn new(name: impl Into<String>, dockerfile_path: PathBuf, build_context: PathBuf) -> Self {
        Self {
            name: name.into(),
            dockerfile_path,
            build_context,
        }
    }

    /// Create the default DAM image.
    pub fn default_dam_image(dockerfile_path: PathBuf, build_context: PathBuf) -> Self {
        Self::new("dam-container", dockerfile_path, build_context)
    }
}

/// Constants for the default image configuration.
pub const DEFAULT_IMAGE_NAME: &str = "dam-container";

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_image_new() {
        let image = Image::new(
            "test-image",
            PathBuf::from("/path/to/Dockerfile"),
            PathBuf::from("/path/to/context"),
        );

        assert_eq!(image.name, "test-image");
        assert_eq!(image.dockerfile_path, PathBuf::from("/path/to/Dockerfile"));
        assert_eq!(image.build_context, PathBuf::from("/path/to/context"));
    }

    #[test]
    fn test_default_dam_image() {
        let image =
            Image::default_dam_image(PathBuf::from("/path/Dockerfile"), PathBuf::from("/context"));

        assert_eq!(image.name, "dam-container");
        assert_eq!(image.dockerfile_path, PathBuf::from("/path/Dockerfile"));
        assert_eq!(image.build_context, PathBuf::from("/context"));
    }

    #[test]
    fn test_default_image_name_constant() {
        assert_eq!(DEFAULT_IMAGE_NAME, "dam-container");
    }
}
