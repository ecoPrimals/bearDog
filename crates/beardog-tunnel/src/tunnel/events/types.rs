

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityLevel {
    Ultimate,
    High,
    Medium,
    Low,
    Adaptive,
    Optimized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatLevel {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkThreatLevel {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceImpact {
    Negligible,
    Minor,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEvolution {
    CryptoUpgraded,
    PerformanceOptimized,
    ThreatAdapted,
    ComplianceUpdated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    pub encryption_strength: u32,
    pub authentication_level: SecurityLevel,
    pub threat_detection_accuracy: f64,
    pub performance_overhead: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEvidence {
    pub timestamp: SystemTime,
    pub source_ip: String,
    pub destination_ip: String,
    pub packet_size: u32,
    pub protocol: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationType {
    Latency,
    Bandwidth,
    Security,
    Reliability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisconnectReason {
    UserRequested,
    Timeout,
    ResourceLimit,
    Unknown,
}
