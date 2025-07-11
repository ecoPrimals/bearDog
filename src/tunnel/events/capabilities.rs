//! Peer capabilities and trust indicator definitions
//!
//! Contains structures for evaluating peer trustworthiness and capabilities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Peer capability information
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

/// Cryptographic proof of peer identity
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

/// Trust indicators for peer evaluation
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