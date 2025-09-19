

use super::super::traits::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
    pub ecosystem_id: String,


    pub instance_id: String,

    /// The service type value
    pub service_type: EcosystemServiceType,

    /// The endpoints value
    pub endpoints: ServiceEndpoints,

    /// Collection of capabilities
    pub capabilities: Vec<Capability>,


    pub discovery_time: chrono::DateTime<chrono::Utc>,

    /// The last seen value
    pub last_seen: chrono::DateTime<chrono::Utc>,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Types of ecosystem service
pub enum EcosystemServiceType {


    /// Represents compute variant
    Compute,


    /// Represents storage variant
    Storage,


    /// Represents communication variant
    Communication,


    /// Represents a i variant
    AI,


    /// Represents bio me variant
    BioMe,


    /// Represents security variant
    Security,


    /// Represents core variant
    Core,

    /// Represents custom variant
    Custom(String),
} 
} 
} 
