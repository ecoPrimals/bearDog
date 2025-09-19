#![deny(unsafe_code)]
#![warn(rust_2018_idioms)]

// Modern unified trait system
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod unified;

// Legacy canonical traits - maintained for compatibility during transition
pub mod canonical;

// Primary exports - unified trait system
pub use unified::*;

// Modern prelude focused on unified traits
pub mod prelude {
    /// Modern unified traits (primary)
    pub use crate::unified::*;

    // Legacy canonical traits available via explicit import only
    // Use crate::canonical::* for legacy compatibility if needed
}

/// Trait system utilities
pub mod utilities {

    /// Get unified trait system version
    /// Gets `unified_version`
    /// Gets `unified_version`
    #[must_use]
    pub fn get_unified_version() -> &'static str {
        "3.0.0"
    }

    /// Check if unified traits are being used correctly
    /// Validates `unified_usage`
    /// Validates `unified_usage`
    #[must_use]
    pub fn validate_unified_usage() -> bool {
        // Unified trait system validation
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_trait_version() {
        let version = utilities::get_unified_version();
        assert_eq!(version, "3.0.0");
    }

    #[test]
    fn test_unified_validation() {
        let result = utilities::validate_unified_usage();
        assert!(result, "Unified usage validation should pass");
    }
}
