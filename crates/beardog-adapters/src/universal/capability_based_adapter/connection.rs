//! # Connection Management
//!
//! Connection management for discovered capabilities.

use super::types::*;
use super::core::UniversalCapabilityAdapter;
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::UniversalCapability;
use crate::universal::types::CapabilityConnection;
use tracing::{debug, info};
use uuid::Uuid;

impl UniversalCapabilityAdapter {
    /// Connect to a discovered capability
    pub async fn connect_to_capability(
        &self,
        capability: &UniversalCapability,
    ) -> Result<String> {
        let connection_id = Uuid::new_v4().to_string();
        info!("🔗 Connecting to capability: {}", capability.id);
        
        // Create connection placeholder
        let connection = CapabilityConnection {
            id: connection_id.clone(),
            capability_id: capability.id.clone(),
            // Add other connection fields as needed
        };
        
        // Store connection
        let mut connections = self.connections.write().await;
        connections.insert(connection_id.clone(), connection);
        
        info!("✅ Connected to capability. Connection ID: {}", connection_id);
        Ok(connection_id)
    }

    /// Register a capability provider
    pub async fn register_capability_provider(
        &self,
        capability: UniversalCapability,
    ) -> Result<()> {
        info!("📋 Registering capability provider: {}", capability.id);
        
        let mut capabilities = self.capabilities.write().await;
        let capability_type = capability.capability_type.clone();
        
        capabilities
            .entry(capability_type)
            .or_insert_with(Vec::new)
            .push(capability);
        
        info!("✅ Capability provider registered successfully");
        Ok(())
    }
} 