//! Trust Management - TOFU, Peer Trust, and mTLS
//!
//! This module documents trust concepts for BTSP tunnels.
//! The actual implementation lives directly on `BeardogBtspProvider`.
//!
//! ## Trust Concepts
//!
//! - **TOFU (Trust On First Use)**: Initial trust establishment with new peers
//! - **Progressive trust**: Promotion based on successful connections  
//! - **mTLS**: Mutual TLS for connection establishment (now Unix sockets)
//! - **BirdSong session keys**: Genetic cryptography for lineage-aware encryption

// NOTE: TrustManager implementation was removed - dead code
// The trust management logic lives directly on BeardogBtspProvider
// See btsp_provider.rs for:
//   - pin_peer_key()
//   - establish_mtls()
//   - generate_session_key()
//   - cleanup_session_key()
//
// TrustLevel, PeerTrustRecord, PeerInfo are in types.rs

#[cfg(test)]
mod tests {
    use super::super::types::TrustLevel;

    #[test]
    fn test_trust_level_ordering() {
        assert!(TrustLevel::Trusted > TrustLevel::Tentative);
        assert!(TrustLevel::Tentative > TrustLevel::Unknown);
    }
}
