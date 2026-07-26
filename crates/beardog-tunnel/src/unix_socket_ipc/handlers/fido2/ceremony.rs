// SPDX-License-Identifier: AGPL-3.0-or-later

//! `beardog.fido2.ceremony` — multi-tap entropy ceremony with timing capture.

use serde_json::Value;
#[cfg(feature = "ctap2")]
use serde_json::json;
#[cfg(feature = "ctap2")]
use tracing::info;

/// Tap-sequence entropy ceremony — harvest multi-source entropy from
/// repeated FIDO2 `GetAssertion` calls with timing capture.
///
/// Each tap produces: OS-RNG challenge (Tier 1), hardware RNG signature nonce
/// (Tier 2), and human motor timing jitter (Tier 3). All sources are mixed
/// via BLAKE3 keyed hash to produce a single 32-byte entropy output.
///
/// # Parameters
///
/// - `rp_id` (string, required): relying party identifier
/// - `credential_id` (string, required): base64-encoded credential ID from prior registration
/// - `tap_count` (u64, optional, default 5): number of taps to request (1..=20)
/// - `purpose` (string, optional, default `"entropy_harvest"`): label for audit trail
/// - `device_path` (string, optional): HID device path
///
/// # Returns
///
/// ```json
/// {
///   "entropy": "<base64, 32 bytes>",
///   "taps_completed": 5,
///   "taps_requested": 5,
///   "tier": 3,
///   "timing_summary": {
///     "total_duration_ms": 8432,
///     "mean_reaction_ms": 340,
///     "reaction_jitter_ms": 127,
///     "inter_tap_intervals_ms": [1200, 980, 1650, 2100],
///     "timing_entropy_bits_estimate": 24
///   },
///   "sources": ["os_rng", "fido2_hardware", "human_temporal"],
///   "purpose": "loam_seed"
/// }
/// ```
pub async fn handle_fido2_ceremony(params: Option<&Value>) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for beardog.fido2.ceremony")?;

    let rp_id = params
        .get("rp_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: rp_id")?;

    let credential_id = params
        .get("credential_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: credential_id")?;

    let tap_count = usize::try_from(
        params
            .get("tap_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(5),
    )
    .unwrap_or(5);

    let purpose = params
        .get("purpose")
        .and_then(|v| v.as_str())
        .unwrap_or("entropy_harvest");

    let _device_path = params.get("device_path").and_then(|v| v.as_str());

    #[cfg(feature = "ctap2")]
    {
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD as BASE64;

        let device_path = super::helpers::resolve_device_path(_device_path).await?;

        let credential_id_bytes = BASE64
            .decode(credential_id)
            .map_err(|e| format!("Invalid base64 credential_id: {e}"))?;

        let provider = super::helpers::create_ctap2_provider(&device_path, rp_id).await?;

        info!(
            rp_id,
            tap_count, purpose, "Starting tap-sequence entropy ceremony"
        );

        let result = provider
            .ceremony_tap_sequence(&credential_id_bytes, tap_count, purpose)
            .await
            .map_err(|e| format!("Ceremony failed: {e}"))?;

        let entropy = ceremony_blake3_mix(&result);
        let entropy_b64 = BASE64.encode(entropy);

        let timing_summary = json!({
            "total_duration_ms": result.total_duration_ms,
            "mean_reaction_ms": result.mean_reaction_ms(),
            "reaction_jitter_ms": result.reaction_jitter_ms(),
            "inter_tap_intervals_ms": result.inter_tap_intervals_ms(),
            "timing_entropy_bits_estimate": result.timing_entropy_bits_estimate(),
        });

        info!(
            rp_id,
            taps_completed = result.taps_completed(),
            total_duration_ms = result.total_duration_ms,
            jitter_ms = result.reaction_jitter_ms(),
            entropy_bits = result.timing_entropy_bits_estimate(),
            "Ceremony complete — Tier 3 entropy harvested"
        );

        Ok(json!({
            "entropy": entropy_b64,
            "taps_completed": result.taps_completed(),
            "taps_requested": result.taps_requested,
            "tier": 3,
            "timing_summary": timing_summary,
            "sources": ["os_rng", "fido2_hardware", "human_temporal"],
            "purpose": purpose,
        }))
    }

    #[cfg(not(feature = "ctap2"))]
    {
        let _ = (rp_id, credential_id, tap_count, purpose);
        Err(
            "FIDO2 feature not enabled — rebuild bearDog with --features ctap2 to use \
             tap-sequence entropy ceremonies"
                .to_string()
                .into(),
        )
    }
}

/// BLAKE3 keyed mixing of all ceremony tap data.
///
/// Combines three independent entropy sources per tap:
/// - Tier 1: fresh OS-RNG challenge (32 bytes)
/// - Tier 2: authenticator signature with hardware RNG nonce (~64 bytes)
/// - Tier 3: human reaction timing (nanoseconds)
///
/// Plus inter-tap rhythm intervals for additional temporal entropy.
#[cfg(feature = "ctap2")]
fn ceremony_blake3_mix(ceremony: &crate::tunnel::hsm::solo_v2::CeremonyResult) -> [u8; 32] {
    let key = *blake3::hash(b"beardog_loam_ceremony_v1________").as_bytes();
    let mut hasher = blake3::Hasher::new_keyed(&key);

    for tap in &ceremony.taps {
        hasher.update(&tap.challenge);
        hasher.update(&tap.signature);
        hasher.update(&tap.timing.reaction_ns().to_le_bytes());
        hasher.update(&tap.timing.eagain_count.to_le_bytes());
        hasher.update(&tap.timing.keepalive_count.to_le_bytes());
    }

    for interval_ns in &ceremony.inter_tap_intervals_ns() {
        hasher.update(&interval_ns.to_le_bytes());
    }

    *hasher.finalize().as_bytes()
}
