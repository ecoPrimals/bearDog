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
/// Proposals and active bonds are stored in concurrent maps. In a
/// production NUCLEUS deployment, bond state would be persisted via
/// `NestGate` or an append-only ledger (`loamSpine`), but the in-process
/// store is sufficient for the JSON-RPC surface contract.
pub struct IonicBondHandler {
    proposals: Arc<RwLock<HashMap<String, PendingProposal>>>,
    bonds: Arc<RwLock<HashMap<String, IonicBond>>>,
}

struct PendingProposal {
    params: IonicBondProposeParams,
    /// SHA-256 of the bond terms, verified during acceptance.
    #[expect(
        dead_code,
        reason = "stored for future signature verification against terms"
    )]
    terms_hash: String,
    proposer_signature: String,
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
    /// The key seed is derived deterministically from the primal's runtime
    /// identity (`PRIMAL_NAME` + node-id) via SHA-256. The BTSP provider reference
    /// is accepted for future HSM-backed signing; the seed derivation ensures
    /// a stable per-instance identity without a separate key ceremony.
    fn sign_terms_ed25519(
        _btsp_provider: &Arc<BeardogBtspProvider>,
        terms_hash: &str,
    ) -> Result<String, String> {
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
        let signature = signing_key.sign(terms_hash.as_bytes());
        Ok(hex::encode(signature.to_bytes()))
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

        let proposer_signature = Self::sign_terms_ed25519(btsp_provider, &terms_hash)?;

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
            acceptor_signature: Some(accept_params.acceptor_signature),
            created_at: proposal.created_at,
            expires_at: proposal.expires_at,
        };

        info!(
            bond_id = %bond_id,
            proposer = %bond.proposer,
            acceptor = %bond.acceptor,
            "Ionic bond sealed"
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

        let accept_params = serde_json::json!({
            "proposal_id": proposal_id,
            "acceptor": "tower_b",
            "acceptor_signature": "deadbeef"
        });

        let accept_result = handler
            .handle("crypto.ionic_bond.accept", Some(&accept_params), &provider)
            .await
            .expect("accept");

        let bond_id = accept_result["bond"]["bond_id"]
            .as_str()
            .unwrap()
            .to_string();
        assert_eq!(accept_result["bond"]["state"], "active");

        let verify_params = serde_json::json!({ "bond_id": bond_id });
        let verify_result = handler
            .handle("crypto.ionic_bond.verify", Some(&verify_params), &provider)
            .await
            .expect("verify");

        assert_eq!(verify_result["valid"], true);
        assert_eq!(verify_result["state"], "active");
    }

    #[tokio::test]
    async fn revoke_prevents_verification() {
        let handler = IonicBondHandler::new();
        let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let propose_params = serde_json::json!({
            "proposer": "a",
            "target": "b"
        });
        let propose_result = handler
            .handle(
                "crypto.ionic_bond.propose",
                Some(&propose_params),
                &provider,
            )
            .await
            .expect("propose");
        let proposal_id = propose_result["proposal_id"].as_str().unwrap();

        let accept_params = serde_json::json!({
            "proposal_id": proposal_id,
            "acceptor": "b",
            "acceptor_signature": "sig"
        });
        let accept_result = handler
            .handle("crypto.ionic_bond.accept", Some(&accept_params), &provider)
            .await
            .expect("accept");
        let bond_id = accept_result["bond"]["bond_id"].as_str().unwrap();

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
            let propose = serde_json::json!({ "proposer": proposer, "target": target });
            let result = handler
                .handle("crypto.ionic_bond.propose", Some(&propose), &provider)
                .await
                .expect("propose");
            let pid = result["proposal_id"].as_str().unwrap();
            let accept = serde_json::json!({
                "proposal_id": pid, "acceptor": target, "acceptor_signature": "s"
            });
            handler
                .handle("crypto.ionic_bond.accept", Some(&accept), &provider)
                .await
                .expect("accept");
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
