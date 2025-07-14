//! Core event types and enumerations
//!
//! Contains the fundamental data types used across all event categories.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Security levels for BearDog operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityLevel {
    /// Maximum security protection for the most sensitive academic data
    ///
    /// Applied to patent applications, classified research, sensitive institutional data,
    /// and other materials requiring the highest level of protection.
    Ultimate,

    /// Strong security protection for confidential research and institutional communications
    ///
    /// Suitable for unpublished research, grant applications, and internal
    /// academic communications requiring enhanced confidentiality.
    High,

    /// Balanced security protection for general academic work and collaboration
    ///
    /// Appropriate for most academic activities, published research,
    /// and general collaboration between institutions.
    Medium,

    /// Basic security protection for public or low-sensitivity communications
    ///
    /// Used for public academic communications, published papers,
    /// and activities where performance is prioritized over maximum security.
    Low,

    /// Security level automatically adapts to current threat conditions
    ///
    /// Intelligent protection that increases security during threats and
    /// optimizes performance during normal conditions.
    Adaptive,

    /// Performance-optimized security for high-throughput research workloads
    ///
    /// Gaming-grade crypto performance for data-intensive research like
    /// machine learning, simulation, and real-time data processing.
    Optimized,
}

/// Threat severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreatLevel {
    /// Immediate network-wide threat requiring emergency protective measures
    ///
    /// Active attacks on multiple institutions, widespread malware,
    /// or coordinated espionage attempts targeting research data.
    Critical,

    /// Serious threat requiring elevated security and enhanced monitoring
    ///
    /// Targeted attacks on specific institutions, advanced persistent threats,
    /// or evidence of coordinated surveillance of academic networks.
    High,

    /// Notable threat requiring increased awareness and targeted protection
    ///
    /// Emerging threats, suspicious activities, or isolated incidents
    /// that could escalate but don't require emergency response.
    Medium,

    /// Minor threat requiring documentation and standard monitoring
    ///
    /// Low-level suspicious activities, potential threats, or normal
    /// security events that should be tracked but don't require action.
    Low,
}

/// Network threat severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkThreatLevel {
    /// Immediate danger requiring emergency protective measures
    ///
    /// Active attacks on research data, institutional security breaches,
    /// or threats to the safety of forest inhabitants.
    Critical,

    /// Serious threat requiring rapid response and enhanced security
    ///
    /// Significant attempts to compromise security that could lead to
    /// research data loss or disruption of academic collaboration.
    High,

    /// Notable threat requiring attention and monitoring
    ///
    /// Concerning activity that poses risks but allows time for
    /// measured protective responses.
    Medium,

    /// Minor threat or suspicious activity requiring basic monitoring
    ///
    /// Low-level concerns that should be tracked but don't require
    /// immediate defensive action.
    Low,
}

/// Performance impact levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PerformanceImpact {
    /// Security enhancement with essentially no performance degradation
    ///
    /// Ideal for continuous protection that doesn't interfere with
    /// research productivity or academic collaboration.
    Negligible,

    /// Minor performance impact acceptable for most research activities
    ///
    /// Slight overhead that maintains full functionality for typical
    /// academic work including data analysis and collaboration.
    Low,

    /// Noticeable performance impact that may affect real-time activities
    ///
    /// Moderate overhead that could slow interactive research tools
    /// but maintains full functionality for most academic tasks.
    Medium,

    /// Significant performance impact prioritizing security over speed
    ///
    /// Substantial overhead justified by serious threats to research
    /// data or institutional security requiring enhanced protection.
    High,

    /// Severe performance impact for emergency security situations
    ///
    /// Emergency protection measures that significantly slow operations
    /// but are necessary to prevent catastrophic security breaches.
    Severe,
}

/// Security evolution types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEvolution {
    /// Cryptographic algorithms and security protocols have been upgraded
    ///
    /// Enhanced encryption, stronger authentication, or improved security
    /// mechanisms that provide better protection for research data.
    CryptoUpgraded,

    /// Security performance has been optimized for better efficiency
    ///
    /// Improvements that maintain or enhance protection levels while
    /// reducing performance impact on research activities.
    PerformanceOptimized,

    /// Security capabilities have adapted to address specific threats
    ///
    /// Targeted improvements designed to counter emerging threats
    /// to academic institutions and research networks.
    ThreatAdapted,

    /// Genetic algorithms have produced hybrid security improvements
    ///
    /// Novel security capabilities that combine multiple evolutionary
    /// improvements to provide comprehensive protection enhancement.
    GeneticHybridEvolved,
}

/// Security metrics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    /// Time required to encrypt data for protection
    ///
    /// Measures the performance impact of cryptographic protection,
    /// helping balance security with research productivity requirements.
    pub encryption_latency: std::time::Duration,

    /// Current threat environment assessment
    ///
    /// Indicates the severity of threats currently facing the forest
    /// protection network and research community.
    pub threat_level: ThreatLevel,

    /// Overall security performance effectiveness (0.0 to 1.0)
    ///
    /// Comprehensive score representing how well the security system
    /// is protecting research data while maintaining usability.
    pub performance_score: f64,
}

/// Network evidence structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEvidence {
    /// Category or type of evidence collected
    ///
    /// Examples: "traffic_pattern", "authentication_failure", "file_access", "network_scan"
    pub evidence_type: String,

    /// Structured data containing the specific evidence details
    ///
    /// Key-value pairs containing relevant information like timestamps,
    /// file names, IP addresses, or behavioral patterns.
    pub data: HashMap<String, String>,

    /// When this evidence was collected
    ///
    /// Essential for correlating evidence across time and understanding
    /// the timeline of potential threats.
    pub timestamp: SystemTime,

    /// Confidence level that this evidence indicates malicious activity (0.0 to 1.0)
    ///
    /// Helps BearDog weight evidence appropriately and avoid false positives
    /// that could disrupt legitimate academic work.
    pub confidence: f64,
}

/// Network optimization types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationType {
    /// Network routes optimized to minimize communication delay
    ///
    /// Critical for real-time research collaboration, video conferences,
    /// and interactive data analysis that requires immediate responses.
    LatencyOptimization,

    /// Network capacity optimized to maximize data transfer speeds
    ///
    /// Essential for transferring large research datasets, genomic data,
    /// astronomical observations, and other high-volume scientific data.
    ThroughputOptimization,

    /// Traffic distributed across multiple paths to prevent bottlenecks
    ///
    /// Ensures consistent performance for research activities even during
    /// peak usage periods or when serving many forest inhabitants.
    LoadBalancing,

    /// Network automatically recovers from failures to maintain protection
    ///
    /// Provides continuous security coverage even when individual network
    /// components fail, ensuring research work isn't disrupted.
    FailoverRecovery,
}

/// Disconnect reasons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisconnectReason {
    /// Network connectivity problems caused the disconnection
    NetworkError,

    /// Security violation detected, peer was forcibly disconnected
    SecurityBreach,

    /// User or administrator requested the disconnection
    UserRequested,

    /// Connection timed out due to inactivity
    Timeout,

    /// Peer exceeded resource limits (bandwidth, connections, etc.)
    ResourceLimit,

    /// Disconnection reason could not be determined
    Unknown,
}
