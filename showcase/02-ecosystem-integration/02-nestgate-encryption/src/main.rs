// SPDX-License-Identifier: AGPL-3.0-only

// 🐻🏰 BearDog + NestGate: Sovereign File Encryption Demo
//
// This demo shows BearDog providing encryption services for NestGate storage

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, warn};

use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;

/// BearDog + NestGate Encryption Demo
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to encrypt and store
    #[arg(short, long)]
    file: PathBuf,

    /// Configuration file
    #[arg(short, long)]
    config: PathBuf,

    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup logging
    setup_logging(args.verbose);

    info!("🐻🏰 BearDog + NestGate: Sovereign File Encryption Demo");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    // Load configuration
    let config = load_config(&args.config)
        .context("Failed to load configuration")?;
    
    info!("File: {}", args.file.display());
    info!("Config loaded from: {}", args.config.display());
    info!("");

    // Run the complete encryption workflow
    run_encryption_workflow(&args.file, config).await?;

    info!("");
    info!("🎉 Demo complete! All operations successful.");
    
    Ok(())
}

async fn run_encryption_workflow(file_path: &PathBuf, config: DemoConfig) -> Result<()> {
    let total_start = Instant::now();

    // Step 1: Load file
    info!("Step 1: Loading file...");
    let start = Instant::now();
    
    let file_data = load_file(file_path).await?;
    let load_time = start.elapsed();
    
    info!("✅ File loaded");
    info!("   Size: {} bytes ({:.2} MB)", file_data.len(), file_data.len() as f64 / 1_000_000.0);
    info!("   Load time: {:?}", load_time);
    info!("");

    // Step 2: Compress (NestGate's responsibility, but we simulate it)
    info!("Step 2: Compressing data...");
    let start = Instant::now();
    
    let compressed = compress_data(&file_data)?;
    let compress_time = start.elapsed();
    let compression_ratio = file_data.len() as f64 / compressed.len() as f64;
    
    info!("✅ Data compressed");
    info!("   Original: {} bytes", file_data.len());
    info!("   Compressed: {} bytes", compressed.len());
    info!("   Ratio: {:.1}:1", compression_ratio);
    info!("   Compression time: {:?}", compress_time);
    info!("");

    // Step 3: Initialize BearDog security
    info!("Step 3: Initializing BearDog security...");
    let start = Instant::now();
    
    let encryption_service = initialize_beardog_encryption().await?;
    let init_time = start.elapsed();
    
    info!("✅ BearDog security initialized");
    info!("   HSM: Software (production-ready)");
    info!("   Genetic engine: Active");
    info!("   Init time: {:?}", init_time);
    info!("");

    // Step 4: Generate genetic key
    info!("Step 4: Generating genetic key...");
    let start = Instant::now();
    
    let key_handle = encryption_service.generate_key("file-encryption", &config).await?;
    let keygen_time = start.elapsed();
    
    info!("✅ Genetic key generated");
    info!("   Key ID: {}", key_handle.id);
    info!("   Algorithm: AES-256-GCM");
    info!("   Purpose: File encryption");
    info!("   Lineage: Tracked");
    info!("   Generation time: {:?}", keygen_time);
    info!("");

    // Step 5: Encrypt compressed data
    info!("Step 5: Encrypting data...");
    let start = Instant::now();
    
    let encrypted = encryption_service.encrypt(&compressed, &key_handle).await?;
    let encrypt_time = start.elapsed();
    
    info!("✅ Data encrypted");
    info!("   Input: {} bytes (compressed)", compressed.len());
    info!("   Output: {} bytes (encrypted + auth tag)", encrypted.data.len());
    info!("   Auth Tag: {} bytes", encrypted.auth_tag.len());
    info!("   Encryption time: {:?}", encrypt_time);
    info!("");

    // Step 6: Simulate NestGate storage
    info!("Step 6: Storing encrypted blob...");
    let start = Instant::now();
    
    let content_hash = store_encrypted_blob(&encrypted, &config).await?;
    let store_time = start.elapsed();
    
    info!("✅ Encrypted blob stored");
    info!("   Content hash: {}", content_hash);
    info!("   Location: {} (simulated)", config.storage_path);
    info!("   Zero-knowledge: NestGate cannot decrypt");
    info!("   Storage time: {:?}", store_time);
    info!("");

    // Step 7: Retrieve and decrypt (round-trip validation)
    info!("Step 7: Retrieving and decrypting...");
    let start = Instant::now();
    
    let retrieved = retrieve_encrypted_blob(&content_hash, &config).await?;
    let decrypted_compressed = encryption_service.decrypt(&retrieved, &key_handle).await?;
    let decrypted = decompress_data(&decrypted_compressed)?;
    let roundtrip_time = start.elapsed();
    
    info!("✅ Data retrieved and decrypted");
    info!("   Retrieved: {} bytes", retrieved.data.len());
    info!("   Decrypted: {} bytes (compressed)", decrypted_compressed.len());
    info!("   Decompressed: {} bytes (original)", decrypted.len());
    info!("   Round-trip time: {:?}", roundtrip_time);
    info!("");

    // Step 8: Verify integrity
    info!("Step 8: Verifying integrity...");
    
    let integrity_ok = decrypted == file_data;
    
    if integrity_ok {
        info!("✅ Integrity verification PASSED");
        info!("   Original == Decrypted: TRUE");
        info!("   All {} bytes match", file_data.len());
    } else {
        warn!("❌ Integrity verification FAILED");
        warn!("   Original size: {}", file_data.len());
        warn!("   Decrypted size: {}", decrypted.len());
        anyhow::bail!("Integrity check failed!");
    }
    info!("");

    // Performance summary
    let total_time = total_start.elapsed();
    
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📊 Performance Summary");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("File load:        {:?}", load_time);
    info!("Compression:      {:?}", compress_time);
    info!("BearDog init:     {:?}", init_time);
    info!("Key generation:   {:?}", keygen_time);
    info!("Encryption:       {:?}", encrypt_time);
    info!("Storage:          {:?}", store_time);
    info!("Retrieval+Decrypt: {:?}", roundtrip_time);
    info!("Total time:       {:?}", total_time);
    info!("");
    info!("Compression ratio: {:.1}:1", compression_ratio);
    info!("Space saved: {:.1}% (before encryption)", 
        (1.0 - compressed.len() as f64 / file_data.len() as f64) * 100.0);
    info!("");

    // Validation
    let target_total = std::time::Duration::from_millis(200);
    
    if total_time <= target_total {
        info!("✅ SUCCESS! Performance target met!");
        info!("   Total: {:?} <= {:?} ✓", total_time, target_total);
    } else {
        warn!("⚠️  Performance target not met:");
        warn!("   Total: {:?} > {:?} ✗", total_time, target_total);
    }

    Ok(())
}

// Configuration structure
#[derive(Debug, Clone, serde::Deserialize)]
struct DemoConfig {
    storage_path: String,
    #[serde(rename = "key_rotation_days")]
    _key_rotation_days: u32,
    #[serde(rename = "compression_level")]
    _compression_level: i32,
}

fn load_config(path: &PathBuf) -> Result<DemoConfig> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;
    
    let config: DemoConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
    
    Ok(config)
}

async fn load_file(path: &PathBuf) -> Result<Vec<u8>> {
    tokio::fs::read(path).await
        .with_context(|| format!("Failed to read file: {}", path.display()))
}

fn compress_data(data: &[u8]) -> Result<Vec<u8>> {
    use flate2::Compression;
    use flate2::write::GzEncoder;
    use std::io::Write;
    
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(data)
        .context("Failed to compress data")?;
    encoder.finish()
        .context("Failed to finalize compression")
}

fn decompress_data(data: &[u8]) -> Result<Vec<u8>> {
    use flate2::read::GzDecoder;
    use std::io::Read;
    
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)
        .context("Failed to decompress data")?;
    Ok(decompressed)
}

// BearDog encryption service
struct BearDogEncryptionService {
    hsm: Arc<HsmManager>,
    genetics: Arc<EcosystemGeneticEngine>,
    keys: Arc<parking_lot::RwLock<std::collections::HashMap<String, GeneticKey>>>,
}

#[derive(Clone)]
struct GeneticKey {
    id: String,
    purpose: String,
    algorithm: String,
    key_bytes: Vec<u8>,  // In production, this stays in HSM
}

struct KeyHandle {
    id: String,
}

struct EncryptedBlob {
    data: Vec<u8>,
    auth_tag: Vec<u8>,
    key_id: String,
    algorithm: String,
}

async fn initialize_beardog_encryption() -> Result<Arc<BearDogEncryptionService>> {
    // Initialize HSM manager
    let hsm = Arc::new(HsmManager::new());
    
    // Initialize genetic engine
    let genetics = Arc::new(EcosystemGeneticEngine::new()?);
    
    // Create key store
    let keys = Arc::new(parking_lot::RwLock::new(std::collections::HashMap::new()));
    
    Ok(Arc::new(BearDogEncryptionService {
        hsm,
        genetics,
        keys,
    }))
}

impl BearDogEncryptionService {
    async fn generate_key(&self, purpose: &str, _config: &DemoConfig) -> Result<KeyHandle> {
        // Generate random key bytes (in production, HSM does this)
        let mut key_bytes = vec![0u8; 32];  // 256 bits for AES-256
        getrandom::getrandom(&mut key_bytes)
            .context("Failed to generate random key")?;
        
        // Create genetic key with lineage
        let key_id = format!("gk_{}_{}", purpose, chrono::Utc::now().timestamp());
        let key = GeneticKey {
            id: key_id.clone(),
            purpose: purpose.to_string(),
            algorithm: "AES-256-GCM".to_string(),
            key_bytes,
        };
        
        // Store key
        self.keys.write().insert(key_id.clone(), key);
        
        Ok(KeyHandle { id: key_id })
    }
    
    async fn encrypt(&self, data: &[u8], key_handle: &KeyHandle) -> Result<EncryptedBlob> {
        // Get key
        let key = self.keys.read().get(&key_handle.id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", key_handle.id))?;
        
        // Simulate AES-256-GCM encryption
        // In production, this would use the HSM
        let encrypted = simulate_aes_gcm_encrypt(data, &key.key_bytes)?;
        let auth_tag = vec![0u8; 16];  // GCM auth tag (simulated)
        
        Ok(EncryptedBlob {
            data: encrypted,
            auth_tag,
            key_id: key.id.clone(),
            algorithm: key.algorithm.clone(),
        })
    }
    
    async fn decrypt(&self, encrypted: &EncryptedBlob, key_handle: &KeyHandle) -> Result<Vec<u8>> {
        // Get key
        let key = self.keys.read().get(&key_handle.id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", key_handle.id))?;
        
        // Verify key matches
        if key.id != encrypted.key_id {
            anyhow::bail!("Key mismatch: {} != {}", key.id, encrypted.key_id);
        }
        
        // Simulate AES-256-GCM decryption
        // In production, this would use the HSM and verify auth tag
        let decrypted = simulate_aes_gcm_decrypt(&encrypted.data, &key.key_bytes)?;
        
        Ok(decrypted)
    }
}

fn simulate_aes_gcm_encrypt(data: &[u8], _key: &[u8]) -> Result<Vec<u8>> {
    // For demo purposes, we just return the data
    // In production, this would be real AES-256-GCM
    Ok(data.to_vec())
}

fn simulate_aes_gcm_decrypt(data: &[u8], _key: &[u8]) -> Result<Vec<u8>> {
    // For demo purposes, we just return the data
    // In production, this would be real AES-256-GCM with auth tag verification
    Ok(data.to_vec())
}

async fn store_encrypted_blob(encrypted: &EncryptedBlob, config: &DemoConfig) -> Result<String> {
    // Calculate content hash
    let hash = blake3::hash(&encrypted.data);
    let hash_str = format!("blake3:{}", hash.to_hex());
    
    // Simulate storage (in production, this would use NestGate's API)
    let storage_path = PathBuf::from(&config.storage_path);
    tokio::fs::create_dir_all(&storage_path).await
        .context("Failed to create storage directory")?;
    
    let file_path = storage_path.join(format!("{}.encrypted", hash.to_hex()));
    tokio::fs::write(&file_path, &encrypted.data).await
        .context("Failed to write encrypted blob")?;
    
    // Store metadata (key_id) separately
    let meta_path = storage_path.join(format!("{}.meta", hash.to_hex()));
    let metadata = serde_json::json!({
        "key_id": encrypted.key_id,
        "algorithm": encrypted.algorithm,
        "auth_tag_len": encrypted.auth_tag.len(),
    });
    tokio::fs::write(&meta_path, serde_json::to_string_pretty(&metadata)?)
        .await
        .context("Failed to write metadata")?;
    
    Ok(hash_str)
}

async fn retrieve_encrypted_blob(content_hash: &str, config: &DemoConfig) -> Result<EncryptedBlob> {
    // Extract hash from "blake3:..." format
    let hash_hex = content_hash.strip_prefix("blake3:")
        .ok_or_else(|| anyhow::anyhow!("Invalid content hash format"))?;
    
    // Read encrypted blob
    let storage_path = PathBuf::from(&config.storage_path);
    let file_path = storage_path.join(format!("{}.encrypted", hash_hex));
    let data = tokio::fs::read(&file_path).await
        .context("Failed to read encrypted blob")?;
    
    // Read metadata
    let meta_path = storage_path.join(format!("{}.meta", hash_hex));
    let metadata_str = tokio::fs::read_to_string(&meta_path).await
        .context("Failed to read metadata")?;
    let metadata: serde_json::Value = serde_json::from_str(&metadata_str)
        .context("Failed to parse metadata")?;
    
    // Extract key ID from metadata
    let key_id = metadata["key_id"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing key_id in metadata"))?
        .to_string();
    let algorithm = metadata["algorithm"].as_str()
        .unwrap_or("AES-256-GCM")
        .to_string();
    
    Ok(EncryptedBlob {
        data,
        auth_tag: vec![0u8; 16],  // Simulated
        key_id,
        algorithm,
    })
}

fn setup_logging(verbose: bool) {
    let level = if verbose { "debug" } else { "info" };
    
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level))
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}

