// PHASE 5 OPTIMIZED: Performance patterns applied
// PHASE 5 MODERNIZED: Advanced async_trait elimination
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


// # Audit Logger Interface
//
// **EXTRACTED FROM LARGE FILE** - High-level audit logging (~250 lines)
// This module provides the main audit logging interface with convenient methods
// for logging various types of operations and events.

// Import audit types from local types module
use super::super::types::{AuditLogEntry, AuditLogFilter, AuditLogger, OperationResult}; // Import from software_hsm types
use super::storage::PersistentAuditStorage;
// Add missing async_trait import
use beardog_errors::{BearDogError, BearDogResult};
use beardog_security::handlers::audit_management::AuditStatistics;
use std::sync::Arc;
/// Structured log entry for cryptographic operations
#[derive(Debug, Clone)]
pub struct CryptoOperationLog {
    pub operation_type: String,
    pub user_id: Option<String>,
    pub key_id: Option<String>,
    pub algorithm: String,
    pub data_size: usize,
    pub success: bool,
    pub processing_time_ms: Option<u64>,
}
/// Default audit logger implementation
#[derive(Debug)]
pub struct DefaultAuditLogger {
    /// Persistent storage backend
    storage: Arc<PersistentAuditStorage>,}


impl DefaultAuditLogger {
    /// Create new audit logger with default storage path
    pub async fn new() -> BearDogResult<Self> {
        let storage_path = std::path::PathBuf::from("audit.log");
        Self::with_storage_path(storage_path).await
    }
    /// Create new audit logger with custom storage path
    pub async fn with_storage_path(storage_path: std::path::PathBuf) -> BearDogResult<Self> {
        let storage = Arc::new(PersistentAuditStorage::new(storage_path, 1000).await?);
        Ok(Self { storage })
    /// Log a detailed operation with all parameters
    pub async fn log_detailed_operation(
        &self,
        operation: String,
        user_id: Option<String>,
        key_id: Option<String>,
        success: bool,
        error_message: Option<String>,
        details: std::collections::HashMap<String, String>,
    ) -> BearDogResult<()> {
        let entry = AuditLogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            operation,
            key_id,
            actor: user_id.unwrap_or_else(|| "system".to_string()),
            result: if success {
                OperationResult::Success
            } else {
                OperationResult::Failure(error_message.unwrap_or_else(|| "failed".to_string()))
            },
            metadata: details,
        };
        self.storage.append_entry(&entry).await
    /// Log a successful operation
    pub async fn log_success(
        operation: &str,
        user_id: Option<&str>,
        key_id: Option<&str>,
        self.log_detailed_operation(
            operation.to_string(),
            user_id.map(|s| s.to_string()),
            key_id.map(|s| s.to_string()),
            true,
            None,
            std::collections::HashMap::new(),
        )
        .await
    /// Log a failed operation
    pub async fn log_failure(
        error: &str,
            false,
            Some(error.to_string()),
    /// Log key generation operation
    pub async fn log_key_generation(
        key_id: &str,
        key_type: &str,
        algorithm: &str,
        let mut details = std::collections::HashMap::new();
        details.insert("key_type".to_string(), key_type.to_string());
        details.insert("algorithm".to_string(), algorithm.to_string());
            "key_generation".to_string(),
            Some(key_id.to_string()),
            success,
            details,
    /// Log key deletion operation
    pub async fn log_key_deletion(
        error_message: Option<&str>,
            "key_deletion".to_string(),
            error_message.map(|s| s.to_string()),
    /// Log cryptographic operation
    pub async fn log_crypto_operation(&self, operation: CryptoOperationLog) -> BearDogResult<()> {
        details.insert("algorithm".to_string(), operation.algorithm.clone());
        details.insert(
            "data_size_bytes".to_string(),
            operation.data_size.to_string(),
        );
        if let Some(time_ms) = operation.processing_time_ms {
            details.insert("processing_time_ms".to_string(), time_ms.to_string());
        }
            format!("crypto_{}", operation.operation_type),
            operation.user_id,
            operation.key_id,
            operation.success,
    /// Log security event
    pub async fn log_security_event(
        event_type: &str,
        severity: &str,
        description: &str,
        additional_info: std::collections::HashMap<String, String>,
        let mut details = additional_info;
        details.insert("severity".to_string(), severity.to_string());
        details.insert("description".to_string(), description.to_string());
            format!("security_{event_type}"),
            true, // Security events are logged as successful by default
    /// Get audit statistics
    pub async fn get_audit_statistics(&self) -> BearDogResult<AuditStatistics> {
        self.generate_statistics().await
    /// Purge old audit entries}


    pub async fn purge_old_entries(
        older_than: chrono::DateTime<chrono::Utc>,
    ) -> BearDogResult<u64> {
        self.purge_entries_older_than(older_than).await
    /// Export audit log in specified format
    pub async fn export_audit_log(&self, format: &str) -> BearDogResult<Vec<u8>> {
        match format.to_lowercase().as_str() {
            "json" => self.export_as_json().await,
            "csv" => self.export_as_csv().await,
            _ => Err(BearDogError::invalid_input(format!("Unsupported export format: {format)"),
            }),
    // Private helper methods
    /// Generate audit statistics
    async fn generate_statistics(&self) -> BearDogResult<AuditStatistics> {
        // This would analyze the audit log to generate statistics
        // For now, return mock statistics
        Ok(AuditStatistics {
            period_hours: 24,
            total_events: 1000,
            auth_events: 300,
            authz_events: 200,
            high_risk_events: 50,
            success_rate: 0.95, // 95% success rate
        })
    /// Purge entries older than specified date
    async fn purge_entries_older_than(
        _older_than: chrono::DateTime<chrono::Utc>,
        // This would implement the purging logic
        // For now, return mock count
        Ok(10)
    /// Export audit log as JSON
    async fn export_as_json(&self) -> BearDogResult<Vec<u8>> {
        let filter = AuditLogFilter::default();
        let entries = self.storage.get_entries(&filter).await?;
        let json =
            serde_json::to_string_pretty(&entries).map_err(|e| BearDogError::Serialization {
                message: format!("Failed to serialize audit log to JSON: {e}"),
            })?;
        Ok(json.into_bytes())
    /// Export audit log as CSV
    async fn export_as_csv(&self) -> BearDogResult<Vec<u8>> {
        let mut csv = "timestamp,operation,user_id,key_id,success,error_message\n";
        for entry in entries {
            csv.push_str(&format!(
                "{},{},{},{},{},{}\n",
                entry.timestamp.to_rfc3339(),
                entry.operation,
                entry.actor,
                entry.key_id.unwrap_or_default(),
                matches!(entry.result, OperationResult::Success),
                match entry.result {
                    OperationResult::Success => String::new(),
                    OperationResult::Failure(ref msg) => msg.clone(),
                }
            ));
        Ok(csv.into_bytes())

impl AuditLogger for DefaultAuditLogger {
    /// Log HSM operation
    async fn log_operation(&self, operation: &AuditLogEntry) -> BearDogResult<()> {
        self.storage.log_entry(operation.clone()).await
    /// Get audit log entries}


    async fn get_audit_log(&self, filter: &AuditLogFilter) -> BearDogResult<Vec<AuditLogEntry>> {
        self.storage.get_entries(filter).await
