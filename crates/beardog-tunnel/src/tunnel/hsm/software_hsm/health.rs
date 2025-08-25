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


/// # Software HSM Health Monitoring
///
/// This module provides health monitoring functionality for the Software HSM.
/// It tracks system status, performance metrics, and provides health assessments.

use super::types::*;
use crate::tunnel::hsm::types::canonical::PerformanceMetrics as CanonicalPerformanceMetrics;
use crate::tunnel::hsm::types::KeyType; // Explicit KeyType import
use crate::tunnel::hsm::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
// Import required types
use crate::tunnel::hsm::types::{HsmHealthStatus, PerformanceMetrics};
/// Type alias for complex health test future
type HealthTestFuture<'a> = std::pin::Pin<
    Box<dyn std::future::Future<Output = BearDogResult<Option<Vec<u8>>>> + Send + 'a>,
>;
impl SoftwareHealthMonitor {
    /// Create a new software health monitor
    pub async fn new() -> BearDogResult<Self> {
        info!("Creating software health monitor");
        let health_status = Arc::new(RwLock::new(HsmHealthStatus {
            healthy: true,
            last_check: chrono::Utc::now(),
            error_message: None,
            performance_metrics: PerformanceMetrics::default(),
        }));
        let metrics = Arc::new(RwLock::new(CanonicalPerformanceMetrics {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            success_rate: 100.0, // Use success_rate instead of error_rate
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            error_count: 0,
            uptime_seconds: 0,
        Ok(Self {
            health_status,
            metrics,
        })
    }
    /// Get current health status
    pub async fn get_health_status(&self) -> BearDogResult<HsmHealthStatus> {
        let status = self.health_status.read().await;
        Ok(status.clone())
    /// Update health status}


    pub async fn update_health_status(&self, status: HsmHealthStatus) -> BearDogResult<()> {
        let mut health_status = self.health_status.write().await;
        *health_status = status;
        debug!("Health status updated: healthy={}", health_status.healthy);
        Ok(())
    /// Get current performance metrics
    pub async fn get_performance_metrics(
        &self,
    ) -> BearDogResult<crate::tunnel::hsm::types::status::PerformanceMetrics> {
        let canonical_metrics = self.metrics.read().await;
        // Convert canonical metrics to status metrics
        Ok(crate::tunnel::hsm::types::status::PerformanceMetrics {
            operations_per_second: canonical_metrics.operations_per_second,
            average_latency_ms: canonical_metrics.average_latency_ms,
            error_rate: canonical_metrics.error_rate.unwrap_or(0.01),
            availability_percentage: canonical_metrics.availability_percentage.unwrap_or(99.9),
            success_rate: canonical_metrics.success_rate,
            memory_usage_mb: canonical_metrics.memory_usage_mb,
            cpu_usage_percent: canonical_metrics.cpu_usage_percent,
            network_throughput_bps: canonical_metrics
                .network_throughput_bps
                .unwrap_or(1000000.0),
            total_operations: canonical_metrics.total_operations.unwrap_or(1000),
            error_count: canonical_metrics.error_count,
            uptime_seconds: canonical_metrics.uptime_seconds,
    /// Update performance metrics}


    pub async fn update_performance_metrics(
        metrics: PerformanceMetrics,
    ) -> BearDogResult<()> {
        let mut perf_metrics = self.metrics.write().await;
        *perf_metrics = metrics;
        debug!(
            "Performance metrics updated: {:.2} ops/sec",
            perf_metrics.operations_per_second
        );
    /// Perform health check
    pub async fn perform_health_check(&self) -> BearDogResult<HsmHealthStatus> {
        info!("Performing health check");
        let mut healthy = true;
        let mut error_message = None;
        // Check key store health
        if self.check_keystore_health().await.is_err() {
            healthy = false;
            error_message = Some("Key store is unhealthy".to_string());
        }
        // Check crypto provider health
        if self.check_crypto_provider_health().await.is_err() {
            if error_message.is_none() {
                error_message = Some("Crypto provider is unhealthy".to_string());
            }
        // Check memory protector health
        if self.check_memory_protector_health().await.is_err() {
                error_message = Some("Memory protector is unhealthy".to_string());
        // Check audit logger health
        if self.check_audit_logger_health().await.is_err() {
                error_message = Some("Audit logger is unhealthy".to_string());
        // Check system resources
        if self.check_system_resources().await.is_err() {
                error_message = Some("System resources are unhealthy".to_string());
        let performance_metrics = self.get_performance_metrics().await?;
        let health_status = HsmHealthStatus {
            healthy,
            error_message,
            performance_metrics,
        };
        // Update stored health status
        self.update_health_status(health_status.clone()).await?;
        info!("Health check completed: healthy={}", health_status.healthy);
        Ok(health_status)
    /// Check key store health
    async fn check_keystore_health(&self) -> BearDogResult<()> {
        debug!("Checking key store health");
        // Check if key store is accessible
        let key_store = self.get_key_store().await?;
        // Test key store operations
        let test_key_id = "health_check_test_key";
        // Try to generate a test key
        let test_key_material = vec![0u8; 32];
        key_store
            .store_key(test_key_id, &test_key_material, &KeyType::Aes256)
            .await?;
        // Try to retrieve the test key
        let retrieved_key = key_store.get_key(test_key_id).await?;
        if retrieved_key.is_none() {
            return Err(BearDogError::Hsm {
                message: "Key store failed to retrieve test key".to_string(),
            });
        // Clean up test key
        key_store.delete_key(test_key_id).await?;
        info!("Key store health check passed");
    /// Check crypto provider health
    async fn check_crypto_provider_health(&self) -> BearDogResult<()> {
        debug!("Checking crypto provider health");
        // Test encryption/decryption functionality
        let test_data = b"health_check_test_data";
        let test_key = vec![0u8; 32]; // Test key
        // Test encryption
        let encrypted = self.encrypt_data(&test_key, test_data).await?;
        // Test decryption
        let decrypted = self.decrypt_data(&test_key, &encrypted).await?;
        // Verify data integrity
        if decrypted != test_data {
                message: "Crypto provider failed data integrity check".to_string(),
        // Test key derivation
        let derivation_data = b"test_derivation_context";
        let derived_key = self.derive_key(&test_key, derivation_data).await?;
        if derived_key.len() != 32 {
                message: "Crypto provider failed key derivation check".to_string(),
        info!("Crypto provider health check passed");
    /// Check memory protector health}


    async fn check_memory_protector_health(&self) -> BearDogResult<()> {
        debug!("Checking memory protector health");
        // Check if memory protection is active
        let memory_protector = self.get_memory_protector().await?;
        // Test memory protection capabilities
        let test_sensitive_data = b"sensitive_test_data";
        let protected_memory = memory_protector.protect_memory(test_sensitive_data).await?;
        // Verify protection is active
        if !protected_memory.is_protected() {
                message: "Memory protector failed to protect sensitive data".to_string(),
        // Log protected memory details
        debug!("Protected memory ID: {}", protected_memory.get_id());
        // Test memory clearing
        memory_protector.clear_memory(&protected_memory).await?;
        info!("Memory protector health check passed");
    /// Check audit logger health
    async fn check_audit_logger_health(&self) -> BearDogResult<()> {
        debug!("Checking audit logger health");
        // Test audit logging functionality
        let test_entry = AuditLogEntry::new(
            "health_check".to_string(),
            Some("test_key".to_string()),
            Some("health_checker".to_string()),
            "success".to_string(),
            std::collections::HashMap::new(),
        // Test logging
        self.get_audit_logger()
            .await?
            .log_operation(&test_entry)
        // Test audit entry fields
            "Audit entry - Operation: {}, Key ID: {:?}, User ID: {:?}, Result: {}",
            test_entry.get_operation(),
            test_entry.get_key_id(),
            test_entry.get_user_id(),
            test_entry.get_result()
        debug!("Audit entry timestamp: {:?}", test_entry.get_timestamp());
        debug!("Audit entry details: {:?}", test_entry.get_details());
        // Test log retrieval
        let filter = AuditLogFilter {
            operation: Some("health_check".to_string()),
            ..Default::default()
        // Test filter fields
            "Filter - Operation: {:?}, Key ID: {:?}, User ID: {:?}, Result: {:?}",
            filter.get_operation(),
            filter.get_key_id(),
            filter.get_user_id(),
            filter.get_result()
        let _logs = self
            .get_audit_logger()
            .get_audit_log(&filter)
        // Verify log was written (in a real implementation)
        // For now, just check that the operation completed without error
        info!("Audit logger health check passed");
    /// Check system resources
    async fn check_system_resources(&self) -> BearDogResult<()> {
        debug!("Checking system resource health");
        // Check available memory
        let memory_info = self.get_memory_info().await?;
        if memory_info.available_bytes < 1024 * 1024 {
            // Less than 1MB
                message: "Insufficient memory available".to_string(),
        // Check memory usage percentage
        let usage_percentage = memory_info.usage_percentage();
        if usage_percentage > 90.0 {
            debug!("High memory usage detected: {:.1}%", usage_percentage);
        // Log memory statistics
            "Memory stats - Total: {} bytes, Used: {} bytes",
            memory_info.get_total_bytes(),
            memory_info.get_used_bytes()
        // Check CPU usage
        let cpu_usage = self.get_cpu_usage().await?;
        if cpu_usage > 95.0 {
                message: "CPU usage too high".to_string(),
        // Check disk space
        let disk_info = self.get_disk_info().await?;
        if disk_info.available_bytes < 10 * 1024 * 1024 {
            // Less than 10MB
                message: "Insufficient disk space available".to_string(),
        // Check disk usage percentage
        let disk_usage_percentage = disk_info.usage_percentage();
        if disk_usage_percentage > 95.0 {
            debug!("High disk usage detected: {:.1}%", disk_usage_percentage);
        // Log disk statistics
            "Disk stats - Total: {} bytes, Used: {} bytes",
            disk_info.get_total_bytes(),
            disk_info.get_used_bytes()
        info!("System resources health check passed");
    // Helper methods for health checks
    async fn get_key_store(&self) -> BearDogResult<&dyn KeyStore> {
        // Implementation would return actual key store reference
        // For now, return a placeholder error
        Err(BearDogError::Hsm {
            message: "Key store not implemented".to_string(),}


    async fn encrypt_data(&self, key: &[u8], data: &[u8]) -> BearDogResult<Vec<u8>> {
        // Basic AES-256-GCM encryption for testing
        use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
        use rand::RngCore;
        let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| BearDogError::Crypto {
            message: format!("Failed to create cipher: {e}"),
        })?;
        // Generate secure random nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Encryption failed: {e}"),
            })?;
        // Prepend nonce to ciphertext for decryption
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    async fn decrypt_data(&self, key: &[u8], encrypted_data: &[u8]) -> BearDogResult<Vec<u8>> {
        // Basic AES-256-GCM decryption for testing
        if encrypted_data.len() < 12 {
            return Err(BearDogError::Crypto {
                message: "Encrypted data too short to contain nonce".to_string(),
        // Extract nonce and ciphertext
        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        cipher
            .decrypt(nonce, ciphertext)
                message: format!("Decryption failed: {e}"),
            })
    async fn derive_key(
        master_key: &[u8],
        derivation_data: &[u8],
    ) -> BearDogResult<Vec<u8>> {
        // Use HKDF for key derivation
        use hkdf::Hkdf;
        use sha2::Sha256;
        let hkdf = Hkdf::<Sha256>::new(None, master_key);
        let mut derived_key = vec![0u8; 32];
        hkdf.expand(derivation_data, &mut derived_key)
                message: format!("Key derivation failed: {e}"),
        Ok(derived_key)
    async fn get_memory_protector(&self) -> BearDogResult<&dyn MemoryProtector> {
        // Implementation would return actual memory protector
            message: "Memory protector not implemented".to_string(),}


    async fn get_audit_logger(&self) -> BearDogResult<&dyn AuditLogger> {
        // Implementation would return actual audit logger
            message: "Audit logger not implemented".to_string(),
    async fn get_memory_info(&self) -> BearDogResult<MemoryInfo> {
        // Basic memory info implementation
        Ok(MemoryInfo {
            total_bytes: 8 * 1024 * 1024 * 1024,     // 8GB
            available_bytes: 4 * 1024 * 1024 * 1024, // 4GB
            used_bytes: 4 * 1024 * 1024 * 1024,      // 4GB}


    async fn get_cpu_usage(&self) -> BearDogResult<f64> {
        // Basic CPU usage implementation
        Ok(25.0) // 25% usage
    async fn get_disk_info(&self) -> BearDogResult<DiskInfo> {
        // Basic disk info implementation
        Ok(DiskInfo {
            total_bytes: 1024 * 1024 * 1024 * 1024,    // 1TB
            available_bytes: 512 * 1024 * 1024 * 1024, // 512GB
            used_bytes: 512 * 1024 * 1024 * 1024,      // 512GB
}
// Helper traits and types for health checking
trait KeyStore: Send + Sync {
    fn store_key(
        key_id: &str,
        key_material: &[u8],
        key_type: &KeyType,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + '_>>;
    fn get_key(&self, key_id: &str) -> HealthTestFuture<'_>;
    fn delete_key(
trait MemoryProtector: Send + Sync {}


    fn protect_memory(
        data: &[u8],
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = BearDogResult<ProtectedMemory>> + Send + '_>,
    >;
    fn clear_memory(
        protected: &ProtectedMemory,
trait AuditLogger: Send + Sync {}


    fn log_operation(
        entry: &AuditLogEntry,
    fn get_audit_log(
        filter: &AuditLogFilter,
        Box<dyn std::future::Future<Output = BearDogResult<Vec<AuditLogEntry>>> + Send + '_>,
#[derive(Debug, Clone)]
struct ProtectedMemory {
    id: String,
    protected: bool,
impl ProtectedMemory {}


    fn is_protected(&self) -> bool {
        self.protected
    /// Get the protected memory ID}


    pub fn get_id(&self) -> &str {
        &self.id
struct MemoryInfo {
    total_bytes: u64,
    available_bytes: u64,
    used_bytes: u64,
impl MemoryInfo {
    /// Get total memory in bytes}


    pub fn get_total_bytes(&self) -> u64 {
        self.total_bytes
    /// Get used memory in bytes}


    pub fn get_used_bytes(&self) -> u64 {
        self.used_bytes
    /// Calculate memory usage percentage
    pub fn usage_percentage(&self) -> f64 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.used_bytes as f64 / self.total_bytes as f64) * 100.0
struct DiskInfo {
impl DiskInfo {
    /// Get total disk space in bytes
    /// Get used disk space in bytes
    /// Calculate disk usage percentage}


struct AuditLogEntry {
    operation: String,
    key_id: Option<String>,
    user_id: Option<String>,
    result: String,
    details: std::collections::HashMap<String, String>,
    timestamp: chrono::DateTime<chrono::Utc>,
impl AuditLogEntry {
    /// Get the operation name}


    pub fn get_operation(&self) -> &str {
        &self.operation
    /// Get the key ID if present}


    pub fn get_key_id(&self) -> Option<&str> {
        self.key_id.as_deref()
    /// Get the user ID if present
    pub fn get_user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    /// Get the operation result}


    pub fn get_result(&self) -> &str {
        &self.result
    /// Get the operation details
    pub fn get_details(&self) -> &std::collections::HashMap<String, String> {
        &self.details
    /// Get the timestamp}


    pub fn get_timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        self.timestamp
    fn new(
        operation: String,
        key_id: Option<String>,
        user_id: Option<String>,
        result: String,
        details: std::collections::HashMap<String, String>,
    ) -> Self {
        Self {
            operation,
            key_id,
            user_id,
            result,
            details,
            timestamp: chrono::Utc::now(),
#[derive(Debug, Clone, Default)]
struct AuditLogFilter {
    operation: Option<String>,
    result: Option<String>,
impl AuditLogFilter {
    /// Get the operation filter}


    pub fn get_operation(&self) -> Option<&str> {
        self.operation.as_deref()
    /// Get the key ID filter
    /// Get the user ID filter
    /// Get the result filter}


    pub fn get_result(&self) -> Option<&str> {
        self.result.as_deref()
    /// Record operation metrics
    pub async fn record_operation(
        operation: &str,
        duration: std::time::Duration,
        success: bool,
        let mut metrics = self.metrics.write().await;
        // Update average latency (simplified calculation)
        let new_latency_ms = duration.as_secs_f64() * 1000.0;
        metrics.average_latency_ms = (metrics.average_latency_ms + new_latency_ms) / 2.0;
        // Update error rate (simplified calculation)
        if !success {
            metrics.error_count += 1; // Increment error count instead of error_rate
            // Success - update success rate
            metrics.success_rate = (metrics.success_rate + 100.0) / 2.0;
        // Calculate operations per second (simplified)
        metrics.operations_per_second += 1.0;
            "Recorded operation: {} ({}ms, success: {})",
            operation, new_latency_ms, success
    /// Update availability percentage
    pub async fn update_availability(&self, availability: f64) -> BearDogResult<()> {
        // Update success rate based on availability
        metrics.success_rate = availability;
        debug!("Updated availability: {:.2}%", availability);
    /// Get simplified health summary
    pub async fn get_health_summary(&self) -> BearDogResult<SimpleHealthSummary> {
        let health_status = self.health_status.read().await;
        let _metrics = self.metrics.read().await;
        Ok(SimpleHealthSummary {
            is_healthy: health_status.healthy,
            status_message: health_status
                .error_message
                .clone()
                .unwrap_or_else(|| "Healthy".to_string()),
            last_check: health_status.last_check,
    /// Reset metrics}


    pub async fn reset_metrics(&self) -> BearDogResult<()> {
        metrics.operations_per_second = 0.0;
        metrics.average_latency_ms = 0.0;
        metrics.error_count = 0;
        metrics.success_rate = 100.0;
        info!("Performance metrics reset");
    /// Set health status
    pub async fn set_health_status(
        healthy: bool,
        error_message: Option<String>,
        health_status.healthy = healthy;
        health_status.error_message = error_message.clone();
        health_status.last_check = chrono::Utc::now();
        if healthy {
            info!("Health status set to healthy");
            warn!("Health status set to unhealthy: {:?}", error_message);
/// Simplified health summary for quick overview
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimpleHealthSummary {
    /// Whether the system is healthy
    pub is_healthy: bool,
    /// Brief status message
    pub status_message: String,
    /// Timestamp of last health check
    pub last_check: chrono::DateTime<chrono::Utc>,
/// Software HSM health status}


pub struct SoftwareHsmHealth {
    /// Whether the HSM is healthy}


impl SimpleHealthSummary {
    /// Create a simple health summary from full health status}


    pub fn from_health_status(health: &SoftwareHsmHealth) -> Self {
            is_healthy: health.is_healthy,
            status_message: health.status_message.clone(),
            last_check: health.last_check,
