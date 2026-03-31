// SPDX-License-Identifier: AGPL-3.0-only

use super::{
    Algorithm, AndroidAttestationService, AndroidDeviceCapabilities, AndroidHealthMonitor,
    AndroidHsmConfig, AndroidKeyParams, AndroidKeyPurpose, AndroidKeystore, AttestationLevel,
    HsmCache, HsmCapabilities, HsmOperation, IOSHsmConfig, KeyType,
};
use std::sync::Arc;

#[test]
fn test_android_key_params_default() -> Result<(), Box<dyn std::error::Error>> {
    let params = AndroidKeyParams::default();
    assert_eq!(params.algorithm, "Ed25519");
    assert_eq!(params.key_size, 256);
    assert!(!params.strongbox_required);
    Ok(())
}

#[test]
fn test_android_key_params_builder() -> Result<(), Box<dyn std::error::Error>> {
    let mut params = AndroidKeyParams::new().set_algorithm("AES256");

    params.set_key_size(256);
    params.set_strongbox_required(true);

    assert_eq!(params.algorithm, "AES256");
    assert_eq!(params.key_size, 256);
    assert!(params.strongbox_required);
    Ok(())
}

#[test]
fn test_android_hsm_config_default() -> Result<(), Box<dyn std::error::Error>> {
    let config = AndroidHsmConfig::default();
    assert!(config.strongbox_enabled);
    assert_eq!(config.security_level, 2);
    Ok(())
}

#[test]
fn test_android_keystore_creation() -> Result<(), Box<dyn std::error::Error>> {
    let config = AndroidHsmConfig::default();
    let keystore = AndroidKeystore::with_stub_transport(config)?;

    assert!(keystore.capabilities.strongbox_available);
    assert!(keystore.capabilities.hardware_backed_keystore);
    Ok(())
}

#[tokio::test]
async fn test_android_attestation_service() -> Result<(), Box<dyn std::error::Error>> {
    let service = AndroidAttestationService::with_stub_transport(AttestationLevel::Hardware);
    assert!(service.enabled);
    assert!(service.initialize().await.is_ok());
    Ok(())
}

#[test]
fn test_hsm_cache_creation() -> Result<(), Box<dyn std::error::Error>> {
    let cache = HsmCache::new();
    // Just ensure it creates without panic
    assert!(Arc::strong_count(&cache.key_metadata) >= 1);
    Ok(())
}

#[tokio::test]
async fn test_android_health_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let monitor = AndroidHealthMonitor::new();
    assert_eq!(monitor.check_interval_seconds, 60);

    let status = monitor.get_health_status().await?;
    assert!(status.is_healthy);
    assert_eq!(status.performance_metrics.operations_per_second, 42.0);
    Ok(())
}

#[test]
fn algorithm_security_bits_and_flags() {
    assert_eq!(Algorithm::Ed25519.security_bits(), 256);
    assert!(Algorithm::Ed25519.is_signature_algorithm());
    assert!(!Algorithm::Ed25519.is_encryption_algorithm());
    assert!(Algorithm::Aes256Gcm.is_encryption_algorithm());
    assert_eq!(Algorithm::EcdsaP384.security_bits(), 384);
    let kt: KeyType = Algorithm::X25519.into();
    assert_eq!(kt, KeyType::X25519);
}

#[test]
fn hsm_capabilities_default() {
    let c = HsmCapabilities::default();
    assert!(c.supports_key_generation && c.supports_signing && c.supports_encryption);
}

#[test]
fn algorithm_security_bits_rsa_and_hkdf() {
    assert_eq!(Algorithm::RsaPss2048.security_bits(), 112);
    assert_eq!(Algorithm::HkdfSha256.security_bits(), 256);
    assert!(!Algorithm::HkdfSha256.is_signature_algorithm());
    assert!(!Algorithm::X25519.is_signature_algorithm());
}

#[test]
fn android_key_params_builder_methods() {
    let mut p = AndroidKeyParams::new().set_algorithm("RSA");
    p.set_key_size(2048);
    p.set_strongbox_required(true);
    assert_eq!(p.algorithm, "RSA");
    assert_eq!(p.key_size, 2048);
    assert!(p.strongbox_required);
}

#[test]
fn android_key_params_set_purposes_and_auth() {
    let mut p = AndroidKeyParams::new();
    p.set_purposes(vec![AndroidKeyPurpose::Encrypt, AndroidKeyPurpose::Decrypt]);
    p.set_user_authentication_required(true);
    p.user_authentication_timeout = Some(30);
    assert_eq!(p.purposes.len(), 2);
    assert!(p.user_authentication_required);
    assert_eq!(p.user_authentication_timeout, Some(30));
}

#[test]
fn ios_hsm_config_default_and_android_device_caps() {
    let ios = IOSHsmConfig::default();
    assert!(ios.secure_enclave_enabled);
    let caps = AndroidDeviceCapabilities {
        strongbox_available: true,
        key_attestation_available: false,
        hardware_backed_keystore: true,
        verified_boot: true,
    };
    assert!(!caps.key_attestation_available);
}

#[test]
fn hsm_operation_json_roundtrip() {
    let op = HsmOperation::Signing {
        key_id: "k1".to_string(),
        algorithm: "ed25519".to_string(),
    };
    let json = serde_json::to_string(&op).expect("ser");
    let back: HsmOperation = serde_json::from_str(&json).expect("de");
    assert!(matches!(back, HsmOperation::Signing { .. }));
}

#[tokio::test]
async fn android_keystore_stub_transport_exercises_logic_without_hardware() {
    let ks = AndroidKeystore::with_stub_transport(AndroidHsmConfig::default()).expect("ks");
    let params = AndroidKeyParams::new();
    ks.test_keystore_access("kid", &params).expect("validation");
    assert!(ks.generate_key("", &params).await.is_err());
    ks.generate_key("kid", &params).await.expect("generate");
    let ct = ks.encrypt("kid", b"plain").await.expect("encrypt");
    let pt = ks.decrypt("kid", &ct).await.expect("decrypt");
    assert_eq!(pt, b"plain");
    let sig = ks.sign("kid", b"msg").await.expect("sign");
    assert!(ks.verify("kid", b"msg", &sig).await.expect("verify"));
    assert!(
        ks.import_key("kid2", b"raw", KeyType::Ed25519)
            .await
            .is_ok()
    );
    let keys = ks.list_keys().await.expect("list");
    assert!(keys.len() >= 2);
    assert!(ks.key_exists("kid").await.expect("exists"));
    assert!(ks.delete_key("kid").await.is_ok());
    assert!(!ks.key_exists("kid").await.expect("gone"));
    assert!(ks.import_key("kid", b"", KeyType::Ed25519).await.is_err());
    let rnd = ks.generate_random_bytes(16).await.expect("rnd");
    assert_eq!(rnd.len(), 16);
    let ch = ks.generate_attestation_challenge(8).expect("chal");
    assert_eq!(ch.len(), 8);
}

#[test]
fn algorithm_maps_to_key_type_variants() {
    let k1: KeyType = Algorithm::RsaPss3072.into();
    assert_eq!(k1, KeyType::Rsa);
    let k2: KeyType = Algorithm::RsaPss4096.into();
    assert_eq!(k2, KeyType::Rsa);
    let k3: KeyType = Algorithm::EccP256.into();
    assert_eq!(k3, KeyType::EllipticCurve);
}
