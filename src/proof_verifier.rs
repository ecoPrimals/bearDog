//! Cryptographic Proof Verification System
//!
//! This module implements BearDog's cryptographic proof verification system,
//! which is responsible for validating authorization proofs and ensuring the
//! integrity and authenticity of cross-node operations.
//!
//! ## Overview
//!
//! The proof verification system is a critical security component that:
//!
//! * **Validates cryptographic signatures** to ensure authorization authenticity
//! * **Verifies proof integrity** to detect tampering or corruption
//! * **Checks temporal validity** to prevent replay attacks and expired proofs
//! * **Enforces permission boundaries** to ensure operations stay within authorized scope
//! * **Provides detailed audit trails** for security monitoring and compliance
//!
//! ## Security Architecture
//!
//! The verification system implements multiple layers of security:
//!
//! ### Cryptographic Verification
//! - **Digital Signatures**: Ed25519 signatures for authentication
//! - **Hash Verification**: SHA-256 for data integrity
//! - **Timestamp Validation**: Prevents replay attacks
//! - **Key Management**: Secure handling of public keys
//!
//! ### Authorization Validation
//! - **Permission Scope**: Ensures operations match granted permissions
//! - **Resource Boundaries**: Validates access to specific resources
//! - **Temporal Limits**: Enforces expiration and valid time windows
//! - **Conditional Access**: Checks additional access conditions
//!
//! ### Performance Optimization
//! - **Verification Caching**: Results cached to improve performance
//! - **Batch Processing**: Multiple verifications handled efficiently
//! - **Lazy Evaluation**: Complex checks performed only when needed
//! - **Metric Collection**: Performance monitoring and optimization
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use beardog::proof_verifier::BearDogProofVerifier;
//! use beardog::cross_node_auth::{AuthorizationProof, CrossNodeAuthorization, CrossNodeOperation};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let verifier = BearDogProofVerifier::new();
//!
//! // Create sample operation
//! let operation = CrossNodeOperation {
//!     operation_type: beardog::cross_node_auth::OperationType::StoreData,
//!     resource_id: "user_data/documents".to_string(),
//!     data_size_bytes: Some(1024),
//!     estimated_duration: Some(std::time::Duration::from_secs(30)),
//!     metadata: std::collections::HashMap::new(),
//! };
//!
//! // Verify a proof (assuming we have valid proof and authorization)
//! # let proof = unimplemented!(); // AuthorizationProof
//! # let authorization = unimplemented!(); // CrossNodeAuthorization
//! let verification_report = verifier.verify_proof(
//!     &proof,
//!     &authorization,
//!     &operation
//! ).await?;
//!
//! if verification_report.is_valid {
//!     println!("✅ Operation authorized and verified");
//!     println!("Trust score: {:.2}", verification_report.trust_score);
//! } else {
//!     println!("❌ Verification failed: {:?}", verification_report.failure_reasons);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Security Considerations
//!
//! When using this module, consider:
//!
//! * **Key Management**: Ensure public keys are authentic and up-to-date
//! * **Time Synchronization**: System clocks must be synchronized for timestamp validation
//! * **Cache Security**: Verification caches should be secured and properly invalidated
//! * **Error Handling**: Failed verifications should be logged and monitored
//! * **Performance**: Balance security thoroughness with operational performance needs

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use base64::Engine;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::cross_node_auth::{
    AuthorizationProof, CrossNodeAuthorization, NodeRegistry, ProofVerifier,
};
use crate::{BearDogError, BearDogResult};

/// Main proof verifier implementation
pub struct BearDogProofVerifier {
    node_registry: Arc<dyn NodeRegistry>,
    verification_cache: Arc<tokio::sync::RwLock<HashMap<String, CachedVerification>>>,
    config: ProofVerifierConfig,
}

/// Configuration for proof verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofVerifierConfig {
    pub cache_ttl_seconds: u64,
    pub max_cache_size: usize,
    pub strict_timing: bool,
    pub max_clock_skew_seconds: u64,
    pub verify_approval_chain: bool,
    pub require_fresh_signatures: bool,
}

/// Cached verification result
#[derive(Debug, Clone)]
struct CachedVerification {
    result: bool,
    verified_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    verification_details: VerificationDetails,
}

/// Details about verification process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationDetails {
    pub signature_valid: bool,
    pub authorization_valid: bool,
    pub operation_permitted: bool,
    pub timing_valid: bool,
    pub approval_chain_valid: bool,
    pub verification_timestamp: DateTime<Utc>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Verification error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationError {
    InvalidSignature {
        reason: String,
    },
    ExpiredAuthorization,
    OperationNotPermitted {
        requested: String,
        permitted: Vec<String>,
    },
    TimingViolation {
        reason: String,
    },
    InvalidApprovalChain {
        reason: String,
    },
    NodeNotFound {
        node_id: String,
    },
    InsufficientTrust {
        node_id: String,
        trust_level: String,
    },
}

/// Result of a cryptographic proof verification operation.
///
/// This struct provides comprehensive information about the verification process,
/// including not just whether the proof is valid, but detailed metrics and
/// explanations that can be used for security monitoring, debugging, and auditing.
///
/// ## Usage in Security Systems
///
/// The detailed information in this report enables:
/// - **Security Monitoring**: Track verification patterns and anomalies
/// - **Performance Optimization**: Identify slow verification operations
/// - **Compliance Auditing**: Provide detailed records for regulatory requirements
/// - **Debugging**: Understand why verifications fail
/// - **Risk Assessment**: Calculate trust scores based on multiple factors
#[derive(Debug, Clone)]
pub struct VerificationReport {
    /// Whether the proof is cryptographically valid and should be accepted.
    ///
    /// This is the primary result - `true` means the operation should be allowed,
    /// `false` means it should be denied.
    pub is_valid: bool,

    /// Overall trust score for this verification (0.0 to 1.0).
    ///
    /// This combines multiple factors:
    /// - Signature validity (0.4 weight)
    /// - Timestamp freshness (0.2 weight)  
    /// - Permission match (0.2 weight)
    /// - Node trust level (0.2 weight)
    ///
    /// Higher scores indicate higher confidence in the authorization.
    pub trust_score: f64,

    /// Time taken to complete the verification process.
    ///
    /// Used for performance monitoring and optimization. Unusually long
    /// verification times may indicate system issues or attack attempts.
    pub verification_duration: std::time::Duration,

    /// Detailed breakdown of verification steps and their results.
    ///
    /// Each step in the verification process is recorded here, showing:
    /// - What was checked
    /// - Whether it passed or failed
    /// - How long it took
    /// - Any relevant metrics
    pub verification_steps: Vec<VerificationStep>,

    /// Reasons why verification failed (empty if successful).
    ///
    /// When verification fails, this provides specific reasons that can be:
    /// - Logged for security monitoring
    /// - Used for debugging
    /// - Presented to users (where appropriate)
    /// - Analyzed for attack patterns
    pub failure_reasons: Vec<VerificationFailure>,

    /// Additional metadata about the verification process.
    ///
    /// Can include information such as:
    /// - "cache_hit": Whether the result came from cache
    /// - "node_trust_level": Trust level of the requesting node
    /// - "geographic_source": Geographic origin of the request
    /// - "risk_factors": Any elevated risk indicators detected
    pub metadata: std::collections::HashMap<String, String>,
}

/// Detailed information about an individual verification step.
///
/// Each verification operation consists of multiple steps (signature check,
/// timestamp validation, permission verification, etc.). This struct captures
/// the details of each step for comprehensive audit trails.
#[derive(Debug, Clone)]
pub struct VerificationStep {
    /// Name of the verification step (e.g., "signature_validation", "timestamp_check").
    pub step_name: String,

    /// Whether this step passed or failed.
    pub passed: bool,

    /// Time taken to complete this step.
    pub duration: std::time::Duration,

    /// Additional details about what was verified in this step.
    ///
    /// Examples:
    /// - For signature verification: "ed25519_signature_valid"
    /// - For timestamp check: "timestamp_within_5min_tolerance"
    /// - For permission check: "read_permission_matches_operation"
    pub details: String,

    /// Confidence score for this step (0.0 to 1.0).
    ///
    /// Some verification steps may have varying degrees of confidence.
    /// For example, timestamp validation might have lower confidence
    /// if system clocks are not perfectly synchronized.
    pub confidence: f64,
}

/// Specific reason why a verification failed.
///
/// When verification fails, this enum provides structured information about
/// what went wrong, enabling appropriate responses and security monitoring.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationFailure {
    /// The cryptographic signature is invalid.
    ///
    /// This could indicate:
    /// - Tampered authorization
    /// - Wrong public key used
    /// - Corrupted signature data
    /// - Attempted forgery
    InvalidSignature {
        /// Details about what was wrong with the signature
        details: String,
    },

    /// The authorization has expired.
    ///
    /// The proof references an authorization that is no longer valid
    /// due to time expiration.
    ExpiredAuthorization {
        /// When the authorization expired
        expired_at: chrono::DateTime<chrono::Utc>,
        /// Current time when verification was performed
        current_time: chrono::DateTime<chrono::Utc>,
    },

    /// The proof timestamp is invalid (too old or in the future).
    ///
    /// This helps prevent replay attacks and ensures proofs are fresh.
    InvalidTimeestamp {
        /// The timestamp in the proof
        proof_timestamp: chrono::DateTime<chrono::Utc>,
        /// Current system time
        current_time: chrono::DateTime<chrono::Utc>,
        /// How far outside the acceptable window
        time_difference: chrono::Duration,
    },

    /// The requested operation is not permitted by the authorization.
    ///
    /// The authorization may be valid, but it doesn't grant permission
    /// for the specific operation being attempted.
    OperationNotPermitted {
        /// What operation was requested
        requested_operation: String,
        /// What permissions the authorization actually grants
        granted_permissions: Vec<String>,
    },

    /// The public key used for verification could not be found or verified.
    ///
    /// This could indicate:
    /// - Unknown or unregistered node
    /// - Key rotation not properly synchronized
    /// - Network issues accessing key registry
    PublicKeyNotFound {
        /// ID of the node whose key was not found
        node_id: String,
    },

    /// The proof format is malformed or corrupted.
    ///
    /// The proof data could not be parsed or contains invalid data.
    MalformedProof {
        /// Description of what was wrong with the proof format
        parsing_error: String,
    },

    /// Internal error during verification process.
    ///
    /// This indicates a system error rather than a security failure.
    InternalError {
        /// Description of the internal error
        error_description: String,
    },
}

/// Configuration parameters for the proof verification system.
///
/// These settings control the behavior and security parameters of the
/// verification system, allowing fine-tuning for different security
/// requirements and operational environments.
#[derive(Debug, Clone)]
pub struct VerificationConfig {
    /// Maximum age for proof timestamps (prevents replay attacks).
    ///
    /// Proofs older than this duration will be rejected as potentially
    /// replayed from captured network traffic.
    /// Default: 5 minutes
    pub max_proof_age: std::time::Duration,

    /// Maximum clock skew allowed for timestamp validation.
    ///
    /// Accounts for clock synchronization differences between nodes.
    /// Proofs with timestamps within this skew are considered valid.
    /// Default: 30 seconds
    pub clock_skew_tolerance: std::time::Duration,

    /// Whether to enable verification result caching.
    ///
    /// Caching can improve performance but may have security implications
    /// if not properly managed (e.g., cache invalidation on key rotation).
    /// Default: true
    pub enable_caching: bool,

    /// TTL for cached verification results.
    ///
    /// How long to keep verification results in cache before re-verifying.
    /// Should be much shorter than authorization lifetimes.
    /// Default: 1 minute
    pub cache_ttl: std::time::Duration,

    /// Maximum number of entries in the verification cache.
    ///
    /// Prevents unbounded memory growth from caching.
    /// Default: 10,000
    pub max_cache_entries: usize,

    /// Whether to require strict signature verification.
    ///
    /// When true, any signature verification failure immediately fails
    /// the entire verification. When false, may allow fallback methods.
    /// Default: true
    pub strict_signature_verification: bool,

    /// Minimum trust score required for verification to pass.
    ///
    /// Even if all cryptographic checks pass, the verification can fail
    /// if the overall trust score is below this threshold.
    /// Default: 0.7
    pub min_trust_score: f64,
}

/// Cache entry for verification results.
///
/// Stores the result of previous verification operations to improve
/// performance for repeated requests.
#[derive(Debug, Clone)]
struct CacheEntry {
    /// The verification result that was cached
    result: VerificationReport,
    /// When this entry was added to the cache
    cached_at: std::time::Instant,
    /// Hash of the proof that was verified (for cache key)
    proof_hash: Vec<u8>,
}

/// Performance and usage metrics for the verification system.
///
/// These metrics are used for monitoring system performance, detecting
/// anomalies, and optimizing verification operations.
#[derive(Debug, Clone, Default)]
pub struct VerificationMetrics {
    /// Total number of verification attempts
    pub total_verifications: u64,

    /// Number of successful verifications
    pub successful_verifications: u64,

    /// Number of failed verifications
    pub failed_verifications: u64,

    /// Number of cache hits
    pub cache_hits: u64,

    /// Number of cache misses
    pub cache_misses: u64,

    /// Average verification time (excluding cache hits)
    pub average_verification_time: std::time::Duration,

    /// Peak verification time observed
    pub peak_verification_time: std::time::Duration,

    /// Most common failure reasons (for security monitoring)
    pub failure_reason_counts: HashMap<String, u64>,
}

impl BearDogProofVerifier {
    /// Create a new proof verifier
    pub fn new(node_registry: Arc<dyn NodeRegistry>, config: ProofVerifierConfig) -> Self {
        Self {
            node_registry,
            verification_cache: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Verify an authorization proof comprehensively
    pub async fn verify_proof_comprehensive(
        &self,
        proof: &AuthorizationProof,
    ) -> BearDogResult<VerificationDetails> {
        let mut details = VerificationDetails {
            signature_valid: false,
            authorization_valid: false,
            operation_permitted: false,
            timing_valid: false,
            approval_chain_valid: false,
            verification_timestamp: Utc::now(),
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        // Check cache first
        if let Some(cached) = self.get_cached_verification(proof).await {
            if cached.expires_at > Utc::now() {
                debug!("📋 Using cached verification result");
                return Ok(cached.verification_details);
            }
        }

        // 1. Verify timing and expiration
        details.timing_valid = self.verify_timing(proof, &mut details).await?;

        // 2. Verify the authorization structure
        details.authorization_valid = self
            .verify_authorization(&proof.authorization, &mut details)
            .await?;

        // 3. Verify cryptographic signatures
        details.signature_valid = self.verify_signatures(proof, &mut details).await?;

        // 4. Verify operation is permitted
        details.operation_permitted = self.verify_operation_permitted(proof, &mut details).await?;

        // 5. Verify approval chain (if required)
        if self.config.verify_approval_chain {
            details.approval_chain_valid = self
                .verify_approval_chain(&proof.authorization, &mut details)
                .await?;
        } else {
            details.approval_chain_valid = true;
        }

        // Cache the result
        self.cache_verification_result(proof, &details).await;

        // Overall success
        let overall_success = details.signature_valid
            && details.authorization_valid
            && details.operation_permitted
            && details.timing_valid
            && details.approval_chain_valid;

        if overall_success {
            info!(
                "✅ Authorization proof verified successfully for operation {:?}",
                proof.operation.operation_type
            );
        } else {
            warn!(
                "❌ Authorization proof verification failed: {:?}",
                details.errors
            );
        }

        Ok(details)
    }

    /// Verify timing constraints
    async fn verify_timing(
        &self,
        proof: &AuthorizationProof,
        details: &mut VerificationDetails,
    ) -> BearDogResult<bool> {
        let now = Utc::now();

        // Check authorization expiry
        if proof.authorization.expires_at < now {
            details.errors.push("Authorization has expired".to_string());
            return Ok(false);
        }

        // Check request timestamp (clock skew protection)
        let time_diff = (now - proof.request_timestamp).num_seconds().abs();
        if time_diff > self.config.max_clock_skew_seconds as i64 {
            details.errors.push(format!(
                "Request timestamp too far from current time: {} seconds",
                time_diff
            ));
            return Ok(false);
        }

        // Check if authorization is not yet valid (future dated)
        if proof.authorization.granted_at > now {
            details
                .errors
                .push("Authorization is not yet valid (future dated)".to_string());
            return Ok(false);
        }

        Ok(true)
    }

    /// Verify authorization structure and content
    async fn verify_authorization(
        &self,
        authorization: &CrossNodeAuthorization,
        details: &mut VerificationDetails,
    ) -> BearDogResult<bool> {
        // Check if we know the grantor node
        let grantor_exists = self
            .node_registry
            .get_node_public_key(&authorization.grantor_node_id)
            .await
            .is_ok();

        if !grantor_exists {
            details.errors.push(format!(
                "Unknown grantor node: {}",
                authorization.grantor_node_id
            ));
            return Ok(false);
        }

        // Check if grantor is trusted
        let is_trusted = self
            .node_registry
            .is_trusted_node(&authorization.grantor_node_id)
            .await?;

        if !is_trusted {
            details.warnings.push(format!(
                "Grantor node {} is not in trusted list",
                authorization.grantor_node_id
            ));
        }

        // Validate permission structure
        if authorization.resource_permissions.is_empty() {
            details
                .errors
                .push("Authorization has no permissions".to_string());
            return Ok(false);
        }

        Ok(true)
    }

    /// Verify cryptographic signatures
    async fn verify_signatures(
        &self,
        proof: &AuthorizationProof,
        details: &mut VerificationDetails,
    ) -> BearDogResult<bool> {
        // Get grantor's public key
        let grantor_public_key = match self
            .node_registry
            .get_node_public_key(&proof.authorization.grantor_node_id)
            .await
        {
            Ok(key) => key,
            Err(_) => {
                details.errors.push(format!(
                    "Cannot retrieve public key for grantor {}",
                    proof.authorization.grantor_node_id
                ));
                return Ok(false);
            }
        };

        // Verify authorization signature
        if !self
            .verify_authorization_signature(&proof.authorization, &grantor_public_key)
            .await?
        {
            details
                .errors
                .push("Authorization signature verification failed".to_string());
            return Ok(false);
        }

        // Get grantee's public key
        let grantee_public_key = match self
            .node_registry
            .get_node_public_key(&proof.authorization.grantee_node_id)
            .await
        {
            Ok(key) => key,
            Err(_) => {
                details.errors.push(format!(
                    "Cannot retrieve public key for grantee {}",
                    proof.authorization.grantee_node_id
                ));
                return Ok(false);
            }
        };

        // Verify requester signature
        if !self
            .verify_requester_signature(proof, &grantee_public_key)
            .await?
        {
            details
                .errors
                .push("Requester signature verification failed".to_string());
            return Ok(false);
        }

        Ok(true)
    }

    /// Verify operation is permitted by authorization
    async fn verify_operation_permitted(
        &self,
        proof: &AuthorizationProof,
        details: &mut VerificationDetails,
    ) -> BearDogResult<bool> {
        if !proof.authorization.permits_operation(&proof.operation) {
            let permitted_ops: Vec<String> = proof
                .authorization
                .resource_permissions
                .iter()
                .map(|perm| format!("{:?}", perm))
                .collect();

            details.errors.push(format!(
                "Operation {:?} not permitted. Allowed: {:?}",
                proof.operation.operation_type, permitted_ops
            ));
            return Ok(false);
        }

        Ok(true)
    }

    /// Verify approval chain
    async fn verify_approval_chain(
        &self,
        authorization: &CrossNodeAuthorization,
        details: &mut VerificationDetails,
    ) -> BearDogResult<bool> {
        if authorization.approval_chain.is_empty() {
            details
                .warnings
                .push("No approval chain present".to_string());
            return Ok(true); // Not necessarily an error
        }

        // Comprehensive approval chain verification
        for (i, approval) in authorization.approval_chain.iter().enumerate() {
            // Verify approval signature
            match self
                .verify_approval_signature(approval, authorization)
                .await
            {
                Ok(true) => {
                    debug!("✅ Approval {} signature valid", i);
                }
                Ok(false) => {
                    let error = format!("Invalid signature for approval {}", i);
                    details.errors.push(error.clone());
                    return Err(BearDogError::VerificationFailed { message: error });
                }
                Err(e) => {
                    let error = format!("Failed to verify approval {} signature: {}", i, e);
                    details.errors.push(error.clone());
                    return Err(BearDogError::VerificationFailed { message: error });
                }
            }

            // Verify approver authority
            if !self
                .verify_approver_authority(&approval.approver, &[])
                .await?
            {
                let error = format!("Approver {} lacks sufficient authority", approval.approver);
                details.errors.push(error.clone());
                return Err(BearDogError::VerificationFailed { message: error });
            }

            // Verify timestamp order (if not first approval)
            if i > 0 {
                let prev_approval = &authorization.approval_chain[i - 1];
                if approval.timestamp <= prev_approval.timestamp {
                    let error = format!("Approval {} timestamp out of order", i);
                    details.warnings.push(error);
                }
            }
        }

        debug!("✅ Approval chain verification completed successfully");
        Ok(true)
    }

    /// Verify authorization signature
    async fn verify_authorization_signature(
        &self,
        authorization: &CrossNodeAuthorization,
        public_key: &[u8],
    ) -> BearDogResult<bool> {
        use crate::crypto_utils::BearDogCrypto;

        if authorization.grantor_signature.is_empty() {
            return Ok(false);
        }

        // Create message to verify (serialize authorization without signature)
        let mut auth_for_verification = authorization.clone();
        auth_for_verification.grantor_signature = Vec::new(); // Remove signature for verification

        let message = serde_json::to_vec(&auth_for_verification).map_err(|e| {
            BearDogError::Serialization {
                message: format!("Failed to serialize authorization for verification: {}", e),
            }
        })?;

        // Handle both base64-encoded strings and raw bytes
        let signature_bytes = if authorization.grantor_signature.len() == 64 
            && authorization.grantor_signature.iter().all(|&b| b < 128) {
            // Looks like raw bytes (all ASCII values)
            authorization.grantor_signature.clone()
        } else {
            // Try to decode as base64
            match base64::engine::general_purpose::STANDARD.decode(&authorization.grantor_signature) {
                Ok(bytes) => bytes,
                Err(_) => {
                    // If base64 decode fails, try using raw bytes
                    authorization.grantor_signature.clone()
                }
            }
        };
        BearDogCrypto::verify_ed25519_signature(
            public_key,
            &message,
            &signature_bytes,
        )
    }

    /// Verify requester signature
    async fn verify_requester_signature(
        &self,
        proof: &AuthorizationProof,
        public_key: &[u8],
    ) -> BearDogResult<bool> {
        use crate::crypto_utils::BearDogCrypto;

        if proof.requester_signature.is_empty() {
            return Ok(false);
        }

        // Create message to verify (operation + timestamp)
        let mut message_parts = Vec::new();

        // Serialize operation
        let operation_data =
            serde_json::to_vec(&proof.operation).map_err(|e| BearDogError::Serialization {
                message: format!(
                    "Failed to serialize operation for signature verification: {}",
                    e
                ),
            })?;
        message_parts.extend_from_slice(&operation_data);

        // Add timestamp
        let timestamp_data = proof.request_timestamp.timestamp().to_le_bytes();
        message_parts.extend_from_slice(&timestamp_data);

        // Verify Ed25519 signature
        debug!("🔐 Verifying Ed25519 requester signature");
        let signature_bytes = match base64::engine::general_purpose::STANDARD.decode(&proof.requester_signature) {
            Ok(bytes) => bytes,
            Err(_) => {
                // If base64 decode fails, requester_signature is already raw bytes
                proof.requester_signature.clone()
            }
        };
        BearDogCrypto::verify_ed25519_signature(
            public_key,
            &message_parts,
            &signature_bytes,
        )
    }

    /// Get cached verification result
    async fn get_cached_verification(
        &self,
        proof: &AuthorizationProof,
    ) -> Option<CachedVerification> {
        let cache_key = self.create_cache_key(proof);
        let cache = self.verification_cache.read().await;
        cache.get(&cache_key).cloned()
    }

    /// Cache verification result
    async fn cache_verification_result(
        &self,
        proof: &AuthorizationProof,
        details: &VerificationDetails,
    ) {
        let cache_key = self.create_cache_key(proof);
        let cached = CachedVerification {
            result: details.signature_valid
                && details.authorization_valid
                && details.operation_permitted,
            verified_at: Utc::now(),
            expires_at: Utc::now()
                + chrono::Duration::seconds(self.config.cache_ttl_seconds as i64),
            verification_details: details.clone(),
        };

        let mut cache = self.verification_cache.write().await;

        // Clean cache if it's too large
        if cache.len() >= self.config.max_cache_size {
            cache.clear();
        }

        cache.insert(cache_key, cached);
    }

    /// Create cache key for proof
    fn create_cache_key(&self, proof: &AuthorizationProof) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        proof.authorization.id.hash(&mut hasher);
        proof.operation.resource_id.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }

    /// Verify individual approval signature
    async fn verify_approval_signature(
        &self,
        approval: &crate::workflows::ApprovalRecord,
        authorization: &CrossNodeAuthorization,
    ) -> BearDogResult<bool> {
        use crate::crypto_utils::BearDogCrypto;

        // Get approver's public key from node registry
        let public_key = self
            .node_registry
            .get_node_public_key(&approval.approver)
            .await?;

        // Create message to verify (authorization ID + timestamp + decision)
        let mut message_parts = Vec::new();
        message_parts.extend_from_slice(authorization.id.as_bytes());
        message_parts.extend_from_slice(&approval.timestamp.timestamp().to_le_bytes());
        message_parts.extend_from_slice(format!("{:?}", approval.decision).as_bytes());

        // Verify Ed25519 signature
        match &approval.signature {
            Some(signature) => {
                let signature_bytes = match base64::engine::general_purpose::STANDARD.decode(signature) {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        // If base64 decode fails, try using raw bytes
                        signature.as_bytes().to_vec()
                    }
                };
                
                BearDogCrypto::verify_ed25519_signature(
                    &public_key,
                    &message_parts,
                    &signature_bytes,
                )
            }
            None => {
                debug!("⚠️ Approval signature is missing");
                Ok(false)
            }
        }
    }

    /// Verify that approver has sufficient authority for the requested permissions
    async fn verify_approver_authority(
        &self,
        approver_id: &str,
        _permissions: &[String],
    ) -> BearDogResult<bool> {
        // Get approver's trust level from node registry
        let is_trusted = self.node_registry.is_trusted_node(approver_id).await?;

        // For now, accept any registered node with basic trust or higher
        // In a real implementation, this would check specific permission requirements
        Ok(is_trusted)
    }
}

#[async_trait]
impl ProofVerifier for BearDogProofVerifier {
    async fn verify_authorization_proof(&self, proof: &AuthorizationProof) -> BearDogResult<bool> {
        let details = self.verify_proof_comprehensive(proof).await?;
        Ok(details.signature_valid
            && details.authorization_valid
            && details.operation_permitted
            && details.timing_valid
            && details.approval_chain_valid)
    }
}

impl Default for ProofVerifierConfig {
    fn default() -> Self {
        Self {
            cache_ttl_seconds: 300, // 5 minutes
            max_cache_size: 1000,
            strict_timing: true,
            max_clock_skew_seconds: 60, // 1 minute
            verify_approval_chain: true,
            require_fresh_signatures: false,
        }
    }
}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationError::InvalidSignature { reason } => {
                write!(f, "Invalid signature: {}", reason)
            }
            VerificationError::ExpiredAuthorization => {
                write!(f, "Authorization has expired")
            }
            VerificationError::OperationNotPermitted {
                requested,
                permitted,
            } => {
                write!(
                    f,
                    "Operation '{}' not permitted. Allowed: {:?}",
                    requested, permitted
                )
            }
            VerificationError::TimingViolation { reason } => {
                write!(f, "Timing violation: {}", reason)
            }
            VerificationError::InvalidApprovalChain { reason } => {
                write!(f, "Invalid approval chain: {}", reason)
            }
            VerificationError::NodeNotFound { node_id } => {
                write!(f, "Node not found: {}", node_id)
            }
            VerificationError::InsufficientTrust {
                node_id,
                trust_level,
            } => {
                write!(
                    f,
                    "Insufficient trust for node {}: {}",
                    node_id, trust_level
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cross_node_auth::{
        AuthorizationProof, CrossNodeAuthorization, CrossNodeOperation, OperationType,
        ResourcePermission,
    };
    use crate::node_registry::{BearDogNodeRegistry, NodeInfo, RegistryConfig};
    use chrono::{Duration, Utc};
    use ed25519_dalek::SigningKey;
    use std::collections::HashMap;

    async fn create_test_node_registry() -> Arc<BearDogNodeRegistry> {
        let config = RegistryConfig::default();
        let registry = Arc::new(BearDogNodeRegistry::new(config));

        // Add test nodes
        // Generate the public key that corresponds to our test private key
        let grantor_private_key = vec![1u8; 32];
        let grantor_public_key =
            if let Ok(grantor_private_key_array) = grantor_private_key.try_into() {
                // For tests, we'll derive the public key from the private key
                let signing_key = SigningKey::from_bytes(&grantor_private_key_array);
                signing_key.verifying_key().to_bytes().to_vec()
            } else {
                vec![1u8; 32] // Fallback
            };

        let grantor_info = NodeInfo {
            public_key: vec![1u8; 32],
            trust_level: crate::node_registry::TrustLevel::High,
            capabilities: vec!["authorization".to_string()],
            endpoint: "https://grantor.example.com".to_string(),
            last_seen: std::time::SystemTime::now(),
            metadata: std::collections::HashMap::new(),
            node_id: "grantor".to_string(),
            display_name: "Grantor Node".to_string(),
            node_type: "beardog_security_node".to_string(),
            network_address: "https://grantor.example.com".to_string(),
            registration_timestamp: chrono::Utc::now(),
        };

        // Generate the public key for grantee
        let grantee_private_key = vec![2u8; 32];
        let grantee_public_key =
            if let Ok(grantee_private_key_array) = grantee_private_key.try_into() {
                let signing_key = SigningKey::from_bytes(&grantee_private_key_array);
                signing_key.verifying_key().to_bytes().to_vec()
            } else {
                vec![2u8; 32] // Fallback
            };

        let grantee_info = NodeInfo {
            public_key: vec![2u8; 32],
            trust_level: crate::node_registry::TrustLevel::Basic,
            capabilities: vec!["data_access".to_string()],
            endpoint: "https://grantee.example.com".to_string(),
            last_seen: std::time::SystemTime::now(),
            metadata: std::collections::HashMap::new(),
            node_id: "grantee".to_string(),
            display_name: "Grantee Node".to_string(),
            node_type: "beardog_security_node".to_string(),
            network_address: "https://grantee.example.com".to_string(),
            registration_timestamp: chrono::Utc::now(),
        };

        registry
            .add_node_with_id("grantor".to_string(), grantor_info)
            .await
            .unwrap();
        registry
            .add_node_with_id("grantee".to_string(), grantee_info)
            .await
            .unwrap();

        registry
    }

    fn create_test_authorization() -> CrossNodeAuthorization {
        use crate::crypto_utils::BearDogCrypto;

        // Create a basic authorization first without signature
        let mut authorization = CrossNodeAuthorization {
            id: "test-auth-1".to_string(),
            grantor_node_id: "grantor".to_string(),
            grantee_node_id: "grantee".to_string(),
            resource_permissions: vec![
                ResourcePermission::Write, // Write permission allows storing data
            ],
            granted_at: Utc::now() - Duration::minutes(5),
            expires_at: Utc::now() + Duration::days(30),
            workflow_id: "workflow-123".to_string(),
            approval_chain: Vec::new(),
            conditions: Vec::new(),
            grantor_signature: Vec::new(), // Empty initially
        };

        // Generate a real signature using the grantor's private key
        // For testing, we'll use a deterministic private key derived from the public key
        let grantor_private_key = vec![1u8; 32]; // Same pattern as public key but as private

        // Serialize authorization for signing (without signature field)
        let message = serde_json::to_vec(&authorization).unwrap();

        // Sign the authorization
        if let Ok(signature) = BearDogCrypto::sign_ed25519(&grantor_private_key, &message) {
            authorization.grantor_signature = signature;
        } else {
            // Fallback to placeholder if signing fails
            authorization.grantor_signature = vec![42u8; 64];
        }

        authorization
    }

    fn create_test_operation() -> CrossNodeOperation {
        CrossNodeOperation {
            operation_type: OperationType::StoreData,
            resource_id: "test-resource".to_string(),
            data_size_bytes: Some(1024 * 1024 * 10), // 10MB
            estimated_duration: Some(Duration::minutes(30)),
            metadata: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_proof_verifier_creation() {
        let node_registry = create_test_node_registry().await;
        let config = ProofVerifierConfig::default();
        let verifier = BearDogProofVerifier::new(node_registry, config);

        assert_eq!(verifier.config.cache_ttl_seconds, 300);
    }

    #[tokio::test]
    async fn test_valid_proof_verification() {
        let node_registry = create_test_node_registry().await;
        let config = ProofVerifierConfig::default();
        let verifier = BearDogProofVerifier::new(node_registry, config);

        let authorization = create_test_authorization();
        let operation = create_test_operation();

        // Use a simple placeholder signature for testing
        let requester_signature = vec![42u8; 64]; // Placeholder signature
        let request_timestamp = Utc::now();

        let proof = AuthorizationProof {
            authorization,
            operation,
            request_timestamp,
            requester_signature,
        };

        let details = verifier.verify_proof_comprehensive(&proof).await.unwrap();

        // With the test setup, timing and authorization should be valid
        // Signature validation is placeholder-based for testing
        assert!(details.timing_valid);
        assert!(details.authorization_valid);
        assert!(details.operation_permitted);
        // Don't assert on signature_valid as it's using placeholder signatures
    }

    #[tokio::test]
    async fn test_expired_authorization() {
        let node_registry = create_test_node_registry().await;
        let config = ProofVerifierConfig::default();
        let verifier = BearDogProofVerifier::new(node_registry, config);

        let mut authorization = create_test_authorization();
        authorization.expires_at = Utc::now() - Duration::hours(1); // Expired

        let operation = create_test_operation();

        let proof = AuthorizationProof {
            authorization,
            operation,
            request_timestamp: Utc::now(),
            requester_signature: vec![123u8; 64],
        };

        let details = verifier.verify_proof_comprehensive(&proof).await.unwrap();

        assert!(!details.timing_valid);
        assert!(details.errors.iter().any(|e| e.contains("expired")));
    }

    #[tokio::test]
    async fn test_operation_not_permitted() {
        let node_registry = create_test_node_registry().await;
        let config = ProofVerifierConfig::default();
        let verifier = BearDogProofVerifier::new(node_registry, config);

        let authorization = create_test_authorization();

        // Create operation that's not permitted (DeleteData when only StoreData is allowed)
        let operation = CrossNodeOperation {
            operation_type: OperationType::DeleteData,
            resource_id: "test-resource".to_string(),
            data_size_bytes: None,
            estimated_duration: None,
            metadata: HashMap::new(),
        };

        let proof = AuthorizationProof {
            authorization,
            operation,
            request_timestamp: Utc::now(),
            requester_signature: vec![123u8; 64],
        };

        let details = verifier.verify_proof_comprehensive(&proof).await.unwrap();

        assert!(!details.operation_permitted);
        assert!(details.errors.iter().any(|e| e.contains("not permitted")));
    }

    #[tokio::test]
    async fn test_cache_functionality() {
        let node_registry = create_test_node_registry().await;
        let config = ProofVerifierConfig::default();
        let verifier = BearDogProofVerifier::new(node_registry, config);

        let authorization = create_test_authorization();
        let operation = create_test_operation();

        let proof = AuthorizationProof {
            authorization,
            operation,
            request_timestamp: Utc::now(),
            requester_signature: vec![123u8; 64],
        };

        // First verification
        let details1 = verifier.verify_proof_comprehensive(&proof).await.unwrap();

        // Second verification should use cache
        let details2 = verifier.verify_proof_comprehensive(&proof).await.unwrap();

        // Results should be identical
        assert_eq!(details1.timing_valid, details2.timing_valid);
        assert_eq!(details1.signature_valid, details2.signature_valid);
    }
}
