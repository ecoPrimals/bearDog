// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM capability detector.
//!
//! Detects which hardware-backed key storage is available at runtime
//! (TPM 2.0, Android `StrongBox`, Apple Secure Enclave, PKCS#11 tokens).
//! Currently returns an empty capability set; detection probes will be
//! wired per-platform behind feature flags.

/// Detects available HSM/hardware-backed key storage on the current platform.
///
/// The detector is instantiated at startup and queried by the HSM manager
/// to select the best available backend. Until platform probes are wired,
/// it reports no hardware capabilities, causing the manager to fall back
/// to the software HSM.
#[derive(Debug, Clone, Default)]
pub struct CapabilityDetector;

impl CapabilityDetector {
    /// Create a new detector (currently no-op; probes are platform-gated).
    pub const fn new() -> Self {
        Self
    }
}
