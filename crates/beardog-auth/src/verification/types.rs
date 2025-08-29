use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::{BearDogGenetics, ResourcePermission};

// VerificationConfig removed - use UnifiedAuthConfig instead

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub valid: bool,
    pub confidence: f64,
    pub details: String,
    pub verified_at: DateTime<Utc>,
    pub method: VerificationMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationMethod {
    CryptographicSignature,

    GeneticConsistency,

    TimestampVerification,

    CrossReference,
    Combined(Vec<VerificationMethod>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedVerification {
    pub result: VerificationResult,
    pub cached_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationContext {
    pub node_genetics: Option<BearDogGenetics>,
    pub required_permission: Option<ResourcePermission>,
    pub context_data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationStats {
    pub total_verifications: u64,
    pub successful_verifications: u64,
    pub failed_verifications: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub avg_verification_time_ms: f64,
}
