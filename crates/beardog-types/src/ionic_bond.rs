// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ionic bond types for cross-atomic-boundary trust negotiation.
//!
//! Ionic bonds enforce cryptographic trust at atomic boundaries in NUCLEUS
//! compositions — e.g. Tower Atomic (crypto) ↔ Node Atomic (compute).
//! The canonical use case is healthSpring's dual-tower enclave pattern:
//! patient data (Tower A) ↔ analytics (Tower B), with BearDog enforcing
//! both sides via `crypto.ionic_bond.*` RPC methods.
//!
//! # Bond Lifecycle
//!
//! ```text
//! Proposer                          BearDog                          Acceptor
//!    |                                 |                                |
//!    |-- crypto.ionic_bond.propose --> |                                |
//!    |<-- proposal_id + terms -------- |                                |
//!    |                                 |<-- crypto.ionic_bond.accept -- |
//!    |                                 |--- bond_id + active_bond ----> |
//!    |                                 |                                |
//!    |       (either party seals — re-verifies both signatures)        |
//!    |                                 |                                |
//!    |-- crypto.ionic_bond.seal ----> |                                |
//!    |<-- {sealed: true, bond} ------- |                                |
//!    |                                 |                                |
//!    |       (bond is now sealed — enforcement-ready)                   |
//! ```

use serde::{Deserialize, Serialize};

/// Encryption tier required at a bond boundary.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncryptionTier {
    /// `ChaCha20-Poly1305` AEAD (default for ionic bonds).
    #[default]
    Aead,
    /// HMAC integrity only — no confidentiality.
    IntegrityOnly,
    /// Transport-layer encryption is sufficient.
    TransportOnly,
}

/// Trust model governing the bond.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BondTrustModel {
    /// `BearDog` enforces both sides (`healthSpring` dual-tower enclave).
    DualTowerEnclave,
    /// BTSP handshake enforced at the boundary.
    #[default]
    BtspEnforced,
    /// Family seed-derived trust (same NUCLEUS family).
    FamilySeed,
}

/// State machine for an ionic bond.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BondState {
    /// Bond has been proposed but not yet accepted.
    Proposed,
    /// Bond has been accepted — both parties' signatures stored.
    Active,
    /// Bond has been cryptographically sealed: both Ed25519 signatures
    /// re-verified and the bond is ready for enforcement.
    Sealed,
    /// Bond has been revoked by either party.
    Revoked,
    /// Bond expired (TTL exceeded).
    Expired,
}

/// An ionic bond between two trust domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicBond {
    /// Unique bond identifier.
    pub bond_id: String,
    /// Proposal ID that created this bond.
    pub proposal_id: String,
    /// SHA-256 hex digest of the canonical bond terms. Both proposer and
    /// acceptor Ed25519 signatures are over this value, enabling
    /// cryptographic re-verification at any point during the bond lifetime.
    pub terms_hash: String,
    /// Proposing primal/domain identifier.
    pub proposer: String,
    /// Accepting primal/domain identifier.
    pub acceptor: String,
    /// Trust model for the bond.
    pub trust_model: BondTrustModel,
    /// Encryption tier at the boundary.
    pub encryption_tier: EncryptionTier,
    /// Current state.
    pub state: BondState,
    /// Capabilities allowed across the boundary.
    pub allowed_capabilities: Vec<String>,
    /// Ed25519 signature over the bond terms (from proposer).
    pub proposer_signature: Option<String>,
    /// Proposer's Ed25519 public key (hex-encoded).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposer_public_key: Option<String>,
    /// Ed25519 signature over the bond terms (from acceptor).
    pub acceptor_signature: Option<String>,
    /// Acceptor's Ed25519 public key (hex-encoded).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptor_public_key: Option<String>,
    /// When the bond was created (RFC 3339).
    pub created_at: String,
    /// When the bond expires (RFC 3339), if time-bounded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

// ── RPC Parameter Types ──────────────────────────────────────────────

/// Parameters for `crypto.ionic_bond.propose`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicBondProposeParams {
    /// Identifier for the proposing domain (e.g. `"tower_a"` or a primal name).
    pub proposer: String,
    /// Identifier for the target domain to bond with.
    pub target: String,
    /// Trust model to use.
    #[serde(default)]
    pub trust_model: BondTrustModel,
    /// Encryption tier at the boundary.
    #[serde(default)]
    pub encryption_tier: EncryptionTier,
    /// Capabilities the proposer wants to share across the boundary.
    #[serde(default)]
    pub allowed_capabilities: Vec<String>,
    /// Optional TTL in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<u64>,
}

/// Response from `crypto.ionic_bond.propose`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicBondProposeResponse {
    /// Proposal ID for the acceptor to reference.
    pub proposal_id: String,
    /// Bond terms hash (for the acceptor to verify before accepting).
    pub terms_hash: String,
    /// Proposer's Ed25519 signature over the terms.
    pub proposer_signature: String,
}

/// Parameters for `crypto.ionic_bond.accept`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicBondAcceptParams {
    /// Proposal ID from `propose`.
    pub proposal_id: String,
    /// Acceptor's domain identifier.
    pub acceptor: String,
    /// Acceptor's Ed25519 signature over the terms hash (hex-encoded, 128 hex chars).
    pub acceptor_signature: String,
    /// Acceptor's Ed25519 public key (hex-encoded, 64 hex chars).
    /// Used to verify `acceptor_signature` against the proposal terms hash.
    pub acceptor_public_key: String,
}

/// Response from `crypto.ionic_bond.accept`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicBondAcceptResponse {
    /// The sealed bond.
    pub bond: IonicBond,
}

/// Parameters for `crypto.ionic_bond.verify`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicBondVerifyParams {
    /// Bond ID to verify.
    pub bond_id: String,
}

/// Response from `crypto.ionic_bond.verify`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicBondVerifyResponse {
    /// Whether the bond is valid and active.
    pub valid: bool,
    /// Current bond state.
    pub state: BondState,
    /// Bond details (if valid).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bond: Option<IonicBond>,
    /// Error (if invalid).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Parameters for `crypto.ionic_bond.seal`.
///
/// Cryptographically seals an active bond by re-verifying both proposer and
/// acceptor Ed25519 signatures. This is the explicit "third step" in the
/// propose → accept → seal lifecycle that downstream primals call to confirm
/// the bond is enforcement-ready.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicBondSealParams {
    /// Bond ID to seal.
    pub bond_id: String,
    /// Identity of the party requesting the seal (must be proposer or acceptor).
    pub sealer: String,
}

/// Response from `crypto.ionic_bond.seal`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicBondSealResponse {
    /// Whether the seal succeeded.
    pub sealed: bool,
    /// The sealed bond (with `state: "sealed"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bond: Option<IonicBond>,
    /// Error detail (set only on failure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Parameters for `crypto.ionic_bond.revoke`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicBondRevokeParams {
    /// Bond ID to revoke.
    pub bond_id: String,
    /// Revoker's identity (must be proposer or acceptor).
    pub revoker: String,
}

/// Response from `crypto.ionic_bond.revoke`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicBondRevokeResponse {
    /// Whether revocation succeeded.
    pub revoked: bool,
}

/// Parameters for `crypto.ionic_bond.list`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicBondListParams {
    /// Filter by domain (show bonds involving this domain).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Filter by state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<BondState>,
}

/// Response from `crypto.ionic_bond.list`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicBondListResponse {
    /// Active bonds matching the filter.
    pub bonds: Vec<IonicBond>,
}

// ── Contract Signing Types ─────────────────────────────────────────────

/// Parameters for `crypto.sign_contract`.
///
/// Signs an arbitrary contract document with `BearDog`'s Ed25519 identity,
/// producing a deterministic terms hash and a verifiable signature. This is
/// the endpoint that enables programmatic cross-family trust establishment
/// for multi-family deployments (CERN-level clouds, data federation,
/// friend-hosted shards).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignContractParams {
    /// Identity of the signing party (primal name, tower ID, etc.).
    pub signer: String,
    /// The contract terms to sign. Serialized canonically (sorted keys) and
    /// SHA-256 hashed before signing.
    pub terms: serde_json::Value,
    /// Optional context label scoping the signature (e.g. `"gpu_lease"`,
    /// `"data_egress_fence"`, `"ionic_bond"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Optional time-to-live in seconds (ionic lease). When set, the signed
    /// contract carries an `expires_at` timestamp; `crypto.verify_contract`
    /// will reject signatures past their lease expiry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<u64>,
}

/// Response from `crypto.sign_contract`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignContractResponse {
    /// SHA-256 hex digest of the canonical contract terms.
    pub terms_hash: String,
    /// Ed25519 signature over the terms hash (standard base64, 64 bytes decoded).
    pub signature: String,
    /// Ed25519 public key of the signer (standard base64, 32 bytes decoded).
    pub public_key: String,
    /// Signing timestamp (RFC 3339).
    pub signed_at: String,
    /// Expiry timestamp (RFC 3339) — present when `ttl_seconds` was specified.
    /// After this time, `crypto.verify_contract` will report the lease as expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Parameters for `crypto.verify_contract`.
///
/// Verifies an Ed25519 signature over a contract terms hash, enabling any
/// party to confirm that a contract was signed by the claimed signer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyContractParams {
    /// SHA-256 hex of the contract terms (from `sign_contract` response).
    pub terms_hash: String,
    /// Ed25519 signature to verify (hex-encoded).
    pub signature: String,
    /// Ed25519 public key of the claimed signer (hex-encoded).
    pub public_key: String,
    /// Optional expiry timestamp (RFC 3339) from the `sign_contract` response.
    /// When present, the verifier rejects the contract if the lease has expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Response from `crypto.verify_contract`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyContractResponse {
    /// Whether the signature is cryptographically valid and the lease (if any)
    /// has not expired.
    pub valid: bool,
    /// Whether the contract's ionic lease has expired. Only set when
    /// `expires_at` was provided in the verify request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    /// Error detail (set only on failure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ── Cross-family contract lifecycle (multi-party signing) ────────────

/// Parameters for `crypto.contract.propose` — initiate a cross-family contract.
///
/// The proposer signs the contract terms first, then shares the `contract_id`
/// with counterparties who countersign via `crypto.contract.countersign`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractProposeParams {
    /// Identity of the proposing party (tower ID, family ID, etc.).
    pub proposer: String,
    /// The contract terms both parties must agree to. Serialized canonically
    /// and SHA-256 hashed before signing.
    pub terms: serde_json::Value,
    /// Optional context label (e.g. `"gpu_lease"`, `"data_federation"`,
    /// `"dual_tower_ionic"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Optional TTL in seconds. After expiry, the contract cannot be countersigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<u64>,
}

/// Response from `crypto.contract.propose`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractProposeResponse {
    /// Unique contract identifier for countersigning.
    pub contract_id: String,
    /// SHA-256 hex digest of the canonical terms.
    pub terms_hash: String,
    /// Proposer's Ed25519 signature (standard base64).
    pub proposer_signature: String,
    /// Proposer's Ed25519 public key (standard base64).
    pub proposer_public_key: String,
    /// When the proposal was created (RFC 3339).
    pub created_at: String,
    /// When the proposal expires (RFC 3339), if time-bounded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Parameters for `crypto.contract.countersign` — second party signs.
///
/// The counterparty provides their own signature over the same `terms_hash`.
/// Both signatures are verified before the contract is sealed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractCountersignParams {
    /// Contract ID from the propose response.
    pub contract_id: String,
    /// Identity of the countersigning party.
    pub countersigner: String,
    /// Countersigner's Ed25519 signature over the terms hash (standard base64).
    pub countersigner_signature: String,
    /// Countersigner's Ed25519 public key (standard base64).
    pub countersigner_public_key: String,
}

/// A sealed cross-family contract with both parties' signatures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossFamilyContract {
    /// Unique contract identifier.
    pub contract_id: String,
    /// SHA-256 hex digest of the canonical terms.
    pub terms_hash: String,
    /// The original contract terms.
    pub terms: serde_json::Value,
    /// Optional context label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Proposer identity.
    pub proposer: String,
    /// Proposer's Ed25519 signature (standard base64).
    pub proposer_signature: String,
    /// Proposer's Ed25519 public key (standard base64).
    pub proposer_public_key: String,
    /// Countersigner identity.
    pub countersigner: String,
    /// Countersigner's Ed25519 signature (standard base64).
    pub countersigner_signature: String,
    /// Countersigner's Ed25519 public key (standard base64).
    pub countersigner_public_key: String,
    /// When the contract was sealed (RFC 3339).
    pub sealed_at: String,
}

/// Response from `crypto.contract.countersign`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractCountersignResponse {
    /// The fully sealed contract with both signatures.
    pub contract: CrossFamilyContract,
}

/// Parameters for `crypto.contract.verify` — verify a sealed contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractVerifyParams {
    /// SHA-256 hex digest of the terms.
    pub terms_hash: String,
    /// First party's signature (standard base64).
    pub proposer_signature: String,
    /// First party's public key (standard base64).
    pub proposer_public_key: String,
    /// Second party's signature (standard base64).
    pub countersigner_signature: String,
    /// Second party's public key (standard base64).
    pub countersigner_public_key: String,
}

/// Response from `crypto.contract.verify`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractVerifyResponse {
    /// Both signatures are cryptographically valid.
    pub valid: bool,
    /// Which party's signature failed, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_party: Option<String>,
    /// Error detail (set only on failure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_params_defaults() {
        let json = r#"{
            "proposer": "tower_a",
            "target": "tower_b"
        }"#;
        let params: IonicBondProposeParams = serde_json::from_str(json).expect("deserialize");
        assert_eq!(params.trust_model, BondTrustModel::BtspEnforced);
        assert_eq!(params.encryption_tier, EncryptionTier::Aead);
        assert!(params.allowed_capabilities.is_empty());
        assert!(params.ttl_seconds.is_none());
    }

    #[test]
    fn bond_state_serialization() {
        let json = serde_json::to_string(&BondState::Active).expect("serialize");
        assert_eq!(json, r#""active""#);
        let parsed: BondState = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed, BondState::Active);

        let sealed_json = serde_json::to_string(&BondState::Sealed).expect("serialize sealed");
        assert_eq!(sealed_json, r#""sealed""#);
        let sealed_parsed: BondState =
            serde_json::from_str(&sealed_json).expect("deserialize sealed");
        assert_eq!(sealed_parsed, BondState::Sealed);
    }

    #[test]
    fn trust_model_serialization() {
        let json = serde_json::to_string(&BondTrustModel::DualTowerEnclave).expect("serialize");
        assert_eq!(json, r#""dual_tower_enclave""#);
    }
}
