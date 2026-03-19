//! Test CTAP2 `GetInfo` Command with Real Hardware (Pure Rust)
//!
//! This example tests the CTAP2 `GetInfo` command implementation
//! with actual FIDO2 devices (`SoloKeys`, `YubiKey`, etc.).
//!
//! # ecoBin Compliance
//!
//! Uses `beardog-hid` (100% Pure Rust) instead of `hidapi` (C library).
//!
//! # Usage
//!
//! ```bash
//! cargo run --example test_ctap2_getinfo --features fido2
//! ```

#[tokio::main]
async fn main() -> Result<(), beardog_errors::BearDogError> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("CTAP2 GetInfo Test - Real Hardware Verification");
    println!("Pure Rust (ecoBin Compliant)");
    println!();

    #[cfg(not(feature = "fido2"))]
    {
        println!("FIDO2 feature not enabled!");
        println!("Run with: cargo run --example test_ctap2_getinfo --features fido2");
        return Ok(());
    }

    #[cfg(feature = "fido2")]
    {
        run_ctap2_test().await
    }
}

#[cfg(feature = "fido2")]
async fn run_ctap2_test() -> Result<(), beardog_errors::BearDogError> {
    use beardog_hid::{discover, open_device, HidDevice};

    // Step 1: Discover HID devices using Pure Rust
    println!("Step 1: Discovering HID devices (Pure Rust /dev/hidraw)...");
    let devices = discover().await?;

    if devices.is_empty() {
        println!("No HID devices found!");
        println!("Please connect a SoloKeys, YubiKey, or other FIDO2 security key");
        return Ok(());
    }

    println!("Found {} HID device(s)\n", devices.len());

    // Filter for likely FIDO2 devices
    let fido2_devices: Vec<_> = devices
        .iter()
        .filter(|d| {
            // Common FIDO2 vendor IDs
            let vid = d.vendor_id.0;
            matches!(
                vid,
                0x1050 | // Yubico
                0x1209 | // SoloKeys
                0x20A0 | // Nitrokey
                0x096E | // Feitian
                0x2581 // Ledger
            )
        })
        .collect();

    if fido2_devices.is_empty() {
        println!(
            "No FIDO2 devices found among {} HID devices!",
            devices.len()
        );
        println!("Listing all HID devices for debugging:");
        for dev in &devices {
            println!(
                "  VID:0x{:04x} PID:0x{:04x} - {} {}",
                dev.vendor_id.0, dev.product_id.0, dev.manufacturer, dev.product
            );
        }
        return Ok(());
    }

    println!("Found {} FIDO2 device(s)\n", fido2_devices.len());

    // Step 2: Test each FIDO2 device
    for (idx, device_info) in fido2_devices.iter().enumerate() {
        println!("-----------------------------------------------------------");
        println!(
            "Device #{}: {} - {}",
            idx + 1,
            device_info.manufacturer,
            device_info.product
        );
        println!(
            "   VID:PID: 0x{:04x}:0x{:04x}",
            device_info.vendor_id.0, device_info.product_id.0
        );
        println!("   Path: {}", device_info.path);
        println!();

        // Open device using Pure Rust beardog-hid
        println!("Opening device (Pure Rust)...");
        match open_device(&device_info.path).await {
            Ok(mut device) => {
                println!("Device opened successfully");
                println!();

                // Send CTAP2 GetInfo command (0x04)
                println!("Sending CTAP2 GetInfo command...");

                // CTAP2 GetInfo: [CTAP2_CMD_GET_INFO]
                let ctap2_getinfo = vec![0x04];
                if let Err(e) = device.write(&ctap2_getinfo).await {
                    println!("Failed to send command: {e}");
                    continue;
                }

                // Read response
                let mut response = vec![0u8; 1024];
                match device.read(&mut response).await {
                    Ok(n) if n > 0 => {
                        let status = response[0];
                        if status == 0x00 {
                            println!("GetInfo SUCCESS! (status: 0x{status:02x})");
                            println!("Response: {n} bytes");
                            // Full CBOR parsing would go here
                            // For now, just show raw response
                            println!("Raw (first 64 bytes): {:02x?}", &response[..n.min(64)]);
                        } else {
                            println!("GetInfo returned error status: 0x{status:02x}");
                        }
                    }
                    Ok(_) => println!("No response received"),
                    Err(e) => println!("Read error: {e}"),
                }
            }
            Err(e) => {
                println!("Failed to open device: {e}");
                println!("This might be:");
                println!(
                    "   - Permission issues (try: sudo chmod 666 {})",
                    device_info.path
                );
                println!("   - Device is in use by another process");
            }
        }
        println!();
    }

    println!("Test Complete!");
    println!();
    println!("Next Steps:");
    println!("   1. Implement full CBOR response parsing");
    println!("   2. Implement MakeCredential (create credentials)");
    println!("   3. Implement GetAssertion (sign with credentials)");
    println!("   4. Implement hmac-secret entropy generation");

    Ok(())
}
