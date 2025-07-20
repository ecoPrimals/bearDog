//! Standalone BearDog Demo - "Crypto in Your Pocket"
//!
//! **Demonstrates portable, self-contained security capabilities**
//!
//! This demo showcases how BearDog can operate completely standalone with:
//! - In-memory key management
//! - Portable crypto operations
//! - Vault sharing between BearDogs
//! - Export/import for backup and migration
//! - Network effects while maintaining independence

use std::sync::Arc;
use tracing::{info, warn};

use beardog::{
    security::memory_key_manager::{KeyMetadata, VaultPermissions},
    BearDogConfig, BearDogCore, BearDogResult,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("🚀 BearDog Standalone Crypto Demo - 'Crypto in Your Pocket'");
    info!("📱 Demonstrating portable, self-contained security capabilities");

    // Demo 1: Initialize standalone BearDog with memory key manager
    let standalone_beardog = initialize_standalone_beardog().await?;

    // Demo 2: Generate and manage keys in memory
    demo_key_management(&standalone_beardog).await?;

    // Demo 3: Vault sharing with another BearDog
    demo_vault_sharing(&standalone_beardog).await?;

    // Demo 4: Export and import vault
    demo_vault_backup(&standalone_beardog).await?;

    // Demo 5: Metrics and monitoring
    demo_metrics(&standalone_beardog).await?;

    // Demo 6: Network effects
    demo_network_effects(&standalone_beardog).await?;

    info!("✅ All demos completed successfully!");
    info!("🔐 BearDog provides portable crypto capabilities that work anywhere!");

    Ok(())
}

/// Initialize a standalone BearDog instance
async fn initialize_standalone_beardog() -> BearDogResult<Arc<BearDogCore>> {
    info!("🔧 Initializing standalone BearDog with memory key manager");

    // Create config with memory key manager enabled
    let mut config = BearDogConfig::default();
    config.app.standalone_mode = true;

    // Create BearDog core
    let beardog_core = Arc::new(BearDogCore::new(config).await?);

    // Start the core
    beardog_core.start().await?;

    // Verify standalone mode
    let security_provider = beardog_core.security_provider();
    if security_provider.is_standalone_mode() {
        info!("✅ BearDog initialized in standalone mode with memory key manager");
    } else {
        warn!("⚠️ BearDog not in standalone mode - memory key manager disabled");
    }

    Ok(beardog_core)
}

/// Demo key management operations
async fn demo_key_management(beardog: &Arc<BearDogCore>) -> BearDogResult<()> {
    info!("🔑 Demo: In-Memory Key Management");

    let security_provider = beardog.security_provider();

    // Generate different types of keys
    let aes_key = security_provider
        .generate_key("AES-256", "file_encryption", "user_123")
        .await?;
    info!("✅ Generated AES-256 key: {}", aes_key);

    let hmac_key = security_provider
        .generate_key("HMAC-SHA256", "message_auth", "user_123")
        .await?;
    info!("✅ Generated HMAC-SHA256 key: {}", hmac_key);

    let chacha_key = security_provider
        .generate_key("ChaCha20", "stream_cipher", "user_123")
        .await?;
    info!("✅ Generated ChaCha20 key: {}", chacha_key);

    // Retrieve keys
    let aes_material = security_provider.get_key(&aes_key).await?;
    info!(
        "✅ Retrieved AES key (length: {} bytes)",
        aes_material.len()
    );

    // Store a custom key
    let custom_key_material = b"my_custom_32_byte_key_material!!".to_vec();
    let custom_metadata = KeyMetadata {
        key_type: "Custom".to_string(),
        purpose: "demo".to_string(),
        algorithm: "Custom-XOR".to_string(),
        key_size: 256,
        owner_id: "user_123".to_string(),
        tags: vec!["demo".to_string(), "custom".to_string()],
        attributes: std::collections::HashMap::new(),
    };

    let custom_key_id = security_provider
        .store_key(&custom_key_material, custom_metadata)
        .await?;
    info!("✅ Stored custom key: {}", custom_key_id);

    // List all keys
    let keys = security_provider.list_keys().await?;
    info!("📋 Current keys in vault:");
    for key in &keys {
        info!(
            "  - {} ({}) - {} - accessed {} times",
            key.id, key.key_type, key.purpose, key.access_count
        );
    }

    // Delete a key
    security_provider.delete_key(&hmac_key).await?;
    info!("🗑️ Deleted HMAC key: {}", hmac_key);

    info!("✅ Key management demo completed");
    Ok(())
}

/// Demo vault sharing with another BearDog
async fn demo_vault_sharing(beardog: &Arc<BearDogCore>) -> BearDogResult<()> {
    info!("🤝 Demo: Vault Sharing for Network Effects");

    let security_provider = beardog.security_provider();

    // Create permissions for sharing
    let permissions = VaultPermissions {
        read: true,
        write: false,
        share: false,
        manage: false,
    };

    // Simulate sharing with another BearDog
    let remote_endpoint = "https://beardog-remote.example.com:8765";
    let vault_id = security_provider
        .share_vault_with(remote_endpoint, permissions)
        .await?;

    info!("✅ Shared vault with remote BearDog: {}", vault_id);
    info!("🔗 Remote endpoint: {}", remote_endpoint);
    info!("📋 Permissions: read=true, write=false, share=false, manage=false");

    // In a real scenario, the remote BearDog would:
    // 1. Receive the vault sharing invitation
    // 2. Authenticate with the provided token
    // 3. Access shared keys based on permissions
    // 4. Contribute its own keys to the shared vault

    info!("✅ Vault sharing demo completed");
    Ok(())
}

/// Demo vault backup and restore
async fn demo_vault_backup(beardog: &Arc<BearDogCore>) -> BearDogResult<()> {
    info!("📦 Demo: Vault Backup and Restore");

    let security_provider = beardog.security_provider();

    // Export vault with password
    let backup_password = "secure_backup_password_123";
    let exported_vault = security_provider.export_vault(backup_password).await?;

    info!("✅ Vault exported ({} bytes)", exported_vault.len());
    info!("🔒 Encrypted with password for secure backup");

    // Simulate importing to another BearDog
    // In real use, this would be done on a different BearDog instance
    let imported_count = security_provider
        .import_vault(&exported_vault, backup_password)
        .await?;
    info!("✅ Vault imported ({} keys)", imported_count);

    // Use cases for vault backup:
    // 1. Migrating to new hardware
    // 2. Disaster recovery
    // 3. Sharing vault with trusted parties
    // 4. Creating secure backups

    info!("💡 Vault backup enables:");
    info!("  - Hardware migration");
    info!("  - Disaster recovery");
    info!("  - Secure sharing");
    info!("  - Offline backups");

    info!("✅ Vault backup demo completed");
    Ok(())
}

/// Demo metrics and monitoring
async fn demo_metrics(beardog: &Arc<BearDogCore>) -> BearDogResult<()> {
    info!("📊 Demo: Metrics and Monitoring");

    let security_provider = beardog.security_provider();

    // Get key manager metrics
    let metrics = security_provider.get_key_manager_metrics().await?;

    info!("📈 Key Manager Metrics:");
    info!("  - Total keys: {}", metrics.total_keys);
    info!("  - Memory usage: {} bytes", metrics.memory_usage_bytes);
    info!("  - Shared vaults: {}", metrics.shared_vaults);
    info!("  - Cache hit rate: {:.2}%", metrics.cache_hit_rate * 100.0);

    info!("📋 Keys by type:");
    for (key_type, count) in &metrics.keys_by_type {
        info!("  - {}: {}", key_type, count);
    }

    // These metrics can be exported to monitoring systems:
    // - Prometheus
    // - Grafana
    // - DataDog
    // - Custom monitoring

    info!("✅ Metrics demo completed");
    Ok(())
}

/// Demo network effects
async fn demo_network_effects(beardog: &Arc<BearDogCore>) -> BearDogResult<()> {
    info!("🌐 Demo: Network Effects");

    info!("🔗 BearDog enables powerful network effects:");
    info!("  1. **Vault Sharing** - Share keys with trusted BearDogs");
    info!("  2. **Distributed Security** - Multiple BearDogs provide redundancy");
    info!("  3. **Ecosystem Integration** - Works with Songbird, NestGate, ToadStool");
    info!("  4. **Collaborative Workflows** - Multi-party approval processes");
    info!("  5. **Threat Intelligence** - Shared threat detection across network");

    info!("🏗️ Architecture Benefits:");
    info!("  - **Standalone**: Each BearDog works independently");
    info!("  - **Interoperable**: Standard protocols for communication");
    info!("  - **Scalable**: Add more BearDogs without central coordination");
    info!("  - **Resilient**: Network failure doesn't break individual BearDogs");

    info!("💡 Use Cases:");
    info!("  - **Personal Security**: Crypto in your pocket");
    info!("  - **Team Collaboration**: Shared security across teams");
    info!("  - **Enterprise**: Distributed security infrastructure");
    info!("  - **IoT**: Secure edge computing");

    info!("✅ Network effects demo completed");
    Ok(())
}

/// Demo data structure showing network topology
#[derive(Debug, serde::Serialize)]
struct NetworkTopology {
    beardog_instances: Vec<BearDogInstance>,
    shared_vaults: Vec<SharedVaultConnection>,
    ecosystem_integrations: Vec<EcosystemIntegration>,
}

#[derive(Debug, serde::Serialize)]
struct BearDogInstance {
    id: String,
    endpoint: String,
    capabilities: Vec<String>,
    status: String,
}

#[derive(Debug, serde::Serialize)]
struct SharedVaultConnection {
    vault_id: String,
    participants: Vec<String>,
    permissions: String,
}

#[derive(Debug, serde::Serialize)]
struct EcosystemIntegration {
    service: String,
    integration_type: String,
    status: String,
}
