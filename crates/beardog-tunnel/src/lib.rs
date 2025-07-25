//! BearDog tunnel module

pub mod tunnel;
pub mod universal_hsm_discovery;

// Re-export commonly used types and errors
pub use beardog_errors::{BearDogError, BearDogResult};
pub use tunnel::hsm::types::*;
pub use tunnel::hsm::{HsmError, HsmProvider};

// Re-export core functionality
pub use tunnel::*;
pub use universal_hsm_discovery::*;
