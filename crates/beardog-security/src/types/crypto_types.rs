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


/// Cryptographic Types
///
/// This module contains all types related to cryptographic operations,
/// key management, entropy handling, and security-related configurations.

use serde::{Deserialize, Serialize};
use beardog_errors::{BearDogError, BearDogResult};
/// Key lifecycle status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyStatus {
    /// Key is active and can be used
    Active,
    /// Key is pending activation
    Pending,
    /// Key has been revoked
    Revoked,
    /// Key has expired
    Expired,
}
// Fix the derive macro issue by defining the struct properly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyAdjustmentConfig {
    pub enabled: bool,
    pub adjustment_factor: f64,
    pub minimum_entropy: u32,
    pub maximum_entropy: u32,
/// Entropy-based expiry configuration
pub struct EntropyBasedExpiry {
    /// Enable entropy-based expiry
    /// Base expiry time in seconds
    pub base_expiry_seconds: u64,
    /// Entropy quality threshold for expiry adjustment
    pub quality_threshold: f64,
/// Configuration for genetic renewal of keys
pub struct GeneticRenewalConfig {
    /// Enable genetic renewal
    /// Generation limit before forced renewal
    pub max_generations: u32,
/// Key expiry policy configuration
pub enum KeyExpiryPolicy {
    /// Fixed time expiry
    Fixed {
        /// Duration in seconds
        duration_seconds: u64,
    },
    /// Entropy-based dynamic expiry
    EntropyBased {
        /// Entropy configuration
        config: EntropyBasedExpiry,
    /// Genetic algorithm-based renewal
    Genetic {
        /// Genetic renewal configuration
        config: GeneticRenewalConfig,
    /// Never expires (use with caution)
    Never,
    /// Usage-based expiry
    UsageBased {
        /// Maximum number of uses
        max_uses: u32,
        /// Time window in seconds
        time_window_seconds: Option<u64>,
/// Current status of key expiry
pub enum KeyExpiryStatus {
    /// Key is valid and not expired
    Valid {
        /// Time until expiry
        expires_in_seconds: u64,
    /// Key is expired
    Expired {
        /// Time since expiry
        expired_seconds_ago: u64,
    /// Key is expiring soon
    ExpiringSoon {
        /// Warning threshold that was exceeded
        warning_threshold_seconds: u64,
    /// Key has unlimited usage
    Unlimited,
    /// Usage-based expiry status
        /// Remaining uses
        remaining_uses: Option<u32>,
/// Context-aware key configuration}


pub struct ContextAwareKeyConfig {
    /// Key context identifier
    pub context: String,
    /// Purpose of the key
    pub purpose: String,
    /// Security tier requirement
    pub security_tier: u8,
    /// Expiry policy
    pub expiry_policy: KeyExpiryPolicy,
/// Rate limiting configuration for cryptographic operations - USE CANONICAL VERSION
// Re-export from canonical security configuration
pub use beardog_types::canonical::configuration::security::RateLimitConfig;
// Default implementations


impl Default for EntropyAdjustmentConfig {}


    fn default() -> Self {
        Self {
            enabled: true,
            adjustment_factor: 1.0,
            minimum_entropy: 128,
            maximum_entropy: 256,
        }
    }
impl Default for EntropyBasedExpiry {
            enabled: false,
            base_expiry_seconds: 86400, // 24 hours
            quality_threshold: 0.9,}


impl Default for GeneticRenewalConfig {
            max_generations: 10,
impl Default for KeyExpiryPolicy {
        Self::Fixed {
            duration_seconds: 86400, // 24 hours}


impl Default for KeyExpiryStatus {
        Self::UsageBased {
            remaining_uses: None,
impl Default for ContextAwareKeyConfig {
            context: "default".to_string(),
            purpose: "general".to_string(),
            security_tier: 1,
            expiry_policy: KeyExpiryPolicy::default(),}


impl Default for RateLimitConfig {
            max_requests_per_minute: 60,
            burst_capacity: 10,
