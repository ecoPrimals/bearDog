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
//!
//! ## Security hardening (Wave 150x)
//!
//! - **Timestamp window**: proofs are only valid within ±`BEARDOG_ENROLLMENT_TIMESTAMP_WINDOW`
//!   seconds of the current wall clock (default 300s / 5 minutes).
//! - **Replay tracking**: each verified proof digest is cached; resubmission within the
//!   window is rejected. The cache is bounded and self-pruning.

use base64::Engine;
use beardog_config::env_keys;
use parking_lot::Mutex;
use serde::Deserialize;
use std::collections::HashMap;
use tracing::{info, warn};

use super::super::HandlerError;
use super::BtspHandler;

/// Default timestamp validity window: ±300 seconds (5 minutes).
const DEFAULT_TIMESTAMP_WINDOW_SECS: u64 = 300;

/// Maximum entries in the replay cache before forced pruning.
const REPLAY_CACHE_MAX_ENTRIES: usize = 10_000;

/// Tracks seen enrollment proofs to prevent replay attacks.
///
/// Keys are BLAKE3 digests of the full proof message; values are the wall-clock
/// timestamp from the proof (used for expiry). Entries older than the timestamp
/// window are pruned on each `check_and_record` call.
pub struct ReplayCache {
    seen: Mutex<HashMap<[u8; 32], u64>>,
}

impl ReplayCache {
    pub(crate) fn new() -> Self {
        Self {
            seen: Mutex::new(HashMap::new()),
        }
    }

    /// Check whether `digest` has been seen. If not, record it. Returns `true`
    /// if this is a replay (already seen), `false` if fresh.
    fn check_and_record(&self, digest: [u8; 32], proof_timestamp: u64) -> bool {
        let window = load_timestamp_window();
        let now = current_unix_timestamp();
        let mut map = self.seen.lock();

        // Prune expired entries
        if map.len() > REPLAY_CACHE_MAX_ENTRIES / 2 {
            let cutoff = now.saturating_sub(window);
            map.retain(|_, ts| *ts > cutoff);
        }

        if map.contains_key(&digest) {
            return true;
        }

        map.insert(digest, proof_timestamp);
        false
    }
}

impl BtspHandler {
    /// Verify a `mesh.enroll` HMAC proof.
    ///
    /// Loads `FAMILY_SEED` from the environment, reconstructs the HMAC message
    /// from the structured fields (`node_id|public_key|timestamp`), and checks
    /// the proof using constant-time comparison.
    ///
    /// ## Security checks (Wave 150x)
    ///
    /// 1. **Timestamp window** — rejects proofs whose timestamp is more than
    ///    `BEARDOG_ENROLLMENT_TIMESTAMP_WINDOW` seconds from the current time.
    /// 2. **Replay rejection** — rejects proofs that have already been
    ///    successfully verified (same `node_id|public_key|timestamp|proof` tuple).
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
    ///
    /// # Errors
    ///
    /// Returns `HandlerError` on missing params, invalid base64, or missing
    /// `FAMILY_SEED` environment variable.
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

        // --- Timestamp window check ---
        let window = load_timestamp_window();
        let now = current_unix_timestamp();
        let ts = verify_params.timestamp;

        let delta = now.abs_diff(ts);
        if delta > window {
            warn!(
                node_id = %verify_params.node_id,
                timestamp = ts,
                now,
                delta,
                window,
                "enrollment.verify: timestamp outside validity window"
            );
            let resp = beardog_types::btsp::EnrollmentVerifyResponse {
                verified: false,
                reason: Some(format!(
                    "Timestamp outside validity window ({delta}s drift, max {window}s)"
                )),
            };
            return serde_json::to_value(resp)
                .map_err(|e| format!("Serialize: {e}"))
                .map_err(Into::into);
        }

        let family_seed = load_family_seed()?;

        let message = format!(
            "{}|{}|{}",
            verify_params.node_id, verify_params.public_key, verify_params.timestamp
        );

        let computed = compute_hmac_sha256(&family_seed, message.as_bytes());

        let verified: bool =
            subtle::ConstantTimeEq::ct_eq(computed.as_slice(), proof_bytes.as_slice()).into();

        // --- Replay check (only for valid proofs) ---
        if verified {
            let digest = blake3::hash(format!("{}|{}", message, verify_params.proof).as_bytes());
            if self.replay_cache.check_and_record(*digest.as_bytes(), ts) {
                warn!(
                    node_id = %verify_params.node_id,
                    timestamp = ts,
                    "enrollment.verify: replay detected"
                );
                let resp = beardog_types::btsp::EnrollmentVerifyResponse {
                    verified: false,
                    reason: Some("Enrollment proof already used (replay rejected)".to_string()),
                };
                return serde_json::to_value(resp)
                    .map_err(|e| format!("Serialize: {e}"))
                    .map_err(Into::into);
            }
        }

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

/// Load the enrollment timestamp window from env, defaulting to 300 seconds.
fn load_timestamp_window() -> u64 {
    beardog_errors::process_env::var(env_keys::ENV_ENROLLMENT_TIMESTAMP_WINDOW)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_TIMESTAMP_WINDOW_SECS)
}

/// Current wall-clock time as Unix seconds.
fn current_unix_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
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

    fn make_valid_proof(node_id: &str, public_key: &str, timestamp: u64) -> String {
        let message = format!("{node_id}|{public_key}|{timestamp}");
        let proof = compute_hmac_sha256(TEST_SEED.as_bytes(), message.as_bytes());
        base64::engine::general_purpose::STANDARD.encode(&proof)
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

    #[test]
    fn timestamp_window_rejects_old() {
        let window = 300_u64;
        let now = current_unix_timestamp();
        let old_ts = now.saturating_sub(window + 100);
        let delta = now - old_ts;
        assert!(delta > window);
    }

    #[test]
    fn timestamp_window_accepts_recent() {
        let window = 300_u64;
        let now = current_unix_timestamp();
        let recent_ts = now.saturating_sub(60);
        let delta = now - recent_ts;
        assert!(delta <= window);
    }

    #[test]
    fn timestamp_window_rejects_future() {
        let window = 300_u64;
        let now = current_unix_timestamp();
        let future_ts = now + window + 100;
        let delta = future_ts - now;
        assert!(delta > window);
    }

    #[test]
    fn replay_cache_detects_duplicate() {
        let cache = ReplayCache::new();
        let digest = [42u8; 32];
        assert!(!cache.check_and_record(digest, 1000));
        assert!(cache.check_and_record(digest, 1000));
    }

    #[test]
    fn replay_cache_allows_different_digests() {
        let cache = ReplayCache::new();
        assert!(!cache.check_and_record([1u8; 32], 1000));
        assert!(!cache.check_and_record([2u8; 32], 1000));
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_valid_proof() {
        set_test_seed();
        let handler = BtspHandler::new();

        let node_id = "southGate";
        let public_key = "wg-pubkey-abc123";
        let timestamp = current_unix_timestamp();

        let proof = make_valid_proof(node_id, public_key, timestamp);

        let params = serde_json::json!({
            "node_id": node_id,
            "public_key": public_key,
            "timestamp": timestamp,
            "proof": proof,
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
            "timestamp": current_unix_timestamp(),
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
            "timestamp": current_unix_timestamp(),
            "proof": base64::engine::general_purpose::STANDARD.encode(&[0u8; 32]),
        });

        let result = handler.handle_enrollment_verify(Some(&params)).await;
        assert!(result.is_err());
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_rejects_expired_timestamp() {
        set_test_seed();
        let handler = BtspHandler::new();

        let old_ts = current_unix_timestamp().saturating_sub(600);
        let proof = make_valid_proof("southGate", "wg-pubkey", old_ts);

        let params = serde_json::json!({
            "node_id": "southGate",
            "public_key": "wg-pubkey",
            "timestamp": old_ts,
            "proof": proof,
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
                .contains("validity window")
        );
        clear_test_seed();
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_rejects_replay() {
        set_test_seed();
        let handler = BtspHandler::new();

        let timestamp = current_unix_timestamp();
        let proof = make_valid_proof("southGate", "wg-pubkey-abc", timestamp);

        let params = serde_json::json!({
            "node_id": "southGate",
            "public_key": "wg-pubkey-abc",
            "timestamp": timestamp,
            "proof": proof,
        });

        // First attempt: should succeed
        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();
        assert_eq!(result["verified"], true);

        // Second attempt with same proof: replay rejected
        let result2 = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();
        assert_eq!(result2["verified"], false);
        assert!(result2["reason"].as_str().unwrap().contains("replay"));

        clear_test_seed();
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_rejects_far_future_timestamp() {
        set_test_seed();
        let handler = BtspHandler::new();

        let future_ts = current_unix_timestamp() + 600;
        let proof = make_valid_proof("southGate", "wg-pubkey", future_ts);

        let params = serde_json::json!({
            "node_id": "southGate",
            "public_key": "wg-pubkey",
            "timestamp": future_ts,
            "proof": proof,
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
                .contains("validity window")
        );
        clear_test_seed();
    }
}
