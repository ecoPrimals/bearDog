// SPDX-License-Identifier: AGPL-3.0-or-later

//! Contract signing — single-party (`crypto.sign_contract`) and cross-family
//! multi-party lifecycle (`crypto.contract.propose` / `countersign` / `verify`).

use super::IonicBondHandler;
use super::crypto::{compute_contract_terms_hash, sign_terms_ed25519, verify_ed25519_signature};
use crate::btsp_provider::BeardogBtspProvider;
use beardog_types::ionic_bond::{
    ContractCountersignParams, ContractCountersignResponse, ContractProposeParams,
    ContractProposeResponse, ContractVerifyParams, ContractVerifyResponse, CrossFamilyContract,
    SignContractParams, SignContractResponse, VerifyContractParams, VerifyContractResponse,
};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use tracing::info;

/// A pending cross-family contract waiting for a countersignature.
pub(super) struct PendingContract {
    pub params: ContractProposeParams,
    pub terms_hash: String,
    pub proposer_signature: String,
    pub proposer_public_key: String,
    pub created_at: String,
    pub expires_at: Option<String>,
}

impl IonicBondHandler {
    // ── Single-party contract signing ────────────────────────────────────

    /// Sign an arbitrary contract document with `BearDog`'s Ed25519 identity.
    pub(super) async fn handle_sign_contract(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        let params_value = params.ok_or("Missing params for crypto.sign_contract")?;
        let sign_params = SignContractParams::deserialize(params_value)
            .map_err(|e| format!("Invalid sign_contract params: {e}"))?;

        let terms_hash = compute_contract_terms_hash(&sign_params.terms);
        let (signature, public_key) = sign_terms_ed25519(btsp_provider, &terms_hash)?;

        let now = Utc::now();
        let expires_at = sign_params
            .ttl_seconds
            .map(|ttl| (now + chrono::Duration::seconds(ttl.cast_signed())).to_rfc3339());

        info!(
            signer = %sign_params.signer,
            context = ?sign_params.context,
            terms_hash = %terms_hash,
            ttl_seconds = ?sign_params.ttl_seconds,
            expires_at = ?expires_at,
            "Contract signed (ionic lease)"
        );

        let resp = SignContractResponse {
            terms_hash,
            signature,
            public_key,
            signed_at: now.to_rfc3339(),
            expires_at,
        };
        serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
    }

    /// Verify a single Ed25519 signature over a contract terms hash.
    /// When `expires_at` is present, also checks ionic lease expiry.
    pub(super) async fn handle_verify_contract(
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let params_value = params.ok_or("Missing params for crypto.verify_contract")?;
        let verify_params = VerifyContractParams::deserialize(params_value)
            .map_err(|e| format!("Invalid verify_contract params: {e}"))?;

        let result = verify_ed25519_signature(
            &verify_params.terms_hash,
            &verify_params.signature,
            &verify_params.public_key,
        );

        let lease_expired = verify_params.expires_at.as_ref().and_then(|exp| {
            chrono::DateTime::parse_from_rfc3339(exp)
                .ok()
                .map(|exp_dt| Utc::now() > exp_dt)
        });

        let resp = match result {
            Ok(()) => {
                if lease_expired == Some(true) {
                    VerifyContractResponse {
                        valid: false,
                        expired: Some(true),
                        error: Some("ionic lease expired".to_string()),
                    }
                } else {
                    VerifyContractResponse {
                        valid: true,
                        expired: lease_expired,
                        error: None,
                    }
                }
            }
            Err(e) => VerifyContractResponse {
                valid: false,
                expired: lease_expired,
                error: Some(e),
            },
        };
        serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
    }

    // ── Cross-family contract lifecycle ──────────────────────────────────

    /// Propose a cross-family contract. The proposer's terms are hashed and
    /// signed; a `contract_id` is returned for the counterparty to sign.
    pub(super) async fn handle_contract_propose(
        &self,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        let params_value = params.ok_or("Missing params for crypto.contract.propose")?;
        let propose_params = ContractProposeParams::deserialize(params_value)
            .map_err(|e| format!("Invalid contract.propose params: {e}"))?;

        let terms_hash = compute_contract_terms_hash(&propose_params.terms);
        let (proposer_signature, proposer_public_key) =
            sign_terms_ed25519(btsp_provider, &terms_hash)?;

        let contract_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let created_at = now.to_rfc3339();
        let expires_at = propose_params
            .ttl_seconds
            .map(|ttl| (now + chrono::Duration::seconds(ttl.cast_signed())).to_rfc3339());

        info!(
            contract_id = %contract_id,
            proposer = %propose_params.proposer,
            context = ?propose_params.context,
            terms_hash = %terms_hash,
            created_at = %created_at,
            "Cross-family contract proposed"
        );

        self.pending_contracts.write().await.insert(
            contract_id.clone(),
            PendingContract {
                params: propose_params,
                terms_hash: terms_hash.clone(),
                proposer_signature: proposer_signature.clone(),
                proposer_public_key: proposer_public_key.clone(),
                created_at: created_at.clone(),
                expires_at: expires_at.clone(),
            },
        );

        let resp = ContractProposeResponse {
            contract_id,
            terms_hash,
            proposer_signature,
            proposer_public_key,
            created_at,
            expires_at,
        };
        serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
    }

    /// Countersign a pending cross-family contract. Verifies the counterparty's
    /// signature, then seals the contract with both parties' signatures.
    pub(super) async fn handle_contract_countersign(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let params_value = params.ok_or("Missing params for crypto.contract.countersign")?;
        let cs_params = ContractCountersignParams::deserialize(params_value)
            .map_err(|e| format!("Invalid contract.countersign params: {e}"))?;

        let pending = self
            .pending_contracts
            .write()
            .await
            .remove(&cs_params.contract_id)
            .ok_or_else(|| {
                format!(
                    "No pending contract with id '{}' (expired or already countersigned)",
                    cs_params.contract_id
                )
            })?;

        if let Some(ref exp) = pending.expires_at
            && let Ok(exp_dt) = chrono::DateTime::parse_from_rfc3339(exp)
            && Utc::now() > exp_dt
        {
            return Err(format!(
                "Contract '{}' expired at {exp}",
                cs_params.contract_id
            ));
        }

        verify_ed25519_signature(
            &pending.terms_hash,
            &cs_params.countersigner_signature,
            &cs_params.countersigner_public_key,
        )
        .map_err(|e| format!("Countersigner signature invalid: {e}"))?;

        let sealed = CrossFamilyContract {
            contract_id: cs_params.contract_id.clone(),
            terms_hash: pending.terms_hash,
            terms: pending.params.terms,
            context: pending.params.context,
            proposer: pending.params.proposer,
            proposer_signature: pending.proposer_signature,
            proposer_public_key: pending.proposer_public_key,
            countersigner: cs_params.countersigner,
            countersigner_signature: cs_params.countersigner_signature,
            countersigner_public_key: cs_params.countersigner_public_key,
            sealed_at: Utc::now().to_rfc3339(),
        };

        info!(
            contract_id = %sealed.contract_id,
            proposer = %sealed.proposer,
            countersigner = %sealed.countersigner,
            proposed_at = %pending.created_at,
            "Cross-family contract sealed (both signatures verified)"
        );

        let resp = ContractCountersignResponse { contract: sealed };
        serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
    }

    /// Verify a sealed cross-family contract — checks both signatures.
    pub(super) async fn handle_contract_verify(
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let params_value = params.ok_or("Missing params for crypto.contract.verify")?;
        let verify_params = ContractVerifyParams::deserialize(params_value)
            .map_err(|e| format!("Invalid contract.verify params: {e}"))?;

        if let Err(e) = verify_ed25519_signature(
            &verify_params.terms_hash,
            &verify_params.proposer_signature,
            &verify_params.proposer_public_key,
        ) {
            let resp = ContractVerifyResponse {
                valid: false,
                failed_party: Some("proposer".to_string()),
                error: Some(e),
            };
            return serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"));
        }

        if let Err(e) = verify_ed25519_signature(
            &verify_params.terms_hash,
            &verify_params.countersigner_signature,
            &verify_params.countersigner_public_key,
        ) {
            let resp = ContractVerifyResponse {
                valid: false,
                failed_party: Some("countersigner".to_string()),
                error: Some(e),
            };
            return serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"));
        }

        let resp = ContractVerifyResponse {
            valid: true,
            failed_party: None,
            error: None,
        };
        serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
    }
}
