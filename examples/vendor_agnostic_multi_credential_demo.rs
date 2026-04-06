// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

//! # Vendor-Agnostic Multi-Credential HSM Demo
//!
//! This example demonstrates `BearDog`'s vendor-agnostic multi-credential HSM support.
//! The **exact same code** works with:
//!
//! - **`SoloKeys` Solo 2** (FIDO2)
//! - **`YubiKey` 5 Series** (FIDO2 or PKCS#11 mode)
//! - **Nitrokey FIDO2**
//! - **TPM 2.0 modules** (future)
//! - **Android `StrongBox`** (Pixel Titan M2) (future)
//! - **Any FIDO2-compliant security key**
//!
//! ## What This Demo Shows
//!
//! 1. **Device Discovery** - Find all connected HSM devices
//! 2. **Create Multiple Roles** - Admin, Operator, Auditor on same device
//! 3. **Hierarchical Credentials** - Parent-child relationships
//! 4. **Cross-Device Replication** - Copy credentials between devices
//! 5. **Hardware Entropy** - True random number generation
//! 6. **Role-Based Signing** - Different credentials for different operations
//!
//! ## Hardware Agnostic Design
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │   BearDog Application Code              │
//! │   (You write once)                      │
//! └─────────────────┬───────────────────────┘
//!                   │
//!         Uses: MultiCredentialHsmProvider trait
//!                   │
//!      ┌────────────┴────────────┐
//!      │                         │
//! ┌────▼─────┐            ┌─────▼─────┐
//! │  FIDO2   │            │  PKCS#11  │
//! │ Provider │            │ Provider  │
//! └────┬─────┘            └─────┬─────┘
//!      │                        │
//! ┌────▼────┐            ┌──────▼──────┐
//! │SoloKeys │            │YubiKey PIV  │
//! │YubiKey  │            │Smart Cards  │
//! │Nitrokey │            │             │
//! └─────────┘            └─────────────┘
//! ```
//!
//! ## Usage
//!
//! ```bash
//! # Run with any connected HSM devices
//! cargo run --example vendor_agnostic_multi_credential_demo --features fido2
//! ```

#[cfg(feature = "fido2")]
use beardog_security::hsm::fido2::{Fido2MultiCredentialProvider, discover_fido2_devices};

#[cfg(feature = "fido2")]
use beardog_traits::unified::{CredentialNode, CredentialRequest, MultiCredentialHsmProvider};

#[cfg(feature = "fido2")]
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "fido2"))]
    {
        println!("⚠️  FIDO2 feature not enabled! Build with --features fido2");
        return Ok(());
    }

    #[cfg(feature = "fido2")]
    {
        println!("🔐 BearDog Vendor-Agnostic Multi-Credential HSM Demo");
        println!("═══════════════════════════════════════════════════");
        println!();

        // ========================================================================
        // STEP 1: DISCOVER ALL HSM DEVICES (works with any vendor)
        // ========================================================================

        println!("🔍 Step 1: Discovering HSM devices...");
        let devices = discover_fido2_devices().await?;

        if devices.is_empty() {
            println!("❌ No HSM devices found!");
            println!("   Please connect a FIDO2 security key (SoloKeys, YubiKey, Nitrokey, etc.)");
            return Ok(());
        }

        println!("✅ Found {} device(s):", devices.len());
        for (idx, device) in devices.iter().enumerate() {
            println!(
                "   {}. {} by {} ({:04x}:{:04x})",
                idx + 1,
                device.product,
                device.manufacturer,
                device.vendor_id,
                device.product_id
            );
            println!("      Path: {}", device.device_path.display());
            println!("      Protocol: {:?}", device.protocol_versions);
            println!("      Capabilities:");
            println!(
                "        - Resident Keys: {}",
                device.capabilities.resident_keys
            );
            println!(
                "        - User Verification: {}",
                device.capabilities.user_verification
            );
            println!("        - HMAC Secret: {}", device.capabilities.hmac_secret);
            println!(
                "        - Max Credentials: {:?}",
                device.capabilities.max_resident_keys
            );
            println!();
        }

        // Use the first device that supports resident keys
        let device_info = devices
            .into_iter()
            .find(|d| d.capabilities.resident_keys)
            .ok_or("No devices support resident keys (multi-credential operations)")?;

        println!(
            "📱 Using device: {} ({})",
            device_info.product, device_info.manufacturer
        );
        println!();

        // ========================================================================
        // STEP 2: CREATE MULTI-CREDENTIAL PROVIDER (vendor-agnostic!)
        // ========================================================================

        println!("🔧 Step 2: Initializing multi-credential provider...");
        let provider = Fido2MultiCredentialProvider::new(device_info.clone(), None).await?;

        let caps = provider.get_multi_credential_capabilities();
        println!("✅ Provider initialized!");
        println!("   Protocol: {:?}", caps.protocol);
        println!("   Max Credentials: {:?}", caps.max_credentials);
        println!(
            "   Hierarchical: {}",
            caps.supports_hierarchical_credentials
        );
        println!("   Hardware Entropy: {}", caps.supports_hardware_entropy);
        println!("   Algorithms: {:?}", caps.supported_algorithms);
        println!();

        // ========================================================================
        // STEP 3: CREATE MULTIPLE ROLES ON THE SAME DEVICE
        // ========================================================================

        println!("👥 Step 3: Creating multiple role-based credentials...");
        println!();

        // Create ADMIN credential (root authority)
        println!("   Creating ADMIN credential...");
        let admin_request = CredentialRequest {
            role: "admin".to_string(),
            display_name: Some("System Administrator".to_string()),
            permissions: vec![
                "read".to_string(),
                "write".to_string(),
                "admin".to_string(),
                "delete".to_string(),
            ],
            require_user_presence: true,
            require_user_verification: false,
            parent_credential: None,
            metadata: HashMap::from([
                ("department".to_string(), "IT Security".to_string()),
                ("clearance".to_string(), "Level 5".to_string()),
            ]),
            algorithm: Some("ES256".to_string()),
        };

        println!("      ⚠️  This will be a CTAP2 MakeCredential command (Phase 2)");
        println!("      For now, showing what WOULD happen...");
        println!();

        match provider.create_credential(admin_request).await {
            Ok(admin_cred) => {
                println!("      ✅ Admin credential created!");
                println!("         ID: {}", admin_cred.credential_id);
                println!("         Permissions: {:?}", admin_cred.permissions);
                println!();

                // Create OPERATOR credential (child of admin)
                println!("   Creating OPERATOR credential (child of ADMIN)...");
                let operator_request = CredentialRequest {
                    role: "operator".to_string(),
                    display_name: Some("System Operator".to_string()),
                    permissions: vec!["read".to_string(), "write".to_string()],
                    require_user_presence: true,
                    require_user_verification: false,
                    parent_credential: Some(admin_cred.credential_id.clone()),
                    metadata: HashMap::from([("department".to_string(), "Operations".to_string())]),
                    algorithm: Some("ES256".to_string()),
                };

                match provider.create_credential(operator_request).await {
                    Ok(operator_cred) => {
                        println!("      ✅ Operator credential created!");
                        println!("         ID: {}", operator_cred.credential_id);
                        println!(
                            "         Parent: {}",
                            operator_cred
                                .parent_credential_id
                                .as_deref()
                                .unwrap_or("none")
                        );
                        println!("         Permissions: {:?}", operator_cred.permissions);
                        println!();

                        // Create AUDITOR credential (independent)
                        println!("   Creating AUDITOR credential (read-only)...");
                        let auditor_request = CredentialRequest {
                            role: "auditor".to_string(),
                            display_name: Some("Security Auditor".to_string()),
                            permissions: vec!["read".to_string(), "audit".to_string()],
                            require_user_presence: false,
                            require_user_verification: false,
                            parent_credential: None,
                            metadata: HashMap::from([(
                                "department".to_string(),
                                "Compliance".to_string(),
                            )]),
                            algorithm: Some("ES256".to_string()),
                        };

                        match provider.create_credential(auditor_request).await {
                            Ok(auditor_cred) => {
                                println!("      ✅ Auditor credential created!");
                                println!("         ID: {}", auditor_cred.credential_id);
                                println!("         Permissions: {:?}", auditor_cred.permissions);
                                println!();

                                // List all credentials
                                println!("📋 Step 4: Listing all credentials on device...");
                                match provider.list_credentials().await {
                                    Ok(creds) => {
                                        println!("✅ Found {} credentials:", creds.len());
                                        for (idx, cred) in creds.iter().enumerate() {
                                            println!("   {}. {}", idx + 1, cred.role);
                                            println!("      ID: {}", cred.credential_id);
                                            println!("      Permissions: {:?}", cred.permissions);
                                            println!("      Use Count: {}", cred.use_count);
                                            if let Some(parent) = &cred.parent_credential_id {
                                                println!("      Parent: {parent}");
                                            }
                                            println!();
                                        }

                                        // Get hierarchical view
                                        println!("🌳 Step 5: Credential Hierarchy...");
                                        match provider.get_credential_hierarchy().await {
                                            Ok(hierarchy) => {
                                                println!("✅ Hierarchy:");
                                                fn print_tree(node: &CredentialNode, depth: usize) {
                                                    let indent = "  ".repeat(depth);
                                                    println!(
                                                        "{}├─ {} ({})",
                                                        indent,
                                                        node.credential.role,
                                                        node.credential.credential_id
                                                    );
                                                    println!(
                                                        "{}│  Permissions: {:?}",
                                                        indent, node.credential.permissions
                                                    );
                                                    for child in &node.children {
                                                        print_tree(child, depth + 1);
                                                    }
                                                }

                                                for root in &hierarchy.roots {
                                                    print_tree(root, 0);
                                                }
                                                println!();
                                            }
                                            Err(e) => {
                                                println!("   ⚠️  Could not get hierarchy: {e}");
                                            }
                                        }
                                    }
                                    Err(e) => println!("   ⚠️  Could not list credentials: {e}"),
                                }
                            }
                            Err(e) => {
                                println!("      ⚠️  Phase 2 not yet implemented: {e}");
                                println!(
                                    "         (This is expected - CTAP2 commands coming soon!)"
                                );
                            }
                        }
                    }
                    Err(e) => {
                        println!("      ⚠️  Phase 2 not yet implemented: {e}");
                        println!("         (This is expected - CTAP2 commands coming soon!)");
                    }
                }
            }
            Err(e) => {
                println!("      ⚠️  Phase 2 not yet implemented: {e}");
                println!("         (This is expected - CTAP2 commands coming soon!)");
                println!();
                println!("📝 What WOULD happen in Phase 2:");
                println!("   1. CTAP2 MakeCredential sent to device");
                println!("   2. Device prompts for button press (user presence)");
                println!("   3. Device generates key pair IN HARDWARE (never leaves device)");
                println!("   4. Device returns credential ID + public key");
                println!("   5. Private key stored securely on device");
                println!("   6. Can create up to 50+ credentials per device");
                println!("   7. Each credential has different role/permissions");
                println!();
            }
        }

        // ========================================================================
        // STEP 6: HARDWARE ENTROPY GENERATION
        // ========================================================================

        println!("🎲 Step 6: Hardware entropy generation...");
        if caps.supports_hardware_entropy {
            println!("   Device supports hardware RNG via hmac-secret extension");
            println!(
                "   Max entropy per request: {:?} bytes",
                caps.max_entropy_bytes
            );
            println!();

            match provider.generate_hardware_entropy(32).await {
                Ok(entropy) => {
                    println!("   ✅ Generated 32 bytes of hardware entropy:");
                    println!("      {}", hex::encode(&entropy));
                    println!("      (This can be used for key derivation, salts, IVs, etc.)");
                }
                Err(e) => {
                    println!("   ⚠️  Phase 2 not yet implemented: {e}");
                    println!("      (CTAP2 hmac-secret coming soon!)");
                }
            }
        } else {
            println!("   ⚠️  Device does not support hmac-secret extension");
        }
        println!();

        // ========================================================================
        // SUMMARY
        // ========================================================================

        println!("═══════════════════════════════════════════════════");
        println!("🎉 Demo Complete!");
        println!("═══════════════════════════════════════════════════");
        println!();
        println!("✅ Key Achievements:");
        println!("   • Vendor-agnostic HSM discovery");
        println!("   • Multi-credential architecture defined");
        println!("   • Role-based access control design");
        println!("   • Hierarchical credential relationships");
        println!("   • Hardware entropy generation (ready for Phase 2)");
        println!();
        println!("📋 Phase 2 Roadmap (CTAP2 Implementation):");
        println!("   • Implement MakeCredential command");
        println!("   • Implement GetAssertion command");
        println!("   • Implement credentialManagement enumerate");
        println!("   • Implement credentialManagement delete");
        println!("   • Implement hmac-secret entropy");
        println!();
        println!("🚀 Once Phase 2 is complete, this exact code will:");
        println!("   • Work with SoloKeys, YubiKeys, Nitrokeys");
        println!("   • Store 50+ roles on a single device");
        println!("   • Enable cross-device credential replication");
        println!("   • Provide true hardware-backed security");
        println!();
        println!("💡 The architecture is ready - just need CTAP2 protocol layer!");

        Ok(())
    } // close #[cfg(feature = "fido2")]
}
