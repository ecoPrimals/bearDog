// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unified Service Definitions for BearDog
//!
//! This module provides the single source of truth for all service-related types across
//! the BearDog ecosystem. It consolidates and unifies types that were previously
//! scattered across multiple modules and crates.
//!
//! ## Consolidation Strategy
//!
//! This module unifies and replaces:
//! - `beardog-types/src/services/mod.rs` - Original service definitions
//! - `beardog-core/src/ecosystem/primal_types.rs` - Ecosystem service types
//! - Scattered service metadata definitions across crates
//!
//! ## Modern Architecture Principles
//!
//! - **Single Source of Truth**: All service types in one canonical location
//! - **Capability-Based Design**: Services defined by what they can do, not what they are
//! - **Zero Hardcoding**: No vendor or primal-specific assumptions
//! - **Type Safety**: Strongly typed with comprehensive error handling
//! - **Performance Optimized**: Zero-cost abstractions with efficient serialization

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

// Re-export capability types from canonical location
pub use crate::canonical::capabilities::{CapabilityType, ServiceCapabilityType};

// Modular structure for better maintainability
pub mod unified;
pub mod endpoints;
pub mod metadata;
pub mod health;

// Re-export all unified types
pub use unified::*;
pub use endpoints::*;
pub use metadata::*;
pub use health::*;

/// Unified service definition that replaces all scattered service types
///
/// This is the canonical service definition that consolidates:
/// - `ServiceDefinition` from `beardog-types/src/services/`
/// - `ServiceMetadata` from `beardog-core/src/ecosystem/primal_types.rs`
/// - Various service-related types across the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedServiceDefinition {
    /// Unique service identifier
    pub id: Uuid,
    
    /// Human-readable service name
    pub name: String,
    
    /// Detailed service description
    pub description: String,
    
    /// Service version identifier
    pub version: String,
    
    /// List of capabilities this service provides
    pub capabilities: Vec<UnifiedCapability>,
    
    /// Available service endpoints
    pub endpoints: Vec<UnifiedServiceEndpoint>,
    
    /// Current health status of the service
    pub health_status: UnifiedHealthStatus,
    
    /// Service category classification
    pub category: ServiceCategory,
    
    /// Security domain classification
    pub security_domain: SecurityDomain,
    
    /// Service dependencies (capability-based, not hardcoded)
    pub dependencies: Vec<ServiceDependency>,
    
    /// Timestamp when service was registered
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when service was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Additional metadata key-value pairs
    pub metadata: BTreeMap<String, String>,
}

/// Unified capability definition that replaces scattered capability types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCapability {
    /// Capability type (from canonical capability system)
    pub capability_type: CapabilityType,
    
    /// Capability name identifier
    pub name: String,
    
    /// Detailed capability description
    pub description: String,
    
    /// Required permissions to use this capability
    pub required_permissions: Vec<String>,
    
    /// Capability-specific configuration
    pub configuration: BTreeMap<String, serde_json::Value>,
    
    /// Performance characteristics
    pub performance_metrics: Option<CapabilityPerformanceMetrics>,
}

/// Service dependency based on capabilities, not hardcoded references
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServiceDependency {
    /// Required dependency - service cannot function without this capability
    Required {
        /// Required capability type
        capability: CapabilityType,
        /// Minimum version required
        min_version: String,
        /// Reason this dependency is required
        reason: String,
        /// Optional specific configuration requirements
        requirements: Option<BTreeMap<String, serde_json::Value>>,
    },
    /// Optional dependency - service can function without this but benefits from it
    Optional {
        /// Optional capability type
        capability: CapabilityType,
        /// Minimum version preferred
        min_version: String,
        /// Reason this dependency is beneficial
        reason: String,
        /// Fallback behavior when dependency is unavailable
        fallback_strategy: Option<String>,
    },
}

/// Service category for classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceCategory {
    /// Core infrastructure service
    Infrastructure,
    /// Security and authentication service
    Security,
    /// Data storage and management service
    Storage,
    /// Compute and processing service
    Compute,
    /// Network and communication service
    Network,
    /// Monitoring and observability service
    Monitoring,
    /// Application-level service
    Application,
    /// Custom category
    Custom(String),
}

/// Security domain classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityDomain {
    /// Public services accessible to all
    Public,
    /// Internal services within the ecosystem
    Internal,
    /// Restricted services requiring special authorization
    Restricted,
    /// Highly sensitive services with maximum security
    Confidential,
    /// Custom security domain
    Custom(String),
}

/// Performance metrics for capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityPerformanceMetrics {
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    
    /// Requests per second capacity
    pub requests_per_second: f64,
    
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    
    /// Resource utilization metrics
    pub resource_utilization: ResourceUtilization,
}

/// Resource utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
    /// CPU utilization percentage (0.0 to 100.0)
    pub cpu_percent: f64,
    
    /// Memory utilization percentage (0.0 to 100.0)
    pub memory_percent: f64,
    
    /// Network bandwidth utilization in Mbps
    pub network_mbps: f64,
    
    /// Storage I/O operations per second
    pub storage_iops: f64,
}

impl Default for UnifiedServiceDefinition {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            description: String::new(),
            version: "1.0.0".to_string(),
            capabilities: Vec::new(),
            endpoints: Vec::new(),
            health_status: UnifiedHealthStatus::Unknown,
            category: ServiceCategory::Application,
            security_domain: SecurityDomain::Internal,
            dependencies: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: BTreeMap::new(),
        }
    }
}

impl UnifiedServiceDefinition {
    /// Create a new service definition with basic information
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            updated_at: Utc::now(),
            ..Default::default()
        }
    }
    
    /// Add a capability to this service
    pub fn add_capability(&mut self, capability: UnifiedCapability) -> &mut Self {
        self.capabilities.push(capability);
        self.updated_at = Utc::now();
        self
    }
    
    /// Add an endpoint to this service
    pub fn add_endpoint(&mut self, endpoint: UnifiedServiceEndpoint) -> &mut Self {
        self.endpoints.push(endpoint);
        self.updated_at = Utc::now();
        self
    }
    
    /// Add a dependency to this service
    pub fn add_dependency(&mut self, dependency: ServiceDependency) -> &mut Self {
        self.dependencies.push(dependency);
        self.updated_at = Utc::now();
        self
    }
    
    /// Check if this service provides a specific capability type
    pub fn provides_capability(&self, capability_type: &CapabilityType) -> bool {
        self.capabilities
            .iter()
            .any(|cap| cap.capability_type == *capability_type)
    }
    
    /// Get all capabilities of a specific type
    pub fn get_capabilities_of_type(&self, capability_type: &CapabilityType) -> Vec<&UnifiedCapability> {
        self.capabilities
            .iter()
            .filter(|cap| cap.capability_type == *capability_type)
            .collect()
    }
    
    /// Check if this service is healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.health_status, UnifiedHealthStatus::Healthy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_service_creation() {
        let service = UnifiedServiceDefinition::new(
            "Test Service".to_string(),
            "A test service for validation".to_string(),
        );
        
        assert_eq!(service.name, "Test Service");
        assert_eq!(service.description, "A test service for validation");
        assert_eq!(service.version, "1.0.0");
        assert!(service.capabilities.is_empty());
        assert!(service.endpoints.is_empty());
    }
    
    #[test]
    fn test_capability_management() {
        let mut service = UnifiedServiceDefinition::default();
        
        let capability = UnifiedCapability {
            capability_type: CapabilityType::Security,
            name: "Authentication".to_string(),
            description: "User authentication capability".to_string(),
            required_permissions: vec!["auth.read".to_string()],
            configuration: BTreeMap::new(),
            performance_metrics: None,
        };
        
        service.add_capability(capability);
        
        assert_eq!(service.capabilities.len(), 1);
        assert!(service.provides_capability(&CapabilityType::Security));
        assert!(!service.provides_capability(&CapabilityType::Storage));
    }
    
    #[test]
    fn test_service_dependencies() {
        let mut service = UnifiedServiceDefinition::default();
        
        let dependency = ServiceDependency::Required {
            capability: CapabilityType::KeyManagement,
            min_version: "2.0.0".to_string(),
            reason: "Required for encryption operations".to_string(),
            requirements: None,
        };
        
        service.add_dependency(dependency);
        
        assert_eq!(service.dependencies.len(), 1);
        
        if let ServiceDependency::Required { capability, .. } = &service.dependencies[0] {
            assert_eq!(*capability, CapabilityType::KeyManagement);
        } else {
            panic!("Expected Required dependency");
        }
    }
} 