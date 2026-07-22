// SPDX-License-Identifier: AGPL-3.0-or-later

//! `enrollment.verify` — HMAC proof verification for `mesh.enroll`.
//!
//! During `mesh.enroll`, the enrolling node computes:
//!
//! ```text
//! proof = HMAC-SHA256(family_seed, node_id || "|" || public_key || "|" || timestamp)
//! ```
//!
//! songBird forwards the structured fields to bearDog's `enrollment.verify`
//! so that proof verification happens in the crypto primal — songBird never
//! holds the `FAMILY_SEED` directly (Tower Atomic separation).

use base64::Engine;
use beardog_config::env_keys;
use serde::Deserialize;
use tracing::info;

use super::super::HandlerError;
use super::BtspHandler;

impl BtspHandler {
    /// Verify a `mesh.enroll` HMAC proof.
    ///
    /// Loads `FAMILY_SEED` from the environment, reconstructs the HMAC message
    /// from the structured fields (`node_id|public_key|timestamp`), and checks
    /// the proof using constant-time comparison.
    ///
    /// # Wire format (songBird → bearDog)
    ///
    /// ```json
    /// {
    ///   "node_id": "southGate",
    ///   "public_key": "<wg-pubkey>",
    ///   "timestamp": 1753128000,
    ///   "proof": "<base64 HMAC-SHA256>"
    /// }
    /// ```
    ///
    /// Returns `{verified: true}` or `{verified: false, reason: "..."}`.
    pub(super) async fn handle_enrollment_verify(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params_value = params.ok_or("Missing params for enrollment.verify")?;
        let verify_params = beardog_types::btsp::EnrollmentVerifyParams::deserialize(params_value)
            .map_err(|e| format!("Invalid enrollment.verify params: {e}"))?;

        let proof_bytes = base64::engine::general_purpose::STANDARD
            .decode(&verify_params.proof)
            .map_err(|e| format!("Invalid proof base64: {e}"))?;

        let family_seed = load_family_seed()?;

        let message = format!(
            "{}|{}|{}",
            verify_params.node_id, verify_params.public_key, verify_params.timestamp
        );

        let computed = compute_hmac_sha256(&family_seed, message.as_bytes());

        let verified: bool =
            subtle::ConstantTimeEq::ct_eq(computed.as_slice(), proof_bytes.as_slice()).into();

        info!(
            node_id = %verify_params.node_id,
            timestamp = verify_params.timestamp,
            verified,
            "enrollment.verify"
        );

        let resp = beardog_types::btsp::EnrollmentVerifyResponse {
            verified,
            reason: if verified {
                None
            } else {
                Some("HMAC proof does not match enrollment data".to_string())
            },
        };

        serde_json::to_value(resp)
            .map_err(|e| format!("Serialize: {e}"))
            .map_err(Into::into)
    }
}

/// Load the family seed from environment variables.
///
/// Checks `BEARDOG_FAMILY_SEED` first, then `FAMILY_SEED`.
fn load_family_seed() -> Result<Vec<u8>, HandlerError> {
    if let Ok(seed) = beardog_errors::process_env::var(env_keys::ENV_FAMILY_SEED_PREFIXED)
        && !seed.is_empty()
    {
        return Ok(seed.into_bytes());
    }
    if let Ok(seed) = beardog_errors::process_env::var(env_keys::ENV_FAMILY_SEED)
        && !seed.is_empty()
    {
        return Ok(seed.into_bytes());
    }
    Err(
        "enrollment.verify requires FAMILY_SEED or BEARDOG_FAMILY_SEED env var"
            .to_string()
            .into(),
    )
}

/// Compute HMAC-SHA256 over data with the given key.
fn compute_hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    #[expect(clippy::expect_used, reason = "HMAC-SHA256 accepts keys of any length")]
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC-SHA256 key init");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SEED: &str = "test-family-seed-for-enrollment";

    fn set_test_seed() {
        beardog_errors::process_env::set_var("FAMILY_SEED", TEST_SEED);
    }

    fn clear_test_seed() {
        beardog_errors::process_env::remove_var("FAMILY_SEED");
        beardog_errors::process_env::remove_var("BEARDOG_FAMILY_SEED");
    }

    #[test]
    fn hmac_message_format() {
        let msg = format!("{}|{}|{}", "southGate", "pubkey123", 1753128000_u64);
        assert_eq!(msg, "southGate|pubkey123|1753128000");
    }

    #[test]
    fn hmac_deterministic() {
        let key = b"test-seed";
        let data = b"southGate|pk|1234";
        let m1 = compute_hmac_sha256(key, data);
        let m2 = compute_hmac_sha256(key, data);
        assert_eq!(m1, m2);
        assert_eq!(m1.len(), 32);
    }

    #[test]
    fn hmac_varies_by_message() {
        let key = b"test-seed";
        let m1 = compute_hmac_sha256(key, b"nodeA|pk|1000");
        let m2 = compute_hmac_sha256(key, b"nodeB|pk|1000");
        assert_ne!(m1, m2);
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_valid_proof() {
        set_test_seed();
        let handler = BtspHandler::new();

        let node_id = "southGate";
        let public_key = "wg-pubkey-abc123";
        let timestamp = 1753128000_u64;

        let message = format!("{node_id}|{public_key}|{timestamp}");
        let proof = compute_hmac_sha256(TEST_SEED.as_bytes(), message.as_bytes());

        let params = serde_json::json!({
            "node_id": node_id,
            "public_key": public_key,
            "timestamp": timestamp,
            "proof": base64::engine::general_purpose::STANDARD.encode(&proof),
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["verified"], true);
        assert!(result.get("reason").is_none() || result["reason"].is_null());
        clear_test_seed();
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_invalid_proof() {
        set_test_seed();
        let handler = BtspHandler::new();

        let params = serde_json::json!({
            "node_id": "southGate",
            "public_key": "wg-pubkey-abc123",
            "timestamp": 1753128000_u64,
            "proof": base64::engine::general_purpose::STANDARD.encode(&[0u8; 32]),
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["verified"], false);
        assert!(
            result["reason"]
                .as_str()
                .unwrap()
                .contains("does not match")
        );
        clear_test_seed();
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_fails_without_seed() {
        clear_test_seed();
        let handler = BtspHandler::new();

        let params = serde_json::json!({
            "node_id": "southGate",
            "public_key": "pk",
            "timestamp": 1000_u64,
            "proof": base64::engine::general_purpose::STANDARD.encode(&[0u8; 32]),
        });

        let result = handler.handle_enrollment_verify(Some(&params)).await;
        assert!(result.is_err());
    }
}
