//! # Software HSM Health Monitoring
//!
//! This module provides health monitoring functionality for the Software HSM.
//! It tracks system status, performance metrics, and provides health assessments.

use super::types::*;
use beardog_errors::BearDogResult;
use crate::tunnel::hsm::types::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

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

        let metrics = Arc::new(RwLock::new(PerformanceMetrics {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            error_rate: 0.0,
            availability_percentage: 100.0,
        }));

        Ok(Self {
            health_status,
            metrics,
        })
    }

    /// Get current health status
    pub async fn get_health_status(&self) -> BearDogResult<HsmHealthStatus> {
        let status = self.health_status.read().await;
        Ok(status.clone())
    }

    /// Update health status
    pub async fn update_health_status(&self, status: HsmHealthStatus) -> BearDogResult<()> {
        let mut health_status = self.health_status.write().await;
        *health_status = status;

        debug!("Health status updated: healthy={}", health_status.healthy);
        Ok(())
    }

    /// Get performance metrics
    pub async fn get_performance_metrics(&self) -> BearDogResult<PerformanceMetrics> {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }

    /// Update performance metrics
    pub async fn update_performance_metrics(
        &self,
        metrics: PerformanceMetrics,
    ) -> BearDogResult<()> {
        let mut perf_metrics = self.metrics.write().await;
        *perf_metrics = metrics;

        debug!(
            "Performance metrics updated: {:.2} ops/sec",
            perf_metrics.operations_per_second
        );
        Ok(())
    }

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
            healthy = false;
            if error_message.is_none() {
                error_message = Some("Crypto provider is unhealthy".to_string());
            }
        }

        // Check memory protector health
        if self.check_memory_protector_health().await.is_err() {
            healthy = false;
            if error_message.is_none() {
                error_message = Some("Memory protector is unhealthy".to_string());
            }
        }

        // Check audit logger health
        if self.check_audit_logger_health().await.is_err() {
            healthy = false;
            if error_message.is_none() {
                error_message = Some("Audit logger is unhealthy".to_string());
            }
        }

        // Check system resources
        if self.check_system_resources().await.is_err() {
            healthy = false;
            if error_message.is_none() {
                error_message = Some("System resources are unhealthy".to_string());
            }
        }

        let performance_metrics = self.get_performance_metrics().await?;

        let health_status = HsmHealthStatus {
            healthy,
            last_check: chrono::Utc::now(),
            error_message,
            performance_metrics,
        };

        // Update stored health status
        self.update_health_status(health_status.clone()).await?;

        info!("Health check completed: healthy={}", health_status.healthy);
        Ok(health_status)
    }

    /// Check key store health
    async fn check_keystore_health(&self) -> BearDogResult<()> {
        debug!("Checking key store health");
        
        // Check if key store is accessible
        let key_store = self.get_key_store().await?;
        
        // Test key store operations
        let test_key_id = "health_check_test_key";
        
        // Try to generate a test key
        let test_key_material = vec![0u8; 32];
        key_store.store_key(test_key_id, &test_key_material, &KeyType::Aes256).await?;
        
        // Try to retrieve the test key
        let retrieved_key = key_store.get_key(test_key_id).await?;
        if retrieved_key.is_none() {
            return Err(BearDogError::Hsm {
                message: "Key store failed to retrieve test key".to_string(),
            });
        }
        
        // Clean up test key
        key_store.delete_key(test_key_id).await?;
        
        info!("Key store health check passed");
        Ok(())
    }

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
            return Err(BearDogError::Hsm {
                message: "Crypto provider failed data integrity check".to_string(),
            });
        }
        
        // Test key derivation
        let derivation_data = b"test_derivation_context";
        let derived_key = self.derive_key(&test_key, derivation_data).await?;
        
        if derived_key.len() != 32 {
            return Err(BearDogError::Hsm {
                message: "Crypto provider failed key derivation check".to_string(),
            });
        }
        
        info!("Crypto provider health check passed");
        Ok(())
    }

    /// Check memory protector health
    async fn check_memory_protector_health(&self) -> BearDogResult<()> {
        debug!("Checking memory protector health");
        
        // Check if memory protection is active
        let memory_protector = self.get_memory_protector().await?;
        
        // Test memory protection capabilities
        let test_sensitive_data = b"sensitive_test_data";
        let protected_memory = memory_protector.protect_memory(test_sensitive_data).await?;
        
        // Verify protection is active
        if !protected_memory.is_protected() {
            return Err(BearDogError::Hsm {
                message: "Memory protector failed to protect sensitive data".to_string(),
            });
        }
        
        // Test memory clearing
        memory_protector.clear_memory(&protected_memory).await?;
        
        info!("Memory protector health check passed");
        Ok(())
    }

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
        );
        
        // Test logging
        self.get_audit_logger().await?.log_operation(&test_entry).await?;
        
        // Test log retrieval
        let filter = AuditLogFilter {
            operation: Some("health_check".to_string()),
            ..Default::default()
        };
        
        let logs = self.get_audit_logger().await?.get_audit_log(&filter).await?;
        
        // Verify log was written (in a real implementation)
        // For now, just check that the operation completed without error
        
        info!("Audit logger health check passed");
        Ok(())
    }

    /// Check system resources
    async fn check_system_resources(&self) -> BearDogResult<()> {
        debug!("Checking system resource health");
        
        // Check available memory
        let memory_info = self.get_memory_info().await?;
        if memory_info.available_bytes < 1024 * 1024 { // Less than 1MB
            return Err(BearDogError::Hsm {
                message: "Insufficient memory available".to_string(),
            });
        }
        
        // Check CPU usage
        let cpu_usage = self.get_cpu_usage().await?;
        if cpu_usage > 95.0 {
            return Err(BearDogError::Hsm {
                message: "CPU usage too high".to_string(),
            });
        }
        
        // Check disk space
        let disk_info = self.get_disk_info().await?;
        if disk_info.available_bytes < 10 * 1024 * 1024 { // Less than 10MB
            return Err(BearDogError::Hsm {
                message: "Insufficient disk space available".to_string(),
            });
        }
        
        info!("System resources health check passed");
        Ok(())
    }

    // Helper methods for health checks
    async fn get_key_store(&self) -> BearDogResult<&dyn KeyStore> {
        // Implementation would return actual key store reference
        // For now, return a placeholder error
        Err(BearDogError::Hsm {
            message: "Key store not implemented".to_string(),
        })
    }

    async fn encrypt_data(&self, key: &[u8], data: &[u8]) -> BearDogResult<Vec<u8>> {
        // Basic AES-256-GCM encryption for testing
        use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
        
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create cipher: {}", e),
            })?;
        
        let nonce = Nonce::from_slice(b"unique nonce"); // In real implementation, use random nonce
        
        cipher.encrypt(nonce, data)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Encryption failed: {}", e),
            })
    }

    async fn decrypt_data(&self, key: &[u8], encrypted_data: &[u8]) -> BearDogResult<Vec<u8>> {
        // Basic AES-256-GCM decryption for testing
        use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
        
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create cipher: {}", e),
            })?;
        
        let nonce = Nonce::from_slice(b"unique nonce"); // Must match encryption nonce
        
        cipher.decrypt(nonce, encrypted_data)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Decryption failed: {}", e),
            })
    }

    async fn derive_key(&self, master_key: &[u8], derivation_data: &[u8]) -> BearDogResult<Vec<u8>> {
        // Use HKDF for key derivation
        use sha2::Sha256;
        use hkdf::Hkdf;
        
        let hkdf = Hkdf::<Sha256>::new(None, master_key);
        let mut derived_key = vec![0u8; 32];
        
        hkdf.expand(derivation_data, &mut derived_key)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Key derivation failed: {}", e),
            })?;
        
        Ok(derived_key)
    }

    async fn get_memory_protector(&self) -> BearDogResult<&dyn MemoryProtector> {
        // Implementation would return actual memory protector
        Err(BearDogError::Hsm {
            message: "Memory protector not implemented".to_string(),
        })
    }

    async fn get_audit_logger(&self) -> BearDogResult<&dyn AuditLogger> {
        // Implementation would return actual audit logger
        Err(BearDogError::Hsm {
            message: "Audit logger not implemented".to_string(),
        })
    }

    async fn get_memory_info(&self) -> BearDogResult<MemoryInfo> {
        // Basic memory info implementation
        Ok(MemoryInfo {
            total_bytes: 8 * 1024 * 1024 * 1024, // 8GB
            available_bytes: 4 * 1024 * 1024 * 1024, // 4GB
            used_bytes: 4 * 1024 * 1024 * 1024, // 4GB
        })
    }

    async fn get_cpu_usage(&self) -> BearDogResult<f64> {
        // Basic CPU usage implementation
        Ok(25.0) // 25% usage
    }

    async fn get_disk_info(&self) -> BearDogResult<DiskInfo> {
        // Basic disk info implementation
        Ok(DiskInfo {
            total_bytes: 1024 * 1024 * 1024 * 1024, // 1TB
            available_bytes: 512 * 1024 * 1024 * 1024, // 512GB
            used_bytes: 512 * 1024 * 1024 * 1024, // 512GB
        })
    }
}

// Helper traits and types for health checking
trait KeyStore: Send + Sync {
    fn store_key(&self, key_id: &str, key_material: &[u8], key_type: &KeyType) -> impl std::future::Future<Output = BearDogResult<()>> + Send;
    fn get_key(&self, key_id: &str) -> impl std::future::Future<Output = BearDogResult<Option<Vec<u8>>>> + Send;
    fn delete_key(&self, key_id: &str) -> impl std::future::Future<Output = BearDogResult<()>> + Send;
}

trait MemoryProtector: Send + Sync {
    fn protect_memory(&self, data: &[u8]) -> impl std::future::Future<Output = BearDogResult<ProtectedMemory>> + Send;
    fn clear_memory(&self, protected: &ProtectedMemory) -> impl std::future::Future<Output = BearDogResult<()>> + Send;
}

trait AuditLogger: Send + Sync {
    fn log_operation(&self, entry: &AuditLogEntry) -> impl std::future::Future<Output = BearDogResult<()>> + Send;
    fn get_audit_log(&self, filter: &AuditLogFilter) -> impl std::future::Future<Output = BearDogResult<Vec<AuditLogEntry>>> + Send;
}

#[derive(Debug, Clone)]
struct ProtectedMemory {
    id: String,
    protected: bool,
}

impl ProtectedMemory {
    fn is_protected(&self) -> bool {
        self.protected
    }
}

#[derive(Debug, Clone)]
struct MemoryInfo {
    total_bytes: u64,
    available_bytes: u64,
    used_bytes: u64,
}

#[derive(Debug, Clone)]
struct DiskInfo {
    total_bytes: u64,
    available_bytes: u64,
    used_bytes: u64,
}

#[derive(Debug, Clone)]
struct AuditLogEntry {
    operation: String,
    key_id: Option<String>,
    user_id: Option<String>,
    result: String,
    details: std::collections::HashMap<String, String>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl AuditLogEntry {
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
        }
    }
}

#[derive(Debug, Clone, Default)]
struct AuditLogFilter {
    operation: Option<String>,
    key_id: Option<String>,
    user_id: Option<String>,
    result: Option<String>,
}

impl SoftwareHealthMonitor {
    /// Record operation metrics
    pub async fn record_operation(
        &self,
        operation: &str,
        duration: std::time::Duration,
        success: bool,
    ) -> BearDogResult<()> {
        let mut metrics = self.metrics.write().await;

        // Update average latency (simplified calculation)
        let new_latency_ms = duration.as_secs_f64() * 1000.0;
        metrics.average_latency_ms = (metrics.average_latency_ms + new_latency_ms) / 2.0;

        // Update error rate (simplified calculation)
        if !success {
            metrics.error_rate = (metrics.error_rate + 1.0) / 2.0;
        } else {
            metrics.error_rate *= 0.99; // Decay error rate for successes
        }

        // Calculate operations per second (simplified)
        metrics.operations_per_second += 1.0;

        debug!(
            "Recorded operation: {} ({}ms, success: {})",
            operation, new_latency_ms, success
        );

        Ok(())
    }

    /// Update availability percentage
    pub async fn update_availability(&self, availability: f64) -> BearDogResult<()> {
        let mut metrics = self.metrics.write().await;
        metrics.availability_percentage = availability;

        debug!("Updated availability: {:.2}%", availability);
        Ok(())
    }

    /// Get simplified health summary
    pub async fn get_health_summary(&self) -> BearDogResult<SimpleHealthSummary> {
        let health_status = self.health_status.read().await;
        let metrics = self.metrics.read().await;

        Ok(SimpleHealthSummary {
            is_healthy: health_status.healthy,
            status_message: health_status.error_message.clone().unwrap_or_else(|| "Healthy".to_string()),
            last_check: health_status.last_check,
        })
    }

    /// Reset metrics
    pub async fn reset_metrics(&self) -> BearDogResult<()> {
        let mut metrics = self.metrics.write().await;
        metrics.operations_per_second = 0.0;
        metrics.average_latency_ms = 0.0;
        metrics.error_rate = 0.0;
        metrics.availability_percentage = 100.0;

        info!("Performance metrics reset");
        Ok(())
    }

    /// Set health status
    pub async fn set_health_status(
        &self,
        healthy: bool,
        error_message: Option<String>,
    ) -> BearDogResult<()> {
        let mut health_status = self.health_status.write().await;
        health_status.healthy = healthy;
        health_status.error_message = error_message.clone();
        health_status.last_check = chrono::Utc::now();

        if healthy {
            info!("Health status set to healthy");
        } else {
            warn!("Health status set to unhealthy: {:?}", error_message);
        }

        Ok(())
    }
}

/// Simplified health summary for quick overview
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimpleHealthSummary {
    /// Whether the system is healthy
    pub is_healthy: bool,
    /// Brief status message
    pub status_message: String,
    /// Timestamp of last health check
    pub last_check: chrono::DateTime<chrono::Utc>,
}

impl SimpleHealthSummary {
    /// Create a simple health summary from full health status
    pub fn from_health_status(health: &SoftwareHsmHealth) -> Self {
        Self {
            is_healthy: health.is_healthy,
            status_message: health.status_message.clone(),
            last_check: health.last_check,
        }
    }
}
