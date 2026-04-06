// SPDX-License-Identifier: AGPL-3.0-or-later

//! Security and attestation types for primal ecosystem integration.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Attestation verification result
///
/// Contains the result of verifying a service's attestation, including
/// whether verification succeeded, confidence level, and method used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationVerificationResult {
    /// Whether the attestation was successfully verified
    pub verified: bool,
    /// Confidence level in the verification (0.0 to 1.0)
    pub confidence: f64,
    /// Method used for verification
    pub method: String,
    /// Timestamp when verification was performed
    pub timestamp: DateTime<Utc>,
}

/// Attestation verification chain
///
/// Represents a chain of attestations for verifying service authenticity
/// across multiple verification points with an overall trust level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationVerificationChain {
    /// Whether the entire chain was successfully verified
    pub verified: bool,
    /// Chain of attestation identifiers forming the trust path
    pub chain: Vec<String>,
    /// Overall trust level for this verification chain
    pub trust_level: String,
}

/// Capability health status
///
/// Tracks health status for multiple capabilities, providing both
/// an overall status and individual status for each capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityHealthStatus {
    /// Overall health status across all capabilities
    pub overall_status: String,
    /// Health status for each individual capability
    pub individual_status: HashMap<String, String>,
}

/// Authentication result
///
/// Contains the outcome of an authentication attempt, including
/// user identity, access token, expiration, and granted permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    /// Whether authentication succeeded
    pub success: bool,
    /// Authenticated user ID if successful
    pub user_id: Option<String>,
    /// Access token for authenticated sessions
    pub token: Option<String>,
    /// Token expiration timestamp
    pub expires_at: Option<DateTime<Utc>>,
    /// List of permissions granted to this user
    pub permissions: Vec<String>,
}

/// Security attestation for service verification
///
/// Cryptographic proof of service identity, security posture, and compliance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityAttestation {
    /// Type of attestation (TPM, SGX, etc.)
    pub attestation_type: String,
    /// Cryptographic signature proving attestation
    pub signature: String,
    /// Timestamp when attestation was created
    pub timestamp: DateTime<Utc>,
    /// Entity that issued the attestation
    pub issuer: String,
}

/// Authentication requirements for service access
///
/// Specifies authentication method, required permissions, and token settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AuthRequirements {
    /// Authentication type required (JWT, `OAuth2`, etc.)
    #[serde(default)]
    pub auth_type: String,
    /// Required permission scopes for access
    #[serde(default)]
    pub required_scopes: Vec<String>,
    /// Optional token lifetime in seconds
    pub token_lifetime: Option<u64>,
}
