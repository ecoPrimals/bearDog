// SPDX-License-Identifier: AGPL-3.0-only

//! TLS 1.2 PRF (Pseudorandom Function) for key expansion.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde_json::Value;
use tracing::{debug, info};

// =============================================================================
// TLS 1.2 PRF (Pseudorandom Function)
// =============================================================================

/// # Errors
///
/// Returns an error if hashing fails.
/// Handle `crypto.kdf.tls12_prf` method
///
/// TLS 1.2 Pseudorandom Function for key expansion (RFC 5246 Section 5).
///
/// # Parameters
///
/// - `secret`: Base64-encoded secret (e.g., premaster secret or master secret)
/// - `label`: ASCII label string (e.g., "master secret", "key expansion")
/// - `seed`: Base64-encoded seed data (e.g., `client_random` + `server_random`)
/// - `output_len`: Desired output length in bytes
/// - `hash`: Hash algorithm ("sha256" or "sha384", default "sha256")
///
/// # Returns
///
/// - `output`: Base64-encoded PRF output
/// - `algorithm`: "TLS12-PRF-SHA256" or "TLS12-PRF-SHA384"
///
/// # TLS 1.2 PRF Definition
///
/// ```text
/// PRF(secret, label, seed) = P_hash(secret, label + seed)
///
/// P_hash(secret, seed) = HMAC_hash(secret, A(1) + seed) +
///                        HMAC_hash(secret, A(2) + seed) +
///                        HMAC_hash(secret, A(3) + seed) + ...
///
/// where:
/// A(0) = seed
/// A(i) = HMAC_hash(secret, A(i-1))
/// ```
pub async fn handle_tls12_prf(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing parameters for TLS 1.2 PRF")?;

    let secret_b64 = params
        .get("secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'secret' parameter")?;

    let label = params
        .get("label")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'label' parameter")?;

    let seed_b64 = params
        .get("seed")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'seed' parameter")?;

    #[expect(
        clippy::cast_possible_truncation,
        reason = "TLS 1.2 PRF output length from parameters"
    )]
    let output_len = params
        .get("output_len")
        .and_then(serde_json::Value::as_u64)
        .ok_or("Missing or invalid 'output_len' parameter")? as usize;

    let hash_alg = params
        .get("hash")
        .and_then(|v| v.as_str())
        .unwrap_or("sha256");

    debug!(
        "🔑 TLS 1.2 PRF: label='{}', hash={}, output_len={}",
        label, hash_alg, output_len
    );

    // Decode inputs
    let secret = BASE64
        .decode(secret_b64)
        .map_err(|e| format!("Invalid secret base64: {e}"))?;

    let seed = BASE64
        .decode(seed_b64)
        .map_err(|e| format!("Invalid seed base64: {e}"))?;

    // Compute PRF
    let output = match hash_alg {
        "sha256" => tls12_prf_sha256(&secret, label.as_bytes(), &seed, output_len)?,
        "sha384" => tls12_prf_sha384(&secret, label.as_bytes(), &seed, output_len)?,
        _ => {
            return Err(format!(
                "Unsupported hash algorithm: {hash_alg} (use 'sha256' or 'sha384')"
            ));
        }
    };

    let output_b64 = BASE64.encode(&output);

    info!(
        "✅ TLS 1.2 PRF complete ({} bytes output, {})",
        output_len,
        hash_alg.to_uppercase()
    );

    Ok(serde_json::json!({
        "output": output_b64,
        "algorithm": format!("TLS12-PRF-{}", hash_alg.to_uppercase()),
    }))
}

/// TLS 1.2 PRF with SHA-256
fn tls12_prf_sha256(
    secret: &[u8],
    label: &[u8],
    seed: &[u8],
    output_len: usize,
) -> Result<Vec<u8>, String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    // Combine label + seed
    let mut label_and_seed = Vec::with_capacity(label.len() + seed.len());
    label_and_seed.extend_from_slice(label);
    label_and_seed.extend_from_slice(seed);

    // P_hash expansion
    let mut output = Vec::with_capacity(output_len);
    let mut a = label_and_seed.clone(); // A(0) = seed

    while output.len() < output_len {
        // A(i) = HMAC(secret, A(i-1))
        let mut mac = HmacSha256::new_from_slice(secret)
            .map_err(|e| format!("HMAC initialization failed: {e}"))?;
        mac.update(&a);
        a = mac.finalize().into_bytes().to_vec();

        // P_hash = HMAC(secret, A(i) + seed)
        let mut mac = HmacSha256::new_from_slice(secret)
            .map_err(|e| format!("HMAC initialization failed: {e}"))?;
        mac.update(&a);
        mac.update(&label_and_seed);
        let hmac_output = mac.finalize().into_bytes();

        output.extend_from_slice(&hmac_output);
    }

    output.truncate(output_len);
    Ok(output)
}

/// TLS 1.2 PRF with SHA-384
fn tls12_prf_sha384(
    secret: &[u8],
    label: &[u8],
    seed: &[u8],
    output_len: usize,
) -> Result<Vec<u8>, String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha384;

    type HmacSha384 = Hmac<Sha384>;

    // Combine label + seed
    let mut label_and_seed = Vec::with_capacity(label.len() + seed.len());
    label_and_seed.extend_from_slice(label);
    label_and_seed.extend_from_slice(seed);

    // P_hash expansion
    let mut output = Vec::with_capacity(output_len);
    let mut a = label_and_seed.clone(); // A(0) = seed

    while output.len() < output_len {
        // A(i) = HMAC(secret, A(i-1))
        let mut mac = HmacSha384::new_from_slice(secret)
            .map_err(|e| format!("HMAC initialization failed: {e}"))?;
        mac.update(&a);
        a = mac.finalize().into_bytes().to_vec();

        // P_hash = HMAC(secret, A(i) + seed)
        let mut mac = HmacSha384::new_from_slice(secret)
            .map_err(|e| format!("HMAC initialization failed: {e}"))?;
        mac.update(&a);
        mac.update(&label_and_seed);
        let hmac_output = mac.finalize().into_bytes();

        output.extend_from_slice(&hmac_output);
    }

    output.truncate(output_len);
    Ok(output)
}
