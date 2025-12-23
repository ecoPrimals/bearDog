//! # Sovereignty Module
//!
//! Provides core sovereignty functionality ensuring human dignity and control
//! throughout the BearDog ecosystem.
//!
//! ## Overview
//!
//! This module implements the fundamental sovereignty principles that
//! distinguish BearDog from traditional systems:
//! - **Human-Owned Entropy**: Cryptographic keys under complete human control
//! - **Genesis Configuration**: Initial sovereignty setup and bootstrapping
//! - **Sovereignty Levels**: Configurable sovereignty guarantees
//!
//! ## Key Components
//!
//! - [`EcosystemConfig`] - Sovereign ecosystem configuration
//! - [`EcosystemManager`] - Manages sovereign ecosystem operations
//! - [`GenesisConfig`] - Genesis block and initial configuration
//! - [`GenesisManager`] - Handles genesis setup and validation
//! - [`SovereigntyConfig`] - Sovereignty policy configuration
//! - [`SovereigntyLevel`] - Defines levels of sovereignty enforcement
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::sovereignty::{SovereigntyConfig, SovereigntyLevel};
//!
//! # fn example() -> Result<(), beardog_errors::BearDogError> {
//! let config = SovereigntyConfig {
//!     level: SovereigntyLevel::Strict,
//!     enforce_human_control: true,
//!     ..Default::default()
//! };
//!
//! // Use configuration to ensure sovereignty guarantees
//! # Ok(())
//! # }
//! ```

/// Ecosystem sovereignty implementation
pub mod ecosystem;

/// Genesis configuration and bootstrapping
pub mod genesis;

/// Sovereignty types and configurations
pub mod types;

pub use crate::ecosystem_simple::{
    SimpleEcosystemConfig as EcosystemConfig, SimpleEcosystemManager as EcosystemManager,
};
pub use genesis::{GenesisConfig, GenesisManager};
pub use types::{SovereigntyConfig, SovereigntyLevel};
