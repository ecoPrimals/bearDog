// SPDX-License-Identifier: AGPL-3.0-only

//! Primal IPC discovery layout (XDG runtime dir + biomeOS socket namespace).
//!
//! Resolution order for socket directories:
//! 1. `BEARDOG_BIOMEOS_SOCKET_DIR` — operator / test override for the whole directory
//! 2. `$XDG_RUNTIME_DIR/biomeos/` — standard biomeOS layout
//! 3. `{std::env::temp_dir()}/biomeos/` — last-resort default (configurable via temp dir)
//!
//! Environment:
//! - `BIOMEOS_IPC_NAMESPACE` — subdirectory name under XDG/temp (default: [`BIOMEOS_RUNTIME_SOCKET_SUBDIR`])
//! - `BIOMEOS_TMP_ROOT` — root for temp fallback layout (default: `/tmp`)

use std::path::PathBuf;

/// Subdirectory under [`XDG_RUNTIME_DIR`](std::env::var_os) for primal Unix sockets.
pub const BIOMEOS_RUNTIME_SOCKET_SUBDIR: &str = "biomeos";

/// Override the IPC subdirectory name (e.g. `biomeos`) without changing the full socket directory.
pub const ENV_BIOMEOS_IPC_NAMESPACE: &str = "BIOMEOS_IPC_NAMESPACE";

/// Root directory for `BIOMEOS_TMP_ROOT/<namespace>/` when `XDG_RUNTIME_DIR` is unset.
pub const ENV_BIOMEOS_TMP_ROOT: &str = "BIOMEOS_TMP_ROOT";

/// Filename for TCP port discovery files under XDG, home, and [`biomeos_tmp_socket_root`] search paths.
pub const BEARDOG_TCP_DISCOVERY_FILENAME: &str = "beardog-ipc-port";

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

/// Resolve IPC subdirectory name: explicit `ipc_namespace`, then [`ENV_BIOMEOS_IPC_NAMESPACE`], then default.
#[must_use]
pub fn resolve_biomeos_ipc_subdir_from_optional(ipc_namespace: Option<&str>) -> String {
    if let Some(s) = ipc_namespace.map(str::trim).filter(|s| !s.is_empty()) {
        return s.to_string();
    }
    std::env::var(ENV_BIOMEOS_IPC_NAMESPACE)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map_or_else(
            || BIOMEOS_RUNTIME_SOCKET_SUBDIR.to_string(),
            |s| s.trim().to_string(),
        )
}

/// Root directory for temp fallback IPC layout (`<root>/<namespace>/`).
#[must_use]
pub fn biomeos_tmp_socket_root() -> PathBuf {
    std::env::var(ENV_BIOMEOS_TMP_ROOT)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map_or_else(|| PathBuf::from("/tmp"), PathBuf::from)
}

/// Resolve `$XDG_RUNTIME_DIR/biomeos` (or override / temp fallback) from process environment.
#[must_use]
pub fn biomeos_ipc_socket_dir() -> PathBuf {
    biomeos_ipc_socket_dir_from_components(None, None, None)
}

/// Resolve biomeOS IPC dir using explicit components (e.g. from a captured env map).
///
/// Precedence: `socket_dir_override` → `xdg_runtime_dir`/`ipc_namespace` → temp fallback.
///
/// `ipc_namespace`: when `Some`, uses that name; when `None`, uses [`resolve_biomeos_ipc_subdir_from_optional`].
#[must_use]
pub fn biomeos_ipc_socket_dir_from_components(
    socket_dir_override: Option<&str>,
    xdg_runtime_dir: Option<&str>,
    ipc_namespace: Option<&str>,
) -> PathBuf {
    if let Some(p) = socket_dir_override.filter(|s| !s.is_empty()) {
        return PathBuf::from(p);
    }
    let subdir = resolve_biomeos_ipc_subdir_from_optional(ipc_namespace);
    if let Some(xdg) = xdg_runtime_dir.filter(|s| !s.is_empty()) {
        return PathBuf::from(xdg).join(&subdir);
    }
    biomeos_tmp_socket_root().join(&subdir)
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
        None,
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
