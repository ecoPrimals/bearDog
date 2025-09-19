

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod engine;
pub mod strategies;

pub use engine::{DiscoveredCapability, DiscoveryEngineConfig, VendorDiscoveryEngine};
pub use strategies::{DiscoveryStrategy, DiscoveryStrategy as VendorDiscoveryStrategy};
