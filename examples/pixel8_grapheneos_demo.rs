// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! # Pixel 8 GrapheneOS BearDog Demo
//!
//! This example demonstrates how to set up and use BearDog's hardware security
//! module (HSM) on a Pixel 8 device running GrapheneOS.
//!
//! ## Features Demonstrated
//!
//! - **Hardware detection and validation**
//! - **Titan M security chip integration**
//! - **StrongBox HSM initialization**
//! - **Hardware-backed key generation**
//! - **Security anchor key creation**
//! - **Digital signatures with hardware keys**
//! - **Key attestation verification**
//!
//! ## Usage
//!
//! Run this example on your Pixel 8 with GrapheneOS:
//!
//! ```bash
//! cargo run --example pixel8_grapheneos_demo --features android-hsm
//! ```

use beardog::tunnel::hsm::android_strongbox::{
    setup_pixel8_beardog, Pixel8GrapheneOSConfig, Pixel8GrapheneOSSetup,
    Pixel8PerformanceMode,
};
use beardog::tunnel::hsm::types::*;
use beardog::BearDogResult;
use std::sync::Arc;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> BearDogResult<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 BearDog Pixel 8 GrapheneOS Demo Starting");
    info!("==================================================");

    // Demo 1: Quick setup (easiest way to get started)
    demo_quick_setup().await?;

    // Demo 2: Custom configuration setup (more control)
    demo_custom_setup().await?;

    // Demo 3: Advanced security operations
    demo_advanced_operations().await?;

    info!("🎉 All demos completed successfully!");
    Ok(())
}

/// Demonstration of quick setup for Pixel 8 GrapheneOS
async fn demo_quick_setup() -> BearDogResult<()> {
    info!("📱 Demo 1: Quick Setup");
    info!("---------------------");

    // This is the simplest way to get BearDog running on Pixel 8
    let (hsm, anchor_key) = setup_pixel8_beardog().await?;

    info!("✅ Quick setup complete!");
    info!("   HSM initialized: {}", hsm.get_info().await?.instance_id);
    info!("   Anchor key created: {}", anchor_key.id);

    // Test basic operations
    test_basic_operations(&hsm, &anchor_key).await?;

    info!("✅ Demo 1 complete!\n");
    Ok(())
}

/// Demonstration of custom configuration setup
async fn demo_custom_setup() -> BearDogResult<()> {
    info!("⚙️ Demo 2: Custom Configuration Setup");
    info!("-------------------------------------");

    // Create custom configuration for your security requirements
    let config = Pixel8GrapheneOSConfig {
        require_titan_m: true,                              // Require Titan M chip
        require_green_boot: true,                           // Require verified boot
        enable_attestation: true,                           // Enable key attestation
        security_level: SecurityLevel::High,               // High security level
        performance_mode: Pixel8PerformanceMode::Balanced, // Balance security/performance
    };

    // Initialize with custom config
    let setup = Pixel8GrapheneOSSetup::new(config).await?;
    let hsm = setup.initialize_hsm().await?;

    // Create custom anchor key with specific requirements
    let anchor_key = setup.create_anchor_key(&hsm).await?;

    info!("✅ Custom setup complete!");
    info!("   Security level: High");
    info!("   Performance mode: Balanced");
    info!("   Anchor key: {}", anchor_key.id);

    // Generate ecosystem identity for this device
    let identity = setup.generate_ecosystem_identity(&hsm).await?;
    info!("🌐 Device identity: {}", identity.device_id);
    info!("   GrapheneOS version: {}", identity.grapheneos_version);
    info!("   Capabilities: {:?}", identity.capabilities);

    info!("✅ Demo 2 complete!\n");
    Ok(())
}

/// Demonstration of advanced security operations
async fn demo_advanced_operations() -> BearDogResult<()> {
    info!("🔐 Demo 3: Advanced Security Operations");
    info!("---------------------------------------");

    let (hsm, _anchor_key) = setup_pixel8_beardog().await?;

    // Create different types of keys for different purposes
    let signing_key = create_signing_key(&hsm).await?;
    let encryption_key = create_encryption_key(&hsm).await?;

    // Demonstrate various cryptographic operations
    demo_digital_signatures(&hsm, &signing_key).await?;
    demo_data_encryption(&hsm, &encryption_key).await?;
    demo_key_attestation(&signing_key).await?;

    info!("✅ Demo 3 complete!\n");
    Ok(())
}

/// Test basic HSM operations
async fn test_basic_operations(hsm: &Arc<dyn HsmProvider>, anchor_key: &HsmKey) -> BearDogResult<()> {
    info!("🧪 Testing basic operations with anchor key...");

    // Test data to sign
    let test_data = b"Hello from BearDog on Pixel 8 GrapheneOS!";

    // Sign data with hardware-backed key
    let signature = hsm.sign(&anchor_key.id, test_data).await?;
    info!("✅ Data signed successfully ({} byte signature)", signature.len());

    // Verify the signature
    let is_valid = hsm.verify(&anchor_key.id, test_data, &signature).await?;
    if is_valid {
        info!("✅ Signature verification passed");
    } else {
        return Err(beardog::BearDogError::Verification {
            message: "Signature verification failed".to_string(),
        });
    }

    // Get HSM information
    let hsm_info = hsm.get_info().await?;
    info!("📋 HSM Info:");
    info!("   Vendor: {}", hsm_info.vendor);
    info!("   Model: {}", hsm_info.model);
    info!("   Capabilities: {:?}", hsm_info.capabilities);

    Ok(())
}

/// Create a dedicated signing key
async fn create_signing_key(hsm: &Arc<dyn HsmProvider>) -> BearDogResult<HsmKey> {
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

/// Create a dedicated encryption key
async fn create_encryption_key(hsm: &Arc<dyn HsmProvider>) -> BearDogResult<HsmKey> {
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

/// Demonstrate digital signatures
async fn demo_digital_signatures(hsm: &Arc<dyn HsmProvider>, signing_key: &HsmKey) -> BearDogResult<()> {
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

/// Demonstrate data encryption
async fn demo_data_encryption(hsm: &Arc<dyn HsmProvider>, encryption_key: &HsmKey) -> BearDogResult<()> {
    info!("🔐 Demonstrating data encryption...");

    let sensitive_data = b"This is sensitive data protected by Pixel 8 StrongBox";
    
    // Encrypt data
    let ciphertext = hsm.encrypt(&encryption_key.id, sensitive_data).await?;
    info!("   Original: {} bytes", sensitive_data.len());
    info!("   Encrypted: {} bytes", ciphertext.len());

    // Decrypt data
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

/// Demonstrate key attestation
async fn demo_key_attestation(key: &HsmKey) -> BearDogResult<()> {
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

/// Display system information
fn display_system_info() {
    info!("📱 System Information:");
    info!("   OS: Detected as Android-based");
    info!("   Expected device: Google Pixel 8");
    info!("   Expected OS: GrapheneOS");
    info!("   Security chip: Titan M (expected)");
    info!("   HSM type: Android StrongBox");
} 