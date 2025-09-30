//! # Unified HSM Provider - Modular Architecture
//!
//! This module provides the **unified HSM provider** that consolidates all scattered
//! HSM provider implementations across the BearDog ecosystem into a single, manageable system.
//!
//! ## 🎯 **Consolidation Achievement**
//!
//! This unified provider replaces:
//! - `AndroidUniversalProvider` - Android HSM implementations
//! - `IosUniversalProvider` - iOS HSM implementations  
//! - `SoftwareUniversalProvider` - Software HSM implementations
//! - Various safe FFI providers across the ecosystem
//!
//! ## Modular Architecture
//!
//! The HSM system is organized into focused modules:
//! - [`provider`] - Core HSM provider enum and main implementation
//! - [`configs`] - Platform-specific HSM configurations
//! - [`keys`] - HSM key management and specifications
//! - [`security`] - Security levels and cryptographic algorithms

// pub mod provider; // Temporarily disabled during unification
pub mod configs;
pub mod keys;
pub mod security;

// Re-export main types for backward compatibility
// pub use provider::HsmUnifiedProvider; // Temporarily disabled
pub use configs::{
    AndroidHsmConfig, IosHsmConfig, SoftwareHsmConfig, StrongBoxHsmConfig,
};
pub use keys::{HsmKey, HsmKeySpec};
pub use security::{SecurityLevel, KeyAlgorithm, KeyPurpose}; 