//! Driven adapters module.

pub mod cli_docker;

#[cfg(feature = "testing")]
pub mod test;

pub use cli_docker::CliDockerAdapter;
