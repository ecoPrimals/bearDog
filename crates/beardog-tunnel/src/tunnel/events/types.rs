// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityLevel {
    /// Number of level
    pub level: u32,
    /// Number of authentication_strength
    pub authentication_strength: u32,
    /// The threat detection accuracy value
    pub threat_detection_accuracy: f64,
    pub performance_overhead: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEvidence {
    pub timestamp: SystemTime,
    /// The source ip value
    pub source_ip: String,
    /// The destination ip value
    pub destination_ip: String,
    /// Number of packet_size
    pub packet_size: u32,
    /// The protocol value
    pub protocol: String,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of optimization
pub enum OptimizationType {
    /// Represents latency variant
    Latency,
    /// Represents bandwidth variant
    Bandwidth,
    /// Represents security variant
    Security,
    /// Represents reliability variant
    Reliability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisconnectReason {
    /// State indicating userrequested
    UserRequested,
    /// Represents timeout variant
    Timeout,
    /// Represents resource limit variant
    ResourceLimit,
    /// Unknown or undefined state
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatLevel {
    /// Represents low variant
    Low,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceImpact {
    /// No none specified
    None,
    /// Represents minimal variant
    Minimal,
    /// Represents moderate variant
    Moderate,
    /// Represents significant variant
    Significant,
}
