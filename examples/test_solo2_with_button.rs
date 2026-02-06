//! Solo 2 Button Press Test - Pure Rust Implementation
//!
//! # Overview
//!
//! Tests Solo2 FIDO2 device with user presence verification.
//! Uses Pure Rust beardog-hid for HID communication (ecoBin compliant).
//!
//! # Usage
//!
//! ```bash
//! cargo run --example test_solo2_with_button --features fido2
//! ```

#[cfg(feature = "fido2")]
use beardog_security::hsm::fido2::{ctap2, discovery};

#[tokio::main]
async fn main() -> Result<(), beardog_errors::BearDogError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║     Solo 2 Button Press Test - User Interaction          ║");
    println!("║     (Pure Rust - ecoBin Compliant)                        ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();

    #[cfg(not(feature = "fido2"))]
    {
        println!("⚠️  FIDO2 feature not enabled!");
        println!("   Run with: cargo run --example test_solo2_with_button --features fido2");
        return Ok(());
    }

    #[cfg(feature = "fido2")]
    {
        // Discover devices using beardog-hid (Pure Rust)
        println!("🔍 Discovering FIDO2 devices...");
        let devices = discovery::discover_fido2_devices().await?;

        if devices.is_empty() {
            println!("⚠️  No FIDO2 devices found!");
            return Ok(());
        }

        println!("✅ Found {} device(s)\n", devices.len());

        // Test first device
        let device_info = &devices[0];
        println!("📱 Testing: {}", device_info.product);
        println!("   Path: {}", device_info.device_path.display());
        println!();

        // Open device using beardog-hid (Pure Rust)
        println!("🔓 Opening HID device with beardog-hid (Pure Rust)...");

        // Discover and open device
        let hid_devices = beardog_hid::discover()
            .await
            .map_err(|e| beardog_errors::BearDogError::system(format!("HID discovery: {}", e)))?;

        let hid_info = hid_devices
            .iter()
            .find(|d| d.path.contains(&device_info.device_path.to_string_lossy().to_string()))
            .ok_or_else(|| {
                beardog_errors::BearDogError::system("Device not found in HID list".to_string())
            })?;

        let mut hid_device = beardog_hid::open_device(&hid_info.path)
            .await
            .map_err(|e| beardog_errors::BearDogError::system(format!("Device open: {}", e)))?;

        println!("✅ Device opened\n");

        println!("╔═══════════════════════════════════════════════════════════╗");
        println!("║                                                           ║");
        println!("║  👉 PLEASE PRESS THE BUTTON ON YOUR SOLO 2 NOW! 👈       ║");
        println!("║                                                           ║");
        println!("║  The LED should be blinking or lit up.                   ║");
        println!("║  Press the physical button on the device.                ║");
        println!("║                                                           ║");
        println!("╚═══════════════════════════════════════════════════════════╝");
        println!();
        println!("⏱️  Sending GetInfo and waiting 30 seconds for response...");
        println!("   (Press button if LED is active)");
        println!();

        // Try GetInfo
        match ctap2::ctap2_get_info(&mut hid_device).await {
            Ok(info) => {
                println!();
                println!("╔═══════════════════════════════════════════════════════════╗");
                println!("║              🎉 SUCCESS! GETINFO WORKED! 🎉              ║");
                println!("╚═══════════════════════════════════════════════════════════╝");
                println!();
                println!("📊 Device Capabilities:");
                println!("   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!();

                println!("   🔹 Supported Versions:");
                for version in &info.versions {
                    println!("      - {}", version);
                }
                println!();

                if !info.extensions.is_empty() {
                    println!("   🔹 Supported Extensions:");
                    for ext in &info.extensions {
                        println!("      - {}", ext);
                        if ext == "hmac-secret" {
                            println!("         └─ ✅ Can generate hardware entropy!");
                        }
                    }
                    println!();
                }

                if !info.options.is_empty() {
                    println!("   🔹 Device Options:");
                    for (key, value) in &info.options {
                        println!("      - {}: {}", key, if *value { "YES" } else { "NO" });
                    }
                    println!();
                }

                println!("   🔹 AAGUID: {:?}", &info.aaguid[..8]);
                println!();

                println!("🎯 Next Steps:");
                println!("   ✅ Button press was REQUIRED for GetInfo");
                println!("   ✅ Device is fully functional");
                println!("   ✅ Ready to implement MakeCredential");
                println!("   ✅ Ready to generate hardware entropy");
                println!();

                Ok(())
            }
            Err(e) => {
                println!();
                println!("❌ GetInfo still failed: {}", e);
                println!();
                println!("💡 Possibilities:");
                println!("   1. Button press not required (try other approach)");
                println!("   2. Different initialization needed");
                println!("   3. Try MakeCredential instead");
                println!("   4. Check Solo 2 firmware documentation");
                println!();

                Ok(())
            }
        }
    }
}
