// SPDX-License-Identifier: AGPL-3.0-only
#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

//! # 🌍 **Cross-Platform HSM Unity Demo**
//!
//! ## **The Ultimate Proof**: Same Code, Different Planets
//!
//! This example demonstrates `BearDog`'s vendor-agnostic HSM architecture at its finest:
//! - **`SoloKeys`** (USB, FIDO2/CTAP2 protocol) 🔑
//! - **Pixel 8a** (Mobile, Android Keystore/StrongBox) 📱
//!
//! **The exact same application code works with both!**
//!
//! ## What This Proves
//!
//! 1. **Protocol Abstraction**: FIDO2 and Android Keystore are completely different,
//!    but the application doesn't know or care!
//!
//! 2. **Hardware Independence**: USB security keys and mobile TPMs use the same traits
//!
//! 3. **Future-Proof**: Add new hardware (TPM 2.0, iOS Secure Enclave) without
//!    changing application code!
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │   This Application Code (ONE CODEBASE)      │
//! │   Works with BOTH devices simultaneously!   │
//! └────────────┬──────────────────────┬─────────┘
//!              │                      │
//!    MultiCredentialHsmProvider   MultiCredentialHsmProvider
//!              │                      │
//!      ┌───────▼────────┐     ┌──────▼──────────┐
//!      │ FIDO2 Provider │     │ StrongBox       │
//!      │ (SoloKeys)     │     │ Provider        │
//!      │                │     │ (Pixel 8a)      │
//!      │ CTAP2 Protocol │     │ Android Keystore│
//!      │ USB HID        │     │ Titan M2 chip   │
//!      └────────────────┘     └─────────────────┘
//!          Desktop                  Mobile
//!          Hardware               Hardware
//! ```
//!
//! ## Usage
//!
//! ```bash
//! # Desktop with SoloKeys
//! cargo run --example cross_platform_hsm_unity --features fido2
//!
//! # Android with Pixel 8a (via Termux or similar)
//! cargo run --example cross_platform_hsm_unity --target aarch64-linux-android
//! ```

// Feature-gated imports - only compile with fido2 feature
#[cfg(feature = "fido2")]
use beardog_security::hsm::fido2::multi_credential_provider::{
    Fido2MultiCredentialProvider, Fido2ProviderConfig,
};

#[cfg(feature = "fido2")]
use beardog_traits::unified::{CredentialNode, CredentialRequest, MultiCredentialHsmProvider};

#[cfg(feature = "fido2")]
use std::collections::HashMap;

#[cfg(feature = "fido2")]
/// Generic function that works with ANY HSM provider
///
/// This function is vendor-agnostic and works identically with:
/// - `SoloKeys` (FIDO2)
/// - Pixel 8a (Android `StrongBox`)
/// - `YubiKey` (FIDO2 or PKCS#11)
/// - TPM 2.0 (future)
/// - Any other hardware!
async fn demonstrate_multi_credential_operations<P: MultiCredentialHsmProvider>(
    provider: &P,
    device_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔐 Testing Multi-Credential Operations on: {device_name}");
    println!("═══════════════════════════════════════════════════════");

    // Get device capabilities
    let caps = provider.get_multi_credential_capabilities();
    println!("\n📊 Device Capabilities:");
    println!("   Protocol: {:?}", caps.protocol);
    println!("   Max Credentials: {:?}", caps.max_credentials);
    println!(
        "   Hierarchical: {}",
        caps.supports_hierarchical_credentials
    );
    println!("   Hardware Entropy: {}", caps.supports_hardware_entropy);
    println!("   Algorithms: {:?}", caps.supported_algorithms);

    // Create multiple roles
    println!("\n👥 Creating multiple roles...");

    // Admin role
    println!("   Creating ADMIN credential...");
    let admin_request = CredentialRequest {
        role: "admin".to_string(),
        display_name: Some(format!("{device_name} Administrator")),
        permissions: vec!["read".to_string(), "write".to_string(), "admin".to_string()],
        require_user_presence: true,
        require_user_verification: false,
        parent_credential: None,
        metadata: HashMap::from([
            ("device".to_string(), device_name.to_string()),
            ("created_by".to_string(), "cross_platform_demo".to_string()),
        ]),
        algorithm: Some("ES256".to_string()),
    };

    match provider.create_credential(admin_request).await {
        Ok(admin_cred) => {
            println!("      ✅ Admin credential: {}", admin_cred.credential_id);
            println!("         Permissions: {:?}", admin_cred.permissions);

            // Create operator (child of admin)
            println!("   Creating OPERATOR credential (child of admin)...");
            let operator_request = CredentialRequest {
                role: "operator".to_string(),
                display_name: Some(format!("{device_name} Operator")),
                permissions: vec!["read".to_string(), "write".to_string()],
                require_user_presence: false,
                require_user_verification: false,
                parent_credential: Some(admin_cred.credential_id.clone()),
                metadata: HashMap::from([
                    ("device".to_string(), device_name.to_string()),
                    ("parent".to_string(), "admin".to_string()),
                ]),
                algorithm: Some("ES256".to_string()),
            };

            match provider.create_credential(operator_request).await {
                Ok(operator_cred) => {
                    println!(
                        "      ✅ Operator credential: {}",
                        operator_cred.credential_id
                    );
                    println!(
                        "         Parent: {}",
                        operator_cred
                            .parent_credential_id
                            .as_deref()
                            .unwrap_or("none")
                    );

                    // List all credentials
                    println!("\n📋 Listing all credentials...");
                    match provider.list_credentials().await {
                        Ok(creds) => {
                            println!("   Found {} credentials:", creds.len());
                            for (idx, cred) in creds.iter().enumerate() {
                                println!(
                                    "      {}. {} ({})",
                                    idx + 1,
                                    cred.role,
                                    cred.credential_id
                                );
                                println!("         Permissions: {:?}", cred.permissions);
                                if let Some(parent) = &cred.parent_credential_id {
                                    println!("         Parent: {parent}");
                                }
                            }
                        }
                        Err(e) => println!("   ⚠️  Phase 2: {e}"),
                    }

                    // Test hardware entropy
                    if caps.supports_hardware_entropy {
                        println!("\n🎲 Generating hardware entropy...");
                        match provider.generate_hardware_entropy(32).await {
                            Ok(entropy) => {
                                println!(
                                    "   ✅ Generated 32 bytes: {}",
                                    hex::encode(&entropy[..8])
                                );
                                println!("      (This is TRUE random from hardware chip!)");
                            }
                            Err(e) => println!("   ⚠️  Phase 2: {e}"),
                        }
                    }

                    // Test hierarchical view
                    println!("\n🌳 Credential Hierarchy:");
                    match provider.get_credential_hierarchy().await {
                        Ok(hierarchy) => {
                            fn print_tree(node: &CredentialNode, depth: usize) {
                                let indent = "  ".repeat(depth);
                                println!(
                                    "{}├─ {} ({})",
                                    indent, node.credential.role, node.credential.credential_id
                                );
                                for child in &node.children {
                                    print_tree(child, depth + 1);
                                }
                            }

                            for root in &hierarchy.roots {
                                print_tree(root, 0);
                            }
                        }
                        Err(e) => println!("   ⚠️  {e}"),
                    }
                }
                Err(e) => println!("      ⚠️  Phase 2: {e}"),
            }
        }
        Err(e) => println!("      ⚠️  Phase 2: {e}"),
    }

    Ok(())
}

#[cfg(feature = "fido2")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 BearDog Cross-Platform HSM Unity Demo");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("Demonstrating vendor-agnostic multi-credential operations");
    println!("across completely different hardware and protocols!");
    println!();

    // ========================================================================
    // FIDO2 Devices (SoloKeys, YubiKey, Nitrokey, etc.)
    // ========================================================================

    #[cfg(feature = "fido2")]
    {
        use beardog_security::hsm::fido2::{Fido2MultiCredentialProvider, discover_fido2_devices};

        println!("🔍 Discovering FIDO2 devices (SoloKeys, YubiKey, etc.)...");
        match discover_fido2_devices().await {
            Ok(devices) => {
                if devices.is_empty() {
                    println!("   ⚠️  No FIDO2 devices found");
                } else {
                    println!("   ✅ Found {} FIDO2 device(s)", devices.len());

                    for device in devices {
                        if device.capabilities.resident_keys {
                            let device_name = format!("{} {}", device.manufacturer, device.product);

                            println!("\n📱 Device: {device_name}");
                            println!("   Path: {}", device.device_path.display());
                            println!("   Protocol: {:?}", device.protocol_versions);

                            match Fido2MultiCredentialProvider::new(device, None).await {
                                Ok(provider) => {
                                    // Call the generic function with FIDO2 provider
                                    demonstrate_multi_credential_operations(
                                        &provider,
                                        &device_name,
                                    )
                                    .await?;
                                }
                                Err(e) => println!("   ❌ Failed to create provider: {e}"),
                            }
                        }
                    }
                }
            }
            Err(e) => println!("   ❌ Discovery failed: {e}"),
        }
    }

    #[cfg(not(feature = "fido2"))]
    {
        println!("ℹ️  FIDO2 support not enabled (compile with --features fido2)");
    }

    // ========================================================================
    // Android StrongBox (Pixel 8a, Samsung, etc.)
    // ========================================================================

    #[cfg(target_os = "android")]
    {
        use beardog_security::hsm::android_strongbox::{
            StrongBoxDeviceInfo, StrongBoxMultiCredentialProvider,
        };

        println!("\n🔍 Detecting Android StrongBox...");
        let device_info = StrongBoxDeviceInfo::default();

        if device_info.strongbox_available {
            let device_name = format!("{} {}", device_info.manufacturer, device_info.model);

            println!("   ✅ StrongBox available!");
            println!("\n📱 Device: {}", device_name);
            println!("   Android: {}", device_info.android_version);
            if let Some(ref titan) = device_info.titan_m_version {
                println!("   Titan M: {}", titan);
            }

            match StrongBoxMultiCredentialProvider::new(device_info, None).await {
                Ok(provider) => {
                    // Call the EXACT SAME generic function with Android provider!
                    demonstrate_multi_credential_operations(&provider, &device_name).await?;
                }
                Err(e) => println!("   ❌ Failed to create provider: {}", e),
            }
        } else {
            println!("   ⚠️  StrongBox not available on this device");
        }
    }

    #[cfg(not(target_os = "android"))]
    {
        println!("\nℹ️  Android StrongBox support requires Android platform");
        println!("   (Compile for Android to test Pixel 8a integration)");
    }

    // ========================================================================
    // Summary
    // ========================================================================

    println!("\n═══════════════════════════════════════════════════════");
    println!("🎉 Cross-Platform Demo Complete!");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("✅ Key Achievements:");
    println!("   • Same application code works with FIDO2 AND Android");
    println!("   • No platform-specific logic in app layer");
    println!("   • Hardware differences completely abstracted");
    println!("   • Multi-credential operations work identically");
    println!();
    println!("🎯 Vendor-Agnostic Philosophy:");
    println!("   • Write once, run on ANY hardware");
    println!("   • SoloKeys, Pixel 8a, YubiKey, TPM, future devices");
    println!("   • Protocol abstraction (FIDO2, Android, PKCS#11, TPM)");
    println!("   • Future-proof architecture");
    println!();
    println!("💡 This is the power of trait-based design!");

    Ok(())
}

#[cfg(not(feature = "fido2"))]
fn main() {
    eprintln!("This example requires the 'fido2' feature.");
    eprintln!("Run with: cargo run --example cross_platform_hsm_unity --features fido2");
    std::process::exit(1);
}
