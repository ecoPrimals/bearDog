// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::HandlerError;
use super::crypto::{compute_terms_hash, sign_terms_ed25519, verify_ed25519_signature};
use super::persistence::BondPersistence;
use super::{IonicBondHandler, PendingProposal};
use crate::btsp_provider::BeardogBtspProvider;
use beardog_types::ionic_bond::{
    BondState, IonicBond, IonicBondAcceptParams, IonicBondAcceptResponse, IonicBondListParams,
    IonicBondListResponse, IonicBondProposeParams, IonicBondProposeResponse, IonicBondRevokeParams,
    IonicBondRevokeResponse, IonicBondSealParams, IonicBondSealResponse, IonicBondVerifyParams,
    IonicBondVerifyResponse,
};
use chrono::Utc;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

impl IonicBondHandler {
    pub(super) async fn handle_propose(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params_value = params.ok_or("Missing params for crypto.ionic_bond.propose")?;
        let propose_params = IonicBondProposeParams::deserialize(params_value)
            .map_err(|e| format!("Invalid propose params: {e}"))?;

        let proposal_id = uuid::Uuid::new_v4().to_string();
        let terms_hash = compute_terms_hash(&propose_params);

        let (proposer_signature, proposer_public_key) =
            sign_terms_ed25519(btsp_provider, &terms_hash)?;

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

        let pk_for_response = proposer_public_key.clone();

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
            proposer_public_key: pk_for_response,
        };
        serde_json::to_value(resp)
            .map_err(|e| format!("Serialize: {e}"))
            .map_err(Into::into)
    }

    pub(super) async fn handle_accept(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
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

        if let Some(ref exp) = proposal.expires_at
            && let Ok(expiry) = chrono::DateTime::parse_from_rfc3339(exp)
            && Utc::now() > expiry
        {
            return Err(format!(
                "Proposal {} has expired (TTL exceeded)",
                accept_params.proposal_id
            )
            .into());
        }

        verify_ed25519_signature(
            &proposal.terms_hash,
            &proposal.proposer_signature,
            &proposal.proposer_public_key,
        )
        .map_err(|e| format!("Proposer signature invalid at accept: {e}"))?;

        verify_ed25519_signature(
            &proposal.terms_hash,
            &accept_params.acceptor_signature,
            &accept_params.acceptor_public_key,
        )?;

        let bond_id = uuid::Uuid::new_v4().to_string();

        let bond = IonicBond {
            bond_id: bond_id.clone(),
            proposal_id: accept_params.proposal_id,
            terms_hash: proposal.terms_hash.clone(),
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
            "Ionic bond accepted (both signatures verified)"
        );

        self.bonds.write().await.insert(bond_id, bond.clone());

        let resp = IonicBondAcceptResponse { bond };
        serde_json::to_value(resp)
            .map_err(|e| format!("Serialize: {e}"))
            .map_err(Into::into)
    }

    /// Seal an active bond by re-verifying both Ed25519 signatures and
    /// transitioning `Active` → `Sealed`. This is the explicit third step
    /// in the propose → accept → seal lifecycle.
    pub(super) async fn handle_seal(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params_value = params.ok_or("Missing params for crypto.ionic_bond.seal")?;
        let seal_params = IonicBondSealParams::deserialize(params_value)
            .map_err(|e| format!("Invalid seal params: {e}"))?;

        let sealed_bond = {
            let mut bonds = self.bonds.write().await;

            let bond = bonds
                .get_mut(&seal_params.bond_id)
                .ok_or_else(|| format!("Bond not found: {}", seal_params.bond_id))?;

            if bond.proposer != seal_params.sealer && bond.acceptor != seal_params.sealer {
                return Err(format!(
                    "Sealer '{}' is neither proposer nor acceptor",
                    seal_params.sealer
                )
                .into());
            }

            if bond.state != BondState::Active {
                let resp = IonicBondSealResponse {
                    sealed: false,
                    bond: None,
                    error: Some(format!(
                        "Bond cannot be sealed from state {:?} (must be Active)",
                        bond.state
                    )),
                };
                return serde_json::to_value(resp)
                    .map_err(|e| format!("Serialize: {e}"))
                    .map_err(Into::into);
            }

            if let Some(ref exp) = bond.expires_at
                && let Ok(expiry) = chrono::DateTime::parse_from_rfc3339(exp)
                && Utc::now() > expiry
            {
                bond.state = BondState::Expired;
                let resp = IonicBondSealResponse {
                    sealed: false,
                    bond: None,
                    error: Some("Bond has expired".to_string()),
                };
                return serde_json::to_value(resp)
                    .map_err(|e| format!("Serialize: {e}"))
                    .map_err(Into::into);
            }

            if !Self::verify_bond_signatures(bond) {
                let resp = IonicBondSealResponse {
                    sealed: false,
                    bond: None,
                    error: Some("Ed25519 signature verification failed during seal".to_string()),
                };
                return serde_json::to_value(resp)
                    .map_err(|e| format!("Serialize: {e}"))
                    .map_err(Into::into);
            }

            bond.state = BondState::Sealed;

            info!(
                bond_id = %seal_params.bond_id,
                sealer = %seal_params.sealer,
                "Ionic bond sealed — both signatures cryptographically verified"
            );

            bond.clone()
        };

        self.persistence
            .store(&sealed_bond)
            .await
            .map_err(|e| e.to_string())?;

        let resp = IonicBondSealResponse {
            sealed: true,
            bond: Some(sealed_bond),
            error: None,
        };
        serde_json::to_value(resp)
            .map_err(|e| format!("Serialize: {e}"))
            .map_err(Into::into)
    }

    pub(super) async fn handle_verify(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params_value = params.ok_or("Missing params for crypto.ionic_bond.verify")?;
        let verify_params = IonicBondVerifyParams::deserialize(params_value)
            .map_err(|e| format!("Invalid verify params: {e}"))?;

        let bonds = self.bonds.read().await;

        if let Some(bond) = bonds.get(&verify_params.bond_id) {
            let is_active = bond.state == BondState::Active || bond.state == BondState::Sealed;

            let is_expired = bond
                .expires_at
                .as_ref()
                .and_then(|exp| chrono::DateTime::parse_from_rfc3339(exp).ok())
                .is_some_and(|exp| Utc::now() > exp);

            let sigs_valid = Self::verify_bond_signatures(bond);

            let (valid, state, error) = if is_expired {
                (
                    false,
                    BondState::Expired,
                    Some("Bond is Expired".to_string()),
                )
            } else if !sigs_valid {
                (
                    false,
                    bond.state,
                    Some("Ed25519 signature verification failed".to_string()),
                )
            } else if !is_active {
                (false, bond.state, Some(format!("Bond is {:?}", bond.state)))
            } else {
                (true, bond.state, None)
            };

            let resp = IonicBondVerifyResponse {
                valid,
                state,
                bond: if valid { Some(bond.clone()) } else { None },
                error,
            };
            serde_json::to_value(resp)
                .map_err(|e| format!("Serialize: {e}"))
                .map_err(Into::into)
        } else {
            let resp = IonicBondVerifyResponse {
                valid: false,
                state: BondState::Revoked,
                bond: None,
                error: Some(format!("Bond not found: {}", verify_params.bond_id)),
            };
            serde_json::to_value(resp)
                .map_err(|e| format!("Serialize: {e}"))
                .map_err(Into::into)
        }
    }

    /// Verify a pending proposal's proposer signature without accepting it.
    ///
    /// Allows the target to inspect and validate the proposal offline before
    /// committing to acceptance. Returns the proposal terms and verification
    /// status.
    pub(super) async fn handle_verify_proposal(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params_value = params.ok_or("Missing params for crypto.ionic_bond.verify_proposal")?;

        let proposal_id = params_value
            .get("proposal_id")
            .and_then(serde_json::Value::as_str)
            .ok_or("Missing proposal_id")?;

        let proposals = self.proposals.read().await;

        if let Some(proposal) = proposals.get(proposal_id) {
            let is_expired = proposal
                .expires_at
                .as_ref()
                .and_then(|exp| chrono::DateTime::parse_from_rfc3339(exp).ok())
                .is_some_and(|exp| Utc::now() > exp);

            let sig_valid = verify_ed25519_signature(
                &proposal.terms_hash,
                &proposal.proposer_signature,
                &proposal.proposer_public_key,
            )
            .is_ok();

            let (valid, error) = if is_expired {
                (false, Some("Proposal has expired".to_string()))
            } else if !sig_valid {
                (
                    false,
                    Some("Proposer Ed25519 signature verification failed".to_string()),
                )
            } else {
                (true, None)
            };

            Ok(serde_json::json!({
                "valid": valid,
                "proposal_id": proposal_id,
                "terms_hash": proposal.terms_hash,
                "proposer": proposal.params.proposer,
                "target": proposal.params.target,
                "proposer_signature": proposal.proposer_signature,
                "proposer_public_key": proposal.proposer_public_key,
                "trust_model": proposal.params.trust_model,
                "created_at": proposal.created_at,
                "expires_at": proposal.expires_at,
                "error": error,
            }))
        } else {
            Ok(serde_json::json!({
                "valid": false,
                "proposal_id": proposal_id,
                "error": format!("Proposal not found: {proposal_id}"),
            }))
        }
    }

    /// Re-verify both proposer and acceptor Ed25519 signatures on the bond's
    /// `terms_hash`. Returns `true` only when both signatures are present and
    /// cryptographically valid.
    pub(super) fn verify_bond_signatures(bond: &IonicBond) -> bool {
        let proposer_ok = match (&bond.proposer_signature, &bond.proposer_public_key) {
            (Some(sig), Some(pk)) => verify_ed25519_signature(&bond.terms_hash, sig, pk).is_ok(),
            _ => false,
        };
        let acceptor_ok = match (&bond.acceptor_signature, &bond.acceptor_public_key) {
            (Some(sig), Some(pk)) => verify_ed25519_signature(&bond.terms_hash, sig, pk).is_ok(),
            _ => false,
        };
        proposer_ok && acceptor_ok
    }

    pub(super) async fn handle_revoke(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params_value = params.ok_or("Missing params for crypto.ionic_bond.revoke")?;
        let revoke_params = IonicBondRevokeParams::deserialize(params_value)
            .map_err(|e| format!("Invalid revoke params: {e}"))?;

        let revoked = {
            let mut bonds = self.bonds.write().await;

            if let Some(bond) = bonds.get_mut(&revoke_params.bond_id) {
                if bond.proposer != revoke_params.revoker && bond.acceptor != revoke_params.revoker
                {
                    return Err(format!(
                        "Revoker '{}' is neither proposer nor acceptor",
                        revoke_params.revoker
                    )
                    .into());
                }

                bond.state = BondState::Revoked;
                warn!(
                    bond_id = %revoke_params.bond_id,
                    revoker = %revoke_params.revoker,
                    "Ionic bond revoked"
                );

                true
            } else {
                false
            }
        };

        if revoked {
            self.persistence
                .remove(&revoke_params.bond_id)
                .await
                .map_err(|e| e.to_string())?;
            let resp = IonicBondRevokeResponse { revoked: true };
            serde_json::to_value(resp)
                .map_err(|e| format!("Serialize: {e}"))
                .map_err(Into::into)
        } else {
            Err(format!("Bond not found: {}", revoke_params.bond_id).into())
        }
    }

    pub(super) async fn handle_list(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let list_params = params
            .map(|p| {
                IonicBondListParams::deserialize(p).map_err(|e| format!("Invalid list params: {e}"))
            })
            .transpose()?;

        let mut merged: HashMap<String, IonicBond> = self
            .persistence
            .list()
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|b| (b.bond_id.clone(), b))
            .collect();

        for b in self.bonds.read().await.values() {
            merged.insert(b.bond_id.clone(), b.clone());
        }

        let filtered: Vec<IonicBond> = merged
            .into_values()
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
            .collect();

        let resp = IonicBondListResponse { bonds: filtered };
        serde_json::to_value(resp)
            .map_err(|e| format!("Serialize: {e}"))
            .map_err(Into::into)
    }
}
