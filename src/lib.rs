//! DAM CLI - Docker management for OpenCode
//!
//! A hexagonal architecture CLI to build, run, and teardown OpenCode containers.

pub mod adapters;
pub mod application;
pub mod domain;
pub mod ports;

pub use adapters::driven::CliDockerAdapter;
pub use application::services::CliApplicationService;
pub use ports::outbound::ContainerPort;

#[cfg(feature = "testing")]
pub use adapters::driven::test::StubDockerAdapter;
