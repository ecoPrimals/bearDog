// Constants module - unified constants for the BearDog ecosystem
//
// **MODERNIZED CONSTANTS SYSTEM** - Domain-organized constants that eliminate fragmentation
//
// This module provides the single source of truth for all constants across BearDog,
// organized by functional domain for better maintainability and discoverability.

// **CANONICAL CONSTANTS SYSTEM** - Domain-based organization (PREFERRED)
/// Domains module
pub mod domains;

// **PRIMARY EXPORTS** - Domain-organized constants (use these)
pub use domains::{
    network,  // Network constants (ports, timeouts, addresses)
    security, // Security constants (auth, crypto, sessions)
    system,   // System constants (versions, limits, defaults)
};

// **CONVENIENCE RE-EXPORTS** - Commonly used constants
pub use domains::system::{
    defaults::{DEFAULT_BUFFER_SIZE, DEFAULT_CACHE_SIZE, DEFAULT_POOL_SIZE},
    limits::{MAX_CONNECTIONS, MAX_MEMORY_USAGE},
    timeouts::{CONNECTION_TIMEOUT, REQUEST_TIMEOUT},
    versions::BEARDOG_VERSION,
};

pub use domains::network::{
    addresses::DEFAULT_DNS_PORT,
    // Note: Use default_metrics_bind() for environment-aware configuration
    timeouts::RETRY_TIMEOUT,
};

pub use domains::security::{
    auth::{MAX_AUTH_ATTEMPTS, MIN_PASSWORD_LENGTH},
    crypto::DEFAULT_KEY_LENGTH,
};

// **MIGRATION COMPLETE** - Legacy constants systems have been removed
// All constants are now organized in the domains:: hierarchy for better maintainability
