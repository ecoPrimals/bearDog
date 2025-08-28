

use serde::{Deserialize, Serialize};
use beardog_errors::BearDogError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyStatus {

    Active,

    Pending,

    Revoked,

    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyAdjustmentConfig {
    pub enabled: bool,
    pub adjustment_factor: f64,
    pub minimum_entropy: u32,
    pub maximum_entropy: u32,

pub struct EntropyBasedExpiry {

    pub base_expiry_seconds: u64,

    pub quality_threshold: f64,

pub struct GeneticRenewalConfig {

    pub max_generations: u32,

pub enum KeyExpiryPolicy {

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

    Valid {

        expires_in_seconds: u64,

    Expired {

        expired_seconds_ago: u64,

    ExpiringSoon {

        warning_threshold_seconds: u64,

    Unlimited,

        remaining_uses: Option<u32>,

pub struct ContextAwareKeyConfig {

    pub context: String,

    pub purpose: String,

    pub security_tier: u8,

    pub expiry_policy: KeyExpiryPolicy,

pub use beardog_types::canonical::configuration::security::RateLimitConfig;

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
