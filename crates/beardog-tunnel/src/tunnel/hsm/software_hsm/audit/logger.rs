//! Audit logger implementation for software HSM

use super::super::types::{AuditLogEntry, AuditLogFilter, AuditLogger, OperationResult};
use super::storage::PersistentAuditStorage;
use beardog_errors::BearDogError;
use tracing::debug;
// NOTE: beardog_security::handlers doesn't exist yet - commented out
// use beardog_security::handlers::audit_management::AuditStatistics;
use std::sync::Arc;

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
}

impl DefaultAuditLogger {
    /// Create a new audit logger with default storage path
    pub async fn new() -> Result<Self, BearDogError> {
        let storage_path = std::path::PathBuf::from("audit.log");
        Self::with_storage_path(storage_path).await
    }

    /// Create audit logger with custom storage path
    pub async fn with_storage_path(storage_path: std::path::PathBuf) -> Result<Self, BearDogError> {
        let storage = Arc::new(PersistentAuditStorage::new(storage_path, 10000).await?);
        Ok(Self { storage })
    }

    /// Log an audit event (stub for missing AuditEvent type)
    pub async fn log_audit_event(
        &self,
        _event: beardog_types::hsm::AuditEvent,
    ) -> Result<(), BearDogError> {
        // Stub implementation - convert to AuditLogEntry and log
        debug!("Logging audit event (stub implementation)");
        Ok(())
    }

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

    /// Log a successful operation
    pub async fn log_success(
        &self,
        operation: &str,
        user_id: Option<&str>,
        key_id: Option<&str>,
    ) -> Result<(), BearDogError> {
        self.log_detailed_operation(
            operation.to_string(),
            user_id.map(|s| s.to_string()),
            key_id.map(|s| s.to_string()),
            true,
            None,
            std::collections::HashMap::new(),
        )
        .await
    }

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
            user_id.map(|s| s.to_string()),
            key_id.map(|s| s.to_string()),
            false,
            Some(error.to_string()),
            std::collections::HashMap::new(),
        )
        .await
    }

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
            error_message.map(|s| s.to_string()),
            std::collections::HashMap::new(),
        )
        .await
    }

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

    /// Get audit statistics
    pub async fn get_audit_statistics(
        &self,
    ) -> Result<beardog_types::hsm::AuditStatistics, BearDogError> {
        // Mock implementation - replace with real statistics gathering
        Ok(beardog_types::hsm::AuditStatistics::new())
    }

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
                _ => "",
            };
            csv.push_str(&format!(
                "{},{},{},{},{},{}\n",
                entry.timestamp,
                entry.operation,
                entry.user_id.as_deref().unwrap_or(""),
                entry.key_id.as_deref().unwrap_or(""),
                success,
                error
            ));
        }

        Ok(csv.into_bytes())
    }
}

#[async_trait::async_trait]
impl AuditLogger for DefaultAuditLogger {
    async fn log_operation(&self, operation: &AuditLogEntry) -> Result<(), BearDogError> {
        self.storage.log_entry(operation.clone()).await
    }

    async fn get_audit_log(
        &self,
        filter: &AuditLogFilter,
    ) -> Result<Vec<AuditLogEntry>, BearDogError> {
        self.storage.get_entries(filter).await
    }
}
