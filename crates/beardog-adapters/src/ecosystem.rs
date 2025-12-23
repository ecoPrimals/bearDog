//! Ecosystem Integration Module
//!
//! Provides basic ecosystem types and functionality for adapter integration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Basic ecosystem service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemService {
    /// Service identifier
    pub id: String,
    /// Service name
    pub name: String,
    /// Service type
    pub service_type: String,
    /// Service endpoints
    pub endpoints: HashMap<String, String>,
    /// Service capabilities
    pub capabilities: Vec<String>,
}

/// Ecosystem integration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationResult {
    /// Success status
    pub success: bool,
    /// Result message
    pub message: String,
    /// Integration data
    pub data: Option<serde_json::Value>,
}

/// Ecosystem capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemCapability {
    /// Capability name
    pub name: String,
    /// Capability version
    pub version: String,
    /// Capability description
    pub description: String,
    /// Required parameters
    pub required_params: Vec<String>,
}

impl Default for EcosystemService {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            name: "Default Service".to_string(),
            service_type: "generic".to_string(),
            endpoints: HashMap::new(),
            capabilities: Vec::new(),
        }
    }
}

impl Default for IntegrationResult {
    fn default() -> Self {
        Self {
            success: false,
            message: "No integration performed".to_string(),
            data: None,
        }
    }
}

/// Primal types for ecosystem integration
pub mod primal_types {
    use super::*;
    
    /// Basic primal service type
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PrimalService {
        /// Primal identifier
        pub id: String,
        /// Primal name
        pub name: String,
        /// Primal type
        pub primal_type: String,
        /// Primal capabilities
        pub capabilities: Vec<String>,
    }
    
    /// Primal integration request
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PrimalIntegrationRequest {
        /// Target primal
        pub primal: PrimalService,
        /// Integration parameters
        pub parameters: HashMap<String, serde_json::Value>,
    }
    
    impl Default for PrimalService {
        fn default() -> Self {
            Self {
                id: "default-primal".to_string(),
                name: "Default Primal".to_string(),
                primal_type: "generic".to_string(),
                capabilities: Vec::new(),
            }
        }
    }
} 