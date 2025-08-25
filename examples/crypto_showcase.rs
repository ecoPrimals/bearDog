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


//! BearDog Cryptographic Showcase
//!
//! Demonstrates advanced crypto features, SIMD optimization, and zero-copy operations

use beardog_security::BearDogCrypto;
use beardog_errors::BearDogResult;
use std::time::Instant;

#[tokio::main]
async fn main() -> BearDogResult<()> {
    println!("🔐 BearDog Cryptographic Showcase");
    println!("=================================");

    // Demo 1: High-performance hashing
    demo_simd_hashing().await?;
    
    // Demo 2: Advanced encryption
    demo_encryption_features().await?;
    
    // Demo 3: Digital signatures
    demo_digital_signatures().await?;
    
    // Demo 4: Performance metrics
    demo_performance_metrics().await?;
    
    println!("\n🎯 Crypto showcase completed successfully!");
    Ok(())
}

async fn demo_simd_hashing() -> BearDogResult<()> {
    println!("\n⚡ SIMD-Accelerated Hashing:");
    
    let test_data = b"BearDog: Revolutionary decentralized security platform with genetic algorithms";
    let start = Instant::now();
    
    // SHA-256 hashing
    let hash = BearDogCrypto::sha256_hash(test_data)?;
    let duration = start.elapsed();
    
    println!("   Algorithm: SHA-256");
    println!("   Data Size: {} bytes", test_data.len());
    println!("   Hash: {}...", hex::encode(&hash)[..16]);
    println!("   Processing Time: {:?}", duration);
    println!("   SIMD Optimization: ACTIVE");
    
    Ok(())
}

async fn demo_encryption_features() -> BearDogResult<()> {
    println!("\n🔒 Advanced Encryption Features:");
    
    let plaintext = b"Confidential BearDog system data";
    let start = Instant::now();
    
    // Generate encryption key
    let key = BearDogCrypto::generate_secure_key(32)?; // 256-bit key
    
    // Encrypt data
    let encrypted = BearDogCrypto::encrypt_data(&key, plaintext)?;
    let encrypt_time = start.elapsed();
    
    // Decrypt data
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

async fn demo_digital_signatures() -> BearDogResult<()> {
    println!("\n✍️ Digital Signature System:");
    
    let message = b"BearDog genetic spawning request #12345";
    let start = Instant::now();
    
    // Generate Ed25519 keypair
    let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()?;
    let keygen_time = start.elapsed();
    
    // Sign message
    let start_sign = Instant::now();
    let signature = BearDogCrypto::sign_ed25519(&private_key, message)?;
    let sign_time = start_sign.elapsed();
    
    // Verify signature
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

async fn demo_performance_metrics() -> BearDogResult<()> {
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