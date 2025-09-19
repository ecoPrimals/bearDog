// Property-based testing framework for BearDog
//
// This module provides a comprehensive property-based testing framework
// split into focused submodules to maintain code organization.

pub mod types;

pub use types::*;

// Re-export the main framework from the original file for backward compatibility
pub use super::property_based_testing::PropertyBasedTestFramework as LegacyFramework;
