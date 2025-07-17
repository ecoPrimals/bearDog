//! BearDog adapters module

pub mod adapters;
pub mod ecosystem_integration;

// Re-export commonly used items
pub use adapters::*;
pub use ecosystem_integration::{EcosystemError, EcosystemIntegration};
