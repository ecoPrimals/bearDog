// SPDX-License-Identifier: AGPL-3.0-or-later

use super::IonicBondHandler;
use super::crypto::{compute_contract_terms_hash, sign_terms_ed25519, verify_ed25519_signature};
use crate::btsp_provider::BeardogBtspProvider;
use beardog_types::ionic_bond::{
    SignContractParams, SignContractResponse, VerifyContractParams, VerifyContractResponse,
};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use tracing::info;

impl IonicBondHandler {
    /// Sign an arbitrary contract document with `BearDog`'s Ed25519 identity.
    ///
    /// The terms JSON is serialized canonically (sorted keys), SHA-256 hashed,
    /// then signed. Returns the terms hash, signature, and public key so any
    /// party can independently verify the contract.
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

        info!(
            signer = %sign_params.signer,
            context = ?sign_params.context,
            terms_hash = %terms_hash,
            "Contract signed"
        );

        let resp = SignContractResponse {
            terms_hash,
            signature,
            public_key,
            signed_at: Utc::now().to_rfc3339(),
        };
        serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
    }

    /// Verify an Ed25519 signature over a contract terms hash.
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

        let resp = match result {
            Ok(()) => VerifyContractResponse {
                valid: true,
                error: None,
            },
            Err(e) => VerifyContractResponse {
                valid: false,
                error: Some(e),
            },
        };
        serde_json::to_value(resp).map_err(|e| format!("Serialize: {e}"))
    }
}
