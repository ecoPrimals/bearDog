//! Configuration management for BearDog Security Manager
//!
//! This crate provides comprehensive configuration management for BearDog,
//! including environment variable handling, file-based configuration,
//! and validation.

pub mod constants;
pub mod core;
pub mod integration;
pub mod monitoring;
pub mod network;
pub mod security;

// Re-export commonly used types
pub use constants::*;
pub use core::BearDogConfig;
pub use integration::*;
pub use monitoring::*;
pub use network::{
    HttpConfig, HttpsConfig, NetworkConfig, NodeCommunicationConfig, RateLimitConfig, RetryConfig,
    WebSocketConfig,
};
pub use security::{
    AuditConfig, ComplianceConfig, EncryptionConfig, SecurityConfig, ThreatDetectionConfig,
};
