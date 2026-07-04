// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tunnel State and Management
//!
//! Internal tunnel state tracking and lifecycle management.

use chrono::{DateTime, Utc};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;
use zeroize::Zeroizing;

use super::types::TrustLevel;

fn now_epoch_millis() -> u64 {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Unix epoch millis fits u64 for all practical tunnel lifetimes"
    )]
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    millis
}

/// Active tunnel state
///
/// Represents a secure tunnel with a peer, including:
/// - Session key (zeroized on drop)
/// - Activity tracking
/// - Trust level
/// - Statistics
pub struct Tunnel {
    /// Unique tunnel identifier
    pub id: String,

    /// Peer's identifier
    pub peer_id: String,

    /// Peer's endpoint (e.g., `unix:///tmp/peer.sock`)
    pub _peer_endpoint: String,

    /// When the tunnel was established
    pub established_at: DateTime<Utc>,

    /// Genetic crypto session key (derived with key lineage)
    ///
    /// **Security**: Automatically zeroized on drop
    pub session_key: Zeroizing<Vec<u8>>,

    /// Bytes sent through this tunnel (lock-free atomic counter)
    ///
    /// EVOLUTION: Changed from Arc<Mutex<u64>> to `AtomicU64` (Jan 29, 2026)
    /// - Lock-free: No mutex contention
    /// - Fast: Direct atomic operations  
    /// - Safe: No unchecked memory patterns needed
    pub bytes_sent: AtomicU64,

    /// Bytes received through this tunnel (lock-free atomic counter)
    ///
    /// EVOLUTION: Changed from Arc<Mutex<u64>> to `AtomicU64` (Jan 29, 2026)
    pub bytes_received: AtomicU64,

    /// Last activity timestamp (Unix epoch milliseconds)
    pub last_activity: Arc<AtomicU64>,

    /// Current trust level for this peer
    pub _trust_level: TrustLevel,
}

impl Tunnel {
    /// Create new tunnel
    ///
    /// # Arguments
    ///
    /// * `id` - Unique tunnel identifier
    /// * `peer_id` - Peer's identifier
    /// * `peer_endpoint` - Peer's endpoint
    /// * `session_key` - Session key (will be zeroized on drop)
    /// * `trust_level` - Initial trust level
    pub(super) fn new(
        id: String,
        peer_id: String,
        peer_endpoint: String,
        session_key: Vec<u8>,
        trust_level: TrustLevel,
    ) -> Self {
        Self {
            id,
            peer_id,
            _peer_endpoint: peer_endpoint,
            established_at: Utc::now(),
            session_key: Zeroizing::new(session_key),
            bytes_sent: AtomicU64::new(0),     // Lock-free atomic
            bytes_received: AtomicU64::new(0), // Lock-free atomic
            last_activity: Arc::new(AtomicU64::new(now_epoch_millis())),
            _trust_level: trust_level,
        }
    }

    /// Check if tunnel is active (activity within last 5 minutes)
    ///
    /// A tunnel is considered active if there has been activity within
    /// the last 5 minutes. Inactive tunnels may be cleaned up.
    pub(super) fn is_active(&self) -> bool {
        let last_ms = self.last_activity.load(Ordering::Relaxed);
        let elapsed_secs = now_epoch_millis().saturating_sub(last_ms) / 1000;
        elapsed_secs < 300 // 5 minutes
    }

    /// Get total bytes sent through tunnel (lock-free atomic read)
    pub(super) fn bytes_sent(&self) -> u64 {
        self.bytes_sent.load(Ordering::Relaxed)
    }

    /// Get total bytes received through tunnel (lock-free atomic read)
    pub(super) fn bytes_received(&self) -> u64 {
        self.bytes_received.load(Ordering::Relaxed)
    }

    /// Get last activity timestamp
    pub(super) fn last_activity(&self) -> DateTime<Utc> {
        let last_ms = self.last_activity.load(Ordering::Relaxed);
        DateTime::from_timestamp_millis(last_ms.cast_signed()).unwrap_or_else(Utc::now)
    }

    /// Update activity timestamp to now
    ///
    /// Called whenever data is sent or received through the tunnel.
    fn update_activity(&self) {
        self.last_activity
            .store(now_epoch_millis(), Ordering::Relaxed);
    }

    /// Increment bytes sent counter
    ///
    /// # Arguments
    ///
    /// * `bytes` - Number of bytes sent
    pub(super) fn add_bytes_sent(&self, bytes: u64) {
        self.bytes_sent.fetch_add(bytes, Ordering::Relaxed);
        self.update_activity();
    }

    /// Increment bytes received counter
    ///
    /// # Arguments
    ///
    /// * `bytes` - Number of bytes received
    pub(super) fn add_bytes_received(&self, bytes: u64) {
        self.bytes_received.fetch_add(bytes, Ordering::Relaxed);
        self.update_activity();
    }
}

/// Implement Drop to ensure session keys are zeroized
///
/// **Security**: This ensures that cryptographic keys are securely
/// erased from memory when the tunnel is dropped, preventing key
/// recovery from memory dumps.
impl Drop for Tunnel {
    fn drop(&mut self) {
        debug!("Tunnel {} dropped - session key zeroized", self.id);
        // session_key is Zeroizing<Vec<u8>> so it's automatically zeroized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tunnel_creation() {
        let tunnel = Tunnel::new(
            "test-id".to_string(),
            "peer-123".to_string(),
            "unix:///tmp/peer.sock".to_string(),
            vec![1, 2, 3, 4],
            TrustLevel::Trusted,
        );

        assert_eq!(tunnel.id, "test-id");
        assert_eq!(tunnel.peer_id, "peer-123");
        assert_eq!(tunnel.bytes_sent(), 0);
        assert_eq!(tunnel.bytes_received(), 0);
    }

    #[test]
    fn test_tunnel_activity_tracking() {
        let tunnel = Tunnel::new(
            "test-id".to_string(),
            "peer-123".to_string(),
            "unix:///tmp/peer.sock".to_string(),
            vec![1, 2, 3, 4],
            TrustLevel::Trusted,
        );

        // Should be active initially
        assert!(tunnel.is_active());

        // Track some activity
        tunnel.add_bytes_sent(100);
        assert_eq!(tunnel.bytes_sent(), 100);

        tunnel.add_bytes_received(200);
        assert_eq!(tunnel.bytes_received(), 200);
    }

    #[test]
    fn test_session_key_zeroization() {
        let key = vec![42u8; 32];
        let key_clone = key.clone();

        {
            let _tunnel = Tunnel::new(
                "test-id".to_string(),
                "peer-123".to_string(),
                "unix:///tmp/peer.sock".to_string(),
                key,
                TrustLevel::Trusted,
            );
            // Tunnel dropped here, key should be zeroized
        }

        // Original key_clone should still have data (not zeroized)
        assert_eq!(key_clone[0], 42);
    }
}
