//! Test CTAP2 GetInfo Command with Real Hardware
//!
//! This example tests the CTAP2 GetInfo command implementation
//! with actual FIDO2 devices (SoloKeys, YubiKey, etc.).

#[cfg(feature = "fido2")]
use beardog_security::hsm::fido2::{ctap2, discovery};

#[tokio::main]
async fn main() -> Result<(), beardog_errors::BearDogError> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║       CTAP2 GetInfo Test - Real Hardware Verification         ║");
    println!("║                   November 9, 2025                             ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
    
    #[cfg(not(feature = "fido2"))]
    {
        println!("⚠️  FIDO2 feature not enabled!");
        println!("💡 Run with: cargo run --example test_ctap2_getinfo --features fido2");
        return Ok(());
    }
    
    #[cfg(feature = "fido2")]
    {
        // Step 1: Discover FIDO2 devices
        println!("🔍 Step 1: Discovering FIDO2 devices...");
        let devices = discovery::discover_fido2_devices().await?;
        
        if devices.is_empty() {
            println!("⚠️  No FIDO2 devices found!");
            println!("💡 Please connect a SoloKeys, YubiKey, or other FIDO2 security key");
            return Ok(());
        }
        
        println!("✅ Found {} device(s)\n", devices.len());
        
        // Step 2: Test each device
        for (idx, device_info) in devices.iter().enumerate() {
            println!("─────────────────────────────────────────────────────────");
            println!("📱 Device #{}: {}", idx + 1, device_info.product);
            println!("   Manufacturer: {}", device_info.manufacturer);
            println!("   Path: {}", device_info.device_path.display());
            println!("   VID:PID: 0x{:04x}:0x{:04x}", device_info.vendor_id, device_info.product_id);
            println!();
            
            // Open HID device
            println!("🔓 Opening device...");
            let path_str = device_info.device_path.to_string_lossy();
            let api = hidapi::HidApi::new()
                .map_err(|e| beardog_errors::BearDogError::system(format!("HID API init failed: {}", e)))?;
            
            let device = api.open_path(std::ffi::CString::new(path_str.as_bytes()).unwrap().as_c_str())
                .map_err(|e| beardog_errors::BearDogError::system(format!("Device open failed: {}", e)))?;
            
            println!("✅ Device opened successfully");
            println!();
            
            // Send CTAP2 GetInfo
            println!("📤 Sending CTAP2 GetInfo command...");
            match ctap2::ctap2_get_info(&device).await {
                Ok(info) => {
                    println!("✅ GetInfo SUCCESS!");
                    println!();
                    println!("📊 Device Capabilities:");
                    println!("   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    
                    println!("   🔹 Supported Versions:");
                    for version in &info.versions {
                        println!("      - {}", version);
                    }
                    
                    if !info.extensions.is_empty() {
                        println!("   🔹 Supported Extensions:");
                        for ext in &info.extensions {
                            println!("      - {}", ext);
                            if ext == "hmac-secret" {
                                println!("         └─ ✅ Can generate hardware entropy!");
                            }
                        }
                    }
                    
                    if !info.options.is_empty() {
                        println!("   🔹 Device Options:");
                        for (key, value) in &info.options {
                            println!("      - {}: {}", key, if *value { "YES" } else { "NO" });
                        }
                    }
                    
                    if let Some(max_msg) = info.max_msg_size {
                        println!("   🔹 Max Message Size: {} bytes", max_msg);
                    }
                    
                    if let Some(protocols) = &info.pin_protocols {
                        println!("   🔹 PIN Protocols: {:?}", protocols);
                    }
                    
                    println!("   🔹 AAGUID: {}", hex::encode(&info.aaguid));
                    
                    println!();
                    println!("🎯 Key Findings:");
                    
                    // Check for CTAP2 support
                    if info.versions.iter().any(|v| v.starts_with("FIDO_2")) {
                        println!("   ✅ FIDO2/CTAP2 supported");
                    }
                    
                    // Check for hmac-secret
                    if info.extensions.contains(&"hmac-secret".to_string()) {
                        println!("   ✅ hmac-secret extension available (can generate entropy)");
                    } else {
                        println!("   ⚠️  hmac-secret not available (entropy generation limited)");
                    }
                    
                    // Check for resident key support
                    if info.options.get("rk").copied().unwrap_or(false) {
                        println!("   ✅ Resident keys supported (can store credentials)");
                    }
                    
                    // Check for user verification
                    if info.options.get("uv").copied().unwrap_or(false) {
                        println!("   ✅ User verification available (PIN/biometric)");
                    }
                    
                    println!();
                }
                Err(e) => {
                    println!("❌ GetInfo FAILED: {}", e);
                    println!("💡 This might be:");
                    println!("   - Device not in FIDO mode");
                    println!("   - Permission issues");
                    println!("   - Device in use by another process");
                    println!();
                }
            }
        }
        
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║                     Test Complete!                             ║");
        println!("╚════════════════════════════════════════════════════════════════╝");
        println!();
        println!("🎉 Next Steps:");
        println!("   1. Implement MakeCredential (create credentials)");
        println!("   2. Implement GetAssertion (sign with credentials)");
        println!("   3. Implement hmac-secret entropy generation");
        println!("   4. Test multi-credential operations");
    }
    
    Ok(())
}

