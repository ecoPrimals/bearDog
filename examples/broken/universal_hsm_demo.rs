use beardog_errors::BearDogError;
use beardog_tunnel::tunnel::hsm::providers::{AndroidUniversalProvider, UniversalProviderRegistry};
use beardog_types::canonical::{
    crypto::KeyType,
    hsm::{
        traits::{
            AuthenticationMethod, CapabilityDiscoveryEngine, CryptoOperation, HsmRequirements,
            MobileHsmProvider, SecurityLevel, UniversalHsmProvider,
        },
        KeyMetadata,
    },
};
use std::collections::HashMap;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    tracing_subscriber::init();

    info!("[ROCKET] Starting Universal HSM Architecture Demo");

    info!("📡 Step 1: Discovering available HSM providers...");

    let mut discovery_engine = CapabilityDiscoveryEngine::new();

    if let Ok(android_provider) = AndroidUniversalProvider::new() {
        discovery_engine.register_provider(Box::new(android_provider));
        info!("[OK] Android provider registered");
    } else {
        info!("[X] Android provider not available");
    }

    info!("[SEARCH] Step 2: Scanning all providers for capabilities...");

    let discovered_providers = discovery_engine.discover_all({} {}",
            vendor_info.name, vendor_info.product
        );
        info!("   Security Level: {}", capabilities.security_level);
        info!("   Operations: {:?}", capabilities.crypto_operations);
        info!("   Key Types: {:?}", capabilities.supported_key_types);
        info!("   Auth Methods: {:?}", capabilities.authentication_methods);
        info!(
            "   Hardware Features: Tamper Resistance = {:?}, True RNG = {}",
            capabilities.hardware_features.tamper_resistance,
            capabilities.hardware_features.true_rng
        );
        info!(
            "   Performance: {} keys/sec, {} sigs/sec",
            capabilities.performance_profile.key_generation_speed,
            capabilities.performance_profile.signing_speed
        );
    }

    info!("📋 Step 3: Defining application security requirements...");

    let requirements = HsmRequirements {
        min_security_level: SecurityLevel::Tee, // Minimum TEE required
        required_operations: vec![
            CryptoOperation::KeyGeneration,
            CryptoOperation::DigitalSigning,
            CryptoOperation::SignatureVerification,
        ],
        preferred_key_types: vec![KeyType::Ed25519, KeyType::EcdsaP256],
        authentication_preference: Some(AuthenticationMethod::Biometric),
        performance_requirements: None,
    };

    info!(
        "   Minimum Security Level: {}",
        requirements.min_security_level
    );
    info!(
        "   Required Operations: {:?}",
        requirements.required_operations
    );

    info!("[TARGET] Step 4: Automatically selecting best provider...");

    let best_provider = discovery_engine.find_best_provider(&requirements)?;
    let provider_info = best_provider.get_provider_info();

    info!(
        "[TROPHY] Selected provider: {} {} (score-based selection)",
        provider_info.name, provider_info.product
    );

    info!("🔑 Step 5: Performing vendor-agnostic key operations...");

    if !best_provider
        .supports_operation(&CryptoOperation::KeyGeneration)
    {
        warn!("[X] Provider doesn't support key generation");
        return Ok(());
    }

    let mut metadata = KeyMetadata::default();
    metadata.purpose = Some("demo_signing_key".to_string());
    metadata.tags.insert("demo".to_string(), "true".to_string());

    let key = best_provider
        .generate_key(
            KeyType::Ed25519,
            metadata,
            None, // No authentication for demo
        )
        ?;

    info!(
        "[OK] Generated key: {} (hardware-backed: {})",
        key.id, key.is_hardware_backed
    );

    let test_data = b"Hello, vendor-agnostic world!";
    let signature = best_provider.sign_data({} bytes signature", signature.len({}", is_valid);

    info!("📱 Step 6: Testing mobile-specific features...");

    if let Ok({}", token.expires_at);
            }
            Err({}", e);
            }
        }

        match mobile_provider
            .require_user_presence({}", e);
            }
        }
    }

    info!("💚 Step 7: Health monitoring...");

    let health = best_provider.health_check({:?}", health);

    info!("[CYCLE] Step 8: Demonstrating seamless vendor switching...");

    let high_security_requirements = HsmRequirements {
        min_security_level: SecurityLevel::Hardware, // Require hardware
        required_operations: vec![
            CryptoOperation::KeyGeneration,
            CryptoOperation::Attestation, // Require attestation
        ],
        preferred_key_types: vec![KeyType::Ed25519],
        authentication_preference: Some(AuthenticationMethod::Biometric),
        performance_requirements: None,
    };

    match discovery_engine
        .find_best_provider({} {}",
                info.name, info.product
            );
        }
        Err({}", e);
            info!(
                "   Application can gracefully degrade or request user to enable hardware security"
            );
        }
    }

    info!("[PARTY] Universal HSM Architecture Demo Complete!");
    info!("[CHART] BENEFITS DEMONSTRATED:");
    info!("   [OK] Zero vendor lock-in - works with ANY HSM vendor");
    info!("   [OK] Runtime capability discovery - no hardcoded assumptions");
    info!("   [OK] Automatic provider selection based on requirements");
    info!("   [OK] Graceful degradation when features unavailable");
    info!("   [OK] Seamless vendor switching without code changes");
    info!("   [OK] Type-safe operations with compile-time guarantees");
    info!("   [OK] Mobile-specific features when available");
    info!("   [OK] Comprehensive health monitoring");

    Ok(())
}

async fn simulate_ios_provider() -> Result<(), BearDogError> {
    info!("📱 iOS Secure Enclave would be detected here");
    info!("   - Face ID / Touch ID integration");
    info!("   - Hardware-backed key generation");
    info!("   - App attestation");
    info!("   - Secure boot verification");
    Ok(())
}

async fn simulate_pkcs11_provider() -> Result<(), BearDogError> {
    info!("🏭 PKCS#11 HSM would be detected here");
    info!("   - Thales, Utimaco, AWS CloudHSM, etc.");
    info!("   - FIPS 140-2 Level 3+ certification");
    info!("   - Hardware tamper resistance");
    info!("   - High-performance crypto operations");
    Ok(())
}

async fn simulate_tpm_provider() -> Result<(), BearDogError> {
    info!("🔐 TPM 2.0 would be detected here");
    info!("   - Platform attestation");
    info!("   - Measured boot");
    info!("   - Hardware-backed keys");
    info!("   - Windows Hello integration");
    Ok(())
}
