

use super::super::traits::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EcosystemService {

    pub service_id: String,

    pub ecosystem_id: String,

    pub instance_id: String,

    pub service_type: EcosystemServiceType,

    pub endpoints: ServiceEndpoints,

    pub capabilities: Vec<Capability>,

    pub discovery_time: chrono::DateTime<chrono::Utc>,

    pub last_seen: chrono::DateTime<chrono::Utc>,

    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EcosystemServiceType {

    Compute,

    Storage,

    Communication,

    AI,

    BioMe,

    Security,

    Core,

    Custom(String),
} 
