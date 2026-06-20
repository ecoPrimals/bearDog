// SPDX-License-Identifier: AGPL-3.0-or-later

//! Audit logger implementation for software HSM

use super::super::types::{AuditLogEntry, AuditLogFilter, AuditLogger, OperationResult};
use super::storage::PersistentAuditStorage;
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use std::fmt::Write as _;
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::debug;

/// Capacity for the in-memory ring buffer of raw [`beardog_types::hsm::AuditEvent`] records.
const MEMORY_AUDIT_EVENT_CAP: usize = 10_000;

/// One in-memory audit record with wall-clock ingestion time.
#[derive(Debug, Clone)]
pub struct MemoryAuditRecord {
    /// When the event was recorded into the ring buffer (UTC).
    pub recorded_at: DateTime<Utc>,
    /// Original HSM audit event.
    pub event: beardog_types::hsm::AuditEvent,
}

fn utc_to_system_time(dt: DateTime<Utc>) -> SystemTime {
    let secs = dt.timestamp();
    let nanos = dt.timestamp_subsec_nanos();
    if secs >= 0 {
        UNIX_EPOCH + Duration::new(secs.cast_unsigned(), nanos)
    } else {
        UNIX_EPOCH
    }
}

fn audit_event_to_log_entry(event: &beardog_types::hsm::AuditEvent) -> AuditLogEntry {
    let timestamp: DateTime<Utc> = event.timestamp.into();
    let outcome = event
        .metadata
        .get("outcome")
        .map(std::string::String::as_str);
    let result = match outcome {
        Some("failure" | "failed" | "error") => OperationResult::Failure(
            event
                .metadata
                .get("error")
                .cloned()
                .unwrap_or_else(|| "operation failed".to_string()),
        ),
        _ => OperationResult::Success,
    };
    AuditLogEntry {
        timestamp,
        operation: format!("audit_event:{}", event.event_type),
        user_id: event.metadata.get("user_id").cloned(),
        key_id: event.metadata.get("resource").cloned(),
        result,
        metadata: event.metadata.clone(),
    }
}

/// Context for crypto operations logging
#[derive(Debug, Clone)]
pub struct CryptoOperationLog {
    /// Type of crypto operation performed
    pub operation_type: String,
    /// User who initiated the operation
    pub user_id: Option<String>,
    /// Key used in the operation
    pub key_id: Option<String>,
    /// Algorithm used
    pub algorithm: String,
    /// Size of data processed in bytes
    pub data_size: usize,
    /// Whether the operation succeeded
    pub success: bool,
    /// Processing time in milliseconds
    pub processing_time_ms: Option<u64>,
}

/// Default audit logger implementation
#[derive(Debug, Clone)]
pub struct DefaultAuditLogger {
    storage: Arc<PersistentAuditStorage>,
    memory_audit_events: Arc<RwLock<VecDeque<MemoryAuditRecord>>>,
}

impl DefaultAuditLogger {
    /// # Errors
    ///
    /// Returns an error if the storage backend cannot be initialized.
    ///
    /// Resolves audit log path from `BEARDOG_AUDIT_DIR` (or `--audit-dir`),
    /// falling back to the system temp directory so beardog never crashes
    /// writing to a read-only CWD (e.g. Android `/data/local/tmp`).
    pub async fn new() -> Result<Self, BearDogError> {
        let dir = std::env::var(env_keys::ENV_AUDIT_DIR).map_or_else(
            |_| std::env::temp_dir().join("beardog"),
            std::path::PathBuf::from,
        );
        let storage_path = dir.join("audit.log");
        Self::with_storage_path(storage_path).await
    }

    /// # Errors
    ///
    /// Returns an error if hashing fails.
    /// Create audit logger with custom storage path
    pub async fn with_storage_path(storage_path: std::path::PathBuf) -> Result<Self, BearDogError> {
        let storage = Arc::new(PersistentAuditStorage::new(storage_path, 10000).await?);
        Ok(Self {
            storage,
            memory_audit_events: Arc::new(RwLock::new(VecDeque::new())),
        })
    }

    /// # Errors
    ///
    /// Returns an error if hashing fails.
    /// Log a typed HSM [`beardog_types::hsm::AuditEvent`]: records to a bounded in-memory ring
    /// buffer (newest entries retained) and persists a normalized [`AuditLogEntry`].
    pub async fn log_audit_event(
        &self,
        event: beardog_types::hsm::AuditEvent,
    ) -> Result<(), BearDogError> {
        let recorded_at = Utc::now();
        {
            let mut q = self.memory_audit_events.write().await;
            q.push_back(MemoryAuditRecord {
                recorded_at,
                event: event.clone(),
            });
            while q.len() > MEMORY_AUDIT_EVENT_CAP {
                q.pop_front();
            }
        }
        let entry = audit_event_to_log_entry(&event);
        self.storage.append_entry(&entry).await?;
        debug!(
            "Logged audit event type={} (memory buffer len capped at {})",
            event.event_type, MEMORY_AUDIT_EVENT_CAP
        );
        Ok(())
    }

    /// # Errors
    ///
    /// Returns an error if hashing fails.
    /// Log a detailed operation
    pub async fn log_detailed_operation(
        &self,
        operation: String,
        user_id: Option<String>,
        key_id: Option<String>,
        success: bool,
        error_message: Option<String>,
        details: std::collections::HashMap<String, String>,
    ) -> Result<(), BearDogError> {
        let entry = AuditLogEntry {
            timestamp: chrono::Utc::now(),
            operation,
            user_id,
            key_id,
            result: if success {
                OperationResult::Success
            } else {
                OperationResult::Failure(
                    error_message.unwrap_or_else(|| "Operation failed".to_string()),
                )
            },
            metadata: details,
        };
        self.storage.append_entry(&entry).await
    }

    /// # Errors
    ///
    /// Returns an error if hashing fails.
    /// Log a successful operation
    pub async fn log_success(
        &self,
        operation: &str,
        user_id: Option<&str>,
        key_id: Option<&str>,
    ) -> Result<(), BearDogError> {
        self.log_detailed_operation(
            operation.to_string(),
            user_id.map(std::string::ToString::to_string),
            key_id.map(std::string::ToString::to_string),
            true,
            None,
            std::collections::HashMap::new(),
        )
        .await
    }

    /// # Errors
    ///
    /// Returns an error if hashing fails.
    /// Log a failed operation
    pub async fn log_failure(
        &self,
        operation: &str,
        user_id: Option<&str>,
        key_id: Option<&str>,
        error: &str,
    ) -> Result<(), BearDogError> {
        self.log_detailed_operation(
            operation.to_string(),
            user_id.map(std::string::ToString::to_string),
            key_id.map(std::string::ToString::to_string),
            false,
            Some(error.to_string()),
            std::collections::HashMap::new(),
        )
        .await
    }

    /// # Errors
    ///
    /// Returns an error if hashing fails.
    /// Log key generation operation
    pub async fn log_key_generation(
        &self,
        key_id: &str,
        key_type: &str,
        algorithm: &str,
        success: bool,
    ) -> Result<(), BearDogError> {
        let mut details = std::collections::HashMap::new();
        details.insert("key_type".to_string(), key_type.to_string());
        details.insert("algorithm".to_string(), algorithm.to_string());

        self.log_detailed_operation(
            "key_generation".to_string(),
            None,
            Some(key_id.to_string()),
            success,
            None,
            details,
        )
        .await
    }

    /// # Errors
    ///
    /// Returns an error if hashing fails.
    /// Log key deletion
    pub async fn log_key_deletion(
        &self,
        key_id: &str,
        success: bool,
        error_message: Option<&str>,
    ) -> Result<(), BearDogError> {
        self.log_detailed_operation(
            "key_deletion".to_string(),
            None,
            Some(key_id.to_string()),
            success,
            error_message.map(std::string::ToString::to_string),
            std::collections::HashMap::new(),
        )
        .await
    }

    /// # Errors
    ///
    /// Returns an error if hashing fails.
    /// Log crypto operation
    pub async fn log_crypto_operation(
        &self,
        operation: CryptoOperationLog,
    ) -> Result<(), BearDogError> {
        let mut details = std::collections::HashMap::new();
        details.insert("algorithm".to_string(), operation.algorithm.clone());
        details.insert(
            "data_size_bytes".to_string(),
            operation.data_size.to_string(),
        );

        if let Some(time_ms) = operation.processing_time_ms {
            details.insert("processing_time_ms".to_string(), time_ms.to_string());
        }

        self.log_detailed_operation(
            format!("crypto_{}", operation.operation_type),
            operation.user_id,
            operation.key_id,
            operation.success,
            None,
            details,
        )
        .await
    }

    /// # Errors
    ///
    /// Returns an error if serialization fails.
    /// Log security event
    pub async fn log_security_event(
        &self,
        event_type: &str,
        severity: &str,
        description: &str,
        additional_info: std::collections::HashMap<String, String>,
    ) -> Result<(), BearDogError> {
        let mut details = additional_info;
        details.insert("severity".to_string(), severity.to_string());
        details.insert("description".to_string(), description.to_string());

        self.log_detailed_operation(
            format!("security_{event_type}"),
            None,
            None,
            true,
            None,
            details,
        )
        .await
    }

    /// # Errors
    ///
    /// Returns an error if serialization fails.
    /// Get audit statistics
    pub async fn get_audit_statistics(
        &self,
    ) -> Result<beardog_types::hsm::AuditStatistics, BearDogError> {
        let filter = AuditLogFilter::default();
        let entries = self.storage.get_entries(&filter).await?;
        let mut stats = beardog_types::hsm::AuditStatistics::new();
        let mut last_activity = SystemTime::UNIX_EPOCH;
        for entry in &entries {
            stats.total_operations = stats.total_operations.saturating_add(1);
            if matches!(entry.result, OperationResult::Failure(_)) {
                stats.failed_operations = stats.failed_operations.saturating_add(1);
            }
            let t = utc_to_system_time(entry.timestamp);
            if t > last_activity {
                last_activity = t;
            }
        }
        if stats.total_operations > 0 {
            stats.last_audit = last_activity;
        }
        Ok(stats)
    }

    /// # Errors
    ///
    /// Returns an error if serialization fails.
    /// Export audit log in specified format
    pub async fn export_audit_log(&self, format: &str) -> Result<Vec<u8>, BearDogError> {
        match format.to_lowercase().as_str() {
            "json" => self.export_as_json().await,
            "csv" => self.export_as_csv().await,
            _ => Err(BearDogError::invalid_input(&format!(
                "Unsupported export format: {format}"
            ))),
        }
    }

    async fn export_as_json(&self) -> Result<Vec<u8>, BearDogError> {
        let filter = AuditLogFilter::default();
        let entries = self.storage.get_entries(&filter).await?;
        let json = serde_json::to_string_pretty(&entries)
            .map_err(|e| BearDogError::serialization(&format!("JSON serialization failed: {e}")))?;
        Ok(json.into_bytes())
    }

    async fn export_as_csv(&self) -> Result<Vec<u8>, BearDogError> {
        let filter = AuditLogFilter::default();
        let entries = self.storage.get_entries(&filter).await?;

        let mut csv = String::from("timestamp,operation,user_id,key_id,success,error_message\n");
        for entry in entries {
            let success = matches!(entry.result, OperationResult::Success);
            let error = match entry.result {
                OperationResult::Failure(ref msg) => msg.as_str(),
                OperationResult::Success => "",
            };
            writeln!(
                csv,
                "{},{},{},{},{},{}",
                entry.timestamp,
                entry.operation,
                entry.user_id.as_deref().unwrap_or(""),
                entry.key_id.as_deref().unwrap_or(""),
                success,
                error
            )
            .map_err(|e| {
                BearDogError::serialization(&format!("CSV audit export line failed: {e}"))
            })?;
        }

        Ok(csv.into_bytes())
    }
}

impl AuditLogger for DefaultAuditLogger {
    fn log_operation(
        &self,
        operation: &AuditLogEntry,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let op = operation.clone();
        let storage = Arc::clone(&self.storage);
        async move { storage.log_entry(op).await }
    }

    fn get_audit_log(
        &self,
        filter: &AuditLogFilter,
    ) -> impl Future<Output = Result<Vec<AuditLogEntry>, BearDogError>> + Send {
        let filter = filter.clone();
        let storage = Arc::clone(&self.storage);
        async move { storage.get_entries(&filter).await }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn audit_logger_with_temp_path_logs_and_exports() {
        let dir = tempdir().expect("tempdir for audit logger test");
        let path = dir.path().join("audit_test.log");
        let logger = DefaultAuditLogger::with_storage_path(path)
            .await
            .expect("logger");

        logger
            .log_success("op1", Some("u1"), Some("k1"))
            .await
            .expect("log");
        logger
            .log_failure("op2", None, Some("k2"), "boom")
            .await
            .expect("fail log");
        logger
            .log_key_generation("kid", "ecc", "p256", true)
            .await
            .expect("kg");
        logger
            .log_key_deletion("kid", false, Some("nope"))
            .await
            .expect("del");

        let crypto = CryptoOperationLog {
            operation_type: "sign".to_string(),
            user_id: Some("u".to_string()),
            key_id: Some("k".to_string()),
            algorithm: "ed25519".to_string(),
            data_size: 32,
            success: true,
            processing_time_ms: Some(12),
        };
        logger.log_crypto_operation(crypto).await.expect("crypto");

        logger
            .log_security_event("alert", "high", "desc", std::collections::HashMap::new())
            .await
            .expect("sec");

        let stats = logger.get_audit_statistics().await.expect("stats");
        assert_eq!(stats.total_operations, 6);
        assert_eq!(stats.failed_operations, 2);

        let json = logger.export_audit_log("json").await.expect("json export");
        assert!(json.starts_with(b"[") || !json.is_empty());

        let csv = logger.export_audit_log("csv").await.expect("csv export");
        let csv_s = String::from_utf8(csv).expect("CSV export must be UTF-8");
        assert!(csv_s.contains("timestamp"));

        assert!(logger.export_audit_log("weird").await.is_err());
    }

    #[tokio::test]
    async fn log_audit_event_records_memory_and_storage() {
        let dir = tempdir().expect("tempdir for audit logger test");
        let logger = DefaultAuditLogger::with_storage_path(dir.path().join("a.log"))
            .await
            .expect("audit logger with temp path");
        let ev = beardog_types::hsm::AuditEvent::default();
        logger
            .log_audit_event(ev)
            .await
            .expect("default audit event should log");
        let stats = logger.get_audit_statistics().await.expect("stats");
        assert_eq!(stats.total_operations, 1);
    }
}
