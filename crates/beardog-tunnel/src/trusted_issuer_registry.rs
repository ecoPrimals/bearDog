// SPDX-License-Identifier: AGPL-3.0-or-later

//! Cross-gate trusted issuer registry for multi-gate ionic token verification.
//!
//! When gates form a covalent mesh, each gate issues ionic tokens signed by its
//! own Ed25519 identity key.  A remote gate's token cannot be verified with the
//! local key — the verifier needs the remote gate's public key.
//!
//! This registry maintains a map of `issuer DID → VerifyingKey` so that
//! [`verify_with_registry`] can transparently verify tokens from any trusted
//! gate.
//!
//! ## Trust establishment
//!
//! 1. **Same family (covalent):** After BTSP handshake proves shared
//!    `FAMILY_SEED`, gates exchange `auth.public_key` and register each
//!    other via [`TrustedIssuerRegistry::register`].
//!
//! 2. **Cross-family (ionic):** Bilateral trust via `crypto.contract.*`
//!    key exchange — one gate calls the other's `auth.public_key` and
//!    registers it after contract verification.
//!
//! ## Security invariants
//!
//! - Only keys registered via explicit trust establishment are accepted.
//! - Registration is append-only (no silent replacement).
//! - Each entry includes metadata (`family_id`, `registered_at`) for audit.

use ed25519_dalek::VerifyingKey;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::ionic_token::{IonicTokenPayload, TokenError, verify_ionic_token};

/// Metadata associated with a trusted issuer.
#[derive(Debug, Clone)]
pub struct IssuerInfo {
    /// The issuer's `did:key:z6Mk...` DID.
    pub did: String,
    /// The gate's `NODE_ID` (if known).
    pub gate_id: Option<String>,
    /// The gate's `FAMILY_ID` (if known).
    pub family_id: Option<String>,
    /// Unix timestamp when this issuer was registered.
    pub registered_at: i64,
    /// How the trust was established.
    pub trust_method: TrustMethod,
}

/// How trust was established with a remote issuer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustMethod {
    /// Same family seed — BTSP handshake proved membership.
    FamilySeed,
    /// Explicit key exchange via `crypto.contract.*`.
    ContractExchange,
    /// Manual operator registration.
    Manual,
}

impl TrustMethod {
    /// Human-readable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FamilySeed => "family_seed",
            Self::ContractExchange => "contract_exchange",
            Self::Manual => "manual",
        }
    }
}

/// Thread-safe registry of trusted remote issuers.
///
/// Keyed by issuer DID string, stores the `VerifyingKey` and metadata.
#[derive(Debug, Clone)]
pub struct TrustedIssuerRegistry {
    inner: Arc<RwLock<RegistryInner>>,
}

#[derive(Debug)]
struct RegistryInner {
    issuers: HashMap<String, (VerifyingKey, IssuerInfo)>,
}

/// Result of a cross-gate verification attempt.
#[derive(Debug)]
pub enum CrossGateVerifyResult {
    /// Token verified by the local gate's own key.
    LocalVerified(IonicTokenPayload),
    /// Token verified by a registered remote issuer.
    RemoteVerified {
        /// The verified token claims.
        payload: IonicTokenPayload,
        /// Metadata about the trusted issuer that verified the token.
        issuer_info: IssuerInfo,
    },
    /// Token verified by an ad-hoc key supplied in the request.
    AdHocVerified(IonicTokenPayload),
    /// Verification failed against all available keys.
    Failed(TokenError),
}

impl TrustedIssuerRegistry {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(RegistryInner {
                issuers: HashMap::new(),
            })),
        }
    }

    /// Register a trusted remote issuer.
    ///
    /// Returns `true` if the issuer was newly registered, `false` if
    /// already present (existing entry is preserved — no silent replacement).
    pub fn register(
        &self,
        did: &str,
        key: VerifyingKey,
        gate_id: Option<String>,
        family_id: Option<String>,
        method: TrustMethod,
    ) -> bool {
        let mut inner = self
            .inner
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if inner.issuers.contains_key(did) {
            return false;
        }
        let info = IssuerInfo {
            did: did.to_owned(),
            gate_id,
            family_id,
            registered_at: chrono::Utc::now().timestamp(),
            trust_method: method,
        };
        inner.issuers.insert(did.to_owned(), (key, info));
        true
    }

    /// Look up a trusted issuer by DID.
    #[must_use]
    pub fn get(&self, did: &str) -> Option<(VerifyingKey, IssuerInfo)> {
        let inner = self
            .inner
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        inner.issuers.get(did).cloned()
    }

    /// Number of registered issuers.
    #[must_use]
    pub fn len(&self) -> usize {
        let inner = self
            .inner
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        inner.issuers.len()
    }

    /// Whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// List all registered issuer DIDs with metadata (for `auth.trusted_issuers`).
    #[must_use]
    pub fn list(&self) -> Vec<IssuerInfo> {
        let inner = self
            .inner
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        inner
            .issuers
            .values()
            .map(|(_, info)| info.clone())
            .collect()
    }

    /// Remove a trusted issuer by DID. Returns `true` if removed.
    pub fn remove(&self, did: &str) -> bool {
        let mut inner = self
            .inner
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        inner.issuers.remove(did).is_some()
    }
}

impl Default for TrustedIssuerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Verify a token against the local key, then the trusted issuer registry,
/// then an optional ad-hoc key.
///
/// This is the core cross-gate verification function.  It tries keys in
/// priority order:
///
/// 1. **Local key** — the gate's own `VerifyingKey` (fast path for local tokens)
/// 2. **Registry** — iterate registered remote issuers
/// 3. **Ad-hoc key** — an optional key supplied in the RPC params
///
/// Returns [`CrossGateVerifyResult`] indicating which key (if any) verified.
pub fn verify_with_registry(
    token_str: &str,
    local_key: &VerifyingKey,
    registry: &TrustedIssuerRegistry,
    adhoc_key: Option<&VerifyingKey>,
) -> CrossGateVerifyResult {
    // 1. Try local key
    match verify_ionic_token(token_str, local_key) {
        Ok(payload) => return CrossGateVerifyResult::LocalVerified(payload),
        Err(TokenError::InvalidSignature) => {} // try other keys
        Err(e) => return CrossGateVerifyResult::Failed(e), // structural / expiry errors are terminal
    }

    // 2. Try registry issuers
    {
        let inner = registry
            .inner
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for (vk, info) in inner.issuers.values() {
            match verify_ionic_token(token_str, vk) {
                Ok(payload) => {
                    return CrossGateVerifyResult::RemoteVerified {
                        payload,
                        issuer_info: info.clone(),
                    };
                }
                Err(TokenError::InvalidSignature) => {}
                Err(e) => return CrossGateVerifyResult::Failed(e),
            }
        }
    }

    // 3. Try ad-hoc key
    if let Some(adhoc) = adhoc_key {
        match verify_ionic_token(token_str, adhoc) {
            Ok(payload) => return CrossGateVerifyResult::AdHocVerified(payload),
            Err(TokenError::InvalidSignature) => {}
            Err(e) => return CrossGateVerifyResult::Failed(e),
        }
    }

    CrossGateVerifyResult::Failed(TokenError::InvalidSignature)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ionic_token::issue_ionic_token;
    use crate::unix_socket_ipc::handlers::primal_signing::{
        derive_primal_signing_key, derive_primal_verifying_key,
    };

    const GATE_A_PRIMAL: &str = "beardog";
    const GATE_A_NODE: &str = "southgate-node-1";
    const GATE_B_PRIMAL: &str = "beardog";
    const GATE_B_NODE: &str = "eastgate-node-1";

    fn issue_token(primal: &str, node: &str, subject: &str) -> String {
        let sk = derive_primal_signing_key(primal, node);
        issue_ionic_token(&sk, "did:key:test", subject, &["*".to_owned()], 3600)
    }

    #[test]
    fn local_token_verifies_without_registry() {
        let local_vk = derive_primal_verifying_key(GATE_A_PRIMAL, GATE_A_NODE);
        let token = issue_token(GATE_A_PRIMAL, GATE_A_NODE, "alice");
        let registry = TrustedIssuerRegistry::new();

        let result = verify_with_registry(&token, &local_vk, &registry, None);
        assert!(matches!(result, CrossGateVerifyResult::LocalVerified(_)));
    }

    #[test]
    fn remote_token_fails_without_registry() {
        let local_vk = derive_primal_verifying_key(GATE_A_PRIMAL, GATE_A_NODE);
        let token = issue_token(GATE_B_PRIMAL, GATE_B_NODE, "bob");
        let registry = TrustedIssuerRegistry::new();

        let result = verify_with_registry(&token, &local_vk, &registry, None);
        assert!(matches!(
            result,
            CrossGateVerifyResult::Failed(TokenError::InvalidSignature)
        ));
    }

    #[test]
    fn remote_token_verifies_with_registered_issuer() {
        let local_vk = derive_primal_verifying_key(GATE_A_PRIMAL, GATE_A_NODE);
        let remote_vk = derive_primal_verifying_key(GATE_B_PRIMAL, GATE_B_NODE);
        let token = issue_token(GATE_B_PRIMAL, GATE_B_NODE, "bob");

        let registry = TrustedIssuerRegistry::new();
        registry.register(
            "did:key:eastgate",
            remote_vk,
            Some("eastgate-node-1".to_owned()),
            Some("family-alpha".to_owned()),
            TrustMethod::FamilySeed,
        );

        let result = verify_with_registry(&token, &local_vk, &registry, None);
        match result {
            CrossGateVerifyResult::RemoteVerified {
                payload,
                issuer_info,
            } => {
                assert_eq!(payload.sub, "bob");
                assert_eq!(issuer_info.gate_id.as_deref(), Some("eastgate-node-1"));
                assert_eq!(issuer_info.trust_method, TrustMethod::FamilySeed);
            }
            other => panic!("expected RemoteVerified, got {other:?}"),
        }
    }

    #[test]
    fn adhoc_key_verifies_unregistered_issuer() {
        let local_vk = derive_primal_verifying_key(GATE_A_PRIMAL, GATE_A_NODE);
        let remote_vk = derive_primal_verifying_key(GATE_B_PRIMAL, GATE_B_NODE);
        let token = issue_token(GATE_B_PRIMAL, GATE_B_NODE, "charlie");
        let registry = TrustedIssuerRegistry::new();

        let result = verify_with_registry(&token, &local_vk, &registry, Some(&remote_vk));
        assert!(matches!(result, CrossGateVerifyResult::AdHocVerified(_)));
    }

    #[test]
    fn register_is_idempotent() {
        let registry = TrustedIssuerRegistry::new();
        let vk = derive_primal_verifying_key(GATE_B_PRIMAL, GATE_B_NODE);

        assert!(registry.register("did:key:b", vk, None, None, TrustMethod::Manual));
        assert!(!registry.register("did:key:b", vk, None, None, TrustMethod::Manual));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn list_returns_all_issuers() {
        let registry = TrustedIssuerRegistry::new();
        let vk_b = derive_primal_verifying_key(GATE_B_PRIMAL, GATE_B_NODE);
        let vk_c = derive_primal_verifying_key("beardog", "westgate-node-1");

        registry.register(
            "did:key:b",
            vk_b,
            Some("eastgate-node-1".to_owned()),
            None,
            TrustMethod::FamilySeed,
        );
        registry.register(
            "did:key:c",
            vk_c,
            Some("westgate-node-1".to_owned()),
            None,
            TrustMethod::ContractExchange,
        );

        let issuers = registry.list();
        assert_eq!(issuers.len(), 2);
    }

    #[test]
    fn remove_issuer() {
        let registry = TrustedIssuerRegistry::new();
        let vk = derive_primal_verifying_key(GATE_B_PRIMAL, GATE_B_NODE);
        registry.register("did:key:b", vk, None, None, TrustMethod::Manual);
        assert_eq!(registry.len(), 1);
        assert!(registry.remove("did:key:b"));
        assert!(registry.is_empty());
        assert!(!registry.remove("did:key:b"));
    }

    #[test]
    fn cross_gate_roundtrip_full_flow() {
        // Simulate: Gate A and Gate B share a family.
        // Gate B issues a token → Gate A should verify it after trust exchange.

        // Gate A's local key
        let gate_a_vk = derive_primal_verifying_key(GATE_A_PRIMAL, GATE_A_NODE);

        // Gate B issues a token
        let gate_b_sk = derive_primal_signing_key(GATE_B_PRIMAL, GATE_B_NODE);
        let gate_b_vk = derive_primal_verifying_key(GATE_B_PRIMAL, GATE_B_NODE);

        let token = crate::ionic_token::issue_ionic_token_with_gate(
            &gate_b_sk,
            "did:key:gate-b",
            "cross-gate-user",
            &["crypto.*".to_owned()],
            3600,
            Some(&crate::ionic_token::GateIdentity {
                node_id: GATE_B_NODE.to_owned(),
                family_id: "family-alpha".to_owned(),
            }),
        );

        // Gate A registers Gate B as trusted (simulating post-BTSP key exchange)
        let registry = TrustedIssuerRegistry::new();
        registry.register(
            "did:key:gate-b",
            gate_b_vk,
            Some(GATE_B_NODE.to_owned()),
            Some("family-alpha".to_owned()),
            TrustMethod::FamilySeed,
        );

        // Gate A verifies Gate B's token
        let result = verify_with_registry(&token, &gate_a_vk, &registry, None);
        match result {
            CrossGateVerifyResult::RemoteVerified {
                payload,
                issuer_info,
            } => {
                assert_eq!(payload.sub, "cross-gate-user");
                assert_eq!(payload.gate_id.as_deref(), Some(GATE_B_NODE));
                assert_eq!(payload.family_id.as_deref(), Some("family-alpha"));
                assert_eq!(issuer_info.trust_method, TrustMethod::FamilySeed);
            }
            other => panic!("expected RemoteVerified, got {other:?}"),
        }
    }
}
