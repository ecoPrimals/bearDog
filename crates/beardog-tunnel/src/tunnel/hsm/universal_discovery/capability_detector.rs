// SPDX-License-Identifier: AGPL-3.0-only

//! Capability detector module
//!
//! NOTE: This module depends on universal_hsm_discovery which is currently
//! commented out pending systematic rebuild.

// pub use crate::universal_hsm_discovery::capability_detection::*;

/// Stub capability detector for compatibility
#[derive(Debug, Clone, Default)]
pub struct CapabilityDetector;

impl CapabilityDetector {
    /// Create new capability detector
    pub fn new() -> Self {
        Self
    }
}
