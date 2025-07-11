//! Configuration management for BearDog
//!
//! **Provides secure-by-default configuration loading and validation.**
//! 
//! This module was refactored from a large file to improve maintainability.
//! The configuration system provides comprehensive settings for security,
//! networking, monitoring, and system integration.
//!
//! ## Module Structure
//!
//! * **core**: Main configuration types and validation
//! * **security**: Security-related configuration (encryption, HSM, MFA, etc.)
//! * **network**: Network and API configuration (TLS, CORS, rate limiting, etc.)
//! * **monitoring**: Monitoring and observability configuration (logging, metrics, etc.)
//! * **integration**: System integration configuration (adapters, workflows, etc.)

// Re-export all configuration types
pub use core::*;
pub use security::*;
pub use network::*;
pub use monitoring::*;
pub use integration::*;

// Module declarations
pub mod core;
pub mod security;
pub mod network;
pub mod monitoring;
pub mod integration; 