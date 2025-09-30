//! Capability Registry for Zero-Knowledge Bootstrap
//!
//! This module manages discovered capabilities and maintains a registry
//! of services and their capabilities within the ecosystem.

use beardog_errors::BearDogResult;
use beardog_types::canonical::capabilities::CapabilityType as Capability;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Registry of discovered capabilities
#[derive(Debug, Clone)]
pub struct CapabilityRegistry {
    /// Registered capabilities by service
    capabilities: Arc<RwLock<HashMap<String, Vec<Capability>>>>,
    /// Capability metadata
    metadata: Arc<RwLock<HashMap<String, CapabilityMetadata>>>,
}

/// Metadata for a registered capability
#[derive(Debug, Clone)]
pub struct CapabilityMetadata {
    /// Service providing the capability
    pub service_id: String,
    /// When the capability was discovered
    pub discovered_at: std::time::Instant,
    /// Reliability score (0.0 to 1.0)
    pub reliability_score: f64,
    /// Last successful usage
    pub last_used: Option<std::time::Instant>,
}

impl CapabilityRegistry {
    /// Create a new capability registry
    pub fn new() -> Self {
        info!("🗂️ Initializing CapabilityRegistry");
        Self {
            capabilities: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a capability for a service
    pub async fn register_capability(
        &self,
        service_id: String,
        capability: Capability,
    ) -> BearDogResult<()> {
        let capability_key = format!("{}::{:?}", service_id, capability);

        debug!("📝 Registering capability: {}", capability_key);

        // Add to capabilities map
        let mut capabilities = self.capabilities.write().await;
        capabilities
            .entry(service_id.clone())
            .or_insert_with(Vec::new)
            .push(capability);

        // Add metadata
        let mut metadata = self.metadata.write().await;
        metadata.insert(
            capability_key,
            CapabilityMetadata {
                service_id,
                discovered_at: std::time::Instant::now(),
                reliability_score: 1.0, // Start with perfect score
                last_used: None,
            },
        );

        Ok(())
    }

    /// Find services that provide a specific capability
    pub async fn find_services_with_capability(&self, capability: &Capability) -> Vec<String> {
        let capabilities = self.capabilities.read().await;
        capabilities
            .iter()
            .filter_map(|(service_id, caps)| {
                if caps.contains(capability) {
                    Some(service_id.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get all capabilities for a service
    pub async fn get_service_capabilities(&self, service_id: &str) -> Vec<Capability> {
        let capabilities = self.capabilities.read().await;
        capabilities.get(service_id).cloned().unwrap_or_default()
    }

    /// Update reliability score for a capability
    pub async fn update_reliability(
        &self,
        service_id: &str,
        capability: &Capability,
        success: bool,
    ) -> BearDogResult<()> {
        let capability_key = format!("{}::{:?}", service_id, capability);
        let mut metadata = self.metadata.write().await;

        if let Some(meta) = metadata.get_mut(&capability_key) {
            // Update reliability using exponential moving average
            let adjustment = if success { 0.1 } else { -0.2 };
            meta.reliability_score = (meta.reliability_score + adjustment).clamp(0.0, 1.0);
            meta.last_used = Some(std::time::Instant::now());

            debug!(
                "📊 Updated reliability for {}: {:.2}",
                capability_key, meta.reliability_score
            );
        }

        Ok(())
    }

    /// Get capability metadata
    pub async fn get_capability_metadata(
        &self,
        service_id: &str,
        capability: &Capability,
    ) -> Option<CapabilityMetadata> {
        let capability_key = format!("{}::{:?}", service_id, capability);
        let metadata = self.metadata.read().await;
        metadata.get(&capability_key).cloned()
    }

    /// Remove a service and all its capabilities
    pub async fn remove_service(&self, service_id: &str) -> BearDogResult<()> {
        info!("🗑️ Removing service from registry: {}", service_id);

        let mut capabilities = self.capabilities.write().await;
        capabilities.remove(service_id);

        let mut metadata = self.metadata.write().await;
        metadata.retain(|key, _| !key.starts_with(&format!("{}::", service_id)));

        Ok(())
    }
}

impl Default for CapabilityRegistry {
    fn default() -> Self {
        Self::new()
    }
}
