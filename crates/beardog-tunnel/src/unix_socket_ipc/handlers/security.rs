// SPDX-License-Identifier: AGPL-3.0-or-later

//! Security and trust evaluation handlers
//!
//! Provides core security capabilities including:
//! - Trust evaluation based on genetic family lineage
//! - Lineage information for identity verification
//! - `BirdSong` encryption/decryption for secure discovery
//! - JWT secret generation for authentication systems

use super::utils::get_primal_name;
use super::{HandlerError, HandlerResult, MethodHandler};
use crate::btsp_provider::BeardogBtspProvider;
use base64::Engine;
use beardog_types::primal_identity::PrimalIdentity;
use chrono::Utc;
use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::Sha256;
use std::sync::Arc;
use tracing::{debug, info, warn};

type HmacSha256 = Hmac<Sha256>;

/// Handler for security and trust methods
///
/// Supports:
/// - `security.evaluate` / `trust.evaluate` - Evaluate trust level based on genetic family
/// - `security.lineage` / `trust.lineage` - Get genetic lineage information
/// - `security.verify_consent` - Verify HMAC consent tokens (wetSpring vault gate)
/// - `security.issue_consent_token` - Issue HMAC consent tokens for owner/scope pairs
/// - `birdsong.encrypt` / `birdsong.decrypt` - `BirdSong` encryption for secure discovery
/// - `birdsong.generate_encrypted_beacon` - Generate a beacon + encrypt it for a family
/// - `security.generate_jwt_secret` - Generate cryptographically secure JWT secrets
///
/// # Architecture
///
/// Uses explicit `PrimalIdentity` injection (no environment variables).
/// This enables:
/// - Concurrent-safe operation (no global state)
/// - Testable with different identities in parallel
/// - Explicit dependencies (visible in constructor)
pub struct SecurityHandler {
    /// Primal identity (family and node)
    ///
    /// Injected at construction, immutable, shared via Arc.
    /// Following TRUE PRIMAL pattern: primal only knows itself.
    identity: Arc<PrimalIdentity>,
}

impl MethodHandler for SecurityHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            // Trust evaluation
            "security.evaluate",
            "trust.evaluate",
            "security.evaluate_trust",
            "trust.evaluate_peer",
            // Lineage information
            "security.lineage",
            "trust.lineage",
            "security.get_lineage",
            "trust.get_lineage",
            // Consent verification (wetSpring vault gate, NUCLEUS consent protocol)
            "security.verify_consent",
            "security.issue_consent_token",
            // BirdSong encryption (semantic domain.operation first; beardog.* = backward compat)
            "birdsong.encrypt",
            "beardog.birdsong.encrypt",
            "birdsong.decrypt",
            "beardog.birdsong.decrypt",
            "birdsong.generate_encrypted_beacon",
            // JWT secret generation (semantic security.* first; beardog.* = backward compat)
            "security.generate_jwt_secret",
            "security.jwt_secret",
            "beardog.generate_jwt_secret",
            "beardog.jwt_secret",
            // rootPulse step handler: primal-identity signing for provenance graphs
            "auth.sign",
        ]
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> HandlerResult {
        match method {
            "security.evaluate"
            | "trust.evaluate"
            | "security.evaluate_trust"
            | "trust.evaluate_peer" => self.handle_trust_evaluation(params),
            "security.lineage" | "trust.lineage" | "security.get_lineage" | "trust.get_lineage" => {
                self.handle_lineage()
            }
            "security.verify_consent" => self.handle_verify_consent(params),
            "security.issue_consent_token" => self.handle_issue_consent_token(params),
            "birdsong.encrypt" | "beardog.birdsong.encrypt" => {
                self.handle_birdsong_encrypt(params, btsp_provider)
            }
            "birdsong.decrypt" | "beardog.birdsong.decrypt" => {
                self.handle_birdsong_decrypt(params, btsp_provider)
            }
            "birdsong.generate_encrypted_beacon" => {
                self.handle_generate_encrypted_beacon(params, btsp_provider)
            }
            "security.generate_jwt_secret"
            | "security.jwt_secret"
            | "beardog.generate_jwt_secret"
            | "beardog.jwt_secret" => self.handle_generate_jwt_secret(params),
            "auth.sign" => self.handle_auth_sign(params),
            _ => Err(format!("Method not found: {method}").into()),
        }
    }
}

impl SecurityHandler {
    /// Create a new `SecurityHandler` with explicit identity injection
    ///
    /// # Arguments
    ///
    /// * `identity` - Primal identity (family and node)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use beardog_tunnel::unix_socket_ipc::handlers::security::SecurityHandler;
    /// use beardog_types::primal_identity::PrimalIdentity;
    /// use std::sync::Arc;
    ///
    /// let identity = Arc::new(PrimalIdentity::for_test("nat0", "tower1"));
    /// let handler = SecurityHandler::new(identity);
    /// ```
    #[must_use]
    pub const fn new(identity: Arc<PrimalIdentity>) -> Self {
        Self { identity }
    }

    /// Handle trust evaluation request
    ///
    /// Evaluates trust level based on genetic family matching.
    /// Same family = auto-accept with limited capabilities.
    /// Different/unknown family = reject.
    fn handle_trust_evaluation(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params = params.ok_or("Missing params for trust evaluation")?;

        // Flexible parameter extraction (works with any primal's naming)
        let peer_id = params
            .get("peer_id")
            .or_else(|| params.get("id"))
            .or_else(|| params.get("peer"))
            .and_then(|v| v.as_str())
            .ok_or("Missing peer identifier")?;

        let peer_family = params
            .get("peer_family")
            .or_else(|| params.get("family"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Use injected identity (no environment variables!)
        // Following TRUE PRIMAL pattern: primal only knows itself
        let our_family = self.identity.family_id();
        let our_node = self.identity.node_id();

        // Phase 1: Dual representation with capability hints + wire decision field
        let (trust_level, trust_level_name, decision, reason, allowed_caps, denied_caps) =
            if peer_family == our_family {
                info!(
                    "✅ Trust: SAME FAMILY - level 1 (limited) - peer: {}, family: {}",
                    peer_id, peer_family
                );
                (
                    1,
                    "limited",
                    "auto_accept", // Same family = auto-accept for genetic lineage
                    "same_genetic_family",
                    vec![
                        "birdsong/*",
                        "coordination/*",
                        "health",
                        "capabilities",
                        "discovery",
                    ],
                    vec!["data/*", "commands/*", "keys/*", "federation/admin"],
                )
            } else if !peer_family.is_empty() {
                info!(
                    "⚠️  Trust: DIFFERENT FAMILY - level 0 (none) - peer: {}, their: {}, ours: {}",
                    peer_id, peer_family, our_family
                );
                (
                    0,
                    "none",
                    "reject", // Different family = reject
                    "different_family",
                    vec![],
                    vec!["*"],
                )
            } else {
                info!(
                    "⚠️  Trust: UNKNOWN FAMILY - level 0 (none) - peer: {}",
                    peer_id
                );
                (
                    0,
                    "none",
                    "reject", // Unknown family = reject
                    "unknown_family",
                    vec![],
                    vec!["*"],
                )
            };

        // Phase 1 Response: Dual representation (int + string) with capability hints + decision field
        Ok(serde_json::json!({
            "decision": decision,                  // Required by callers (wire compatibility)
            "trust_level": trust_level,           // Integer (compact, backward compat)
            "trust_level_name": trust_level_name, // String (callers expect this)
            "reason": reason,
            "peer_id": peer_id,
            "peer_family": peer_family,
            "our_family": our_family,
            "our_node": our_node,
            "evaluated_by": get_primal_name(),
            "capabilities": {                      // Capability hints (Phase 1)
                "allowed": allowed_caps,
                "denied": denied_caps,
            },
            "metadata": {
                "policy_version": 1,
                "evaluation_method": "genetic_family_match",
                "timestamp": Utc::now().to_rfc3339(),
            }
        }))
    }

    /// Handle lineage information request
    ///
    /// Returns genetic lineage information for identity verification.
    fn handle_lineage(&self) -> Result<serde_json::Value, HandlerError> {
        // Use injected identity (no environment variables!)
        let family_id = self.identity.family_id();
        let node_id = self.identity.node_id();

        // Generate encryption tag using identity helper
        let encryption_tag = self.identity.encryption_tag();

        info!(
            "🌳 Lineage info requested - family: {}, node: {}",
            family_id, node_id
        );

        Ok(serde_json::json!({
            "primal": get_primal_name(),
            "family": family_id,
            "node": node_id,
            "encryption_tag": encryption_tag,
            "generation": 0,
            "parent": null,
            "capabilities": ["security", "encryption", "trust"],
        }))
    }

    /// Derive the HMAC key for consent tokens from the family identity.
    ///
    /// Uses BLAKE3 keyed hash: `BLAKE3(family_id || ":consent-hmac-key")` to
    /// produce a deterministic 32-byte key unique per family. This means only
    /// the same `BearDog` instance (same family) can issue and verify tokens.
    fn consent_hmac_key(&self) -> [u8; 32] {
        let material = format!("{}:consent-hmac-key", self.identity.family_id());
        *blake3::hash(material.as_bytes()).as_bytes()
    }

    /// Verify an HMAC-SHA256 consent token.
    ///
    /// # Wire contract
    ///
    /// ```json
    /// { "owner_id": "alice", "scope": "vault:read:genome", "token": "<base64>" }
    /// → { "valid": true|false, "owner_id": "…", "scope": "…" }
    /// ```
    ///
    /// wetSpring calls this via Neural API `capability.call("security", "verify_consent", …)`
    /// to gate vault data access. The token is an HMAC-SHA256 over `owner_id:scope`.
    fn handle_verify_consent(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params = params.ok_or("Missing params for verify_consent")?;

        let owner_id = params
            .get("owner_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing owner_id")?;
        let scope = params
            .get("scope")
            .and_then(|v| v.as_str())
            .ok_or("Missing scope")?;
        let token_b64 = params
            .get("token")
            .and_then(|v| v.as_str())
            .ok_or("Missing token")?;

        let token_bytes = base64::engine::general_purpose::STANDARD
            .decode(token_b64)
            .map_err(|e| format!("Invalid base64 token: {e}"))?;

        let key = self.consent_hmac_key();
        let message = format!("{owner_id}:{scope}");

        let mut mac = <HmacSha256 as Mac>::new_from_slice(&key)
            .map_err(|e| format!("HMAC key error: {e}"))?;
        mac.update(message.as_bytes());

        let valid = mac.verify_slice(&token_bytes).is_ok();

        if valid {
            info!("Consent verified — owner={}, scope={}", owner_id, scope);
        } else {
            warn!("Consent rejected — owner={}, scope={}", owner_id, scope);
        }

        Ok(serde_json::json!({
            "valid": valid,
            "owner_id": owner_id,
            "scope": scope,
            "verified_at": Utc::now().to_rfc3339(),
            "verified_by": get_primal_name(),
        }))
    }

    /// Issue an HMAC-SHA256 consent token for an owner/scope pair.
    ///
    /// # Wire contract
    ///
    /// ```json
    /// { "owner_id": "alice", "scope": "vault:read:genome" }
    /// → { "token": "<base64>", "owner_id": "…", "scope": "…" }
    /// ```
    ///
    /// The returned token can later be verified via `security.verify_consent`.
    fn handle_issue_consent_token(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params = params.ok_or("Missing params for issue_consent_token")?;

        let owner_id = params
            .get("owner_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing owner_id")?;
        let scope = params
            .get("scope")
            .and_then(|v| v.as_str())
            .ok_or("Missing scope")?;

        let key = self.consent_hmac_key();
        let message = format!("{owner_id}:{scope}");

        let mut mac = <HmacSha256 as Mac>::new_from_slice(&key)
            .map_err(|e| format!("HMAC key error: {e}"))?;
        mac.update(message.as_bytes());

        let token_bytes = mac.finalize().into_bytes();
        let token_b64 = base64::engine::general_purpose::STANDARD.encode(token_bytes);

        info!("Consent token issued — owner={}, scope={}", owner_id, scope);

        Ok(serde_json::json!({
            "token": token_b64,
            "owner_id": owner_id,
            "scope": scope,
            "issued_at": Utc::now().to_rfc3339(),
            "issued_by": get_primal_name(),
        }))
    }

    /// Handle `BirdSong` encryption request
    ///
    /// Encrypts plaintext for a specific family using `BirdSong` protocol.
    fn handle_birdsong_encrypt(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params = params.ok_or("Missing params")?;
        let plaintext = params["plaintext"].as_str().ok_or("Missing plaintext")?;
        let family_id = params["family_id"].as_str().ok_or("Missing family_id")?;

        // Decode base64 plaintext
        let plaintext_bytes = base64::engine::general_purpose::STANDARD
            .decode(plaintext)
            .map_err(|e| format!("Invalid base64: {e}"))?;

        // Encrypt using BirdSong
        let ciphertext = btsp_provider
            .birdsong_manager()
            .encrypt_discovery_for_family(&plaintext_bytes, family_id)
            .map_err(|e| format!("Encryption failed: {e}"))?;

        // Encode to base64
        let ciphertext_b64 = base64::engine::general_purpose::STANDARD.encode(&ciphertext);

        Ok(serde_json::json!({
            "ciphertext": ciphertext_b64,
            "family_id": family_id,
        }))
    }

    /// Handle `BirdSong` decryption request
    ///
    /// Decrypts ciphertext from a specific family using `BirdSong` protocol.
    /// Gracefully returns success=false if not for our family (privacy).
    fn handle_birdsong_decrypt(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params = params.ok_or("Missing params")?;
        let ciphertext = params["ciphertext"].as_str().ok_or("Missing ciphertext")?;
        let family_id = params["family_id"].as_str().ok_or("Missing family_id")?;

        // Decode base64 ciphertext
        let ciphertext_bytes = base64::engine::general_purpose::STANDARD
            .decode(ciphertext)
            .map_err(|e| format!("Invalid base64: {e}"))?;

        // Decrypt using BirdSong
        match btsp_provider
            .birdsong_manager()
            .decrypt_discovery_from_family(&ciphertext_bytes, family_id)
        {
            Ok(plaintext) => {
                // Encode to base64
                let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(&plaintext);

                Ok(serde_json::json!({
                    "plaintext": plaintext_b64,
                    "family_id": family_id,
                    "success": true,
                }))
            }
            Err(e) => {
                // Graceful privacy: Not an error, just "not for us"
                debug!("🔇 Decryption failed (different family): {}", e);
                Ok(serde_json::json!({
                    "plaintext": "",
                    "family_id": family_id,
                    "success": false,
                }))
            }
        }
    }

    /// Generate a Dark Forest beacon and encrypt a discovery payload with
    /// `BirdSong` for a target family.
    ///
    /// The encrypted payload contains the public beacon ID so that only
    /// members of the target family can learn which beacon to listen for.
    /// The raw seed never leaves this primal.
    ///
    /// # Request
    ///
    /// ```json
    /// { "family_id": "nat0" }
    /// ```
    fn handle_generate_encrypted_beacon(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
        use beardog_genetics::birdsong::BeaconSeed;

        let params = params.ok_or("Missing params")?;
        let family_id = params["family_id"].as_str().ok_or("Missing family_id")?;

        let beacon = BeaconSeed::generate();
        let beacon_id_hex = beacon.id().to_hex();

        let encrypted = btsp_provider
            .birdsong_manager()
            .encrypt_discovery_for_family(beacon_id_hex.as_bytes(), family_id)
            .map_err(|e| format!("BirdSong encryption failed: {e}"))?;

        let ciphertext_b64 = base64::engine::general_purpose::STANDARD.encode(&encrypted);

        Ok(serde_json::json!({
            "beacon_id": beacon_id_hex,
            "encrypted_beacon_id": ciphertext_b64,
            "family_id": family_id,
        }))
    }

    /// Handle JWT secret generation request
    ///
    /// Generates a cryptographically secure JWT secret for authentication systems.
    /// Supports different strength levels: high (64 bytes), medium (48 bytes), low (32 bytes).
    fn handle_generate_jwt_secret(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        info!("🔐 JWT Secret Generation requested");

        let params = params.ok_or("Missing params for JWT secret generation")?;

        let purpose = params
            .get("purpose")
            .and_then(|v| v.as_str())
            .unwrap_or("authentication");

        let strength = params
            .get("strength")
            .and_then(|v| v.as_str())
            .unwrap_or("high");

        // Determine byte length based on strength
        // high: 64 bytes (512 bits) -> 88 chars base64
        // medium: 48 bytes (384 bits) -> 64 chars base64
        // low: 32 bytes (256 bits) -> 44 chars base64
        let byte_length = match strength {
            "high" => 64,
            "medium" => 48,
            "low" => 32,
            _ => 64, // Default to high security
        };

        // Generate cryptographically secure random bytes
        let mut secret_bytes = vec![0u8; byte_length];
        rand::rng().fill_bytes(&mut secret_bytes);

        // Encode to base64 for safe transmission
        let secret_b64 = base64::engine::general_purpose::STANDARD.encode(&secret_bytes);

        info!(
            "✅ JWT Secret generated - purpose: {}, strength: {}, length: {} bytes",
            purpose, strength, byte_length
        );

        Ok(serde_json::json!({
            "secret": secret_b64,
            "purpose": purpose,
            "strength": strength,
            "byte_length": byte_length,
            "encoded_length": secret_b64.len(),
            "provider": get_primal_name(),
            "generated_at": Utc::now().to_rfc3339(),
        }))
    }

    /// Handle `auth.sign` — primal-identity signing for rootPulse graph steps.
    ///
    /// Signs data using the primal's Ed25519 identity key. Accepts flexible
    /// params from graph step bindings:
    ///   - `data` (base64): raw bytes to sign
    ///   - `content_hash` (base64): hash to sign (`rootpulse_commit`)
    ///   - `dag_ref` (string): reference to sign (`rootpulse_harvest`)
    ///   - fallback: canonical JSON hash of all params
    fn handle_auth_sign(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let b64 = &base64::engine::general_purpose::STANDARD;
        let primal_name = self.identity.family_id();
        let node_id = self.identity.node_id();

        let message_bytes: Vec<u8> = if let Some(p) = params {
            if let Some(data_b64) = p.get("data").and_then(|v| v.as_str()) {
                b64.decode(data_b64)
                    .map_err(|e| format!("Invalid base64 in 'data': {e}"))?
            } else if let Some(hash_b64) = p.get("content_hash").and_then(|v| v.as_str()) {
                b64.decode(hash_b64)
                    .map_err(|e| format!("Invalid base64 in 'content_hash': {e}"))?
            } else if let Some(dag_ref) = p.get("dag_ref").and_then(|v| v.as_str()) {
                dag_ref.as_bytes().to_vec()
            } else if let Some(blob_b64) = p.get("dehydrated_blob").and_then(|v| v.as_str()) {
                b64.decode(blob_b64)
                    .map_err(|e| format!("Invalid base64 in 'dehydrated_blob': {e}"))?
            } else {
                use sha2::{Digest, Sha256};
                let canonical = serde_json::to_string(p)
                    .map_err(|e| format!("Failed to serialize params: {e}"))?;
                Sha256::digest(canonical.as_bytes()).to_vec()
            }
        } else {
            return Err("Missing params for auth.sign".into());
        };

        let (signature_b64, public_key_b64) =
            super::primal_signing::sign_with_primal_identity(primal_name, node_id, &message_bytes);

        info!(
            "auth.sign: signed {} bytes with primal identity",
            message_bytes.len()
        );

        Ok(serde_json::json!({
            "signature": signature_b64,
            "public_key": public_key_b64,
            "algorithm": "Ed25519",
            "signer": primal_name,
            "signed_bytes": message_bytes.len(),
        }))
    }
}

#[cfg(test)]
#[path = "security_tests.rs"]
mod tests;
