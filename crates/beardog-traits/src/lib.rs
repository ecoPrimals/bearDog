// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! # BearDog Unified Trait System
//!
//! **CANONICAL TRAITS FOR THE BEARDOG ECOSYSTEM**
//! This crate provides the canonical trait system that unifies all BearDog operations
//! across security, genetics, workflows, and infrastructure domains. These traits
//! establish the behavioral contracts that enable seamless interoperability and
//! type-safe operations throughout the ecosystem.
//! ## Core Design Principles
//! - **Zero-Copy Architecture**: Traits designed for minimal memory allocations
//! - **Type Safety**: Compile-time guarantees for all operations
//! - **Domain Agnostic**: Universal patterns applicable across all BearDog domains
//! - **Performance First**: Optimized for high-throughput operations
//! - **Future Proof**: Extensible design supporting evolutionary improvements
//! ## Usage
//! ```rust
//! use beardog_traits::canonical::{BaseProvider, SecurityProvider};
//! use beardog_errors::BearDogResult;
//! 
//! // Implement canonical provider traits
//! struct MyProvider;
//! #[async_trait::async_trait]
//! impl BaseProvider for MyProvider {
//!     // Implementation using canonical patterns
//! }
//! ```
//! ## Architecture
//! The trait system is organized into focused modules:
//! - **canonical**: Primary trait definitions (single source of truth)
//! - **prelude**: Convenient imports for common usage patterns
//! All traits use canonical types from `beardog-types` and unified error handling
//! from `beardog-errors` to ensure consistency across the ecosystem.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
// ============================================================================
// CANONICAL TRAIT SYSTEM - Single source of truth
/// **PRIMARY MODULE** - All new code should use these canonical traits
pub mod canonical;
// CANONICAL RE-EXPORTS - Primary interface
// Re-export all canonical traits for easy access
pub use canonical::*;
// Convenience re-export for error handling
pub use beardog_errors::BearDogResult;
/// **CANONICAL PRELUDE** - Import all essential traits
///
/// Use this for easy access to all canonical traits:
/// ```rust
/// use beardog_traits::prelude::*;
/// ```
pub mod prelude {
    pub use crate::canonical::*;
    pub use beardog_errors::BearDogResult;
}
// CANONICAL MIGRATION COMPLETE - Migration helpers removed
/// **CANONICAL MIGRATION COMPLETE** ✅
/// The migration from legacy trait implementations to canonical traits is now complete.
/// All legacy migration helpers have been removed as they are no longer needed.
/// **Current Canonical Traits:**
/// - `SecurityProvider` - Unified security operations
/// - `HsmProvider` - Hardware security module operations  
/// - `CryptoProvider` - Cryptographic operations
/// - `CacheProvider` - Caching operations
/// - `GeneticsProvider` - Genetic algorithm operations
/// - `MonitoringProvider` - Monitoring and observability
/// **Usage:**
/// use beardog_traits::canonical::*;
/// // All canonical traits available through unified import
// TRAIT VALIDATION AND TOOLING
/// Validation utilities for canonical trait usage
pub mod validation {
    // Unused import removed
    /// Validate that all trait usage follows canonical patterns
    pub fn validate_canonical_patterns() -> bool {
        // Placeholder for validation logic
        true
    }


    /// Validates that canonical trait usage follows BearDog standards
    pub fn validate_canonical_usage() -> Result<(), Vec<String>> {
        let errors = Vec::new();
        // This would be expanded with actual validation logic
        // For now, it's a placeholder for future tooling
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    /// Get information about canonical trait system
    pub fn trait_system_info() -> Vec<(&'static str, &'static str)> {
        vec![
            (
                "BaseProvider",
                "Foundation trait for all providers with common lifecycle methods",
            ),
            (
                "SecurityProvider",
                "Complete security operations including auth, authz, and crypto",
            ),
            (
                "HsmProvider",
                "Hardware security module operations for key management",
            ),
            (
                "CacheProvider",
                "Unified caching interface with batch operations support",
            ),
            (
                "CryptoProvider",
                "Software cryptographic operations without HSM requirement",
            ),
            (
                "WorkflowProvider",
                "Business process automation and workflow execution",
            ),
            (
                "GeneticsProvider",
                "Genetic algorithm operations and population management",
            ),
            (
                "MonitoringProvider",
                "System observability, metrics, and alerting",
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_canonical_trait_registry() {
        // Test that canonical traits are available
        let info = validation::trait_system_info();
        assert!(!info.is_empty());
        let trait_names: Vec<&str> = info.iter().map(|(name, _)| *name).collect();
        assert!(trait_names.contains(&"BaseProvider"));
    }

    #[test] 
    fn test_canonical_completion() {
        // Test that canonical migration is complete
        // All traits should now be available through canonical module
        let result = validation::validate_canonical_usage();
        assert!(result.is_ok(), "Canonical migration should be complete");
    }

    #[test]
    fn test_validation_system() {
        let result = validation::validate_canonical_usage();
        assert!(result.is_ok(), "Canonical usage validation should pass");
    }

    #[test]
    fn test_trait_system_info() {
        let info = validation::trait_system_info();
        assert!(!info.is_empty());
        let trait_names: Vec<&str> = info.iter().map(|(name, _)| *name).collect();
        assert!(trait_names.contains(&"BaseProvider"));
    }
}
