// SPDX-License-Identifier: AGPL-3.0-only

//! Primal IPC discovery layout (XDG runtime dir + biomeOS socket namespace).
//!
//! Resolution order for socket directories:
//! 1. `BEARDOG_BIOMEOS_SOCKET_DIR` — operator / test override for the whole directory
//! 2. `$XDG_RUNTIME_DIR/biomeos/` — standard biomeOS layout
//! 3. `{std::env::temp_dir()}/biomeos/` — last-resort default (configurable via temp dir)

use std::path::PathBuf;

/// Subdirectory under [`XDG_RUNTIME_DIR`](std::env::var_os) for primal Unix sockets.
pub const BIOMEOS_RUNTIME_SOCKET_SUBDIR: &str = "biomeos";

/// Default UPA / service-registry listener socket (filename only, under the biomeOS dir).
pub const DEFAULT_UPA_REGISTRY_SOCKET_NAME: &str = "registry.sock";

/// File stem reserved for the registry listener (excluded from generic primal socket scan).
pub const DEFAULT_UPA_REGISTRY_SOCKET_STEM: &str = "registry";

/// Override the biomeOS IPC socket directory (operators, integration tests).
pub const ENV_BIOMEOS_SOCKET_DIR_OVERRIDE: &str = "BEARDOG_BIOMEOS_SOCKET_DIR";

/// Primary registry endpoint (URI or `unix://` path). See wateringHole PRIMAL IPC docs.
pub const ENV_BEARDOG_REGISTRY_ENDPOINT: &str = "BEARDOG_REGISTRY_ENDPOINT";

/// Canonical service-registry endpoint env (used elsewhere in the workspace).
pub const ENV_BEARDOG_SERVICE_REGISTRY_ENDPOINT: &str = "BEARDOG_SERVICE_REGISTRY_ENDPOINT";

/// Legacy UPA registry address env.
pub const ENV_UPA_REGISTRY_ADDR: &str = "UPA_REGISTRY_ADDR";

/// Resolve `$XDG_RUNTIME_DIR/biomeos` (or override / temp fallback) from process environment.
#[must_use]
pub fn biomeos_ipc_socket_dir() -> PathBuf {
    biomeos_ipc_socket_dir_from_components(None, None)
}

/// Resolve biomeOS IPC dir using explicit components (e.g. from a captured env map).
///
/// Precedence: `socket_dir_override` → `xdg_runtime_dir`/biomeos → temp dir fallback.
#[must_use]
pub fn biomeos_ipc_socket_dir_from_components(
    socket_dir_override: Option<&str>,
    xdg_runtime_dir: Option<&str>,
) -> PathBuf {
    if let Some(p) = socket_dir_override.filter(|s| !s.is_empty()) {
        return PathBuf::from(p);
    }
    if let Some(xdg) = xdg_runtime_dir.filter(|s| !s.is_empty()) {
        return PathBuf::from(xdg).join(BIOMEOS_RUNTIME_SOCKET_SUBDIR);
    }
    std::env::temp_dir().join(BIOMEOS_RUNTIME_SOCKET_SUBDIR)
}

/// Same layout as [`biomeos_ipc_socket_dir`], but reads `BEARDOG_BIOMEOS_SOCKET_DIR` and `XDG_RUNTIME_DIR` from the process environment.
#[must_use]
pub fn biomeos_ipc_socket_dir_from_env() -> PathBuf {
    biomeos_ipc_socket_dir_from_components(
        std::env::var_os(ENV_BIOMEOS_SOCKET_DIR_OVERRIDE)
            .as_ref()
            .and_then(|s| s.to_str()),
        std::env::var_os("XDG_RUNTIME_DIR")
            .as_ref()
            .and_then(|s| s.to_str()),
    )
}

/// Default `unix://` URI for the UPA / registry socket under the resolved biomeOS dir.
#[must_use]
pub fn default_upa_registry_unix_uri() -> String {
    let path = biomeos_ipc_socket_dir().join(DEFAULT_UPA_REGISTRY_SOCKET_NAME);
    format!("unix://{}", path.display())
}

/// Resolve registry endpoint: env chain then default `unix://` path under biomeOS dir.
#[must_use]
pub fn resolve_upa_registry_endpoint() -> String {
    default_upa_registry_unix_uri()
}

/// Resolve registry endpoint from environment, then fall back to default `unix://` path.
#[must_use]
pub fn resolve_upa_registry_endpoint_from_env() -> String {
    resolve_upa_registry_endpoint_from_optional(
        std::env::var(ENV_BEARDOG_REGISTRY_ENDPOINT).ok().as_deref(),
        std::env::var(ENV_BEARDOG_SERVICE_REGISTRY_ENDPOINT)
            .ok()
            .as_deref(),
        std::env::var(ENV_UPA_REGISTRY_ADDR).ok().as_deref(),
    )
}

#[must_use]
fn resolve_upa_registry_endpoint_from_optional(
    primary: Option<&str>,
    service: Option<&str>,
    legacy_upa: Option<&str>,
) -> String {
    for candidate in [primary, service, legacy_upa] {
        if let Some(v) = candidate.map(str::trim).filter(|s| !s.is_empty()) {
            return v.to_string();
        }
    }
    default_upa_registry_unix_uri()
}
