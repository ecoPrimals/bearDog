// SPDX-License-Identifier: AGPL-3.0-or-later

//! Capability detector module
//!
//! NOTE: Full capability detection lives under `crate::universal_hsm_discovery`; this file is a stub.

/// Stub capability detector for compatibility
#[derive(Debug, Clone, Default)]
pub struct CapabilityDetector;

impl CapabilityDetector {
    /// Create new capability detector
    pub const fn new() -> Self {
        Self
    }
}
