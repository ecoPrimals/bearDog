// SPDX-License-Identifier: AGPL-3.0-or-later

//! Device Enrollment - Seed Derivation and Lineage Certificates
//!
//! Handles device enrollment into a family via HKDF seed derivation
//! and Ed25519 lineage certificate signing/verification.
//!
//! # Deep Debt Principle: DERIVE, not COPY
//!
//! Each device gets a uniquely derived seed from the root genesis seed.
//! Compromising one device's seed does NOT reveal the root seed.
//!
//! # Handlers
//!
//! - `handle_derive_device_seed` - HKDF-SHA256 device seed derivation
//! - `handle_sign_lineage_certificate` - Ed25519 certificate signing
//! - `handle_verify_lineage_certificate` - Certificate verification with chain support

use super::{
    CertificateVerificationDetails, DeriveDeviceSeedRequest, DeriveDeviceSeedResponse,
    LineageCertificate, SignLineageCertificateRequest, SignLineageCertificateResponse,
    VerifyLineageCertificateRequest, VerifyLineageCertificateResponse,
};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use ed25519_dalek::{Signature as DalekSignature, Signer, SigningKey, Verifier, VerifyingKey};
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tracing::{debug, info, warn};

type HmacSha256 = Hmac<Sha256>;

/// # Errors
///
/// Returns an error if serialization fails.
/// Handle `genetic.derive_device_seed` RPC method
///
/// Derives a UNIQUE device seed from the family's root genesis seed.
///
/// # Security Properties
/// - Forward secrecy: compromising one device doesn't reveal root seed
/// - Device isolation: each device has unique cryptographic material
/// - Verifiability: `derivation_proof` proves correct derivation
///
/// # Performance
/// - Expected: < 200μs (HKDF-SHA256)
pub async fn handle_derive_device_seed(params: &Value) -> Result<Value, BearDogError> {
    debug!("🧬 RPC: genetic.derive_device_seed");

    let request: DeriveDeviceSeedRequest =
        DeriveDeviceSeedRequest::deserialize(params).map_err(|e| {
            BearDogError::invalid_input(&format!("Invalid derive_device_seed params: {e}"))
        })?;

    let root_seed = BASE64.decode(&request.root_seed).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid root_seed (not base64): {e}"))
    })?;

    if root_seed.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "root_seed must be 32 bytes, got {}",
            root_seed.len()
        )));
    }

    let device_entropy = BASE64.decode(&request.device_entropy).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid device_entropy (not base64): {e}"))
    })?;

    if device_entropy.len() < 16 {
        return Err(BearDogError::invalid_input(&format!(
            "device_entropy must be at least 16 bytes, got {}",
            device_entropy.len()
        )));
    }

    let domain = b"beardog_device_seed_v1";
    let hkdf = Hkdf::<Sha256>::new(Some(&device_entropy), &root_seed);

    let mut info = format!("{}:{}", domain.escape_ascii(), request.device_id);
    if let Some(ts) = request.enrollment_timestamp {
        use std::fmt::Write as _;
        write!(&mut info, ":{ts}").map_err(|_| {
            BearDogError::internal("enrollment HKDF info string formatting failed".to_string())
        })?;
    }

    let mut device_seed = [0u8; 32];
    hkdf.expand(info.as_bytes(), &mut device_seed)
        .map_err(|_| BearDogError::internal("HKDF expansion failed".to_string()))?;

    let mut mac = HmacSha256::new_from_slice(&root_seed)
        .map_err(|_| BearDogError::internal("HMAC key creation failed".to_string()))?;
    mac.update(&device_seed);
    mac.update(request.device_id.as_bytes());
    let proof = mac.finalize().into_bytes();

    let device_seed_b64 = BASE64.encode(device_seed);
    let proof_b64 = BASE64.encode(proof);

    info!(
        "✅ Derived device seed for '{}' (32 bytes, unique derivation)",
        request.device_id
    );

    Ok(json!(DeriveDeviceSeedResponse {
        device_seed: device_seed_b64,
        device_id: request.device_id,
        kdf: "HKDF-SHA256".to_string(),
        domain: String::from_utf8_lossy(domain).to_string(),
        derivation_proof: proof_b64,
    }))
}

/// # Errors
///
/// Returns an error if serialization fails.
/// Handle `genetic.sign_lineage_certificate` RPC method
///
/// Signs a lineage certificate for device enrollment using Ed25519.
///
/// # Performance
/// - Expected: < 500μs (Ed25519 sign)
pub async fn handle_sign_lineage_certificate(params: &Value) -> Result<Value, BearDogError> {
    debug!("🧬 RPC: genetic.sign_lineage_certificate");

    let request: SignLineageCertificateRequest = SignLineageCertificateRequest::deserialize(params)
        .map_err(|e| {
            BearDogError::invalid_input(&format!("Invalid sign_lineage_certificate params: {e}"))
        })?;

    let parent_seed = BASE64.decode(&request.parent_seed).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid parent_seed (not base64): {e}"))
    })?;

    if parent_seed.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "parent_seed must be 32 bytes, got {}",
            parent_seed.len()
        )));
    }

    let child_pubkey_bytes = BASE64.decode(&request.child_public_key).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid child_public_key (not base64): {e}"))
    })?;

    if child_pubkey_bytes.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "child_public_key must be 32 bytes, got {}",
            child_pubkey_bytes.len()
        )));
    }

    let parent_seed_array: [u8; 32] = parent_seed.try_into().map_err(|_| {
        BearDogError::internal("Failed to convert parent seed to array".to_string())
    })?;

    let signing_key = SigningKey::from_bytes(&parent_seed_array);
    let parent_public_key = VerifyingKey::from(&signing_key);

    let issued_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let version: u8 = 1;
    let depth: u32 = 1;

    let mut message = Vec::new();
    message.push(version);
    message.extend(request.parent_device_id.as_bytes());
    message.push(0);
    message.extend(request.child_device_id.as_bytes());
    message.push(0);
    message.extend(&child_pubkey_bytes);
    message.extend(request.family_id.as_bytes());
    message.push(0);
    message.extend(&issued_at.to_le_bytes());
    message.extend(&depth.to_le_bytes());

    let signature = signing_key.sign(&message);

    let certificate = LineageCertificate {
        version,
        parent_device_id: request.parent_device_id,
        child_device_id: request.child_device_id,
        child_public_key: request.child_public_key,
        family_id: request.family_id,
        issued_at,
        expires_at: request.expires_at,
        depth,
        parent_signature: BASE64.encode(signature.to_bytes()),
        parent_public_key: BASE64.encode(parent_public_key.to_bytes()),
    };

    let cert_json = serde_json::to_string(&certificate)
        .map_err(|e| BearDogError::internal(format!("Failed to serialize certificate: {e}")))?;
    let cert_id = sha2::Sha256::digest(cert_json.as_bytes());
    let cert_id_hex = hex::encode(cert_id);

    info!(
        "✅ Signed lineage certificate: {} → {} (cert: {}...)",
        certificate.parent_device_id,
        certificate.child_device_id,
        &cert_id_hex[..16]
    );

    Ok(json!(SignLineageCertificateResponse {
        certificate,
        certificate_id: cert_id_hex,
    }))
}

/// # Errors
///
/// Returns an error if serialization fails.
/// Handle `genetic.verify_lineage_certificate` RPC method
///
/// Verifies a lineage certificate's Ed25519 signature, expiration,
/// `family_id`, and optional trust chain.
///
/// # Performance
/// - Expected: < 300μs (Ed25519 verify)
pub async fn handle_verify_lineage_certificate(params: &Value) -> Result<Value, BearDogError> {
    debug!("🔍 RPC: genetic.verify_lineage_certificate");

    let request: VerifyLineageCertificateRequest =
        VerifyLineageCertificateRequest::deserialize(params).map_err(|e| {
            BearDogError::invalid_input(&format!("Invalid verify_lineage_certificate params: {e}"))
        })?;

    let cert = &request.certificate;

    let mut details = CertificateVerificationDetails {
        signature_valid: false,
        not_expired: true,
        family_id_matches: true,
        chain_verified: request.trust_anchors.is_empty(),
        chain_depth: cert.depth,
        failure_reason: None,
    };

    let parent_pubkey_bytes = BASE64
        .decode(&cert.parent_public_key)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid parent_public_key: {e}")))?;

    let signature_bytes = BASE64
        .decode(&cert.parent_signature)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid parent_signature: {e}")))?;

    let child_pubkey_bytes = BASE64
        .decode(&cert.child_public_key)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid child_public_key: {e}")))?;

    let parent_pubkey_array: [u8; 32] = parent_pubkey_bytes
        .try_into()
        .map_err(|_| BearDogError::invalid_input("parent_public_key must be 32 bytes"))?;

    let signature_array: [u8; 64] = signature_bytes
        .try_into()
        .map_err(|_| BearDogError::invalid_input("parent_signature must be 64 bytes"))?;

    let verifying_key = VerifyingKey::from_bytes(&parent_pubkey_array)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid public key: {e}")))?;

    let signature = DalekSignature::from_bytes(&signature_array);

    let mut message = Vec::new();
    message.push(cert.version);
    message.extend(cert.parent_device_id.as_bytes());
    message.push(0);
    message.extend(cert.child_device_id.as_bytes());
    message.push(0);
    message.extend(&child_pubkey_bytes);
    message.extend(cert.family_id.as_bytes());
    message.push(0);
    message.extend(&cert.issued_at.to_le_bytes());
    message.extend(&cert.depth.to_le_bytes());

    details.signature_valid = verifying_key.verify(&message, &signature).is_ok();

    if !details.signature_valid {
        details.failure_reason = Some("Invalid Ed25519 signature".to_string());
    }

    if let Some(expires_at) = cert.expires_at {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        if now > expires_at {
            details.not_expired = false;
            details.failure_reason = Some(format!(
                "Certificate expired at {expires_at} (current: {now})"
            ));
        }
    }

    if let Some(expected) = &request.expected_family_id
        && &cert.family_id != expected
    {
        details.family_id_matches = false;
        details.failure_reason = Some(format!(
            "Family ID mismatch: expected '{}', got '{}'",
            expected, cert.family_id
        ));
    }

    if !request.trust_anchors.is_empty() {
        let parent_key_matches = request.trust_anchors.iter().any(|anchor| {
            anchor.child_public_key == cert.child_public_key
                || anchor.parent_public_key == cert.parent_public_key
        });

        details.chain_verified = parent_key_matches;

        if !parent_key_matches {
            details.failure_reason =
                Some("Certificate chain doesn't link to any trust anchor".to_string());
        }
    }

    let valid = details.signature_valid
        && details.not_expired
        && details.family_id_matches
        && details.chain_verified;

    if valid {
        info!(
            "✅ Certificate verified: {} → {} (depth: {})",
            cert.parent_device_id, cert.child_device_id, cert.depth
        );
    } else {
        warn!(
            "❌ Certificate verification FAILED: {} → {} (reason: {:?})",
            cert.parent_device_id, cert.child_device_id, details.failure_reason
        );
    }

    Ok(json!(VerifyLineageCertificateResponse { valid, details }))
}

#[cfg(test)]
#[path = "enrollment_tests.rs"]
mod tests;
