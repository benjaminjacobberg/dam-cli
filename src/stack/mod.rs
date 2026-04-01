//! Stack subsystem - discovers and manages development environment stacks.
//!
//! A "stack" represents a development environment configuration (e.g., Java,
//! Python, Node.js) that can be used to build Docker images. Stacks are
//! discovered from `.sh` script files in a designated directory.
//!
//! # Usage
//!
//! ```rust
//! use dam_cli::stack::StackDiscovery;
//! use std::path::PathBuf;
//!
//! let discovery = StackDiscovery::new(PathBuf::from("./stacks"));
//! let stacks = discovery.discover();
//! ```

pub mod discovery;
pub mod entity;

pub use discovery::StackDiscovery;
pub use entity::Stack;
