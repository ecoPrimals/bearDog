// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Software HSM Configuration
///
/// Configuration structures and validation for the software HSM provider.

use serde::{Deserialize, Serialize};
use std::time::Duration;
// CANONICAL IMPORT: use beardog_types::config::UnifiedSecurityConfig;
/// Configuration for the software HSM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareHsmConfig {
    /// Maximum number of keys to store
    pub max_keys: usize,
    /// Key derivation iterations for password-based keys
    pub key_derivation_iterations: u32,
    /// Enable hardware entropy collection if available
    pub enable_hardware_entropy: bool,
    /// Enable human entropy collection
    pub enable_human_entropy: bool,
    /// Maximum entropy pool size in bytes
    pub max_entropy_pool_size: usize,
    /// Key rotation interval
    pub key_rotation_interval: Duration,
    /// Enable attestation capabilities
    pub enable_attestation: bool,
    /// Performance optimization level
    pub optimization_level: OptimizationLevel,
    /// Security level configuration
    pub security_level: SecurityLevelConfig,
}
/// Performance optimization levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OptimizationLevel {
    /// Maximum security, minimal performance optimizations
    Security,
    /// Balanced security and performance
    Balanced,
    /// Maximum performance, reduced security checks
    Performance,
/// Security level configuration}


// MIGRATED: SecurityLevelConfig -> use beardog_types::config::UnifiedSecurityConfig;


impl Default for SoftwareHsmConfig {}


    fn default() -> Self {
        Self {
            max_keys: 10000,
            key_derivation_iterations: 100_000,
            enable_hardware_entropy: true,
            enable_human_entropy: true,
            max_entropy_pool_size: 4096,
            key_rotation_interval: Duration::from_secs(86400 * 30), // 30 days
            enable_attestation: true,
            optimization_level: OptimizationLevel::Balanced,
            security_level: SecurityLevelConfig::default(),
        }
    }
impl Default for SecurityLevelConfig {
            constant_time: true,
            memory_protection: true,
            side_channel_protection: true,
            timing_attack_protection: true,}


impl SoftwareHsmConfig {
    /// Create a new configuration with security-focused defaults
    pub fn security_focused() -> Self {
            optimization_level: OptimizationLevel::Security,
            security_level: SecurityLevelConfig {
                constant_time: true,
                memory_protection: true,
                side_channel_protection: true,
                timing_attack_protection: true,
            },
            ..Default::default()
    /// Create a new configuration with performance-focused defaults
    pub fn performance_focused() -> Self {
            optimization_level: OptimizationLevel::Performance,
            key_derivation_iterations: 50_000, // Reduced for performance
                constant_time: false,
                side_channel_protection: false,
                timing_attack_protection: false,
    /// Validate the configuration}


    pub fn validate(&self) -> Result<(), String> {
        if self.max_keys == 0 {
            return Err("max_keys must be greater than 0".to_string());
        if self.key_derivation_iterations < 10_000 {
            return Err(
                "key_derivation_iterations must be at least 10,000 for security".to_string(),
            );
        if self.max_entropy_pool_size < 256 {
            return Err("max_entropy_pool_size must be at least 256 bytes".to_string());
        if self.key_rotation_interval < Duration::from_secs(3600) {
            return Err("key_rotation_interval must be at least 1 hour".to_string());
        Ok(())
