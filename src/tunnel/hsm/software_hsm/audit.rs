//! # Software HSM Audit Logging
//!
//! This module provides comprehensive audit logging functionality for the Software HSM.
//! It tracks all operations, maintains audit trails, and provides query capabilities.

use super::types::*;
use crate::error::{BearDogError, BearDogResult};
use async_trait::async_trait;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

impl DefaultAuditLogger {
    /// Create a new default audit logger
    pub async fn new() -> BearDogResult<Self> {
        info!("Creating default audit logger");
        Ok(Self)
    }

    /// Log operation with detailed information
    async fn log_detailed_operation(
        &self,
        operation: &str,
        result: &str,
        details: &std::collections::HashMap<String, String>,
    ) -> BearDogResult<()> {
        let entry = AuditLogEntry {
            timestamp: chrono::Utc::now(),
            operation: operation.to_string(),
            key_id: details.get("key_id").cloned(),
            user_id: details.get("user_id").cloned(),
            result: result.to_string(),
            details: details.clone(),
        };

        self.log_operation(&entry).await
    }

    /// Log successful operation
    pub async fn log_success(
        &self,
        operation: &str,
        key_id: Option<String>,
        user_id: Option<String>,
    ) -> BearDogResult<()> {
        let entry = AuditLogEntry::success(operation.to_string(), key_id, user_id);
        self.log_operation(&entry).await
    }

    /// Log failed operation
    pub async fn log_failure(
        &self,
        operation: &str,
        key_id: Option<String>,
        user_id: Option<String>,
        error: String,
    ) -> BearDogResult<()> {
        let entry = AuditLogEntry::failure(operation.to_string(), key_id, user_id, error);
        self.log_operation(&entry).await
    }

    /// Log key generation
    pub async fn log_key_generation(
        &self,
        key_id: &str,
        key_type: &str,
        user_id: Option<String>,
    ) -> BearDogResult<()> {
        let mut details = std::collections::HashMap::new();
        details.insert("key_type".to_string(), key_type.to_string());
        
        let entry = AuditLogEntry::new(
            "key_generation".to_string(),
            Some(key_id.to_string()),
            user_id,
            "success".to_string(),
            details,
        );
        
        self.log_operation(&entry).await
    }

    /// Log key deletion
    pub async fn log_key_deletion(
        &self,
        key_id: &str,
        user_id: Option<String>,
    ) -> BearDogResult<()> {
        let entry = AuditLogEntry::success(
            "key_deletion".to_string(),
            Some(key_id.to_string()),
            user_id,
        );
        
        self.log_operation(&entry).await
    }

    /// Log cryptographic operation
    pub async fn log_crypto_operation(
        &self,
        operation: &str,
        key_id: &str,
        data_size: usize,
        user_id: Option<String>,
    ) -> BearDogResult<()> {
        let mut details = std::collections::HashMap::new();
        details.insert("data_size".to_string(), data_size.to_string());
        
        let entry = AuditLogEntry::new(
            format!("crypto_{}", operation),
            Some(key_id.to_string()),
            user_id,
            "success".to_string(),
            details,
        );
        
        self.log_operation(&entry).await
    }

    /// Log security event
    pub async fn log_security_event(
        &self,
        event_type: &str,
        severity: &str,
        description: &str,
        user_id: Option<String>,
    ) -> BearDogResult<()> {
        let mut details = std::collections::HashMap::new();
        details.insert("event_type".to_string(), event_type.to_string());
        details.insert("severity".to_string(), severity.to_string());
        details.insert("description".to_string(), description.to_string());
        
        let entry = AuditLogEntry::new(
            "security_event".to_string(),
            None,
            user_id,
            "alert".to_string(),
            details,
        );
        
        self.log_operation(&entry).await
    }

    /// Get audit statistics
    pub async fn get_audit_statistics(&self) -> BearDogResult<AuditStatistics> {
        // TODO: Implement actual statistics collection
        // For now, return placeholder statistics
        Ok(AuditStatistics {
            total_entries: 0,
            entries_by_operation: std::collections::HashMap::new(),
            entries_by_result: std::collections::HashMap::new(),
            entries_by_user: std::collections::HashMap::new(),
            oldest_entry: None,
            newest_entry: None,
        })
    }

    /// Purge old audit entries
    pub async fn purge_old_entries(&self, older_than: chrono::DateTime<chrono::Utc>) -> BearDogResult<usize> {
        info!("Purging audit entries older than {}", older_than);
        // TODO: Implement actual purging logic
        // For now, return 0 as no entries were purged
        Ok(0)
    }

    /// Export audit log
    pub async fn export_audit_log(&self, format: &str) -> BearDogResult<Vec<u8>> {
        info!("Exporting audit log in format: {}", format);
        
        // Get all audit entries
        let filter = AuditLogFilter::new();
        let entries = self.get_audit_log(&filter).await?;
        
        match format {
            "json" => {
                let json = serde_json::to_string_pretty(&entries).map_err(|e| {
                    BearDogError::SerializationError {
                        error: e.to_string(),
                    }
                })?;
                Ok(json.into_bytes())
            }
            "csv" => {
                let mut csv = String::new();
                csv.push_str("timestamp,operation,key_id,user_id,result,details\n");
                
                for entry in entries {
                    let details_json = serde_json::to_string(&entry.details).unwrap_or_default();
                    csv.push_str(&format!(
                        "{},{},{},{},{},\"{}\"\n",
                        entry.timestamp.to_rfc3339(),
                        entry.operation,
                        entry.key_id.unwrap_or_default(),
                        entry.user_id.unwrap_or_default(),
                        entry.result,
                        details_json
                    ));
                }
                
                Ok(csv.into_bytes())
            }
            _ => Err(BearDogError::UnsupportedFormat {
                format: format.to_string(),
            }),
        }
    }
}

#[async_trait]
impl AuditLogger for DefaultAuditLogger {
    /// Log HSM operation
    async fn log_operation(&self, entry: &AuditLogEntry) -> BearDogResult<()> {
        // TODO: Implement actual logging to persistent storage
        // For now, just log to tracing
        info!(
            "AUDIT: {} - {} - {} - {:?}",
            entry.timestamp.to_rfc3339(),
            entry.operation,
            entry.result,
            entry.details
        );
        
        if entry.result == "failure" || entry.result == "error" {
            warn!(
                "AUDIT FAILURE: {} failed for key_id={:?} user_id={:?}",
                entry.operation, entry.key_id, entry.user_id
            );
        }
        
        Ok(())
    }

    /// Get audit log entries
    async fn get_audit_log(&self, filter: &AuditLogFilter) -> BearDogResult<Vec<AuditLogEntry>> {
        debug!("Getting audit log with filter: {:?}", filter);
        
        // TODO: Implement actual filtering and retrieval from storage
        // For now, return empty vector
        Ok(vec![])
    }
}

/// In-memory audit logger for testing and development
pub struct InMemoryAuditLogger {
    entries: Arc<RwLock<VecDeque<AuditLogEntry>>>,
    max_entries: usize,
}

impl InMemoryAuditLogger {
    /// Create a new in-memory audit logger
    pub async fn new(max_entries: usize) -> BearDogResult<Self> {
        info!("Creating in-memory audit logger with max {} entries", max_entries);
        
        Ok(Self {
            entries: Arc::new(RwLock::new(VecDeque::new())),
            max_entries,
        })
    }

    /// Get current entry count
    pub async fn get_entry_count(&self) -> usize {
        let entries = self.entries.read().await;
        entries.len()
    }

    /// Clear all entries
    pub async fn clear(&self) -> BearDogResult<()> {
        let mut entries = self.entries.write().await;
        entries.clear();
        info!("Cleared all audit log entries");
        Ok(())
    }

    /// Get all entries
    pub async fn get_all_entries(&self) -> BearDogResult<Vec<AuditLogEntry>> {
        let entries = self.entries.read().await;
        Ok(entries.iter().cloned().collect())
    }
}

#[async_trait]
impl AuditLogger for InMemoryAuditLogger {
    /// Log HSM operation
    async fn log_operation(&self, entry: &AuditLogEntry) -> BearDogResult<()> {
        let mut entries = self.entries.write().await;
        
        // Add new entry
        entries.push_back(entry.clone());
        
        // Remove old entries if we exceed max
        while entries.len() > self.max_entries {
            entries.pop_front();
        }
        
        info!(
            "AUDIT: {} - {} - {} - {:?}",
            entry.timestamp.to_rfc3339(),
            entry.operation,
            entry.result,
            entry.details
        );
        
        Ok(())
    }

    /// Get audit log entries
    async fn get_audit_log(&self, filter: &AuditLogFilter) -> BearDogResult<Vec<AuditLogEntry>> {
        let entries = self.entries.read().await;
        
        let filtered: Vec<AuditLogEntry> = entries
            .iter()
            .filter(|entry| {
                // Apply time filter
                if let Some(start_time) = filter.start_time {
                    if entry.timestamp < start_time {
                        return false;
                    }
                }
                if let Some(end_time) = filter.end_time {
                    if entry.timestamp > end_time {
                        return false;
                    }
                }
                
                // Apply operation filter
                if let Some(ref operation) = filter.operation {
                    if entry.operation != *operation {
                        return false;
                    }
                }
                
                // Apply key_id filter
                if let Some(ref key_id) = filter.key_id {
                    match &entry.key_id {
                        Some(entry_key_id) => {
                            if entry_key_id != key_id {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
                
                // Apply user_id filter
                if let Some(ref user_id) = filter.user_id {
                    match &entry.user_id {
                        Some(entry_user_id) => {
                            if entry_user_id != user_id {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
                
                true
            })
            .cloned()
            .collect();
        
        Ok(filtered)
    }
}

/// Audit statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditStatistics {
    pub total_entries: usize,
    pub entries_by_operation: std::collections::HashMap<String, usize>,
    pub entries_by_result: std::collections::HashMap<String, usize>,
    pub entries_by_user: std::collections::HashMap<String, usize>,
    pub oldest_entry: Option<chrono::DateTime<chrono::Utc>>,
    pub newest_entry: Option<chrono::DateTime<chrono::Utc>>,
}

/// Create audit logger factory
pub async fn create_audit_logger(logger_type: &str) -> BearDogResult<Box<dyn AuditLogger>> {
    match logger_type {
        "default" => {
            let logger = DefaultAuditLogger::new().await?;
            Ok(Box::new(logger))
        }
        "memory" => {
            let logger = InMemoryAuditLogger::new(10000).await?;
            Ok(Box::new(logger))
        }
        _ => Err(BearDogError::UnsupportedLoggerType {
            logger_type: logger_type.to_string(),
        }),
    }
} 