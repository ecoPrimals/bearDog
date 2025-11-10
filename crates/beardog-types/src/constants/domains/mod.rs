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
/// Buffers module
pub mod buffers;
/// Ecosystem module
pub mod ecosystem;
/// Math module
pub mod math;
/// Network module
pub mod network;
/// PKCS#11 module
pub mod pkcs11;
/// Security module
pub mod security;
/// Storage module
pub mod storage;
/// System module
pub mod system;
/// Validation module
pub mod validation;

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
