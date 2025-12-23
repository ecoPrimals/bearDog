// **UNIFIED PROVIDER TRAIT SYSTEM**
//
// This module consolidates all provider trait definitions across BearDog into a single,
// coherent hierarchy that eliminates fragmentation and provides a canonical interface.
//
// ## Consolidation Strategy
//
// This unifies and replaces:
// - `beardog-traits::canonical::*Provider` - Canonical provider traits
// - `beardog-traits::unified::*Provider` - Unified provider traits
// - `zero_cost::hsm::HsmProviderTrait` - Zero-cost HSM traits (deprecated)
// - Local provider trait definitions scattered across crates
//
// ## Modern Architecture Principles
//
// - **Single Source of Truth**: All provider traits in one canonical location
// - **Hierarchical Design**: Clear inheritance chain from base to specialized
// - **Zero Fragmentation**: No duplicate trait definitions anywhere
// - **Native Async**: Zero-cost native async/await support throughout
// - **Type Safety**: Strongly typed with comprehensive error handling
// - **Performance Optimized**: Zero-cost abstractions with native async functions

// Modular structure for better maintainability
pub mod base_traits;
pub mod other_traits;
pub mod security_traits;

// Re-export all traits and types for backward compatibility
pub use base_traits::*;
pub use other_traits::*;
pub use security_traits::*;
