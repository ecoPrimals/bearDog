// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared Handler Utilities
//!
//! Common utilities used across multiple JSON-RPC handlers.
//!
//! ## Self-Knowledge Pattern
//!
//! Identity values can be injected via [`IdentityHints`] for tests; production code uses
//! [`IdentityHints::from_env`] or the `_from_env` convenience wrappers.

use beardog_config::env_keys::{self, resolve_primal_name};

/// Identity strings for self-knowledge helpers (no I/O in [`Default`]).
#[derive(Debug, Clone, Default)]
pub struct IdentityHints {
    /// `BEARDOG_PRIMAL_NAME` / `PRIMAL_NAME`
    pub primal_name: Option<String>,
    /// `FAMILY_ID`
    pub family_id: Option<String>,
    /// `BIOMEOS_FAMILY`
    pub biomeos_family: Option<String>,
    /// `NODE_ID`
    pub node_id: Option<String>,
    /// `HOSTNAME`
    pub hostname: Option<String>,
}

impl IdentityHints {
    /// Load from [`beardog_errors::process_env`].
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            primal_name: beardog_errors::process_env::var(env_keys::ENV_PRIMAL_NAME_PREFIXED)
                .ok()
                .or_else(|| beardog_errors::process_env::var(env_keys::ENV_PRIMAL_NAME).ok()),
            family_id: beardog_errors::process_env::var(env_keys::ENV_FAMILY_ID_PREFIXED)
                .ok()
                .or_else(|| beardog_errors::process_env::var(env_keys::ENV_FAMILY_ID).ok()),
            biomeos_family: beardog_errors::process_env::var(env_keys::ENV_BIOMEOS_FAMILY).ok(),
            node_id: beardog_errors::process_env::var(env_keys::ENV_NODE_ID_PREFIXED)
                .ok()
                .or_else(|| beardog_errors::process_env::var(env_keys::ENV_NODE_ID).ok()),
            hostname: beardog_errors::process_env::var(env_keys::ENV_HOSTNAME).ok(),
        }
    }
}

/// Resolves primal name from explicit hints.
#[must_use]
pub fn get_primal_name_with(h: &IdentityHints) -> String {
    h.primal_name.clone().unwrap_or_else(resolve_primal_name)
}

/// Get the primal name using self-knowledge pattern (reads environment).
#[must_use]
pub fn get_primal_name() -> String {
    get_primal_name_with(&IdentityHints::from_env())
}

/// Resolves family id from explicit hints.
///
/// Fallback chain: `BEARDOG_FAMILY_ID` / `FAMILY_ID` → `BIOMEOS_FAMILY` → `"standalone"`
/// (aligned with `PrimalIdentity::from_env`).
#[must_use]
pub fn get_family_id_with(h: &IdentityHints) -> String {
    h.family_id
        .clone()
        .or_else(|| h.biomeos_family.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| beardog_types::primal_identity::DEFAULT_STANDALONE_FAMILY.to_string())
}

/// Get the family ID using self-knowledge pattern (reads environment).
#[must_use]
pub fn get_family_id() -> String {
    get_family_id_with(&IdentityHints::from_env())
}

/// Resolves node id from explicit hints.
///
/// Fallback chain: `BEARDOG_NODE_ID` / `NODE_ID` → `HOSTNAME` → ephemeral
/// `standalone-{uuid}` (same as `PrimalIdentity::from_env` and `SocketConfig`).
#[must_use]
pub fn get_node_id_with(h: &IdentityHints) -> String {
    h.node_id
        .clone()
        .or_else(|| h.hostname.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(beardog_types::primal_identity::resolve_process_node_id)
}

/// Get the node ID using self-knowledge pattern (reads environment).
#[must_use]
pub fn get_node_id() -> String {
    get_node_id_with(&IdentityHints::from_env())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_primal_name_default() {
        assert_eq!(
            get_primal_name_with(&IdentityHints::default()),
            beardog_config::env_keys::DEFAULT_PRIMAL_NAME
        );
    }

    #[test]
    fn test_get_primal_name_from_env() {
        let h = IdentityHints {
            primal_name: Some("test_primal".to_string()),
            ..Default::default()
        };
        assert_eq!(get_primal_name_with(&h), "test_primal");
    }

    #[test]
    fn test_get_family_id_default() {
        assert_eq!(get_family_id_with(&IdentityHints::default()), "standalone");
    }

    #[test]
    fn test_get_node_id_default() {
        let h = IdentityHints {
            node_id: Some("node-1".to_string()),
            ..Default::default()
        };
        assert_eq!(get_node_id_with(&h), "node-1");
    }
}
