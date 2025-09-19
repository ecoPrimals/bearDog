// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::{BearDogGenetics, ResourcePermission};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Whether success is enabled
    pub success: bool,
    pub confidence: f64,
    /// The details value
    pub details: String,
    /// The verified at value
    pub verified_at: DateTime<Utc>,
    /// The method value
    pub method: VerificationMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedVerification {
    /// The result value
    pub result: VerificationResult,
    /// The cached at value
    pub cached_at: DateTime<Utc>,
    /// The expires at value
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationContext {
    /// Optional genetics
    pub genetics: Option<BearDogGenetics>,
    /// Optional required permission
    pub required_permission: Option<ResourcePermission>,
    /// Mapping of context data
    pub context_data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMetrics {
    /// Number of total_verifications
    pub total_verifications: u64,
    /// Number of successful_verifications
    pub successful_verifications: u64,
    /// Number of failed_verifications
    pub failed_verifications: u64,
    /// Number of cache_hits
    pub cache_hits: u64,
    /// Number of cache_misses
    pub cache_misses: u64,
    pub avg_verification_time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationMethod {
    /// Represents cryptographic variant
    Cryptographic,
    /// Represents biometric variant
    Biometric,
    /// Represents genetic variant
    Genetic,
    /// Represents behavioral variant
    Behavioral,
    /// Represents multi factor auth variant
    MultiFactorAuth,
    /// Represents zero knowledge proof variant
    ZeroKnowledgeProof,
}
