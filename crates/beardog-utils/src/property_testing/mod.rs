//! Property-Based Testing Framework - Canonical Location
//!
//! **Unified property-based testing framework** - consolidated from scattered implementations.
//!
//! This module provides comprehensive property-based testing capabilities for `BearDog`,
//! with all implementations consolidated into a single, maintainable location.
//!
//! ## Migration Note
//! This replaces the fragmented `property_based_testing` module and standalone file.
//! All property testing functionality is now centralized here.

// External imports needed by property implementations
// Note: These imports are used by submodules
#[allow(unused_imports)]
use beardog_errors::BearDogError;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::fmt::Debug;
#[allow(unused_imports)]
use std::time::{Duration, Instant};
#[allow(unused_imports)]
use tracing::{debug, info, warn};

// Core types and configuration
pub mod types;

// Property implementations - Re-enabled and testing
pub mod api_properties;
pub mod config_properties;
pub mod crypto_properties;
pub mod mock_implementations;

// Re-export primary types for convenient access
pub use types::*;

// Backward compatibility re-exports
#[deprecated(
    since = "3.0.2",
    note = "Use property_testing module directly instead of property_based_testing"
)]
pub use types::PropertyBasedTestFramework as LegacyFramework;
