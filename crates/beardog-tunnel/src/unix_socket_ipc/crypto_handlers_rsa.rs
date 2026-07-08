// SPDX-License-Identifier: AGPL-3.0-or-later

//! RSA Crypto Handlers — PKCS#1 v1.5 (legacy) and RSA-PSS (modern).
//!
//! Pure Rust RSA signature operations using `RustCrypto`'s rsa crate.
//!
//! # Supported Algorithms
//!
//! - **RSA PKCS#1 v1.5**: Legacy padding (2048, 3072, 4096-bit)
//! - **RSA-PSS**: Modern probabilistic padding (2048, 3072, 4096-bit)
//!
//! # RPC Methods
//!
//! - `crypto.sign_rsa_pkcs1_sha256` / `crypto.verify_rsa_pkcs1_sha256`
//! - `crypto.sign_rsa_pss_sha256` / `crypto.verify_rsa_pss_sha256`

use crate::unix_socket_ipc::handlers::HandlerError;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use rsa::pkcs1v15::{SigningKey as Pkcs1SigningKey, VerifyingKey as Pkcs1VerifyingKey};
use rsa::pss::{SigningKey as PssSigningKey, VerifyingKey as PssVerifyingKey};
use rsa::rand_core::OsRng;
use rsa::signature::{RandomizedSigner, SignatureEncoding, Verifier};
use rsa::{RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;
use tracing::{debug, info};
use zeroize::Zeroizing;

const VALID_KEY_SIZES: [usize; 3] = [2048, 3072, 4096];

/// Parsed signing request (shared across PKCS#1 and PSS).
struct SignRequest {
    data: Vec<u8>,
    key_size: usize,
}

/// Parsed verification request (shared across PKCS#1 and PSS).
struct VerifyRequest {
    data: Vec<u8>,
    signature_bytes: Zeroizing<Vec<u8>>,
    public_key: RsaPublicKey,
}

fn parse_sign_request(params: Option<&serde_json::Value>) -> Result<SignRequest, HandlerError> {
    let params = params.ok_or("Missing parameters for RSA signing")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'data' parameter")?;

    #[expect(
        clippy::cast_possible_truncation,
        reason = "RSA modulus size validated to 2048/3072/4096"
    )]
    let key_size = params
        .get("key_size")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(2048) as usize;

    if !VALID_KEY_SIZES.contains(&key_size) {
        return Err(format!("Invalid key size: {key_size}. Supported: 2048, 3072, 4096").into());
    }

    let data = BASE64
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    debug!(
        "RSA sign request: {} bytes, key size: {} bits",
        data.len(),
        key_size
    );

    Ok(SignRequest { data, key_size })
}

fn parse_verify_request(
    params: Option<&serde_json::Value>,
) -> Result<VerifyRequest, HandlerError> {
    let params = params.ok_or("Missing parameters for RSA verification")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'data' parameter")?;
    let signature_b64 = params
        .get("signature")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'signature' parameter")?;
    let public_key_pem = params
        .get("public_key_pem")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'public_key_pem' parameter")?;

    let data = BASE64
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    let signature_bytes = Zeroizing::new(
        BASE64
            .decode(signature_b64)
            .map_err(|e| format!("Invalid base64 signature: {e}"))?,
    );

    debug!(
        "RSA verify request: data={} bytes, signature={} bytes",
        data.len(),
        signature_bytes.len()
    );

    let public_key = rsa::pkcs8::DecodePublicKey::from_public_key_pem(public_key_pem)
        .map_err(|e| format!("Invalid PEM public key: {e}"))?;

    Ok(VerifyRequest {
        data,
        signature_bytes,
        public_key,
    })
}

fn build_sign_response(
    data_len: usize,
    key_size: usize,
    signature: &[u8],
    public_key: &RsaPublicKey,
    algorithm: &str,
) -> Result<serde_json::Value, HandlerError> {
    let public_key_pem =
        rsa::pkcs8::EncodePublicKey::to_public_key_pem(public_key, rsa::pkcs8::LineEnding::LF)
            .map_err(|e| format!("Failed to encode public key: {e}"))?;

    info!(
        "RSA {algorithm}: Signed {data_len} bytes with {key_size}-bit key, signature {} bytes",
        signature.len()
    );

    Ok(serde_json::json!({
        "signature": BASE64.encode(signature),
        "public_key_pem": public_key_pem,
        "algorithm": algorithm,
        "key_size": key_size,
        "hash": "SHA-256"
    }))
}

// ---------------------------------------------------------------------------
// RSA PKCS#1 v1.5 (Legacy Support)
// ---------------------------------------------------------------------------

/// Sign data with RSA PKCS#1 v1.5 + SHA-256.
///
/// # Errors
///
/// Returns an error if parameters are missing/invalid or key generation fails.
pub async fn handle_sign_rsa_pkcs1_sha256(
    params: Option<&serde_json::Value>,
) -> Result<serde_json::Value, HandlerError> {
    let req = parse_sign_request(params)?;

    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, req.key_size)
        .map_err(|e| format!("Failed to generate RSA key: {e}"))?;
    let public_key = RsaPublicKey::from(&private_key);

    let signing_key = Pkcs1SigningKey::<Sha256>::new(private_key);
    let signature = signing_key.sign_with_rng(&mut rng, &req.data).to_vec();

    build_sign_response(
        req.data.len(),
        req.key_size,
        &signature,
        &public_key,
        "rsa_pkcs1_sha256",
    )
}

/// Verify RSA PKCS#1 v1.5 + SHA-256 signature.
///
/// # Errors
///
/// Returns an error if parameters are missing/invalid or signature parsing fails.
pub async fn handle_verify_rsa_pkcs1_sha256(
    params: Option<&serde_json::Value>,
) -> Result<serde_json::Value, HandlerError> {
    let req = parse_verify_request(params)?;

    let verifying_key = Pkcs1VerifyingKey::<Sha256>::new(req.public_key);
    let signature = rsa::pkcs1v15::Signature::try_from(req.signature_bytes.as_slice())
        .map_err(|e| format!("Invalid signature format: {e}"))?;

    let valid = verifying_key.verify(&req.data, &signature).is_ok();
    info!("RSA PKCS#1: Signature {}", if valid { "VALID" } else { "INVALID" });

    Ok(serde_json::json!({
        "valid": valid,
        "algorithm": "rsa_pkcs1_sha256"
    }))
}

// ---------------------------------------------------------------------------
// RSA-PSS (Modern, Recommended)
// ---------------------------------------------------------------------------

/// Sign data with RSA-PSS + SHA-256.
///
/// # Errors
///
/// Returns an error if parameters are missing/invalid or key generation fails.
pub async fn handle_sign_rsa_pss_sha256(
    params: Option<&serde_json::Value>,
) -> Result<serde_json::Value, HandlerError> {
    let req = parse_sign_request(params)?;

    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, req.key_size)
        .map_err(|e| format!("Failed to generate RSA key: {e}"))?;
    let public_key = RsaPublicKey::from(&private_key);

    let signing_key = PssSigningKey::<Sha256>::new(private_key);
    let signature = signing_key.sign_with_rng(&mut rng, &req.data).to_vec();

    build_sign_response(
        req.data.len(),
        req.key_size,
        &signature,
        &public_key,
        "rsa_pss_sha256",
    )
}

/// Verify RSA-PSS + SHA-256 signature.
///
/// # Errors
///
/// Returns an error if parameters are missing/invalid or signature parsing fails.
pub async fn handle_verify_rsa_pss_sha256(
    params: Option<&serde_json::Value>,
) -> Result<serde_json::Value, HandlerError> {
    let req = parse_verify_request(params)?;

    let verifying_key = PssVerifyingKey::<Sha256>::new(req.public_key);
    let signature = rsa::pss::Signature::try_from(req.signature_bytes.as_slice())
        .map_err(|e| format!("Invalid signature format: {e}"))?;

    let valid = verifying_key.verify(&req.data, &signature).is_ok();
    info!("RSA-PSS: Signature {}", if valid { "VALID" } else { "INVALID" });

    Ok(serde_json::json!({
        "valid": valid,
        "algorithm": "rsa_pss_sha256"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn pkcs1_2048_sign_and_verify_roundtrip() {
        let data_b64 = BASE64.encode(b"Hello, RSA PKCS#1 v1.5!");
        let sign_params = serde_json::json!({ "data": data_b64, "key_size": 2048 });

        let sign_result = handle_sign_rsa_pkcs1_sha256(Some(&sign_params))
            .await
            .expect("Signing should succeed");

        assert_eq!(sign_result["key_size"], 2048);
        assert_eq!(sign_result["algorithm"], "rsa_pkcs1_sha256");

        let verify_params = serde_json::json!({
            "data": data_b64,
            "signature": sign_result["signature"],
            "public_key_pem": sign_result["public_key_pem"]
        });

        let verify_result = handle_verify_rsa_pkcs1_sha256(Some(&verify_params))
            .await
            .expect("Verification should succeed");

        assert_eq!(verify_result["valid"], true);
    }

    #[tokio::test]
    async fn pkcs1_verify_tampered_data_is_invalid() {
        let data_b64 = BASE64.encode(b"Hello, RSA PKCS#1!");
        let sign_params = serde_json::json!({ "data": data_b64 });

        let sign_result = handle_sign_rsa_pkcs1_sha256(Some(&sign_params))
            .await
            .expect("Signing should succeed");

        let verify_params = serde_json::json!({
            "data": BASE64.encode(b"Tampered data!"),
            "signature": sign_result["signature"],
            "public_key_pem": sign_result["public_key_pem"]
        });

        let verify_result = handle_verify_rsa_pkcs1_sha256(Some(&verify_params))
            .await
            .expect("Verification should complete");

        assert_eq!(verify_result["valid"], false);
    }

    #[tokio::test]
    async fn pss_2048_sign_and_verify_roundtrip() {
        let data_b64 = BASE64.encode(b"Hello, RSA-PSS!");
        let sign_params = serde_json::json!({ "data": data_b64, "key_size": 2048 });

        let sign_result = handle_sign_rsa_pss_sha256(Some(&sign_params))
            .await
            .expect("Signing should succeed");

        assert_eq!(sign_result["key_size"], 2048);
        assert_eq!(sign_result["algorithm"], "rsa_pss_sha256");

        let verify_params = serde_json::json!({
            "data": data_b64,
            "signature": sign_result["signature"],
            "public_key_pem": sign_result["public_key_pem"]
        });

        let verify_result = handle_verify_rsa_pss_sha256(Some(&verify_params))
            .await
            .expect("Verification should succeed");

        assert_eq!(verify_result["valid"], true);
    }

    #[tokio::test]
    async fn pss_verify_tampered_data_is_invalid() {
        let data_b64 = BASE64.encode(b"Hello, RSA-PSS!");
        let sign_params = serde_json::json!({ "data": data_b64 });

        let sign_result = handle_sign_rsa_pss_sha256(Some(&sign_params))
            .await
            .expect("Signing should succeed");

        let verify_params = serde_json::json!({
            "data": BASE64.encode(b"Tampered data!"),
            "signature": sign_result["signature"],
            "public_key_pem": sign_result["public_key_pem"]
        });

        let verify_result = handle_verify_rsa_pss_sha256(Some(&verify_params))
            .await
            .expect("Verification should complete");

        assert_eq!(verify_result["valid"], false);
    }

    #[tokio::test]
    async fn pkcs1_invalid_key_size_rejected() {
        let sign_params = serde_json::json!({
            "data": BASE64.encode(b"Test"),
            "key_size": 1024
        });
        let result = handle_sign_rsa_pkcs1_sha256(Some(&sign_params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid key size"));
    }

    #[tokio::test]
    async fn pss_missing_params_rejected() {
        let result = handle_sign_rsa_pss_sha256(None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing parameters"));
    }
}
