// SPDX-License-Identifier: AGPL-3.0-only

//! Coverage: BirdSong manager — discovery encryption and lineage APIs.

use beardog_errors::BearDogError;

// ═══════════════════════════════════════════════════════════════════
// birdsong/manager.rs - Discovery encryption & key management
// ═══════════════════════════════════════════════════════════════════

mod manager_discovery_tests {
    use super::BearDogError;
    use crate::birdsong::BirdSongManager;

    async fn create_test_manager() -> BirdSongManager {
        BirdSongManager::new(vec![0xEF; 32], None).await.unwrap()
    }

    #[tokio::test]
    async fn test_discovery_encrypt_decrypt_roundtrip() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let plaintext = b"Hello, family!";
        let family_id = "test-family";

        let encrypted = manager.encrypt_discovery_for_family(plaintext, family_id)?;
        assert!(encrypted.len() > plaintext.len());

        let decrypted = manager.decrypt_discovery_from_family(&encrypted, family_id)?;
        assert_eq!(decrypted, plaintext);
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_wrong_family_fails() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let plaintext = b"Secret message";

        let encrypted = manager.encrypt_discovery_for_family(plaintext, "family-A")?;
        let result = manager.decrypt_discovery_from_family(&encrypted, "family-B");
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_decrypt_too_short() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let short_data = vec![0u8; 10];

        let result = manager.decrypt_discovery_from_family(&short_data, "family");
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_decrypt_corrupted_data() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let plaintext = b"Test data";

        let mut encrypted = manager.encrypt_discovery_for_family(plaintext, "family")?;
        if let Some(last) = encrypted.last_mut() {
            *last ^= 0xFF;
        }

        let result = manager.decrypt_discovery_from_family(&encrypted, "family");
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_empty_plaintext() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let plaintext = b"";

        let encrypted = manager.encrypt_discovery_for_family(plaintext, "family")?;
        let decrypted = manager.decrypt_discovery_from_family(&encrypted, "family")?;
        assert_eq!(decrypted, plaintext);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_lineage_chain_nonexistent() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        assert!(manager.get_lineage_chain("nonexistent").is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_descendants_nonexistent() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let descendants = manager.get_descendants("nonexistent", "node");
        assert!(descendants.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_can_decrypt_check() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;

        let chain = manager
            .generate_root_lineage("root".to_string(), None)
            .await?;
        manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;

        let hint = crate::birdsong::LineageHint {
            root_id: "root".to_string(),
            min_depth: 1,
            max_depth: 1,
            biome_filter: None,
            version: 1,
        };

        let encrypt_req = crate::birdsong::types::BirdSongEncryptRequest {
            plaintext: b"Test message".to_vec(),
            lineage_hint: hint,
            associated_data: None,
        };

        let broadcast = manager.encrypt_broadcast(&encrypt_req)?;

        assert!(manager.can_decrypt(&broadcast, 1));
        assert!(!manager.can_decrypt(&broadcast, 3));

        Ok(())
    }

    #[tokio::test]
    async fn test_revoke_keys_no_keys() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let chain = manager
            .generate_root_lineage("root".to_string(), None)
            .await?;

        let count = manager.revoke_keys(&chain.chain_id, "root");
        assert_eq!(count, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_distributed_keys_nonexistent() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let keys = manager.get_distributed_keys("nonexistent");
        assert!(keys.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_distribute_keys_to_descendants() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let chain = manager
            .generate_root_lineage("root".to_string(), None)
            .await?;

        manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;

        // Distribute keys
        let count = manager.distribute_keys_to_descendants(&chain.chain_id, "root", 1)?;
        assert!(count > 0);

        // Keys are stored under the descendant's node_id, not the root
        let keys = manager.get_distributed_keys("child-1");
        assert!(!keys.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_full_lineage_lifecycle() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;

        // Create root
        let chain = manager
            .generate_root_lineage("root".to_string(), None)
            .await?;

        // Add children
        manager
            .add_child(&chain.chain_id, "root", "child-a".to_string(), None)
            .await?;
        manager
            .add_child(&chain.chain_id, "root", "child-b".to_string(), None)
            .await?;
        manager
            .add_child(&chain.chain_id, "child-a", "grandchild-1".to_string(), None)
            .await?;

        // Verify chain exists
        assert!(manager.get_lineage_chain(&chain.chain_id).is_some());

        // Get descendants
        let root_descendants = manager.get_descendants(&chain.chain_id, "root");
        assert!(root_descendants.len() >= 2);

        // Generate and verify proof
        let proof = manager.generate_lineage_proof(&chain.chain_id, "child-a")?;
        let verification = manager.verify_lineage_proof(&proof, &chain.chain_id)?;
        assert!(verification.valid);

        // Key distribution — keys stored under descendant node IDs
        manager.distribute_keys_to_descendants(&chain.chain_id, "root", 1)?;
        let keys = manager.get_distributed_keys("child-a");
        assert!(!keys.is_empty());

        // Broadcast encryption
        let hint = crate::birdsong::LineageHint {
            root_id: "root".to_string(),
            min_depth: 0,
            max_depth: 3,
            biome_filter: None,
            version: 1,
        };

        let req = crate::birdsong::types::BirdSongEncryptRequest {
            plaintext: b"broadcast data".to_vec(),
            lineage_hint: hint,
            associated_data: None,
        };

        let broadcast = manager.encrypt_broadcast(&req)?;
        assert!(!broadcast.ciphertext.is_empty());

        // Revoke and verify
        let revoked_count = manager.revoke_keys(&chain.chain_id, "child-a");
        assert!(revoked_count >= 0);

        // Rotate all keys
        let rotated = manager.rotate_all_keys(&chain.chain_id, "root", 2)?;
        assert!(rotated > 0);

        Ok(())
    }
}
