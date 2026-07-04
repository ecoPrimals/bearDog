// SPDX-License-Identifier: AGPL-3.0-or-later

use ed25519_dalek::VerifyingKey;

use crate::ionic_token::{TokenError, verify_ionic_token};

use super::registry::TrustedIssuerRegistry;
use super::types::CrossGateVerifyResult;

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

    // 2. Try registry issuers — signature must verify AND payload.iss must
    //    match the registered DID (prevents key-confusion attacks).
    {
        let inner = registry
            .inner
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for (vk, info) in inner.issuers.values() {
            match verify_ionic_token(token_str, vk) {
                Ok(payload) if payload.iss == info.did => {
                    return CrossGateVerifyResult::RemoteVerified {
                        payload,
                        issuer_info: info.clone(),
                    };
                }
                Ok(_) => {} // sig valid but iss mismatch — skip
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
