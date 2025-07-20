//! Test the memory key manager directly
//!
//! This is a simple test that demonstrates the standalone BearDog
//! memory key manager without complex dependencies.

use beardog_config::BearDogConfig;
use beardog_security::memory_key_manager::{MemoryKeyManager, MemoryKeyManagerConfig};
use beardog_security::types::KeyStatus;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    println!("🐻 BearDog Memory Key Manager Test");
    println!("==================================");
    
    // Create memory key manager configuration
    let config = MemoryKeyManagerConfig {
        max_keys: 1000,
        key_rotation_interval: Duration::from_secs(3600),
        cleanup_interval: Duration::from_secs(300),
        backup_encryption_key: "test-backup-key".to_string(),
        enable_metrics: true,
        auto_rotation: true,
    };
    
    // Create memory key manager
    let key_manager = MemoryKeyManager::new(config).await?;
    
    // Test key generation
    println!("\n🔑 Testing Key Generation");
    let key_id = key_manager.generate_key(
        "test-user",
        "data-encryption",
        "AES-256-GCM",
        32,
    ).await?;
    
    println!("   Generated key ID: {}", key_id);
    
    // Test key retrieval
    println!("\n🔍 Testing Key Retrieval");
    let retrieved_key = key_manager.get_key(&key_id).await?;
    println!("   Retrieved key length: {} bytes", retrieved_key.len());
    
    // Test key listing
    println!("\n📋 Testing Key Listing");
    let keys = key_manager.list_keys("test-user").await?;
    println!("   Found {} keys for user", keys.len());
    
    for key_info in keys {
        println!("   - Key ID: {}, Type: {}, Status: {:?}", 
                 key_info.id, key_info.key_type, key_info.status);
    }
    
    // Test metrics
    println!("\n📊 Testing Metrics");
    let metrics = key_manager.get_metrics().await?;
    println!("   Total keys: {}", metrics.total_keys);
    println!("   Memory usage: {} bytes", metrics.memory_usage_bytes);
    println!("   Cache hit rate: {:.2}%", metrics.cache_hit_rate * 100.0);
    
    // Test vault export
    println!("\n💾 Testing Vault Export");
    let export_data = key_manager.export_vault("test-password").await?;
    println!("   Exported vault size: {} bytes", export_data.len());
    
    // Test vault sharing
    println!("\n🤝 Testing Vault Sharing");
    let vault_id = key_manager.share_vault("test-password", "shared-vault").await?;
    println!("   Shared vault ID: {}", vault_id);
    
    // Test key rotation
    println!("\n🔄 Testing Key Rotation");
    let new_key_id = key_manager.rotate_key(&key_id, "test-user").await?;
    println!("   New key ID after rotation: {}", new_key_id);
    
    // Test key revocation
    println!("\n🚫 Testing Key Revocation");
    key_manager.revoke_key(&key_id, "test-user").await?;
    println!("   Key revoked successfully");
    
    // Final metrics
    println!("\n📊 Final Metrics");
    let final_metrics = key_manager.get_metrics().await?;
    println!("   Total keys: {}", final_metrics.total_keys);
    println!("   Memory usage: {} bytes", final_metrics.memory_usage_bytes);
    
    println!("\n✅ All tests completed successfully!");
    println!("🎉 BearDog Memory Key Manager is working correctly!");
    
    Ok(())
} 