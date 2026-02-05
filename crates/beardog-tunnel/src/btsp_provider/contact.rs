//! Contact Exchange - Genetic Lineage-Based NAT Traversal
//!
//! This module defines types for decentralized NAT traversal using genetic lineage
//! relationships. The actual implementation lives in `BeardogBtspProvider`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// =============================================================================
// Contact Exchange Types
// =============================================================================

/// Contact information for a peer (decentralized NAT traversal)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    /// Peer identifier
    pub peer_id: String,
    /// Peer addresses (IP:Port combinations)
    pub addresses: Vec<String>,
    /// Lineage proof (cryptographic verification of relationship)
    pub lineage_proof: String,
    /// Path through genetic lineage to reach peer
    pub lineage_path: Vec<String>,
    /// Depth of search through lineage tree
    pub search_depth: usize,
    /// Last time peer was seen
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_seen: DateTime<Utc>,
}

// NOTE: ContactExchange implementation was removed - dead code
// The contact_exchange logic lives directly on BeardogBtspProvider
// See btsp_provider.rs:335 for the active implementation

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_contact_info_serialization() {
        let contact = ContactInfo {
            peer_id: "test-peer".to_string(),
            addresses: vec!["192.168.1.1:8080".to_string()],
            lineage_proof: "proof_abc123".to_string(),
            lineage_path: vec!["root".to_string(), "test-peer".to_string()],
            search_depth: 2,
            last_seen: Utc::now(),
        };

        let json = serde_json::to_string(&contact).expect("Serialize failed");
        let deserialized: ContactInfo = serde_json::from_str(&json).expect("Deserialize failed");

        assert_eq!(contact.peer_id, deserialized.peer_id);
        assert_eq!(contact.addresses, deserialized.addresses);
        assert_eq!(contact.search_depth, deserialized.search_depth);
    }
}
