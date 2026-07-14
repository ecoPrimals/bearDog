// SPDX-License-Identifier: AGPL-3.0-or-later

//! Service Metadata Definitions
//!
//! This module provides metadata types for services, consolidating metadata
//! definitions from across the codebase.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Comprehensive service metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    /// Service tags for categorization
    pub tags: Vec<String>,
    
    /// Service labels for identification
    pub labels: BTreeMap<String, String>,
    
    /// Service annotations for additional information
    pub annotations: BTreeMap<String, String>,
    
    /// Service owner information
    pub owner: Option<ServiceOwner>,
    
    /// Service contact information
    pub contacts: Vec<ServiceContact>,
    
    /// Service documentation links
    pub documentation: Vec<DocumentationLink>,
    
    /// Service SLA information
    pub sla: Option<ServiceSla>,
    
    /// Service resource requirements
    pub resource_requirements: Option<ResourceRequirements>,
    
    /// Service deployment information
    pub deployment: Option<DeploymentInfo>,
    
    /// Custom metadata fields
    pub custom_fields: BTreeMap<String, serde_json::Value>,
}

/// Service owner information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOwner {
    /// Owner name or team name
    pub name: String,
    
    /// Owner email
    pub email: Option<String>,
    
    /// Owner team or department
    pub team: Option<String>,
    
    /// Owner role or responsibility
    pub role: Option<String>,
}

/// Service contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceContact {
    /// Contact type (e.g., "oncall", "support", "development")
    pub contact_type: String,
    
    /// Contact name
    pub name: String,
    
    /// Contact email
    pub email: Option<String>,
    
    /// Contact phone
    pub phone: Option<String>,
    
    /// Contact availability
    pub availability: Option<String>,
}

/// Documentation link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationLink {
    /// Link type (e.g., "api", "runbook", "architecture")
    pub link_type: String,
    
    /// Link title
    pub title: String,
    
    /// Link URL
    pub url: String,
    
    /// Link description
    pub description: Option<String>,
}

/// Service Level Agreement information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSla {
    /// Availability target (e.g., 99.9%)
    pub availability_target: f64,
    
    /// Response time target in milliseconds
    pub response_time_target_ms: f64,
    
    /// Error rate target (e.g., 0.1%)
    pub error_rate_target: f64,
    
    /// Recovery time objective in seconds
    pub recovery_time_objective_seconds: u64,
    
    /// Recovery point objective in seconds
    pub recovery_point_objective_seconds: u64,
    
    /// SLA measurement period
    pub measurement_period: SlaMeasurementPeriod,
}

/// SLA measurement period
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SlaMeasurementPeriod {
    /// Daily measurement
    Daily,
    /// Weekly measurement
    Weekly,
    /// Monthly measurement
    Monthly,
    /// Quarterly measurement
    Quarterly,
    /// Annual measurement
    Annual,
    /// Custom period in seconds
    Custom(u64),
}

/// Resource requirements for the service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// CPU requirements
    pub cpu: Option<CpuRequirements>,
    
    /// Memory requirements
    pub memory: Option<MemoryRequirements>,
    
    /// Storage requirements
    pub storage: Option<StorageRequirements>,
    
    /// Network requirements
    pub network: Option<NetworkRequirements>,
}

/// CPU requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuRequirements {
    /// Minimum CPU cores required
    pub min_cores: f64,
    
    /// Recommended CPU cores
    pub recommended_cores: f64,
    
    /// Maximum CPU cores that can be utilized
    pub max_cores: Option<f64>,
    
    /// CPU architecture requirements
    pub architecture: Option<String>,
}

/// Memory requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRequirements {
    /// Minimum memory in bytes
    pub min_bytes: u64,
    
    /// Recommended memory in bytes
    pub recommended_bytes: u64,
    
    /// Maximum memory that can be utilized
    pub max_bytes: Option<u64>,
}

/// Storage requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageRequirements {
    /// Minimum storage in bytes
    pub min_bytes: u64,
    
    /// Recommended storage in bytes
    pub recommended_bytes: u64,
    
    /// Storage type requirements
    pub storage_type: Option<StorageType>,
    
    /// IOPS requirements
    pub iops_requirements: Option<u64>,
}

/// Storage type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageType {
    /// Solid State Drive
    Ssd,
    /// Hard Disk Drive
    Hdd,
    /// Network Attached Storage
    Nas,
    /// Object Storage
    ObjectStorage,
    /// Block Storage
    BlockStorage,
    /// Custom storage type
    Custom(String),
}

/// Network requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRequirements {
    /// Minimum bandwidth in bits per second
    pub min_bandwidth_bps: u64,
    
    /// Recommended bandwidth in bits per second
    pub recommended_bandwidth_bps: u64,
    
    /// Maximum latency tolerance in milliseconds
    pub max_latency_ms: f64,
    
    /// Required network ports
    pub required_ports: Vec<NetworkPort>,
}

/// Network port requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPort {
    /// Port number
    pub port: u16,
    
    /// Protocol (TCP, UDP, etc.)
    pub protocol: String,
    
    /// Port description
    pub description: Option<String>,
    
    /// Whether the port is required or optional
    pub required: bool,
}

/// Deployment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentInfo {
    /// Deployment environment
    pub environment: String,
    
    /// Deployment region
    pub region: Option<String>,
    
    /// Deployment zone or availability zone
    pub zone: Option<String>,
    
    /// Container image information
    pub container_image: Option<ContainerImageInfo>,
    
    /// Scaling configuration
    pub scaling: Option<ScalingConfig>,
    
    /// Health check configuration
    pub health_checks: Vec<String>,
}

/// Container image information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerImageInfo {
    /// Image registry URL
    pub registry: String,
    
    /// Image name
    pub name: String,
    
    /// Image tag
    pub tag: String,
    
    /// Image digest
    pub digest: Option<String>,
}

/// Scaling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingConfig {
    /// Minimum number of instances
    pub min_instances: u32,
    
    /// Maximum number of instances
    pub max_instances: u32,
    
    /// Target CPU utilization for scaling
    pub target_cpu_utilization: Option<f64>,
    
    /// Target memory utilization for scaling
    pub target_memory_utilization: Option<f64>,
    
    /// Custom scaling metrics
    pub custom_metrics: BTreeMap<String, f64>,
}

impl Default for ServiceMetadata {
    fn default() -> Self {
        Self {
            tags: Vec::new(),
            labels: BTreeMap::new(),
            annotations: BTreeMap::new(),
            owner: None,
            contacts: Vec::new(),
            documentation: Vec::new(),
            sla: None,
            resource_requirements: None,
            deployment: None,
            custom_fields: BTreeMap::new(),
        }
    }
}

impl ServiceMetadata {
    /// Create new service metadata with basic information
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a tag to the service
    pub fn add_tag(&mut self, tag: String) -> &mut Self {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
        self
    }
    
    /// Add a label to the service
    pub fn add_label(&mut self, key: String, value: String) -> &mut Self {
        self.labels.insert(key, value);
        self
    }
    
    /// Add an annotation to the service
    pub fn add_annotation(&mut self, key: String, value: String) -> &mut Self {
        self.annotations.insert(key, value);
        self
    }
    
    /// Set the service owner
    pub fn with_owner(mut self, owner: ServiceOwner) -> Self {
        self.owner = Some(owner);
        self
    }
    
    /// Add a contact to the service
    pub fn add_contact(&mut self, contact: ServiceContact) -> &mut Self {
        self.contacts.push(contact);
        self
    }
    
    /// Add documentation link
    pub fn add_documentation(&mut self, link: DocumentationLink) -> &mut Self {
        self.documentation.push(link);
        self
    }
    
    /// Set SLA information
    pub fn with_sla(mut self, sla: ServiceSla) -> Self {
        self.sla = Some(sla);
        self
    }
    
    /// Check if service has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
    
    /// Get label value by key
    pub fn get_label(&self, key: &str) -> Option<&String> {
        self.labels.get(key)
    }
    
    /// Get annotation value by key
    pub fn get_annotation(&self, key: &str) -> Option<&String> {
        self.annotations.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_metadata_creation() {
        let mut metadata = ServiceMetadata::new();
        
        metadata
            .add_tag("production".to_string())
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            .add_tag("api".to_string())
            .add_label("version".to_string(), "1.0.0".to_string())
            .add_annotation("description".to_string(), "Main API service".to_string());
        
        assert_eq!(metadata.tags.len(), 2);
        assert!(metadata.has_tag("production"));
        assert!(metadata.has_tag("api"));
        assert_eq!(metadata.get_label("version"), Some(&"1.0.0".to_string()));
        assert_eq!(metadata.get_annotation("description"), Some(&"Main API service".to_string()));
    }
    
    #[test]
    fn test_service_owner() {
        let owner = ServiceOwner {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            name: "Platform Team".to_string(),
            email: Some("platform@example.com".to_string()),
            team: Some("Engineering".to_string()),
            role: Some("Service Owner".to_string()),
        };
        
        let metadata = ServiceMetadata::new().with_owner(owner);
        
        assert!(metadata.owner.is_some());
        let owner = metadata.owner.expect("Owner should be set after with_owner call");
        assert_eq!(owner.name, "Platform Team");
        assert_eq!(owner.email, Some("platform@example.com".to_string()));
    }
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_sla_configuration() {
        let sla = ServiceSla {
            availability_target: 99.9,
            response_time_target_ms: 100.0,
            error_rate_target: 0.1,
            recovery_time_objective_seconds: 300,
            recovery_point_objective_seconds: 60,
            measurement_period: SlaMeasurementPeriod::Monthly,
        };
        
        let metadata = ServiceMetadata::new().with_sla(sla);
        
        assert!(metadata.sla.is_some());
        let sla = metadata.sla.expect("SLA should be set after with_sla call");
        assert_eq!(sla.availability_target, 99.9);
        assert_eq!(sla.measurement_period, SlaMeasurementPeriod::Monthly);
    }
} 