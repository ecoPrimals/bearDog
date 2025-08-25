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


/// Type definitions and data structures for proof verification
///
/// Contains all structs, enums, and type aliases for verification operations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Import auth types that verification depends on
use crate::auth::{BearDogGenetics, ResourcePermission};
/// Configuration for proof verification operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationConfig {
    /// Whether to enable proof caching
    pub enable_proof_caching: bool,
    /// Maximum cache size for verification results
    pub max_cache_size: usize,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    /// Whether to perform deep verification
    pub deep_verification: bool,
    /// Cryptographic strength requirements
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
/// Verification result for authorization proofs
pub struct VerificationResult {
    /// Whether the proof is valid
    pub valid: bool,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Details about the verification
    pub details: String,
    /// Timestamp of verification
    pub verified_at: DateTime<Utc>,
    /// Verification method used
    pub method: VerificationMethod,
/// Methods used for verification
pub enum VerificationMethod {
    /// Cryptographic signature verification
    CryptographicSignature,
    /// Genetic consistency check
    GeneticConsistency,
    /// Timestamp verification
    TimestampVerification,
    /// Cross-reference validation
    CrossReference,
    /// Combined verification
    Combined(Vec<VerificationMethod>),
/// Cached verification result
#[derive(Debug, Clone)]}


pub struct CachedVerification {
    /// The verification result
    pub result: VerificationResult,
    /// When this was cached
    pub cached_at: DateTime<Utc>,
    /// Cache expiry time
    pub expires_at: DateTime<Utc>,
/// Proof verification context
pub struct VerificationContext {
    /// Node genetics for genetic verification
    pub node_genetics: Option<BearDogGenetics>,
    /// Required permission level
    pub required_permission: Option<ResourcePermission>,
    /// Additional context data
    pub context_data: HashMap<String, String>,
/// Verification statistics
pub struct VerificationStats {
    /// Total verifications performed
    pub total_verifications: u64,
    /// Successful verifications
    pub successful_verifications: u64,
    /// Failed verifications
    pub failed_verifications: u64,
    /// Cache hits
    pub cache_hits: u64,
    /// Cache misses
    pub cache_misses: u64,
    /// Average verification time in milliseconds
    pub avg_verification_time_ms: f64,
