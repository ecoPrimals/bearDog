// Universal Primal Registry - Modernized capability-based discovery
// Provides unified ecosystem integration through capability discovery

use beardog_core::universal_discovery::CapabilityType;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Modernized Primal ID using capability-based identification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PrimalId {
    pub id: String,
    /// Name of the item
    pub name: String,
    /// The version value
    pub version: String,
    /// Collection of capabilities
    pub capabilities: Vec<CapabilityType>,
}

impl PrimalId {
    /// Create new primal ID with capabilities
    /// Creates a new instance
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        capabilities: Vec<CapabilityType>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            capabilities,
        }
    }

    /// Create from ID with default capabilities
    /// Creates instance from id
    pub fn from_id(id: impl Into<String>) -> Self {
        let id_str = id.into();
        Self {
            id: id_str.clone(),
            name: id_str,
            version: "3.0.0".to_string(),
            capabilities: vec![CapabilityType::Universal],
        }
    }

    /// BearDog ecosystem integration - self-identity only
    /// This is the only hardcoded identity as BearDog only knows itself
    pub fn beardog() -> Self {
        Self::new(
            "beardog ",
            "BearDog",
            "3.0.0",
            vec![
                CapabilityType::Security,
                CapabilityType::Storage,
                CapabilityType::Compute,
                CapabilityType::Network,
                CapabilityType::Universal,
            ],
        )
    }

    /// Create primal ID from discovered service with capabilities
    /// This replaces hardcoded primal constructors with dynamic discovery
    /// Creates instance from discovered service
    pub fn from_discovered_service(
        service_id: impl Into<String>,
        service_name: impl Into<String>,
        version: impl Into<String>,
        capabilities: Vec<CapabilityType>,
    ) -> Self {
        Self::new(service_id, service_name, version, capabilities)
    }

    pub fn for_capability(capability: CapabilityType) -> Self {
        let capability_name = format!("{:?}", capability).to_lowercase();
        Self::new(
            format!("dynamic-{}", capability_name),
            format!("Dynamic {} Provider", capability_name),
            "dynamic",
            vec![capability],
        )
    }

    /// Check if primal supports capability
    pub fn supports_capability(&self, capability: &CapabilityType) -> bool {
        self.capabilities.contains(capability)
            || self.capabilities.contains(&CapabilityType::Universal)
    }

    pub fn capability_score(&self, required: &CapabilityType) -> f64 {
        if self.capabilities.contains(required) {
            1.0
        } else if self.capabilities.contains(&CapabilityType::Universal) {
            0.8 // Universal providers have lower priority than specific ones
        } else {
            0.0
        }
    }
}

/// Modernized capability-based primal registry
#[derive(Debug, Default)]
pub struct CapabilityBasedPrimalRegistry {
    primals: HashMap<String, PrimalId>,
    capability_index: HashMap<CapabilityType, Vec<String>>,
}

impl CapabilityBasedPrimalRegistry {
    /// Create new registry
    /// Creates a new instance
    pub fn new() -> Self {
        let mut registry = Self::default();
        // Register BearDog ecosystem by default
        registry.register_primal(PrimalId::beardog()).ok();
        registry
    }

    /// Register a primal with capabilities
    pub fn register_primal(&mut self, primal: PrimalId) -> Result<(), BearDogError> {
        // Update capability index
        for capability in &primal.capabilities {
            self.capability_index
                .entry(capability.clone())
                .or_insert_with(Vec::new)
                .push(primal.id.clone());
        }

        self.primals.insert(primal.id.clone(), primal);
        Ok(())
    }

    /// Discover primals by capability
    pub fn discover_capability(&self, capability: CapabilityType) -> Vec<&PrimalId> {
        if let Some(primal_ids) = self.capability_index.get(&capability) {
            primal_ids
                .iter()
                .filter_map(|id| self.primals.get(id))
                .collect()
        } else {
            // Fallback to universal capability providers
            self.primals
                .values()
                .filter(|p| p.supports_capability(&capability))
                .collect()
        }
    }

    /// Gets best_primal
    /// Gets best_primal
    pub fn get_best_primal(&self, capability: CapabilityType) -> Option<&PrimalId> {
        self.discover_capability(capability)
            .into_iter()
            .max_by(|a, b| {
                a.capability_score(&capability)
                    .partial_cmp(&b.capability_score(&capability))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Get all registered primals
    pub fn list_primals(&self) -> Vec<&PrimalId> {
        self.primals.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[test]
    fn test_capability_based_discovery() {
        let mut registry = CapabilityBasedPrimalRegistry::new();

        let custom_primal = PrimalId::new(
            "custom",
            "CustomPrimal",
            "1.0.0",
            vec![CapabilityType::Security, CapabilityType::Storage],
        );

        registry
            .register_primal(custom_primal)
            .map_err(|e| {
                eprintln!("Warning: Failed to register custom primal: {}", e);
            })
            .ok();

        let security_primals = registry.discover_capability(CapabilityType::Security);
        assert!(!security_primals.is_empty());

        let best = registry.get_best_primal(CapabilityType::Security);
        assert!(best.is_some());
    }
}
