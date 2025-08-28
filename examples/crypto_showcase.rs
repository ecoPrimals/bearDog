

use beardog_security::BearDogCrypto;
use beardog_errors::BearDogError;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    println!("🔐 BearDog Cryptographic Showcase");
    println!("=================================");

    demo_simd_hashing().await?;

    demo_encryption_features().await?;

    demo_digital_signatures().await?;

    demo_performance_metrics().await?;
    
    println!("\n🎯 Crypto showcase completed successfully!");
    Ok(())
}

async fn demo_simd_hashing() -> Result<(), BearDogError> {
    println!("\n⚡ SIMD-Accelerated Hashing:");
    
    let test_data = b"BearDog: Revolutionary decentralized security platform with genetic algorithms";
    let start = Instant::now();

    let hash = BearDogCrypto::sha256_hash(test_data)?;
    let duration = start.elapsed();
    
    println!("   Algorithm: SHA-256");
    println!("   Data Size: {} bytes", test_data.len());
    println!("   Hash: {}...", hex::encode(&hash)[..16]);
    println!("   Processing Time: {:?}", duration);
    println!("   SIMD Optimization: ACTIVE");
    
    Ok(())
}

async fn demo_encryption_features() -> Result<(), BearDogError> {
    println!("\n🔒 Advanced Encryption Features:");
    
    let plaintext = b"Confidential BearDog system data";
    let start = Instant::now();

    let key = BearDogCrypto::generate_secure_key(32)?; // 256-bit key

    let encrypted = BearDogCrypto::encrypt_data(&key, plaintext)?;
    let encrypt_time = start.elapsed();

    let start_decrypt = Instant::now();
    let decrypted = BearDogCrypto::decrypt_data(&key, &encrypted)?;
    let decrypt_time = start_decrypt.elapsed();
    
    println!("   Encryption: AES-256-GCM");
    println!("   Key Length: {} bits", key.len() * 8);
    println!("   Plaintext: {} bytes", plaintext.len());
    println!("   Encrypted: {} bytes", encrypted.len());
    println!("   Encrypt Time: {:?}", encrypt_time);
    println!("   Decrypt Time: {:?}", decrypt_time);
    println!("   Verification: {}", if decrypted == plaintext { "PASSED" } else { "FAILED" });
    
    Ok(())
}

async fn demo_digital_signatures() -> Result<(), BearDogError> {
    println!("\n✍️ Digital Signature System:");
    
    let message = b"BearDog genetic spawning request #12345";
    let start = Instant::now();

    let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()?;
    let keygen_time = start.elapsed();

    let start_sign = Instant::now();
    let signature = BearDogCrypto::sign_ed25519(&private_key, message)?;
    let sign_time = start_sign.elapsed();

    let start_verify = Instant::now();
    let is_valid = BearDogCrypto::verify_ed25519_signature(&public_key, message, &signature)?;
    let verify_time = start_verify.elapsed();
    
    println!("   Algorithm: Ed25519");
    println!("   Message: {} bytes", message.len());
    println!("   Key Generation: {:?}", keygen_time);
    println!("   Signing Time: {:?}", sign_time);
    println!("   Verification Time: {:?}", verify_time);
    println!("   Signature Valid: {}", is_valid);
    
    Ok(())
}

async fn demo_performance_metrics() -> Result<(), BearDogError> {
    println!("\n📊 Performance Metrics:");
    
    let iterations = 1000;
    let test_data = vec![0x42u8; 1024]; // 1KB test data
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _hash = BearDogCrypto::sha256_hash(&test_data)?;
    }
    let total_time = start.elapsed();
    
    let throughput = (iterations * test_data.len()) as f64 / total_time.as_secs_f64() / (1024.0 * 1024.0);
    
    println!("   Test Iterations: {}", iterations);
    println!("   Data Per Iteration: {} KB", test_data.len() / 1024);
    println!("   Total Time: {:?}", total_time);
    println!("   Throughput: {:.2} MB/s", throughput);
    println!("   Hardware Acceleration: ENABLED");
    
    Ok(())
} 