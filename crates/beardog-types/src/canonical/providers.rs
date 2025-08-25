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


use serde::{Deserialize, Serialize};

/// **CANONICAL** Provider Types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderType {
    Security,
    Hsm,
    Storage,
    Network,
    Compute,
    AI,
    Monitoring,
    Compliance,
    Generic,
}
impl Default for ProviderType {
    fn default() -> Self {
        Self::Generic
    }
}

/// **CANONICAL** Provider Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderStatus {
    Active,
    Inactive,
    Degraded,
    Failed,
    Maintenance,
    Unknown,
}

impl Default for ProviderStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

/// **CANONICAL** Provider Health Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealth {
    pub status: ProviderStatus,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub error_count: u32,
    pub success_rate: f64,
}

impl Default for ProviderHealth {
    fn default() -> Self {
        Self {
            status: ProviderStatus::Unknown,
            last_check: chrono::Utc::now(),
            error_count: 0,
            success_rate: 0.0,
        }
    }
}

/// **PROVIDER CAPABILITY** - Capability descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapability {
    pub name: String,
    pub version: String,
    pub description: String,
}

/// **PROVIDER REGISTRY ENTRY** - Registry entry for providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRegistryEntry {
    pub provider_type: ProviderType,
    pub capabilities: Vec<ProviderCapability>,
}

/// **PROVIDER CONFIG** - Configuration for providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider_id: String,
    pub provider_type: ProviderType,
    pub endpoint: Option<String>,
    pub timeout: std::time::Duration,
    pub retry_attempts: u32,
    pub health_check_interval: std::time::Duration,
    pub metadata: std::collections::HashMap<String, String>,
    pub enabled: bool,
    pub priority: u32,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            provider_id: "default".to_string(),
            provider_type: ProviderType::Generic,
            endpoint: None,
            timeout: std::time::Duration::from_secs(30),
            retry_attempts: 3,
            health_check_interval: std::time::Duration::from_secs(60),
            metadata: std::collections::HashMap::new(),
            enabled: true,
            priority: 100,
        }
    }
}

/// **PROVIDER HEALTH STATUS** - Health status information for a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealthStatus {
    /// Whether the provider is healthy
    pub is_healthy: bool,
    /// Last health check timestamp
    pub last_check: chrono::DateTime<chrono::Utc>,
    /// Health check details
    pub details: Option<String>,
    /// Response time in milliseconds
    pub response_time_ms: Option<u64>,
}

impl Default for ProviderHealthStatus {
    fn default() -> Self {
        Self {
            is_healthy: false,
            last_check: chrono::Utc::now(),
            details: None,
            response_time_ms: None,
        }
    }
}

// ============================================================================
// HSM PROVIDER TRAITS - CANONICAL HSM INTERFACE
use super::hsm::KeyType;

/// **CANONICAL** HSM key information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKeyInfo {
    pub key_id: String,
    pub key_type: KeyType,
    pub metadata: super::hsm::KeyMetadata,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub usage_count: u64,
}

/// **CANONICAL** HSM hardware status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHardwareStatus {
    pub available: bool,
    pub temperature: Option<f64>,
    pub free_memory: Option<u64>,
    pub uptime_seconds: Option<u64>,
    pub error_count: u64,
}

/// **CANONICAL** General HSM information and capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmInfo {
    pub instance_id: String,
    pub vendor: String,
    pub model: String,
    pub firmware_version: String,
    pub api_version: String,
    pub supported_algorithms: Vec<String>,
    pub max_key_count: u32,
    pub current_key_count: u32,
    pub certification: Option<String>,
    pub tamper_resistant: bool,
}
