// SPDX-License-Identifier: AGPL-3.0-only
#![allow(
    missing_docs,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::redundant_clone,
    clippy::needless_collect
)]
//! Debug `CTAPHID_INIT` Communication - Pure Rust Implementation
//!
//! This is a minimal test to debug the `CTAPHID_INIT` handshake.
//! Uses Pure Rust beardog-hid for HID communication (ecoBin compliant).

#[cfg(feature = "fido2")]
use beardog_security::hsm::fido2::discovery;

#[tokio::main]
async fn main() -> Result<(), beardog_errors::BearDogError> {
    // Initialize logging with debug level
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║    CTAPHID_INIT Debug - Minimal Communication Test        ║");
    println!("║    (Pure Rust - ecoBin Compliant)                         ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();

    #[cfg(not(feature = "fido2"))]
    {
        println!("⚠️  FIDO2 feature not enabled!");
        println!("   Run with: cargo run --example test_ctaphid_init_debug --features fido2");
        return Ok(());
    }

    #[cfg(feature = "fido2")]
    {
        // Discover devices
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

        let hid_devices = beardog_hid::discover()
            .await
            .map_err(|e| beardog_errors::BearDogError::system(format!("HID discovery: {e}")))?;

        let hid_info = hid_devices
            .iter()
            .find(|d| {
                d.path
                    .contains(&device_info.device_path.to_string_lossy().to_string())
            })
            .ok_or_else(|| {
                beardog_errors::BearDogError::system("Device not found in HID list".to_string())
            })?;

        let mut hid_device = beardog_hid::open_device(&hid_info.path)
            .await
            .map_err(|e| beardog_errors::BearDogError::system(format!("Device open: {e}")))?;

        println!("✅ Device opened\n");

        // Generate nonce
        let mut nonce = [0u8; 8];
        for (i, byte) in nonce.iter_mut().enumerate() {
            // Low 8 bits of linear function — always fits in `u8`.
            *byte = ((i * 17 + 42) & 0xFF) as u8;
        }

        println!("📋 Building CTAPHID_INIT packet:");
        println!("   Nonce: {:02x?}", &nonce);

        // Build CTAPHID_INIT packet
        let mut packet = vec![
            0xFF, 0xFF, 0xFF, 0xFF, // Broadcast CID
            0x86, // CTAPHID_INIT
            0x00, 0x08, // Length (8 bytes)
        ];
        packet.extend_from_slice(&nonce);

        // Pad to 64 bytes
        while packet.len() < 64 {
            packet.push(0);
        }

        println!("   Packet size: {} bytes", packet.len());
        println!("   Packet hex: {:02x?}", &packet[..16]); // First 16 bytes
        println!();

        // Send packet
        println!("📤 Sending CTAPHID_INIT...");
        match hid_device.write(&packet).await {
            Ok(bytes_written) => {
                println!("✅ Sent {bytes_written} bytes");
            }
            Err(e) => {
                println!("❌ Write failed: {e}");
                return Ok(());
            }
        }

        // Try to read response with timeout using tokio
        println!();
        println!("📥 Attempting to read response...");

        for attempt in 1..=5 {
            println!("   Attempt {attempt}/5...");

            let mut response = vec![0u8; 64];

            // Use tokio timeout for read
            let read_result = tokio::time::timeout(
                tokio::time::Duration::from_secs(1),
                hid_device.read(&mut response),
            )
            .await;

            match read_result {
                Ok(Ok(bytes_read)) if bytes_read > 0 => {
                    println!("   ✅ Got {bytes_read} bytes!");
                    println!("   Response hex: {:02x?}", &response[..bytes_read.min(32)]);
                    println!();

                    // Parse response
                    if bytes_read >= 7 {
                        let cid = u32::from_be_bytes([
                            response[0],
                            response[1],
                            response[2],
                            response[3],
                        ]);
                        let cmd = response[4];
                        let len = (usize::from(response[5]) << 8) | usize::from(response[6]);

                        println!("📊 Parsed response:");
                        println!("   CID: 0x{cid:08X}");
                        println!("   CMD: 0x{cmd:02X}");
                        println!("   LEN: {len} bytes");

                        if cmd == 0x86 && bytes_read >= 19 {
                            // Verify nonce
                            let echoed_nonce = &response[7..15];
                            println!("   Echoed nonce: {echoed_nonce:02x?}");

                            if echoed_nonce == nonce {
                                println!("   ✅ Nonce matches!");
                            } else {
                                println!("   ⚠️  Nonce mismatch!");
                            }

                            // Extract new CID
                            let new_cid = u32::from_be_bytes([
                                response[15],
                                response[16],
                                response[17],
                                response[18],
                            ]);
                            println!("   New CID: 0x{new_cid:08X}");

                            if bytes_read >= 20 {
                                println!("   Protocol version: {}", response[19]);
                                println!("   Device version (major): {}", response[20]);
                                println!("   Device version (minor): {}", response[21]);
                                println!("   Device version (build): {}", response[22]);
                            }

                            println!();
                            println!("🎉 CTAPHID_INIT SUCCESS!");
                            return Ok(());
                        } else if cmd == 0xBF && bytes_read >= 8 {
                            // Error response
                            let error_code = response[7];
                            println!("   ❌ Device returned error: 0x{error_code:02X}");
                        }
                    }

                    return Ok(());
                }
                Ok(Ok(_)) => {
                    println!("   ⏱️  Timeout (no data)");
                }
                Ok(Err(e)) => {
                    println!("   ❌ Read error: {e}");
                    return Ok(());
                }
                Err(_) => {
                    println!("   ⏱️  Read timeout");
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }

        println!();
        println!("❌ No response received after 5 attempts");
        println!();
        println!("💡 Troubleshooting:");
        println!("   1. Check device permissions (may need udev rules)");
        println!("   2. Try: sudo chmod 666 /dev/hidraw*");
        println!("   3. Verify device is not locked by another process");
        println!("   4. Try unplugging and replugging the device");
    }

    Ok(())
}
