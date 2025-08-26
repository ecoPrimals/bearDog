

use beardog_errors::BearDogResult;
use beardog_types::canonical::{
    crypto::KeyType,
    hsm::{
        traits::{
            CapabilityDiscoveryEngine, CryptoOperation, HsmRequirements, SecurityLevel,
            UniversalHsmProvider, MobileHsmProvider, AuthenticationMethod,
        },
        KeyMetadata,
    },
};
use beardog_tunnel::tunnel::hsm::providers::{
    AndroidUniversalProvider, UniversalProviderRegistry,
};
use std::collections::HashMap;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> BearDogResult<()> {
    tracing_subscriber::init();
    
    info!("🚀 Starting Universal HSM Architecture Demo");

    info!("\n📡 Step 1: Discovering available HSM providers...");
    
    let mut discovery_engine = CapabilityDiscoveryEngine::new();

    if let Ok(android_provider) = AndroidUniversalProvider::new().await {
        discovery_engine.register_provider(Box::new(android_provider));
        info!("✅ Android provider registered");
    } else {
        info!("❌ Android provider not available");
    }

    info!("\n🔍 Step 2: Scanning all providers for capabilities...");
    
    let discovered_providers = discovery_engine.discover_all().await?;
    
    for (vendor_info, capabilities) in &discovered_providers {
        info!("🏭 Found provider: {} {}", vendor_info.name, vendor_info.product);
        info!("   Security Level: {}", capabilities.security_level);
        info!("   Operations: {:?}", capabilities.crypto_operations);
        info!("   Key Types: {:?}", capabilities.supported_key_types);
        info!("   Auth Methods: {:?}", capabilities.authentication_methods);
        info!("   Hardware Features: Tamper Resistance = {:?}, True RNG = {}", 
              capabilities.hardware_features.tamper_resistance,
              capabilities.hardware_features.true_rng);
        info!("   Performance: {} keys/sec, {} sigs/sec", 
              capabilities.performance_profile.key_generation_speed,
              capabilities.performance_profile.signing_speed);
    }

    info!("\n📋 Step 3: Defining application security requirements...");
    
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
    
    info!("   Minimum Security Level: {}", requirements.min_security_level);
    info!("   Required Operations: {:?}", requirements.required_operations);

    info!("\n🎯 Step 4: Automatically selecting best provider...");
    
    let best_provider = discovery_engine.find_best_provider(&requirements).await?;
    let provider_info = best_provider.get_provider_info();
    
    info!("🏆 Selected provider: {} {} (score-based selection)", 
          provider_info.name, provider_info.product);

    info!("\n🔑 Step 5: Performing vendor-agnostic key operations...");

    if !best_provider.supports_operation(&CryptoOperation::KeyGeneration).await {
        warn!("❌ Provider doesn't support key generation");
        return Ok(());
    }

    let mut metadata = KeyMetadata::default();
    metadata.purpose = Some("demo_signing_key".to_string());
    metadata.tags.insert("demo".to_string(), "true".to_string());
    
    let key = best_provider.generate_key(
        KeyType::Ed25519,
        metadata,
        None, // No authentication for demo
    ).await?;
    
    info!("✅ Generated key: {} (hardware-backed: {})", 
          key.id, key.is_hardware_backed);

    let test_data = b"Hello, vendor-agnostic world!";
    let signature = best_provider.sign_data(&key.id, test_data, None).await?;
    
    info!("✅ Signed data: {} bytes signature", signature.len());

    let is_valid = best_provider.verify_signature(&key.id, test_data, &signature).await?;
    
    info!("✅ Signature verified: {}", is_valid);

    info!("\n📱 Step 6: Testing mobile-specific features...");

    if let Ok(mobile_provider) = best_provider as &dyn MobileHsmProvider {
        match mobile_provider.authenticate_biometric().await {
            Ok(token) => {
                info!("✅ Biometric authentication successful");
                info!("   Token expires: {}", token.expires_at);
            }
            Err(e) => {
                info!("❌ Biometric authentication not available: {}", e);
            }
        }
        
        match mobile_provider.require_user_presence("Demo user presence").await {
            Ok(()) => {
                info!("✅ User presence confirmed");
            }
            Err(e) => {
                info!("❌ User presence not available: {}", e);
            }
        }
    }

    info!("\n💚 Step 7: Health monitoring...");
    
    let health = best_provider.health_check().await?;
    info!("Provider health: {:?}", health);

    info!("\n🔄 Step 8: Demonstrating seamless vendor switching...");

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
    
    match discovery_engine.find_best_provider(&high_security_requirements).await {
        Ok(high_sec_provider) => {
            let info = high_sec_provider.get_provider_info();
            info!("🔒 High security provider available: {} {}", info.name, info.product);

        }
        Err(e) => {
            info!("⚠️  No high-security provider available: {}", e);
            info!("   Application can gracefully degrade or request user to enable hardware security");
        }
    }
    
    info!("\n🎉 Universal HSM Architecture Demo Complete!");
    info!("\n📊 BENEFITS DEMONSTRATED:");
    info!("   ✅ Zero vendor lock-in - works with ANY HSM vendor");
    info!("   ✅ Runtime capability discovery - no hardcoded assumptions");
    info!("   ✅ Automatic provider selection based on requirements");
    info!("   ✅ Graceful degradation when features unavailable");
    info!("   ✅ Seamless vendor switching without code changes");
    info!("   ✅ Type-safe operations with compile-time guarantees");
    info!("   ✅ Mobile-specific features when available");
    info!("   ✅ Comprehensive health monitoring");
    
    Ok(())
}

async fn simulate_ios_provider() -> BearDogResult<()> {
    info!("📱 iOS Secure Enclave would be detected here");
    info!("   - Face ID / Touch ID integration");
    info!("   - Hardware-backed key generation");
    info!("   - App attestation");
    info!("   - Secure boot verification");
    Ok(())
}

async fn simulate_pkcs11_provider() -> BearDogResult<()> {
    info!("🏭 PKCS#11 HSM would be detected here");
    info!("   - Thales, Utimaco, AWS CloudHSM, etc.");
    info!("   - FIPS 140-2 Level 3+ certification");
    info!("   - Hardware tamper resistance");
    info!("   - High-performance crypto operations");
    Ok(())
}

async fn simulate_tpm_provider() -> BearDogResult<()> {
    info!("🔐 TPM 2.0 would be detected here");
    info!("   - Platform attestation");
    info!("   - Measured boot");
    info!("   - Hardware-backed keys");
    info!("   - Windows Hello integration");
    Ok(())
} 