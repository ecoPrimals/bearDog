//! # BSTP Events - BearDog Secure Tunnel Protocol Event System
//!
//! This module defines the critical communication interface between BearDog's security layer
//! and the Songbird network layer. These events enable real-time security adaptation and
//! threat response in the forest protection network.
//!
//! ## Forest Protection Communication Model
//!
//! BearDog protects the digital forest through a sophisticated event-driven architecture:
//!
//! ```text
//! ┌─────────────────┐    Events    ┌─────────────────┐
//! │  Songbird       │◄────────────►│   BearDog       │
//! │  Network Layer  │              │  Security Layer │
//! │                 │              │                 │
//! │ • Routing       │              │ • Threat Detection │
//! │ • Performance   │              │ • Access Control   │
//! │ • Optimization  │              │ • Compliance       │
//! └─────────────────┘              └─────────────────┘
//! ```
//!
//! ## Core Event Categories
//!
//! - **Network → Security**: `NetworkSecurityEvent` - Network alerts security layer about threats, performance changes, peer discoveries
//! - **Security → Network**: `SecurityNetworkEvent` - Security layer instructs network about routing restrictions, compliance requirements
//! - **Security Responses**: `SecurityResponse` - Security layer responses to network events with specific actions
//!
//! ## Example: Protecting Scientists from Threats
//!
//! ```rust,no_run
//! use beardog::tunnel::events::{NetworkSecurityEvent, SuspiciousActivityType, NetworkThreatLevel};
//!
//! // Songbird detects suspicious activity
//! let threat_event = NetworkSecurityEvent::SuspiciousActivity {
//!     source_peer: "unknown-node-123".to_string(),
//!     activity_type: SuspiciousActivityType::DataExfiltration,
//!     severity: NetworkThreatLevel::Critical,
//!     evidence: vec![/* threat evidence */],
//! };
//!
//! // BearDog responds by isolating the threat and protecting innocent forest explorers
//! ```
//!
//! ## Design Principles
//!
//! - **Proactive Protection**: Events enable preemptive threat response
//! - **Adaptive Security**: Security levels automatically adjust to threats
//! - **Democratic Access**: All forest explorers benefit from collective security intelligence
//! - **Academic Transparency**: Event structures are open for research and improvement

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Events that Songbird network layer sends to BearDog security layer
///
/// These events inform BearDog's protective systems about network conditions, peer behavior,
/// and potential threats. This enables real-time security adaptation to protect scientists
/// and forest explorers from digital threats.
///
/// # Event Categories
///
/// - **Peer Events**: Discovery and disconnection of network peers
/// - **Performance Events**: Network condition changes that may affect security
/// - **Threat Events**: Suspicious activities requiring immediate security response
/// - **Optimization Events**: Network improvements that may enable enhanced security
///
/// # Forest Protection Use Cases
///
/// - **Scientist Safety**: Detect peers trying to access restricted research data
/// - **Newcomer Protection**: Identify potential threats to naive forest explorers  
/// - **Collective Intelligence**: Share threat information across the protective network
/// - **Academic Integrity**: Protect sacred knowledge from unauthorized access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkSecurityEvent {
    /// New peer discovered in the forest - requires security verification
    ///
    /// When a new node joins the network, BearDog must evaluate whether it's safe
    /// for scientists and researchers to interact with this peer.
    PeerDiscovered {
        /// Unique identifier for the discovered peer
        peer_id: String,
        /// Security and performance capabilities of the peer
        peer_capabilities: PeerCapabilities,
        /// Trust indicators that help assess peer safety
        trust_indicators: Vec<TrustIndicator>,
    },

    /// Peer disconnected from the forest - security cleanup needed
    ///
    /// When peers leave, BearDog must clean up security state and assess
    /// whether the disconnection indicates a potential threat.
    PeerDisconnected {
        /// Unique identifier for the disconnected peer
        peer_id: String,
        /// Why the peer disconnected (helps detect attacks)
        reason: DisconnectReason,
        /// Whether this was an expected disconnection
        was_planned: bool,
    },

    /// Network performance change - may affect security capabilities
    ///
    /// Performance changes can impact security effectiveness. BearDog adapts
    /// protection levels based on available network resources.
    NetworkConditionChanged {
        /// Current network latency in milliseconds
        latency_ms: u64,
        /// Percentage of packets being lost
        packet_loss_percent: f64,
        /// Available bandwidth in megabits per second
        bandwidth_mbps: u64,
        /// Network timing variance in milliseconds
        jitter_ms: u64,
    },

    /// Suspicious network activity detected - immediate protection needed
    ///
    /// The most critical event type - indicates potential threats to forest
    /// inhabitants requiring immediate defensive action.
    SuspiciousActivity {
        /// Peer that appears to be the source of suspicious activity
        source_peer: String,
        /// Type of suspicious behavior detected
        activity_type: SuspiciousActivityType,
        /// How severe the threat appears to be
        severity: NetworkThreatLevel,
        /// Evidence supporting the threat assessment
        evidence: Vec<NetworkEvidence>,
    },

    /// Network route optimization completed - may enable better security
    ///
    /// Route improvements can provide opportunities for enhanced security
    /// through better performance or reduced attack surface.
    RouteOptimized {
        /// Identifier for the optimized tunnel
        tunnel_id: String,
        /// Previous latency before optimization
        old_latency_ms: u64,
        /// New improved latency after optimization
        new_latency_ms: u64,
        /// Type of optimization that was performed
        optimization_type: OptimizationType,
    },
}

/// Events that BearDog security layer sends to Songbird network layer
///
/// These events allow BearDog to direct network behavior based on security requirements,
/// ensuring that routing and communication decisions support forest protection goals.
///
/// # Security-Driven Network Control
///
/// BearDog can influence network behavior to:
/// - Route traffic away from compromised peers
/// - Enforce compliance requirements for different geographic regions
/// - Adapt network performance to security posture changes
/// - Coordinate threat response across the protective network
///
/// # Example: Protecting Academic Collaborations
///
/// When researchers from different institutions collaborate, BearDog ensures their
/// communications are routed through secure channels and comply with institutional
/// security policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityNetworkEvent {
    /// Security session established - network can now route protected traffic
    ///
    /// Notifies the network layer that BearDog has established a secure communication
    /// channel with a peer, enabling safe data exchange for forest inhabitants.
    SessionEstablished {
        /// Unique identifier for the established security session
        session_id: String,
        /// Peer that the session was established with
        peer_id: String,
        /// Level of security protection provided by this session
        security_level: SecurityLevel,
        /// Optional bandwidth limit to prevent resource exhaustion attacks
        bandwidth_limit: Option<u64>,
    },

    /// Security threat detected - network may need to adjust routing
    ///
    /// Alerts the network layer about threats that may require routing changes
    /// to protect vulnerable forest explorers from malicious actors.
    ThreatDetected {
        /// Severity of the detected threat
        threat_level: ThreatLevel,
        /// Peer that appears to be affected by or source of the threat
        affected_peer: String,
        /// BearDog's recommendation for how the network should respond
        recommended_action: ThreatMitigationAction,
    },

    /// Security upgrade applied - network performance characteristics may change
    ///
    /// Informs the network that BearDog has upgraded security protections,
    /// which may affect throughput, latency, or routing preferences.
    SecurityUpgraded {
        /// Session that received the security upgrade
        session_id: String,
        /// Previous security level before upgrade
        old_security_level: SecurityLevel,
        /// New enhanced security level after upgrade
        new_security_level: SecurityLevel,
        /// Expected impact on network performance
        performance_impact: PerformanceImpact,
    },

    /// Compliance requirement imposed - routing behavior must be adjusted
    ///
    /// Notifies the network about regulatory or policy requirements that
    /// restrict how traffic can be routed to protect academic integrity
    /// and comply with institutional policies.
    ComplianceRequirement {
        /// Type of compliance requirement (GDPR, HIPAA, institutional policy, etc.)
        requirement_type: ComplianceType,
        /// Geographic regions affected by this requirement
        affected_regions: Vec<GeographicRegion>,
        /// Specific routing restrictions that must be enforced
        routing_restrictions: Vec<RoutingRestriction>,
    },
}

/// Security responses from BearDog to network events
///
/// These responses represent BearDog's immediate reactions to network events,
/// providing the network layer with specific instructions for maintaining
/// forest protection while the security layer processes the event fully.
///
/// # Response Philosophy
///
/// BearDog responses prioritize:
/// - **Immediate Safety**: Quick protective actions for forest inhabitants
/// - **Proportional Response**: Security measures match threat levels  
/// - **Minimal Disruption**: Maintain access for legitimate forest explorers
/// - **Adaptive Learning**: Responses improve based on threat patterns
///
/// # Example Response Workflow
///
/// ```text
/// Network Event → BearDog Analysis → Immediate Response → Full Security Processing
///      ↓                ↓                    ↓                     ↓
/// Peer discovered → Trust assessment → Session created → Background verification
/// ```
#[derive(Debug, Clone)]
pub enum SecurityResponse {
    /// Security session created successfully
    ///
    /// Confirms that BearDog has established secure communication with a peer
    /// and the network can begin routing protected traffic.
    SessionCreated {
        /// Unique identifier for the new security session
        session_id: String,
        /// Peer that the session was created with
        peer_id: String,
        /// Level of security protection established
        security_level: SecurityLevel,
    },

    /// Security session terminated
    ///
    /// Notifies that BearDog has ended secure communication with a peer,
    /// requiring the network to stop routing traffic to that destination.
    SessionTerminated {
        /// Peer whose session was terminated
        peer_id: String,
    },

    /// Security level automatically adapted to changing conditions
    ///
    /// Indicates that BearDog has adjusted protection levels in response
    /// to changing threat conditions or performance requirements.
    SecurityAdapted {
        /// Explanation of why security adaptation was necessary
        reason: String,
        /// New security level after adaptation
        new_security_level: SecurityLevel,
    },

    /// Threat detected requiring immediate protective action
    ///
    /// Urgent notification that BearDog has identified a threat requiring
    /// immediate network-level response to protect forest inhabitants.
    ThreatDetected {
        /// Severity of the detected threat
        threat_level: ThreatLevel,
        /// Peer that appears to be the source of the threat
        source_peer: String,
        /// Recommended immediate action for the network layer
        recommended_action: ThreatMitigationAction,
    },
}

// Supporting types for BSTP event system

/// Comprehensive capability profile for forest network peers
///
/// PeerCapabilities represent what a peer can offer to the forest protection network
/// and help BearDog assess whether the peer is trustworthy and valuable for protecting
/// scientists and researchers.
///
/// # Capability Assessment for Forest Protection
///
/// BearDog uses peer capabilities to:
/// - **Trust Evaluation**: Determine if a peer can be trusted with sensitive research data
/// - **Resource Planning**: Understand what protective services the peer can provide
/// - **Security Matching**: Pair peers with compatible security requirements
/// - **Load Distribution**: Balance protective workload across capable peers
///
/// # Example: Research Collaboration Assessment
///
/// ```rust,no_run
/// use beardog::tunnel::events::PeerCapabilities;
///
/// // University research node joining the forest
/// let university_peer = PeerCapabilities {
///     identity_proof: Some(university_certificate),
///     gaming_profile: None, // Academic institution, not gaming-focused
///     supported_crypto: vec!["AES-256-GCM".to_string(), "ChaCha20-Poly1305".to_string()],
///     max_bandwidth: 10_000_000, // 10 Mbps for research data transfer
///     latency_tolerance: std::time::Duration::from_millis(100),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerCapabilities {
    /// Cryptographic proof of peer identity and legitimacy
    ///
    /// Essential for academic environments where identity verification
    /// protects against research espionage and data theft.
    pub identity_proof: Option<CryptographicProof>,

    /// Gaming-specific security profile if peer supports gaming workloads
    ///
    /// Used for peers that provide gaming-grade crypto performance,
    /// enabling high-throughput protection for data-intensive research.
    pub gaming_profile: Option<crate::tunnel::GamingSecurityProfile>,

    /// List of cryptographic algorithms supported by this peer
    ///
    /// Enables security-level matching between peers for optimal protection
    /// of different types of research data and academic communications.
    pub supported_crypto: Vec<String>,

    /// Maximum bandwidth this peer can provide (in bytes per second)
    ///
    /// Critical for assessing whether a peer can handle large research datasets
    /// while maintaining security protection levels.
    pub max_bandwidth: u64,

    /// How much network latency this peer can tolerate while maintaining security
    ///
    /// Some peers require low-latency for real-time research collaboration,
    /// while others can tolerate higher latency for better security.
    pub latency_tolerance: std::time::Duration,
}

/// Cryptographic proof of peer identity and capabilities
///
/// This structure contains the cryptographic evidence that a peer is who they
/// claim to be and can provide the security capabilities they advertise.
/// Essential for protecting academic integrity and preventing research espionage.
///
/// # Academic Security Requirements
///
/// In academic environments, identity proof helps ensure:
/// - **Research Integrity**: Collaborators are legitimate academic institutions
/// - **Data Protection**: Sensitive research data goes only to authorized peers
/// - **Intellectual Property**: Patent-worthy discoveries are protected from theft
/// - **Grant Compliance**: Research funding requirements for data security are met
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptographicProof {
    /// Unique identifier for the node providing this proof
    pub node_id: String,

    /// Digital signature proving the peer controls their claimed identity
    pub signature: Vec<u8>,

    /// Public key that can verify the signature and enable secure communication
    pub public_key: Vec<u8>,

    /// When this proof was generated (prevents replay attacks)
    pub timestamp: SystemTime,

    /// Additional capability claims that can be cryptographically verified
    pub capabilities: HashMap<String, String>,
}

/// Trust indicators that help assess peer safety for forest inhabitants
///
/// These indicators help BearDog quickly assess whether a peer is likely to be
/// safe for scientists, researchers, and newcomers to interact with.
///
/// # Trust Assessment for Academic Safety
///
/// Trust indicators enable rapid decisions about peer safety:
/// - **LocalNetworkPeer**: Same institution, likely safe for internal collaboration
/// - **KnownGoodPeer**: Previously verified as safe through successful interactions
/// - **CertificateVerified**: Has valid institutional or certificate authority credentials
/// - **ReputationGood**: Other forest peers report positive interactions
/// - **GeographicallyClose**: Reduced risk of international data interception
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustIndicator {
    /// Peer is on the same local network (university campus, research facility)
    LocalNetworkPeer,

    /// Peer has been previously verified as trustworthy and beneficial
    KnownGoodPeer,

    /// Peer's identity certificate has been cryptographically verified
    CertificateVerified,

    /// Peer has a good reputation based on community feedback
    ReputationGood,

    /// Peer is geographically close, reducing interception risks
    GeographicallyClose,
}

/// Reasons why a peer disconnected from the forest network
///
/// Understanding disconnection reasons helps BearDog distinguish between
/// normal network events and potential security incidents that threaten
/// forest inhabitants.
///
/// # Security Implications
///
/// - **NetworkError/Timeout**: Usually benign, may indicate poor connectivity
/// - **SecurityBreach**: Serious - peer may have been compromised
/// - **UserRequested**: Normal - user chose to disconnect
/// - **ResourceLimit**: May indicate DoS attack or resource exhaustion
/// - **Unknown**: Requires investigation - could indicate attack
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

/// Types of suspicious network activity that threaten forest inhabitants
///
/// These activity types help BearDog classify different categories of threats
/// so that appropriate protective measures can be applied to safeguard
/// scientists, researchers, and newcomers exploring the digital forest.
///
/// # Threat Categories for Forest Protection
///
/// - **Connection Attacks**: Attempt to overwhelm or infiltrate the network
/// - **Traffic Analysis**: Unusual patterns that may indicate surveillance or attacks
/// - **Authentication Attacks**: Attempts to gain unauthorized access to research data
/// - **Data Theft**: Active attempts to steal sacred scientific knowledge
/// - **Network Reconnaissance**: Mapping the network for future attacks
///
/// # Example: Research Data Protection
///
/// ```rust,no_run
/// use beardog::tunnel::events::SuspiciousActivityType;
///
/// // Detecting potential research espionage
/// let threat = SuspiciousActivityType::DataExfiltration;
/// // BearDog immediately isolates the threat and protects researchers
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuspiciousActivityType {
    /// Peer is making too many simultaneous connections
    ///
    /// May indicate a DDoS attack or an attempt to overwhelm the network
    /// to disrupt scientific research and collaboration.
    ExcessiveConnections,

    /// Traffic patterns don't match normal research or collaboration activities
    ///
    /// Could indicate surveillance, data exfiltration, or attempts to map
    /// the research network for future attacks.
    UnusualTrafficPattern,

    /// Multiple failed authentication attempts
    ///
    /// Suggests attempts to gain unauthorized access to research data,
    /// intellectual property, or confidential academic communications.
    FailedAuthentication,

    /// Evidence of unauthorized data extraction
    ///
    /// The most serious threat - active attempts to steal research data,
    /// scientific discoveries, or confidential academic information.
    DataExfiltration,

    /// Systematic probing of network services and ports
    ///
    /// Reconnaissance activity that typically precedes more serious attacks
    /// on academic institutions and research facilities.
    PortScanning,

    /// Suspicious activity of unknown or unclassified type
    ///
    /// Novel threats that don't fit existing patterns but show
    /// concerning characteristics requiring investigation.
    Unknown,
}

/// Severity levels for network threats in the forest protection system
///
/// NetworkThreatLevel helps BearDog prioritize protective responses and
/// allocate security resources appropriately to defend forest inhabitants
/// from digital threats of varying severity.
///
/// # Threat Severity for Academic Protection
///
/// - **Critical**: Immediate danger to research data or institutional security
/// - **High**: Serious threat requiring rapid response but not immediately catastrophic
/// - **Medium**: Notable threat that should be addressed but allows time for planning
/// - **Low**: Minor threat or potential threat requiring monitoring
///
/// # Response Escalation
///
/// Higher threat levels trigger more aggressive protective measures,
/// but BearDog balances security with accessibility for legitimate forest explorers.
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

/// Evidence supporting threat detection and analysis
///
/// NetworkEvidence provides the forensic foundation for BearDog's protective
/// decisions, ensuring that defensive actions are based on solid evidence
/// rather than false positives that could disrupt legitimate research.
///
/// # Evidence-Based Forest Protection
///
/// BearDog's evidence system enables:
/// - **Accurate Threat Assessment**: Distinguishing real threats from normal research activity
/// - **Academic Transparency**: Providing clear reasons for security decisions
/// - **Continuous Learning**: Improving threat detection through evidence analysis
/// - **Legal Compliance**: Maintaining audit trails for institutional security requirements
///
/// # Example: Research Data Protection Evidence
///
/// ```rust,no_run
/// use beardog::tunnel::events::NetworkEvidence;
/// use std::collections::HashMap;
///
/// // Evidence of potential research espionage
/// let mut evidence_data = HashMap::new();
/// evidence_data.insert("accessed_files".to_string(), "research_proposal_2024.pdf".to_string());
/// evidence_data.insert("access_time".to_string(), "outside_normal_hours".to_string());
///
/// let evidence = NetworkEvidence {
///     evidence_type: "unauthorized_file_access".to_string(),
///     data: evidence_data,
///     timestamp: SystemTime::now(),
///     confidence: 0.85, // 85% confidence this is malicious
/// };
/// ```
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

/// Types of network optimizations that can enhance forest protection
///
/// OptimizationType categorizes different performance improvements that can
/// be applied to strengthen BearDog's protective capabilities and ensure
/// optimal security for forest inhabitants.
///
/// # Optimization Goals for Academic Protection
///
/// - **Performance**: Reduce latency for real-time research collaboration
/// - **Capacity**: Maximize throughput for large dataset transfers
/// - **Reliability**: Ensure consistent protection through load balancing
/// - **Resilience**: Maintain protection during network failures
///
/// # Example: Research Collaboration Optimization
///
/// When researchers need low-latency communication for real-time data analysis,
/// LatencyOptimization ensures their secure collaboration remains responsive.
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

/// Security protection levels available in the BearDog forest protection system
///
/// SecurityLevel represents the intensity of cryptographic and behavioral protections
/// applied to safeguard different types of academic activities and research data.
/// Higher levels provide stronger protection but may impact performance.
///
/// # Security Level Guidelines for Academics
///
/// - **Ultimate**: Maximum protection for the most sensitive research (patents, classified research)
/// - **High**: Strong protection for confidential research data and institutional communications
/// - **Medium**: Balanced protection for general academic work and collaboration
/// - **Low**: Basic protection for public or non-sensitive academic communications
/// - **Adaptive**: Automatically adjusts protection based on threat conditions
/// - **Optimized**: Performance-optimized protection for gaming-grade crypto workloads
///
/// # Example: Research Data Classification
///
/// ```rust,no_run
/// use beardog::tunnel::events::SecurityLevel;
///
/// // Patent application data requires maximum protection
/// let patent_security = SecurityLevel::Ultimate;
///
/// // General research collaboration can use adaptive protection
/// let collaboration_security = SecurityLevel::Adaptive;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Threat severity levels for coordinating protective responses
///
/// ThreatLevel helps BearDog coordinate protective responses across the forest
/// protection network, ensuring that all inhabitants receive appropriate
/// security coverage based on the current threat environment.
///
/// # Threat Response Coordination
///
/// - **Critical**: Network-wide emergency protection, isolation of threats
/// - **High**: Enhanced monitoring, elevated security for vulnerable targets
/// - **Medium**: Increased awareness, targeted protection for affected areas
/// - **Low**: Standard monitoring with documented awareness
///
/// # Example: Research Institution Protection
///
/// During a Critical threat, all research institutions in the forest network
/// automatically receive enhanced protection to prevent data theft or espionage.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Protective actions BearDog can recommend to defend the forest from threats
///
/// ThreatMitigationAction represents the specific defensive measures that BearDog
/// can coordinate with the network layer to protect scientists, researchers, and
/// other forest inhabitants from digital threats.
///
/// # Defensive Strategy Philosophy
///
/// BearDog's mitigation actions follow these principles:
/// - **Proportional Response**: Match defensive intensity to threat severity
/// - **Minimal Disruption**: Maintain access for legitimate forest explorers
/// - **Adaptive Protection**: Actions that evolve based on threat patterns
/// - **Collaborative Defense**: Coordinate protection across the network
///
/// # Example: Research Data Protection
///
/// ```rust,no_run
/// use beardog::tunnel::events::ThreatMitigationAction;
///
/// // Protecting against research espionage
/// let action = ThreatMitigationAction::IsolateTraffic;
/// // Isolates the threat while maintaining access for legitimate researchers
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatMitigationAction {
    /// Isolate suspicious traffic while maintaining normal forest operations
    ///
    /// Creates a protective barrier around suspicious activity without
    /// disrupting legitimate research and academic collaboration.
    IsolateTraffic,

    /// Enhance security levels for affected sessions or network segments
    ///
    /// Applies stronger cryptographic protection and stricter access controls
    /// to defend against elevated threats while preserving functionality.
    UpgradeSecurity,

    /// Immediately terminate compromised or suspicious sessions
    ///
    /// Emergency action to prevent active threats from spreading or
    /// accessing sensitive research data and academic resources.
    TerminateSession,

    /// Increase monitoring and surveillance of suspicious activities
    ///
    /// Enhanced observation to gather intelligence about threats while
    /// allowing continued operation under heightened awareness.
    MonitorClosely,

    /// Redirect traffic away from compromised or dangerous network paths
    ///
    /// Maintains connectivity while avoiding compromised infrastructure
    /// that could threaten research data or academic communications.
    RerouteTraffic,

    /// Strengthen authentication requirements for affected resources
    ///
    /// Adds additional identity verification steps to prevent unauthorized
    /// access to sensitive research data and academic systems.
    IncreaseAuthentication,
}

/// Performance impact levels for security operations
///
/// PerformanceImpact helps forest inhabitants understand how security measures
/// will affect their research activities, enabling informed decisions about
/// balancing protection with productivity.
///
/// # Impact Assessment for Academic Work
///
/// - **Negligible**: Security enhancement with minimal performance effect
/// - **Low**: Slight performance impact, suitable for most research activities
/// - **Medium**: Noticeable impact, may affect real-time collaboration
/// - **High**: Significant impact, prioritizes security over performance
/// - **Severe**: Emergency security measures with substantial performance cost
///
/// # Balancing Security and Productivity
///
/// BearDog strives to provide maximum protection with minimal disruption to
/// legitimate academic work and scientific collaboration.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Regulatory and institutional compliance requirements for academic institutions
///
/// ComplianceType represents the various legal, regulatory, and institutional
/// requirements that govern how research data and academic communications
/// must be protected and handled.
///
/// # Academic Compliance Landscape
///
/// - **GDPR**: European data protection for international research collaboration
/// - **HIPAA**: Healthcare data protection for medical and biological research
/// - **SOX**: Financial data protection for economic and business research
/// - **Data Sovereignty**: National requirements for research data residency
/// - **Export Control**: Restrictions on sharing sensitive research internationally
///
/// # Example: International Research Collaboration
///
/// ```rust,no_run
/// use beardog::tunnel::events::ComplianceType;
///
/// // Ensuring European research data stays compliant
/// let requirement = ComplianceType::GDPR;
/// // BearDog ensures data handling meets European privacy standards
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceType {
    /// General Data Protection Regulation (European Union)
    ///
    /// Protects personal data in research involving European subjects
    /// or institutions, ensuring privacy rights for international collaboration.
    GDPR,

    /// Health Insurance Portability and Accountability Act (United States)
    ///
    /// Protects healthcare data in medical research, ensuring patient
    /// privacy in biological and health-related academic studies.
    HIPAA,

    /// Sarbanes-Oxley Act (United States)
    ///
    /// Protects financial data in economic research, ensuring proper
    /// handling of financial information in business and economic studies.
    SOX,

    /// National data sovereignty requirements
    ///
    /// Ensures research data remains within specific national boundaries
    /// to comply with data residency laws and protect national interests.
    DataSovereignty,

    /// Export control restrictions on sensitive research
    ///
    /// Prevents unauthorized international sharing of research that could
    /// have national security implications or dual-use applications.
    ExportControl,
}

/// Geographic regions for data residency and compliance requirements
///
/// GeographicRegion helps BearDog enforce data sovereignty requirements and
/// ensure that sensitive research data complies with regional regulations
/// and institutional policies.
///
/// # Regional Considerations for Academic Collaboration
///
/// - **EU**: GDPR compliance, European research framework participation
/// - **US**: HIPAA/SOX compliance, federal research grant requirements
/// - **China/Russia**: Data sovereignty concerns, export control restrictions
/// - **Other**: Emerging regions with specific academic collaboration requirements
///
/// # Example: Protecting European Research Data
///
/// European research institutions may require that personal data in studies
/// never leaves EU jurisdiction to comply with GDPR privacy requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeographicRegion {
    /// European Union and associated regions
    ///
    /// Includes EU member states and regions following European
    /// data protection and academic collaboration frameworks.
    EU,

    /// United States and territories
    ///
    /// Includes regions following US regulatory frameworks and
    /// federal research compliance requirements.
    US,

    /// China and associated regions
    ///
    /// Requires careful handling due to data sovereignty laws and
    /// potential export control restrictions on research collaboration.
    China,

    /// Russia and associated regions
    ///
    /// Requires special consideration for geopolitical tensions and
    /// restrictions on academic and research collaboration.
    Russia,

    /// Other geographic regions not explicitly categorized
    ///
    /// Covers emerging regions with specific requirements for
    /// academic collaboration and research data handling.
    Other(String),
}

/// Routing restrictions that ensure compliance and protect sensitive research
///
/// RoutingRestriction defines specific network behavior requirements that
/// BearDog enforces to protect academic integrity, comply with regulations,
/// and safeguard sensitive research data.
///
/// # Academic Protection Through Routing Control
///
/// - **Storage Restrictions**: Prevent sensitive data from reaching uncontrolled storage
/// - **Geographic Restrictions**: Ensure data stays within required jurisdictions
/// - **Security Requirements**: Mandate encryption and audit trails for sensitive research
/// - **Residency Controls**: Keep data within specific geographic or institutional boundaries
///
/// # Example: Medical Research Protection
///
/// Medical research involving patient data might require NoInternationalRouting
/// and AuditTrailRequired to comply with HIPAA and institutional ethics requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingRestriction {
    /// Prevent data from being stored in external cloud services
    ///
    /// Ensures sensitive research data remains on controlled infrastructure
    /// rather than potentially unsecured third-party cloud storage.
    NoCloudStorage,

    /// Keep all traffic within national or regional boundaries
    ///
    /// Prevents research data from crossing international borders
    /// to comply with data sovereignty and export control requirements.
    NoInternationalRouting,

    /// Mandate encryption for all data transmission
    ///
    /// Ensures that sensitive research data is cryptographically protected
    /// during transmission to prevent interception or tampering.
    EncryptionRequired,

    /// Require detailed logging of all data access and transmission
    ///
    /// Creates audit trails for sensitive research data to meet compliance
    /// requirements and enable forensic analysis if needed.
    AuditTrailRequired,

    /// Restrict data to specific geographic regions
    ///
    /// Ensures research data remains within specified jurisdictions
    /// to comply with data residency laws and institutional policies.
    DataResidency(GeographicRegion),
}

// Additional types for BSTP security

/// Types of security evolution that can occur in the forest protection network
///
/// SecurityEvolution represents the ways that BearDog's protective capabilities
/// can improve and adapt over time to provide better protection for scientists
/// and researchers while maintaining optimal performance.
///
/// # Evolutionary Improvement Categories
///
/// - **Cryptographic**: Enhanced encryption and security algorithms
/// - **Performance**: Optimized security with improved speed and efficiency
/// - **Adaptive**: Intelligence-driven responses to emerging threats
/// - **Genetic**: Hybrid evolution combining multiple protective improvements
///
/// # Example: Adaptive Research Protection
///
/// During a period of increased cyber threats targeting academic institutions,
/// BearDog might evolve through ThreatAdapted improvements to provide enhanced
/// protection specifically designed for the current threat landscape.
#[derive(Debug, Clone, PartialEq)]
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

/// Comprehensive security performance metrics for forest protection assessment
///
/// SecurityMetrics provides quantitative assessment of BearDog's protective
/// effectiveness, helping administrators and researchers understand the
/// current security posture and performance characteristics.
///
/// # Metrics for Academic Security Assessment
///
/// - **Encryption Latency**: How quickly sensitive data can be protected
/// - **Threat Level**: Current security environment assessment
/// - **Performance Score**: Overall effectiveness of protective measures
///
/// # Example: Research Infrastructure Assessment
///
/// ```rust,no_run
/// use beardog::tunnel::events::{SecurityMetrics, ThreatLevel};
/// use std::time::Duration;
///
/// // Assessing protection for a research data center
/// let metrics = SecurityMetrics {
///     encryption_latency: Duration::from_millis(2), // 2ms encryption overhead
///     threat_level: ThreatLevel::Medium,            // Current threat environment
///     performance_score: 0.95,                     // 95% efficiency rating
/// };
/// ```
#[derive(Debug, Clone)]
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
