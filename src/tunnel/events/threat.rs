//! Threat detection and mitigation events
//!
//! Contains threat-related types and mitigation actions.

use serde::{Deserialize, Serialize};

/// Types of suspicious network activities
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

/// Threat mitigation actions
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
