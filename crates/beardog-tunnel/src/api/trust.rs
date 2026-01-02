//! Trust Evaluation API for biomeOS Integration
//!
//! This module provides a clean, stateless API for Songbird (and other orchestrators)
//! to evaluate trust decisions based on genetic lineage and cryptographic tags.
//!
//! ## Architecture
//!
//! - **BearDog**: Cryptography layer (knows lineage, BTSP, BirdSong)
//! - **Songbird**: Orchestration layer (knows discovery, federation)
//! - **Clean API**: Songbird doesn't need to understand crypto, just asks for decisions
//!
//! ## Endpoints
//!
//! - `GET /api/v1/identity` - Get our encryption tag and capabilities
//! - `POST /api/v1/trust/evaluate` - Evaluate trust for a peer
//!
//! ## API Format
//!
//! Supports two formats:
//! - **Universal Trust v1**: Generic, capability-based (recommended)
//! - **Legacy**: Backward compatible with early integrations

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, info, warn};

use crate::api::types::{ApiError, ApiResponse};

/// Trust evaluation state (shared with API server)
#[derive(Clone)]
pub struct TrustApiState {
    /// Our family ID (from USB seed or genesis)
    pub family_id: Option<String>,
    /// Our encryption tag (advertised in discovery)
    pub encryption_tag: String,
    /// Our capabilities
    pub capabilities: Vec<String>,
}

impl TrustApiState {
    /// Create new trust API state
    pub fn new(family_id: Option<String>, node_id: &str) -> Self {
        // Generate encryption tag
        let encryption_tag = if let Some(ref family) = family_id {
            format!("beardog:family:{}:{}", family, node_id)
        } else {
            format!("beardog:node:{}", node_id)
        };

        Self {
            family_id,
            encryption_tag,
            capabilities: vec![
                "btsp".to_string(),
                "birdsong".to_string(),
                "lineage".to_string(),
            ],
        }
    }
}

/// Response from identity endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityResponse {
    /// Our encryption tag (to advertise in discovery)
    pub encryption_tag: String,
    /// Our capabilities
    pub capabilities: Vec<String>,
    /// Our family ID (if part of a USB family)
    pub family_id: Option<String>,
    /// Identity attestations (for generic discovery)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_attestations: Option<Vec<IdentityAttestation>>,
}

/// Identity attestation (generic format for discovery)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAttestation {
    /// Provider capability (e.g., "security/identity")
    pub provider_capability: String,
    /// Format of the attestation (e.g., "tag_list")
    pub format: String,
    /// Attestation data (format-specific)
    pub data: Value,
}

/// Evaluator information (for universal_trust_v1 format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluatorInfo {
    /// Peer's node ID
    pub peer_id: String,
    /// Identity attestations from the peer
    pub attestations: Vec<IdentityAttestation>,
}

/// Request to evaluate trust for a peer
///
/// Supports two formats:
/// 1. Universal Trust v1 (recommended): Uses `request_format`, `evaluator`, `context` (Value)
/// 2. Legacy: Uses `peer_id`, `peer_tags`, `context` (HashMap)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEvaluationRequest {
    // === Universal Trust v1 Format ===
    /// Request format version (e.g., "universal_trust_v1")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_format: Option<String>,
    /// Evaluator info (universal format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluator: Option<EvaluatorInfo>,
    /// Context (universal format - flexible JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    // === Progressive Trust v1 Format ===
    /// Requested operation (e.g., "data/read", "commands/execute")
    /// Used to determine if operation is allowed at current trust level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_operation: Option<String>,

    // === Legacy Format (Backward Compatible) ===
    /// Peer's node ID (legacy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_id: Option<String>,
    /// Peer's advertised tags (legacy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_tags: Option<Vec<String>>,
    /// Connection info (legacy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_info: Option<HashMap<String, String>>,
}

/// Trust decision result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TrustDecision {
    /// Auto-accept (same family, high trust)
    AutoAccept,
    /// Prompt user (different family or unknown)
    PromptUser,
    /// Reject (no lineage, untrusted)
    Reject,
}

/// Trust level (progressive trust model)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum TrustLevel {
    /// No lineage or different family → Reject
    None = 0,
    /// Same genetic family → LIMITED coordination
    /// Can: Hear BirdSong, see capabilities, health checks
    /// CANNOT: Access data, execute commands, full federation
    Limited = 1,
    /// Human approved → FULL federation
    /// Can: Everything in Limited + federation + resource sharing
    /// CANNOT: Sensitive operations, key access
    Elevated = 2,
    /// Human entropy added → HIGHEST trust
    /// Can: Everything including sensitive operations
    Highest = 3,
}

/// Elevation path - how to increase trust level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElevationPath {
    /// Next trust level available
    pub next_level: u8,
    /// What's required to elevate
    pub requirements: Vec<String>,
    /// Method to elevate (e.g., "user_consent_ui", "human_entropy")
    pub method: String,
}

/// Response from trust evaluation
///
/// Supports Universal Trust v1 format with backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEvaluationResponse {
    // === Universal Trust v1 Format ===
    /// Response format version (e.g., "universal_trust_v1")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    /// Trust decision (auto_accept, prompt_user, reject)
    pub decision: TrustDecision,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Human-readable reason
    pub reason: String,
    /// Machine-readable reason code (e.g., "same_genetic_family")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    /// Additional metadata (extensible)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    /// When this trust decision expires (for caching)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    // === Progressive Trust v1 Format ===
    /// Numeric trust level (0-3: None, Limited, Elevated, Highest)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_level_numeric: Option<u8>,
    /// Allowed capabilities at this trust level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_capabilities: Option<Vec<String>>,
    /// Denied capabilities at this trust level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denied_capabilities: Option<Vec<String>>,
    /// How to elevate trust to next level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation_path: Option<ElevationPath>,

    // === Legacy Fields (Backward Compatible) ===
    /// Trust level (high, medium, low, none) - legacy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_level: Option<TrustLevel>,
    /// Peer's encryption tag (if found) - legacy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_tag: Option<String>,
}

/// Request to elevate trust level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustElevationRequest {
    /// Peer ID to elevate
    pub peer_id: String,
    /// Current trust level
    pub current_level: u8,
    /// Requested trust level
    pub requested_level: u8,
    /// Evidence for elevation
    pub evidence: ElevationEvidence,
}

/// Evidence for trust elevation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElevationEvidence {
    /// Type of evidence (e.g., "human_approval", "human_entropy")
    #[serde(rename = "type")]
    pub evidence_type: String,
    /// Timestamp of evidence
    pub timestamp: String,
    /// Method used (e.g., "user_consent_ui", "phone_hsm")
    pub method: String,
    /// Optional entropy or signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entropy: Option<String>,
}

/// Response from trust elevation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustElevationResponse {
    /// Whether elevation succeeded
    pub success: bool,
    /// New trust level
    pub new_level: u8,
    /// Human-readable message
    pub message: String,
    /// New allowed capabilities
    pub new_allowed_capabilities: Vec<String>,
}

/// Create trust evaluation routes
pub fn routes(state: TrustApiState) -> Router {
    Router::new()
        .route("/identity", get(get_identity))
        .route("/evaluate", post(evaluate_trust))
        .route("/elevate", post(elevate_trust))
        .with_state(state)
}

// ====================================================================================
// Helper Functions
// ====================================================================================

/// Get allowed and denied capabilities for a trust level
fn get_capabilities_for_level(level: TrustLevel) -> (Vec<String>, Vec<String>) {
    match level {
        TrustLevel::Limited => (
            vec![
                "discovery".to_string(),
                "coordination/*".to_string(),
                "health".to_string(),
                "capabilities".to_string(),
            ],
            vec![
                "data/*".to_string(),
                "commands/*".to_string(),
                "federation/*".to_string(),
                "keys/*".to_string(),
            ],
        ),
        TrustLevel::Elevated => (
            vec![
                "discovery".to_string(),
                "coordination/*".to_string(),
                "health".to_string(),
                "capabilities".to_string(),
                "federation/*".to_string(),
                "data/read".to_string(),
            ],
            vec![
                "data/write".to_string(),
                "commands/sensitive".to_string(),
                "keys/*".to_string(),
            ],
        ),
        TrustLevel::Highest => (
            vec!["*".to_string()], // Everything allowed
            vec![],                // Nothing denied
        ),
        TrustLevel::None => (
            vec![],         // Nothing allowed
            vec!["*".to_string()], // Everything denied
        ),
    }
}

/// Check if an operation is allowed given the capability lists
fn is_operation_allowed(
    operation: &str,
    allowed: &[String],
    denied: &[String],
) -> bool {
    // Check denied first (deny takes precedence)
    for pattern in denied {
        if matches_capability_pattern(operation, pattern) {
            return false;
        }
    }

    // Then check allowed
    for pattern in allowed {
        if matches_capability_pattern(operation, pattern) {
            return true;
        }
    }

    false
}

/// Check if operation matches a capability pattern
/// Supports wildcards: "data/*" matches "data/read", "data/write", etc.
fn matches_capability_pattern(operation: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }

    if pattern.ends_with("/*") {
        let prefix = &pattern[..pattern.len() - 2];
        return operation.starts_with(prefix);
    }

    operation == pattern
}

// ====================================================================================
// Endpoint Handlers
// ====================================================================================

/// GET /trust/identity - Get our identity and encryption tag
///
/// Returns unwrapped response (HTTP status indicates success/failure)
async fn get_identity(
    State(state): State<TrustApiState>,
) -> Result<Json<IdentityResponse>, ApiError> {
    info!("🔍 Identity query received");

    // Build identity attestations (generic format for discovery)
    let attestations = if let Some(ref family_id) = state.family_id {
        let mut tags = vec![state.encryption_tag.clone()];
        tags.extend(state.capabilities.iter().map(|c| format!("beardog:{}", c)));

        Some(vec![IdentityAttestation {
            provider_capability: "security/identity".to_string(),
            format: "tag_list".to_string(),
            data: serde_json::json!({
                "tags": tags,
                "family_id": family_id
            }),
        }])
    } else {
        None
    };

    let response = IdentityResponse {
        encryption_tag: state.encryption_tag.clone(),
        capabilities: state.capabilities.clone(),
        family_id: state.family_id.clone(),
        identity_attestations: attestations,
    };

    // Return response directly (unwrapped) - HTTP 200 indicates success
    Ok(Json(response))
}

/// Extract tags from identity attestations (universal format)
fn extract_tags_from_attestations(attestations: &[IdentityAttestation]) -> Vec<String> {
    for att in attestations {
        if att.format == "tag_list" {
            if let Some(tags) = att.data.get("tags").and_then(|t| t.as_array()) {
                return tags
                    .iter()
                    .filter_map(|t| t.as_str().map(String::from))
                    .collect();
            }
        }
    }
    vec![] // No tags found
}

/// POST /trust/evaluate - Evaluate trust for a peer
async fn evaluate_trust(
    State(state): State<TrustApiState>,
    Json(request): Json<TrustEvaluationRequest>,
) -> Result<Json<TrustEvaluationResponse>, ApiError> {
    // Determine format and extract peer_id + tags
    let (peer_id, peer_tags, is_universal) = if let Some(ref fmt) = request.request_format {
        // Universal Trust v1 format
        if fmt == "universal_trust_v1" {
            debug!("🔍 Universal Trust v1 request received");
            let evaluator = request
                .evaluator
                .as_ref()
                .ok_or_else(|| ApiError::bad_request("Missing evaluator field"))?;
            let tags = extract_tags_from_attestations(&evaluator.attestations);
            (evaluator.peer_id.clone(), tags, true)
        } else {
            return Err(ApiError::bad_request(&format!(
                "Unsupported request format: {}",
                fmt
            )));
        }
    } else {
        // Legacy format
        debug!("🔍 Legacy trust evaluation request");
        let peer_id = request
            .peer_id
            .as_ref()
            .ok_or_else(|| ApiError::bad_request("Missing peer_id"))?
            .clone();
        let peer_tags = request
            .peer_tags
            .as_ref()
            .ok_or_else(|| ApiError::bad_request("Missing peer_tags"))?
            .clone();
        (peer_id, peer_tags, false)
    };

    info!("🔍 Trust evaluation request for peer: {}", peer_id);

    // Extract BearDog family tag from peer tags
    let peer_family_tag = peer_tags
        .iter()
        .find(|t| t.starts_with("beardog:family:"))
        .map(|t| t.to_string());

    // Extract family ID from tag (beardog:family:a3f2:tower1 → a3f2)
    let peer_family_id = peer_family_tag.as_ref().and_then(|tag| {
        tag.split(':').nth(2).map(|s| s.to_string())
    });

    // Evaluate trust based on family relationship (progressive trust model)
    let (decision, trust_level, confidence, reason, reason_code, metadata_value) = match (
        &state.family_id,
        peer_family_id,
    ) {
        // Both have families, same family → LIMITED trust (auto-accept for coordination only)
        (Some(our_family), Some(peer_family)) if our_family == &peer_family => {
            (
                TrustDecision::AutoAccept,
                TrustLevel::Limited, // Changed from High to Limited
                1.0,
                format!("Same genetic family ({}) - limited trust for coordination", our_family),
                "same_genetic_family",
                serde_json::json!({
                    "same_family": true,
                    "family_id": our_family,
                    "provider": "beardog",
                    "lineage_verified": true,
                    "relationship": "sibling",
                    "trust_model": "progressive"
                }),
            )
        }

        // Both have families, different families → prompt user (no auto-trust)
        (Some(our_family), Some(peer_family)) => {
            (
                TrustDecision::PromptUser,
                TrustLevel::None, // Changed from Medium to None
                0.5,
                format!(
                    "Valid lineage but different genetic family ({} vs {})",
                    peer_family, our_family
                ),
                "different_genetic_family",
                serde_json::json!({
                    "same_family": false,
                    "peer_family_id": peer_family,
                    "our_family_id": our_family,
                    "provider": "beardog",
                    "relationship": "stranger"
                }),
            )
        }

        // We have family, peer doesn't → prompt user (could be legacy)
        (Some(_our_family), None) => {
            (
                TrustDecision::PromptUser,
                TrustLevel::None, // Changed from Low to None
                0.3,
                "Peer has no genetic lineage".to_string(),
                "peer_has_no_genetic_lineage",
                serde_json::json!({
                    "provider": "beardog"
                }),
            )
        }

        // We don't have family, peer does → prompt user
        (None, Some(peer_family)) => {
            (
                TrustDecision::PromptUser,
                TrustLevel::None, // Changed from Low to None
                0.3,
                "We have no family lineage".to_string(),
                "we_have_no_family_lineage",
                serde_json::json!({
                    "peer_family_id": peer_family,
                    "provider": "beardog"
                }),
            )
        }

        // Neither has family → reject (no crypto identity)
        (None, None) => {
            (
                TrustDecision::Reject,
                TrustLevel::None,
                0.0,
                "No genetic lineage present".to_string(),
                "no_genetic_lineage_present",
                serde_json::json!({
                    "provider": "beardog"
                }),
            )
        }
    };

    // Get capability restrictions for this trust level
    let (allowed_caps, denied_caps) = get_capabilities_for_level(trust_level);

    // Check if requested operation is allowed (if specified)
    let operation_allowed = if let Some(ref op) = request.requested_operation {
        is_operation_allowed(op, &allowed_caps, &denied_caps)
    } else {
        true // No specific operation requested
    };

    // Determine elevation path
    let elevation_path = if trust_level < TrustLevel::Highest {
        let next_level = (trust_level as u8) + 1;
        let (requirements, method) = match next_level {
            2 => (
                vec!["human_approval".to_string()],
                "user_consent_ui".to_string(),
            ),
            3 => (
                vec!["human_entropy".to_string()],
                "phone_hsm_or_solokey".to_string(),
            ),
            _ => (vec![], "".to_string()),
        };
        Some(ElevationPath {
            next_level,
            requirements,
            method,
        })
    } else {
        None // Already at highest trust
    };

    // Calculate expiry (24 hours from now)
    let expires_at = if is_universal {
        let expiry = chrono::Utc::now() + chrono::Duration::hours(24);
        Some(expiry.to_rfc3339())
    } else {
        None
    };

    // Build response based on format
    let response = if is_universal {
        // Universal Trust v1 format (with progressive trust)
        TrustEvaluationResponse {
            response_format: Some("universal_trust_v1".to_string()),
            decision: if operation_allowed && trust_level > TrustLevel::None {
                decision.clone()
            } else {
                TrustDecision::Reject // Reject if operation not allowed or no trust
            },
            confidence,
            reason: reason.clone(),
            reason_code: Some(reason_code.to_string()),
            metadata: Some(metadata_value),
            expires_at,
            // Progressive trust fields
            trust_level_numeric: Some(trust_level as u8),
            allowed_capabilities: Some(allowed_caps.clone()),
            denied_capabilities: Some(denied_caps.clone()),
            elevation_path,
            // Legacy fields (optional)
            trust_level: Some(trust_level),
            encryption_tag: peer_family_tag.clone(),
        }
    } else {
        // Legacy format (convert metadata_value to HashMap for backward compatibility)
        let metadata_map = if let Some(obj) = metadata_value.as_object() {
            Some(
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect::<HashMap<String, String>>(),
            )
        } else {
            None
        };

        TrustEvaluationResponse {
            response_format: None,
            decision: decision.clone(),
            confidence,
            reason: reason_code.to_string(), // Use reason_code for backward compatibility
            reason_code: None,
            metadata: metadata_map.map(|m: HashMap<String, String>| {
                serde_json::to_value(m).unwrap_or(serde_json::Value::Null)
            }),
            expires_at: None,
            // Legacy format: don't include progressive trust fields
            trust_level_numeric: None,
            allowed_capabilities: None,
            denied_capabilities: None,
            elevation_path: None,
            trust_level: Some(trust_level),
            encryption_tag: peer_family_tag,
        }
    };

    info!(
        "✅ Trust evaluation complete: {:?} (level={}) - format: {}",
        decision,
        trust_level as u8,
        if is_universal { "universal" } else { "legacy" }
    );

    // Return response directly (unwrapped) for Generic Trust API compliance
    Ok(Json(response))
}

/// POST /trust/elevate - Elevate trust level for a peer
///
/// This endpoint allows elevation of trust after human approval or entropy addition.
/// In a real implementation, this would:
/// 1. Verify the evidence (signature, entropy, etc.)
/// 2. Store the elevated trust level persistently
/// 3. Return the new capabilities
///
/// For now, this is a demonstration implementation that validates the request
/// and returns what the new capabilities would be.
async fn elevate_trust(
    State(state): State<TrustApiState>,
    Json(request): Json<TrustElevationRequest>,
) -> Result<Json<TrustElevationResponse>, ApiError> {
    info!(
        "🔓 Trust elevation request for peer: {} (current={}, requested={})",
        request.peer_id, request.current_level, request.requested_level
    );

    // Validate current and requested levels
    if request.current_level > 3 {
        return Err(ApiError::bad_request("Invalid current_level (must be 0-3)"));
    }
    if request.requested_level > 3 {
        return Err(ApiError::bad_request(
            "Invalid requested_level (must be 0-3)",
        ));
    }
    if request.requested_level <= request.current_level {
        return Err(ApiError::bad_request(
            "Requested level must be higher than current level",
        ));
    }

    // Validate evidence for the requested level
    let valid_evidence = match request.requested_level {
        2 => {
            // Elevated (2) requires human approval
            request.evidence.evidence_type == "human_approval"
                && request.evidence.method == "user_consent_ui"
        }
        3 => {
            // Highest (3) requires human entropy
            request.evidence.evidence_type == "human_entropy"
                && (request.evidence.method == "phone_hsm"
                    || request.evidence.method == "solokey")
                && request.evidence.entropy.is_some()
        }
        _ => {
            // Only levels 2 and 3 can be elevated to
            return Err(ApiError::bad_request(
                "Can only elevate to level 2 (Elevated) or 3 (Highest)",
            ));
        }
    };

    if !valid_evidence {
        return Err(ApiError::bad_request(&format!(
            "Invalid evidence for level {} elevation",
            request.requested_level
        )));
    }

    // Get new capabilities for the requested level
    let requested_trust_level = match request.requested_level {
        0 => TrustLevel::None,
        1 => TrustLevel::Limited,
        2 => TrustLevel::Elevated,
        3 => TrustLevel::Highest,
        _ => unreachable!(),
    };

    let (new_allowed_caps, _denied_caps) =
        get_capabilities_for_level(requested_trust_level);

    info!(
        "✅ Trust elevation validated: {} -> level {} with {} capabilities",
        request.peer_id,
        request.requested_level,
        new_allowed_caps.len()
    );

    // In production, we would:
    // 1. Store the elevated trust level in a database/cache
    // 2. Verify cryptographic signatures on the evidence
    // 3. Audit log the elevation event
    // For now, just return success with the new capabilities

    Ok(Json(TrustElevationResponse {
        success: true,
        new_level: request.requested_level,
        message: format!(
            "Trust elevated to level {} ({})",
            request.requested_level,
            match requested_trust_level {
                TrustLevel::Elevated => "Elevated",
                TrustLevel::Highest => "Highest",
                _ => "Unknown",
            }
        ),
        new_allowed_capabilities: new_allowed_caps,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_api_state_with_family() {
        let state = TrustApiState::new(Some("a3f2".to_string()), "tower1");

        assert_eq!(state.family_id, Some("a3f2".to_string()));
        assert_eq!(state.encryption_tag, "beardog:family:a3f2:tower1");
        assert!(state.capabilities.contains(&"btsp".to_string()));
        assert!(state.capabilities.contains(&"birdsong".to_string()));
        assert!(state.capabilities.contains(&"lineage".to_string()));
    }

    #[test]
    fn test_trust_api_state_without_family() {
        let state = TrustApiState::new(None, "tower1");

        assert_eq!(state.family_id, None);
        assert_eq!(state.encryption_tag, "beardog:node:tower1");
        assert_eq!(state.capabilities.len(), 3);
    }

    #[test]
    fn test_trust_decision_serialization() {
        let decision = TrustDecision::AutoAccept;
        let json = serde_json::to_string(&decision).unwrap();
        assert_eq!(json, "\"auto_accept\"");

        let decision = TrustDecision::PromptUser;
        let json = serde_json::to_string(&decision).unwrap();
        assert_eq!(json, "\"prompt_user\"");

        let decision = TrustDecision::Reject;
        let json = serde_json::to_string(&decision).unwrap();
        assert_eq!(json, "\"reject\"");
    }

    #[test]
    fn test_progressive_trust_levels() {
        // Test trust level ordering
        assert!(TrustLevel::None < TrustLevel::Limited);
        assert!(TrustLevel::Limited < TrustLevel::Elevated);
        assert!(TrustLevel::Elevated < TrustLevel::Highest);

        // Test numeric values
        assert_eq!(TrustLevel::None as u8, 0);
        assert_eq!(TrustLevel::Limited as u8, 1);
        assert_eq!(TrustLevel::Elevated as u8, 2);
        assert_eq!(TrustLevel::Highest as u8, 3);
    }

    #[test]
    fn test_capability_restrictions() {
        // Test Limited (1) - coordination only
        let (allowed, denied) = get_capabilities_for_level(TrustLevel::Limited);
        assert!(allowed.contains(&"coordination/*".to_string()));
        assert!(allowed.contains(&"health".to_string()));
        assert!(denied.contains(&"data/*".to_string()));
        assert!(denied.contains(&"federation/*".to_string()));

        // Test Elevated (2) - can federate and read data
        let (allowed, denied) = get_capabilities_for_level(TrustLevel::Elevated);
        assert!(allowed.contains(&"federation/*".to_string()));
        assert!(allowed.contains(&"data/read".to_string()));
        assert!(denied.contains(&"keys/*".to_string()));

        // Test Highest (3) - everything allowed
        let (allowed, denied) = get_capabilities_for_level(TrustLevel::Highest);
        assert_eq!(allowed, vec!["*"]);
        assert!(denied.is_empty());

        // Test None (0) - nothing allowed
        let (allowed, denied) = get_capabilities_for_level(TrustLevel::None);
        assert!(allowed.is_empty());
        assert_eq!(denied, vec!["*"]);
    }

    #[test]
    fn test_operation_matching() {
        // Test exact matches
        assert!(is_operation_allowed(
            "health",
            &["health".to_string()],
            &[]
        ));

        // Test wildcard matches
        assert!(is_operation_allowed(
            "coordination/birdsong",
            &["coordination/*".to_string()],
            &[]
        ));
        assert!(is_operation_allowed(
            "data/read",
            &["data/*".to_string()],
            &[]
        ));

        // Test deny takes precedence
        assert!(!is_operation_allowed(
            "data/write",
            &["data/*".to_string()],
            &["data/write".to_string()]
        ));

        // Test wildcard deny
        assert!(!is_operation_allowed(
            "keys/delete",
            &["*".to_string()],
            &["keys/*".to_string()]
        ));
    }

    #[test]
    fn test_capability_pattern_matching() {
        // Test exact match
        assert!(matches_capability_pattern("health", "health"));

        // Test prefix wildcard
        assert!(matches_capability_pattern("data/read", "data/*"));
        assert!(matches_capability_pattern("data/write", "data/*"));
        assert!(!matches_capability_pattern("commands/execute", "data/*"));

        // Test global wildcard
        assert!(matches_capability_pattern("anything", "*"));
        assert!(matches_capability_pattern("data/read", "*"));

        // Test no match
        assert!(!matches_capability_pattern("health", "data"));
    }

    #[test]
    fn test_trust_level_serialization() {
        let level = TrustLevel::High;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"high\"");

        let level = TrustLevel::Medium;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"medium\"");

        let level = TrustLevel::Low;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"low\"");

        let level = TrustLevel::None;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"none\"");
    }

    #[test]
    fn test_identity_response_serialization() {
        let response = IdentityResponse {
            encryption_tag: "beardog:family:a3f2:tower1".to_string(),
            capabilities: vec!["btsp".to_string(), "birdsong".to_string()],
            family_id: Some("a3f2".to_string()),
            identity_attestations: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("beardog:family:a3f2:tower1"));
        assert!(json.contains("btsp"));
        assert!(json.contains("a3f2"));
    }

    #[test]
    fn test_trust_evaluation_request_legacy_deserialization() {
        let json = r#"{
            "peer_id": "tower2",
            "peer_tags": ["beardog:family:a3f2:tower2", "btsp_enabled"],
            "connection_info": {"endpoint": "http://192.168.1.134:8080"}
        }"#;

        let request: TrustEvaluationRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.peer_id, Some("tower2".to_string()));
        assert_eq!(request.peer_tags.as_ref().unwrap().len(), 2);
        assert_eq!(
            request
                .connection_info
                .as_ref()
                .unwrap()
                .get("endpoint")
                .unwrap(),
            "http://192.168.1.134:8080"
        );
    }

    #[test]
    fn test_trust_evaluation_request_universal_deserialization() {
        let json = r#"{
            "request_format": "universal_trust_v1",
            "evaluator": {
                "peer_id": "tower2",
                "attestations": [{
                    "provider_capability": "security/identity",
                    "format": "tag_list",
                    "data": {
                        "tags": ["beardog:family:a3f2:tower2", "btsp_enabled"],
                        "family_id": "a3f2"
                    }
                }]
            },
            "context": {
                "discovery_method": "udp_multicast",
                "endpoint": "http://192.168.1.134:8080"
            }
        }"#;

        let request: TrustEvaluationRequest = serde_json::from_str(json).unwrap();
        assert_eq!(
            request.request_format,
            Some("universal_trust_v1".to_string())
        );
        assert!(request.evaluator.is_some());
        assert_eq!(request.evaluator.as_ref().unwrap().peer_id, "tower2");
        assert_eq!(
            request.evaluator.as_ref().unwrap().attestations.len(),
            1
        );
    }

    #[test]
    fn test_trust_evaluation_response_universal_serialization() {
        let response = TrustEvaluationResponse {
            response_format: Some("universal_trust_v1".to_string()),
            decision: TrustDecision::AutoAccept,
            confidence: 1.0,
            reason: "Same genetic family (a3f2)".to_string(),
            reason_code: Some("same_genetic_family".to_string()),
            metadata: Some(serde_json::json!({
                "same_family": true,
                "family_id": "a3f2",
                "provider": "beardog"
            })),
            expires_at: Some("2026-01-03T00:00:00Z".to_string()),
            trust_level: Some(TrustLevel::High),
            encryption_tag: Some("beardog:family:a3f2:tower2".to_string()),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("universal_trust_v1"));
        assert!(json.contains("auto_accept"));
        assert!(json.contains("1.0"));
        assert!(json.contains("same_genetic_family"));
        assert!(json.contains("a3f2"));
    }

    #[test]
    fn test_extract_tags_from_attestations() {
        let attestations = vec![IdentityAttestation {
            provider_capability: "security/identity".to_string(),
            format: "tag_list".to_string(),
            data: serde_json::json!({
                "tags": ["beardog:family:a3f2:tower2", "btsp_enabled"],
                "family_id": "a3f2"
            }),
        }];

        let tags = extract_tags_from_attestations(&attestations);
        assert_eq!(tags.len(), 2);
        assert!(tags.contains(&"beardog:family:a3f2:tower2".to_string()));
        assert!(tags.contains(&"btsp_enabled".to_string()));
    }

    #[test]
    fn test_extract_tags_from_empty_attestations() {
        let attestations: Vec<IdentityAttestation> = vec![];
        let tags = extract_tags_from_attestations(&attestations);
        assert_eq!(tags.len(), 0);
    }
}

