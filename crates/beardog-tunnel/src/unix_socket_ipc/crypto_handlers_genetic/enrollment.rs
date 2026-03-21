// SPDX-License-Identifier: AGPL-3.0-only

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

use super::*;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use hkdf::Hkdf;
use serde_json::{Value, json};
use sha2::Sha256;
use tracing::{debug, info, warn};

/// Handle `genetic.derive_device_seed` RPC method
///
/// Derives a UNIQUE device seed from the family's root genesis seed.
///
/// # Security Properties
/// - Forward secrecy: compromising one device doesn't reveal root seed
/// - Device isolation: each device has unique cryptographic material
/// - Verifiability: derivation_proof proves correct derivation
///
/// # Performance
/// - Expected: < 200μs (HKDF-SHA256)
pub async fn handle_derive_device_seed(params: Value) -> Result<Value, BearDogError> {
    debug!("🧬 RPC: genetic.derive_device_seed");

    let request: DeriveDeviceSeedRequest = serde_json::from_value(params).map_err(|e| {
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

    use hmac::{Hmac, Mac};
    type HmacSha256 = Hmac<Sha256>;

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

/// Handle `genetic.sign_lineage_certificate` RPC method
///
/// Signs a lineage certificate for device enrollment using Ed25519.
///
/// # Performance
/// - Expected: < 500μs (Ed25519 sign)
pub async fn handle_sign_lineage_certificate(params: Value) -> Result<Value, BearDogError> {
    debug!("🧬 RPC: genetic.sign_lineage_certificate");

    let request: SignLineageCertificateRequest = serde_json::from_value(params).map_err(|e| {
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

    use ed25519_dalek::{Signer, SigningKey, VerifyingKey};

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

    use sha2::Digest;
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

/// Handle `genetic.verify_lineage_certificate` RPC method
///
/// Verifies a lineage certificate's Ed25519 signature, expiration,
/// family_id, and optional trust chain.
///
/// # Performance
/// - Expected: < 300μs (Ed25519 verify)
pub async fn handle_verify_lineage_certificate(params: Value) -> Result<Value, BearDogError> {
    debug!("🔍 RPC: genetic.verify_lineage_certificate");

    let request: VerifyLineageCertificateRequest = serde_json::from_value(params).map_err(|e| {
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

    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let parent_pubkey_array: [u8; 32] = parent_pubkey_bytes
        .try_into()
        .map_err(|_| BearDogError::invalid_input("parent_public_key must be 32 bytes"))?;

    let signature_array: [u8; 64] = signature_bytes
        .try_into()
        .map_err(|_| BearDogError::invalid_input("parent_signature must be 64 bytes"))?;

    let verifying_key = VerifyingKey::from_bytes(&parent_pubkey_array)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid public key: {e}")))?;

    let signature = Signature::from_bytes(&signature_array);

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

    if let Some(expected) = &request.expected_family_id {
        if &cert.family_id != expected {
            details.family_id_matches = false;
            details.failure_reason = Some(format!(
                "Family ID mismatch: expected '{}', got '{}'",
                expected, cert.family_id
            ));
        }
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
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_derive_device_seed() -> Result<(), Box<dyn std::error::Error>> {
        let root_seed = BASE64.encode(b"genesis_root_seed_32byteslong!!!");
        let device_entropy = BASE64.encode(b"pixel8a_entropy_data_here!!!_32!");

        let params = json!({
            "root_seed": root_seed,
            "device_entropy": device_entropy,
            "device_id": "pixel8a",
        });

        let result = handle_derive_device_seed(params).await?;
        let response: DeriveDeviceSeedResponse = serde_json::from_value(result)?;

        assert_eq!(response.device_id, "pixel8a");
        assert_eq!(response.kdf, "HKDF-SHA256");
        assert!(!response.device_seed.is_empty());
        assert!(!response.derivation_proof.is_empty());

        let device_seed_bytes = BASE64.decode(&response.device_seed)?;
        let proof_bytes = BASE64.decode(&response.derivation_proof)?;

        assert_eq!(device_seed_bytes.len(), 32);
        assert_eq!(proof_bytes.len(), 32);

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_device_seed_different_devices() -> Result<(), Box<dyn std::error::Error>> {
        let root_seed = BASE64.encode(b"shared_family_root_seed_here!!!!");

        let entropy1 = BASE64.encode(b"usb_desktop_entropy_1234567890!!");
        let entropy2 = BASE64.encode(b"pixel8a_mobile_entropy_987654!!");

        let result1 = handle_derive_device_seed(json!({
            "root_seed": root_seed.clone(),
            "device_entropy": entropy1,
            "device_id": "usb-desktop",
        }))
        .await?;

        let result2 = handle_derive_device_seed(json!({
            "root_seed": root_seed,
            "device_entropy": entropy2,
            "device_id": "pixel8a",
        }))
        .await?;

        let resp1: DeriveDeviceSeedResponse = serde_json::from_value(result1)?;
        let resp2: DeriveDeviceSeedResponse = serde_json::from_value(result2)?;

        assert_ne!(
            resp1.device_seed, resp2.device_seed,
            "Different devices should have different derived seeds"
        );
        assert_ne!(
            resp1.derivation_proof, resp2.derivation_proof,
            "Derivation proofs should be unique per device"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_device_seed_deterministic() -> Result<(), Box<dyn std::error::Error>> {
        let root_seed = BASE64.encode(b"deterministic_root_seed_test!!!!");
        let device_entropy = BASE64.encode(b"deterministic_device_entropy!!!!");

        let params = json!({
            "root_seed": root_seed,
            "device_entropy": device_entropy,
            "device_id": "test-device",
        });

        let resp1: DeriveDeviceSeedResponse =
            serde_json::from_value(handle_derive_device_seed(params.clone()).await?)?;
        let resp2: DeriveDeviceSeedResponse =
            serde_json::from_value(handle_derive_device_seed(params).await?)?;

        assert_eq!(
            resp1.device_seed, resp2.device_seed,
            "Same inputs should produce same device seed"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_sign_and_verify_lineage_certificate() -> Result<(), Box<dyn std::error::Error>> {
        let parent_seed = BASE64.encode(b"parent_device_seed_32_bytes!!!!!");

        use ed25519_dalek::SigningKey;
        let child_signing_key = SigningKey::from_bytes(&[42u8; 32]);
        let child_public_key = BASE64.encode(child_signing_key.verifying_key().to_bytes());

        let sign_result = handle_sign_lineage_certificate(json!({
            "parent_seed": parent_seed,
            "parent_device_id": "usb-desktop",
            "child_public_key": child_public_key,
            "child_device_id": "pixel8a",
            "family_id": "8ff3b864a4bc589a",
        }))
        .await?;

        let sign_response: SignLineageCertificateResponse = serde_json::from_value(sign_result)?;

        assert!(!sign_response.certificate_id.is_empty());
        assert_eq!(sign_response.certificate.parent_device_id, "usb-desktop");
        assert_eq!(sign_response.certificate.child_device_id, "pixel8a");

        let verify_result = handle_verify_lineage_certificate(json!({
            "certificate": sign_response.certificate,
            "expected_family_id": "8ff3b864a4bc589a",
            "trust_anchors": [],
        }))
        .await?;

        let verify_response: VerifyLineageCertificateResponse =
            serde_json::from_value(verify_result)?;

        assert!(verify_response.valid, "Certificate should be valid");
        assert!(verify_response.details.signature_valid);
        assert!(verify_response.details.not_expired);
        assert!(verify_response.details.family_id_matches);

        Ok(())
    }

    #[tokio::test]
    async fn test_verify_lineage_certificate_wrong_family() -> Result<(), Box<dyn std::error::Error>>
    {
        let parent_seed = BASE64.encode(b"parent_device_seed_for_test!!!!!");

        use ed25519_dalek::SigningKey;
        let child_key = SigningKey::from_bytes(&[99u8; 32]);
        let child_pubkey = BASE64.encode(child_key.verifying_key().to_bytes());

        let sign_result = handle_sign_lineage_certificate(json!({
            "parent_seed": parent_seed,
            "parent_device_id": "device-a",
            "child_public_key": child_pubkey,
            "child_device_id": "device-b",
            "family_id": "family_aaa",
        }))
        .await?;
        let sign_response: SignLineageCertificateResponse = serde_json::from_value(sign_result)?;

        let verify_result = handle_verify_lineage_certificate(json!({
            "certificate": sign_response.certificate,
            "expected_family_id": "family_bbb",
            "trust_anchors": [],
        }))
        .await?;
        let verify_response: VerifyLineageCertificateResponse =
            serde_json::from_value(verify_result)?;

        assert!(!verify_response.valid);
        assert!(!verify_response.details.family_id_matches);

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_device_seed_invalid_root_seed_length() {
        let short_seed = BASE64.encode(b"only_16_bytes!!");
        let device_entropy = BASE64.encode(b"device_entropy_data_32_bytes!!!!");

        let result = handle_derive_device_seed(json!({
            "root_seed": short_seed,
            "device_entropy": device_entropy,
            "device_id": "test-device",
        }))
        .await;

        assert!(result.is_err(), "Should reject short root seed");
    }

    #[tokio::test]
    async fn test_concurrent_device_seed_derivation() -> Result<(), Box<dyn std::error::Error>> {
        use tokio::task::JoinSet;

        let root_seed = BASE64.encode(b"concurrent_root_seed_testing!!!!");
        let mut tasks = JoinSet::new();

        for i in 0..10 {
            let root = root_seed.clone();
            let entropy =
                BASE64.encode(format!("entropy_for_device_{:02}_padding!!", i).as_bytes());
            let device_id = format!("device-{}", i);

            tasks.spawn(async move {
                handle_derive_device_seed(json!({
                    "root_seed": root,
                    "device_entropy": entropy,
                    "device_id": device_id,
                }))
                .await
            });
        }

        let mut results = Vec::new();
        while let Some(result) = tasks.join_next().await {
            results.push(result??);
        }

        assert_eq!(results.len(), 10);

        let seeds: std::collections::HashSet<_> = results
            .iter()
            .map(|r| r.get("device_seed").unwrap().as_str().unwrap())
            .collect();
        assert_eq!(
            seeds.len(),
            10,
            "All concurrent derivations should be unique"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_complete_enrollment_flow() -> Result<(), Box<dyn std::error::Error>> {
        let root_seed = BASE64.encode(b"genesis_root_seed_for_e2e_test!!");

        // Genesis derives its device seed
        let genesis_result = handle_derive_device_seed(json!({
            "root_seed": root_seed.clone(),
            "device_entropy": BASE64.encode(b"genesis_device_hardware_entropy!"),
            "device_id": "genesis-device",
        }))
        .await?;
        let genesis_seed: DeriveDeviceSeedResponse = serde_json::from_value(genesis_result)?;

        // USB Tower derives its device seed
        let usb_result = handle_derive_device_seed(json!({
            "root_seed": root_seed.clone(),
            "device_entropy": BASE64.encode(b"usb_tower_hardware_entropy_here!"),
            "device_id": "usb-tower",
        }))
        .await?;
        let usb_seed: DeriveDeviceSeedResponse = serde_json::from_value(usb_result)?;

        assert_ne!(genesis_seed.device_seed, usb_seed.device_seed);

        // Genesis signs certificate for USB Tower
        use ed25519_dalek::SigningKey;
        let usb_signing_key = SigningKey::from_bytes(
            &BASE64.decode(&usb_seed.device_seed)?[..32]
                .try_into()
                .unwrap(),
        );
        let usb_pubkey = BASE64.encode(usb_signing_key.verifying_key().to_bytes());

        let cert_result = handle_sign_lineage_certificate(json!({
            "parent_seed": genesis_seed.device_seed,
            "parent_device_id": "genesis-device",
            "child_public_key": usb_pubkey,
            "child_device_id": "usb-tower",
            "family_id": "test-family-e2e",
        }))
        .await?;
        let usb_cert: SignLineageCertificateResponse = serde_json::from_value(cert_result)?;

        let verify_result = handle_verify_lineage_certificate(json!({
            "certificate": usb_cert.certificate,
            "expected_family_id": "test-family-e2e",
            "trust_anchors": [],
        }))
        .await?;
        let verify_resp: VerifyLineageCertificateResponse = serde_json::from_value(verify_result)?;
        assert!(verify_resp.valid, "USB Tower certificate should be valid");

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_device_seed_invalid_json() {
        let r = handle_derive_device_seed(json!("not-an-object")).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn test_derive_device_seed_short_device_entropy() {
        let r = handle_derive_device_seed(json!({
            "root_seed": BASE64.encode([1u8; 32]),
            "device_entropy": BASE64.encode([2u8; 8]),
            "device_id": "x",
        }))
        .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn test_derive_device_seed_enrollment_timestamp_changes_output() {
        let root = BASE64.encode([3u8; 32]);
        let ent = BASE64.encode([4u8; 16]);
        let a: DeriveDeviceSeedResponse = serde_json::from_value(
            handle_derive_device_seed(json!({
                "root_seed": root.clone(),
                "device_entropy": ent.clone(),
                "device_id": "dev",
                "enrollment_timestamp": 1_700_000_000u64,
            }))
            .await
            .expect("a"),
        )
        .expect("parse");
        let b: DeriveDeviceSeedResponse = serde_json::from_value(
            handle_derive_device_seed(json!({
                "root_seed": root,
                "device_entropy": ent,
                "device_id": "dev",
                "enrollment_timestamp": 1_800_000_000u64,
            }))
            .await
            .expect("b"),
        )
        .expect("parse");
        assert_ne!(a.device_seed, b.device_seed);
    }

    #[tokio::test]
    async fn test_verify_lineage_certificate_expired() -> Result<(), Box<dyn std::error::Error>> {
        let parent_seed = BASE64.encode([5u8; 32]);
        use ed25519_dalek::SigningKey;
        let child = SigningKey::from_bytes(&[6u8; 32]);
        let child_pk = BASE64.encode(child.verifying_key().to_bytes());

        let sign = handle_sign_lineage_certificate(json!({
            "parent_seed": parent_seed,
            "parent_device_id": "p",
            "child_public_key": child_pk,
            "child_device_id": "c",
            "family_id": "fam",
            "expires_at": 1u64,
        }))
        .await?;
        let resp: SignLineageCertificateResponse = serde_json::from_value(sign)?;

        let verify = handle_verify_lineage_certificate(json!({
            "certificate": resp.certificate,
            "expected_family_id": "fam",
            "trust_anchors": [],
        }))
        .await?;
        let vr: VerifyLineageCertificateResponse = serde_json::from_value(verify)?;
        assert!(!vr.valid);
        assert!(!vr.details.not_expired);
        Ok(())
    }

    #[tokio::test]
    async fn test_verify_lineage_certificate_tampered_signature()
    -> Result<(), Box<dyn std::error::Error>> {
        let parent_seed = BASE64.encode([7u8; 32]);
        use ed25519_dalek::SigningKey;
        let child = SigningKey::from_bytes(&[8u8; 32]);
        let child_pk = BASE64.encode(child.verifying_key().to_bytes());

        let sign = handle_sign_lineage_certificate(json!({
            "parent_seed": parent_seed,
            "parent_device_id": "p",
            "child_public_key": child_pk,
            "child_device_id": "c",
            "family_id": "fam2",
        }))
        .await?;
        let mut resp: SignLineageCertificateResponse = serde_json::from_value(sign)?;
        let mut sig = BASE64.decode(&resp.certificate.parent_signature)?;
        sig[0] ^= 0xFF;
        resp.certificate.parent_signature = BASE64.encode(&sig);

        let verify = handle_verify_lineage_certificate(json!({
            "certificate": resp.certificate,
            "expected_family_id": "fam2",
            "trust_anchors": [],
        }))
        .await?;
        let vr: VerifyLineageCertificateResponse = serde_json::from_value(verify)?;
        assert!(!vr.valid);
        assert!(!vr.details.signature_valid);
        Ok(())
    }

    #[tokio::test]
    async fn test_verify_lineage_certificate_trust_anchor_match()
    -> Result<(), Box<dyn std::error::Error>> {
        let parent_seed = BASE64.encode([9u8; 32]);
        use ed25519_dalek::SigningKey;
        let child = SigningKey::from_bytes(&[10u8; 32]);
        let child_pk = BASE64.encode(child.verifying_key().to_bytes());

        let sign = handle_sign_lineage_certificate(json!({
            "parent_seed": parent_seed,
            "parent_device_id": "root",
            "child_public_key": child_pk.clone(),
            "child_device_id": "leaf",
            "family_id": "fam3",
        }))
        .await?;
        let resp: SignLineageCertificateResponse = serde_json::from_value(sign)?;

        let verify = handle_verify_lineage_certificate(json!({
            "certificate": resp.certificate.clone(),
            "expected_family_id": "fam3",
            "trust_anchors": [resp.certificate.clone()],
        }))
        .await?;
        let vr: VerifyLineageCertificateResponse = serde_json::from_value(verify)?;
        assert!(vr.valid);
        assert!(vr.details.chain_verified);
        Ok(())
    }

    #[tokio::test]
    async fn test_verify_lineage_certificate_trust_anchor_no_match()
    -> Result<(), Box<dyn std::error::Error>> {
        let parent_seed = BASE64.encode([11u8; 32]);
        use ed25519_dalek::SigningKey;
        let child = SigningKey::from_bytes(&[12u8; 32]);
        let child_pk = BASE64.encode(child.verifying_key().to_bytes());

        let sign = handle_sign_lineage_certificate(json!({
            "parent_seed": parent_seed,
            "parent_device_id": "p",
            "child_public_key": child_pk,
            "child_device_id": "c",
            "family_id": "fam4",
        }))
        .await?;
        let resp: SignLineageCertificateResponse = serde_json::from_value(sign)?;

        let other = LineageCertificate {
            version: 1,
            parent_device_id: "x".into(),
            child_device_id: "y".into(),
            child_public_key: BASE64.encode([99u8; 32]),
            family_id: "other".into(),
            issued_at: 0,
            expires_at: None,
            depth: 0,
            parent_signature: BASE64.encode([0u8; 64]),
            parent_public_key: BASE64.encode([0u8; 32]),
        };

        let verify = handle_verify_lineage_certificate(json!({
            "certificate": resp.certificate,
            "expected_family_id": "fam4",
            "trust_anchors": [other],
        }))
        .await?;
        let vr: VerifyLineageCertificateResponse = serde_json::from_value(verify)?;
        assert!(!vr.valid);
        assert!(!vr.details.chain_verified);
        Ok(())
    }
}
