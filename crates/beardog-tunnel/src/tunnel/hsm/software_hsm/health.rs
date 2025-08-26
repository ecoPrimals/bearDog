

use super::types::*;
use crate::tunnel::hsm::types::canonical::PerformanceMetrics as CanonicalPerformanceMetrics;
use crate::tunnel::hsm::types::KeyType; // Explicit KeyType import
use crate::tunnel::hsm::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::tunnel::hsm::types::{HsmHealthStatus, PerformanceMetrics};

type HealthTestFuture<'a> = std::pin::Pin<
    Box<dyn std::future::Future<Output = BearDogResult<Option<Vec<u8>>>> + Send + 'a>,
>;
impl SoftwareHealthMonitor {

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

    pub async fn get_health_status(&self) -> BearDogResult<HsmHealthStatus> {
        let status = self.health_status.read().await;
        Ok(status.clone())

    pub async fn update_health_status(&self, status: HsmHealthStatus) -> BearDogResult<()> {
        let mut health_status = self.health_status.write().await;
        *health_status = status;
        debug!("Health status updated: healthy={}", health_status.healthy);
        Ok(())

    pub async fn get_performance_metrics(
        &self,
    ) -> BearDogResult<crate::tunnel::hsm::types::status::PerformanceMetrics> {
        let canonical_metrics = self.metrics.read().await;

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

    pub async fn update_performance_metrics(
        metrics: PerformanceMetrics,
    ) -> BearDogResult<()> {
        let mut perf_metrics = self.metrics.write().await;
        *perf_metrics = metrics;
        debug!(
            "Performance metrics updated: {:.2} ops/sec",
            perf_metrics.operations_per_second
        );

    pub async fn perform_health_check(&self) -> BearDogResult<HsmHealthStatus> {
        info!("Performing health check");
        let mut healthy = true;
        let mut error_message = None;

        if self.check_keystore_health().await.is_err() {
            healthy = false;
            error_message = Some("Key store is unhealthy".to_string());
        }

        if self.check_crypto_provider_health().await.is_err() {
            if error_message.is_none() {
                error_message = Some("Crypto provider is unhealthy".to_string());
            }

        if self.check_memory_protector_health().await.is_err() {
                error_message = Some("Memory protector is unhealthy".to_string());

        if self.check_audit_logger_health().await.is_err() {
                error_message = Some("Audit logger is unhealthy".to_string());

        if self.check_system_resources().await.is_err() {
                error_message = Some("System resources are unhealthy".to_string());
        let performance_metrics = self.get_performance_metrics().await?;
        let health_status = HsmHealthStatus {
            healthy,
            error_message,
            performance_metrics,
        };

        self.update_health_status(health_status.clone()).await?;
        info!("Health check completed: healthy={}", health_status.healthy);
        Ok(health_status)

    async fn check_keystore_health(&self) -> BearDogResult<()> {
        debug!("Checking key store health");

        let key_store = self.get_key_store().await?;

        let test_key_id = "health_check_test_key";

        let test_key_material = vec![0u8; 32];
        key_store
            .store_key(test_key_id, &test_key_material, &KeyType::Aes256)
            .await?;

        let retrieved_key = key_store.get_key(test_key_id).await?;
        if retrieved_key.is_none() {
            return Err(BearDogError::Hsm {
                message: "Key store failed to retrieve test key".to_string(),
            });

        key_store.delete_key(test_key_id).await?;
        info!("Key store health check passed");

    async fn check_crypto_provider_health(&self) -> BearDogResult<()> {
        debug!("Checking crypto provider health");

        let test_data = b"health_check_test_data";
        let test_key = vec![0u8; 32]; // Test key

        let encrypted = self.encrypt_data(&test_key, test_data).await?;

        let decrypted = self.decrypt_data(&test_key, &encrypted).await?;

        if decrypted != test_data {
                message: "Crypto provider failed data integrity check".to_string(),

        let derivation_data = b"test_derivation_context";
        let derived_key = self.derive_key(&test_key, derivation_data).await?;
        if derived_key.len() != 32 {
                message: "Crypto provider failed key derivation check".to_string(),
        info!("Crypto provider health check passed");

    async fn check_memory_protector_health(&self) -> BearDogResult<()> {
        debug!("Checking memory protector health");

        let memory_protector = self.get_memory_protector().await?;

        let test_sensitive_data = b"sensitive_test_data";
        let protected_memory = memory_protector.protect_memory(test_sensitive_data).await?;

        if !protected_memory.is_protected() {
                message: "Memory protector failed to protect sensitive data".to_string(),

        debug!("Protected memory ID: {}", protected_memory.get_id());

        memory_protector.clear_memory(&protected_memory).await?;
        info!("Memory protector health check passed");

    async fn check_audit_logger_health(&self) -> BearDogResult<()> {
        debug!("Checking audit logger health");

        let test_entry = AuditLogEntry::new(
            "health_check".to_string(),
            Some("test_key".to_string()),
            Some("health_checker".to_string()),
            "success".to_string(),
            std::collections::HashMap::with_capacity(16),

        self.get_audit_logger()
            .await?
            .log_operation(&test_entry)

            "Audit entry - Operation: {}, Key ID: {:?}, User ID: {:?}, Result: {}",
            test_entry.get_operation(),
            test_entry.get_key_id(),
            test_entry.get_user_id(),
            test_entry.get_result()
        debug!("Audit entry timestamp: {:?}", test_entry.get_timestamp());
        debug!("Audit entry details: {:?}", test_entry.get_details());

        let filter = AuditLogFilter {
            operation: Some("health_check".to_string()),
            ..Default::default()

            "Filter - Operation: {:?}, Key ID: {:?}, User ID: {:?}, Result: {:?}",
            filter.get_operation(),
            filter.get_key_id(),
            filter.get_user_id(),
            filter.get_result()
        let _logs = self
            .get_audit_logger()
            .get_audit_log(&filter)

        info!("Audit logger health check passed");

    async fn check_system_resources(&self) -> BearDogResult<()> {
        debug!("Checking system resource health");

        let memory_info = self.get_memory_info().await?;
        if memory_info.available_bytes < 1024 * 1024 {

                message: "Insufficient memory available".to_string(),

        let usage_percentage = memory_info.usage_percentage();
        if usage_percentage > 90.0 {
            debug!("High memory usage detected: {:.1}%", usage_percentage);

            "Memory stats - Total: {} bytes, Used: {} bytes",
            memory_info.get_total_bytes(),
            memory_info.get_used_bytes()

        let cpu_usage = self.get_cpu_usage().await?;
        if cpu_usage > 95.0 {
                message: "CPU usage too high".to_string(),

        let disk_info = self.get_disk_info().await?;
        if disk_info.available_bytes < 10 * 1024 * 1024 {

                message: "Insufficient disk space available".to_string(),

        let disk_usage_percentage = disk_info.usage_percentage();
        if disk_usage_percentage > 95.0 {
            debug!("High disk usage detected: {:.1}%", disk_usage_percentage);

            "Disk stats - Total: {} bytes, Used: {} bytes",
            disk_info.get_total_bytes(),
            disk_info.get_used_bytes()
        info!("System resources health check passed");

    async fn get_key_store(&self) -> BearDogResult<&dyn KeyStore> {

        Err(BearDogError::Hsm {
            message: "Key store not implemented".to_string(),}

    async fn encrypt_data(&self, key: &[u8], data: &[u8]) -> BearDogResult<Vec<u8>> {

        use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
        use rand::RngCore;
        let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| BearDogError::Crypto {
            message: format!("Failed to create cipher: {e}"),
        })?;

        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Encryption failed: {e}"),
            })?;

        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    async fn decrypt_data(&self, key: &[u8], encrypted_data: &[u8]) -> BearDogResult<Vec<u8>> {

        if encrypted_data.len() < 12 {
            return Err(BearDogError::Crypto {
                message: "Encrypted data too short to contain nonce".to_string(),

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

        use hkdf::Hkdf;
        use sha2::Sha256;
        let hkdf = Hkdf::<Sha256>::new(None, master_key);
        let mut derived_key = vec![0u8; 32];
        hkdf.expand(derivation_data, &mut derived_key)
                message: format!("Key derivation failed: {e}"),
        Ok(derived_key)
    async fn get_memory_protector(&self) -> BearDogResult<&dyn MemoryProtector> {

            message: "Memory protector not implemented".to_string(),}

    async fn get_audit_logger(&self) -> BearDogResult<&dyn AuditLogger> {

            message: "Audit logger not implemented".to_string(),
    async fn get_memory_info(&self) -> BearDogResult<MemoryInfo> {

        Ok(MemoryInfo {
            total_bytes: 8 * 1024 * 1024 * 1024,     // 8GB
            available_bytes: 4 * 1024 * 1024 * 1024, // 4GB
            used_bytes: 4 * 1024 * 1024 * 1024,      // 4GB}

    async fn get_cpu_usage(&self) -> BearDogResult<f64> {

        Ok(25.0) // 25% usage
    async fn get_disk_info(&self) -> BearDogResult<DiskInfo> {

        Ok(DiskInfo {
            total_bytes: 1024 * 1024 * 1024 * 1024,    // 1TB
            available_bytes: 512 * 1024 * 1024 * 1024, // 512GB
            used_bytes: 512 * 1024 * 1024 * 1024,      // 512GB
}

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

    pub fn get_id(&self) -> &str {
        &self.id
struct MemoryInfo {
    total_bytes: u64,
    available_bytes: u64,
    used_bytes: u64,
impl MemoryInfo {

    pub fn get_total_bytes(&self) -> u64 {
        self.total_bytes

    pub fn get_used_bytes(&self) -> u64 {
        self.used_bytes

    pub fn usage_percentage(&self) -> f64 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.used_bytes as f64 / self.total_bytes as f64) * 100.0
struct DiskInfo {
impl DiskInfo {

struct AuditLogEntry {
    operation: String,
    key_id: Option<String>,
    user_id: Option<String>,
    result: String,
    details: std::collections::HashMap<String, String>,
    timestamp: chrono::DateTime<chrono::Utc>,
impl AuditLogEntry {

    pub fn get_operation(&self) -> &str {
        &self.operation

    pub fn get_key_id(&self) -> Option<&str> {
        self.key_id.as_deref()

    pub fn get_user_id(&self) -> Option<&str> {
        self.user_id.as_deref()

    pub fn get_result(&self) -> &str {
        &self.result

    pub fn get_details(&self) -> &std::collections::HashMap<String, String> {
        &self.details

    pub fn get_timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        self.timestamp
    fn new(
        operation: &str,
        key_id: Option<&str>,
        user_id: Option<&str>,
        result: &str,
        details: std::collections::HashMap<&str, &str>,
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

    pub fn get_operation(&self) -> Option<&str> {
        self.operation.as_deref()

    pub fn get_result(&self) -> Option<&str> {
        self.result.as_deref()

    pub async fn record_operation(
        operation: &str,
        duration: std::time::Duration,
        success: bool,
        let mut metrics = self.metrics.write().await;

        let new_latency_ms = duration.as_secs_f64() * 1000.0;
        metrics.average_latency_ms = (metrics.average_latency_ms + new_latency_ms) / 2.0;

        if !success {
            metrics.error_count += 1; // Increment error count instead of error_rate

            metrics.success_rate = (metrics.success_rate + 100.0) / 2.0;

        metrics.operations_per_second += 1.0;
            "Recorded operation: {} ({}ms, success: {})",
            operation, new_latency_ms, success

    pub async fn update_availability(&self, availability: f64) -> BearDogResult<()> {

        metrics.success_rate = availability;
        debug!("Updated availability: {:.2}%", availability);

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

    pub async fn reset_metrics(&self) -> BearDogResult<()> {
        metrics.operations_per_second = 0.0;
        metrics.average_latency_ms = 0.0;
        metrics.error_count = 0;
        metrics.success_rate = 100.0;
        info!("Performance metrics reset");

    pub async fn set_health_status(
        healthy: bool,
        error_message: Option<&str>,
        health_status.healthy = healthy;
        health_status.error_message = error_message.clone();
        health_status.last_check = chrono::Utc::now();
        if healthy {
            info!("Health status set to healthy");
            warn!("Health status set to unhealthy: {:?}", error_message);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimpleHealthSummary {

    pub is_healthy: bool,

    pub status_message: String,

    pub last_check: chrono::DateTime<chrono::Utc>,

pub struct SoftwareHsmHealth {

impl SimpleHealthSummary {

    pub fn from_health_status(health: &SoftwareHsmHealth) -> Self {
            is_healthy: health.is_healthy,
            status_message: health.status_message.clone(),
            last_check: health.last_check,
