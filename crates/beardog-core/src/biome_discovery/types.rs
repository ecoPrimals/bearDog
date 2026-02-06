//! # Biome Discovery Types
//!
//! This module provides types for discovering and managing biomes in the
//! BearDog ecosystem. Biomes are clusters of primals that share common
//! genetic signatures and security policies.

use beardog_errors::BearDogError;
use beardog_genetics::{BiomeIdentity, GeneticSignature, TrustLevel};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::AtomicU64;

// ============================================================
// Discovery Configuration
// ============================================================

/// Configuration for biome discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Interval between discovery runs in seconds
    pub discovery_interval_seconds: u32,

    /// Timeout for network scans in seconds
    pub network_scan_timeout_seconds: u32,

    /// Depth of genetic analysis
    pub genetic_analysis_depth: AnalysisDepth,

    /// Minimum trust threshold for accepting biomes
    pub trust_threshold: f64,

    /// Whether to auto-register discovered biomes
    pub auto_registration_enabled: bool,

    /// Whether to quarantine suspicious biomes
    pub quarantine_suspicious_biomes: bool,

    /// Protocols to use for discovery
    pub discovery_protocols: Vec<DiscoveryProtocol>,

    /// Maximum concurrent discovery operations
    pub max_concurrent_discoveries: usize,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            discovery_interval_seconds: 3600,
            network_scan_timeout_seconds: 30,
            genetic_analysis_depth: AnalysisDepth::Standard,
            trust_threshold: 0.5,
            auto_registration_enabled: false,
            quarantine_suspicious_biomes: true,
            discovery_protocols: vec![
                DiscoveryProtocol::NetworkBroadcast,
                DiscoveryProtocol::GeneticBeacon,
                DiscoveryProtocol::PeerRecommendation,
            ],
            max_concurrent_discoveries: 10,
        }
    }
}

/// Depth of genetic analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalysisDepth {
    /// Quick scan - minimal analysis
    Quick,

    /// Standard analysis depth
    Standard,

    /// Deep analysis - full genetic tree
    Deep,

    /// Exhaustive - includes historical analysis
    Exhaustive,
}

impl Default for AnalysisDepth {
    fn default() -> Self {
        Self::Standard
    }
}

// ============================================================
// Discovery Protocols
// ============================================================

/// Discovery protocol for finding biomes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiscoveryProtocol {
    /// Network broadcast discovery
    NetworkBroadcast,

    /// Genetic beacon discovery
    GeneticBeacon,

    /// Peer recommendation
    PeerRecommendation,

    /// DNS-based discovery
    DnsDiscovery,

    /// Multicast discovery
    Multicast,

    /// Manual configuration
    Manual,
}

/// Resource usage during discovery
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// CPU time used in milliseconds
    pub cpu_time_ms: u64,

    /// Memory used in bytes
    pub memory_bytes: u64,

    /// Network bytes sent
    pub network_bytes_sent: u64,

    /// Network bytes received
    pub network_bytes_received: u64,
}

/// Discovery strategy trait
pub trait DiscoveryStrategy: Send + Sync {
    /// Discover biomes matching the query
    fn discover_biomes(
        &self,
        config: &DiscoveryConfig,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<BiomeCandidate>, BearDogError>> + Send>,
    >;

    /// Get the protocol used by this strategy
    fn protocol(&self) -> DiscoveryProtocol;
}

// ============================================================
// Biome Candidate Types
// ============================================================

/// A discovered biome candidate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeCandidate {
    /// Unique identifier for this discovery
    pub discovery_id: String,

    /// Discovered endpoint
    pub discovered_endpoint: String,

    /// Biome identity
    pub biome_identity: BiomeIdentity,

    /// Genetic signature
    pub genetic_signature: GeneticSignature,

    /// How this biome was discovered
    pub discovery_method: DiscoveryProtocol,

    /// Confidence in this discovery (0.0 - 1.0)
    pub discovery_confidence: f64,

    /// Initial trust score
    pub initial_trust_score: f64,

    /// Network metadata
    pub network_metadata: NetworkMetadata,

    /// Capabilities claimed by the biome
    pub capabilities_claimed: Vec<String>,

    /// When this candidate was discovered
    pub discovery_timestamp: DateTime<Utc>,
}

/// Network metadata for a discovered biome
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkMetadata {
    /// IP address
    pub address: String,

    /// Port number
    pub port: u16,

    /// Protocol (tcp, udp, etc.)
    pub protocol: String,

    /// Response time in milliseconds
    pub response_time_ms: u64,

    /// Geographic location if known
    pub geographic_location: Option<GeographicLocation>,

    /// Network segment
    pub network_segment: String,

    /// Connection quality (0.0 - 1.0)
    pub connection_quality: f64,
}

/// Geographic location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GeographicLocation {
    /// Country code
    pub country: Option<String>,

    /// Region/state
    pub region: Option<String>,

    /// City
    pub city: Option<String>,

    /// Latitude
    pub latitude: Option<f64>,

    /// Longitude
    pub longitude: Option<f64>,
}

// ============================================================
// Discovery Session Types
// ============================================================

/// Status of a discovery operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryStatus {
    /// Not started
    Pending,

    /// Currently running
    Running,

    /// Completed successfully
    Completed,

    /// Failed
    Failed,

    /// Cancelled
    Cancelled,
}

/// Discovery session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverySession {
    /// Unique session ID
    pub session_id: String,

    /// Protocol being used
    pub protocol: DiscoveryProtocol,

    /// When the session started
    pub started_at: DateTime<Utc>,

    /// Current status
    pub status: DiscoveryStatus,

    /// Progress percentage (0.0 - 100.0)
    pub progress_percent: f64,

    /// Number of candidates found
    pub candidates_found: u32,

    /// Resources used
    pub resources_used: ResourceUsage,

    /// Estimated completion time
    pub estimated_completion: Option<DateTime<Utc>>,
}

/// Discovery event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryEvent {
    /// Type of event
    pub event_type: DiscoveryEventType,

    /// Discovery session ID
    pub discovery_id: String,

    /// When the event occurred
    pub timestamp: DateTime<Utc>,

    /// Additional details
    pub details: HashMap<String, String>,
}

/// Discovery event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryEventType {
    /// Discovery started
    Started,

    /// Candidate found
    CandidateFound,

    /// Candidate validated
    CandidateValidated,

    /// Candidate rejected
    CandidateRejected,

    /// Discovery completed
    Completed,

    /// Discovery failed
    Failed,

    /// Discovery timeout
    Timeout,
}

// ============================================================
// Network Topology Types
// ============================================================

/// Network topology map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTopology {
    /// Known network nodes
    pub known_nodes: HashMap<String, NetworkNode>,

    /// Connection graph
    pub connection_graph: Vec<NetworkConnection>,

    /// When the topology was last updated
    pub last_updated: DateTime<Utc>,
}

/// Network node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNode {
    /// Node ID
    pub node_id: String,

    /// Endpoint address
    pub endpoint: String,

    /// Type of node
    pub node_type: String,

    /// Trust score
    pub trust_score: f64,

    /// When the node was last seen
    pub last_seen: DateTime<Utc>,
}

/// Network connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    /// Source node
    pub from_node: String,

    /// Target node
    pub to_node: String,

    /// Connection quality
    pub connection_quality: f64,

    /// Latency in milliseconds
    pub latency_ms: u64,

    /// Bandwidth in Mbps
    pub bandwidth_mbps: f64,
}

// ============================================================
// Assessment Types
// ============================================================

/// Biome assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeAssessment {
    /// Candidate ID being assessed
    pub candidate_id: String,

    /// Overall score (0.0 - 1.0)
    pub overall_score: f64,

    /// Trust score
    pub trust_score: f64,

    /// Security score
    pub security_score: f64,

    /// Performance score
    pub performance_score: f64,

    /// Compatibility score
    pub compatibility_score: f64,

    /// Genetic quality score
    pub genetic_quality_score: f64,

    /// Risk assessment
    pub risk_assessment: RiskAssessment,

    /// Recommendations
    pub recommendations: Vec<AssessmentRecommendation>,

    /// Final decision
    pub decision: AssessmentDecision,

    /// When the assessment was performed
    pub assessed_at: DateTime<Utc>,

    /// Who/what performed the assessment
    pub assessor: String,
}

/// Risk assessment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Overall risk level (0.0 - 1.0)
    pub overall_risk: f64,

    /// Security risk
    pub security_risk: f64,

    /// Operational risk
    pub operational_risk: f64,

    /// Compliance risk
    pub compliance_risk: f64,

    /// Reputation risk
    pub reputation_risk: f64,

    /// Identified risk factors
    pub risk_factors: Vec<RiskFactor>,

    /// Suggested mitigation strategies
    pub mitigation_strategies: Vec<String>,
}

/// Risk factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Factor name
    pub name: String,

    /// Category
    pub category: String,

    /// Severity (0.0 - 1.0)
    pub severity: f64,

    /// Description
    pub description: String,

    /// Likelihood (0.0 - 1.0)
    pub likelihood: f64,

    /// Potential impact
    pub impact: f64,
}

/// Assessment recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentRecommendation {
    /// Priority level
    pub priority: Priority,

    /// Recommendation description
    pub description: String,

    /// Rationale
    pub rationale: String,

    /// Implementation effort
    pub implementation_effort: EffortLevel,

    /// Expected benefit
    pub expected_benefit: f64,
}

/// Priority level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    /// Low priority
    Low,

    /// Medium priority
    Medium,

    /// High priority
    High,

    /// Critical
    Critical,
}

/// Effort level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffortLevel {
    /// Minimal effort
    Minimal,

    /// Low effort
    Low,

    /// Moderate effort
    Moderate,

    /// High effort
    High,

    /// Extensive effort
    Extensive,
}

/// Assessment decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssessmentDecision {
    /// Accept the biome
    Accept,

    /// Accept conditionally
    AcceptConditionally { conditions: Vec<String> },

    /// Under review
    UnderReview { review_deadline: DateTime<Utc> },

    /// Rejected
    Rejected { reason: String },

    /// Deferred
    Deferred { defer_until: DateTime<Utc> },
}

// ============================================================
// Discovered Biome Types
// ============================================================

/// Status of a discovered biome
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveredBiomeStatus {
    /// Newly discovered
    New,

    /// Under assessment
    Assessing,

    /// Pending registration
    PendingRegistration,

    /// Registered
    Registered,

    /// Quarantined
    Quarantined,

    /// Rejected
    Rejected,

    /// Stale (not seen recently)
    Stale,
}

/// A discovered biome with full state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredBiome {
    /// The candidate information
    pub candidate: BiomeCandidate,

    /// Assessment result (if performed)
    pub assessment: Option<BiomeAssessment>,

    /// Registration ID (if registered)
    pub registration: Option<String>,

    /// Current status
    pub current_status: DiscoveredBiomeStatus,

    /// When discovered
    pub discovery_timestamp: DateTime<Utc>,

    /// Last update timestamp
    pub last_updated: DateTime<Utc>,

    /// Notes/observations
    pub notes: Vec<String>,
}

// ============================================================
// Metrics Types
// ============================================================

/// Discovery metrics
#[derive(Debug)]
pub struct DiscoveryMetrics {
    /// Total discovery attempts
    pub total_discoveries: AtomicU64,

    /// Successful discoveries
    pub successful_discoveries: AtomicU64,

    /// Failed discoveries
    pub failed_discoveries: AtomicU64,

    /// Total biomes found
    pub biomes_found: AtomicU64,

    /// Successful assessments
    pub successful_assessments: AtomicU64,

    /// Failed assessments
    pub failed_assessments: AtomicU64,

    /// Successful registrations
    pub successful_registrations: AtomicU64,

    /// Failed registrations
    pub failed_registrations: AtomicU64,

    /// Average discovery time in milliseconds
    pub average_discovery_time_ms: AtomicU64,

    /// Network coverage percentage
    pub network_coverage_percent: AtomicU64,
}

impl DiscoveryMetrics {
    /// Create new discovery metrics
    pub fn new() -> Self {
        Self {
            total_discoveries: AtomicU64::new(0),
            successful_discoveries: AtomicU64::new(0),
            failed_discoveries: AtomicU64::new(0),
            biomes_found: AtomicU64::new(0),
            successful_assessments: AtomicU64::new(0),
            failed_assessments: AtomicU64::new(0),
            successful_registrations: AtomicU64::new(0),
            failed_registrations: AtomicU64::new(0),
            average_discovery_time_ms: AtomicU64::new(0),
            network_coverage_percent: AtomicU64::new(0),
        }
    }
}

impl Default for DiscoveryMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_config_default() {
        let config = DiscoveryConfig::default();
        assert_eq!(config.discovery_interval_seconds, 3600);
        assert_eq!(config.trust_threshold, 0.5);
        assert!(!config.auto_registration_enabled);
        assert!(config.quarantine_suspicious_biomes);
    }

    #[test]
    fn test_analysis_depth_default() {
        assert_eq!(AnalysisDepth::default(), AnalysisDepth::Standard);
    }

    #[test]
    fn test_discovery_metrics_new() {
        let metrics = DiscoveryMetrics::new();
        use std::sync::atomic::Ordering;
        assert_eq!(metrics.total_discoveries.load(Ordering::Relaxed), 0);
        assert_eq!(metrics.biomes_found.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
    }
}
