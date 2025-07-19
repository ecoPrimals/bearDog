//! Environment Configuration
//!
//! This module defines environment configurations for production deployments,
//! including resource allocation, CPU, memory, and storage configurations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Deployment environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentEnvironment {
    /// Environment name
    pub name: String,
    /// Environment type
    pub environment_type: EnvironmentType,
    /// Environment variables
    pub environment_variables: HashMap<String, String>,
    /// Resource limits
    pub resource_limits: ResourceLimits,
}

/// Environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Environment name
    pub name: String,
    /// Environment type
    pub env_type: EnvironmentType,
    /// Resource allocation
    pub resources: ResourceAllocation,
    /// Environment variables
    pub environment_variables: HashMap<String, String>,
}

/// Environment type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentType {
    /// Development environment
    Development,
    /// Staging environment
    Staging,
    /// Production environment
    Production,
    /// Disaster recovery environment
    DisasterRecovery,
}

/// Resource allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    /// CPU allocation
    pub cpu: CpuAllocation,
    /// Memory allocation
    pub memory: MemoryAllocation,
    /// Storage allocation
    pub storage: StorageAllocation,
    /// Network allocation
    pub network: NetworkAllocation,
}

/// Resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// CPU limit
    pub cpu_limit: f64,
    /// Memory limit
    pub memory_limit: f64,
    /// Storage limit
    pub storage_limit: f64,
    /// Network limit
    pub network_limit: f64,
}

/// CPU allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuAllocation {
    /// Number of cores
    pub cores: u32,
    /// CPU priority
    pub priority: CpuPriority,
    /// CPU affinity
    pub affinity: Vec<u32>,
}

/// CPU priority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CpuPriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Real-time priority
    RealTime,
}

/// Memory allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAllocation {
    /// Memory size in MB
    pub size_mb: u64,
    /// Swap configuration
    pub swap: SwapConfig,
    /// Memory optimization
    pub optimization: MemoryOptimization,
}

/// Swap configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapConfig {
    /// Enable swap
    pub enabled: bool,
    /// Swap size in MB
    pub size_mb: u64,
    /// Swap priority
    pub priority: i32,
}

/// Memory optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryOptimization {
    /// Enable memory compression
    pub compression: bool,
    /// Overcommit policy
    pub overcommit: OvercommitPolicy,
}

/// Overcommit policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OvercommitPolicy {
    /// Never overcommit
    Never,
    /// Always overcommit
    Always,
    /// Guess overcommit
    Guess,
}

/// Storage allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAllocation {
    /// Storage size in GB
    pub size_gb: u64,
    /// Storage type
    pub storage_type: StorageType,
    /// IOPS limit
    pub iops_limit: u32,
}

/// Storage type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    /// SSD storage
    Ssd,
    /// HDD storage
    Hdd,
    /// NVMe storage
    NVMe,
    /// Network storage
    Network,
}

/// Network allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAllocation {
    /// Bandwidth limit in Mbps
    pub bandwidth_mbps: u32,
    /// Quality of service
    pub qos: NetworkQoS,
}

/// Network quality of service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkQoS {
    /// Best effort
    BestEffort,
    /// Guaranteed
    Guaranteed,
    /// Expedited
    Expedited,
}

impl Default for DeploymentEnvironment {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            environment_type: EnvironmentType::Development,
            environment_variables: HashMap::new(),
            resource_limits: ResourceLimits::default(),
        }
    }
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            env_type: EnvironmentType::Development,
            resources: ResourceAllocation::default(),
            environment_variables: HashMap::new(),
        }
    }
}

impl Default for ResourceAllocation {
    fn default() -> Self {
        Self {
            cpu: CpuAllocation::default(),
            memory: MemoryAllocation::default(),
            storage: StorageAllocation::default(),
            network: NetworkAllocation::default(),
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            cpu_limit: 80.0,
            memory_limit: 80.0,
            storage_limit: 80.0,
            network_limit: 80.0,
        }
    }
}

impl Default for CpuAllocation {
    fn default() -> Self {
        Self {
            cores: 2,
            priority: CpuPriority::Normal,
            affinity: Vec::new(),
        }
    }
}

impl Default for MemoryAllocation {
    fn default() -> Self {
        Self {
            size_mb: 4096,
            swap: SwapConfig::default(),
            optimization: MemoryOptimization::default(),
        }
    }
}

impl Default for SwapConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            size_mb: 2048,
            priority: 0,
        }
    }
}

impl Default for MemoryOptimization {
    fn default() -> Self {
        Self {
            compression: false,
            overcommit: OvercommitPolicy::Guess,
        }
    }
}

impl Default for StorageAllocation {
    fn default() -> Self {
        Self {
            size_gb: 100,
            storage_type: StorageType::Ssd,
            iops_limit: 3000,
        }
    }
}

impl Default for NetworkAllocation {
    fn default() -> Self {
        Self {
            bandwidth_mbps: 1000,
            qos: NetworkQoS::BestEffort,
        }
    }
}

impl DeploymentEnvironment {
    /// Create production environment configuration
    pub fn production() -> Self {
        Self {
            name: "production".to_string(),
            environment_type: EnvironmentType::Production,
            environment_variables: HashMap::new(),
            resource_limits: ResourceLimits {
                cpu_limit: 90.0,
                memory_limit: 90.0,
                storage_limit: 90.0,
                network_limit: 90.0,
            },
        }
    }

    /// Create development environment configuration
    pub fn development() -> Self {
        Self {
            name: "development".to_string(),
            environment_type: EnvironmentType::Development,
            environment_variables: HashMap::new(),
            resource_limits: ResourceLimits {
                cpu_limit: 50.0,
                memory_limit: 50.0,
                storage_limit: 50.0,
                network_limit: 50.0,
            },
        }
    }
}
