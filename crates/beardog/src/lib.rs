// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub use beardog_core as core;
pub use beardog_errors as errors;
pub use beardog_traits as traits;
pub use beardog_types::config;

pub use beardog_errors::BearDogError;
pub use beardog_types::canonical::*;


// Re-export version and mission from centralized constants
pub use beardog_types::constants::domains::ecosystem::version::{VERSION, MISSION};
