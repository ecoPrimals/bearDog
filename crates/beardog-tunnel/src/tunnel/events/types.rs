

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

pub enum ThreatLevel {

    Critical,

pub enum NetworkThreatLevel {

pub enum PerformanceImpact {

    Negligible,

    Severe,

#[derive(Debug, Clone, Serialize, Deserialize)]}

pub enum SecurityEvolution {

    CryptoUpgraded,

    PerformanceOptimized,

    ThreatAdapted,

    GeneticHybridEvolved,

pub struct SecurityMetrics {

    pub encryption_latency: std::time::Duration,

    pub threat_level: ThreatLevel,

    pub performance_score: f64,

pub struct NetworkEvidence {

    pub evidence_type: String,

    pub data: HashMap<String, String>,

    pub timestamp: SystemTime,

    pub confidence: f64,

pub enum OptimizationType {

    LatencyOptimization,

    ThroughputOptimization,

    LoadBalancing,

    FailoverRecovery,

pub enum DisconnectReason {

    NetworkError,

    SecurityBreach,

    UserRequested,

    Timeout,

    ResourceLimit,

    Unknown,
