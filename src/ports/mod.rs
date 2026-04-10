//! Ports layer - interfaces defining boundaries.
//!
//! Ports define the contracts between the application/core domain
//! and external systems. They are divided into:
//! - Inbound ports: Define what drivers can invoke
//! - Outbound ports: Define contracts for external dependencies

pub mod outbound;

pub use outbound::{ContainerInfo, ContainerPort};
