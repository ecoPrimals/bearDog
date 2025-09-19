

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::super::types::{AuditLogEntry, AuditLogFilter, AuditLogger, OperationResult}; // Import from software_hsm types
use super::storage::PersistentAuditStorage;

use beardog_errors::BearDogError;
use beardog_security::handlers::audit_management::AuditStatistics;
use std::sync::Arc;

#[derive(Debug, Clone)]
    pub user_id: Option<String>,
    pub key_id: Option<String>,
    /// The algorithm value
    pub algorithm: String,
    /// Number of data_size
    pub data_size: usize,
    /// Whether success is enabled
    pub success: bool,
    pub processing_time_ms: Option<u64>,
}

#[derive(Arc<PersistentAuditStorage>,}

impl DefaultAuditLogger {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new() -> Result<Self, BearDogError> {
        let storage_path = std::path::PathBuf::from("audit.log");
        Self::with_storage_path(storage_path)
    }

/// With Storage Path operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates instance with storage path
    pub fn with_storage_path(storage_path: std::path::PathBuf) -> Result<Self, BearDogError> {
        let storage = Arc::new(PersistentAuditStorage::new(&str,
        user_id: Option<&str>,
        key_id: Option<&str>,
        success: bool,
        error_message: Option<&str>,
        details: std::collections::HashMap<&str, &str>,
    ) -> Result<(), BearDogError> {
        let entry = AuditLogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            operation,
            key_id,
            actor: user_id.unwrap_or_else(|| "system".to_string()),
            result: if success {
                OperationResult::Success
            } else {
                OperationResult::Failure(details,
        };
        self.storage.append_entry(&str,
        user_id: Option<&str>,
        key_id: Option<&str>,
        self.log_detailed_operation(
            operation.to_string(),
            user_id.map(std::string::ToString::to_string),
            key_id.map(std::string::ToString::to_string),
            true,
            None,
            std::collections::HashMap::with_capacity(&str,
            false,
            Some(&str,
        key_type: &str,
        algorithm: &str,
        let mut details = std::collections::HashMap::with_capacity(Option<&str>,
            "key_deletion".to_string(),
            error_message.map(std::string::ToString::to_string),

/// Log Crypto Operation operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn log_crypto_operation(&self, operation: CryptoOperationLog) -> Result<(), BearDogError> {
        details.insert(&str,
        severity: str,
        description: &str,
        additional_info: std::collections::HashMap<&str, &str>,
        let mut details = additional_info;
        details.insert(chrono::DateTime<chrono::Utc>,
    ) -> Result<u64, BearDogError> {
        self.purge_entries_older_than(older_than)

/// Export Audit Log operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn export_audit_log(&self, format: &str) -> Result<Vec<u8>, BearDogError>> {
        match format.to_lowercase().as_str() {
            "json" => self.export_as_json(),
            "csv" => self.export_as_csv(),
            _ => Err(BearDogError::invalid_input(format!("Unsupported export format: {}format"),
            }),


    fn generate_statistics(24,
            total_events: 1000,
            auth_events: 300,
            authz_events: 200,
            high_risk_events: 50,
            success_rate: 0.95, // 95% success rate
        })


    fn purge_entries_older_than(chrono::DateTime<chrono::Utc>,

        Ok(10)


    fn export_as_json(&self) -> Result<Vec<u8>, BearDogError>> {
        let filter = AuditLogFilter::default();
        let entries = self.storage.get_entries(&filter)?;
        let json =
            serde_json::to_string_pretty(&entries).map_err(|e| BearDogError::Serialization {
                message: format!("Failed to serialize audit log to JSON: {e}"),
            })?;
        Ok(json.into_bytes())


    fn export_as_csv(&self) -> Result<Vec<u8>, BearDogError>> {
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
                    OperationResult::Success => String::with_capacity(64),
                    OperationResult::Failure(ref msg) => msg.clone(),
                }
            ));
        Ok(csv.into_bytes())

impl AuditLogger for DefaultAuditLogger {


    fn log_operation(&self, operation: &AuditLogEntry) -> Result<(), BearDogError> {
        self.storage.log_entry(&operation)

    /// Gets audit_log
    fn get_audit_log(&self, filter: &AuditLogFilter) -> Result<Vec<AuditLogEntry>, BearDogError>> {
        self.storage.get_entries(filter)
