//! # Discovery Configuration
//!
//! Re-exports canonical discovery configuration.
//!
//! **Migration Note**: This module now uses the canonical DiscoveryConfig from
//! `beardog_types::canonical::config::domains::discovery` instead of a local definition.

// Use canonical DiscoveryConfig
pub use beardog_types::canonical::config::domains::discovery::DiscoveryConfig; 