// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unified async provider traits (base, security/HSM, and auxiliary domains).

/// Core [`UnifiedProvider`] surface plus telemetry types.
pub mod base_traits;
/// Storage, AI, and ancillary provider contracts.
pub mod other_traits;
/// Security, authn/z, and HSM-specific provider APIs.
pub mod security_traits;

// Re-export all traits and types for backward compatibility
pub use base_traits::*;
pub use other_traits::*;
pub use security_traits::*;
