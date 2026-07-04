// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::ionic_token::{IonicTokenPayload, TokenError};
use serde::{Deserialize, Serialize};

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

/// Error returned when issuer registration fails.
#[derive(Debug, thiserror::Error)]
pub enum RegisterError {
    /// The supplied DID does not match the canonical DID derived from the key.
    #[error("DID does not match public key (expected {expected})")]
    DidKeyMismatch {
        /// The DID that the key actually produces.
        expected: String,
    },
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

    pub(crate) fn parse(s: &str) -> Option<Self> {
        match s {
            "family_seed" => Some(Self::FamilySeed),
            "contract_exchange" => Some(Self::ContractExchange),
            "manual" => Some(Self::Manual),
            _ => None,
        }
    }
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

#[derive(Serialize, Deserialize)]
pub struct PersistedRegistry {
    pub(crate) version: u32,
    pub(crate) issuers: Vec<PersistedIssuer>,
}

#[derive(Serialize, Deserialize)]
pub struct PersistedIssuer {
    pub(crate) did: String,
    pub(crate) public_key_b64: String,
    pub(crate) gate_id: Option<String>,
    pub(crate) family_id: Option<String>,
    pub(crate) trust_method: String,
    pub(crate) registered_at: i64,
}
