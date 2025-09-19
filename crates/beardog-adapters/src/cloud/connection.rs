

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::providers::CloudProvider;
use std::collections::HashMap;

#[derive(Debug, Clone)]
    pub provider: CloudProvider,

    /// Current status of the component
    pub status: ConnectionStatus,

    /// The created at value
    pub created_at: std::time::SystemTime,

    /// The last used value
    pub last_used: std::time::SystemTime,
}

pub struct CloudConnectionPool {

    connections: HashMap<CloudProvider, Vec<CloudConnection>>,
}

impl Default for CloudConnectionPool {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudConnectionPool {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            connections: HashMap::with_capacity(16),
        }
    }

/// Get Connection operation.
    /// Gets connection
    /// Gets connection
    pub fn get_connection(&self, provider: &CloudProvider) -> Option<&CloudConnection> {
        self.connections.get(provider)?.first()
    }

/// Add Connection operation.
    pub fn add_connection(&mut self, connection: CloudConnection) {
        self.connections
            .entry(connection.provider)
            .or_default()
            .push(connection);
    }
}
