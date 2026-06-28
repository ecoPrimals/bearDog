// SPDX-License-Identifier: AGPL-3.0-or-later

//! Auth event bus for cross-gate trust provenance.
//!
//! Provides a bounded, poll-based event log for trust lifecycle events.
//! Subscribers (e.g. rhizoCrypt's `MeshEventListener`) poll via `auth.events.poll`
//! to receive events since a given timestamp.
//!
//! The bus stores events in a bounded ring buffer (oldest evicted when full).
//! In-process subscribers can call [`AuthEventBus::subscribe`] for a broadcast
//! channel. An RPC method (`auth.events.subscribe`) is not yet wired.
//!
//! ## Wire format (FRAGO: wave76c-beardog-auth-events-subscribe)
//!
//! ```json
//! {
//!   "kind": { "type": "TrustIssuerRegistered", "payload": { ... } },
//!   "source_gate": "<gate>",
//!   "timestamp": <unix_secs>
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

const DEFAULT_CAPACITY: usize = 10_000;

/// Auth event kind — maps 1:1 to rhizoCrypt's `MeshTrustEvent` variants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum AuthEventKind {
    /// A new remote issuer was registered in the `TrustedIssuerRegistry`.
    TrustIssuerRegistered {
        /// The issuer's `did:key:z6Mk...` DID.
        issuer_did: String,
        /// Hex-encoded truncated fingerprint of the Ed25519 public key.
        issuer_fingerprint: String,
        /// How trust was established (`family_seed`, `contract_exchange`, `manual`).
        trust_method: String,
    },
    /// An Ed25519 key exchange completed with a remote gate.
    KeyExchangeCompleted {
        /// The remote gate identifier.
        remote_gate: String,
        /// Key exchange method used.
        method: String,
    },
    /// A new gate enrolled into the family.
    FamilyEnrollment {
        /// The family identifier.
        family_id: String,
        /// Number of primals in the family after enrollment.
        primal_count: u32,
    },
    /// A gate joined the covalent mesh.
    MeshJoin {
        /// The mesh network identifier.
        mesh_id: String,
    },
    /// A gate left the covalent mesh.
    MeshLeave {
        /// The mesh network identifier.
        mesh_id: String,
        /// Reason: `Graceful`, `Disconnected`, `Evicted`, or `TrustRevoked`.
        reason: String,
    },
}

/// A single auth event with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthEvent {
    /// The event kind and payload.
    pub kind: AuthEventKind,
    /// Gate that originated the event.
    pub source_gate: String,
    /// Unix timestamp (seconds) when the event occurred.
    pub timestamp: i64,
}

/// Thread-safe, bounded auth event bus with poll + broadcast support.
#[derive(Debug, Clone)]
pub struct AuthEventBus {
    events: Arc<RwLock<VecDeque<AuthEvent>>>,
    capacity: usize,
    broadcast_tx: broadcast::Sender<AuthEvent>,
}

impl AuthEventBus {
    /// Create a new event bus with the given capacity.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        let cap = if capacity == 0 {
            DEFAULT_CAPACITY
        } else {
            capacity
        };
        let (broadcast_tx, _) = broadcast::channel(cap);
        Self {
            events: Arc::new(RwLock::new(VecDeque::with_capacity(cap.min(1024)))),
            capacity: cap,
            broadcast_tx,
        }
    }

    /// Emit an event into the bus.
    ///
    /// The event is appended to the poll buffer (oldest evicted if full)
    /// and broadcast to any active streaming subscribers.
    pub fn emit(&self, event: AuthEvent) {
        let mut events = self
            .events
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if events.len() >= self.capacity {
            events.pop_front();
        }
        let _ = self.broadcast_tx.send(event.clone());
        events.push_back(event);
    }

    /// Poll for events since a given Unix timestamp (inclusive).
    #[must_use]
    pub fn poll_since(&self, since_timestamp: i64) -> Vec<AuthEvent> {
        let events = self
            .events
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        events
            .iter()
            .filter(|e| e.timestamp >= since_timestamp)
            .cloned()
            .collect()
    }

    /// Total events currently buffered.
    #[must_use]
    pub fn len(&self) -> usize {
        let events = self
            .events
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        events.len()
    }

    /// Whether the buffer is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Subscribe to the broadcast channel (for future streaming support).
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<AuthEvent> {
        self.broadcast_tx.subscribe()
    }
}

impl Default for AuthEventBus {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_event(ts: i64) -> AuthEvent {
        AuthEvent {
            kind: AuthEventKind::TrustIssuerRegistered {
                issuer_did: "did:key:z6MkTest".into(),
                issuer_fingerprint: "aabb".into(),
                trust_method: "family_seed".into(),
            },
            source_gate: "test-gate".into(),
            timestamp: ts,
        }
    }

    #[test]
    fn emit_and_poll_returns_events() {
        let bus = AuthEventBus::new(100);
        bus.emit(sample_event(1000));
        bus.emit(sample_event(2000));
        bus.emit(sample_event(3000));

        let events = bus.poll_since(2000);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].timestamp, 2000);
        assert_eq!(events[1].timestamp, 3000);
    }

    #[test]
    fn poll_since_zero_returns_all() {
        let bus = AuthEventBus::new(100);
        bus.emit(sample_event(100));
        bus.emit(sample_event(200));
        assert_eq!(bus.poll_since(0).len(), 2);
    }

    #[test]
    fn poll_since_future_returns_empty() {
        let bus = AuthEventBus::new(100);
        bus.emit(sample_event(100));
        assert!(bus.poll_since(9999).is_empty());
    }

    #[test]
    fn capacity_evicts_oldest() {
        let bus = AuthEventBus::new(3);
        bus.emit(sample_event(1));
        bus.emit(sample_event(2));
        bus.emit(sample_event(3));
        bus.emit(sample_event(4));
        assert_eq!(bus.len(), 3);
        let events = bus.poll_since(0);
        assert_eq!(events[0].timestamp, 2);
        assert_eq!(events[2].timestamp, 4);
    }

    #[test]
    fn broadcast_delivers_to_subscriber() {
        let bus = AuthEventBus::new(100);
        let mut rx = bus.subscribe();
        bus.emit(sample_event(42));
        let received = rx.try_recv().expect("broadcast event");
        assert_eq!(received.timestamp, 42);
    }

    #[test]
    fn default_creates_with_default_capacity() {
        let bus = AuthEventBus::default();
        assert!(bus.is_empty());
        assert_eq!(bus.capacity, DEFAULT_CAPACITY);
    }

    #[test]
    fn event_kind_serializes_to_wire_format() {
        let event = sample_event(1000);
        let json = serde_json::to_value(&event).expect("serialize");
        let kind = &json["kind"];
        assert_eq!(kind["type"], "TrustIssuerRegistered");
        assert!(kind["payload"]["issuer_fingerprint"].is_string());
        assert_eq!(json["source_gate"], "test-gate");
        assert_eq!(json["timestamp"], 1000);
    }

    #[test]
    fn all_event_kinds_serialize() {
        let kinds = vec![
            AuthEventKind::TrustIssuerRegistered {
                issuer_did: "did:key:z6MkA".into(),
                issuer_fingerprint: "ff".into(),
                trust_method: "manual".into(),
            },
            AuthEventKind::KeyExchangeCompleted {
                remote_gate: "east-gate".into(),
                method: "ed25519_dh".into(),
            },
            AuthEventKind::FamilyEnrollment {
                family_id: "fam-1".into(),
                primal_count: 3,
            },
            AuthEventKind::MeshJoin {
                mesh_id: "mesh-alpha".into(),
            },
            AuthEventKind::MeshLeave {
                mesh_id: "mesh-alpha".into(),
                reason: "Graceful".into(),
            },
        ];
        for kind in kinds {
            let json = serde_json::to_value(&kind).expect("serialize kind");
            assert!(
                json["type"].is_string(),
                "tagged enum must have 'type' field"
            );
        }
    }
}
