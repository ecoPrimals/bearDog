// SPDX-License-Identifier: AGPL-3.0-or-later

//! State and key-store resilience fault tests.

use beardog_config::BearDogConfig;
use beardog_core::crypto_service::{BearDogCryptoService, CryptoService, CryptoServiceConfig};
use beardog_security::{MemoryKeyConfig, MemoryKeyManager};
use beardog_types::crypto_service::{
    CryptoAlgorithm, DecryptOptions, EncryptedData, EncryptionMetadata,
};
use std::io::Write;
use std::time::SystemTime;
use tempfile::NamedTempFile;

#[tokio::test]
async fn state_fault_corrupt_config_then_fresh_key_manager_initializes() {
    let mut file = NamedTempFile::new().expect("temp file");
    writeln!(file, "{{{{{{not json").expect("write");
    let path = file.path();
    let load = BearDogConfig::from_file(path);
    assert!(load.is_err(), "corrupt config should not load");

    let mgr = MemoryKeyManager::new(MemoryKeyConfig::default()).expect("fresh key manager");
    let key_id = mgr.generate_key().expect("generate after failed load");
    assert!(key_id.starts_with("key_"));
}

#[tokio::test]
async fn state_fault_corrupted_encrypted_struct_decrypt_errors() {
    let service = BearDogCryptoService::new(CryptoServiceConfig::default()).expect("crypto init");
    let bad = EncryptedData {
        ciphertext: vec![],
        algorithm: CryptoAlgorithm::Aes256Gcm,
        metadata: EncryptionMetadata {
            timestamp: SystemTime::UNIX_EPOCH,
            key_id: Some("k".to_string()),
            nonce: vec![0u8; 12],
            tag: Some(vec![0u8; 16]),
        },
    };
    let r = service
        .decrypt(
            &bad,
            DecryptOptions {
                key_id: "k".to_string(),
                ..Default::default()
            },
        )
        .await;
    assert!(
        r.is_err(),
        "empty ciphertext with fake tag should fail decrypt"
    );
}
