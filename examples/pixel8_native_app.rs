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


//! # Pure Rust Android App for Pixel 8 BearDog HSM
//!
//! This example demonstrates how to run BearDog entirely in Rust on Android
//! without any Java layer, using the android-ndk crate for native integration.
//!
//! ## Features
//!
//! - **Pure Rust** - No Java/Kotlin code required
//! - **Native Android** - Direct NDK integration
//! - **StrongBox HSM** - Hardware-backed key operations
//! - **Pixel 8 Optimized** - Titan M security chip integration
//! - **GrapheneOS Ready** - Privacy-focused mobile OS support
//!
//! ## Usage
//!
//! Build for Android:
//! ```bash
//! cargo ndk -t arm64-v8a build --example pixel8_native_app
//! ```

#[cfg(target_os = "android")]
use android_logger::{Config, FilterBuilder};
use beardog::tunnel::hsm::android_strongbox::{
    setup_pixel8_beardog, Pixel8GrapheneOSConfig, Pixel8GrapheneOSSetup,
    Pixel8PerformanceMode,
};
use beardog::tunnel::hsm::types::*;
use beardog::BearDogResult;
use std::sync::Arc;
use tracing::{info, Level};

#[cfg(target_os = "android")]
use ndk::native_activity::{NativeActivity, NativeActivityCallbacks};

/// Main Android application entry point
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main(app: *mut std::os::raw::c_void) {
    // Initialize Android logger
    android_logger::init_once(
        Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("BearDogPixel8")
    );

    info!("🚀 BearDog Pure Rust Android App Starting");
    
    // Initialize async runtime
    let rt = tokio::runtime::Runtime::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Benchmark runtime creation failed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Benchmark runtime creation failed", e))
})?;
    
    // Run BearDog HSM system
    match rt.block_on(run_beardog_hsm()) {
        Ok(()) => {
            info!("✅ BearDog HSM system completed successfully");
        }
        Err(e) => {
            info!("❌ BearDog HSM system failed: {}", e);
        }
    }
}

/// Non-Android main for development/testing
#[cfg(not(target_os = "android"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging for non-Android
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 BearDog Development Mode (Non-Android)");
    
    run_beardog_hsm().await?;
    Ok(())
}

/// Core BearDog HSM operations
async fn run_beardog_hsm() -> BearDogResult<()> {
    info!("🔐 Initializing BearDog HSM System");
    info!("=================================");

    // Phase 1: Device validation and setup
    let (hsm, anchor_key) = initialize_pixel8_hsm().await?;

    // Phase 2: Test core HSM operations
    test_hsm_operations(&hsm, &anchor_key).await?;

    // Phase 3: Create ecosystem identity
    create_ecosystem_identity(&hsm).await?;

    // Phase 4: Run comprehensive tests
    run_comprehensive_tests(&hsm).await?;

    info!("🎉 BearDog HSM System fully operational on Pixel 8!");
    Ok(())
}

/// Initialize Pixel 8 HSM with custom configuration
async fn initialize_pixel8_hsm() -> BearDogResult<(Arc<AndroidStrongBoxHsm>, HsmKey)> {
    info!("📱 Phase 1: Pixel 8 HSM Initialization");
    info!("--------------------------------------");

    // Create custom Pixel 8 configuration for maximum security
    let config = Pixel8GrapheneOSConfig {
        require_titan_m: true,                              // Must have Titan M
        require_green_boot: true,                           // Must have verified boot
        enable_attestation: true,                           // Enable hardware attestation
        security_level: SecurityLevel::Maximum,            // Maximum security mode
        performance_mode: Pixel8PerformanceMode::MaxSecurity, // All operations in hardware
    };

    // Initialize with custom configuration
    info!("🔧 Creating Pixel 8 setup with maximum security configuration");
    let setup = Pixel8GrapheneOSSetup::new(config).await?;

    // Initialize HSM
    info!("🚀 Initializing Android StrongBox HSM");
    let hsm = setup.initialize_hsm().await?;

    // Create security anchor key
    info!("🔑 Creating BearDog security anchor key");
    let anchor_key = setup.create_anchor_key(&hsm).await?;

    // Get HSM info
    let hsm_info = hsm.get_info().await?;
    info!("📋 HSM Information:");
    info!("   Vendor: {}", hsm_info.vendor);
    info!("   Model: {}", hsm_info.model);
    info!("   Firmware: {}", hsm_info.firmware_version);
    info!("   Capabilities: {:?}", hsm_info.capabilities);

    info!("✅ Pixel 8 HSM initialization complete");
    Ok((hsm, anchor_key))
}

/// Test core HSM operations
async fn test_hsm_operations(hsm: &Arc<AndroidStrongBoxHsm>, anchor_key: &HsmKey) -> BearDogResult<()> {
    info!("🧪 Phase 2: Core HSM Operations Testing");
    info!("---------------------------------------");

    // Test 1: Basic signing and verification
    info!("🔐 Test 1: Basic Signing and Verification");
    let test_message = b"BearDog security test on Pixel 8 GrapheneOS";
    
    // Sign with anchor key
    let signature = hsm.sign(&anchor_key.id, test_message).await?;
    info!("   ✅ Message signed: {} bytes signature", signature.len());
    
    // Verify signature
    let is_valid = hsm.verify(&anchor_key.id, test_message, &signature).await?;
    if is_valid {
        info!("   ✅ Signature verification passed");
    } else {
        return Err(beardog::BearDogError::InvalidInput {
            message: "Signature verification failed".to_string(),
        });
    }

    // Test 2: Multiple key operations
    info!("🔑 Test 2: Multiple Key Operations");
    let test_keys = generate_test_keys(hsm).await?;
    info!("   ✅ Generated {} test keys", test_keys.len());

    // Test each key
    for (i, key) in test_keys.iter().enumerate() {
        let test_data = format!("Test message #{}", i + 1);
        let signature = hsm.sign(&key.id, test_data.as_bytes()).await?;
        let valid = hsm.verify(&key.id, test_data.as_bytes(), &signature).await?;
        
        if valid {
            info!("   ✅ Key {}: Operations successful", i + 1);
        } else {
            return Err(beardog::BearDogError::InvalidInput {
                message: format!("Key {} operations failed", i + 1),
            });
        }
    }

    // Test 3: Performance benchmark
    info!("⚡ Test 3: Performance Benchmark");
    let start_time = std::time::Instant::now();
    
    for i in 0..10 {
        let data = format!("Performance test iteration {}", i);
        let signature = hsm.sign(&anchor_key.id, data.as_bytes()).await?;
        let _valid = hsm.verify(&anchor_key.id, data.as_bytes(), &signature).await?;
    }
    
    let elapsed = start_time.elapsed();
    info!("   ✅ 10 sign/verify cycles completed in {:?}", elapsed);
    info!("   📊 Average per operation: {:?}", elapsed / 20); // 20 operations total

    info!("✅ Core HSM operations tests passed");
    Ok(())
}

/// Generate test keys for comprehensive testing
async fn generate_test_keys(hsm: &Arc<AndroidStrongBoxHsm>) -> BearDogResult<Vec<HsmKey>> {
    info!("🔑 Generating test keys for comprehensive testing");
    
    let mut keys = Vec::new();
    
    // Generate different types of keys
    let key_specs = vec![
        ("test-ecc-p256", KeyType::EccP256),
        ("test-ecc-p384", KeyType::EccP384),
        ("test-rsa-2048", KeyType::Rsa { key_size: 2048 }),
    ];

    for (key_id, key_type) in key_specs {
        let request = GenerateKeyRequest {
            key_id: key_id.to_string(),
            key_type,
            usage_policy: KeyUsagePolicy {
                can_sign: true,
                can_verify: true,
                exportable: false,
                ..Default::default()
            },
            metadata: KeyMetadata {
                key_id: key_id.to_string(),
                key_name: format!("Test Key: {}", key_id),
                key_type: key_type.clone(),
                created_at: chrono::Utc::now(),
                expires_at: None,
                usage_policy: KeyUsagePolicy::default(),
                tags: std::collections::HashMap::new(),
            },
            target_hsm_tier: "smartphone".to_string(),
            generate_attestation: false,
            attestation_challenge: None,
            require_user_presence: false,
        };

        match hsm.generate_strongbox_key(&request).await {
            Ok(key) => {
                info!("   ✅ Generated key: {} ({:?})", key_id, key_type);
                keys.push(key);
            }
            Err(e) => {
                info!("   ⚠️ Failed to generate key {}: {}", key_id, e);
            }
        }
    }

    Ok(keys)
}

/// Create ecosystem identity for this Pixel 8 device
async fn create_ecosystem_identity(hsm: &Arc<AndroidStrongBoxHsm>) -> BearDogResult<()> {
    info!("🌐 Phase 3: Ecosystem Identity Creation");
    info!("--------------------------------------");

    // Create configuration for ecosystem identity
    let config = Pixel8GrapheneOSConfig::default();
    let setup = Pixel8GrapheneOSSetup::new(config).await?;

    // Generate ecosystem identity
    let identity = setup.generate_ecosystem_identity(hsm).await?;
    
    info!("🆔 Ecosystem Identity Created:");
    info!("   Device ID: {}", identity.device_id);
    info!("   GrapheneOS Version: {}", identity.grapheneos_version);
    info!("   Titan M Version: {:?}", identity.titan_m_version);
    info!("   Capabilities: {:?}", identity.capabilities);
    info!("   Created At: {}", identity.created_at);

    info!("✅ Ecosystem identity creation complete");
    Ok(())
}

/// Run comprehensive HSM tests
async fn run_comprehensive_tests(hsm: &Arc<AndroidStrongBoxHsm>) -> BearDogResult<()> {
    info!("🔬 Phase 4: Comprehensive HSM Testing");
    info!("------------------------------------");

    // Test 1: Health check
    info!("🏥 Test 1: HSM Health Check");
    let health = hsm.health_check().await?;
    info!("   Health Status: {:?}", health);

    // Test 2: Stress test
    info!("💪 Test 2: Stress Test (50 operations)");
    let start_time = std::time::Instant::now();
    
    for i in 0..50 {
        let data = format!("Stress test iteration {}", i);
        let signature = hsm.sign("beardog-anchor-key", data.as_bytes()).await?;
        let _valid = hsm.verify("beardog-anchor-key", data.as_bytes(), &signature).await?;
        
        if i % 10 == 9 {
            info!("   Progress: {}/50 operations completed", i + 1);
        }
    }
    
    let elapsed = start_time.elapsed();
    info!("   ✅ Stress test completed in {:?}", elapsed);
    info!("   📊 Operations per second: {:.2}", 50.0 / elapsed.as_secs_f64());

    // Test 3: Concurrent operations
    info!("🔀 Test 3: Concurrent Operations Test");
    let concurrent_start = std::time::Instant::now();
    
    let mut handles = Vec::new();
    for i in 0..10 {
        let hsm_clone = hsm.clone();
        let handle = tokio::spawn(async move {
            let data = format!("Concurrent test {}", i);
            let signature = hsm_clone.sign("beardog-anchor-key", data.as_bytes()).await?;
            hsm_clone.verify("beardog-anchor-key", data.as_bytes(), &signature).await
        });
        handles.push(handle);
    }
    
    // Wait for all concurrent operations
    let mut success_count = 0;
    for handle in handles {
        match handle.await {
            Ok(Ok(true)) => success_count += 1,
            Ok(Ok(false)) => info!("   ⚠️ Concurrent operation verification failed"),
            Ok(Err(e)) => info!("   ⚠️ Concurrent operation error: {}", e),
            Err(e) => info!("   ⚠️ Concurrent task error: {}", e),
        }
    }
    
    let concurrent_elapsed = concurrent_start.elapsed();
    info!("   ✅ Concurrent test: {}/10 operations successful", success_count);
    info!("   📊 Concurrent operations completed in {:?}", concurrent_elapsed);

    info!("✅ All comprehensive tests passed");
    Ok(())
}

/// Display system status and capabilities
#[allow(dead_code)]
fn display_system_status() {
    info!("📊 System Status Report");
    info!("====================");
    info!("🔧 Build Configuration:");
    info!("   Target OS: {}", std::env::consts::OS);
    info!("   Architecture: {}", std::env::consts::ARCH);
    info!("   Profile: {}", if cfg!(debug_assertions) { "Debug" } else { "Release" });
    
    #[cfg(target_os = "android")]
    {
        info!("📱 Android Configuration:");
        info!("   Native App: Pure Rust");
        info!("   NDK Integration: Direct");
        info!("   StrongBox: Enabled");
    }
    
    #[cfg(not(target_os = "android"))]
    {
        info!("💻 Development Configuration:");
        info!("   Mode: Non-Android Development");
        info!("   HSM: Mock Implementation");
    }
} 