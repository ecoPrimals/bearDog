//! Universal NestGate Core Adapter
//!
//! Core NestGate adapter implementation using universal patterns that can be used
//! by any ecosystem component (BearDog, SongBird, ToadStool, biomeOS, etc.)

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use super::audit::AuditManager;
use super::policy::PolicyEngine;
use super::types::*;
use super::zfs::ZfsManager;

/// Universal NestGate adapter
///
/// The UniversalNestGateAdapter provides seamless integration with NestGate's secure
/// file transfer capabilities, enabling encrypted file operations, secure
/// sharing workflows, and audit trail integration for any primal provider.
pub struct UniversalNestGateAdapter {
    /// Provider name
    name: String,
    /// Primal provider
    provider: Arc<dyn PrimalProvider>,
    /// Adapter configuration
    config: NestGateConfig,
    /// Key mapping (NestGate ID -> Provider ID)
    key_mapping: Arc<RwLock<HashMap<String, String>>>,
    /// ZFS manager
    zfs_manager: Arc<ZfsManager>,
    /// Policy engine
    policy_engine: Arc<PolicyEngine>,
    /// Audit manager
    audit_manager: Arc<AuditManager>,
    /// Connection pool
    connection_pool: Arc<RwLock<ConnectionPool>>,
}

/// Connection pool for NestGate connections
#[derive(Debug)]
pub struct ConnectionPool {
    /// Active connections
    connections: HashMap<String, Connection>,
    /// Maximum connections
    max_connections: usize,
    /// Connection timeout in seconds
    timeout: u64,
}

/// NestGate connection
#[derive(Debug)]
pub struct Connection {
    /// Connection ID
    id: String,
    /// Connection status
    status: ConnectionStatus,
    /// Last activity timestamp
    last_activity: chrono::DateTime<chrono::Utc>,
    /// Connection metadata
    metadata: HashMap<String, String>,
}

/// Connection status
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    /// Connection is active
    Active,
    /// Connection is idle
    Idle,
    /// Connection is failed
    Failed,
    /// Connection is closed
    Closed,
}

impl UniversalNestGateAdapter {
    /// Create a new universal NestGate adapter
    pub async fn new(
        provider: Arc<dyn PrimalProvider>,
        config: NestGateConfig,
    ) -> NestGateResult<Self> {
        info!(
            "Creating universal NestGate adapter for provider: {}",
            provider.name()
        );

        // Validate configuration
        Self::validate_config(&config)?;

        // Create managers
        let zfs_manager = Arc::new(ZfsManager::new(config.zfs.clone()).await?);
        let policy_engine = Arc::new(PolicyEngine::new(config.policies.clone()).await?);
        let audit_manager = Arc::new(AuditManager::new(config.audit.clone()).await?);

        // Create connection pool
        let connection_pool = Arc::new(RwLock::new(ConnectionPool::new(10, 300)));

        let adapter = Self {
            name: format!("nestgate-{}", provider.name()),
            provider,
            config,
            key_mapping: Arc::new(RwLock::new(HashMap::new())),
            zfs_manager,
            policy_engine,
            audit_manager,
            connection_pool,
        };

        info!("Universal NestGate adapter created successfully");
        Ok(adapter)
    }

    /// Validate configuration
    fn validate_config(config: &NestGateConfig) -> NestGateResult<()> {
        if config.api_endpoint.is_empty() {
            return Err(NestGateError::Configuration(
                "API endpoint cannot be empty".to_string(),
            ));
        }

        if config.provider_name.is_empty() {
            return Err(NestGateError::Configuration(
                "Provider name cannot be empty".to_string(),
            ));
        }

        if config.auth.api_key.is_empty() && config.auth.method == AuthMethod::ApiKey {
            return Err(NestGateError::Configuration(
                "API key cannot be empty for API key authentication".to_string(),
            ));
        }

        Ok(())
    }

    /// Get adapter name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get provider
    pub fn provider(&self) -> &Arc<dyn PrimalProvider> {
        &self.provider
    }

    /// Get configuration
    pub fn config(&self) -> &NestGateConfig {
        &self.config
    }

    /// Generate master key for provider
    pub async fn generate_master_key(&self, owner_id: &str) -> NestGateResult<NestGateMasterKey> {
        info!(
            "Generating master key for owner: {} (provider: {})",
            owner_id,
            self.provider.name()
        );

        // Check policy
        let policy_result = self
            .policy_engine
            .check_access(owner_id, "master_key", &FileOperation::Write)
            .await?;

        if !policy_result.allowed {
            return Err(NestGateError::PolicyViolation(policy_result.reason));
        }

        // Generate master key using ZFS manager
        let master_key = self.zfs_manager.generate_master_key(owner_id).await?;

        // Store key mapping
        self.key_mapping
            .write()
            .await
            .insert(master_key.id.clone(), owner_id.to_string());

        // Log audit event
        self.audit_manager
            .log_event(NestGateAuditEvent {
                id: Uuid::new_v4().to_string(),
                event_type: "master_key_generated".to_string(),
                user_id: owner_id.to_string(),
                provider_id: self.provider.name().to_string(),
                resource: "master_key".to_string(),
                operation: "generate".to_string(),
                timestamp: chrono::Utc::now(),
                metadata: HashMap::new(),
                result: OperationResult::Success,
                severity: EventSeverity::Info,
            })
            .await?;

        info!("Master key generated successfully for owner: {}", owner_id);
        Ok(master_key)
    }

    /// Wrap key using master key
    pub async fn wrap_key(&self, key_data: &[u8], wrapping_key_id: &str) -> NestGateResult<WrappedKey> {
        debug!("Wrapping key with wrapping key ID: {}", wrapping_key_id);

        // Get wrapping key
        let wrapping_key = self.get_wrapping_key(wrapping_key_id).await?;

        // Wrap key using ZFS manager
        let wrapped_key = self.zfs_manager.wrap_key(key_data, &wrapping_key).await?;

        debug!("Key wrapped successfully");
        Ok(wrapped_key)
    }

    /// Unwrap key using master key
    pub async fn unwrap_key(
        &self,
        wrapped_key: &WrappedKey,
        wrapping_key_id: &str,
    ) -> NestGateResult<Vec<u8>> {
        debug!("Unwrapping key with wrapping key ID: {}", wrapping_key_id);

        // Get wrapping key
        let wrapping_key = self.get_wrapping_key(wrapping_key_id).await?;

        // Unwrap key using ZFS manager
        let unwrapped_key = self.zfs_manager.unwrap_key(wrapped_key, &wrapping_key).await?;

        debug!("Key unwrapped successfully");
        Ok(unwrapped_key)
    }

    /// Rotate keys for owner
    pub async fn rotate_keys(&self, owner_id: &str) -> NestGateResult<KeyRotationResult> {
        info!("Rotating keys for owner: {} (provider: {})", owner_id, self.provider.name());

        // Check policy
        let policy_result = self
            .policy_engine
            .check_access(owner_id, "key_rotation", &FileOperation::Write)
            .await?;

        if !policy_result.allowed {
            return Err(NestGateError::PolicyViolation(policy_result.reason));
        }

        // Perform key rotation using ZFS manager
        let rotation_result = self.zfs_manager.rotate_keys(owner_id).await?;

        // Log audit event
        self.audit_manager
            .log_event(NestGateAuditEvent {
                id: Uuid::new_v4().to_string(),
                event_type: "key_rotation_initiated".to_string(),
                user_id: owner_id.to_string(),
                provider_id: self.provider.name().to_string(),
                resource: "keys".to_string(),
                operation: "rotate".to_string(),
                timestamp: chrono::Utc::now(),
                metadata: HashMap::new(),
                result: OperationResult::Success,
                severity: EventSeverity::Info,
            })
            .await?;

        info!("Key rotation initiated for owner: {}", owner_id);
        Ok(rotation_result)
    }

    /// Perform file operation
    pub async fn perform_file_operation(
        &self,
        request: FileOperationRequest,
    ) -> NestGateResult<FileOperationResult> {
        info!(
            "Performing file operation: {:?} for user: {} (provider: {})",
            request.operation,
            request.user_id,
            self.provider.name()
        );

        // Check policy
        let policy_result = self
            .policy_engine
            .check_access(
                &request.user_id,
                &request.source_path.to_string_lossy(),
                &request.operation,
            )
            .await?;

        if !policy_result.allowed {
            let error_msg = format!("Policy violation: {}", policy_result.reason);
            
            // Log audit event for denied operation
            self.audit_manager
                .log_event(NestGateAuditEvent {
                    id: Uuid::new_v4().to_string(),
                    event_type: "file_operation_denied".to_string(),
                    user_id: request.user_id.clone(),
                    provider_id: self.provider.name().to_string(),
                    resource: request.source_path.to_string_lossy().to_string(),
                    operation: format!("{:?}", request.operation),
                    timestamp: chrono::Utc::now(),
                    metadata: request.metadata.clone(),
                    result: OperationResult::Denied {
                        reason: policy_result.reason.clone(),
                    },
                    severity: EventSeverity::Warning,
                })
                .await?;

            return Err(NestGateError::PolicyViolation(error_msg));
        }

        // Perform operation using ZFS manager
        let operation_result = self.zfs_manager.perform_file_operation(request.clone()).await?;

        // Log audit event
        self.audit_manager
            .log_event(NestGateAuditEvent {
                id: Uuid::new_v4().to_string(),
                event_type: "file_operation_completed".to_string(),
                user_id: request.user_id.clone(),
                provider_id: self.provider.name().to_string(),
                resource: request.source_path.to_string_lossy().to_string(),
                operation: format!("{:?}", request.operation),
                timestamp: chrono::Utc::now(),
                metadata: request.metadata.clone(),
                result: if operation_result.success {
                    OperationResult::Success
                } else {
                    OperationResult::Failed {
                        error: operation_result.error_message.clone().unwrap_or_default(),
                    }
                },
                severity: if operation_result.success {
                    EventSeverity::Info
                } else {
                    EventSeverity::Error
                },
            })
            .await?;

        info!("File operation completed successfully");
        Ok(operation_result)
    }

    /// Get audit trail
    pub async fn get_audit_trail(
        &self,
        filter: Option<&str>,
    ) -> NestGateResult<Vec<NestGateAuditEvent>> {
        debug!("Getting audit trail with filter: {:?}", filter);
        self.audit_manager.get_audit_trail(filter).await
    }

    /// Get policy status
    pub async fn get_policy_status(&self) -> NestGateResult<Vec<AccessPolicy>> {
        debug!("Getting policy status");
        self.policy_engine.get_all_policies().await
    }

    /// Health check
    pub async fn health_check(&self) -> NestGateResult<HealthStatus> {
        debug!("Performing health check");

        let mut components = HashMap::new();

        // Check provider health
        match self.provider.health_check().await {
            Ok(provider_health) => {
                components.insert("provider".to_string(), ComponentHealth {
                    healthy: provider_health.healthy,
                    status: provider_health.message,
                    metrics: HashMap::new(),
                });
            }
            Err(e) => {
                components.insert("provider".to_string(), ComponentHealth {
                    healthy: false,
                    status: format!("Provider health check failed: {}", e),
                    metrics: HashMap::new(),
                });
            }
        }

        // Check ZFS manager health
        match self.zfs_manager.health_check().await {
            Ok(zfs_health) => {
                components.insert("zfs".to_string(), ComponentHealth {
                    healthy: zfs_health.healthy,
                    status: zfs_health.message,
                    metrics: HashMap::new(),
                });
            }
            Err(e) => {
                components.insert("zfs".to_string(), ComponentHealth {
                    healthy: false,
                    status: format!("ZFS health check failed: {}", e),
                    metrics: HashMap::new(),
                });
            }
        }

        // Check policy engine health
        match self.policy_engine.health_check().await {
            Ok(policy_health) => {
                components.insert("policy".to_string(), ComponentHealth {
                    healthy: policy_health.healthy,
                    status: policy_health.message,
                    metrics: HashMap::new(),
                });
            }
            Err(e) => {
                components.insert("policy".to_string(), ComponentHealth {
                    healthy: false,
                    status: format!("Policy engine health check failed: {}", e),
                    metrics: HashMap::new(),
                });
            }
        }

        // Check audit manager health
        match self.audit_manager.health_check().await {
            Ok(audit_health) => {
                components.insert("audit".to_string(), ComponentHealth {
                    healthy: audit_health.healthy,
                    status: audit_health.message,
                    metrics: HashMap::new(),
                });
            }
            Err(e) => {
                components.insert("audit".to_string(), ComponentHealth {
                    healthy: false,
                    status: format!("Audit manager health check failed: {}", e),
                    metrics: HashMap::new(),
                });
            }
        }

        // Overall health status
        let overall_healthy = components.values().all(|c| c.healthy);

        Ok(HealthStatus {
            healthy: overall_healthy,
            message: if overall_healthy {
                "All components healthy".to_string()
            } else {
                "Some components unhealthy".to_string()
            },
            components,
            last_check: chrono::Utc::now(),
        })
    }

    /// Get provider status
    pub async fn get_status(&self) -> NestGateResult<HashMap<String, String>> {
        debug!("Getting adapter status");

        let mut status = HashMap::new();

        // Get provider health status
        match self.provider.health_check().await {
            Ok(health) => {
                status.insert("provider_status".to_string(), if health.healthy { "healthy" } else { "unhealthy" }.to_string());
                status.insert("provider_message".to_string(), health.message);
            }
            Err(e) => {
                status.insert("provider_status".to_string(), "unhealthy".to_string());
                status.insert("provider_error".to_string(), e.to_string());
            }
        }

        // Check NestGate-specific status
        status.insert("adapter_name".to_string(), self.name.clone());
        status.insert("provider_name".to_string(), self.provider.name().to_string());
        status.insert("configuration_valid".to_string(), "true".to_string());
        status.insert(
            "key_mapping_count".to_string(),
            self.key_mapping.read().await.len().to_string(),
        );

        // Check connection pool status
        let pool = self.connection_pool.read().await;
        status.insert("connection_pool_size".to_string(), pool.connections.len().to_string());
        status.insert("connection_pool_max".to_string(), pool.max_connections.to_string());

        Ok(status)
    }

    /// Get wrapping key
    async fn get_wrapping_key(&self, wrapping_key_id: &str) -> NestGateResult<Vec<u8>> {
        debug!("Getting wrapping key: {}", wrapping_key_id);

        // Check if key exists in mapping
        let key_mapping = self.key_mapping.read().await;
        if let Some(owner_id) = key_mapping.get(wrapping_key_id) {
            // Get key from ZFS manager
            self.zfs_manager.get_key(wrapping_key_id, owner_id).await
        } else {
            Err(NestGateError::KeyManagement(format!(
                "Wrapping key not found: {}",
                wrapping_key_id
            )))
        }
    }

    /// Generate encryption key
    pub async fn generate_key(
        &self,
        key_type: &str,
        purpose: &str,
        owner_id: &str,
    ) -> NestGateResult<EncryptionKey> {
        info!("Generating encryption key for owner: {} (purpose: {})", owner_id, purpose);

        // Check policy
        let policy_result = self
            .policy_engine
            .check_access(owner_id, "encryption_key", &FileOperation::Write)
            .await?;

        if !policy_result.allowed {
            return Err(NestGateError::PolicyViolation(policy_result.reason));
        }

        // Generate key using ZFS manager
        let encryption_key = self.zfs_manager.generate_encryption_key(key_type, purpose, owner_id).await?;

        // Log audit event
        self.audit_manager
            .log_event(NestGateAuditEvent {
                id: Uuid::new_v4().to_string(),
                event_type: "encryption_key_generated".to_string(),
                user_id: owner_id.to_string(),
                provider_id: self.provider.name().to_string(),
                resource: "encryption_key".to_string(),
                operation: "generate".to_string(),
                timestamp: chrono::Utc::now(),
                metadata: HashMap::new(),
                result: OperationResult::Success,
                severity: EventSeverity::Info,
            })
            .await?;

        info!("Encryption key generated successfully");
        Ok(encryption_key)
    }
}

impl ConnectionPool {
    /// Create new connection pool
    pub fn new(max_connections: usize, timeout: u64) -> Self {
        Self {
            connections: HashMap::new(),
            max_connections,
            timeout,
        }
    }

    /// Get connection
    pub fn get_connection(&mut self, id: &str) -> Option<&mut Connection> {
        self.connections.get_mut(id)
    }

    /// Add connection
    pub fn add_connection(&mut self, connection: Connection) -> Result<(), String> {
        if self.connections.len() >= self.max_connections {
            return Err("Connection pool full".to_string());
        }

        self.connections.insert(connection.id.clone(), connection);
        Ok(())
    }

    /// Remove connection
    pub fn remove_connection(&mut self, id: &str) -> Option<Connection> {
        self.connections.remove(id)
    }

    /// Clean up expired connections
    pub fn cleanup_expired(&mut self) {
        let now = chrono::Utc::now();
        let timeout_duration = chrono::Duration::seconds(self.timeout as i64);

        self.connections.retain(|_, conn| {
            now.signed_duration_since(conn.last_activity) < timeout_duration
        });
    }
}

impl Connection {
    /// Create new connection
    pub fn new(id: String) -> Self {
        Self {
            id,
            status: ConnectionStatus::Active,
            last_activity: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Update last activity
    pub fn update_activity(&mut self) {
        self.last_activity = chrono::Utc::now();
    }

    /// Set status
    pub fn set_status(&mut self, status: ConnectionStatus) {
        self.status = status;
    }
} 