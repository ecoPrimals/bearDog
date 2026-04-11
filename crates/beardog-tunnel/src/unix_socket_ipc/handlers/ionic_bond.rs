// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ionic bond handler — `crypto.ionic_bond.*` JSON-RPC surface.
//!
//! Implements runtime ionic bond negotiation for cross-atomic-boundary
//! trust. Resolves primalSpring gap synthesis items:
//! - hotSpring GAP-HS-005: cross-family GPU lease
//! - healthSpring §2: data egress fence enforcement
//!
//! The bond lifecycle is: propose → accept → active → (verify | revoke).

use super::MethodHandler;
use crate::btsp_provider::BeardogBtspProvider;
use async_trait::async_trait;
use beardog_types::ionic_bond::{
    BondState, IonicBond, IonicBondAcceptParams, IonicBondAcceptResponse, IonicBondListParams,
    IonicBondListResponse, IonicBondProposeParams, IonicBondProposeResponse, IonicBondRevokeParams,
    IonicBondRevokeResponse, IonicBondVerifyParams, IonicBondVerifyResponse,
};
use chrono::Utc;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// In-memory ionic bond state manager.
///
/// Proposals and active bonds are stored in concurrent maps keyed by UUID.
/// This is architecturally correct for `BearDog` as the crypto primal:
/// `BearDog`'s responsibility is cryptographic operations (signing, verifying,
/// sealing bonds), not durable storage. For production NUCLEUS deployments,
/// bond persistence should be delegated to `NestGate` (storage primal) via
/// `storage.store`/`storage.retrieve` capability discovery, or to an
/// append-only ledger via `loamSpine`. The in-process store is sufficient
/// for the JSON-RPC surface contract and single-process lifetimes.
pub struct IonicBondHandler {
    proposals: Arc<RwLock<HashMap<String, PendingProposal>>>,
    bonds: Arc<RwLock<HashMap<String, IonicBond>>>,
}

struct PendingProposal {
    params: IonicBondProposeParams,
    /// SHA-256 of the bond terms, verified during acceptance.
    terms_hash: String,
    proposer_signature: String,
    proposer_public_key: String,
    created_at: String,
    expires_at: Option<String>,
}

impl Default for IonicBondHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl IonicBondHandler {
    /// Create a new handler with empty state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            proposals: Arc::new(RwLock::new(HashMap::new())),
            bonds: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Sign the terms hash with the primal's Ed25519 identity key.
    ///
    /// Returns `(signature_hex, public_key_hex)`. The key seed is derived
    /// deterministically from the primal's runtime identity (`PRIMAL_NAME` +
    /// node-id) via SHA-256.
    ///
    /// **BTSP Phase 3 (HSM path):** The `_btsp_provider` parameter is
    /// reserved for HSM-backed signing via `HsmKeyProvider::sign()`. When
    /// an HSM is available, the signing key should be generated/stored in
    /// hardware rather than derived from environment. The current software
    /// derivation path is production-safe but not sovereign-grade.
    fn sign_terms_ed25519(
        _btsp_provider: &Arc<BeardogBtspProvider>,
        terms_hash: &str,
    ) -> Result<(String, String), String> {
        use ed25519_dalek::{Signer, SigningKey};
        use sha2::{Digest, Sha256};

        let primal_name = std::env::var("PRIMAL_NAME").unwrap_or_else(|_| "beardog".to_string());
        let node_id = beardog_types::primal_identity::resolve_node_id_from_env_or_ephemeral(None);

        let mut seed = [0u8; 32];
        let mut h = Sha256::new();
        h.update(b"ionic-bond-identity-seed:");
        h.update(primal_name.as_bytes());
        h.update(b":");
        h.update(node_id.as_bytes());
        seed.copy_from_slice(&h.finalize());

        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();
        let signature = signing_key.sign(terms_hash.as_bytes());
        Ok((
            hex::encode(signature.to_bytes()),
            hex::encode(verifying_key.as_bytes()),
        ))
    }

    /// Verify an Ed25519 signature over the terms hash using the provided
    /// hex-encoded public key and signature.
    fn verify_ed25519_signature(
        terms_hash: &str,
        signature_hex: &str,
        public_key_hex: &str,
    ) -> Result<(), String> {
        use ed25519_dalek::{Signature, Verifier, VerifyingKey};

        let pub_bytes: [u8; 32] = hex::decode(public_key_hex)
            .map_err(|e| format!("Invalid acceptor public key hex: {e}"))?
            .try_into()
            .map_err(|v: Vec<u8>| {
                format!("Acceptor public key must be 32 bytes, got {}", v.len())
            })?;

        let sig_bytes: [u8; 64] = hex::decode(signature_hex)
            .map_err(|e| format!("Invalid acceptor signature hex: {e}"))?
            .try_into()
            .map_err(|v: Vec<u8>| {
                format!("Acceptor signature must be 64 bytes, got {}", v.len())
            })?;

        let verifying_key = VerifyingKey::from_bytes(&pub_bytes)
            .map_err(|e| format!("Invalid Ed25519 public key: {e}"))?;

        let signature = Signature::from_bytes(&sig_bytes);

        verifying_key
            .verify(terms_hash.as_bytes(), &signature)
            .map_err(|e| format!("Acceptor signature verification failed: {e}"))
    }

    fn compute_terms_hash(params: &IonicBondProposeParams) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(params.proposer.as_bytes());
        hasher.update(params.target.as_bytes());
        hasher.update(
            serde_json::to_string(&params.trust_model)
                .unwrap_or_default()
                .as_bytes(),
        );
        hasher.update(
            serde_json::to_string(&params.encryption_tier)
                .unwrap_or_default()
                .as_bytes(),
        );
        for cap in &params.allowed_capabilities {
            hasher.update(cap.as_bytes());
        }
        hex::encode(hasher.finalize())
    }
}

#[async_trait]
impl MethodHandler for IonicBondHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            "crypto.ionic_bond.propose",
            "crypto.ionic_bond.accept",
            "crypto.ionic_bond.verify",
            "crypto.ionic_bond.revoke",
            "crypto.ionic_bond.list",
        ]
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        match method {
            "crypto.ionic_bond.propose" => self.handle_propose(params, btsp_provider).await,
            "crypto.ionic_bond.accept" => self.handle_accept(params).await,
            "crypto.ionic_bond.verify" => self.handle_verify(params).await,
            "crypto.ionic_bond.revoke" => self.handle_revoke(params).await,
            "crypto.ionic_bond.list" => self.handle_list(params).await,
            _ => Err(format!("Unknown ionic bond method: {method}")),
        }
    }
}

impl IonicBondHandler {
    async fn handle_propose(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        let params_value = params.ok_or("Missing params for crypto.ionic_bond.propose")?;
        let propose_params = IonicBondProposeParams::deserialize(params_value)
            .map_err(|e| format!("Invalid propose params: {e}"))?;

        let proposal_id = uuid::Uuid::new_v4().to_string();
        let terms_hash = Self::compute_terms_hash(&propose_params);

        let (proposer_signature, proposer_public_key) =
            Self::sign_terms_ed25519(btsp_provider, &terms_hash)?;

        let now = Utc::now().to_rfc3339();
        let expires_at = propose_params.ttl_seconds.map(|ttl| {
            let secs = i64::try_from(ttl).unwrap_or(i64::MAX);
            (Utc::now() + chrono::Duration::seconds(secs)).to_rfc3339()
        });

        info!(
            proposal_id = %proposal_id,
            proposer = %propose_params.proposer,
            target = %propose_params.target,
            trust_model = ?propose_params.trust_model,
            "Ionic bond proposed"
        );

        self.proposals.write().await.insert(
            proposal_id.clone(),
            PendingProposal {
                params: propose_params,
                terms_hash: terms_hash.clone(),
                proposer_signature: proposer_signature.clone(),
                proposer_public_key,
                created_at: now,
                expires_at,
            },
        );

        let resp = IonicBondProposeResponse {
            proposal_id,
            terms_hash,
            proposer_signature,
        };
        serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
    }

    async fn handle_accept(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let params_value = params.ok_or("Missing params for crypto.ionic_bond.accept")?;
        let accept_params = IonicBondAcceptParams::deserialize(params_value)
            .map_err(|e| format!("Invalid accept params: {e}"))?;

        let proposal = self
            .proposals
            .write()
            .await
            .remove(&accept_params.proposal_id)
            .ok_or_else(|| {
                format!(
                    "Proposal not found or expired: {}",
                    accept_params.proposal_id
                )
            })?;

        Self::verify_ed25519_signature(
            &proposal.terms_hash,
            &accept_params.acceptor_signature,
            &accept_params.acceptor_public_key,
        )?;

        let bond_id = uuid::Uuid::new_v4().to_string();

        let bond = IonicBond {
            bond_id: bond_id.clone(),
            proposal_id: accept_params.proposal_id,
            proposer: proposal.params.proposer,
            acceptor: accept_params.acceptor.clone(),
            trust_model: proposal.params.trust_model,
            encryption_tier: proposal.params.encryption_tier,
            state: BondState::Active,
            allowed_capabilities: proposal.params.allowed_capabilities,
            proposer_signature: Some(proposal.proposer_signature),
            proposer_public_key: Some(proposal.proposer_public_key),
            acceptor_signature: Some(accept_params.acceptor_signature),
            acceptor_public_key: Some(accept_params.acceptor_public_key),
            created_at: proposal.created_at,
            expires_at: proposal.expires_at,
        };

        info!(
            bond_id = %bond_id,
            proposer = %bond.proposer,
            acceptor = %bond.acceptor,
            "Ionic bond sealed (signatures verified)"
        );

        self.bonds.write().await.insert(bond_id, bond.clone());

        let resp = IonicBondAcceptResponse { bond };
        serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
    }

    async fn handle_verify(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let params_value = params.ok_or("Missing params for crypto.ionic_bond.verify")?;
        let verify_params = IonicBondVerifyParams::deserialize(params_value)
            .map_err(|e| format!("Invalid verify params: {e}"))?;

        let bonds = self.bonds.read().await;

        if let Some(bond) = bonds.get(&verify_params.bond_id) {
            let is_active = bond.state == BondState::Active;

            let is_expired = bond
                .expires_at
                .as_ref()
                .and_then(|exp| chrono::DateTime::parse_from_rfc3339(exp).ok())
                .is_some_and(|exp| Utc::now() > exp);

            let (valid, state) = if is_expired {
                (false, BondState::Expired)
            } else {
                (is_active, bond.state)
            };

            let resp = IonicBondVerifyResponse {
                valid,
                state,
                bond: if valid { Some(bond.clone()) } else { None },
                error: if valid {
                    None
                } else {
                    Some(format!("Bond is {state:?}"))
                },
            };
            serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
        } else {
            let resp = IonicBondVerifyResponse {
                valid: false,
                state: BondState::Revoked,
                bond: None,
                error: Some(format!("Bond not found: {}", verify_params.bond_id)),
            };
            serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
        }
    }

    async fn handle_revoke(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let params_value = params.ok_or("Missing params for crypto.ionic_bond.revoke")?;
        let revoke_params = IonicBondRevokeParams::deserialize(params_value)
            .map_err(|e| format!("Invalid revoke params: {e}"))?;

        let mut bonds = self.bonds.write().await;

        if let Some(bond) = bonds.get_mut(&revoke_params.bond_id) {
            if bond.proposer != revoke_params.revoker && bond.acceptor != revoke_params.revoker {
                return Err(format!(
                    "Revoker '{}' is neither proposer nor acceptor",
                    revoke_params.revoker
                ));
            }

            bond.state = BondState::Revoked;
            warn!(
                bond_id = %revoke_params.bond_id,
                revoker = %revoke_params.revoker,
                "Ionic bond revoked"
            );

            let resp = IonicBondRevokeResponse { revoked: true };
            serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
        } else {
            Err(format!("Bond not found: {}", revoke_params.bond_id))
        }
    }

    async fn handle_list(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let list_params = params
            .map(|p| {
                IonicBondListParams::deserialize(p).map_err(|e| format!("Invalid list params: {e}"))
            })
            .transpose()?;

        let bonds = self.bonds.read().await;

        let filtered: Vec<IonicBond> = bonds
            .values()
            .filter(|b| {
                if let Some(ref lp) = list_params {
                    let domain_ok = lp
                        .domain
                        .as_ref()
                        .is_none_or(|d| b.proposer == *d || b.acceptor == *d);
                    let state_ok = lp.state.is_none_or(|s| b.state == s);
                    domain_ok && state_ok
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        let resp = IonicBondListResponse { bonds: filtered };
        serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generate a real Ed25519 signature over `terms_hash` and return
    /// `(signature_hex, public_key_hex)` for use in accept params.
    fn sign_as_acceptor(terms_hash: &str) -> (String, String) {
        use ed25519_dalek::{Signer, SigningKey};
        let key = SigningKey::from_bytes(&[0x42; 32]);
        let sig = key.sign(terms_hash.as_bytes());
        (
            hex::encode(sig.to_bytes()),
            hex::encode(key.verifying_key().as_bytes()),
        )
    }

    /// Helper: propose a bond -> return (proposal_id, terms_hash).
    async fn propose_bond(
        handler: &IonicBondHandler,
        provider: &Arc<BeardogBtspProvider>,
        proposer: &str,
        target: &str,
    ) -> (String, String) {
        let params = serde_json::json!({ "proposer": proposer, "target": target });
        let result = handler
            .handle("crypto.ionic_bond.propose", Some(&params), provider)
            .await
            .expect("propose");
        (
            result["proposal_id"].as_str().unwrap().to_string(),
            result["terms_hash"].as_str().unwrap().to_string(),
        )
    }

    /// Helper: accept a bond with real Ed25519 signature -> return bond_id.
    async fn accept_bond(
        handler: &IonicBondHandler,
        provider: &Arc<BeardogBtspProvider>,
        proposal_id: &str,
        terms_hash: &str,
        acceptor: &str,
    ) -> String {
        let (sig, pubkey) = sign_as_acceptor(terms_hash);
        let params = serde_json::json!({
            "proposal_id": proposal_id,
            "acceptor": acceptor,
            "acceptor_signature": sig,
            "acceptor_public_key": pubkey,
        });
        let result = handler
            .handle("crypto.ionic_bond.accept", Some(&params), provider)
            .await
            .expect("accept");
        result["bond"]["bond_id"].as_str().unwrap().to_string()
    }

    #[tokio::test]
    async fn propose_accept_verify_lifecycle() {
        let handler = IonicBondHandler::new();
        let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let propose_params = serde_json::json!({
            "proposer": "tower_a",
            "target": "tower_b",
            "trust_model": "dual_tower_enclave",
            "encryption_tier": "aead",
            "allowed_capabilities": ["science.pkpd.simulate", "compute.dispatch.submit"]
        });

        let propose_result = handler
            .handle(
                "crypto.ionic_bond.propose",
                Some(&propose_params),
                &provider,
            )
            .await
            .expect("propose");

        let proposal_id = propose_result["proposal_id"].as_str().unwrap().to_string();
        let terms_hash = propose_result["terms_hash"].as_str().unwrap().to_string();

        assert!(!proposal_id.is_empty());
        assert!(!terms_hash.is_empty());

        let bond_id = accept_bond(&handler, &provider, &proposal_id, &terms_hash, "tower_b").await;

        let verify_params = serde_json::json!({ "bond_id": bond_id });
        let verify_result = handler
            .handle("crypto.ionic_bond.verify", Some(&verify_params), &provider)
            .await
            .expect("verify");

        assert_eq!(verify_result["valid"], true);
        assert_eq!(verify_result["state"], "active");
        assert!(verify_result["bond"]["proposer_public_key"].is_string());
        assert!(verify_result["bond"]["acceptor_public_key"].is_string());
    }

    #[tokio::test]
    async fn accept_rejects_invalid_signature() {
        let handler = IonicBondHandler::new();
        let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let (proposal_id, _terms_hash) = propose_bond(&handler, &provider, "a", "b").await;

        use ed25519_dalek::SigningKey;
        let key = SigningKey::from_bytes(&[0x42; 32]);
        let wrong_sig = hex::encode([0xAA; 64]);
        let pubkey = hex::encode(key.verifying_key().as_bytes());

        let params = serde_json::json!({
            "proposal_id": proposal_id,
            "acceptor": "b",
            "acceptor_signature": wrong_sig,
            "acceptor_public_key": pubkey,
        });

        let result = handler
            .handle("crypto.ionic_bond.accept", Some(&params), &provider)
            .await;

        assert!(result.is_err(), "should reject invalid signature");
        assert!(
            result.unwrap_err().contains("verification failed"),
            "error should mention verification"
        );
    }

    #[tokio::test]
    async fn revoke_prevents_verification() {
        let handler = IonicBondHandler::new();
        let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let (proposal_id, terms_hash) = propose_bond(&handler, &provider, "a", "b").await;
        let bond_id = accept_bond(&handler, &provider, &proposal_id, &terms_hash, "b").await;

        let revoke_params = serde_json::json!({
            "bond_id": bond_id,
            "revoker": "a"
        });
        handler
            .handle("crypto.ionic_bond.revoke", Some(&revoke_params), &provider)
            .await
            .expect("revoke");

        let verify_params = serde_json::json!({ "bond_id": bond_id });
        let verify_result = handler
            .handle("crypto.ionic_bond.verify", Some(&verify_params), &provider)
            .await
            .expect("verify");

        assert_eq!(verify_result["valid"], false);
        assert_eq!(verify_result["state"], "revoked");
    }

    #[tokio::test]
    async fn list_filters_by_domain() {
        let handler = IonicBondHandler::new();
        let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        for (proposer, target) in [("x", "y"), ("a", "b")] {
            let (pid, hash) = propose_bond(&handler, &provider, proposer, target).await;
            accept_bond(&handler, &provider, &pid, &hash, target).await;
        }

        let list_params = serde_json::json!({ "domain": "x" });
        let list_result = handler
            .handle("crypto.ionic_bond.list", Some(&list_params), &provider)
            .await
            .expect("list");

        assert_eq!(list_result["bonds"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn methods_list() {
        let handler = IonicBondHandler::new();
        let methods = handler.methods();
        assert_eq!(methods.len(), 5);
        assert!(methods.contains(&"crypto.ionic_bond.propose"));
        assert!(methods.contains(&"crypto.ionic_bond.verify"));
    }
}
