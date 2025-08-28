use beardog_errors::BearDogError;


use std::sync::Arc;
use tracing::{info, warn};

use beardog::{
    security::memory_key_manager::{KeyMetadata, VaultPermissions},
    BearDogConfig, BearDogCore, BearDogResult,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("🚀 BearDog Standalone Crypto Demo - 'Crypto in Your Pocket'");
    info!("📱 Demonstrating portable, self-contained security capabilities");

    let standalone_beardog = initialize_standalone_beardog().await?;

    demo_key_management(&standalone_beardog).await?;

    demo_vault_sharing(&standalone_beardog).await?;

    demo_vault_backup(&standalone_beardog).await?;

    demo_metrics(&standalone_beardog).await?;

    demo_network_effects(&standalone_beardog).await?;

    info!("✅ All demos completed successfully!");
    info!("🔐 BearDog provides portable crypto capabilities that work anywhere!");

    Ok(())
}

async fn initialize_standalone_beardog() -> Result<Arc<BearDogCore, BearDogError>> {
    info!("🔧 Initializing standalone BearDog with memory key manager");

    let mut config = BearDogConfig::default();
    config.app.standalone_mode = true;

    let beardog_core = Arc::new(BearDogCore::new(config).await?);

    beardog_core.start().await?;

    let security_provider = beardog_core.security_provider();
    if security_provider.is_standalone_mode() {
        info!("✅ BearDog initialized in standalone mode with memory key manager");
    } else {
        warn!("⚠️ BearDog not in standalone mode - memory key manager disabled");
    }

    Ok(beardog_core)
}

async fn demo_key_management(beardog: &Arc<BearDogCore>) -> Result<(), BearDogError> {
    info!("🔑 Demo: In-Memory Key Management");

    let security_provider = beardog.security_provider();

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

    let aes_material = security_provider.get_key(&aes_key).await?;
    info!(
        "✅ Retrieved AES key (length: {} bytes)",
        aes_material.len()
    );

    let custom_key_material = b"my_custom_32_byte_key_material!!".to_vec();
    let custom_metadata = KeyMetadata {
        key_type: "Custom".to_string(),
        purpose: "demo".to_string(),
        algorithm: "Custom-XOR".to_string(),
        key_size: 256,
        owner_id: "user_123".to_string(),
        tags: vec!["demo".to_string(), "custom".to_string()],
        attributes: std::collections::HashMap::with_capacity(16),
    };

    let custom_key_id = security_provider
        .store_key(&custom_key_material, custom_metadata)
        .await?;
    info!("✅ Stored custom key: {}", custom_key_id);

    let keys = security_provider.list_keys().await?;
    info!("📋 Current keys in vault:");
    for key in &keys {
        info!(
            "  - {} ({}) - {} - accessed {} times",
            key.id, key.key_type, key.purpose, key.access_count
        );
    }

    security_provider.delete_key(&hmac_key).await?;
    info!("🗑️ Deleted HMAC key: {}", hmac_key);

    info!("✅ Key management demo completed");
    Ok(())
}

async fn demo_vault_sharing(beardog: &Arc<BearDogCore>) -> Result<(), BearDogError> {
    info!("🤝 Demo: Vault Sharing for Network Effects");

    let security_provider = beardog.security_provider();

    let permissions = VaultPermissions {
        read: true,
        write: false,
        share: false,
        manage: false,
    };

    let remote_endpoint = "https://beardog-remote.example.com:8765";
    let vault_id = security_provider
        .share_vault_with(remote_endpoint, permissions)
        .await?;

    info!("✅ Shared vault with remote BearDog: {}", vault_id);
    info!("🔗 Remote endpoint: {}", remote_endpoint);
    info!("📋 Permissions: read=true, write=false, share=false, manage=false");

    info!("✅ Vault sharing demo completed");
    Ok(())
}

async fn demo_vault_backup(beardog: &Arc<BearDogCore>) -> Result<(), BearDogError> {
    info!("📦 Demo: Vault Backup and Restore");

    let security_provider = beardog.security_provider();

    let backup_password = "secure_backup_password_123";
    let exported_vault = security_provider.export_vault(backup_password).await?;

    info!("✅ Vault exported ({} bytes)", exported_vault.len());
    info!("🔒 Encrypted with password for secure backup");

    let imported_count = security_provider
        .import_vault(&exported_vault, backup_password)
        .await?;
    info!("✅ Vault imported ({} keys)", imported_count);

    info!("💡 Vault backup enables:");
    info!("  - Hardware migration");
    info!("  - Disaster recovery");
    info!("  - Secure sharing");
    info!("  - Offline backups");

    info!("✅ Vault backup demo completed");
    Ok(())
}

async fn demo_metrics(beardog: &Arc<BearDogCore>) -> Result<(), BearDogError> {
    info!("📊 Demo: Metrics and Monitoring");

    let security_provider = beardog.security_provider();

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

    info!("✅ Metrics demo completed");
    Ok(())
}

async fn demo_network_effects(beardog: &Arc<BearDogCore>) -> Result<(), BearDogError> {
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
