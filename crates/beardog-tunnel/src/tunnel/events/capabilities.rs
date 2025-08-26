

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerCapabilities {

    pub identity_proof: Option<CryptographicProof>,

    pub gaming_profile: Option<crate::tunnel::GamingSecurityProfile>,

    pub supported_crypto: Vec<String>,

    pub max_bandwidth: u64,

    pub latency_tolerance: std::time::Duration,
}

pub struct CryptographicProof {

    pub node_id: String,

    pub signature: Vec<u8>,

    pub public_key: Vec<u8>,

    pub timestamp: SystemTime,

    pub capabilities: HashMap<String, String>,

pub enum TrustIndicator {

    LocalNetworkPeer,

    KnownGoodPeer,

    CertificateVerified,

    ReputationGood,

    GeographicallyClose,
