// MODERNIZED: Removed async_trait - now uses native async fn in trait

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


/// Android StrongBox HSM Types
///
/// This module provides type definitions for Android StrongBox HSM integration,
/// supporting hardware-backed cryptographic operations on GrapheneOS/Pixel devices.

use crate::tunnel::hsm::types::*;
use beardog_traits::canonical::HsmProvider;
use beardog_core::{HsmHealthStatus, HsmKey};
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
/// StrongBox implementation types for different Android devices
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StrongBoxImplementation {
    /// Google Titan M security chip (Pixel 3+)
    TitanM {
        /// Titan M chip version
        version: String,
        /// Security level achieved
        security_level: String,
    },
    /// Qualcomm Secure Processing Unit
    QualcommSpu {
        /// SPU version identifier
        /// Hardware attestation support
        attestation_support: bool,
    /// Samsung Knox hardware security
    SamsungKnox {
        /// Knox version
        /// Knox security level
    /// MediaTek hardware security module
    MediaTekHsm {
        /// HSM version
        /// Available security features
        features: Vec<String>,
    /// Generic StrongBox implementation
    Generic {
        /// Vendor name
        vendor: String,
        /// Implementation version
        /// Security capabilities
        capabilities: Vec<String>,
}
impl Default for StrongBoxImplementation {}


    fn default() -> Self {
        StrongBoxImplementation::TitanM {
            version: "1.0".to_string(),
            security_level: "EAL4+".to_string(),
        }
    }
/// Main Android StrongBox HSM structure
/// This structure represents the Android StrongBox HSM implementation with
/// hardware-backed key operations on GrapheneOS/Pixel devices.
pub struct AndroidStrongBoxHsm {
    /// Android HSM configuration
    pub config: AndroidHsmConfig,
    /// Android Keystore interface
    pub keystore: Arc<AndroidKeystore>,
    /// Attestation service for key verification
    pub attestation_service: Arc<AndroidAttestationService>,
    /// Device information and capabilities
    pub device_info: Arc<AndroidDeviceInfo>,
    /// Cache for frequently accessed key information
    pub key_cache: Arc<RwLock<HashMap<String, CachedKeyInfo>>>,
    /// Health monitoring service
    pub health_monitor: Arc<AndroidHealthMonitor>,}


impl AndroidStrongBoxHsm {
    /// Create a new AndroidStrongBoxHsm instance
    pub async fn new(config: AndroidHsmConfig) -> BearDogResult<Self> {
        // Initialize keystore
        let keystore = Arc::new(AndroidKeystore::new(config.clone())?);
        // Initialize attestation service
        let attestation_service = Arc::new(AndroidAttestationService::new(
            config.attestation_level.clone(),
        ));
        // Initialize device info
        let device_info = Arc::new(AndroidDeviceInfo::new().await?);
        // Initialize key cache
        let key_cache = Arc::new(RwLock::new(HashMap::new()));
        // Initialize health monitor
        let health_monitor = Arc::new(AndroidHealthMonitor::new());
        Ok(Self {
            config,
            keystore,
            attestation_service,
            device_info,
            key_cache,
            health_monitor,
        })
    /// Initialize the HSM and perform startup checks
    pub async fn initialize(&mut self) -> BearDogResult<()> {
        // Test keystore access
        self.keystore.test_keystore_access().await?;
        self.attestation_service.initialize().await?;
        // Start health monitoring
        self.health_monitor.start_monitoring().await?;
        Ok(())
impl HsmProvider for AndroidStrongBoxHsm {}


    async fn generate_key(&mut self, key_type: &str, key_id: &str) -> BearDogResult<HsmKey> {
        // Use the keystore to generate a key
        let key_params = AndroidKeyParams::new().set_algorithm(match key_type {
            "EC" => "EcP256",
            "RSA" => "Rsa2048",
            _ => {
                return Err(BearDogError::NotImplemented {
                    message: format!("Key type not supported: {}", key_type),
                })
            }
        });
        self.keystore.generate_key(key_id, &key_params).await?;
        // Return a canonical HsmKey
        Ok(HsmKey {
            id: key_id.to_string(),
            key_type: key_type.to_string(),
            created_at: chrono::Utc::now(),
            metadata: HashMap::new(),
    async fn sign(&mut self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        self.keystore.sign(key_id, data).await}


    async fn verify(&mut self, key_id: &str, data: &[u8], signature: &[u8]) -> BearDogResult<bool> {
        self.keystore.verify(key_id, data, signature).await
    async fn encrypt(&mut self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        self.keystore.encrypt(key_id, data).await}


    async fn decrypt(&mut self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        self.keystore.decrypt(key_id, data).await
    async fn delete_key(&mut self, key_id: &str) -> BearDogResult<()> {
        self.keystore.delete_key(key_id).await}


    async fn list_keys(&mut self) -> BearDogResult<Vec<String>> {
        // Return cached key IDs
        let cache = self.key_cache.read().await;
        Ok(cache.keys().cloned().collect())
    async fn get_health_status(&mut self) -> BearDogResult<HsmHealthStatus> {
        self.health_monitor.get_health_status().await
/// Android Keystore integration structure
/// Handles communication with the Android Keystore service and StrongBox.
pub struct AndroidKeystore {
    /// Keystore configuration
    pub config: KeystoreConfig,
    /// Whether StrongBox is available on this device
    pub strongbox_available: bool,
    /// Type of StrongBox implementation
    pub strongbox_implementation: StrongBoxImplementation,
    /// Native Android keystore handle (Android only)
    #[cfg(target_os = "android")]
    pub native_handle: Option<AndroidNativeHandle>,
/// Native Android keystore handle for direct API access
#[cfg(target_os = "android")]
pub struct AndroidNativeHandle {
    /// Device context for Android operations
    pub device_context: String,
/// Android Attestation Service structure
/// Manages key attestation and certificate chain verification.}


pub struct AndroidAttestationService {
    /// Attestation configuration
    pub config: AttestationConfig,
    /// Trusted root certificates for validation
    pub trusted_certificates: Vec<Vec<u8>>,
    /// Challenge generator for attestation
    pub challenge_generator: Arc<ChallengeGenerator>,
/// Android Device Information structure
/// Contains information about the Android device and its security capabilities.
pub struct AndroidDeviceInfo {
    /// Device manufacturer (e.g., "Google", "Samsung")
    pub manufacturer: String,
    /// Device model (e.g., "Pixel 8a", "Galaxy S23")
    pub model: String,
    /// Android version (e.g., "13", "14")
    pub android_version: String,
    /// StrongBox implementation version
    pub strongbox_version: Option<String>,
    /// Titan M chip version (for Pixel devices)
    pub titan_m_version: Option<String>,
    /// Security patch level date
    pub security_patch_level: String,
    /// Verified boot state of the device
    pub verified_boot_state: VerifiedBootState,}


impl AndroidDeviceInfo {
    /// Create a new AndroidDeviceInfo instance
    pub async fn new() -> BearDogResult<Self> {
            manufacturer: "Google".to_string(),}


            model: "Pixel 8".to_string(),
            android_version: "14".to_string(),
            strongbox_version: Some("1.0".to_string()),
            titan_m_version: Some("1.0".to_string()),
            security_patch_level: "2024-01-01".to_string(),
            verified_boot_state: VerifiedBootState::Green,
    /// Detect device information from the Android system
    pub async fn detect() -> BearDogResult<Self> {
        // For now, return mock data. In a real implementation, this would
        // query the Android system properties and device capabilities
        Self::new().await
/// Cached key information for Android StrongBox
#[derive(Debug, Clone)]
pub struct CachedKeyInfo {
    /// Unique key identifier
    pub key_id: String,
    /// Type of cryptographic key
    pub key_type: KeyType,
    /// HSM tier information
    pub hsm_type: HsmTier,
    /// Whether key is backed by StrongBox
    pub strongbox_backed: bool,
    /// Whether attestation has been verified
    pub attestation_verified: bool,
    /// Timestamp of last key usage
    pub last_used: chrono::DateTime<Utc>,
    /// Number of times key has been used
    pub usage_count: u64,
    /// Timestamp when key was created
    pub created_at: chrono::DateTime<Utc>,
    /// Key usage policy
    pub usage_policy: KeyUsagePolicy,
    /// Current health status of the key
    pub health_status: KeyHealthStatus,
/// Android Health Monitor structure
/// Monitors the health of various Android StrongBox components.}


pub struct AndroidHealthMonitor {
    /// Health status of Android Keystore
    pub keystore_health: Arc<RwLock<HsmHealthStatus>>,
    /// Health status of StrongBox hardware
    pub strongbox_health: Arc<RwLock<HsmHealthStatus>>,
    /// Health status of attestation service
    pub attestation_health: Arc<RwLock<HsmHealthStatus>>,
/// Challenge Generator structure
/// Generates cryptographic challenges for attestation and authentication.
pub struct ChallengeGenerator {
    /// Source of cryptographic entropy
    pub entropy_source: Arc<dyn EntropySource>,
/// Trait for providing cryptographic entropy.
pub trait EntropySource: Send + Sync {
    /// Generate cryptographic entropy of the specified length
    fn generate_entropy(&self, length: usize) -> BearDogResult<Vec<u8>>;
/// Android Entropy Source implementation
/// Uses Android's hardware random number generator.
pub struct AndroidEntropySource;
/// Verified Boot State enumeration
/// Represents the verified boot state of the Android device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifiedBootState {
    /// Verified boot with locked bootloader
    Green, // Verified boot with locked bootloader
    /// Verified boot with unlocked bootloader
    Yellow, // Verified boot with unlocked bootloader
    /// Custom OS
    Orange, // Custom OS
    /// Boot failure
    Red, // Boot failure
    /// Unknown boot state
    Unknown,
/// Android Key Parameters structure
/// Configuration parameters for Android Keystore key generation.}


pub struct AndroidKeyParams {
    /// Cryptographic algorithm to use
    pub algorithm: AndroidKeyAlgorithm,
    /// Key size in bits
    pub key_size: u32,
    /// Allowed purposes for the key
    pub purposes: Vec<AndroidKeyPurpose>,
    /// Whether StrongBox backing is required
    pub strongbox_required: bool,
    /// Whether user authentication is required
    pub user_authentication_required: bool,
    /// User authentication validity duration in seconds
    pub user_authentication_validity_duration: Option<u32>,
    /// Challenge for key attestation
    pub attestation_challenge: Option<Vec<u8>>,
    /// Key validity end time
    pub key_validity_end: Option<chrono::DateTime<Utc>>,
    /// Elliptic curve for EC keys
    pub curve: Option<AndroidEcCurve>,
/// Android Key Algorithm enumeration
/// Supported cryptographic algorithms in Android StrongBox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidKeyAlgorithm {
    /// Elliptic Curve cryptography
    Ec,
    /// RSA cryptography
    Rsa,
    /// AES symmetric encryption
    Aes,
/// Android Key Purpose enumeration
/// Specifies the allowed purposes for an Android key.}


pub enum AndroidKeyPurpose {
    /// Key can be used for encryption operations
    Encrypt,
    /// Key can be used for decryption operations
    Decrypt,
    /// Key can be used for signing operations
    Sign,
    /// Key can be used for verification operations
    Verify,
    /// Key can be used for key wrapping operations
    Wrap,
    /// Key can be used for key unwrapping operations
    Unwrap,
/// Supported elliptic curves in Android StrongBox.
pub enum AndroidEcCurve {
    /// NIST P-256 elliptic curve
    P256,
    /// NIST P-384 elliptic curve
    P384,
    /// NIST P-521 elliptic curve
    P521,}


impl AndroidKeyParams {
    /// Create new Android key parameters with default values}


    pub fn new() -> Self {
        Self {
            algorithm: AndroidKeyAlgorithm::Ec,
            key_size: 256,
            purposes: vec![AndroidKeyPurpose::Sign, AndroidKeyPurpose::Verify],
            strongbox_required: true,
            user_authentication_required: false,
            user_authentication_validity_duration: None,
            attestation_challenge: None,
            key_validity_end: None,
            curve: Some(AndroidEcCurve::P256),
    /// Set the key algorithm}


    pub fn set_algorithm(&mut self, algorithm: AndroidKeyAlgorithm) {
        self.algorithm = algorithm;
    /// Set the key size
    pub fn set_key_size(&mut self, size: u32) {
        self.key_size = size;
    /// Set the key purposes}


    pub fn set_purposes(&mut self, purposes: Vec<AndroidKeyPurpose>) {
        self.purposes = purposes;
    /// Set whether StrongBox is required
    pub fn set_strongbox_required(&mut self, required: bool) {
        self.strongbox_required = required;
    /// Set whether user authentication is required}


    pub fn set_user_authentication_required(&mut self, required: bool) {
        self.user_authentication_required = required;
    /// Set user authentication validity duration
    pub fn set_user_authentication_validity_duration(&mut self, duration: u32) {
        self.user_authentication_validity_duration = Some(duration);
    /// Set attestation challenge}


    pub fn set_attestation_challenge(&mut self, challenge: Vec<u8>) {
        self.attestation_challenge = Some(challenge);
    /// Set key validity end time
    pub fn set_key_validity_end(&mut self, end: chrono::DateTime<Utc>) {
        self.key_validity_end = Some(end);
    /// Set elliptic curve}


    pub fn set_curve(&mut self, curve: AndroidEcCurve) {
        self.curve = Some(curve);
impl Default for AndroidKeyParams {
        Self::new()}


impl From<StrongBoxError> for BearDogError {
    fn from(err: StrongBoxError) -> Self {
        BearDogError::Hsm {
            message: format!("StrongBox error: {err:?}"),}


impl StrongBoxError {
    /// Create a BearDogError from a StrongBox error code and message}


    pub fn from_strongbox_error(code: i32, message: &str) -> BearDogError {
            message: format!("StrongBox error (code {code}): {message}"),
