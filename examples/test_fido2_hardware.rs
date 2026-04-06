// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(
    missing_docs,
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::redundant_clone,
    clippy::needless_collect
)]

//! Test FIDO2 Device Discovery
//!
//! This example discovers FIDO2/CTAP2 security keys connected to the system.
//!
//! Usage:
//!     cargo run --example `test_fido2_hardware` --features fido2

#[cfg(feature = "fido2")]
use beardog_security::hsm::fido2::discovery::discover_fido2_devices;

#[cfg(feature = "fido2")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🔍 BearDog FIDO2 Device Discovery Test\n");
    println!("Scanning for FIDO2/CTAP2 security keys...\n");

    // Discover FIDO2 devices
    match discover_fido2_devices().await {
        Ok(devices) => {
            if devices.is_empty() {
                println!("❌ No FIDO2 devices found.");
                println!("\nExpected devices:");
                println!("  - SoloKeys Solo 2 (VID: 1209, PID: BEEE)");
                println!("  - YubiKey 5 Series (VID: 1050)");
                println!("  - Google Titan (VID: 18D1)");
                println!("\nTroubleshooting:");
                println!("  1. Check if devices are plugged in");
                println!("  2. Check USB permissions: ls -la /dev/hidraw*");
                println!("  3. Try: sudo usermod -a -G plugdev $USER");
                println!("  4. Log out and back in for group changes");
            } else {
                println!("✅ Found {} FIDO2 device(s):\n", devices.len());

                for (idx, device) in devices.iter().enumerate() {
                    println!("Device #{}", idx + 1);
                    println!("  Product:      {}", device.product);
                    println!("  Manufacturer: {}", device.manufacturer);
                    println!("  Path:         {}", device.device_path.display());
                    println!(
                        "  VID:PID:      {:04X}:{:04X}",
                        device.vendor_id, device.product_id
                    );

                    if let Some(serial) = &device.serial {
                        println!("  Serial:       {serial}");
                    }

                    println!("\n  Protocols:");
                    for proto in &device.protocol_versions {
                        println!("    - {proto}");
                    }

                    println!("\n  Extensions:");
                    for ext in &device.extensions {
                        println!("    - {ext}");
                    }

                    println!("\n  Capabilities:");
                    println!(
                        "    Resident Keys:     {}",
                        device.capabilities.resident_keys
                    );
                    println!(
                        "    User Presence:     {}",
                        device.capabilities.user_presence
                    );
                    println!(
                        "    User Verification: {}",
                        device.capabilities.user_verification
                    );
                    println!("    HMAC-Secret:       {}", device.capabilities.hmac_secret);
                    println!(
                        "    Max Message Size:  {} bytes",
                        device.capabilities.max_msg_size
                    );

                    println!();
                }

                println!("✨ Detection successful!");
                println!("\nNext steps:");
                println!("  - Phase 1.1: Implement CTAP2 GetInfo to query actual capabilities");
                println!("  - Phase 1.2: Implement hmac-secret for entropy generation");
                println!("  - Phase 1.3: Implement credential management for keys");
                println!("  - Phase 1.4: Implement signature operations");
            }
        }
        Err(e) => {
            eprintln!("❌ Error during discovery: {e}");
            eprintln!("\nThis might be due to:");
            eprintln!("  - Insufficient permissions to access HID devices");
            eprintln!("  - No FIDO2 devices connected");
            eprintln!("  - HID library initialization failure");
            return Err(e.into());
        }
    }

    Ok(())
}

#[cfg(not(feature = "fido2"))]
fn main() {
    eprintln!("This example requires the 'fido2' feature.");
    eprintln!("Run with: cargo run --example test_fido2_hardware --features fido2");
    std::process::exit(1);
}
