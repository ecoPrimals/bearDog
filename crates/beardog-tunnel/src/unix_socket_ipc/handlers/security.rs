// SPDX-License-Identifier: AGPL-3.0-only

//! Security and trust evaluation handlers
//!
//! Provides core security capabilities including:
//! - Trust evaluation based on genetic family lineage
//! - Lineage information for identity verification
//! - `BirdSong` encryption/decryption for secure discovery
//! - JWT secret generation for authentication systems

use super::MethodHandler;
use super::utils::get_primal_name;
use crate::btsp_provider::BeardogBtspProvider;
use async_trait::async_trait;
use base64::Engine;
use beardog_types::primal_identity::PrimalIdentity;
use chrono::Utc;
use std::sync::Arc;
use tracing::{debug, info};

/// Handler for security and trust methods
///
/// Supports:
/// - `security.evaluate` / `trust.evaluate` - Evaluate trust level based on genetic family
/// - `security.lineage` / `trust.lineage` - Get genetic lineage information
/// - `birdsong.encrypt` / `birdsong.decrypt` - `BirdSong` encryption for secure discovery
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

#[async_trait]
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
            // BirdSong encryption (semantic domain.operation first; beardog.* = backward compat)
            "birdsong.encrypt",
            "beardog.birdsong.encrypt",
            "birdsong.decrypt",
            "beardog.birdsong.decrypt",
            // JWT secret generation (semantic security.* first; beardog.* = backward compat)
            "security.generate_jwt_secret",
            "security.jwt_secret",
            "beardog.generate_jwt_secret",
            "beardog.jwt_secret",
        ]
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        match method {
            "security.evaluate"
            | "trust.evaluate"
            | "security.evaluate_trust"
            | "trust.evaluate_peer" => self.handle_trust_evaluation(params).await,
            "security.lineage" | "trust.lineage" | "security.get_lineage" | "trust.get_lineage" => {
                self.handle_lineage().await
            }
            "birdsong.encrypt" | "beardog.birdsong.encrypt" => {
                self.handle_birdsong_encrypt(params, btsp_provider).await
            }
            "birdsong.decrypt" | "beardog.birdsong.decrypt" => {
                self.handle_birdsong_decrypt(params, btsp_provider).await
            }
            "security.generate_jwt_secret"
            | "security.jwt_secret"
            | "beardog.generate_jwt_secret"
            | "beardog.jwt_secret" => self.handle_generate_jwt_secret(params).await,
            _ => Err(format!("Method not found: {method}")),
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
    pub const fn new(identity: Arc<PrimalIdentity>) -> Self {
        Self { identity }
    }

    /// Handle trust evaluation request
    ///
    /// Evaluates trust level based on genetic family matching.
    /// Same family = auto-accept with limited capabilities.
    /// Different/unknown family = reject.
    async fn handle_trust_evaluation(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
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
    async fn handle_lineage(&self) -> Result<serde_json::Value, String> {
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

    /// Handle `BirdSong` encryption request
    ///
    /// Encrypts plaintext for a specific family using `BirdSong` protocol.
    async fn handle_birdsong_encrypt(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
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
    async fn handle_birdsong_decrypt(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
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

    /// Handle JWT secret generation request
    ///
    /// Generates a cryptographically secure JWT secret for authentication systems.
    /// Supports different strength levels: high (64 bytes), medium (48 bytes), low (32 bytes).
    async fn handle_generate_jwt_secret(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
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
        use rand::RngCore;
        let mut secret_bytes = vec![0u8; byte_length];
        rand::thread_rng()
            .try_fill_bytes(&mut secret_bytes)
            .map_err(|e| format!("Failed to generate random bytes: {e}"))?;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_handler_methods() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = SecurityHandler::new(identity);
        let methods = handler.methods();

        // Should support trust evaluation
        assert!(methods.contains(&"security.evaluate"));
        assert!(methods.contains(&"trust.evaluate"));

        // Should support lineage
        assert!(methods.contains(&"security.lineage"));
        assert!(methods.contains(&"trust.lineage"));

        // Should support BirdSong
        assert!(methods.contains(&"birdsong.encrypt"));
        assert!(methods.contains(&"birdsong.decrypt"));

        // Should support JWT secrets (semantic names primary in list order)
        assert!(methods.contains(&"security.generate_jwt_secret"));
        assert!(methods.contains(&"security.jwt_secret"));
        assert!(methods.contains(&"beardog.generate_jwt_secret"));
        assert!(methods.contains(&"beardog.jwt_secret"));

        let i_bird = methods
            .iter()
            .position(|&m| m == "birdsong.encrypt")
            .expect("birdsong.encrypt");
        let i_old_bird = methods
            .iter()
            .position(|&m| m == "beardog.birdsong.encrypt")
            .expect("beardog.birdsong.encrypt");
        assert!(i_bird < i_old_bird);

        let i_sec = methods
            .iter()
            .position(|&m| m == "security.generate_jwt_secret")
            .expect("security.generate_jwt_secret");
        let i_bd_jwt = methods
            .iter()
            .position(|&m| m == "beardog.generate_jwt_secret")
            .expect("beardog.generate_jwt_secret");
        assert!(i_sec < i_bd_jwt);
    }

    #[tokio::test]
    async fn test_trust_evaluation_same_family() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = SecurityHandler::new(identity);

        // Set environment for testing
        beardog_errors::process_env::set_var("FAMILY_ID", "test-family");
        beardog_errors::process_env::set_var("NODE_ID", "test-node");

        let params = serde_json::json!({
            "peer_id": "peer-123",
            "peer_family": "test-family"
        });

        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
        let result = handler
            .handle("security.evaluate", Some(&params), &btsp_provider)
            .await;

        assert!(result.is_ok());
        let response = result.expect("trust evaluation should succeed in test");

        assert_eq!(response["decision"], "auto_accept");
        assert_eq!(response["trust_level"], 1);
        assert_eq!(response["trust_level_name"], "limited");
        assert_eq!(response["reason"], "same_genetic_family");
    }

    #[tokio::test]
    async fn test_trust_evaluation_different_family() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = SecurityHandler::new(identity);

        beardog_errors::process_env::set_var("FAMILY_ID", "our-family");

        let params = serde_json::json!({
            "peer_id": "peer-456",
            "peer_family": "other-family"
        });

        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
        let result = handler
            .handle("trust.evaluate", Some(&params), &btsp_provider)
            .await;

        assert!(result.is_ok());
        let response = result.expect("trust evaluation should succeed in test");

        assert_eq!(response["decision"], "reject");
        assert_eq!(response["trust_level"], 0);
        assert_eq!(response["reason"], "different_family");
    }

    #[tokio::test]
    async fn test_lineage_information() {
        struct CleanupGuard;
        impl Drop for CleanupGuard {
            fn drop(&mut self) {
                beardog_errors::process_env::remove_var("PRIMAL_NAME");
            }
        }
        beardog_errors::process_env::set_var("PRIMAL_NAME", "beardog");
        let _guard = CleanupGuard;

        let identity = Arc::new(PrimalIdentity::for_test("lineage-family", "lineage-node"));
        let handler = SecurityHandler::new(identity);

        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
        let result = handler
            .handle("security.lineage", None, &btsp_provider)
            .await;

        assert!(result.is_ok());
        let response = result.expect("lineage info should succeed in test");

        assert_eq!(response["primal"], "beardog");
        assert_eq!(response["family"], "lineage-family");
        assert_eq!(response["node"], "lineage-node");
        assert!(response["encryption_tag"].is_string());
    }

    #[tokio::test]
    async fn test_jwt_secret_generation() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = SecurityHandler::new(identity);

        let params = serde_json::json!({
            "purpose": "testing",
            "strength": "high"
        });

        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
        let result = handler
            .handle(
                "security.generate_jwt_secret",
                Some(&params),
                &btsp_provider,
            )
            .await;

        assert!(result.is_ok());
        let response = result.expect("JWT secret generation should succeed in test");

        assert!(response["secret"].is_string());
        assert_eq!(response["purpose"], "testing");
        assert_eq!(response["strength"], "high");
        assert_eq!(response["byte_length"], 64);
        assert!(
            response["encoded_length"]
                .as_u64()
                .expect("encoded_length should be number")
                >= 88
        );

        // Verify secret is valid base64
        let secret = response["secret"]
            .as_str()
            .expect("secret should be string");
        assert!(
            base64::engine::general_purpose::STANDARD
                .decode(secret)
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_jwt_secret_read_alias_matches_generate() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = SecurityHandler::new(identity);
        let params = serde_json::json!({ "strength": "low" });
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
        let via_read = handler
            .handle("security.jwt_secret", Some(&params), &btsp_provider)
            .await
            .expect("security.jwt_secret");
        let via_legacy = handler
            .handle("beardog.jwt_secret", Some(&params), &btsp_provider)
            .await
            .expect("beardog.jwt_secret");
        assert_eq!(via_read["byte_length"], via_legacy["byte_length"]);
        assert_eq!(via_read["strength"], via_legacy["strength"]);
    }

    #[tokio::test]
    async fn test_birdsong_encrypt_semantic_alias_requires_params() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = SecurityHandler::new(identity);
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
        let e1 = handler
            .handle("birdsong.encrypt", None, &btsp_provider)
            .await
            .expect_err("missing params");
        let e2 = handler
            .handle("beardog.birdsong.encrypt", None, &btsp_provider)
            .await
            .expect_err("missing params");
        assert_eq!(e1, e2);
    }

    #[tokio::test]
    async fn test_jwt_secret_different_strengths() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = SecurityHandler::new(identity);
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        // Test high strength
        let params = serde_json::json!({"strength": "high"});
        let result = handler
            .handle(
                "security.generate_jwt_secret",
                Some(&params),
                &btsp_provider,
            )
            .await
            .expect("JWT high strength should succeed");
        assert_eq!(result["byte_length"], 64);

        // Test medium strength
        let params = serde_json::json!({"strength": "medium"});
        let result = handler
            .handle(
                "security.generate_jwt_secret",
                Some(&params),
                &btsp_provider,
            )
            .await
            .expect("JWT medium strength should succeed");
        assert_eq!(result["byte_length"], 48);

        // Test low strength
        let params = serde_json::json!({"strength": "low"});
        let result = handler
            .handle(
                "security.generate_jwt_secret",
                Some(&params),
                &btsp_provider,
            )
            .await
            .expect("JWT low strength should succeed");
        assert_eq!(result["byte_length"], 32);
    }
}
