// SPDX-License-Identifier: AGPL-3.0-or-later

//! `enrollment.verify` handler — the thin orchestration layer.

use base64::Engine;
use serde::Deserialize;
use tracing::{info, warn};

use super::config::{
    current_unix_timestamp, load_family_seed, load_seed_generation, load_timestamp_window,
};
use super::crypto::{build_enrollment_message, compute_hmac_sha256, derive_enrollment_key};
use super::lineage::classify_lineage_distance;
use crate::unix_socket_ipc::handlers::HandlerError;
use crate::unix_socket_ipc::handlers::btsp::BtspHandler;

/// Build a rejection `EnrollmentVerifyResponse` as JSON.
fn reject(reason: String) -> Result<serde_json::Value, HandlerError> {
    let resp = beardog_types::btsp::EnrollmentVerifyResponse {
        verified: false,
        reason: Some(reason),
        verified_generation: None,
        enrollment_tier: None,
        genetic_distance: None,
    };
    serde_json::to_value(resp)
        .map_err(|e| format!("Serialize: {e}"))
        .map_err(Into::into)
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
    pub(in crate::unix_socket_ipc::handlers::btsp) fn handle_enrollment_verify(
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
            return reject(format!(
                "Timestamp outside validity window ({delta}s drift, max {window}s)"
            ));
        }

        let family_seed = load_family_seed()?;
        let current_gen = load_seed_generation();
        let requested_gen = verify_params.seed_generation;

        let generations_to_try: Vec<u32> = if requested_gen == current_gen {
            vec![current_gen]
        } else if current_gen > 0 && requested_gen == current_gen - 1 {
            vec![requested_gen]
        } else {
            warn!(
                node_id = %verify_params.node_id,
                requested_gen,
                current_gen,
                "enrollment.verify: seed generation out of range"
            );
            return reject(format!(
                "Seed generation {requested_gen} not accepted \
                 (current: {current_gen}, grace: {})",
                current_gen.saturating_sub(1)
            ));
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
                return reject("Enrollment proof already used (replay rejected)".to_string());
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
