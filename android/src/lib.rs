//! # BearDog Pixel 8 Android Library
//!
//! Pure Rust Android library for BearDog HSM operations on Pixel 8 with GrapheneOS.

#[cfg(target_os = "android")]
use android_logger::{Config, FilterBuilder};
use beardog_tunnel::tunnel::hsm::android_strongbox::{
    setup_pixel8_beardog, Pixel8GrapheneOSConfig, Pixel8PerformanceMode,
};
use beardog_tunnel::tunnel::hsm::HsmProvider;
use beardog_tunnel::tunnel::hsm::SecurityLevel;
use std::sync::Arc;
use tracing::{error, info};

#[cfg(target_os = "android")]
use ndk_glue::native_activity::NativeActivity;

/// Initialize Android logging
#[cfg(target_os = "android")]
fn init_android_logging() {
    android_logger::init_once(
        Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("BearDogPixel8")
            .with_filter(FilterBuilder::new().parse("debug,beardog=info").build()),
    );
}

/// Main Android entry point
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main(_app: &mut NativeActivity) {
    init_android_logging();

    info!("🚀 BearDog Pixel 8 Android App Starting");
    info!("========================================");

    // Create async runtime
    let rt = match Runtime::new() {
        Ok(runtime) => runtime,
        Err(e) => {
            error!("❌ Failed to create async runtime: {}", e);
            return;
        }
    };

    // Run the main application
    match rt.block_on(run_beardog_android_app()) {
        Ok(()) => {
            info!("✅ BearDog Android app completed successfully");
        }
        Err(e) => {
            error!("❌ BearDog Android app failed: {}", e);
        }
    }
}

/// Main BearDog Android application logic
#[allow(dead_code)] // Will be used when Android app integration is fully implemented
async fn run_beardog_android_app() -> Result<(), Box<dyn std::error::Error>> {
    info!("🔐 Starting BearDog HSM operations on Android");

    // Configure for Pixel 8 maximum security
    let config = Pixel8GrapheneOSConfig {
        require_titan_m: true,
        require_green_boot: true,
        enable_attestation: true,
        security_level: SecurityLevel::Maximum,
        performance_mode: Pixel8PerformanceMode::MaxSecurity,
    };

    info!("📱 Pixel 8 Configuration:");
    info!("   Titan M Required: {}", config.require_titan_m);
    info!("   Green Boot Required: {}", config.require_green_boot);
    info!("   Attestation Enabled: {}", config.enable_attestation);
    info!("   Security Level: {:?}", config.security_level);
    info!("   Performance Mode: {:?}", config.performance_mode);

    // Initialize BearDog HSM system
    info!("🚀 Initializing BearDog HSM system...");
    let (hsm, anchor_key) = setup_pixel8_beardog().await?;

    info!("✅ HSM System Initialized:");
    let hsm_info = hsm.get_info().await?;
    info!("   HSM ID: {}", hsm_info.instance_id);
    info!("   Vendor: {}", hsm_info.vendor);
    info!("   Model: {}", hsm_info.model);
    info!("   Anchor Key: {}", anchor_key.id);

    // Test core operations
    test_core_operations(&(hsm.clone() as Arc<dyn HsmProvider>), &anchor_key).await?;
    info!("✅ Core operations test completed");

    // Performance benchmark
    performance_test(&(hsm.clone() as Arc<dyn HsmProvider>), &anchor_key).await?;
    info!("✅ Performance test completed");

    // Keep the app running (in real app, this would be event loop)
    info!("🏃 BearDog Android app running successfully");
    info!("💤 Entering sleep mode (real app would handle events here)");

    // Sleep for demonstration
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    info!("🎉 BearDog Android demo completed successfully!");
    Ok(())
}

/// Test core HSM operations
#[allow(dead_code)] // Will be used when Android HSM testing is fully implemented
async fn test_core_operations(
    hsm: &Arc<dyn HsmProvider>,
    anchor_key: &beardog_tunnel::tunnel::hsm::types::HsmKey,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🔐 Testing basic cryptographic operations...");

    let test_data = b"BearDog test on Pixel 8 GrapheneOS with Titan M";

    // Sign data
    let signature = hsm.sign(&anchor_key.id, test_data).await?;
    info!("   ✅ Data signed: {} bytes", signature.len());

    // Verify signature
    let is_valid = hsm.verify(&anchor_key.id, test_data, &signature).await?;
    if is_valid {
        info!("   ✅ Signature verification passed");
    } else {
        error!("   ❌ Signature verification failed");
        return Err("Signature verification failed".into());
    }

    // Test health
    let health = hsm.health_check().await?;
    info!("   ✅ HSM Health: {:?}", health);

    Ok(())
}

/// Performance test
#[allow(dead_code)] // Will be used when Android HSM testing is fully implemented
async fn performance_test(
    hsm: &Arc<dyn HsmProvider>,
    anchor_key: &beardog_tunnel::tunnel::hsm::types::HsmKey,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("⚡ Performance testing: 20 sign/verify operations...");

    let start_time = std::time::Instant::now();

    for i in 0..20 {
        let data = format!("Performance test iteration {i}");
        let signature = hsm.sign(&anchor_key.id, data.as_bytes()).await?;
        let _valid = hsm
            .verify(&anchor_key.id, data.as_bytes(), &signature)
            .await?;
    }

    let elapsed = start_time.elapsed();
    let ops_per_sec = 40.0 / elapsed.as_secs_f64(); // 40 operations total

    info!("   ✅ Performance test completed:");
    info!("   📊 Total time: {:?}", elapsed);
    info!("   📊 Operations/second: {:.2}", ops_per_sec);
    info!("   📊 Average latency: {:?}", elapsed / 40);

    Ok(())
}

// Export the android_main function for NDK
#[cfg(target_os = "android")]
ndk_glue::ndk_glue!(android_main);
