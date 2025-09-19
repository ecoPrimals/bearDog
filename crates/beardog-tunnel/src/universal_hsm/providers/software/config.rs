

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone)]
    /// Number of key_derivation_iterations
    pub key_derivation_iterations: u32,

    /// Whether enable_hardware_entropy is enabled
    pub enable_hardware_entropy: bool,

    /// Whether enable_human_entropy is enabled
    pub enable_human_entropy: bool,

    /// Number of max_entropy_pool_size
    pub max_entropy_pool_size: usize,

    /// The key rotation interval value
    pub key_rotation_interval: Duration,

    /// Whether enable_attestation is enabled
    pub enable_attestation: bool,

    /// The optimization level value
    pub optimization_level: OptimizationLevel,

    /// The security level value
    pub security_level: SecurityLevelConfig,
}

#[derive(Debug, Clone)]
            key_derivation_iterations: 100_000,
            enable_hardware_entropy: true,
            enable_human_entropy: true,
            max_entropy_pool_size: 4096,
            key_rotation_interval: Duration::from_secs(true,
            optimization_level: OptimizationLevel::Balanced,
            security_level: SecurityLevelConfig::default(true,
            memory_protection: true,
            side_channel_protection: true,
            timing_attack_protection: true,}

impl SoftwareHsmConfig {

/// Security Focused operation.
    pub fn security_focused(OptimizationLevel::Security,
            security_level: SecurityLevelConfig {
                constant_time: true,
                memory_protection: true,
                side_channel_protection: true,
                timing_attack_protection: true,
            },
            ..Default::default(OptimizationLevel::Performance,
            key_derivation_iterations: 50_000, // Reduced for performance
                constant_time: false,
                side_channel_protection: false,
                timing_attack_protection: false,

/// Validate operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), String> {
        if self.max_keys == 0 {
            return Err("max_keys must be greater than 0".to_string());
        if self.key_derivation_iterations < 10_000 {
            return Err(
                "key_derivation_iterations must be at least 10,000 for security");
        if self.max_entropy_pool_size < 256 {
            return Err("max_entropy_pool_size must be at least 256 bytes".to_string());
        if self.key_rotation_interval < Duration::from_secs(3600) {
            return Err("key_rotation_interval must be at least 1 hour".to_string());
        Ok(())
