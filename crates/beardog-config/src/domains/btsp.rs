// SPDX-License-Identifier: AGPL-3.0-or-later

//! BTSP / `BirdSong` defaults for the tunnel stack.
//!
//! Values mirror documented fallbacks in `configs/beardog-primal-capabilities.toml` and are
//! overridden via `BEARDOG_*` environment variables at runtime.

use crate::env_keys;

/// HSM key label for `BirdSong` master material (`generate_key`).
pub const DEFAULT_BTSP_BIRDSONG_KEY_LABEL: &str = "birdsong_master";

/// Default prefix for `BirdSong` lineage root ids in session setup hints.
pub const DEFAULT_BTSP_LINEAGE_ROOT_PREFIX: &str = "btsp_root";

/// Default maximum lineage depth for BTSP session hints.
pub const DEFAULT_BTSP_LINEAGE_MAX_DEPTH: u32 = 10;

/// Returns the HSM key label for `BirdSong` master material, reading
/// the env var when set or falling back to [`DEFAULT_BTSP_BIRDSONG_KEY_LABEL`].
#[must_use]
pub fn resolve_btsp_birdsong_key_label() -> String {
    std::env::var(env_keys::ENV_BTSP_BIRDSONG_KEY_LABEL)
        .unwrap_or_else(|_| DEFAULT_BTSP_BIRDSONG_KEY_LABEL.to_string())
}

/// Returns the prefix used for `BirdSong` lineage root identifiers in session hints.
#[must_use]
pub fn resolve_btsp_lineage_root_prefix() -> String {
    std::env::var(env_keys::ENV_BTSP_LINEAGE_ROOT_PREFIX)
        .unwrap_or_else(|_| DEFAULT_BTSP_LINEAGE_ROOT_PREFIX.to_string())
}

/// Returns the maximum lineage depth for BTSP session hints.
#[must_use]
pub fn resolve_btsp_lineage_max_depth() -> u32 {
    std::env::var(env_keys::ENV_BTSP_LINEAGE_MAX_DEPTH)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_BTSP_LINEAGE_MAX_DEPTH)
}
