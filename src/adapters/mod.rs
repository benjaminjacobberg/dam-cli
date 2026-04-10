//! Adapters layer - concrete implementations of ports.
//!
//! Driven adapters implement outbound ports and provide the actual
//! infrastructure implementations (Docker CLI, HTTP clients, etc.).
//!
//! Driving adapters would be placed in `adapters/driving/` but for this
//! CLI application, main.rs serves as the driving adapter.

pub mod driven;
