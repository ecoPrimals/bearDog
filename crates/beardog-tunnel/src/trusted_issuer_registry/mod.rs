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

mod did;
mod persistence;
mod registry;
mod types;
mod verify;

#[cfg(test)]
mod tests;

pub use did::{did_from_verifying_key, did_matches_key};
pub use registry::TrustedIssuerRegistry;
pub use types::{
    CrossGateVerifyResult, IssuerInfo, RegisterError, TrustMethod,
};
pub use verify::verify_with_registry;
