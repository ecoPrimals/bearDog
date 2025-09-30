//! Additional Unified Service Types
//!
//! This module contains supplementary unified service types that don't fit
//! into the main service definition categories.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Service registry entry for tracking discovered services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistryEntry {
    /// Service definition
    pub service: super::UnifiedServiceDefinition,
    
    /// Registration timestamp
    pub registered_at: chrono::DateTime<chrono::Utc>,
    
    /// Last heartbeat timestamp
    pub last_heartbeat: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Service discovery metadata
    pub discovery_metadata: HashMap<String, String>,
    
    /// Registration source
    pub source: RegistrationSource,
}

/// Source of service registration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegistrationSource {
    /// Service registered itself
    SelfRegistration,
    /// Service discovered via network scan
    NetworkDiscovery,
    /// Service registered by administrator
    ManualRegistration,
    /// Service discovered via DNS
    DnsDiscovery,
    /// Service discovered via configuration
    ConfigurationBased,
    /// Custom registration source
    Custom(String),
}

impl Default for ServiceRegistryEntry {
    fn default() -> Self {
        Self {
            service: super::UnifiedServiceDefinition::default(),
            registered_at: chrono::Utc::now(),
            last_heartbeat: None,
            discovery_metadata: HashMap::new(),
            source: RegistrationSource::SelfRegistration,
        }
    }
}

impl ServiceRegistryEntry {
    /// Create a new registry entry for a service
    pub fn new(service: super::UnifiedServiceDefinition, source: RegistrationSource) -> Self {
        Self {
            service,
            source,
            ..Default::default()
        }
    }
    
    /// Update the last heartbeat timestamp
    pub fn update_heartbeat(&mut self) {
        self.last_heartbeat = Some(chrono::Utc::now());
    }
    
    /// Check if the service registration is stale
    pub fn is_stale(&self, stale_threshold_seconds: u64) -> bool {
        if let Some(last_heartbeat) = self.last_heartbeat {
            let now = chrono::Utc::now();
            let threshold = chrono::Duration::seconds(stale_threshold_seconds as i64);
            now - last_heartbeat > threshold
        } else {
            // No heartbeat recorded, check registration time
            let now = chrono::Utc::now();
            let threshold = chrono::Duration::seconds(stale_threshold_seconds as i64);
            now - self.registered_at > threshold
        }
    }
} 