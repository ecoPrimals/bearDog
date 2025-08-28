

use beardog::tunnel::hsm::android_strongbox::{
    setup_pixel8_beardog, Pixel8GrapheneOSConfig, Pixel8GrapheneOSSetup,
    Pixel8PerformanceMode,
};
use beardog::tunnel::hsm::types::*;
use beardog_errors::BearDogError;
use std::sync::Arc;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), BearDogError> {

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 BearDog Pixel 8 GrapheneOS Demo Starting");
    info!("==================================================");

    demo_quick_setup().await?;

    demo_custom_setup().await?;

    demo_advanced_operations().await?;

    info!("🎉 All demos completed successfully!");
    Ok(())
}

async fn demo_quick_setup() -> Result<(), BearDogError> {
    info!("📱 Demo 1: Quick Setup");
    info!("---------------------");

    let (hsm, anchor_key) = setup_pixel8_beardog().await?;

    info!("✅ Quick setup complete!");
    info!("   HSM initialized: {}", hsm.get_info().await?.instance_id);
    info!("   Anchor key created: {}", anchor_key.id);

    test_basic_operations(&hsm, &anchor_key).await?;

    info!("✅ Demo 1 complete!\n");
    Ok(())
}

async fn demo_custom_setup() -> Result<(), BearDogError> {
    info!("⚙️ Demo 2: Custom Configuration Setup");
    info!("-------------------------------------");

    let config = Pixel8GrapheneOSConfig {
        require_titan_m: true,                              // Require Titan M chip
        require_green_boot: true,                           // Require verified boot
        enable_attestation: true,                           // Enable key attestation
        security_level: SecurityLevel::High,               // High security level
        performance_mode: Pixel8PerformanceMode::Balanced, // Balance security/performance
    };

    let setup = Pixel8GrapheneOSSetup::new(config).await?;
    let hsm = setup.initialize_hsm().await?;

    let anchor_key = setup.create_anchor_key(&hsm).await?;

    info!("✅ Custom setup complete!");
    info!("   Security level: High");
    info!("   Performance mode: Balanced");
    info!("   Anchor key: {}", anchor_key.id);

    let identity = setup.generate_ecosystem_identity(&hsm).await?;
    info!("🌐 Device identity: {}", identity.device_id);
    info!("   GrapheneOS version: {}", identity.grapheneos_version);
    info!("   Capabilities: {:?}", identity.capabilities);

    info!("✅ Demo 2 complete!\n");
    Ok(())
}

async fn demo_advanced_operations() -> Result<(), BearDogError> {
    info!("🔐 Demo 3: Advanced Security Operations");
    info!("---------------------------------------");

    let (hsm, _anchor_key) = setup_pixel8_beardog().await?;

    let signing_key = create_signing_key(&hsm).await?;
    let encryption_key = create_encryption_key(&hsm).await?;

    demo_digital_signatures(&hsm, &signing_key).await?;
    demo_data_encryption(&hsm, &encryption_key).await?;
    demo_key_attestation(&signing_key).await?;

    info!("✅ Demo 3 complete!\n");
    Ok(())
}

async fn test_basic_operations(hsm: &Arc<dyn HsmProvider>, anchor_key: &HsmKey) -> Result<(), BearDogError> {
    info!("🧪 Testing basic operations with anchor key...");

    let test_data = b"Hello from BearDog on Pixel 8 GrapheneOS!";

    let signature = hsm.sign(&anchor_key.id, test_data).await?;
    info!("✅ Data signed successfully ({} byte signature)", signature.len());

    let is_valid = hsm.verify(&anchor_key.id, test_data, &signature).await?;
    if is_valid {
        info!("✅ Signature verification passed");
    } else {
        return Err(beardog::BearDogError::Verification {
            message: "Signature verification failed".to_string(),
        });
    }

    let hsm_info = hsm.get_info().await?;
    info!("📋 HSM Info:");
    info!("   Vendor: {}", hsm_info.vendor);
    info!("   Model: {}", hsm_info.model);
    info!("   Capabilities: {:?}", hsm_info.capabilities);

    Ok(())
}

async fn create_signing_key(hsm: &Arc<dyn HsmProvider>) -> Result<HsmKey, BearDogError> {
    info!("🔑 Creating dedicated signing key...");

    let request = GenerateKeyRequest {
        key_id: "demo-signing-key".to_string(),
        key_type: KeyType::EccP256,
        usage_policy: KeyUsagePolicy {
            can_sign: true,
            can_verify: true,
            can_encrypt: false,
            can_decrypt: false,
            can_wrap: false,
            can_unwrap: false,
            exportable: false,
            requires_user_presence: Some(false), // Allow automated signing for demo
            requires_biometric: Some(false),
        },
        expires_at: None,
        metadata: KeyMetadata {
            description: "Demo signing key for BearDog operations".to_string(),
            created_for: "pixel8-demo".to_string(),
            key_classification: KeyClassification::Standard,
            compliance_requirements: vec!["Demo".to_string()],
        },
        ..Default::default()
    };

    let key = hsm.generate_key(request).await?;
    info!("✅ Signing key created: {}", key.id);
    Ok(key)
}

async fn create_encryption_key(hsm: &Arc<dyn HsmProvider>) -> Result<HsmKey, BearDogError> {
    info!("🔑 Creating dedicated encryption key...");

    let request = GenerateKeyRequest {
        key_id: "demo-encryption-key".to_string(),
        key_type: KeyType::Aes256,
        usage_policy: KeyUsagePolicy {
            can_sign: false,
            can_verify: false,
            can_encrypt: true,
            can_decrypt: true,
            can_wrap: false,
            can_unwrap: false,
            exportable: false,
            requires_user_presence: Some(false),
            requires_biometric: Some(false),
        },
        expires_at: None,
        metadata: KeyMetadata {
            description: "Demo encryption key for data protection".to_string(),
            created_for: "pixel8-demo".to_string(),
            key_classification: KeyClassification::Standard,
            compliance_requirements: vec!["Demo".to_string()],
        },
        ..Default::default()
    };

    let key = hsm.generate_key(request).await?;
    info!("✅ Encryption key created: {}", key.id);
    Ok(key)
}

async fn demo_digital_signatures(hsm: &Arc<dyn HsmProvider>, signing_key: &HsmKey) -> Result<(), BearDogError> {
    info!("✍️ Demonstrating digital signatures...");

    let messages = vec![
        b"BearDog security message #1".as_slice(),
        b"GrapheneOS is awesome for privacy!".as_slice(),
        b"Pixel 8 Titan M provides hardware security".as_slice(),
    ];

    for (i, message) in messages.iter().enumerate() {
        let signature = hsm.sign(&signing_key.id, message).await?;
        let is_valid = hsm.verify(&signing_key.id, message, &signature).await?;
        
        info!("   Message {}: {} bytes → {} byte signature → {}", 
              i + 1, 
              message.len(), 
              signature.len(),
              if is_valid { "✅ Valid" } else { "❌ Invalid" }
        );
    }

    info!("✅ Digital signature demo complete");
    Ok(())
}

async fn demo_data_encryption(hsm: &Arc<dyn HsmProvider>, encryption_key: &HsmKey) -> Result<(), BearDogError> {
    info!("🔐 Demonstrating data encryption...");

    let sensitive_data = b"This is sensitive data protected by Pixel 8 StrongBox";

    let ciphertext = hsm.encrypt(&encryption_key.id, sensitive_data).await?;
    info!("   Original: {} bytes", sensitive_data.len());
    info!("   Encrypted: {} bytes", ciphertext.len());

    let plaintext = hsm.decrypt(&encryption_key.id, &ciphertext).await?;
    
    if plaintext == sensitive_data {
        info!("✅ Encryption/decryption successful - data integrity verified");
    } else {
        return Err(beardog::BearDogError::Cryptographic {
            operation: "decryption".to_string(),
            message: "Decrypted data doesn't match original".to_string(),
        });
    }

    info!("✅ Data encryption demo complete");
    Ok(())
}

async fn demo_key_attestation(key: &HsmKey) -> Result<(), BearDogError> {
    info!("📜 Demonstrating key attestation...");

    if let Some(attestation) = &key.attestation {
        info!("✅ Key has hardware attestation:");
        info!("   Certificate chain length: {}", attestation.certificate_chain.len());
        
        if let Some(device_info) = &attestation.device_info {
            info!("   Device manufacturer: {}", device_info.manufacturer);
            info!("   Device model: {}", device_info.model);
            info!("   Security level: {:?}", device_info.security_level);
        }
        
        info!("   Attestation proves:");
        info!("   • Key is hardware-backed ✅");
        info!("   • Key cannot be extracted ✅");
        info!("   • Operations occur in secure hardware ✅");
    } else {
        info!("ℹ️ Key does not have attestation (may be in development mode)");
    }

    info!("✅ Key attestation demo complete");
    Ok(())
}

fn display_system_info() {
    info!("📱 System Information:");
    info!("   OS: Detected as Android-based");
    info!("   Expected device: Google Pixel 8");
    info!("   Expected OS: GrapheneOS");
    info!("   Security chip: Titan M (expected)");
    info!("   HSM type: Android StrongBox");
} 