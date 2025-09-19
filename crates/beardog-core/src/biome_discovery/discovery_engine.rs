

use super::types::*;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Arc<RwLock<HashMap<String, ActiveDiscovery>>>,
    discovery_strategies: HashMap<DiscoveryProtocol, Box<dyn DiscoveryStrategy + Send + Sync>>,
    discovery_history: Arc<RwLock<Vec<DiscoveryEvent>>>,
    network_topology: Arc<RwLock<NetworkTopology>>,
}

impl DiscoveryEngine {
/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            active_discoveries: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            discovery_strategies: HashMap::with_capacity(16),
            discovery_history: Arc::new(RwLock::new(Vec::new())),
            network_topology: Arc::new(RwLock::new(NetworkTopology {
                known_nodes: HashMap::with_capacity(16),
                connection_graph: Vec::new(),
                last_updated: chrono::Utc::now(&str,
        discovery: ActiveDiscovery,
    ) -> Result<(), BearDogError> {
        self.active_discoveries.write().insert(discovery_id, discovery);
        Ok(())
    }

/// Remove Active Discovery operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Removes active_discovery
    /// Removes active_discovery
    pub fn remove_active_discovery(&self, discovery_id: &str) -> Result<(), BearDogError> {
        self.active_discoveries.write(DiscoveryProtocol,
    ) -> Result<Vec<BiomeCandidate>, BearDogError> {

        Ok(vec![])
    }
} 
