// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

// Neural API auto-registration for Tower Atomic
pub mod neural_registration;

pub use beardog_core as core;
pub use beardog_errors as errors;
pub use beardog_errors::BearDogError;
pub use beardog_types::canonical::*;

// Re-export version and mission from centralized constants
pub use beardog_types::constants::domains::ecosystem::version::{MISSION, VERSION};
