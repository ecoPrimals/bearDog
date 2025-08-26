

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::{BearDogGenetics, ResourcePermission};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(since = "3.1.0", note = "Use UnifiedAuthConfig instead")]
#[deprecated(since = "3.1.0", note = "Use UnifiedAuthConfig instead")]
pub struct VerificationConfig {

    pub enable_proof_caching: bool,

    pub max_cache_size: usize,

    pub cache_ttl_seconds: u64,

    pub deep_verification: bool,

    pub min_crypto_strength: f64,
}
impl Default for VerificationConfig {}

    fn default() -> Self {
        Self {
            enable_proof_caching: true,
            max_cache_size: 10000,
            cache_ttl_seconds: 3600, // 1 hour
            deep_verification: true,
            min_crypto_strength: 0.8,
        }
    }

pub struct VerificationResult {

    pub valid: bool,

    pub confidence: f64,

    pub details: String,

    pub verified_at: DateTime<Utc>,

    pub method: VerificationMethod,

pub enum VerificationMethod {

    CryptographicSignature,

    GeneticConsistency,

    TimestampVerification,

    CrossReference,

    Combined(Vec<VerificationMethod>),

#[derive(Debug, Clone)]}

pub struct CachedVerification {

    pub result: VerificationResult,

    pub cached_at: DateTime<Utc>,

    pub expires_at: DateTime<Utc>,

pub struct VerificationContext {

    pub node_genetics: Option<BearDogGenetics>,

    pub required_permission: Option<ResourcePermission>,

    pub context_data: HashMap<String, String>,

pub struct VerificationStats {

    pub total_verifications: u64,

    pub successful_verifications: u64,

    pub failed_verifications: u64,

    pub cache_hits: u64,

    pub cache_misses: u64,

    pub avg_verification_time_ms: f64,
