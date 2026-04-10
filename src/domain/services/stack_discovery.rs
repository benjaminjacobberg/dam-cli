//! Stack discovery - finds available stacks from the filesystem.

use crate::domain::entities::Stack;
use std::path::PathBuf;

/// Discovers available stacks from a directory.
pub struct StackDiscovery {
    /// Base path where stacks are stored.
    stacks_path: PathBuf,
}

impl StackDiscovery {
    pub fn new(stacks_path: PathBuf) -> Self {
        Self { stacks_path }
    }

    /// Discover all available stacks.
    ///
    /// Looks for `.sh` files in the stacks directory and treats
    /// the filename stem as the stack name.
    pub fn discover(&self) -> Vec<Stack> {
        let mut stacks = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&self.stacks_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                // Only consider .sh files
                if path.extension().and_then(|e| e.to_str()) == Some("sh")
                    && let Some(name) = path.file_stem().and_then(|n| n.to_str())
                {
                    stacks.push(Stack::new(name.to_string(), path));
                }
            }
        }

        stacks.sort_by(|a, b| a.name.cmp(&b.name));
        stacks
    }

    /// Format available stacks as a help string.
    pub fn format_help(&self) -> String {
        let stacks = self.discover();

        if stacks.is_empty() {
            "none (place .sh files in stacks/)".to_string()
        } else {
            stacks
                .iter()
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    /// Get a stack by name.
    pub fn get(&self, name: &str) -> Option<Stack> {
        self.discover().into_iter().find(|s| s.name == name)
    }
}

// Make StackDiscovery constructible from a Path
impl From<PathBuf> for StackDiscovery {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_discover_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let discovery = StackDiscovery::new(temp_dir.path().to_path_buf());

        let stacks = discovery.discover();
        assert!(stacks.is_empty());
    }

    #[test]
    fn test_discover_finds_sh_files() {
        let temp_dir = TempDir::new().unwrap();

        // Create test .sh files
        std::fs::write(temp_dir.path().join("python.sh"), "#!/bin/bash").unwrap();
        std::fs::write(temp_dir.path().join("node.sh"), "#!/bin/bash").unwrap();
        std::fs::write(temp_dir.path().join("java.sh"), "#!/bin/bash").unwrap();

        let discovery = StackDiscovery::new(temp_dir.path().to_path_buf());
        let stacks = discovery.discover();

        assert_eq!(stacks.len(), 3);
        let names: Vec<&str> = stacks.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"java"));
        assert!(names.contains(&"node"));
        assert!(names.contains(&"python"));
    }

    #[test]
    fn test_discover_ignores_non_sh_files() {
        let temp_dir = TempDir::new().unwrap();

        std::fs::write(temp_dir.path().join("valid.sh"), "#!/bin/bash").unwrap();
        std::fs::write(temp_dir.path().join("README.md"), "# README").unwrap();
        std::fs::write(temp_dir.path().join("config.yml"), "key: value").unwrap();

        let discovery = StackDiscovery::new(temp_dir.path().to_path_buf());
        let stacks = discovery.discover();

        assert_eq!(stacks.len(), 1);
        assert_eq!(stacks[0].name, "valid");
    }

    #[test]
    fn test_discover_sorted_alphabetically() {
        let temp_dir = TempDir::new().unwrap();

        std::fs::write(temp_dir.path().join("zulu.sh"), "#!/bin/bash").unwrap();
        std::fs::write(temp_dir.path().join("alpha.sh"), "#!/bin/bash").unwrap();
        std::fs::write(temp_dir.path().join("middle.sh"), "#!/bin/bash").unwrap();

        let discovery = StackDiscovery::new(temp_dir.path().to_path_buf());
        let stacks = discovery.discover();

        assert_eq!(stacks[0].name, "alpha");
        assert_eq!(stacks[1].name, "middle");
        assert_eq!(stacks[2].name, "zulu");
    }

    #[test]
    fn test_format_help_empty() {
        let temp_dir = TempDir::new().unwrap();
        let discovery = StackDiscovery::new(temp_dir.path().to_path_buf());

        let help = discovery.format_help();
        assert!(help.contains("none"));
    }

    #[test]
    fn test_format_help_with_stacks() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("python.sh"), "").unwrap();
        std::fs::write(temp_dir.path().join("node.sh"), "").unwrap();

        let discovery = StackDiscovery::new(temp_dir.path().to_path_buf());
        let help = discovery.format_help();

        assert!(help.contains("node"));
        assert!(help.contains("python"));
    }

    #[test]
    fn test_get_existing_stack() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("python.sh"), "").unwrap();

        let discovery = StackDiscovery::new(temp_dir.path().to_path_buf());
        let stack = discovery.get("python");

        assert!(stack.is_some());
        assert_eq!(stack.unwrap().name, "python");
    }

    #[test]
    fn test_get_nonexistent_stack() {
        let temp_dir = TempDir::new().unwrap();
        let discovery = StackDiscovery::new(temp_dir.path().to_path_buf());

        let stack = discovery.get("nonexistent");
        assert!(stack.is_none());
    }
}
