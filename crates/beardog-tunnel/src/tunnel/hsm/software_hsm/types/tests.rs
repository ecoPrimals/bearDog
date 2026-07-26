// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::tunnel::hsm::software_hsm::KeyMetadata;
use crate::tunnel::hsm::types::{KeyStorageType, KeyType, MemoryProtectionLevel};

use super::*;

#[test]
fn test_protected_memory_creation() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1, 2, 3, 4, 5];
    let protected = ProtectedMemory::new(data.clone(), true);

    assert!(protected.is_protected());
    assert_eq!(protected.data(), &data);
    Ok(())
}

#[test]
fn test_software_key_creation() -> Result<(), Box<dyn std::error::Error>> {
    let key_material = ProtectedMemory::new(vec![0u8; 32], true);
    let metadata = KeyMetadata::new("test-key".to_string(), KeyType::Ed25519);

    let key = SoftwareKey::new("test-key".to_string(), KeyType::Aes, key_material, metadata);

    assert_eq!(key.id(), "test-key");
    assert_eq!(key.key_type(), &KeyType::Aes);
    Ok(())
}

#[tokio::test]
async fn test_memory_storage_backend() -> Result<(), Box<dyn std::error::Error>> {
    let backend = StorageBackend::Memory(MemoryStorageBackend::new()?);

    backend.initialize().await?;

    let key_id = "test-key";
    let key_data = vec![1, 2, 3, 4, 5];

    backend.store(key_id, &key_data).await?;

    let retrieved = backend.retrieve(key_id).await?;
    assert_eq!(retrieved, key_data);

    let keys = backend.list_keys().await?;
    assert_eq!(keys.len(), 1);
    assert!(keys.contains(&key_id.to_string()));

    backend.delete(key_id).await?;

    let result = backend.retrieve(key_id).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_memory_storage_backup_restore() -> Result<(), Box<dyn std::error::Error>> {
    let backend = StorageBackend::Memory(MemoryStorageBackend::new()?);

    backend.store("key1", &[1, 2, 3]).await?;
    backend.store("key2", &[4, 5, 6]).await?;

    let backup = backend.backup().await?;

    let backend2 = StorageBackend::Memory(MemoryStorageBackend::new()?);
    backend2.restore(&backup).await?;

    let key1 = backend2.retrieve("key1").await?;
    let key2 = backend2.retrieve("key2").await?;

    assert_eq!(key1, vec![1, 2, 3]);
    assert_eq!(key2, vec![4, 5, 6]);
    Ok(())
}

#[tokio::test]
async fn test_default_encryption_key() -> Result<(), Box<dyn std::error::Error>> {
    let config = SoftwareHsmConfig {
        storage: SoftwareHsmStorageKind::InMemory,
        memory_protection: MemoryProtectionLevel::Medium,
        key_storage: KeyStorageType::Encrypted,
        audit_logging: true,
        max_keys: Some(100),
    };

    let enc_key = DefaultEncryptionKey::create(&config)?;

    let plaintext = b"Hello, BearDog!";
    let ciphertext = enc_key.encrypt(plaintext).await?;

    assert_ne!(ciphertext, plaintext);
    assert!(ciphertext.len() > plaintext.len()); // includes nonce

    let decrypted = enc_key.decrypt(&ciphertext).await?;
    assert_eq!(decrypted, plaintext);
    Ok(())
}

#[tokio::test]
async fn test_default_encryption_key_invalid_ciphertext() -> Result<(), Box<dyn std::error::Error>>
{
    let config = SoftwareHsmConfig {
        storage: SoftwareHsmStorageKind::InMemory,
        memory_protection: MemoryProtectionLevel::Medium,
        key_storage: KeyStorageType::Encrypted,
        audit_logging: true,
        max_keys: Some(100),
    };

    let enc_key = DefaultEncryptionKey::create(&config)?;

    // Too short
    let result = enc_key.decrypt(&[1, 2, 3]).await;
    assert!(result.is_err());

    // Invalid ciphertext
    let result = enc_key.decrypt(&[0u8; 50]).await;
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_audit_log_filter_default() -> Result<(), Box<dyn std::error::Error>> {
    let filter = AuditLogFilter::default();
    // AuditLogFilter::default() creates a default filter
    assert!(filter.from_time.is_none());
    Ok(())
}

#[test]
fn test_storage_backend_clone() -> Result<(), Box<dyn std::error::Error>> {
    let backend1 = SoftwareHsmStorageKind::InMemory;
    let backend2 = backend1.clone();

    match backend2 {
        SoftwareHsmStorageKind::InMemory => {}
        _ => panic!("Expected InMemory variant"),
    }

    let backend3 = SoftwareHsmStorageKind::File {
        path: "/tmp/keys".to_string(),
    };
    let backend4 = backend3.clone();

    match backend4 {
        SoftwareHsmStorageKind::File { path } => assert_eq!(path, "/tmp/keys"),
        _ => panic!("Expected File variant"),
    }
    Ok(())
}
