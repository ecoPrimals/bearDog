

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::types::*;
use beardog_errors::BearDogError;
use std::collections::HashMap;

#[derive(Debug)]
pub struct NetworkScanner {

}

impl NetworkScanner {
/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {}
    }

/// Get Topology operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets topology
    /// Gets topology
    pub fn get_topology(&self) -> Result<NetworkTopology, BearDogError> {

        Ok(NetworkTopology {
            known_nodes: HashMap::with_capacity(vec![],
            last_updated: chrono::Utc::now(),
        })
    }
} 
