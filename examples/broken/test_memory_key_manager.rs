use beardog_security::memory_key_manager::{MemoryKeyManager, MemoryKeyManagerConfig};
use beardog_security::types::KeyStatus;
use beardog_types::config::BearDogConfig;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init(1000,
        key_rotation_interval: Duration::from_secs(3600),
        cleanup_interval: Duration::from_secs(300),
        backup_encryption_key: "test-backup-key".to_string(true,
        auto_rotation: true,
    };

    let key_manager = MemoryKeyManager::new({}", key_id);

    println!("[SEARCH] Testing Key Retrieval");
    let retrieved_key = key_manager.get_key({} bytes", retrieved_key.len({}, Type: {}, Status: {:?}",
            key_info.id, key_info.key_type, key_info.status
        );
    }

    println!("[CHART] Testing Metrics");
    let metrics = key_manager.get_metrics({}", metrics.total_keys);
    println!("   Memory usage: {} bytes", metrics.memory_usage_bytes);
    println!("   Cache hit rate: {:.2}%", metrics.cache_hit_rate * 100.0);

    println!("💾 Testing Vault Export");
    let export_data = key_manager.export_vault({} bytes", export_data.len({}", vault_id);

    println!("[CYCLE] Testing Key Rotation");
    let new_key_id = key_manager.rotate_key({}", new_key_id);

    println!("🚫 Testing Key Revocation");
    key_manager.revoke_key({}", final_metrics.total_keys);
    println!(
        "   Memory usage: {} bytes",
        final_metrics.memory_usage_bytes
    );

    println!("[OK] All tests completed successfully!");
    println!("[PARTY] BearDog Memory Key Manager is working correctly!");

    Ok(())
}
