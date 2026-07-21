// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unified storage trait system.
//!
//! Provides the ecosystem [`CredentialStore`] contract — the canonical abstraction
//! for name→value secret storage across all bearDog backends. This is the
//! **Silicon Atheism evolving edge**: platform-native secret stores (DPAPI,
//! Android Keystore, macOS Keychain) are backends behind this trait, not
//! `#[cfg]`-gated code paths.
//!
//! # Two-Tier Model (bearDog + squirrel)
//!
//! ```text
//! squirrel (consumer)            bearDog (authority)
//! ─────────────────────          ──────────────────────────
//! Memory / File / Platform       InMemory / FileVault / PlatformNative
//! = cache + bootstrap            = HSM-backed, persistent, family-scoped
//! SecurityProvider IPC ────────► secrets.store / secrets.retrieve
//! ```
//!
//! Ownership: squirrel **caches**, bearDog **stores**. Native OS credential
//! backends live exclusively in bearDog behind [`CredentialStore`].

use beardog_errors::BearDogError;
use std::future::Future;

/// Metadata returned alongside a retrieved secret.
#[derive(Debug, Clone)]
pub struct SecretMetadata {
    /// ISO 8601 timestamp of when the secret was stored.
    pub stored_at: String,
}

/// Ecosystem credential store contract.
///
/// Abstracts name→value secret storage with pluggable backends:
/// in-memory (dev/test), encrypted file vault (production default),
/// or platform-native stores (DPAPI, Keychain, Android Keystore).
///
/// All operations are keyed by a string `name`. Values are opaque strings
/// (typically base64-encoded key material or API tokens). The store is
/// responsible for encryption-at-rest — callers should not pre-encrypt.
///
/// Backends that cannot support a given operation (e.g., a read-only
/// platform store) return [`BearDogError`] with an appropriate category.
pub trait CredentialStore: Send + Sync {
    /// Persist a secret value under `name`, overwriting any existing entry.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] on I/O, encryption, or backend-specific failures.
    fn store(
        &self,
        name: &str,
        value: &str,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send;

    /// Retrieve the plaintext secret for `name`.
    ///
    /// Returns the value and optional metadata (storage timestamp, etc.).
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] if the secret does not exist, cannot be
    /// decrypted, or the backend is unavailable.
    fn retrieve(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<(String, SecretMetadata), BearDogError>> + Send;

    /// List all stored secret names (never values).
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] on backend I/O failures.
    fn list(&self) -> impl Future<Output = Result<Vec<String>, BearDogError>> + Send;

    /// Delete the secret for `name`. Returns `true` if a secret existed and
    /// was removed, `false` if `name` was not found.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] on I/O or backend-specific failures.
    fn delete(&self, name: &str) -> impl Future<Output = Result<bool, BearDogError>> + Send;

    /// Human-readable backend identifier (e.g. `"in-memory"`, `"file-vault"`,
    /// `"windows-dpapi"`, `"linux-secret-service"`).
    fn backend_id(&self) -> &'static str;

    /// Whether this backend persists across process restarts.
    fn is_persistent(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_metadata_debug() {
        let m = SecretMetadata {
            stored_at: "2026-07-21T12:00:00Z".to_string(),
        };
        let dbg = format!("{m:?}");
        assert!(dbg.contains("2026-07-21"));
    }
}
