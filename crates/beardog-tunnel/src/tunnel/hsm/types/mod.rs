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


/// HSM Types Module - Canonical Type System
///
/// This module provides unified HSM types, replacing fragmented definitions
/// and enabling consistent type usage across the tunnel crate.

use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// Canonical types - the source of truth
pub mod canonical;
// Legacy modules - to be gradually migrated
pub mod algorithm;
pub mod capability;
pub mod config;
pub mod key;
pub mod status;
pub mod tier;
// Import and re-export canonical types from beardog-types for consistency
pub use beardog_types::canonical::hsm::{AuthenticationMethod, HsmCapabilities, SecurityLevel};
// Import and re-export crypto types
pub use beardog_types::canonical::crypto::KeyType;
// Import and re-export additional types from discovery module
pub use beardog_types::canonical::hsm::discovery::{HsmConnectionInfo, HsmInterfaceType};
// Import types from tier module (canonical definitions)
pub use tier::{
    AndroidKeyAlgorithm, AttestationLevel, KeyStorageType, MemoryProtectionLevel,
    SecureEnclaveType, SmartphoneType, SoftwareHsmType, StrongBoxImplementation,
};
// Re-export canonical types as primary interface
pub use beardog_types::CapabilityRequirements;
// Key management types
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AndroidKeyPurpose {
    Encrypt,
    Decrypt,
    Sign,
    Verify,
    Wrap,
    Unwrap,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AndroidKeyParams {
    pub algorithm: String,
    pub key_size: u32,
    pub purposes: Vec<AndroidKeyPurpose>,
    pub strongbox_required: bool,
    pub user_authentication_required: bool,
    pub user_authentication_timeout: Option<i32>,
    pub key_validity_start: Option<chrono::DateTime<chrono::Utc>>,
    pub key_validity_end: Option<chrono::DateTime<chrono::Utc>>,
    pub attestation_challenge: Option<Vec<u8>>,}


impl AndroidKeyParams {}


    pub fn new() -> Self {
        Self {
            algorithm: "Ed25519".to_string(),
            key_size: 256,
            purposes: vec![AndroidKeyPurpose::Sign, AndroidKeyPurpose::Verify],
            strongbox_required: false,
            user_authentication_required: false,
            user_authentication_timeout: None,
            key_validity_start: None,
            key_validity_end: None,
            attestation_challenge: None,
        }
    }
    pub fn set_algorithm(mut self, algorithm: &str) -> Self {
        self.algorithm = algorithm.to_string();
        self}


    pub fn set_key_size(&mut self, size: u32) {
        self.key_size = size;
    pub fn set_purposes(&mut self, purposes: Vec<AndroidKeyPurpose>) {
        self.purposes = purposes;}


    pub fn set_strongbox_required(&mut self, required: bool) {
        self.strongbox_required = required;
    pub fn set_user_authentication_required(&mut self, required: bool) {
        self.user_authentication_required = required;}


    pub fn set_key_validity_end(&mut self, end: chrono::DateTime<chrono::Utc>) {
        self.key_validity_end = Some(end);
    pub fn set_attestation_challenge(&mut self, challenge: Vec<u8>) {
        self.attestation_challenge = Some(challenge);
impl Default for AndroidKeyParams {}


    fn default() -> Self {
        Self::new()
// Configuration types
pub struct AndroidHsmConfig {
    pub strongbox_enabled: bool,
    pub key_params: AndroidKeyParams,
    pub attestation_level: AttestationLevel,
    pub security_level: u8,}


impl Default for AndroidHsmConfig {
            strongbox_enabled: true,
            key_params: AndroidKeyParams::new(),
            attestation_level: AttestationLevel::Hardware,
            security_level: 2,}


pub struct IOSHsmConfig {
    pub secure_enclave_enabled: bool,
    pub biometric_authentication: bool,
    pub key_attestation: bool,
// Device and hardware types
pub struct AndroidDeviceCapabilities {
    pub strongbox_available: bool,
    pub key_attestation_available: bool,
    pub hardware_backed_keystore: bool,
    pub verified_boot: bool,
}


pub struct IOSDeviceCapabilities {
    pub secure_enclave_available: bool,
    pub biometric_id_available: bool,
    pub hardware_security_module: bool,
// Provider interfaces
#[derive(Debug)]
pub struct AndroidKeystore {
    pub config: AndroidHsmConfig,
    pub capabilities: AndroidDeviceCapabilities,}


impl AndroidKeystore {}


    pub fn new(config: AndroidHsmConfig) -> BearDogResult<Self> {
        Ok(Self {
            config,
            capabilities: AndroidDeviceCapabilities {
                strongbox_available: true,
                key_attestation_available: true,
                hardware_backed_keystore: true,
                verified_boot: true,
            },
        })
    pub async fn test_keystore_access(&self) -> BearDogResult<()> {
        // Placeholder implementation
        Ok(())}


    pub async fn generate_key(&self, key_id: &str, params: &AndroidKeyParams) -> BearDogResult<()> {
        Err(BearDogError::NotImplemented {
            message: "Android keystore key generation".to_string(),
    pub async fn encrypt(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
            message: "Android keystore encryption".to_string(),}


    pub async fn decrypt(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
            message: "Android keystore decryption".to_string(),
    pub async fn sign(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
            message: "Android keystore signing".to_string(),}


    pub async fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> BearDogResult<bool> {
            message: "Android keystore verification".to_string(),
    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
            message: "Android keystore key deletion".to_string(),}


pub struct AndroidAttestationService {
    pub enabled: bool,
    pub challenge_generator: ChallengeGenerator,
}


pub struct ChallengeGenerator {
    pub entropy_source: String,}


impl ChallengeGenerator {
            entropy_source: "system_random".to_string(),}


    pub fn generate_challenge(&self, size: usize) -> BearDogResult<Vec<u8>> {
        use rand::RngCore;
        let mut challenge = vec![0u8; size];
        rand::thread_rng().fill_bytes(&mut challenge);
        tracing::debug!("Generated attestation challenge of {} bytes", size);
        Ok(challenge)
impl AndroidAttestationService {}


    pub fn new(attestation_level: AttestationLevel) -> Self {
            enabled: true,
            attestation_level,
            challenge_generator: ChallengeGenerator::new(),}


    pub async fn initialize(&self) -> BearDogResult<()> {
        tracing::info!("Initializing Android attestation service with level: {:?}", self.attestation_level);
        
        // Verify attestation capabilities are available
        if !self.enabled {
            return Err(BearDogError::configuration("Attestation service is disabled".to_string()));
        tracing::info!("Android attestation service initialized successfully");
pub struct AndroidHealthMonitor {
    pub check_interval_seconds: u64,}


impl AndroidHealthMonitor {
            check_interval_seconds: 60,
    pub async fn start_monitoring(&self) -> BearDogResult<()> {
            return Err(BearDogError::configuration("Health monitoring is disabled".to_string()));
        tracing::info!("Starting Android health monitoring with {}-second intervals", self.check_interval_seconds);
        // Start background monitoring task
        let check_interval = std::time::Duration::from_secs(self.check_interval_seconds);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(check_interval);
            loop {
                interval.tick().await;
                tracing::debug!("Performing Android HSM health check");
                // Health check logic would go here
            }
        });
    pub async fn get_health_status(&self) -> BearDogResult<beardog_core::HsmHealthStatus> {
        tracing::debug!("Getting Android HSM health status");
        let mut is_healthy = true;
        let mut error_message = None;
        let start_time = std::time::Instant::now();
        // Check if monitoring is enabled
            is_healthy = false;
            error_message = Some("Health monitoring is disabled".to_string());
        // Perform basic system checks
        let (operations_per_second, error_count) = self.get_performance_metrics().await?;
        let latency_ms = start_time.elapsed().as_millis() as f64;
        // Calculate availability based on error rate
        let error_rate = if operations_per_second > 0.0 {
            (error_count as f64) / operations_per_second
        } else {
            0.0
        };
        let availability_percentage = if error_rate < 0.01 {
            100.0
        } else if error_rate < 0.05 {
            95.0
            90.0 - (error_rate * 100.0)
        // Check if health thresholds are met
        if error_rate > 0.1 || availability_percentage < 90.0 {
            error_message = Some(format!("High error rate: {:.2}%, availability: {:.1}%", 
                                       error_rate * 100.0, availability_percentage));
        Ok(beardog_core::HsmHealthStatus {
            is_healthy,
            last_check: chrono::Utc::now(),
            error_message,
            performance_metrics: crate::tunnel::hsm::types::status::PerformanceMetrics {
                operations_per_second,
                latency_ms,
                throughput_mbps: operations_per_second * 0.001, // Rough estimate
                cpu_usage_percent: self.estimate_cpu_usage(),
                memory_usage_mb: self.estimate_memory_usage(),
                error_count: error_count as u64,
                uptime_seconds: self.get_uptime_seconds(),
                availability_percentage,
                error_rate,
                network_throughput_bps: operations_per_second * 1024.0, // Rough estimate
    
    /// Get current performance metrics
    async fn get_performance_metrics(&self) -> BearDogResult<(f64, u32)> {
        // In a real implementation, this would query actual metrics
        // For now, return simulated healthy values
        let operations_per_second = 100.0;
        let error_count = 0;
        Ok((operations_per_second, error_count))
    /// Estimate CPU usage}


    fn estimate_cpu_usage(&self) -> f64 {
        // In a real implementation, this would query system metrics
        // For now, return a low simulated value
        5.0
    /// Estimate memory usage in MB}


    fn estimate_memory_usage(&self) -> f64 {
        // In a real implementation, this would query actual memory usage
        // For now, return a reasonable simulated value
        50.0
    /// Get uptime in seconds}


    fn get_uptime_seconds(&self) -> u64 {
        // In a real implementation, this would track actual uptime
        // For now, return a large value indicating good uptime
        86400 // 24 hours}


impl Default for AndroidHealthMonitor {
// HSM operation types
pub enum HsmOperation {
    KeyGeneration { key_type: String, key_size: u32 },
    Encryption { key_id: String, algorithm: String },
    Decryption { key_id: String, algorithm: String },
    Signing { key_id: String, algorithm: String },
    Verification { key_id: String, algorithm: String },
    KeyDeletion { key_id: String },
// HSM result types
pub struct HsmOperationResult {
    pub operation: HsmOperation,
    pub success: bool,
    pub result_data: Option<Vec<u8>>,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
// Audit and logging types
pub struct HsmAuditEntry {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<String>,
    pub result: HsmOperationResult,
    pub security_context: HashMap<String, String>,
// Cache types for performance optimization
#[derive(Debug, Clone)]
pub struct HsmCache {
    pub key_metadata: Arc<RwLock<HashMap<String, KeyMetadata>>>,
    pub operation_cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,}


impl HsmCache {
            key_metadata: Arc::new(RwLock::new(HashMap::new())),
            operation_cache: Arc::new(RwLock::new(HashMap::new())),}


impl Default for HsmCache {
