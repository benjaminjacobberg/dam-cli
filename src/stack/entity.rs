//! Stack entity representing a development environment configuration.
//!
//! # Example
//!
//! ```
//! use dam_cli::stack::Stack;
//! use std::path::PathBuf;
//! use std::fs;
//!
//! // Create a temp script file for the doctest
//! let temp_dir = std::env::temp_dir();
//! let script_path = temp_dir.join("python.sh");
//! fs::write(&script_path, "#!/bin/bash").ok();
//!
//! let stack = Stack::new("python", script_path);
//! assert_eq!(stack.name, "python");
//! assert!(stack.is_valid());
//! ```

use std::path::PathBuf;

/// Represents a development stack (e.g., Java, Python, Node.js).
#[derive(Debug, Clone)]
pub struct Stack {
    /// The name of the stack (e.g., "java", "python").
    pub name: String,
    /// Path to the stack's initialization script.
    pub script_path: PathBuf,
}

impl Stack {
    pub fn new(name: impl Into<String>, script_path: PathBuf) -> Self {
        Self {
            name: name.into(),
            script_path,
        }
    }

    /// Validate that the stack has a valid script.
    pub fn is_valid(&self) -> bool {
        self.script_path.exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_stack_new() {
        let temp_dir = TempDir::new().unwrap();
        let stack = Stack::new("python", temp_dir.path().join("python.sh"));

        assert_eq!(stack.name, "python");
        assert_eq!(stack.script_path, temp_dir.path().join("python.sh"));
    }

    #[test]
    fn test_is_valid_when_file_exists() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.sh");
        std::fs::write(&path, "").unwrap();

        let stack = Stack::new("test", path);
        assert!(stack.is_valid());
    }

    #[test]
    fn test_is_valid_when_file_missing() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("missing.sh");

        let stack = Stack::new("missing", path);
        assert!(!stack.is_valid());
    }
}
