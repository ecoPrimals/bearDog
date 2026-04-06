// SPDX-License-Identifier: AGPL-3.0-or-later

// Canonical Provider System - Unified Provider Hierarchy
//
// This module consolidates ALL provider trait definitions from across BearDog
// into a single, canonical hierarchy that eliminates fragmentation and provides
// the definitive interface for all provider implementations.
//
// ## Consolidation Summary
//
// This replaces and unifies:
// - `beardog-traits/src/unified/providers.rs` - Unified provider traits
// - `beardog-traits/src/canonical/*Provider` - Canonical provider traits  
// - `beardog-types/src/canonical/providers_unified/` - Previous unified system
// - Scattered provider definitions across 22 crates
//
// ## Architecture Principles
//
// - **Single Source of Truth**: All provider traits in one canonical location
// - **Hierarchical Design**: Clear inheritance from BaseProvider to specialized traits
// - **Zero Fragmentation**: No duplicate trait definitions anywhere in codebase
// - **Native Async**: Zero-cost native async/await throughout
// - **Type Safety**: Strongly typed with comprehensive error handling
// - **Ecosystem Integration**: Built for ecosystem relationship patterns

use beardog_errors::BearDogError;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Core provider modules
pub mod base;

// Re-export all provider traits and types
pub use base::*;

/// Provider registry for managing all providers in the ecosystem
#[derive(Debug, Clone)]
pub struct ProviderRegistry {
    /// Registered providers by ID
    providers: HashMap<String, RegisteredProvider>,
    
    /// Provider relationships for ecosystem coordination
    relationships: HashMap<String, Vec<ProviderRelationship>>,
    
    /// Registry configuration
    config: ProviderRegistryConfig,
}

/// Information about a registered provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredProvider {
    /// Provider information
    pub info: ProviderInfo,
    
    /// Provider health status
    pub health: ProviderHealth,
    
    /// Provider capabilities
    pub capabilities: Vec<ProviderCapability>,
    
    /// Registration timestamp
    pub registered_at: SystemTime,
    
    /// Last health check
    pub last_health_check: SystemTime,
    
    /// Provider metadata
    pub metadata: HashMap<String, String>,
}

/// Relationship between providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRelationship {
    /// Related provider ID
    pub provider_id: String,
    
    /// Type of relationship
    pub relationship_type: RelationshipType,
    
    /// Relationship strength (0.0-1.0)
    pub strength: f64,
    
    /// Relationship metadata
    pub metadata: HashMap<String, String>,
}

/// Type of provider relationship
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Providers depend on each other
    Dependency,
    
    /// Providers complement each other
    Complementary,
    
    /// Providers can substitute for each other
    Alternative,
    
    /// Providers work together in a chain
    Pipeline,
    
    /// Providers collaborate on shared objectives
    Collaborative,
}

/// Configuration for the provider registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRegistryConfig {
    /// Maximum number of providers to register
    pub max_providers: usize,
    
    /// Health check interval
    pub health_check_interval: Duration,
    
    /// Provider timeout for operations
    pub provider_timeout: Duration,
    
    /// Enable automatic provider discovery
    pub enable_auto_discovery: bool,
    
    /// Registry metadata
    pub metadata: HashMap<String, String>,
}

/// Provider information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Unique provider identifier
    pub id: String,
    
    /// Human-readable provider name
    pub name: String,
    
    /// Provider version
    pub version: String,
    
    /// Provider type category
    pub provider_type: ProviderType,
    
    /// Provider description
    pub description: Option<String>,
    
    /// Provider vendor/author
    pub vendor: Option<String>,
    
    /// Provider capabilities
    pub capabilities: Vec<String>,
    
    /// Provider metadata
    pub metadata: HashMap<String, String>,
}

/// Provider type categories
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProviderType {
    /// Security and authentication providers
    Security,
    
    /// Cryptographic operation providers
    Cryptographic,
    
    /// Hardware Security Module providers
    HardwareSecurityModule,
    
    /// AI/ML and genetics providers
    Genetics,
    
    /// Monitoring and observability providers
    Monitoring,
    
    /// External system adapter providers
    Adapter,
    
    /// Workflow and orchestration providers
    Workflow,
    
    /// Storage and persistence providers
    Storage,
    
    /// Network and communication providers
    Network,
    
    /// Custom provider type
    Custom(String),
}

/// Provider health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealth {
    /// Overall health status
    pub status: HealthStatus,
    
    /// Health score (0.0-1.0)
    pub score: f64,
    
    /// Last health check timestamp
    pub last_check: SystemTime,
    
    /// Health check details
    pub details: HashMap<String, String>,
    
    /// Performance metrics
    pub performance: PerformanceMetrics,
}

/// Health status enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Provider is healthy and operational
    Healthy,
    
    /// Provider is operational but with warnings
    Warning,
    
    /// Provider is degraded but still functional
    Degraded,
    
    /// Provider is unhealthy and may not function correctly
    Unhealthy,
    
    /// Provider is completely unavailable
    Unavailable,
}

/// Provider performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average response time in milliseconds
    pub avg_response_time: f64,
    
    /// Success rate (0.0-1.0)
    pub success_rate: f64,
    
    /// Error rate (0.0-1.0)
    pub error_rate: f64,
    
    /// Throughput (operations per second)
    pub throughput: f64,
    
    /// Resource utilization (0.0-1.0)
    pub resource_utilization: f64,
    
    /// Additional metrics
    pub custom_metrics: HashMap<String, f64>,
}

/// Provider capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapability {
    /// Capability name
    pub name: String,
    
    /// Capability version
    pub version: String,
    
    /// Capability description
    pub description: Option<String>,
    
    /// Capability parameters
    pub parameters: HashMap<String, CapabilityParameter>,
    
    /// Capability metadata
    pub metadata: HashMap<String, String>,
}

/// Parameter for a provider capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityParameter {
    /// Parameter name
    pub name: String,
    
    /// Parameter type
    pub parameter_type: ParameterType,
    
    /// Whether parameter is required
    pub required: bool,
    
    /// Default value if any
    pub default_value: Option<String>,
    
    /// Parameter description
    pub description: Option<String>,
}

/// Parameter type for capabilities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Array,
    Object,
    Custom(String),
}

impl ProviderRegistry {
    /// Create a new provider registry
    pub fn new(config: ProviderRegistryConfig) -> Self {
        Self {
            providers: HashMap::new(),
            relationships: HashMap::new(),
            config,
        }
    }
    
    /// Register a new provider
    pub fn register_provider(
        &mut self,
        provider_info: ProviderInfo,
        capabilities: Vec<ProviderCapability>,
    ) -> Result<()> {
                    if self.providers.len() >= self.config.max_providers {
                return Err(BearDogError::system("Provider registry at capacity".to_string()));
            }
        
        let registered_provider = RegisteredProvider {
            info: provider_info.clone(),
            health: ProviderHealth::default(),
            capabilities,
            registered_at: SystemTime::now(),
            last_health_check: SystemTime::now(),
            metadata: HashMap::new(),
        };
        
        self.providers.insert(provider_info.id.clone(), registered_provider);
        Ok(())
    }
    
    /// Get a registered provider by ID
    pub fn get_provider(&self, provider_id: &str) -> Option<&RegisteredProvider> {
        self.providers.get(provider_id)
    }
    
    /// Get providers by type
    pub fn get_providers_by_type(&self, provider_type: &ProviderType) -> Vec<&RegisteredProvider> {
        self.providers
            .values()
            .filter(|p| &p.info.provider_type == provider_type)
            .collect()
    }
    
    /// Get providers by capability
    pub fn get_providers_by_capability(&self, capability: &str) -> Vec<&RegisteredProvider> {
        self.providers
            .values()
            .filter(|p| p.capabilities.iter().any(|c| c.name == capability))
            .collect()
    }
    
    /// Update provider health
    pub fn update_provider_health(
        &mut self,
        provider_id: &str,
        health: ProviderHealth,
    ) -> Result<()> {
                    let provider = self.providers.get_mut(provider_id)
                .ok_or_else(|| BearDogError::business("Provider not found".to_string()))?;
            
        provider.health = health;
        provider.last_health_check = SystemTime::now();
        Ok(())
    }
    
    /// Add a relationship between providers
    pub fn add_relationship(
        &mut self,
        provider_id: &str,
        relationship: ProviderRelationship,
    ) -> Result<()> {
        self.relationships
            .entry(provider_id.to_string())
            .or_insert_with(Vec::new)
            .push(relationship);
        Ok(())
    }
    
    /// Get relationships for a provider
    pub fn get_relationships(&self, provider_id: &str) -> Vec<&ProviderRelationship> {
        self.relationships
            .get(provider_id)
            .map(|rels| rels.iter().collect())
            .unwrap_or_default()
    }
    
    /// Get all healthy providers
    pub fn get_healthy_providers(&self) -> Vec<&RegisteredProvider> {
        self.providers
            .values()
            .filter(|p| p.health.status == HealthStatus::Healthy)
            .collect()
    }
}

impl Default for ProviderRegistryConfig {
    fn default() -> Self {
        Self {
            max_providers: beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE,
            health_check_interval: Duration::from_secs(60),
            provider_timeout: Duration::from_secs(30),
            enable_auto_discovery: true,
            metadata: HashMap::new(),
        }
    }
}

impl Default for ProviderHealth {
    fn default() -> Self {
        Self {
            status: HealthStatus::Healthy,
            score: 1.0,
            last_check: SystemTime::now(),
            details: HashMap::new(),
            performance: PerformanceMetrics::default(),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_response_time: 0.0,
            success_rate: 1.0,
            error_rate: 0.0,
            throughput: 0.0,
            resource_utilization: 0.0,
            custom_metrics: HashMap::new(),
        }
    }
} 