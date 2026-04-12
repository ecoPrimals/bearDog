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
use beardog_types::ionic_bond::{IonicBond, IonicBondProposeParams};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

mod contract;
mod crypto;
mod lifecycle;

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
            "crypto.sign_contract",
            "crypto.verify_contract",
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
            "crypto.sign_contract" => self.handle_sign_contract(params, btsp_provider).await,
            "crypto.verify_contract" => Self::handle_verify_contract(params).await,
            _ => Err(format!("Unknown ionic bond method: {method}")),
        }
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
