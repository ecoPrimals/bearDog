// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Capabilities advertised by a tunnel endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelCapabilities {
    /// Optional proof
    pub proof: Option<CryptographicProof>,
    /// Optional gaming profile
    pub gaming_profile: Option<crate::tunnel::GamingSecurityProfile>,
    /// Collection of supported crypto
    pub supported_crypto: Vec<String>,
    /// Maximum bandwidth in bytes per second
    pub max_bandwidth: u64,
    /// The latency tolerance value
    pub latency_tolerance: std::time::Duration,
}

/// Cryptographic proof of tunnel capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptographicProof {
    /// The algorithm value
    pub algorithm: String,
    /// Collection of signature
    pub signature: Vec<u8>,
    /// Collection of public key
    pub public_key: Vec<u8>,
    /// When the proof was generated
    pub timestamp: SystemTime,
    /// Mapping of capabilities
    pub capabilities: HashMap<String, String>,
}

/// Trust indicators for peer evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustIndicator {
    /// Represents local network peer variant
    LocalNetworkPeer,
    /// Represents known good peer variant
    KnownGoodPeer,
    /// State indicating certificateverified
    CertificateVerified,
    /// Represents reputation good variant
    ReputationGood,
    /// Represents geographically close variant
    GeographicallyClose,
}
