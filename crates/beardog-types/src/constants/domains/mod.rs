// Domain-Specific Constants
//
// This module provides domain-organized constants that replace the large monolithic
// unified.rs file with maintainable, modular constant definitions.
//
// ## Architecture
//
// Constants are organized by functional domain:
// - **System**: Core system constants, versions, limits, timeouts
// - **Network**: Network addresses, ports, headers, timeouts
// - **Security**: Cryptographic, authentication, session constants
// - **Database**: Database connection, query, schema constants
// - **Monitoring**: Metrics, logging, health check constants

// Domain-specific constant modules
pub mod buffers;
pub mod config;
pub mod ecosystem;
pub mod limits; // NEW - Nov 11, 2025
pub mod math;
pub mod network;
pub mod pkcs11;
pub mod security;
pub mod storage;
pub mod system;
pub mod timeouts; // NEW - Nov 11, 2025
pub mod validation;

// Test modules
#[cfg(test)]
mod buffers_tests;
#[cfg(test)]
mod limits_tests;
#[cfg(test)]
mod timeouts_tests;

// Re-export commonly used constants for convenience
pub use system::{defaults::*, limits::MAX_CONNECTIONS, versions::BEARDOG_VERSION};

pub use network::{
    addresses::DEFAULT_DNS_PORT,
    // Note: Use default_metrics_bind() for environment-aware configuration
    timeouts::CONNECTION_TIMEOUT,
};

pub use security::{
    auth::{MAX_AUTH_ATTEMPTS, MIN_PASSWORD_LENGTH},
    crypto::DEFAULT_KEY_LENGTH,
};
