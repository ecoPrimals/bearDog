use beardog_errors::BearDogError;
use beardog_security::BearDogCrypto;
use beardog_types::canonical::licensing::LicenseManager;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    tracing_subscriber::fmt::init();

    println!("🛡️ BearDog Production Security Demonstration");
    println!("===========================================");

    demonstrate_ed25519_cryptography()?;
    demonstrate_secure_nonce_generation()?;
    demonstrate_licensing_security()?;

    println!("🏆 All security demonstrations completed successfully!");
    Ok(())
}

async fn demonstrate_ed25519_cryptography() -> Result<(), BearDogError> {
    println!("🔐 Ed25519 Cryptographic Operations Demo");
    println!("----------------------------------------");

    let test_message = b"BearDog Production Security Test Message";
    
    // Generate Ed25519 key pair
    let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()?;
    
    // Sign the message
    let signature = BearDogCrypto::sign_ed25519(&private_key, test_message)?;
    
    // Display information
    if let Ok(message_str) = std::str::from_utf8(test_message) {
        println!("📝 Message: {:?}", message_str);
    } else {
        println!("📝 Message: [binary data]");
    }
    println!("🔑 Public key length: {} bytes", public_key.len());
    println!("✍️ Signature length: {} bytes", signature.len());

    let is_valid = BearDogCrypto::verify_ed25519_signature(&public_key, test_message, &signature)?;

    if is_valid {
        println!("[OK] Signature verification: VALID (real cryptographic verification)");
    } else {
        println!("[X] Signature verification: INVALID");
    }

    let mut invalid_signature = signature.clone();
    invalid_signature[0] ^= 0xFF; // Corrupt the signature

    let is_invalid =
        BearDogCrypto::verify_ed25519_signature(&public_key, test_message, &invalid_signature)?;

    println!(
        "[SEARCH] Invalid signature test: {} (should be false)",
        is_invalid
    );

    Ok(())
}

async fn demonstrate_secure_nonce_generation() -> Result<(), BearDogError> {
    println!("🎲 Secure Nonce Generation Demo");
    println!("--------------------------------");

    let mut nonces = Vec::new();
    
    // Generate multiple nonces of different sizes
    for i in 0..5 {
        let nonce = BearDogCrypto::generate_secure_nonce(32)?;
        println!("🎯 Nonce {}: {}", i + 1, hex::encode(&nonce));
        nonces.push(nonce);
    }

    // Generate a large nonce for demonstration
    let large_nonce = BearDogCrypto::generate_secure_nonce(64)?;
    println!("🎯 Large nonce (64 bytes): {}", hex::encode(&large_nonce));

    Ok(())
}

async fn demonstrate_licensing_security() -> Result<(), BearDogError> {
    println!("📜 Licensing System Security Demo");
    println!("----------------------------------");

    let license_manager = LicenseManager::new()?;
    
    // Test different license types
    let license_types = vec!["basic", "professional", "enterprise"];
    
    for license_type in license_types {
        println!("🔍 Testing {} license...", license_type);
        match license_manager.validate_license(license_type) {
            Ok(valid) => {
                if valid {
                    println!("✅ License type '{}' is valid", license_type);
                } else {
                    println!("❌ License type '{}' is invalid", license_type);
                }
            }
            Err(e) => {
                println!("⚠️ Error validating license '{}': {}", license_type, e);
            }
        }
    }

    println!("🏢 Testing enterprise license classification...");
    
    let enterprise_features = license_manager.get_enterprise_features()?;
    println!("🎯 Enterprise features available: {}", enterprise_features.len());
    
    for feature in enterprise_features {
        println!("  - {}", feature);
    }

    Ok(())
}
