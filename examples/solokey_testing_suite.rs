#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

//! SoloKey Testing Suite
//!
//! Comprehensive testing for SoloKey FIDO2 devices.
//!
//! **NOTE**: This example requires FIDO2 feature and updated API.
//! It's currently disabled pending FIDO2 Phase 2 implementation.
//!
//! Run with (when re-enabled):
//! ```bash
//! cargo run --example solokey_testing_suite --features fido2
//! ```

#[cfg(not(feature = "fido2"))]
fn main() {
    eprintln!("❌ This example requires the 'fido2' feature.");
    eprintln!("   Run with: cargo run --example solokey_testing_suite --features fido2");
    eprintln!("\n⚠️  Note: FIDO2 API is currently being updated (Phase 2)");
    std::process::exit(1);
}

#[cfg(feature = "fido2")]
use beardog_security::hsm::fido2::discovery::discover_fido2_devices;
#[cfg(feature = "fido2")]
use std::io::{self, Write};

#[cfg(feature = "fido2")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║          BearDog SoloKey Testing Suite - Nov 9, 2025          ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    // Test 1: Device Discovery
    println!("🔍 TEST 1: Device Discovery");
    println!("─────────────────────────────────────────────────────────");

    match discover_fido2_devices().await {
        Ok(devices) => {
            if devices.is_empty() {
                println!("❌ No FIDO2 devices found!");
                println!("   Please insert your SoloKeys and try again.");
                return Ok(());
            }

            println!("✅ Found {} FIDO2 device(s)", devices.len());
            println!();

            for (idx, device) in devices.iter().enumerate() {
                println!("   Device {}: {}", idx + 1, device.product);
                println!("      Manufacturer: {}", device.manufacturer);
                println!("      Path: {}", device.device_path.display());
                println!(
                    "      VID: 0x{:04x}, PID: 0x{:04x}",
                    device.vendor_id, device.product_id
                );

                // Check protocol version
                if !device.protocol_versions.is_empty() {
                    println!("      Protocol: CTAP2");
                }

                println!("      Capabilities: Pending query (GetInfo needed)");
                println!();
            }

            // Test 2: Device Selection
            println!();
            println!("🎯 TEST 2: Device Selection");
            println!("─────────────────────────────────────────────────────────");

            if devices.len() == 1 {
                println!("✅ Single device mode - automatically selected");
                test_device(&devices[0]).await?;
            } else {
                println!("   Multiple devices detected. Select device to test:");
                for (idx, device) in devices.iter().enumerate() {
                    println!(
                        "   [{}] {} ({})",
                        idx + 1,
                        device.product,
                        device.device_path.display()
                    );
                }

                print!("\n   Enter device number (1-{}): ", devices.len());
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if let Ok(selection) = input.trim().parse::<usize>() {
                    if selection > 0 && selection <= devices.len() {
                        println!("   Selected: {}", devices[selection - 1].product);
                        test_device(&devices[selection - 1]).await?;
                    } else {
                        println!("   ❌ Invalid selection");
                    }
                } else {
                    println!("   ❌ Invalid input");
                }
            }
        }
        Err(e) => {
            println!("❌ Discovery failed: {}", e);
            println!("   Make sure:");
            println!("   1. SoloKeys are inserted");
            println!("   2. You have USB access permissions");
            println!("   3. hidapi is installed (libhidapi-dev on Ubuntu)");
        }
    }

    println!();
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                      Testing Complete!                         ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    Ok(())
}

#[cfg(feature = "fido2")]
async fn test_device(
    device: &beardog_security::hsm::fido2::types::Fido2DeviceInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    println!("🧪 TEST 3: Device Capabilities");
    println!("─────────────────────────────────────────────────────────");

    // Check if device supports required capabilities
    println!("   Checking CTAP2 capabilities...");

    println!("   ⚠️  Capabilities not yet queried");
    println!("   💡 GetInfo command needs to be implemented");
    println!("   📝 This will be done in FIDO2 Phase 2");
    println!();
    println!("   Expected SoloKey capabilities:");
    println!("      - MakeCredential (credential creation)");
    println!("      - GetAssertion (authentication)");
    println!("      - hmac-secret (entropy generation) 🎲");
    println!("      - Credential Management (resident keys)");
    println!("      - User Verification (PIN support)");

    println!();
    println!("🔐 TEST 4: User Presence Check");
    println!("─────────────────────────────────────────────────────────");
    println!("   💡 To test user presence:");
    println!("   1. Touch the gold/copper button on your SoloKey");
    println!("   2. The LED will flash to confirm");
    println!();
    println!("   This will be implemented in Phase 2 (CTAP2 commands)");

    println!();
    println!("🎲 TEST 5: Entropy Generation (hmac-secret)");
    println!("─────────────────────────────────────────────────────────");
    println!("   💡 SoloKeys can generate high-quality entropy using hmac-secret");
    println!("   🔬 This uses the secure element's hardware RNG");
    println!();
    println!("   Implementation plan:");
    println!("   1. Send CTAP2 GetInfo command");
    println!("   2. Verify hmac-secret extension support");
    println!("   3. Create credential with hmac-secret enabled");
    println!("   4. Generate assertion to get entropy");
    println!();
    println!("   Status: Planned for FIDO2 Phase 2");

    Ok(())
}
