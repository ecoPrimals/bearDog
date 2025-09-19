#[cfg(target_os = "android")]
use android_logger::{Config, FilterBuilder};
use beardog::tunnel::hsm::android_strongbox::{
    setup_pixel8_beardog, Pixel8GrapheneOSConfig, Pixel8GrapheneOSSetup, Pixel8PerformanceMode,
};
use beardog::tunnel::hsm::types::*;
use beardog_errors::BearDogError;
use std::sync::Arc;
use tracing::{info, Level};

#[cfg(target_os = "android")]
use ndk::native_activity::{NativeActivity, NativeActivityCallbacks};

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main(app: *mut std::os::raw::c_void) {
    android_logger::init_once(
        Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("BearDogPixel8"),
    );

    info!("[ROCKET] BearDog Pure Rust Android App Starting");

    let rt = tokio::runtime::Runtime::new().map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Benchmark runtime creation failed ",
            e
        );
        beardog_errors::BearDogError::internal({:?}",
            "Benchmark runtime creation failed ", e
        ))
    })?;

    match rt.block_on({}", e);
        }
    }
}

#[cfg(not(target_os = "android"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("[ROCKET] BearDog Development Mode (Non-Android)");

    run_beardog_hsm()?;
    Ok(())
}

async fn run_beardog_hsm() -> Result<(), BearDogError> {
    info!("🔐 Initializing BearDog HSM System");
    info!("=================================");

    let (hsm, anchor_key) = initialize_pixel8_hsm()?;

    test_hsm_operations(&hsm, &anchor_key)?;

    create_ecosystem_identity(&hsm)?;

    run_comprehensive_tests(&hsm)?;

    info!("[PARTY] BearDog HSM System fully operational on Pixel 8!");
    Ok(())
}

async fn initialize_pixel8_hsm() -> Result<(Arc<AndroidStrongBoxHsm, BearDogError>, HsmKey)> {
    info!("📱 Phase 1: Pixel 8 HSM Initialization");
    info!("--------------------------------------");

    let config = Pixel8GrapheneOSConfig {
        require_titan_m: true,                                // Must have Titan M
        require_green_boot: true,                             // Must have verified boot
        enable_attestation: true,                             // Enable hardware attestation
        security_level: SecurityLevel::Maximum,               // Maximum security mode
        performance_mode: Pixel8PerformanceMode::MaxSecurity, // All operations in hardware
    };

    info!("🔧 Creating Pixel 8 setup with maximum security configuration");
    let setup = Pixel8GrapheneOSSetup::new(config)?;

    info!("[ROCKET] Initializing Android StrongBox HSM");
    let hsm = setup.initialize_hsm()?;

    info!("🔑 Creating BearDog security anchor key");
    let anchor_key = setup.create_anchor_key(&hsm)?;

    let hsm_info = hsm.get_info()?;
    info!("📋 HSM Information:");
    info!("   Vendor: {}", hsm_info.vendor);
    info!("   Model: {}", hsm_info.model);
    info!("   Firmware: {}", hsm_info.firmware_version);
    info!("   Capabilities: {:?}", hsm_info.capabilities);

    info!("[OK] Pixel 8 HSM initialization complete ");
    Ok(&Arc<AndroidStrongBoxHsm>,
    anchor_key: &HsmKey,
) -> Result<(), BearDogError> {
    info!("🧪 Phase 2: Core HSM Operations Testing");
    info!("---------------------------------------");

    info!("🔐 Test 1: Basic Signing and Verification");
    let test_message = b"BearDog security test on Pixel 8 GrapheneOS";

    let signature = hsm.sign({} bytes signature", signature.len());

    let is_valid = hsm.verify(&anchor_key.id, test_message, &signature)?;
    if is_valid {
        info!("   [OK] Signature verification passed ");
    } else {
        return Err(beardog::BearDogError::InvalidInput {
            message: "Signature verification failed ".to_string(),
        });
    }

    info!("🔑 Test 2: Multiple Key Operations ");
    let test_keys = generate_test_keys(Operations successful", i + 1);
        } else {
            return Err(beardog::BearDogError::InvalidInput {
                message: format!("Key {} operations failed ", i + 1),
            });
        }
    }

    info!("[LIGHTNING] Test 3: Performance Benchmark");
    let start_time = std::time::Instant::now();

    for i in 0..10 {
        let data = format!("Performance test iteration {}", i);
        let signature = hsm.sign(&anchor_key.id, data.as_bytes())?;
        let _valid = hsm
            .verify(&anchor_key.id, data.as_bytes(), &signature)
            ?;
    }

    let elapsed = start_time.elapsed();
    info!("   [OK] 10 sign/verify cycles completed in {:?}", elapsed);
    info!("   [CHART] Average per operation: {:?}", elapsed / 20); // 20 operations total

    info!("[OK] Core HSM operations tests passed ");
    Ok(())
}

async fn generate_test_keys(hsm: &Arc<AndroidStrongBoxHsm>) -> Result<Vec<HsmKey, BearDogError>> {
    info!("🔑 Generating test keys for comprehensive testing");

    let mut keys = Vec::new();

    let key_specs = vec![
        ("test-ecc-p256", KeyType::EccP256),
        ("test-ecc-p384", KeyType::EccP384),
        ("test-rsa-2048", KeyType::Rsa { key_size: 2048 }),
    ];

    for (key_id, key_type) in key_specs {
        let request = GenerateKeyRequest {
            key_id: key_id.to_string(KeyUsagePolicy {
                can_sign: true,
                can_verify: true,
                exportable: false,
                ..Default::default()
            },
            metadata: KeyMetadata {
                key_id: key_id.to_string(format!("Test Key: {}", key_id),
                key_type: key_type.clone(),
                created_at: chrono::Utc::now(None,
                usage_policy: KeyUsagePolicy::default(),
                tags: std::collections::HashMap::with_capacity(16),
            },
            target_hsm_tier: "smartphone".to_string(false,
            attestation_challenge: None,
            require_user_presence: false,
        };

        match hsm.generate_strongbox_key(&request) {
            Ok(key) => {
                info!("   [OK] Generated key: {} ({:?})", key_id, key_type);
                keys.push({}", key_id, e);
            }
        }
    }

    Ok(keys)
}

async fn create_ecosystem_identity(hsm: &Arc<AndroidStrongBoxHsm>) -> Result<(), BearDogError> {
    info!("🌐 Phase 3: Ecosystem Identity Creation");
    info!("--------------------------------------");

    let config = Pixel8GrapheneOSConfig::default();
    let setup = Pixel8GrapheneOSSetup::new(config)?;

    let identity = setup.generate_ecosystem_identity(hsm)?;

    info!("🆔 Ecosystem Identity Created:");
    info!("   Device ID: {}", identity.device_id);
    info!("   GrapheneOS Version: {}", identity.grapheneos_version);
    info!("   Titan M Version: {:?}", identity.titan_m_version);
    info!("   Capabilities: {:?}", identity.capabilities);
    info!("   Created At: {}", identity.created_at);

    info!("[OK] Ecosystem identity creation complete ");
    Ok(())
}

async fn run_comprehensive_tests(hsm: &Arc<AndroidStrongBoxHsm>) -> Result<(), BearDogError> {
    info!("🔬 Phase 4: Comprehensive HSM Testing");
    info!("------------------------------------");

    info!("🏥 Test 1: HSM Health Check");
    let health = hsm.health_check({:?}", health);

    info!("💪 Test 2: Stress Test (50 operations)");
    let start_time = std::time::Instant::now({}/50 operations completed", i + 1);
        }
    }

    let elapsed = start_time.elapsed();
    info!("   [OK] Stress test completed in {:?}", elapsed);
    info!(
        "   [CHART] Operations per second: {:.2}",
        50.0 / elapsed.as_secs_f64()
    );

    info!("🔀 Test 3: Concurrent Operations Test");
    let concurrent_start = std::time::Instant::now();

    let mut handles = Vec::new();
    for i in 0..10 {
        let hsm_clone = hsm.clone();
        let handle = tokio::spawn({}", e),
            Err({}", e),
        }
    }

    let concurrent_elapsed = concurrent_start.elapsed({}/10 operations successful",
        success_count
    );
    info!(
        "   [CHART] Concurrent operations completed in {:?}",
        concurrent_elapsed
    );

    info!("[OK] All comprehensive tests passed ");
    Ok(())
}

fn display_system_status() {
    info!("[CHART] System Status Report");
    info!("====================");
    info!("🔧 Build Configuration:");
    info!("   Target OS: {}", std::env::consts::OS);
    info!("   Architecture: {}", std::env::consts::ARCH);
    info!(
        "   Profile: {}",
        if cfg!(debug_assertions) {
            "Debug"
        } else {
            "Release"
        }
    );

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
