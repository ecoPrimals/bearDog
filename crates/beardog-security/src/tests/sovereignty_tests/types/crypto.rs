// SPDX-License-Identifier: AGPL-3.0-only

//! Cryptographic Sovereignty Types
//!
//! Test helper types for crypto sovereignty policies.

#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]

use beardog_errors::BearDogError;
use std::collections::HashSet;

/// Key storage types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum KeyStorage {
    LocalOnly,
    CloudManaged,
    HybridEscrow,
}

/// Cryptographic algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Algorithm {
    AES256,
    AES128,
    Ed25519,
    RSA2048,
    RSA4096,
    DES, // Weak - for testing rejection
    MD5, // Broken - for testing rejection
}

/// Export control levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportControl {
    Unrestricted,
    Restricted,
    Prohibited,
}

/// Crypto sovereignty policy
#[derive(Debug, Clone)]
pub struct CryptoSovereigntyPolicy {
    key_storage: KeyStorage,
    allowed_algorithms: HashSet<Algorithm>,
    export_control: ExportControl,
}

impl CryptoSovereigntyPolicy {
    pub fn new() -> Self {
        Self {
            key_storage: KeyStorage::LocalOnly,
            allowed_algorithms: HashSet::new(),
            export_control: ExportControl::Unrestricted,
        }
    }

    pub fn with_key_storage(mut self, storage: KeyStorage) -> Self {
        self.key_storage = storage;
        self
    }

    pub fn with_algorithm(mut self, algorithm: Algorithm) -> Self {
        self.allowed_algorithms.insert(algorithm);
        self
    }

    pub fn with_export_control(mut self, control: ExportControl) -> Self {
        self.export_control = control;
        self
    }

    pub fn key_storage(&self) -> KeyStorage {
        self.key_storage
    }

    pub fn export_control(&self) -> ExportControl {
        self.export_control
    }

    pub fn allows_algorithm(&self, algorithm: Algorithm) -> bool {
        self.allowed_algorithms.contains(&algorithm)
    }

    pub fn validate_key_location(&self, location: &KeyLocation) -> Result<(), BearDogError> {
        if location.storage() != self.key_storage {
            return Err(BearDogError::security(format!(
                "Key storage {:?} not allowed",
                location.storage()
            )));
        }
        Ok(())
    }

    pub fn validate_export(&self, request: &ExportRequest) -> Result<(), BearDogError> {
        if self.export_control == ExportControl::Restricted
            && request.source_region() != request.dest_region()
        {
            return Err(BearDogError::security(
                "International export restricted".to_string(),
            ));
        }
        Ok(())
    }

    pub fn derive_key(
        &self,
        root: &KeyLocation,
        context: &[u8],
    ) -> Result<KeyLocation, BearDogError> {
        // Simple key derivation for testing
        let derived_id = format!("{}_{}", root.id(), context.len());
        Ok(KeyLocation::new(&derived_id, root.storage()))
    }

    pub fn validate_encryption_method(
        &self,
        method: &EncryptionMethod,
    ) -> Result<(), BearDogError> {
        if !self.allows_algorithm(method.algorithm()) {
            return Err(BearDogError::security("Algorithm not allowed".to_string()));
        }
        if method.key_size() < 128 {
            return Err(BearDogError::security("Key size too small".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct KeyLocation {
    id: String,
    storage: KeyStorage,
}

impl KeyLocation {
    pub fn new(id: &str, storage: KeyStorage) -> Self {
        Self {
            id: id.to_string(),
            storage,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn storage(&self) -> KeyStorage {
        self.storage
    }
}

#[derive(Debug, Clone)]
pub struct ExportRequest {
    source_region: String,
    dest_region: String,
}

impl ExportRequest {
    pub fn new(source: &str, dest: &str) -> Self {
        Self {
            source_region: source.to_string(),
            dest_region: dest.to_string(),
        }
    }

    pub fn source_region(&self) -> &str {
        &self.source_region
    }

    pub fn dest_region(&self) -> &str {
        &self.dest_region
    }
}

#[derive(Debug, Clone)]
pub struct EncryptionMethod {
    algorithm: Algorithm,
    key_size: u32,
}

impl EncryptionMethod {
    pub fn new(algorithm: Algorithm, key_size: u32) -> Self {
        Self {
            algorithm,
            key_size,
        }
    }

    pub fn algorithm(&self) -> Algorithm {
        self.algorithm
    }

    pub fn key_size(&self) -> u32 {
        self.key_size
    }
}
