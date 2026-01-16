//! Tunnel Lifecycle Management
//!
//! This module handles the complete lifecycle of secure tunnels:
//! - Creation and initialization
//! - Session key management
//! - State tracking
//! - Graceful termination

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use zeroize::Zeroizing;

use super::types::TrustLevel;
use beardog_errors::BearDogError;

// =============================================================================
// Tunnel State
// =============================================================================

/// Active tunnel state with genetic cryptography session
pub(super) struct Tunnel {
    pub(super) id: String,
    pub(super) peer_id: String,
    pub(super) peer_endpoint: String,
    pub(super) established_at: DateTime<Utc>,

    // Genetic crypto session key (derived with key lineage)
    pub(super) session_key: Zeroizing<Vec<u8>>,

    // Statistics
    pub(super) bytes_sent: Arc<Mutex<u64>>,
    pub(super) bytes_received: Arc<Mutex<u64>>,
    pub(super) last_activity: Arc<Mutex<SystemTime>>,

    // Trust level
    pub(super) trust_level: TrustLevel,
}

impl Tunnel {
    /// Create new tunnel
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
            peer_endpoint,
            established_at: Utc::now(),
            session_key: Zeroizing::new(session_key),
            bytes_sent: Arc::new(Mutex::new(0)),
            bytes_received: Arc::new(Mutex::new(0)),
            last_activity: Arc::new(Mutex::new(SystemTime::now())),
            trust_level,
        }
    }

    /// Update statistics after sending data
    pub(super) fn record_sent(&self, bytes: u64) {
        *self.bytes_sent.lock() += bytes;
        *self.last_activity.lock() = SystemTime::now();
    }

    /// Update statistics after receiving data
    pub(super) fn record_received(&self, bytes: u64) {
        *self.bytes_received.lock() += bytes;
        *self.last_activity.lock() = SystemTime::now();
    }

    /// Get total bytes sent
    pub(super) fn total_bytes_sent(&self) -> u64 {
        *self.bytes_sent.lock()
    }

    /// Get total bytes received
    pub(super) fn total_bytes_received(&self) -> u64 {
        *self.bytes_received.lock()
    }

    /// Get last activity time
    pub(super) fn last_activity(&self) -> SystemTime {
        *self.last_activity.lock()
    }

    /// Check if tunnel is idle
    pub(super) fn is_idle(&self, idle_threshold: std::time::Duration) -> bool {
        match self.last_activity().elapsed() {
            Ok(elapsed) => elapsed > idle_threshold,
            Err(_) => false, // Clock issue, assume not idle
        }
    }
}

// Auto-zeroize session key on drop (security)
impl Drop for Tunnel {
    fn drop(&mut self) {
        // Session key is already Zeroizing, will be cleared automatically
        tracing::debug!(
            tunnel_id = %self.id,
            peer = %self.peer_id,
            "Tunnel dropped, session key zeroized"
        );
    }
}

// =============================================================================
// Tunnel Lifecycle Manager
// =============================================================================

/// Manages tunnel lifecycle operations
pub(super) struct TunnelLifecycleManager {
    idle_timeout: std::time::Duration,
    max_tunnels: usize,
    tunnel_counter: AtomicU64,
}

impl TunnelLifecycleManager {
    /// Create new lifecycle manager
    pub(super) fn new(idle_timeout: std::time::Duration, max_tunnels: usize) -> Self {
        Self {
            idle_timeout,
            max_tunnels,
            tunnel_counter: AtomicU64::new(0),
        }
    }

    /// Generate unique tunnel ID
    pub(super) fn generate_tunnel_id(&self) -> String {
        let counter = self.tunnel_counter.fetch_add(1, Ordering::SeqCst);
        format!("tunnel_{}", counter)
    }

    /// Create new tunnel with session key
    pub(super) fn create_tunnel(
        &self,
        peer_id: String,
        peer_endpoint: String,
        session_key: Vec<u8>,
        trust_level: TrustLevel,
    ) -> Result<Tunnel, BearDogError> {
        let tunnel_id = self.generate_tunnel_id();
        
        Ok(Tunnel::new(
            tunnel_id,
            peer_id,
            peer_endpoint,
            session_key,
            trust_level,
        ))
    }

    /// Check if tunnel should be cleaned up
    pub(super) fn should_cleanup(&self, tunnel: &Tunnel) -> bool {
        tunnel.is_idle(self.idle_timeout)
    }

    /// Get maximum allowed tunnels
    pub(super) fn max_tunnels(&self) -> usize {
        self.max_tunnels
    }
}

impl Default for TunnelLifecycleManager {
    fn default() -> Self {
        Self::new(
            std::time::Duration::from_secs(300), // 5 minute idle timeout
            1000, // Max 1000 concurrent tunnels
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tunnel_creation() {
        let tunnel = Tunnel::new(
            "test-1".to_string(),
            "peer-123".to_string(),
            "endpoint".to_string(),
            vec![1, 2, 3, 4],
            TrustLevel::Verified,
        );

        assert_eq!(tunnel.id, "test-1");
        assert_eq!(tunnel.peer_id, "peer-123");
        assert_eq!(tunnel.total_bytes_sent(), 0);
        assert_eq!(tunnel.total_bytes_received(), 0);
    }

    #[test]
    fn test_tunnel_statistics() {
        let tunnel = Tunnel::new(
            "test-1".to_string(),
            "peer-123".to_string(),
            "endpoint".to_string(),
            vec![1, 2, 3, 4],
            TrustLevel::Verified,
        );

        tunnel.record_sent(100);
        tunnel.record_received(50);

        assert_eq!(tunnel.total_bytes_sent(), 100);
        assert_eq!(tunnel.total_bytes_received(), 50);
    }

    #[test]
    fn test_lifecycle_manager() {
        let manager = TunnelLifecycleManager::default();
        
        let id1 = manager.generate_tunnel_id();
        let id2 = manager.generate_tunnel_id();
        
        assert_ne!(id1, id2);
        assert!(id1.starts_with("tunnel_"));
    }

    #[test]
    fn test_tunnel_creation_via_manager() {
        let manager = TunnelLifecycleManager::default();
        
        let tunnel = manager.create_tunnel(
            "peer-456".to_string(),
            "endpoint".to_string(),
            vec![5, 6, 7, 8],
            TrustLevel::Trusted,
        ).unwrap();

        assert_eq!(tunnel.peer_id, "peer-456");
        assert_eq!(tunnel.trust_level, TrustLevel::Trusted);
    }
}

