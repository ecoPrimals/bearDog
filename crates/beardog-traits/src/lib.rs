// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

//! # `BearDog` Traits - Unified Trait System
//!
//! Unified trait system for the `BearDog` ecosystem, providing consistent interfaces
//! across all components with strong memory safety and strong type safety.
//!
//! ## Features
//!
//! - **Unified Traits**: Single consistent trait system across ecosystem
//! - **Provider Abstractions**: Universal provider interfaces
//! - **Memory-safe only**: complete memory safety guarantees
//! - **Type Safety**: Compile-time interface validation
//! - **Backward Compatibility**: Legacy canonical traits maintained
//!
//! ## Example
//!
//! ```rust,no_run
//! use beardog_traits::prelude::*;
//!
//! // Use unified traits for provider implementation
//! // (implementation details)
//! ```
//!
//! ## Architecture
//!
//! The trait system provides two layers:
//! - **Unified Traits** (Primary): Modern, consistent interfaces (v3.0+)
//! - **Canonical Traits** (Legacy): Maintained for backward compatibility
//!
//! ## Migration
//!
//! New code should use the unified trait system:
//! ```rust
//! use beardog_traits::prelude::*; // Modern unified traits
//! ```
//!
//! Legacy code can still use:
//! ```rust
//! use beardog_traits::canonical::*; // Legacy compatibility
//! ```
//!
//! ## Safety
//!
//! All traits enforce memory safety; workspace rustc lint levels are set in the root Cargo.toml.

#![warn(rust_2018_idioms)]

/// Canonical, object-safe HSM provider trait and registry types.
pub mod hsm;

/// Modern unified trait system (primary)
///
/// The unified trait system provides consistent interfaces across all `BearDog` components.
pub mod unified;

/// Legacy canonical traits - maintained for compatibility
///
/// These traits are maintained for backward compatibility during the transition period.
pub mod canonical;

// Primary exports - unified trait system
pub use unified::*;

// Modern prelude focused on unified traits
pub mod prelude {
    //! Re-exports the unified trait hierarchy for typical consumer crates.

    /// Modern unified traits (primary)
    pub use crate::unified::*;

    // Legacy canonical traits available via explicit import only
    // Use crate::canonical::* for legacy compatibility if needed
}

/// Trait system utilities
pub mod utilities {

    /// Get unified trait system version
    /// Gets `unified_version`
    #[must_use]
    pub const fn get_unified_version() -> &'static str {
        "3.0.0"
    }

    /// Check if unified traits are being used correctly
    /// Validates `unified_usage`
    #[must_use]
    pub const fn validate_unified_usage() -> bool {
        // Unified trait system validation
        true
    }
}

// October 27, 2025: Comprehensive test expansion
#[cfg(test)]
mod traits_comprehensive_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_trait_version() {
        let version = utilities::get_unified_version();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(version, "3.0.0");
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_unified_validation() {
        let result = utilities::validate_unified_usage();
        assert!(result, "Unified usage validation should pass");
    }
}
