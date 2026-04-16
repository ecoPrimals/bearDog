// SPDX-License-Identifier: AGPL-3.0-or-later

#![allow(async_fn_in_trait)] // Native `async fn` in traits; `Send` on futures matches our `Send + Sync` impls

//! Generic capability trait definitions
//!
//! These traits define capabilities without any primal-specific knowledge.
//! `BearDog` implements these traits, and any primal can consume them.

use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

// =============================================================================
// Secure Tunnel Capability
// =============================================================================

/// Generic peer endpoint (no primal names)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerEndpoint {
    /// Peer identifier (UUID or similar)
    pub id: String,
    /// Network endpoint (IP:port or hostname:port)
    pub endpoint: String,
    /// Optional public key for verification (TOFU support)
    pub public_key: Option<Vec<u8>>,
}

/// Handle to an established secure tunnel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelHandle {
    /// Unique tunnel identifier
    pub id: String,
    /// Remote peer identifier
    pub peer_id: String,
    /// Timestamp when tunnel was established (ISO 8601)
    pub established_at: String,
}

/// Tunnel status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelStatus {
    /// Tunnel identifier
    pub tunnel_id: String,
    /// Whether tunnel is active
    pub active: bool,
    /// Bytes sent
    pub bytes_sent: u64,
    /// Bytes received
    pub bytes_received: u64,
    /// Last activity timestamp (ISO 8601)
    pub last_activity: String,
}

/// Generic secure tunnel capability
///
/// This capability provides encrypted point-to-point tunnels between peers.
/// The implementation may use various cryptographic protocols (e.g., genetic
/// crypto, mTLS, WireGuard-like schemes), but consumers only see this generic
/// interface.
///
/// **Note**: Documentation may reference "BTSP" (`BearDog` Tunnel Security Protocol)
/// for developer context, but the code remains fully generic.
pub trait SecureTunnelProvider: Send + Sync {
    /// Establish a secure tunnel to a peer
    ///
    /// # Arguments
    /// * `peer` - Remote peer endpoint information
    ///
    /// # Returns
    /// Handle to the established tunnel
    async fn establish_tunnel(
        &self,
        peer: PeerEndpoint,
    ) -> Result<TunnelHandle, beardog_errors::BearDogError>;

    /// Encrypt data for transmission through tunnel
    ///
    /// # Arguments
    /// * `handle` - Tunnel to use for encryption
    /// * `data` - Plaintext data to encrypt
    ///
    /// # Returns
    /// Encrypted data ready for transmission
    async fn tunnel_encrypt(
        &self,
        handle: &TunnelHandle,
        data: &[u8],
    ) -> Result<Vec<u8>, beardog_errors::BearDogError>;

    /// Decrypt data received from tunnel
    ///
    /// # Arguments
    /// * `handle` - Tunnel to use for decryption
    /// * `data` - Encrypted data received
    ///
    /// # Returns
    /// Decrypted plaintext data
    async fn tunnel_decrypt(
        &self,
        handle: &TunnelHandle,
        data: &[u8],
    ) -> Result<Vec<u8>, beardog_errors::BearDogError>;

    /// Get tunnel status
    ///
    /// # Arguments
    /// * `handle` - Tunnel to query
    ///
    /// # Returns
    /// Current tunnel status and statistics
    async fn tunnel_status(
        &self,
        handle: &TunnelHandle,
    ) -> Result<TunnelStatus, beardog_errors::BearDogError>;

    /// Close a tunnel
    ///
    /// # Arguments
    /// * `handle` - Tunnel to close
    async fn close_tunnel(&self, handle: &TunnelHandle)
    -> Result<(), beardog_errors::BearDogError>;
}

// =============================================================================
// Lineage Signing Capability
// =============================================================================

/// Lineage chain (parent-child relationships)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageChain {
    /// Chain identifier
    pub id: String,
    /// Root node identifier
    pub root: String,
    /// Ordered list of node IDs (root to leaf)
    pub nodes: Vec<String>,
    /// Cryptographic signatures for each relationship
    pub signatures: Vec<Vec<u8>>,
}

/// Cryptographic lineage proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageProof {
    /// Node claiming lineage
    pub node_id: String,
    /// Root of lineage
    pub root_id: String,
    /// Path from root to node
    pub path: Vec<String>,
    /// Merkle proof or signature chain
    pub proof_data: Vec<u8>,
}

/// Cryptographic signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    /// Signature algorithm (e.g., "Ed25519", "ECDSA")
    pub algorithm: String,
    /// Signature bytes
    pub signature: Vec<u8>,
    /// Public key used for verification
    pub public_key: Vec<u8>,
}

/// Generic lineage signing capability
///
/// This capability provides cryptographic lineage chains for trust relationships.
/// Useful for hierarchical trust, family-based access control, or ancestry tracking.
pub trait LineageSigningProvider: Send + Sync {
    /// Generate a lineage chain
    ///
    /// # Arguments
    /// * `node_id` - Identifier for the node
    /// * `parent_id` - Optional parent node (None for root)
    ///
    /// # Returns
    /// Lineage chain with cryptographic signatures
    async fn generate_lineage(
        &self,
        node_id: &str,
        parent_id: Option<&str>,
    ) -> Result<LineageChain, beardog_errors::BearDogError>;

    /// Sign a parent-child relationship
    ///
    /// # Arguments
    /// * `parent` - Parent node identifier
    /// * `child` - Child node identifier
    ///
    /// # Returns
    /// Cryptographic signature of the relationship
    async fn sign_relationship(
        &self,
        parent: &str,
        child: &str,
    ) -> Result<Signature, beardog_errors::BearDogError>;

    /// Verify a lineage proof
    ///
    /// # Arguments
    /// * `proof` - Lineage proof to verify
    ///
    /// # Returns
    /// True if proof is valid, false otherwise
    async fn verify_lineage(
        &self,
        proof: &LineageProof,
    ) -> Result<bool, beardog_errors::BearDogError>;

    /// Get all descendants of a node
    ///
    /// # Arguments
    /// * `root` - Root node to query
    ///
    /// # Returns
    /// List of descendant node identifiers
    async fn get_descendants(
        &self,
        root: &str,
    ) -> Result<Vec<String>, beardog_errors::BearDogError>;
}

// =============================================================================
// Broadcast Encryption Capability
// =============================================================================

/// Lineage hint for encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageHint {
    /// Lineage identifier
    pub lineage_id: String,
    /// Maximum depth for decryption (family scope)
    pub max_depth: usize,
}

/// Encrypted broadcast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedBroadcast {
    /// Broadcast identifier
    pub id: String,
    /// Encrypted payload
    pub ciphertext: Vec<u8>,
    /// Lineage hint (public)
    pub lineage_hint: LineageHint,
    /// Additional authenticated data
    pub aad: Vec<u8>,
}

/// Broadcast encryption key
#[derive(Debug, Clone)]
pub struct BroadcastKey {
    /// Key identifier
    pub id: String,
    /// Key material (zeroized on drop)
    pub key: Zeroizing<Vec<u8>>,
    /// Lineage this key is for
    pub lineage_id: String,
}

/// Generic broadcast encryption capability
///
/// This capability provides "family-only" broadcast encryption where only
/// nodes within a specific lineage can decrypt messages. Non-family sees
/// only encrypted noise.
pub trait BroadcastEncryptionProvider: Send + Sync {
    /// Encrypt data for a specific lineage
    ///
    /// # Arguments
    /// * `payload` - Data to encrypt
    /// * `lineage_hint` - Lineage scope for decryption
    ///
    /// # Returns
    /// Encrypted broadcast (family-only decryption)
    async fn encrypt_for_lineage(
        &self,
        payload: &[u8],
        lineage_hint: LineageHint,
    ) -> Result<EncryptedBroadcast, beardog_errors::BearDogError>;

    /// Decrypt a broadcast (if authorized by lineage)
    ///
    /// # Arguments
    /// * `broadcast` - Encrypted broadcast
    /// * `proof` - Lineage proof for authorization
    ///
    /// # Returns
    /// Decrypted payload (or error if not authorized)
    async fn decrypt_broadcast(
        &self,
        broadcast: &EncryptedBroadcast,
        proof: LineageProof,
    ) -> Result<Vec<u8>, beardog_errors::BearDogError>;

    /// Request decryption key for a lineage
    ///
    /// # Arguments
    /// * `lineage_hint` - Lineage to request key for
    /// * `proof` - Lineage proof for authorization
    ///
    /// # Returns
    /// Broadcast key (if authorized)
    async fn request_key(
        &self,
        lineage_hint: &LineageHint,
        proof: LineageProof,
    ) -> Result<BroadcastKey, beardog_errors::BearDogError>;

    /// Rotate keys for a lineage
    ///
    /// # Arguments
    /// * `lineage` - Lineage to rotate keys for
    async fn rotate_keys(&self, lineage: &str) -> Result<(), beardog_errors::BearDogError>;
}

// =============================================================================
// Key Derivation Capability
// =============================================================================

/// Key derivation path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivationPath {
    /// Path components (e.g. `["m", "44'", "0'", "0'"]`)
    pub components: Vec<String>,
    /// Additional context info
    pub context: Vec<u8>,
}

/// Generic key derivation capability
///
/// This capability provides cryptographic key derivation from context data.
/// Useful for generating session keys, deriving child keys, or creating
/// deterministic key hierarchies.
pub trait KeyDerivationProvider: Send + Sync {
    /// Derive a key from context
    ///
    /// # Arguments
    /// * `context` - Context data for derivation
    /// * `info` - Additional info string
    /// * `length` - Desired key length in bytes
    ///
    /// # Returns
    /// Derived key (zeroized on drop)
    async fn derive_key(
        &self,
        context: &[u8],
        info: &[u8],
        length: usize,
    ) -> Result<Zeroizing<Vec<u8>>, beardog_errors::BearDogError>;

    /// Derive multiple keys in a hierarchy
    ///
    /// # Arguments
    /// * `root_context` - Root context for hierarchy
    /// * `derivation_paths` - Paths to derive
    ///
    /// # Returns
    /// Vector of derived keys (zeroized on drop)
    async fn derive_key_hierarchy(
        &self,
        root_context: &[u8],
        derivation_paths: &[DerivationPath],
    ) -> Result<Vec<Zeroizing<Vec<u8>>>, beardog_errors::BearDogError>;
}
