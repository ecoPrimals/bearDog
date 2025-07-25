//! Core Types for BearDog
//!
//! This module defines the core types used throughout the BearDog system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Local type definitions to avoid circular dependencies with beardog-tunnel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KeyType {
    Ed25519,
    EccP256,
    Rsa2048,
    Rsa4096,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HsmTier {
    Hardware,
    Software,
    SmartCard,
    CloudHsm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHealthStatus {
    pub is_healthy: bool,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub error_message: Option<String>,
}

// Core system types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub is_healthy: bool,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentStatus {
    Running,
    Stopped,
    Error(String),
    Starting,
    Stopping,
}

impl ComponentStatus {
    pub fn healthy(&self) -> bool {
        matches!(self, ComponentStatus::Running)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreState {
    pub components: HashMap<String, ComponentStatus>,
    pub overall_health: HealthStatus,
    pub startup_time: chrono::DateTime<chrono::Utc>,
    // Add fields that are being accessed in the code
    pub health_status: HealthStatus,
    pub component_status: HashMap<String, ComponentStatus>,
    pub start_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub component_name: String,
    pub status: ComponentStatus,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub details: HashMap<String, String>,
    pub uptime: Option<chrono::Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_activity: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmCapabilities {
    pub supported_key_types: Vec<KeyType>,
    pub max_key_size: u32,
    pub supports_attestation: bool,
    pub hardware_backed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKey {
    pub id: String,
    pub key_type: KeyType,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

// Capability types
#[derive(Debug, Clone)]
pub struct HsmCapability {
    pub name: String,
    pub supported: bool,
    pub metadata: HashMap<String, String>,
}

// Additional missing types commonly referenced
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmProvider {
    pub id: String,
    pub name: String,
    pub tier: HsmTier,
    pub capabilities: HsmCapabilities,
}

// Default implementations
impl Default for HsmHealthStatus {
    fn default() -> Self {
        Self {
            is_healthy: true,
            last_check: chrono::Utc::now(),
            error_message: None,
        }
    }
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self {
            is_healthy: true,
            last_check: chrono::Utc::now(),
            error_message: None,
        }
    }
}

impl Default for ComponentStatus {
    fn default() -> Self {
        ComponentStatus::Stopped
    }
}

impl Default for CoreState {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            components: HashMap::new(),
            overall_health: HealthStatus::default(),
            startup_time: now,
            health_status: HealthStatus::default(),
            component_status: HashMap::new(),
            start_time: now,
        }
    }
}

impl HsmHealthStatus {
    pub fn healthy() -> Self {
        Self::default()
    }

    pub fn unhealthy(error: String) -> Self {
        Self {
            is_healthy: false,
            last_check: chrono::Utc::now(),
            error_message: Some(error),
        }
    }
}

impl HealthStatus {
    pub fn healthy() -> Self {
        Self::default()
    }

    pub fn unhealthy(error: String) -> Self {
        Self {
            is_healthy: false,
            last_check: chrono::Utc::now(),
            error_message: Some(error),
        }
    }

    pub fn Healthy() -> Self {
        Self::healthy()
    }
}

impl Default for HsmCapabilities {
    fn default() -> Self {
        Self {
            supported_key_types: vec![KeyType::Ed25519, KeyType::EccP256],
            max_key_size: 4096,
            supports_attestation: false,
            hardware_backed: false,
        }
    }
}

// Placeholder types for missing dependencies
#[derive(Debug, Clone)]
pub struct SystemMonitor {
    pub active: bool,
}

impl SystemMonitor {
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self { active: true })
    }

    pub async fn start(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct GeneticOptimizer {
    pub active: bool,
}

impl GeneticOptimizer {
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self { active: true })
    }

    pub async fn initialize(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}

// Add placeholder for BearDogSecurityProvider since we're using it in core/mod.rs
#[derive(Debug)]
pub struct BearDogSecurityProvider {
    pub active: bool,
}

impl BearDogSecurityProvider {
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self { active: true })
    }

    pub async fn initialize(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}

// Re-export BearDogError for the placeholder implementations
use beardog_errors::BearDogError;
