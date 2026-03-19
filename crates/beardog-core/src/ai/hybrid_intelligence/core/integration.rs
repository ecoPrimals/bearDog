// SPDX-License-Identifier: AGPL-3.0-only

//! # External System Integration
//!
//! This module handles integration with external systems, data exchange protocols,
//! and system connectivity for the hybrid intelligence system.

use super::super::config::HybridIntelligenceConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// External system integration configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntegrationConfig {
    /// Enable external integrations
    pub enabled: bool,
    /// Integration endpoints
    pub endpoints: Vec<String>,
    /// Authentication configuration
    pub auth: HashMap<String, String>,
    /// Connection timeout in seconds
    pub timeout_seconds: u64,
}

/// System connector for external integrations
pub struct SystemConnector {
    /// Connection configuration
    config: IntegrationConfig,
    /// Active connections
    connections: HashMap<String, ConnectionInfo>,
}

/// Connection information
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    /// Connection ID
    pub id: String,
    /// Connection status
    pub status: ConnectionStatus,
    /// Last activity timestamp
    pub last_activity: std::time::SystemTime,
}

/// Connection status enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionStatus {
    /// Connection is active
    Connected,
    /// Connection is disconnected
    Disconnected,
    /// Connection is in error state
    Error,
    /// Connection is being established
    Connecting,
}

/// Data exchange protocol
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DataExchangeProtocol {
    /// HTTP/REST protocol
    Http,
    /// gRPC protocol
    Grpc,
    /// WebSocket protocol
    WebSocket,
    /// Message queue protocol
    MessageQueue,
    /// Custom protocol
    Custom,
}

/// Integration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationStatus {
    /// Total active integrations
    pub active_integrations: usize,
    /// Total data exchanges
    pub total_exchanges: u64,
    /// Last exchange timestamp
    pub last_exchange: Option<std::time::SystemTime>,
    /// Integration health
    pub health: IntegrationHealth,
}

/// Integration health status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum IntegrationHealth {
    /// All integrations healthy
    Healthy,
    /// Some integrations degraded
    Degraded,
    /// Critical integration failures
    Critical,
}

/// Integration metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationMetrics {
    /// Total connections
    pub total_connections: usize,
    /// Active connections
    pub active_connections: usize,
    /// Failed connections
    pub failed_connections: usize,
    /// Average response time
    pub avg_response_time_ms: f64,
    /// Data throughput
    pub throughput_bps: f64,
}

/// **EXTERNAL SYSTEM INTEGRATION** - Manages external integrations
pub struct ExternalSystemIntegration {
    /// Configuration
    config: HybridIntelligenceConfig,
    
    /// Integration configuration
    integration_config: IntegrationConfig,
    
    /// System connectors
    connectors: HashMap<String, SystemConnector>,
    
    /// Integration metrics
    metrics: IntegrationMetrics,
}

/// **INTEGRATION MANAGER** - Coordinates all integrations
pub struct IntegrationManager {
    /// External system integration
    external_systems: ExternalSystemIntegration,
    
    /// Integration status
    status: IntegrationStatus,
}

impl IntegrationManager {
    /// Create a new integration manager
    #[must_use]
    pub fn new(config: HybridIntelligenceConfig) -> Self {
        Self {
            external_systems: ExternalSystemIntegration::new(config),
            status: IntegrationStatus::default(),
        }
    }
    
    /// Get integration metrics
    pub async fn get_metrics(&self) -> Result<IntegrationMetrics, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.external_systems.metrics.clone())
    }
}

impl ExternalSystemIntegration {
    #[must_use]
    /// Create a new external system integration
    pub fn new(config: HybridIntelligenceConfig) -> Self {
        Self {
            config,
            integration_config: IntegrationConfig::default(),
            connectors: HashMap::new(),
            metrics: IntegrationMetrics::default(),
        }
    }
}

#[must_use]
impl SystemConnector {
    /// Create a new system connector
    pub fn new(config: IntegrationConfig) -> Self {
        Self {
            config,
            connections: HashMap::new(),
        }
    }
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        // Use config-driven endpoint with proper hierarchy
        use beardog_config::global::BEARDOG_CONFIG;
        let default_endpoint = std::env::var("BEARDOG_AI_INTEGRATION_ENDPOINT")
            .unwrap_or_else(|_| {
                format!(
                    "http://{}:{}",
                    BEARDOG_CONFIG.network.addresses.api_host,
                    BEARDOG_CONFIG.network.ports.api_port
                )
            });
        
        Self {
            enabled: true,
            endpoints: vec![default_endpoint],
            auth: HashMap::new(),
            timeout_seconds: 30,
        }
    }
}

impl Default for IntegrationStatus {
    fn default() -> Self {
        Self {
            active_integrations: 0,
            total_exchanges: 0,
            last_exchange: None,
            health: IntegrationHealth::Healthy,
        }
    }
}

impl Default for IntegrationMetrics {
    fn default() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            failed_connections: 0,
            avg_response_time_ms: 0.0,
            throughput_bps: 0.0,
        }
    }
} 