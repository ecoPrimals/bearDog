// SPDX-License-Identifier: AGPL-3.0-or-later

//! Gossip injection client for swarmVine mesh.
//!
//! Sends structured events to the local swarmVine primal via `gossip.spread`
//! JSON-RPC over UDS. Fire-and-forget semantics — gossip failures are logged
//! but never block the caller.
//!
//! # Security
//!
//! **No key material is ever included in gossip payloads.** Events carry
//! metadata (IDs, algorithm names, quality scores) but never private keys,
//! seeds, or entropy bytes.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::watch;
use tracing::{debug, trace};

// ─── Topic constants ────────────────────────────────────────────────────────

/// Gossip topic constants — dotted canonical form for swarmVine mesh.
pub mod topics {
    /// Ionic bond created between two primals.
    pub const BOND_CREATED: &str = "trust.bond.created";
    /// Ionic bond revoked.
    pub const BOND_REVOKED: &str = "trust.bond.revoked";
    /// Family seed was rotated (generation number, no seed material).
    pub const SEED_ROTATED: &str = "trust.seed.rotated";
    /// BTSP encrypted session established with a peer.
    pub const BTSP_SESSION: &str = "trust.btsp.session";
    /// Consent grant issued to a subject.
    pub const CONSENT_GRANTED: &str = "trust.consent.granted";

    /// FIDO2 hardware security key connected.
    pub const FIDO2_DISCOVERED: &str = "hsm.fido2.discovered";
    /// FIDO2 hardware security key removed.
    pub const FIDO2_REMOVED: &str = "hsm.fido2.removed";
    /// HSM backend changed (e.g. software → hardware).
    pub const HSM_BACKEND_CHANGED: &str = "hsm.backend.changed";
    /// Entropy source quality shifted.
    pub const ENTROPY_QUALITY_CHANGED: &str = "entropy.quality.changed";
    /// Android `StrongBox` availability at startup.
    pub const STRONGBOX_STATUS: &str = "hsm.strongbox.status";

    /// Spine commit was signed (commit hash + public key, no private key).
    pub const SPINE_SIGNED: &str = "crypto.spine.signed";
    /// TLS certificate issued.
    pub const CERT_ISSUED: &str = "crypto.cert.issued";
    /// TLS certificate approaching expiry.
    pub const CERT_EXPIRING: &str = "crypto.cert.expiring";
    /// Cryptographic key derived (metadata only).
    pub const KEY_DERIVED: &str = "crypto.key.derived";

    /// riboCipher mito tag decoded to a protocol type.
    pub const MITO_DECODED: &str = "ribocipher.mito.decoded";
    /// riboCipher mito tag rejected (unknown protocol).
    pub const MITO_REJECTED: &str = "ribocipher.mito.rejected";
}

// ─── Event types ────────────────────────────────────────────────────────────

/// A gossip event to spread to the swarmVine mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipEvent {
    /// Dotted topic (e.g. `trust.bond.created`)
    pub topic: String,
    /// Source primal
    pub origin: String,
    /// Structured payload (metadata only — never key material)
    pub payload: serde_json::Value,
    /// Monotonic event sequence within this primal instance
    pub seq: u64,
}

// ─── Client ─────────────────────────────────────────────────────────────────

/// Environment variable for swarmVine socket path override.
const ENV_SWARMVINE_SOCKET: &str = "SWARMVINE_SOCKET";

/// Default swarmVine socket path (same-gate IPC).
///
/// Resolution order:
/// 1. `SWARMVINE_SOCKET` env (explicit override)
/// 2. `BIOMEOS_SOCKET_DIR` env + `swarmvine.sock` (deployment standard)
/// 3. `biomeos_ipc_socket_dir_from_env()` + `swarmvine.sock` (XDG / temp fallback)
fn default_swarmvine_socket() -> PathBuf {
    if let Ok(p) = std::env::var(ENV_SWARMVINE_SOCKET) {
        return PathBuf::from(p);
    }
    if let Ok(dir) = std::env::var(beardog_config::env_keys::ENV_BIOMEOS_SOCKET_DIR) {
        return PathBuf::from(dir).join("swarmvine.sock");
    }
    beardog_types::constants::domains::network::ipc_discovery::biomeos_ipc_socket_dir_from_env()
        .join("swarmvine.sock")
}

/// Fire-and-forget gossip client. Sends events to the local swarmVine primal.
///
/// The client is cheap to clone (`Arc` internals). All methods are non-blocking
/// — gossip failures are logged at `warn` level and silently dropped.
#[derive(Clone)]
pub struct GossipClient {
    inner: Arc<GossipClientInner>,
}

struct GossipClientInner {
    socket_path: PathBuf,
    primal_name: String,
    seq: AtomicU64,
    /// Shutdown signal — when true, no new gossip is sent.
    shutdown: watch::Receiver<bool>,
}

impl GossipClient {
    /// Create a new gossip client for the given primal.
    ///
    /// The `shutdown_rx` receiver should be signalled during graceful shutdown
    /// to stop gossip injection.
    #[must_use]
    pub fn new(primal_name: &str, shutdown_rx: watch::Receiver<bool>) -> Self {
        Self {
            inner: Arc::new(GossipClientInner {
                socket_path: default_swarmvine_socket(),
                primal_name: primal_name.to_owned(),
                seq: AtomicU64::new(1),
                shutdown: shutdown_rx,
            }),
        }
    }

    /// Create a client with a custom socket path (for testing).
    #[must_use]
    pub fn with_socket(primal_name: &str, socket_path: PathBuf, shutdown_rx: watch::Receiver<bool>) -> Self {
        Self {
            inner: Arc::new(GossipClientInner {
                socket_path,
                primal_name: primal_name.to_owned(),
                seq: AtomicU64::new(1),
                shutdown: shutdown_rx,
            }),
        }
    }

    /// Spread a gossip event (fire-and-forget).
    ///
    /// Spawns a background task — never blocks the caller. Failures are logged.
    pub fn spread(&self, topic: &str, payload: serde_json::Value) {
        if *self.inner.shutdown.borrow() {
            trace!(topic, "gossip suppressed (shutdown)");
            return;
        }

        let event = GossipEvent {
            topic: topic.to_owned(),
            origin: self.inner.primal_name.clone(),
            payload,
            seq: self.inner.seq.fetch_add(1, Ordering::Relaxed),
        };

        let socket_path = self.inner.socket_path.clone();
        tokio::spawn(async move {
            if let Err(e) = send_gossip_inject(&socket_path, &event).await {
                debug!(topic = %event.topic, error = %e, "gossip inject failed (non-fatal)");
            }
        });
    }

    /// Check if the swarmVine socket is reachable.
    pub async fn is_available(&self) -> bool {
        self.inner.socket_path.exists()
    }
}

/// Map a bearDog dotted topic to a swarmVine gossip domain.
///
/// swarmVine has three domains: `tower` (topology/capabilities), `data` (CAS/content),
/// `compute` (resources/dispatch). All bearDog trust/crypto/HSM events are `tower`.
fn topic_to_domain(topic: &str) -> &'static str {
    if topic.starts_with("compute.") {
        "compute"
    } else if topic.starts_with("data.") || topic.starts_with("cas.") {
        "data"
    } else {
        "tower"
    }
}

/// Send a `gossip.inject` JSON-RPC call to the local swarmVine socket.
///
/// Uses `gossip.inject` (local origination) rather than `gossip.spread`
/// (peer-to-peer replication). swarmVine handles nonce, TTL, and expiry
/// for injected entries.
async fn send_gossip_inject(
    socket_path: &std::path::Path,
    event: &GossipEvent,
) -> Result<(), GossipError> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    #[cfg(unix)]
    let mut stream = tokio::net::UnixStream::connect(socket_path)
        .await
        .map_err(|e| GossipError::Connect(format!("{}: {e}", socket_path.display())))?;

    #[cfg(not(unix))]
    return Err(GossipError::Connect(
        "gossip inject requires Unix sockets".into(),
    ));

    let domain = topic_to_domain(&event.topic);
    let key = format!("{}:{}", event.topic, event.origin);

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "gossip.inject",
        "params": {
            "topic": domain,
            "key": key,
            "payload": {
                "event_topic": event.topic,
                "origin": event.origin,
                "detail": event.payload,
                "seq": event.seq,
            },
        },
        "id": event.seq,
    });

    let mut bytes = serde_json::to_vec(&request)
        .map_err(|e| GossipError::Serialize(e.to_string()))?;
    bytes.push(b'\n');

    #[cfg(unix)]
    {
        stream
            .write_all(&bytes)
            .await
            .map_err(|e| GossipError::Send(e.to_string()))?;

        let mut reader = BufReader::new(&mut stream);
        let mut response = String::new();
        let read_result = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            reader.read_line(&mut response),
        )
        .await;

        match read_result {
            Ok(Ok(_)) => {
                trace!(topic = %event.topic, domain, "gossip inject acknowledged");
            }
            Ok(Err(e)) => {
                debug!(topic = %event.topic, error = %e, "gossip response read failed");
            }
            Err(_) => {
                trace!(topic = %event.topic, "gossip response timeout (fire-and-forget)");
            }
        }
    }

    Ok(())
}

/// Gossip-specific errors (internal, never exposed to callers of `spread`).
#[derive(Debug)]
enum GossipError {
    Connect(String),
    Serialize(String),
    #[cfg(unix)]
    Send(String),
}

impl std::fmt::Display for GossipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connect(e) => write!(f, "gossip connect: {e}"),
            Self::Serialize(e) => write!(f, "gossip serialize: {e}"),
            #[cfg(unix)]
            Self::Send(e) => write!(f, "gossip send: {e}"),
        }
    }
}

// ─── Convenience constructors for common events ─────────────────────────────

impl GossipClient {
    /// Announce an ionic bond creation.
    pub fn bond_created(&self, bond_id: &str, proposer: &str, acceptor: &str, scope: &str) {
        self.spread(
            topics::BOND_CREATED,
            serde_json::json!({
                "bond_id": bond_id,
                "proposer": proposer,
                "acceptor": acceptor,
                "scope": scope,
            }),
        );
    }

    /// Announce an ionic bond revocation.
    pub fn bond_revoked(&self, bond_id: &str, reason: &str) {
        self.spread(
            topics::BOND_REVOKED,
            serde_json::json!({ "bond_id": bond_id, "reason": reason }),
        );
    }

    /// Announce a spine commit was signed.
    pub fn spine_signed(&self, commit_hash: &str, key_id: &str, public_key: &str) {
        self.spread(
            topics::SPINE_SIGNED,
            serde_json::json!({
                "commit_hash": commit_hash,
                "key_id": key_id,
                "public_key": public_key,
            }),
        );
    }

    /// Announce a FIDO2 device was discovered.
    pub fn fido2_discovered(&self, device_path: &str, aaguid: &str) {
        self.spread(
            topics::FIDO2_DISCOVERED,
            serde_json::json!({ "device_path": device_path, "aaguid": aaguid }),
        );
    }

    /// Announce a FIDO2 device was removed.
    pub fn fido2_removed(&self, device_path: &str) {
        self.spread(
            topics::FIDO2_REMOVED,
            serde_json::json!({ "device_path": device_path }),
        );
    }

    /// Announce a riboCipher mito tag was decoded.
    pub fn mito_decoded(&self, protocol_type: u8, protocol_name: &str) {
        self.spread(
            topics::MITO_DECODED,
            serde_json::json!({
                "protocol_type": protocol_type,
                "protocol_name": protocol_name,
            }),
        );
    }

    /// Announce a key was derived (metadata only).
    pub fn key_derived(&self, key_id: &str, purpose: &str, algorithm: &str) {
        self.spread(
            topics::KEY_DERIVED,
            serde_json::json!({
                "key_id": key_id,
                "purpose": purpose,
                "algorithm": algorithm,
            }),
        );
    }

    /// Announce entropy quality changed.
    pub fn entropy_quality_changed(&self, source: &str, quality: f64) {
        self.spread(
            topics::ENTROPY_QUALITY_CHANGED,
            serde_json::json!({ "source": source, "quality": quality }),
        );
    }

    /// Announce BTSP session established with a peer.
    pub fn btsp_session(&self, peer_id: &str, direction: &str) {
        self.spread(
            topics::BTSP_SESSION,
            serde_json::json!({ "peer_id": peer_id, "direction": direction }),
        );
    }
}

// ─── Global singleton ───────────────────────────────────────────────────────

use std::sync::OnceLock;

static GLOBAL_GOSSIP: OnceLock<GossipClient> = OnceLock::new();

/// Initialize the global gossip client (call once at startup).
///
/// Returns `true` if initialization succeeded, `false` if already initialized.
pub fn init_global_gossip(client: GossipClient) -> bool {
    GLOBAL_GOSSIP.set(client).is_ok()
}

/// Get a reference to the global gossip client.
///
/// Returns `None` if [`init_global_gossip`] has not been called. Handlers
/// should treat `None` as "gossip not yet wired" and skip injection.
#[must_use]
pub fn gossip() -> Option<&'static GossipClient> {
    GLOBAL_GOSSIP.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gossip_event_serializes() {
        let event = GossipEvent {
            topic: topics::SPINE_SIGNED.to_owned(),
            origin: "beardog".to_owned(),
            payload: serde_json::json!({"commit_hash": "abc123"}),
            seq: 42,
        };
        let json = serde_json::to_string(&event).expect("serialize");
        assert!(json.contains("crypto.spine.signed"));
        assert!(json.contains("abc123"));
    }

    #[test]
    fn gossip_event_round_trips() {
        let event = GossipEvent {
            topic: topics::BOND_CREATED.to_owned(),
            origin: "beardog".to_owned(),
            payload: serde_json::json!({"bond_id": "b1", "scope": "full"}),
            seq: 1,
        };
        let json = serde_json::to_string(&event).unwrap();
        let back: GossipEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(back.topic, topics::BOND_CREATED);
        assert_eq!(back.seq, 1);
    }

    #[test]
    fn default_socket_path_resolves() {
        let path = default_swarmvine_socket();
        assert!(
            path.to_string_lossy().contains("swarmvine"),
            "socket path should contain 'swarmvine': {path:?}"
        );
    }

    #[tokio::test]
    async fn client_creation_and_availability() {
        let (_tx, rx) = watch::channel(false);
        let client = GossipClient::new("beardog", rx);
        // swarmVine socket won't exist in test env
        assert!(!client.is_available().await);
    }

    #[tokio::test]
    async fn spread_to_missing_socket_is_nonfatal() {
        let (_tx, rx) = watch::channel(false);
        let client = GossipClient::with_socket(
            "beardog",
            PathBuf::from("/tmp/nonexistent-swarmvine-test.sock"),
            rx,
        );
        // Should not panic or block
        client.spine_signed("abc", "key1", "pubkey1");
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    #[tokio::test]
    async fn shutdown_suppresses_gossip() {
        let (tx, rx) = watch::channel(false);
        let client = GossipClient::with_socket(
            "beardog",
            PathBuf::from("/tmp/nonexistent-swarmvine-test.sock"),
            rx,
        );
        tx.send(true).unwrap();
        // After shutdown, spread should be a no-op
        client.bond_created("b1", "a", "b", "test");
    }

    #[test]
    fn topic_constants_are_dotted() {
        let all_topics = [
            topics::BOND_CREATED,
            topics::BOND_REVOKED,
            topics::SEED_ROTATED,
            topics::BTSP_SESSION,
            topics::CONSENT_GRANTED,
            topics::FIDO2_DISCOVERED,
            topics::FIDO2_REMOVED,
            topics::HSM_BACKEND_CHANGED,
            topics::ENTROPY_QUALITY_CHANGED,
            topics::STRONGBOX_STATUS,
            topics::SPINE_SIGNED,
            topics::CERT_ISSUED,
            topics::CERT_EXPIRING,
            topics::KEY_DERIVED,
            topics::MITO_DECODED,
            topics::MITO_REJECTED,
        ];
        for topic in all_topics {
            assert!(topic.contains('.'), "topic must be dotted: {topic}");
        }
        assert_eq!(all_topics.len(), 16);
    }
}
