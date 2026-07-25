// SPDX-License-Identifier: AGPL-3.0-or-later

//! `enrollment.verify` — two-layer genetic enrollment verification.
//!
//! ## Two-layer genetic model
//!
//! Enrollment mirrors the biological two-DNA system:
//!
//! 1. **Mitochondrial gate** — shared `FAMILY_SEED` HMAC proves the enrollee
//!    can "hear the birdsong" (same family). This is the mito-beacon layer:
//!    if you don't share the family seed, you can't even produce a valid proof.
//!
//! 2. **Nuclear lineage distance** — when a `lineage_proof` is attached, the
//!    verifier computes genetic distance (tree hops) and classifies the
//!    enrollee into a trust tier (`identity`, `kin`, `sibling`, `extended`,
//!    `distant`). Like nuclear DNA, this determines authority and permissions.
//!
//! ```text
//! enrollment_key(gen) = HKDF-SHA256(
//!     ikm  = FAMILY_SEED,
//!     salt = FAMILY_ID (or "default"),
//!     info = "enrollment-v{gen}"
//! )
//! proof = HMAC-SHA256(enrollment_key(gen), node_id|public_key|timestamp|gen)
//! ```
//!
//! ## Seed rotation
//!
//! - `BEARDOG_ENROLLMENT_SEED_GENERATION` sets the current generation (default 0).
//! - Grace period accepts N and N−1 (same as `BirdSong` key rotation).
//!
//! ## Security hardening
//!
//! - **Timestamp window**: ±`BEARDOG_ENROLLMENT_TIMESTAMP_WINDOW` seconds.
//! - **Replay tracking**: BLAKE3-digest dedup cache, bounded and self-pruning.

use base64::Engine;
use beardog_config::env_keys;
use beardog_genetics::birdsong::lineage_proof::LineageProofManager;
use hkdf::Hkdf;
use parking_lot::Mutex;
use serde::Deserialize;
use sha2::Sha256;
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

/// Derive an enrollment HMAC key from the family seed via HKDF.
///
/// This mirrors `LineageKeyDerivation::derive_key` — same HKDF-SHA256 pattern,
/// generation in the info string, `FAMILY_ID` as salt for family scoping.
///
/// ```text
/// enrollment_key = HKDF-SHA256(
///     ikm  = family_seed,
///     salt = family_id_bytes,
///     info = "enrollment-v{generation}"
/// )
/// ```
fn derive_enrollment_key(family_seed: &[u8], generation: u32) -> [u8; 32] {
    let family_id = beardog_errors::process_env::var(env_keys::ENV_FAMILY_ID)
        .unwrap_or_else(|_| "default".to_string());

    let info = format!("enrollment-v{generation}");
    let hk = Hkdf::<Sha256>::new(Some(family_id.as_bytes()), family_seed);
    let mut key = [0u8; 32];
    #[expect(
        clippy::expect_used,
        reason = "HKDF-SHA256 expand to 32 bytes is infallible"
    )]
    hk.expand(info.as_bytes(), &mut key)
        .expect("HKDF expand 32 bytes");
    key
}

/// Build the enrollment HMAC message including seed generation.
fn build_enrollment_message(
    node_id: &str,
    public_key: &str,
    timestamp: u64,
    generation: u32,
) -> String {
    format!("{node_id}|{public_key}|{timestamp}|{generation}")
}

impl BtspHandler {
    /// Verify a `mesh.enroll` HMAC proof.
    ///
    /// Derives the enrollment HMAC key from `FAMILY_SEED` through the genetic
    /// HKDF hierarchy at the requested `seed_generation`, then verifies the
    /// proof using constant-time comparison.
    ///
    /// ## Seed rotation grace period
    ///
    /// If the proof's `seed_generation` matches the current generation, it is
    /// verified directly. If the current generation is > 0, the verifier also
    /// accepts generation N−1 (grace period for nodes that haven't rotated yet).
    ///
    /// ## Security checks (Wave 150x)
    ///
    /// 1. **Timestamp window** — rejects proofs whose timestamp is more than
    ///    `BEARDOG_ENROLLMENT_TIMESTAMP_WINDOW` seconds from the current time.
    /// 2. **Replay rejection** — rejects proofs that have already been
    ///    successfully verified (same digest).
    ///
    /// # Wire format (songBird → bearDog)
    ///
    /// ```json
    /// {
    ///   "node_id": "southGate",
    ///   "public_key": "<wg-pubkey>",
    ///   "timestamp": 1753128000,
    ///   "proof": "<base64 HMAC-SHA256>",
    ///   "seed_generation": 0
    /// }
    /// ```
    ///
    /// Returns `{verified: true, verified_generation: N}` or
    /// `{verified: false, reason: "..."}`.
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
                verified_generation: None,
                enrollment_tier: None,
                genetic_distance: None,
            };
            return serde_json::to_value(resp)
                .map_err(|e| format!("Serialize: {e}"))
                .map_err(Into::into);
        }

        let family_seed = load_family_seed()?;
        let current_gen = load_seed_generation();
        let requested_gen = verify_params.seed_generation;

        // Determine which generations to try: current, and N−1 during grace period.
        let generations_to_try: Vec<u32> = if requested_gen == current_gen {
            vec![current_gen]
        } else if current_gen > 0 && requested_gen == current_gen - 1 {
            // Grace period: accept previous generation
            vec![requested_gen]
        } else {
            // Reject: generation too old or too new
            warn!(
                node_id = %verify_params.node_id,
                requested_gen,
                current_gen,
                "enrollment.verify: seed generation out of range"
            );
            let resp = beardog_types::btsp::EnrollmentVerifyResponse {
                verified: false,
                reason: Some(format!(
                    "Seed generation {requested_gen} not accepted \
                     (current: {current_gen}, grace: {})",
                    current_gen.saturating_sub(1)
                )),
                verified_generation: None,
                enrollment_tier: None,
                genetic_distance: None,
            };
            return serde_json::to_value(resp)
                .map_err(|e| format!("Serialize: {e}"))
                .map_err(Into::into);
        };

        // Try each allowed generation
        let mut verified = false;
        let mut matched_gen = 0u32;

        for candidate_gen in &generations_to_try {
            let enrollment_key = derive_enrollment_key(&family_seed, *candidate_gen);
            let message = build_enrollment_message(
                &verify_params.node_id,
                &verify_params.public_key,
                verify_params.timestamp,
                *candidate_gen,
            );
            let computed = compute_hmac_sha256(&enrollment_key, message.as_bytes());
            let ok: bool =
                subtle::ConstantTimeEq::ct_eq(computed.as_slice(), proof_bytes.as_slice()).into();
            if ok {
                verified = true;
                matched_gen = *candidate_gen;
                break;
            }
        }

        // --- Replay check (only for valid proofs) ---
        if verified {
            let replay_input = format!(
                "{}|{}|{}|{}|{}",
                verify_params.node_id,
                verify_params.public_key,
                verify_params.timestamp,
                matched_gen,
                verify_params.proof,
            );
            let digest = blake3::hash(replay_input.as_bytes());
            if self.replay_cache.check_and_record(*digest.as_bytes(), ts) {
                warn!(
                    node_id = %verify_params.node_id,
                    timestamp = ts,
                    "enrollment.verify: replay detected"
                );
                let resp = beardog_types::btsp::EnrollmentVerifyResponse {
                    verified: false,
                    reason: Some("Enrollment proof already used (replay rejected)".to_string()),
                    verified_generation: None,
                    enrollment_tier: None,
                    genetic_distance: None,
                };
                return serde_json::to_value(resp)
                    .map_err(|e| format!("Serialize: {e}"))
                    .map_err(Into::into);
            }
        }

        // --- Nuclear lineage distance (when proof provided) ---
        let (enrollment_tier, distance) = if verified {
            if let Some(ref lp) = verify_params.lineage_proof {
                classify_lineage_distance(&verify_params.node_id, lp)
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        info!(
            node_id = %verify_params.node_id,
            timestamp = verify_params.timestamp,
            seed_generation = if verified { matched_gen } else { requested_gen },
            ?enrollment_tier,
            ?distance,
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
            verified_generation: if verified { Some(matched_gen) } else { None },
            enrollment_tier,
            genetic_distance: distance,
        };

        serde_json::to_value(resp)
            .map_err(|e| format!("Serialize: {e}"))
            .map_err(Into::into)
    }
}

/// Classify the nuclear lineage distance from an enrollment proof.
///
/// The enrollee's `lineage_proof.path` encodes their depth from root.
/// The verifier's own node ID is resolved from `BEARDOG_PRIMAL_NAME` / `NODE_ID`.
///
/// If the verifier is in the same chain, we compute true genetic distance:
/// `depth(enrollee) + depth(verifier) - 2 * depth(common_ancestor)`.
///
/// If the verifier's node is not in the enrollee's path (different branch
/// or verifier doesn't participate in chain proofs), we fall back to
/// classifying the enrollee's **depth from root** — still useful for
/// trust tiering since depth 1 (root's child) has more authority than depth 5.
fn classify_lineage_distance(
    enrollee_node_id: &str,
    proof: &beardog_types::btsp::EnrollmentLineageProof,
) -> (Option<String>, Option<u32>) {
    if proof.path.is_empty() {
        return (None, None);
    }

    // Enrollee's depth from root
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Lineage path lengths fit u32"
    )]
    let enrollee_depth = (proof.path.len() - 1) as u32;

    // Check the enrollee is actually the last node in their own path
    if proof.path.last().map(String::as_str) != Some(enrollee_node_id) {
        return (None, None);
    }

    // Try to find verifier's node in the same path (same branch → direct lineage)
    let verifier_id = beardog_errors::process_env::var(env_keys::ENV_NODE_ID_PREFIXED)
        .or_else(|_| beardog_errors::process_env::var(env_keys::ENV_NODE_ID))
        .or_else(|_| beardog_errors::process_env::var(env_keys::ENV_PRIMAL_NAME_PREFIXED))
        .or_else(|_| beardog_errors::process_env::var(env_keys::ENV_PRIMAL_NAME))
        .unwrap_or_default();

    let distance = if verifier_id.is_empty() {
        // No verifier identity → use raw depth as distance proxy
        enrollee_depth
    } else if verifier_id == enrollee_node_id {
        0
    } else if let Some(verifier_pos) = proof.path.iter().position(|n| n == &verifier_id) {
        // Verifier is an ancestor of the enrollee → distance is depth difference
        #[expect(clippy::cast_possible_truncation, reason = "Position in path fits u32")]
        let verifier_depth = verifier_pos as u32;
        enrollee_depth - verifier_depth
    } else {
        // Verifier not in enrollee's path — use enrollee depth as upper bound
        // (true distance would need verifier's own path + common ancestor lookup)
        enrollee_depth
    };

    let tier = LineageProofManager::classify_enrollment_tier(distance);
    (Some(tier.wire_name().to_string()), Some(distance))
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

/// Load the current enrollment seed generation from env (default 0).
fn load_seed_generation() -> u32 {
    beardog_errors::process_env::var(env_keys::ENV_ENROLLMENT_SEED_GENERATION)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
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

    fn clear_test_env() {
        beardog_errors::process_env::remove_var("FAMILY_SEED");
        beardog_errors::process_env::remove_var("BEARDOG_FAMILY_SEED");
        beardog_errors::process_env::remove_var("BEARDOG_ENROLLMENT_SEED_GENERATION");
        beardog_errors::process_env::remove_var("FAMILY_ID");
    }

    fn make_valid_proof_gen(
        node_id: &str,
        public_key: &str,
        timestamp: u64,
        generation: u32,
    ) -> String {
        let enrollment_key = derive_enrollment_key(TEST_SEED.as_bytes(), generation);
        let message = build_enrollment_message(node_id, public_key, timestamp, generation);
        let proof = compute_hmac_sha256(&enrollment_key, message.as_bytes());
        base64::engine::general_purpose::STANDARD.encode(&proof)
    }

    fn make_valid_proof(node_id: &str, public_key: &str, timestamp: u64) -> String {
        make_valid_proof_gen(node_id, public_key, timestamp, 0)
    }

    // --- Unit tests for derivation ---

    #[test]
    fn enrollment_key_deterministic() {
        let k1 = derive_enrollment_key(b"seed", 0);
        let k2 = derive_enrollment_key(b"seed", 0);
        assert_eq!(k1, k2);
    }

    #[test]
    fn enrollment_key_varies_by_generation() {
        let k0 = derive_enrollment_key(b"seed", 0);
        let k1 = derive_enrollment_key(b"seed", 1);
        assert_ne!(k0, k1);
    }

    #[test]
    fn enrollment_key_varies_by_seed() {
        let ka = derive_enrollment_key(b"seed-a", 0);
        let kb = derive_enrollment_key(b"seed-b", 0);
        assert_ne!(ka, kb);
    }

    #[test]
    fn enrollment_message_includes_generation() {
        let msg = build_enrollment_message("node", "pk", 1000, 3);
        assert_eq!(msg, "node|pk|1000|3");
    }

    #[test]
    fn hmac_deterministic() {
        let key = b"test-seed";
        let data = b"southGate|pk|1234|0";
        let m1 = compute_hmac_sha256(key, data);
        let m2 = compute_hmac_sha256(key, data);
        assert_eq!(m1, m2);
        assert_eq!(m1.len(), 32);
    }

    #[test]
    fn hmac_varies_by_message() {
        let key = b"test-seed";
        let m1 = compute_hmac_sha256(key, b"nodeA|pk|1000|0");
        let m2 = compute_hmac_sha256(key, b"nodeB|pk|1000|0");
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

    // --- Integration tests (serial — env manipulation) ---

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_valid_proof_gen0() {
        set_test_seed();
        let handler = BtspHandler::new();

        let timestamp = current_unix_timestamp();
        let proof = make_valid_proof("southGate", "wg-pubkey-abc123", timestamp);

        let params = serde_json::json!({
            "node_id": "southGate",
            "public_key": "wg-pubkey-abc123",
            "timestamp": timestamp,
            "proof": proof,
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["verified"], true);
        assert_eq!(result["verified_generation"], 0);
        clear_test_env();
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_valid_proof_gen1() {
        set_test_seed();
        beardog_errors::process_env::set_var("BEARDOG_ENROLLMENT_SEED_GENERATION", "1");
        let handler = BtspHandler::new();

        let timestamp = current_unix_timestamp();
        let proof = make_valid_proof_gen("southGate", "wg-pubkey", timestamp, 1);

        let params = serde_json::json!({
            "node_id": "southGate",
            "public_key": "wg-pubkey",
            "timestamp": timestamp,
            "proof": proof,
            "seed_generation": 1,
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["verified"], true);
        assert_eq!(result["verified_generation"], 1);
        clear_test_env();
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_grace_period_accepts_previous_gen() {
        set_test_seed();
        beardog_errors::process_env::set_var("BEARDOG_ENROLLMENT_SEED_GENERATION", "2");
        let handler = BtspHandler::new();

        let timestamp = current_unix_timestamp();
        // Proof made with gen 1 (previous)
        let proof = make_valid_proof_gen("southGate", "wg-pubkey", timestamp, 1);

        let params = serde_json::json!({
            "node_id": "southGate",
            "public_key": "wg-pubkey",
            "timestamp": timestamp,
            "proof": proof,
            "seed_generation": 1,
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["verified"], true);
        assert_eq!(result["verified_generation"], 1);
        clear_test_env();
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_rejects_stale_generation() {
        set_test_seed();
        beardog_errors::process_env::set_var("BEARDOG_ENROLLMENT_SEED_GENERATION", "5");
        let handler = BtspHandler::new();

        let timestamp = current_unix_timestamp();
        // Proof made with gen 2 (too old — current is 5, grace is 4)
        let proof = make_valid_proof_gen("southGate", "wg-pubkey", timestamp, 2);

        let params = serde_json::json!({
            "node_id": "southGate",
            "public_key": "wg-pubkey",
            "timestamp": timestamp,
            "proof": proof,
            "seed_generation": 2,
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["verified"], false);
        assert!(result["reason"].as_str().unwrap().contains("not accepted"));
        clear_test_env();
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
        clear_test_env();
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_fails_without_seed() {
        clear_test_env();
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
        clear_test_env();
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

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();
        assert_eq!(result["verified"], true);

        let result2 = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();
        assert_eq!(result2["verified"], false);
        assert!(result2["reason"].as_str().unwrap().contains("replay"));

        clear_test_env();
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
        clear_test_env();
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn cross_gen_proof_rejected_without_grace() {
        set_test_seed();
        // Current gen is 0 (default), proof uses gen 1 → rejected
        let handler = BtspHandler::new();

        let timestamp = current_unix_timestamp();
        let proof = make_valid_proof_gen("southGate", "wg-pubkey", timestamp, 1);

        let params = serde_json::json!({
            "node_id": "southGate",
            "public_key": "wg-pubkey",
            "timestamp": timestamp,
            "proof": proof,
            "seed_generation": 1,
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["verified"], false);
        assert!(result["reason"].as_str().unwrap().contains("not accepted"));
        clear_test_env();
    }

    // --- Nuclear lineage distance tests ---

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_with_lineage_proof_returns_tier() {
        set_test_seed();
        beardog_errors::process_env::set_var("BEARDOG_NODE_ID", "root");
        let handler = BtspHandler::new();

        let timestamp = current_unix_timestamp();
        let proof = make_valid_proof("child-1", "wg-pubkey", timestamp);

        let params = serde_json::json!({
            "node_id": "child-1",
            "public_key": "wg-pubkey",
            "timestamp": timestamp,
            "proof": proof,
            "lineage_proof": {
                "chain_id": "test-chain",
                "path": ["root", "child-1"],
                "generation": 0
            }
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["verified"], true);
        // Verifier is "root", enrollee is "child-1" in path [root, child-1]
        // Verifier is at position 0 (depth 0), enrollee at depth 1 → distance 1 → "kin"
        assert_eq!(result["enrollment_tier"], "kin");
        assert_eq!(result["genetic_distance"], 1);
        clear_test_env();
        beardog_errors::process_env::remove_var("BEARDOG_NODE_ID");
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_without_lineage_proof_omits_tier() {
        set_test_seed();
        let handler = BtspHandler::new();

        let timestamp = current_unix_timestamp();
        let proof = make_valid_proof("southGate", "wg-pubkey", timestamp);

        let params = serde_json::json!({
            "node_id": "southGate",
            "public_key": "wg-pubkey",
            "timestamp": timestamp,
            "proof": proof,
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["verified"], true);
        assert!(result.get("enrollment_tier").is_none() || result["enrollment_tier"].is_null());
        assert!(result.get("genetic_distance").is_none() || result["genetic_distance"].is_null());
        clear_test_env();
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_lineage_proof_sibling_distance() {
        set_test_seed();
        beardog_errors::process_env::set_var("BEARDOG_NODE_ID", "child-1");
        let handler = BtspHandler::new();

        let timestamp = current_unix_timestamp();
        let proof = make_valid_proof("child-2", "wg-pubkey", timestamp);

        // child-2's path: [root, child-2]. Verifier is "child-1" which is NOT
        // in child-2's path, so distance falls back to enrollee's depth (1)
        let params = serde_json::json!({
            "node_id": "child-2",
            "public_key": "wg-pubkey",
            "timestamp": timestamp,
            "proof": proof,
            "lineage_proof": {
                "chain_id": "test-chain",
                "path": ["root", "child-2"],
                "generation": 0
            }
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["verified"], true);
        // Verifier not in path → fallback to enrollee depth = 1 → "kin"
        assert_eq!(result["enrollment_tier"], "kin");
        assert_eq!(result["genetic_distance"], 1);
        clear_test_env();
        beardog_errors::process_env::remove_var("BEARDOG_NODE_ID");
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_lineage_proof_deep_enrollee() {
        set_test_seed();
        beardog_errors::process_env::set_var("BEARDOG_NODE_ID", "root");
        let handler = BtspHandler::new();

        let timestamp = current_unix_timestamp();
        let proof = make_valid_proof("gc-3", "wg-pubkey", timestamp);

        // Deep enrollee: root → child-1 → gc-1 → gc-2 → gc-3 (depth 4)
        // Verifier at root (pos 0), enrollee depth 4 → distance 4 → "extended"
        let params = serde_json::json!({
            "node_id": "gc-3",
            "public_key": "wg-pubkey",
            "timestamp": timestamp,
            "proof": proof,
            "lineage_proof": {
                "chain_id": "test-chain",
                "path": ["root", "child-1", "gc-1", "gc-2", "gc-3"],
                "generation": 0
            }
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["verified"], true);
        assert_eq!(result["enrollment_tier"], "extended");
        assert_eq!(result["genetic_distance"], 4);
        clear_test_env();
        beardog_errors::process_env::remove_var("BEARDOG_NODE_ID");
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_lineage_proof_distant_enrollee() {
        set_test_seed();
        beardog_errors::process_env::set_var("BEARDOG_NODE_ID", "root");
        let handler = BtspHandler::new();

        let timestamp = current_unix_timestamp();
        let proof = make_valid_proof("far-node", "wg-pubkey", timestamp);

        // Very deep: 6 hops from root → distance 6 → "distant"
        let params = serde_json::json!({
            "node_id": "far-node",
            "public_key": "wg-pubkey",
            "timestamp": timestamp,
            "proof": proof,
            "lineage_proof": {
                "chain_id": "test-chain",
                "path": ["root", "a", "b", "c", "d", "e", "far-node"],
                "generation": 0
            }
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["verified"], true);
        assert_eq!(result["enrollment_tier"], "distant");
        assert_eq!(result["genetic_distance"], 6);
        clear_test_env();
        beardog_errors::process_env::remove_var("BEARDOG_NODE_ID");
    }

    #[serial_test::serial]
    #[tokio::test]
    async fn verify_lineage_proof_self_enrollment() {
        set_test_seed();
        beardog_errors::process_env::set_var("BEARDOG_NODE_ID", "southGate");
        let handler = BtspHandler::new();

        let timestamp = current_unix_timestamp();
        let proof = make_valid_proof("southGate", "wg-pubkey", timestamp);

        let params = serde_json::json!({
            "node_id": "southGate",
            "public_key": "wg-pubkey",
            "timestamp": timestamp,
            "proof": proof,
            "lineage_proof": {
                "chain_id": "test-chain",
                "path": ["root", "southGate"],
                "generation": 0
            }
        });

        let result = handler
            .handle_enrollment_verify(Some(&params))
            .await
            .unwrap();

        assert_eq!(result["verified"], true);
        // node_id == verifier_id → distance 0 → "identity"
        assert_eq!(result["enrollment_tier"], "identity");
        assert_eq!(result["genetic_distance"], 0);
        clear_test_env();
        beardog_errors::process_env::remove_var("BEARDOG_NODE_ID");
    }
}
