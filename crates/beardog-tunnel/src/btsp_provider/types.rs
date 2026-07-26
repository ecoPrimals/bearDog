// SPDX-License-Identifier: AGPL-3.0-or-later

//! BTSP Internal Types
//!
//! Internal type definitions for BTSP implementation.
//! These types are used internally and converted to/from capability trait types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// =============================================================================
// Public Types (Re-exported from main module)
// =============================================================================

/// Security context for encryption/decryption operations
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// Tunnel identifier
    pub tunnel_id: String,
    /// Direction of data flow
    pub direction: Direction,
}

/// Data flow direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    /// Outbound (local -> remote)
    Outbound,
    /// Inbound (remote -> local)
    Inbound,
}

/// Peer information for tunnel establishment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Peer identifier
    pub id: String,
    /// Peer endpoint (IP:Port or Unix socket path)
    pub endpoint: String,
    /// Optional public key for TOFU
    pub public_key: Option<Vec<u8>>,
}

/// Trust level for peers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TrustLevel {
    /// Unknown peer (never seen before)
    Unknown = 0,
    /// Tentative trust (TOFU - first connection)
    Tentative = 1,
    /// Trusted peer (multiple successful connections)
    Trusted = 2,
    /// Verified peer (cryptographically verified via genetic lineage or certificate)
    Verified = 3,
}

/// Peer trust record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerTrustRecord {
    /// Unique peer identifier
    pub peer_id: String,
    /// Peer's public key bytes
    pub public_key: Vec<u8>,
    /// Current trust level for this peer
    pub trust_level: TrustLevel,
    /// When this peer was first encountered
    pub first_seen: DateTime<Utc>,
    /// When this peer was last seen
    pub last_seen: DateTime<Utc>,
    /// Total number of connections from this peer
    pub connection_count: u64,
    /// Family this peer belongs to (set during trust seeding)
    #[serde(default)]
    pub family_id: Option<String>,
}

// =============================================================================
// Internal Types
// =============================================================================

/// Internal tunnel handle (uses `DateTime` for implementation convenience)
///
/// This is an internal type that gets converted to the capability trait's
/// `TunnelHandle` type which uses Unix timestamps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalTunnelHandle {
    /// Unique tunnel identifier
    pub id: String,

    /// Peer node identifier
    pub peer_id: String,

    /// Timestamp when tunnel was established
    pub established_at: DateTime<Utc>,
}

impl InternalTunnelHandle {
    /// Create new internal tunnel handle
    #[must_use]
    pub fn new(id: String, peer_id: String) -> Self {
        Self {
            id,
            peer_id,
            established_at: Utc::now(),
        }
    }
    /// Get age of tunnel in seconds
    #[must_use]
    pub fn age_seconds(&self) -> i64 {
        Utc::now()
            .signed_duration_since(self.established_at)
            .num_seconds()
    }
}

/// Internal tunnel status (uses `DateTime` for implementation convenience)
///
/// This is an internal type that gets converted to the capability trait's
/// `TunnelStatus` type which uses Unix timestamps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalTunnelStatus {
    /// Whether tunnel is currently active
    pub active: bool,

    /// Total bytes sent through tunnel
    pub bytes_sent: u64,

    /// Total bytes received through tunnel
    pub bytes_received: u64,

    /// Timestamp of last activity
    pub last_activity: DateTime<Utc>,
}

impl InternalTunnelStatus {
    /// Create new active tunnel status
    #[must_use]
    pub fn new_active() -> Self {
        Self {
            active: true,
            bytes_sent: 0,
            bytes_received: 0,
            last_activity: Utc::now(),
        }
    }
    /// Create new inactive tunnel status
    #[must_use]
    pub fn new_inactive() -> Self {
        Self {
            active: false,
            bytes_sent: 0,
            bytes_received: 0,
            last_activity: Utc::now(),
        }
    }

    /// Update last activity timestamp
    pub fn touch(&mut self) {
        self.last_activity = Utc::now();
    }

    /// Record bytes sent
    pub fn add_bytes_sent(&mut self, bytes: u64) {
        self.bytes_sent = self.bytes_sent.saturating_add(bytes);
        self.touch();
    }

    /// Record bytes received
    pub fn add_bytes_received(&mut self, bytes: u64) {
        self.bytes_received = self.bytes_received.saturating_add(bytes);
        self.touch();
    }
    /// Get seconds since last activity
    #[must_use]
    pub fn idle_seconds(&self) -> i64 {
        Utc::now()
            .signed_duration_since(self.last_activity)
            .num_seconds()
    }
    /// Check if tunnel is stale (idle > threshold)
    #[must_use]
    pub fn is_stale(&self, idle_threshold_seconds: i64) -> bool {
        self.idle_seconds() > idle_threshold_seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_tunnel_handle() {
        let handle = InternalTunnelHandle::new("tunnel-1".into(), "peer-1".into());
        assert_eq!(handle.id, "tunnel-1");
        assert_eq!(handle.peer_id, "peer-1");
        assert!(handle.age_seconds() >= 0);
    }

    #[test]
    fn test_internal_tunnel_status_active() {
        let status = InternalTunnelStatus::new_active();
        assert!(status.active);
        assert_eq!(status.bytes_sent, 0);
        assert_eq!(status.bytes_received, 0);
    }

    #[test]
    fn test_internal_tunnel_status_inactive() {
        let status = InternalTunnelStatus::new_inactive();
        assert!(!status.active);
    }

    #[test]
    fn test_add_bytes() {
        let mut status = InternalTunnelStatus::new_active();
        status.add_bytes_sent(100);
        status.add_bytes_received(200);
        assert_eq!(status.bytes_sent, 100);
        assert_eq!(status.bytes_received, 200);
    }

    #[test]
    fn test_idle_tracking() {
        let mut status = InternalTunnelStatus::new_active();
        // Freshly created — idle time is essentially zero
        assert!(status.idle_seconds() >= 0);

        status.touch();
        assert!(status.idle_seconds() < 1);
    }

    #[test]
    fn test_stale_detection() {
        let status = InternalTunnelStatus::new_active();
        assert!(!status.is_stale(3600)); // Not stale if threshold is 1 hour
    }
}
