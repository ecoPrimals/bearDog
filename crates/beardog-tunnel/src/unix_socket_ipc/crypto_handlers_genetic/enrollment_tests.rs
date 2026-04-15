// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use ed25519_dalek::SigningKey;

#[tokio::test]
async fn test_derive_device_seed() -> Result<(), Box<dyn std::error::Error>> {
    let root_seed = BASE64.encode(b"genesis_root_seed_32byteslong!!!");
    let device_entropy = BASE64.encode(b"pixel8a_entropy_data_here!!!_32!");

    let params = json!({
        "root_seed": root_seed,
        "device_entropy": device_entropy,
        "device_id": "pixel8a",
    });

    let result = handle_derive_device_seed(&params).await?;
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

    let result1 = handle_derive_device_seed(&json!({
        "root_seed": root_seed.clone(),
        "device_entropy": entropy1,
        "device_id": "usb-desktop",
    }))
    .await?;

    let result2 = handle_derive_device_seed(&json!({
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
        serde_json::from_value(handle_derive_device_seed(&params).await?)?;
    let resp2: DeriveDeviceSeedResponse =
        serde_json::from_value(handle_derive_device_seed(&params).await?)?;

    assert_eq!(
        resp1.device_seed, resp2.device_seed,
        "Same inputs should produce same device seed"
    );

    Ok(())
}

#[tokio::test]
async fn test_sign_and_verify_lineage_certificate() -> Result<(), Box<dyn std::error::Error>> {
    let parent_seed = BASE64.encode(b"parent_device_seed_32_bytes!!!!!");

    let child_signing_key = SigningKey::from_bytes(&[42u8; 32]);
    let child_public_key = BASE64.encode(child_signing_key.verifying_key().to_bytes());

    let sign_result = handle_sign_lineage_certificate(&json!({
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

    let verify_result = handle_verify_lineage_certificate(&json!({
        "certificate": sign_response.certificate,
        "expected_family_id": "8ff3b864a4bc589a",
        "trust_anchors": [],
    }))
    .await?;

    let verify_response: VerifyLineageCertificateResponse = serde_json::from_value(verify_result)?;

    assert!(verify_response.valid, "Certificate should be valid");
    assert!(verify_response.details.signature_valid);
    assert!(verify_response.details.not_expired);
    assert!(verify_response.details.family_id_matches);

    Ok(())
}

#[tokio::test]
async fn test_verify_lineage_certificate_wrong_family() -> Result<(), Box<dyn std::error::Error>> {
    let parent_seed = BASE64.encode(b"parent_device_seed_for_test!!!!!");

    let child_key = SigningKey::from_bytes(&[99u8; 32]);
    let child_pubkey = BASE64.encode(child_key.verifying_key().to_bytes());

    let sign_result = handle_sign_lineage_certificate(&json!({
        "parent_seed": parent_seed,
        "parent_device_id": "device-a",
        "child_public_key": child_pubkey,
        "child_device_id": "device-b",
        "family_id": "family_aaa",
    }))
    .await?;
    let sign_response: SignLineageCertificateResponse = serde_json::from_value(sign_result)?;

    let verify_result = handle_verify_lineage_certificate(&json!({
        "certificate": sign_response.certificate,
        "expected_family_id": "family_bbb",
        "trust_anchors": [],
    }))
    .await?;
    let verify_response: VerifyLineageCertificateResponse = serde_json::from_value(verify_result)?;

    assert!(!verify_response.valid);
    assert!(!verify_response.details.family_id_matches);

    Ok(())
}

#[tokio::test]
async fn test_derive_device_seed_invalid_root_seed_length() {
    let short_seed = BASE64.encode(b"only_16_bytes!!");
    let device_entropy = BASE64.encode(b"device_entropy_data_32_bytes!!!!");

    let result = handle_derive_device_seed(&json!({
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
        let entropy = BASE64.encode(format!("entropy_for_device_{:02}_padding!!", i).as_bytes());
        let device_id = format!("device-{}", i);

        tasks.spawn(async move {
            handle_derive_device_seed(&json!({
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
        .map(|r| {
            r.get("device_seed")
                .expect("derive response includes device_seed")
                .as_str()
                .expect("device_seed is a JSON string")
        })
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
    let genesis_result = handle_derive_device_seed(&json!({
        "root_seed": root_seed.clone(),
        "device_entropy": BASE64.encode(b"genesis_device_hardware_entropy!"),
        "device_id": "genesis-device",
    }))
    .await?;
    let genesis_seed: DeriveDeviceSeedResponse = serde_json::from_value(genesis_result)?;

    // USB Tower derives its device seed
    let usb_result = handle_derive_device_seed(&json!({
        "root_seed": root_seed.clone(),
        "device_entropy": BASE64.encode(b"usb_tower_hardware_entropy_here!"),
        "device_id": "usb-tower",
    }))
    .await?;
    let usb_seed: DeriveDeviceSeedResponse = serde_json::from_value(usb_result)?;

    assert_ne!(genesis_seed.device_seed, usb_seed.device_seed);

    // Genesis signs certificate for USB Tower
    let usb_signing_key = SigningKey::from_bytes(
        &(BASE64.decode(&usb_seed.device_seed)?[..32]
            .try_into()
            .expect("decoded device seed yields 32-byte signing key material")),
    );
    let usb_pubkey = BASE64.encode(usb_signing_key.verifying_key().to_bytes());

    let cert_result = handle_sign_lineage_certificate(&json!({
        "parent_seed": genesis_seed.device_seed,
        "parent_device_id": "genesis-device",
        "child_public_key": usb_pubkey,
        "child_device_id": "usb-tower",
        "family_id": "test-family-e2e",
    }))
    .await?;
    let usb_cert: SignLineageCertificateResponse = serde_json::from_value(cert_result)?;

    let verify_result = handle_verify_lineage_certificate(&json!({
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
    let r = handle_derive_device_seed(&json!("not-an-object")).await;
    assert!(r.is_err());
}

#[tokio::test]
async fn test_derive_device_seed_short_device_entropy() {
    let r = handle_derive_device_seed(&json!({
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
        handle_derive_device_seed(&json!({
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
        handle_derive_device_seed(&json!({
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
    let child = SigningKey::from_bytes(&[6u8; 32]);
    let child_pk = BASE64.encode(child.verifying_key().to_bytes());

    let sign = handle_sign_lineage_certificate(&json!({
        "parent_seed": parent_seed,
        "parent_device_id": "p",
        "child_public_key": child_pk,
        "child_device_id": "c",
        "family_id": "fam",
        "expires_at": 1u64,
    }))
    .await?;
    let resp: SignLineageCertificateResponse = serde_json::from_value(sign)?;

    let verify = handle_verify_lineage_certificate(&json!({
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
    let child = SigningKey::from_bytes(&[8u8; 32]);
    let child_pk = BASE64.encode(child.verifying_key().to_bytes());

    let sign = handle_sign_lineage_certificate(&json!({
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

    let verify = handle_verify_lineage_certificate(&json!({
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
    let child = SigningKey::from_bytes(&[10u8; 32]);
    let child_pk = BASE64.encode(child.verifying_key().to_bytes());

    let sign = handle_sign_lineage_certificate(&json!({
        "parent_seed": parent_seed,
        "parent_device_id": "root",
        "child_public_key": child_pk.clone(),
        "child_device_id": "leaf",
        "family_id": "fam3",
    }))
    .await?;
    let resp: SignLineageCertificateResponse = serde_json::from_value(sign)?;

    let verify = handle_verify_lineage_certificate(&json!({
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
    let child = SigningKey::from_bytes(&[12u8; 32]);
    let child_pk = BASE64.encode(child.verifying_key().to_bytes());

    let sign = handle_sign_lineage_certificate(&json!({
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

    let verify = handle_verify_lineage_certificate(&json!({
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
