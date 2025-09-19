

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use serde::{Deserialize, Serialize};
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
    /// The adjustment factor value
    pub adjustment_factor: f64,
    /// Number of minimum_entropy
    pub minimum_entropy: u32,
    /// Number of maximum_entropy
    pub maximum_entropy: u32,

pub struct EntropyBasedExpiry {

    /// Number of base_expiry_seconds
    pub base_expiry_seconds: u64,

    /// The quality threshold value
    pub quality_threshold: f64,

pub struct GeneticRenewalConfig {

    /// Number of max_generations
    pub max_generations: u32,

pub enum KeyExpiryPolicy {

    /// State indicating fixed
    Fixed {

        duration_seconds: u64,
    },

    EntropyBased {

        config: EntropyBasedExpiry,

    Genetic {

        config: GeneticRenewalConfig,


    Never,

    UsageBased {

        max_uses: u32,

        time_window_seconds: Option<u64>,

pub enum KeyExpiryStatus {

    /// Represents valid variant
    Valid {

        expires_in_seconds: u64,

    /// State indicating expired
    Expired {

        expired_seconds_ago: u64,

    /// Represents expiring soon variant
    ExpiringSoon {

        warning_threshold_seconds: u64,


    /// State indicating unlimited
    Unlimited,

        remaining_uses: Option<u32>,

pub struct ContextAwareKeyConfig {

    /// The context value
    pub context: String,

    /// The purpose value
    pub purpose: String,

    /// Number of security_tier
    pub security_tier: u8,

    /// The expiry policy value
    pub expiry_policy: KeyExpiryPolicy,

pub use beardog_types::canonical::configuration::security::RateLimitConfig;

impl Default for EntropyAdjustmentConfig {}
impl Default for EntropyAdjustmentConfig {}
impl Default for EntropyAdjustmentConfig {}

    fn default(true,
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
            expiry_policy: KeyExpiryPolicy::default(60,
            burst_capacity: 10,
