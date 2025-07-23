//! Address Management - Keys ARE the Authority
//!
//! This module implements BearDog's core principle: Keys are the source of authority,
//! addresses are derived from keys, and no external validation is required.
//!
//! ## Core Principle: Self-Sovereign Address Generation
//!
//! - **Keys generate addresses** - Addresses are cryptographically derived from keys
//! - **No external dependencies** - Address generation is purely mathematical
//! - **Multiple formats supported** - Bitcoin, Ethereum, custom BearDog addresses
//! - **Deterministic derivation** - Same key always generates same address
//! - **Hierarchical support** - HD wallet-style key/address trees

use ripemd::Ripemd160;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// Note: BearDogCrypto not currently used but available for future cryptographic operations
use beardog_errors::{BearDogError, BearDogResult};

/// Address management system
pub struct AddressManager {
    /// Cached address derivations for performance
    address_cache: HashMap<String, CachedAddress>,
    /// Supported address formats
    supported_formats: Vec<AddressFormat>,
}

/// Cached address information
#[derive(Debug, Clone)]
struct CachedAddress {
    /// The derived address
    address: String,
    /// Format used for derivation
    format: AddressFormat,
    /// Source key fingerprint (for cache invalidation)
    #[allow(dead_code)]
    key_fingerprint: String,
    /// Derivation path (if applicable)
    derivation_path: Option<String>,
    /// Creation timestamp (for cache expiration)
    #[allow(dead_code)]
    created_at: chrono::DateTime<chrono::Utc>,
}

/// Address formats supported by BearDog
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AddressFormat {
    /// Bitcoin Legacy (P2PKH) - starts with '1'
    BitcoinLegacy,
    /// Bitcoin SegWit (P2WPKH) - starts with 'bc1q'
    BitcoinSegWit,
    /// Ethereum addresses - starts with '0x'
    Ethereum,
    /// BearDog native format - starts with 'bear'
    BearDogNative,
    /// Ed25519 public key hash - starts with 'ed25519'
    Ed25519Hash,
    /// Custom format for specific use cases
    Custom { name: String, prefix: String },
}

/// Address with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInfo {
    /// The address string
    pub address: String,
    /// Format used
    pub format: AddressFormat,
    /// Associated public key (if applicable)
    pub public_key: Option<Vec<u8>>,
    /// Derivation path (for HD addresses)
    pub derivation_path: Option<String>,
    /// Address metadata
    pub metadata: HashMap<String, String>,
    /// Verification status
    pub verified: bool,
}

/// HD (Hierarchical Deterministic) key derivation path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivationPath {
    /// Path components (e.g., [44, 0, 0, 0, 0] for m/44'/0'/0'/0/0)
    pub components: Vec<u32>,
    /// Whether components are hardened
    pub hardened: Vec<bool>,
}

impl AddressManager {
    /// Create new address manager
    pub fn new() -> Self {
        Self {
            address_cache: HashMap::new(),
            supported_formats: vec![
                AddressFormat::BitcoinLegacy,
                AddressFormat::BitcoinSegWit,
                AddressFormat::Ethereum,
                AddressFormat::BearDogNative,
                AddressFormat::Ed25519Hash,
            ],
        }
    }

    /// Generate address from Ed25519 public key
    pub fn generate_address_from_ed25519(
        &mut self,
        public_key: &[u8],
        format: AddressFormat,
    ) -> BearDogResult<AddressInfo> {
        if public_key.len() != 32 {
            return Err(BearDogError::InvalidInput {
                message: format!("Invalid Ed25519 public key length: {}", public_key.len()),
            });
        }

        let key_fingerprint = self.create_key_fingerprint(public_key);
        let cache_key = format!("{key_fingerprint}:{format:?}");

        // Check cache first
        if let Some(cached) = self.address_cache.get(&cache_key) {
            return Ok(AddressInfo {
                address: cached.address.clone(),
                format: cached.format.clone(),
                public_key: Some(public_key.to_vec()),
                derivation_path: cached.derivation_path.clone(),
                metadata: HashMap::new(),
                verified: true,
            });
        }

        // Generate address based on format
        let address = match format {
            AddressFormat::BearDogNative => self.generate_beardog_address(public_key)?,
            AddressFormat::Ed25519Hash => self.generate_ed25519_hash_address(public_key)?,
            AddressFormat::Ethereum => self.generate_ethereum_style_address(public_key)?,
            AddressFormat::BitcoinLegacy => self.generate_bitcoin_legacy_address(public_key)?,
            AddressFormat::BitcoinSegWit => self.generate_bitcoin_segwit_address(public_key)?,
            AddressFormat::Custom {
                ref name,
                ref prefix,
            } => self.generate_custom_address(public_key, name, prefix)?,
        };

        // Cache the result
        let cached_address = CachedAddress {
            address: address.clone(),
            format: format.clone(),
            key_fingerprint,
            derivation_path: None,
            created_at: chrono::Utc::now(),
        };
        self.address_cache.insert(cache_key, cached_address);

        Ok(AddressInfo {
            address,
            format,
            public_key: Some(public_key.to_vec()),
            derivation_path: None,
            metadata: HashMap::new(),
            verified: true,
        })
    }

    /// Generate address from raw key material (for other key types)
    pub fn generate_address_from_key_material(
        &mut self,
        key_material: &[u8],
        key_type: &str,
        format: AddressFormat,
    ) -> BearDogResult<AddressInfo> {
        match key_type {
            "Ed25519" => {
                if key_material.len() == 64 {
                    // Assume it's a keypair, extract public key
                    self.generate_address_from_ed25519(&key_material[32..], format)
                } else if key_material.len() == 32 {
                    // Assume it's just the public key
                    self.generate_address_from_ed25519(key_material, format)
                } else {
                    Err(BearDogError::InvalidInput {
                        message: format!(
                            "Invalid Ed25519 key material length: {}",
                            key_material.len()
                        ),
                    })
                }
            }
            "AES-256" => {
                // For symmetric keys, generate address from key hash
                self.generate_address_from_hash(key_material, format)
            }
            _ => {
                // For unknown key types, use hash-based address generation
                self.generate_address_from_hash(key_material, format)
            }
        }
    }

    /// Verify that an address was derived from a specific key
    pub fn verify_address_ownership(
        &mut self,
        address: &str,
        public_key: &[u8],
        format: AddressFormat,
    ) -> BearDogResult<bool> {
        // Generate address from the key and compare
        let derived_address = self.generate_address_from_ed25519(public_key, format)?;
        Ok(derived_address.address == address)
    }

    /// Generate hierarchical deterministic address
    pub fn generate_hd_address(
        &mut self,
        master_key: &[u8],
        derivation_path: &DerivationPath,
        format: AddressFormat,
    ) -> BearDogResult<AddressInfo> {
        // Derive child key from master key using derivation path
        let child_key = self.derive_child_key(master_key, derivation_path)?;

        // Generate address from child key
        let mut address_info = self.generate_address_from_ed25519(&child_key, format)?;
        address_info.derivation_path = Some(self.derivation_path_to_string(derivation_path));

        Ok(address_info)
    }

    /// Get supported address formats
    pub fn supported_formats(&self) -> &[AddressFormat] {
        &self.supported_formats
    }

    /// Add custom address format
    pub fn add_custom_format(&mut self, name: String, prefix: String) {
        let custom_format = AddressFormat::Custom { name, prefix };
        if !self.supported_formats.contains(&custom_format) {
            self.supported_formats.push(custom_format);
        }
    }

    /// Clear address cache
    pub fn clear_cache(&mut self) {
        self.address_cache.clear();
    }

    // Private implementation methods

    /// Generate BearDog native address format
    fn generate_beardog_address(&self, public_key: &[u8]) -> BearDogResult<String> {
        // BearDog format: bear + base58(hash160(pubkey)) + checksum
        let hash160 = self.hash160(public_key);
        let address_bytes = self.add_checksum(&hash160, b"bear")?;
        Ok(format!("bear{}", self.base58_encode(&address_bytes)))
    }

    /// Generate Ed25519 hash address
    fn generate_ed25519_hash_address(&self, public_key: &[u8]) -> BearDogResult<String> {
        // Ed25519 format: ed25519 + hex(sha256(pubkey)[:20])
        let mut hasher = Sha256::new();
        hasher.update(public_key);
        let hash = hasher.finalize();
        Ok(format!("ed25519{}", hex::encode(&hash[..20])))
    }

    /// Generate Ethereum-style address
    fn generate_ethereum_style_address(&self, public_key: &[u8]) -> BearDogResult<String> {
        // Ethereum format: 0x + hex(keccak256(pubkey)[-20:])
        let hash = self.keccak256(public_key);
        Ok(format!("0x{}", hex::encode(&hash[12..])))
    }

    /// Generate Bitcoin legacy address (P2PKH)
    fn generate_bitcoin_legacy_address(&self, public_key: &[u8]) -> BearDogResult<String> {
        // Bitcoin format: base58(version + hash160(pubkey) + checksum)
        let mut address_bytes = vec![0x00]; // Version byte for mainnet
        address_bytes.extend_from_slice(&self.hash160(public_key));
        let checksum = self.double_sha256_checksum(&address_bytes);
        address_bytes.extend_from_slice(&checksum[..4]);
        Ok(self.base58_encode(&address_bytes))
    }

    /// Generate Bitcoin SegWit address (P2WPKH)
    fn generate_bitcoin_segwit_address(&self, public_key: &[u8]) -> BearDogResult<String> {
        // SegWit format: bech32 encoding of witness program
        let hash160 = self.hash160(public_key);
        Ok(format!("bc1q{}", self.bech32_encode(&hash160)))
    }

    /// Generate custom format address
    fn generate_custom_address(
        &self,
        public_key: &[u8],
        name: &str,
        prefix: &str,
    ) -> BearDogResult<String> {
        // Custom format: prefix + hash + checksum
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        hasher.update(public_key);
        let hash = hasher.finalize();
        Ok(format!("{}{}", prefix, hex::encode(&hash[..16])))
    }

    /// Generate address from hash (for non-public key material)
    fn generate_address_from_hash(
        &self,
        data: &[u8],
        format: AddressFormat,
    ) -> BearDogResult<AddressInfo> {
        let hash = {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.finalize().to_vec()
        };

        let address = match format {
            AddressFormat::BearDogNative => {
                format!("bear{}", hex::encode(&hash[..20]))
            }
            AddressFormat::Ed25519Hash => {
                format!("ed25519{}", hex::encode(&hash[..20]))
            }
            AddressFormat::Ethereum => {
                format!("0x{}", hex::encode(&hash[..20]))
            }
            _ => {
                format!("hash{}", hex::encode(&hash[..20]))
            }
        };

        Ok(AddressInfo {
            address,
            format,
            public_key: None,
            derivation_path: None,
            metadata: HashMap::new(),
            verified: false, // Cannot verify hash-based addresses
        })
    }

    /// Create key fingerprint for caching
    fn create_key_fingerprint(&self, key: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key);
        hex::encode(&hasher.finalize()[..8])
    }

    /// Derive child key using hierarchical deterministic derivation
    fn derive_child_key(&self, master_key: &[u8], path: &DerivationPath) -> BearDogResult<Vec<u8>> {
        let mut current_key = master_key.to_vec();

        for (i, &component) in path.components.iter().enumerate() {
            let hardened = path.hardened.get(i).copied().unwrap_or(false);
            current_key = self.derive_single_child(&current_key, component, hardened)?;
        }

        // Return the public key portion (last 32 bytes for Ed25519)
        if current_key.len() >= 32 {
            Ok(current_key[current_key.len() - 32..].to_vec())
        } else {
            Err(BearDogError::Crypto {
                message: "Invalid derived key length".to_string(),
            })
        }
    }

    /// Derive single child key
    fn derive_single_child(
        &self,
        parent_key: &[u8],
        index: u32,
        hardened: bool,
    ) -> BearDogResult<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(parent_key);
        hasher.update(if hardened { "hardened" } else { "normal" });
        hasher.update(index.to_be_bytes());
        Ok(hasher.finalize().to_vec())
    }

    /// Convert derivation path to string representation
    fn derivation_path_to_string(&self, path: &DerivationPath) -> String {
        let mut result = String::from("m");
        for (i, &component) in path.components.iter().enumerate() {
            let hardened = path.hardened.get(i).copied().unwrap_or(false);
            if hardened {
                result.push_str(&format!("/{component}'"));
            } else {
                result.push_str(&format!("/{component}"));
            }
        }
        result
    }

    // Cryptographic helper functions

    /// Calculate RIPEMD160(SHA256(data))
    fn hash160(&self, data: &[u8]) -> Vec<u8> {
        let sha256_hash = {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.finalize()
        };

        let mut ripemd_hasher = Ripemd160::new();
        ripemd_hasher.update(sha256_hash);
        ripemd_hasher.finalize().to_vec()
    }

    /// Calculate Keccak256 hash (for Ethereum compatibility)
    fn keccak256(&self, data: &[u8]) -> Vec<u8> {
        // Simplified Keccak256 - in production, use proper Keccak implementation
        let mut hasher = Sha256::new();
        hasher.update(b"keccak256:");
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Add checksum to address bytes
    fn add_checksum(&self, data: &[u8], prefix: &[u8]) -> BearDogResult<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(prefix);
        hasher.update(data);
        let hash = hasher.finalize();

        let mut result = data.to_vec();
        result.extend_from_slice(&hash[..4]);
        Ok(result)
    }

    /// Calculate double SHA256 checksum (Bitcoin style)
    fn double_sha256_checksum(&self, data: &[u8]) -> Vec<u8> {
        let first_hash = {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hasher.finalize()
        };

        let mut second_hasher = Sha256::new();
        second_hasher.update(first_hash);
        second_hasher.finalize().to_vec()
    }

    /// Base58 encoding (simplified)
    fn base58_encode(&self, data: &[u8]) -> String {
        // Simplified base58 encoding - in production, use proper base58 library
        const ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

        if data.is_empty() {
            return String::new();
        }

        // Convert to base58 (simplified implementation)
        let mut result = String::new();
        for chunk in data.chunks(8) {
            let mut num = 0u64;
            for &byte in chunk {
                num = num * 256 + byte as u64;
            }

            while num > 0 {
                result.push(ALPHABET[(num % 58) as usize] as char);
                num /= 58;
            }
        }

        result.chars().rev().collect()
    }

    /// Bech32 encoding (simplified)
    fn bech32_encode(&self, data: &[u8]) -> String {
        // Simplified bech32 encoding - in production, use proper bech32 library
        hex::encode(data)
    }
}

impl Default for AddressManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DerivationPath {
    /// Create new derivation path
    pub fn new(components: Vec<u32>, hardened: Vec<bool>) -> Self {
        Self {
            components,
            hardened,
        }
    }

    /// Parse derivation path from string (e.g., "m/44'/0'/0'/0/0")
    pub fn from_string(path: &str) -> BearDogResult<Self> {
        if !path.starts_with("m/") && !path.starts_with("M/") {
            return Err(BearDogError::InvalidInput {
                message: "Derivation path must start with 'm/'".to_string(),
            });
        }

        let parts: Vec<&str> = path[2..].split('/').collect();
        let mut components = Vec::new();
        let mut hardened = Vec::new();

        for part in parts {
            if part.is_empty() {
                continue;
            }

            let (num_str, is_hardened) = if part.ends_with('\'') || part.ends_with('h') {
                (&part[..part.len() - 1], true)
            } else {
                (part, false)
            };

            let component = num_str
                .parse::<u32>()
                .map_err(|_| BearDogError::InvalidInput {
                    message: format!("Invalid derivation path component: {part}"),
                })?;

            components.push(component);
            hardened.push(is_hardened);
        }

        Ok(Self {
            components,
            hardened,
        })
    }

    /// Standard Bitcoin derivation path (m/44'/0'/0'/0/0)
    pub fn bitcoin_standard() -> Self {
        Self {
            components: vec![44, 0, 0, 0, 0],
            hardened: vec![true, true, true, false, false],
        }
    }

    /// Standard Ethereum derivation path (m/44'/60'/0'/0/0)
    pub fn ethereum_standard() -> Self {
        Self {
            components: vec![44, 60, 0, 0, 0],
            hardened: vec![true, true, true, false, false],
        }
    }

    /// BearDog standard derivation path (m/44'/1337'/0'/0/0)
    pub fn beardog_standard() -> Self {
        Self {
            components: vec![44, 1337, 0, 0, 0],
            hardened: vec![true, true, true, false, false],
        }
    }
}
