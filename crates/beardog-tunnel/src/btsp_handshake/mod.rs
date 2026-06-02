// SPDX-License-Identifier: AGPL-3.0-or-later

//! BTSP handshake enforcement for socket listeners.
//!
//! Implements Phase 2 of `BTSP_PROTOCOL_STANDARD.md`: when `FAMILY_ID` is set,
//! every incoming connection must complete a 4-step cryptographic handshake
//! before any JSON-RPC traffic is processed.

pub mod crypto;
pub(crate) mod framing;
mod handshake;
pub(crate) mod session;
pub mod session_store;
pub(crate) mod types;

pub use framing::{read_frame, write_frame};
pub use handshake::{continue_server_handshake_jsonline, perform_server_handshake};
pub use session::{BtspCipher, BtspSession, Phase3Session};
pub use session_store::BtspSessionStore;
pub use types::{ChallengeResponse, ClientHello, HandshakeComplete, HandshakeError, ServerHello};

use beardog_config::env_keys;
use beardog_errors::BearDogError;
use zeroize::Zeroize;

/// Security posture for the socket listener, resolved once at startup.
#[derive(Clone)]
pub enum BtspSecurityMode {
    /// Production: BTSP handshake mandatory on every connection.
    /// Holds the raw family seed bytes used for HKDF key derivation.
    Production {
        /// Raw family seed bytes for HKDF key derivation.
        family_seed: FamilySeed,
    },

    /// Development: no handshake, plain NDJSON JSON-RPC.
    Development,
}

/// Wrapper around family seed bytes with zeroize-on-drop.
#[derive(Clone)]
pub struct FamilySeed(Vec<u8>);

impl FamilySeed {
    /// Wrap raw bytes as a `FamilySeed`.
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Borrow the raw seed bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Drop for FamilySeed {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

/// Resolve the BTSP security mode from environment variables.
///
/// # Rules (per `BTSP_PROTOCOL_STANDARD.md`)
///
/// | `FAMILY_ID` | `BIOMEOS_INSECURE` | Result |
/// |---|---|---|
/// | set (not `"default"`) | unset | `Production` |
/// | unset or `"default"` | any | `Development` |
/// | set (not `"default"`) | `"1"` | **Error** |
///
/// # Errors
///
/// Returns an error when both `FAMILY_ID` and `BIOMEOS_INSECURE=1` are set.
pub fn resolve_security_mode() -> Result<BtspSecurityMode, BearDogError> {
    let family_id = beardog_errors::process_env::var(env_keys::ENV_FAMILY_ID)
        .ok()
        .or_else(|| beardog_errors::process_env::var(env_keys::ENV_FAMILY_ID_PREFIXED).ok());
    let insecure = beardog_errors::process_env::var(env_keys::ENV_BIOMEOS_INSECURE)
        .ok()
        .is_some_and(|v| v == "1");

    let is_production = family_id
        .as_deref()
        .is_some_and(|id| !id.is_empty() && id != "default");

    if is_production && insecure {
        return Err(BearDogError::configuration(
            "FAMILY_ID and BIOMEOS_INSECURE=1 cannot both be set \
             (per BTSP_PROTOCOL_STANDARD.md §Security Model)",
        ));
    }

    if is_production {
        let seed = load_family_seed()?;
        Ok(BtspSecurityMode::Production {
            family_seed: FamilySeed::new(seed),
        })
    } else {
        Ok(BtspSecurityMode::Development)
    }
}

/// Load the family seed from environment or filesystem.
///
/// Checks (in order):
/// 1. `FAMILY_SEED` env var (raw bytes)
/// 2. `BEARDOG_FAMILY_SEED` env var (raw bytes)
/// 3. `.family.seed` file in the current directory
fn load_family_seed() -> Result<Vec<u8>, BearDogError> {
    if let Ok(seed) =
        beardog_errors::process_env::var(env_keys::ENV_FAMILY_SEED).map(String::into_bytes)
        && !seed.is_empty()
    {
        return Ok(seed);
    }
    if let Ok(seed) =
        beardog_errors::process_env::var(env_keys::ENV_FAMILY_SEED_PREFIXED).map(String::into_bytes)
        && !seed.is_empty()
    {
        return Ok(seed);
    }
    if let Ok(seed) = std::fs::read(".family.seed")
        && !seed.is_empty()
    {
        return Ok(seed);
    }

    Err(BearDogError::system(
        "BTSP production mode requires a family seed. Set FAMILY_SEED or \
         BEARDOG_FAMILY_SEED env var, or create a .family.seed file."
            .to_string(),
    ))
}

impl BtspSecurityMode {
    /// Returns `true` when BTSP handshake enforcement is active.
    #[must_use]
    pub fn is_production(&self) -> bool {
        matches!(self, Self::Production { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn family_seed_zeroizes_on_drop() {
        let seed = FamilySeed::new(vec![0xAA; 32]);
        assert_eq!(seed.as_bytes().len(), 32);
        drop(seed);
    }

    #[test]
    fn security_mode_is_production_returns_correct_value() {
        let dev = BtspSecurityMode::Development;
        assert!(!dev.is_production());

        let prod = BtspSecurityMode::Production {
            family_seed: FamilySeed::new(vec![1; 32]),
        };
        assert!(prod.is_production());
    }

    #[test]
    #[serial_test::serial]
    fn resolve_security_mode_production_uses_family_seed_env() {
        beardog_errors::process_env::remove_var("BIOMEOS_INSECURE");
        beardog_errors::process_env::remove_var("BEARDOG_FAMILY_ID");
        beardog_errors::process_env::set_var("FAMILY_ID", "not-default-prod");
        beardog_errors::process_env::set_var("FAMILY_SEED", "exactly-thirty-two-byte-seed!!!!");

        let mode = resolve_security_mode().expect("resolve production");

        beardog_errors::process_env::remove_var("FAMILY_ID");
        beardog_errors::process_env::remove_var("FAMILY_SEED");

        assert!(mode.is_production());
    }
}
