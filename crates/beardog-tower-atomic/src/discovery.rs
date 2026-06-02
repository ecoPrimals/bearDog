// SPDX-License-Identifier: AGPL-3.0-or-later

//! Primal discovery via Unix socket paths

use crate::error::{Error, Result};
use beardog_config::env_keys;
use beardog_types::constants::domains::network::ipc_discovery::resolve_biomeos_ipc_subdir_from_optional;
use std::path::PathBuf;
use tracing::{debug, warn};

/// Environment paths used by [`discover_primal_socket_with`].
#[derive(Debug, Clone, Default)]
pub struct DiscoverSocketEnv {
    /// `XDG_RUNTIME_DIR` when present.
    pub xdg_runtime_dir: Option<String>,
    /// `BIOMEOS_SOCKET_DIR` when present (orchestrator-managed socket directory).
    pub biomeos_socket_dir: Option<String>,
    /// Effective user id for `/run/user/{uid}/` resolution.
    pub uid: Option<u32>,
    /// IPC namespace override (`BIOMEOS_IPC_NAMESPACE`); defaults to
    /// [`BIOMEOS_RUNTIME_SOCKET_SUBDIR`] when `None`.
    pub ipc_namespace: Option<String>,
}

impl DiscoverSocketEnv {
    /// Read discovery inputs from the process environment.
    ///
    /// Uses [`beardog_errors::process_env::var`] so tests can override values via the overlay
    /// instead of calling the soundness-critical [`std::env::set_var`] API directly.
    #[must_use]
    pub fn from_process_env() -> Self {
        Self {
            xdg_runtime_dir: beardog_errors::process_env::var(env_keys::ENV_XDG_RUNTIME_DIR).ok(),
            biomeos_socket_dir: beardog_errors::process_env::var(env_keys::ENV_BIOMEOS_SOCKET_DIR)
                .ok(),
            uid: beardog_errors::process_env::var(env_keys::ENV_UID)
                .ok()
                .and_then(|s| s.parse().ok()),
            ipc_namespace: beardog_errors::process_env::var(env_keys::ENV_BIOMEOS_IPC_NAMESPACE)
                .ok(),
        }
    }
}

/// Resolve the real UID by reading `/proc/self/status` (Linux) without `unsafe`.
fn resolve_uid_from_proc() -> Option<u32> {
    #[cfg(target_os = "linux")]
    {
        let status = std::fs::read_to_string("/proc/self/status").ok()?;
        for line in status.lines() {
            if let Some(rest) = line.strip_prefix("Uid:") {
                return rest.split_whitespace().next()?.parse().ok();
            }
        }
        None
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// Discover primal's Unix socket path using the 5-tier standard (testable).
///
/// The IPC namespace (default [`BIOMEOS_RUNTIME_SOCKET_SUBDIR`]) is resolved from
/// `env.ipc_namespace` → `BIOMEOS_IPC_NAMESPACE` env → compile-time default, so
/// deployments can override the ecosystem layout without code changes.
///
/// Search order (aligned with `beardog-core/socket_config.rs`):
/// 1. `{PRIMAL_UPPER}_SOCKET` env var (primal-specific override)
/// 2. `BIOMEOS_SOCKET_DIR/{primal}.sock` (orchestrator-managed directory)
/// 3. `XDG_RUNTIME_DIR/{namespace}/{primal}.sock`
/// 4. `/run/user/{uid}/{namespace}/{primal}.sock`
/// 5. `{std::env::temp_dir()}/{namespace}/{primal}.sock` (last-resort fallback)
pub async fn discover_primal_socket_with(
    primal_name: &str,
    env: &DiscoverSocketEnv,
) -> Result<PathBuf> {
    let namespace = resolve_biomeos_ipc_subdir_from_optional(env.ipc_namespace.as_deref());

    // Tier 1: primal-specific env var (e.g. BEARDOG_SOCKET, {PRIMAL}_SOCKET)
    let env_key = format!("{}_SOCKET", primal_name.to_uppercase().replace('-', "_"));
    if let Ok(val) = beardog_errors::process_env::var(&env_key)
        && !val.is_empty()
    {
        let p = PathBuf::from(&val);
        if p.exists() {
            debug!("Found {} via {} (Tier 1)", primal_name, env_key);
            return Ok(p);
        }
    }

    // Tier 2: orchestrator-managed directory
    if let Some(ref dir) = env.biomeos_socket_dir {
        let socket_path = PathBuf::from(dir).join(format!("{primal_name}.sock"));
        if socket_path.exists() {
            debug!("Found {} via BIOMEOS_SOCKET_DIR (Tier 2)", primal_name);
            return Ok(socket_path);
        }
    }

    // Tier 3: XDG runtime (ecosystem namespace)
    if let Some(ref xdg_runtime) = env.xdg_runtime_dir {
        let socket_path = PathBuf::from(xdg_runtime)
            .join(&namespace)
            .join(format!("{primal_name}.sock"));
        if socket_path.exists() {
            debug!(
                "Found {} via XDG_RUNTIME_DIR/{} (Tier 3)",
                primal_name, namespace
            );
            return Ok(socket_path);
        }
    }

    // Tier 4: /run/user/{uid}/{namespace}/
    let uid = env
        .uid
        .unwrap_or_else(|| resolve_uid_from_proc().unwrap_or(1000));
    let run_path = PathBuf::from(format!("/run/user/{uid}"))
        .join(&namespace)
        .join(format!("{primal_name}.sock"));
    if run_path.exists() {
        debug!(
            "Found {} via /run/user/{}/{} (Tier 4)",
            primal_name, uid, namespace
        );
        return Ok(run_path);
    }

    // Tier 5: OS temp directory + namespace/ (last resort; avoids hardcoding `/tmp`)
    let tmp_path = std::env::temp_dir()
        .join(&namespace)
        .join(format!("{primal_name}.sock"));
    if tmp_path.exists() {
        debug!("Found {} via {} (Tier 5)", primal_name, tmp_path.display());
        return Ok(tmp_path);
    }

    warn!("Primal not found: {}", primal_name);
    Err(Error::PrimalNotFound(format!(
        "{primal_name} (searched 5-tier: env, BIOMEOS_SOCKET_DIR, \
         XDG/{namespace}, /run/user/{namespace}, temp_dir/{namespace})"
    )))
}

/// Discover primal's Unix socket path using [`DiscoverSocketEnv::from_process_env`].
pub async fn discover_primal_socket(primal_name: &str) -> Result<PathBuf> {
    discover_primal_socket_with(primal_name, &DiscoverSocketEnv::from_process_env()).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::constants::domains::network::ipc_discovery::BIOMEOS_RUNTIME_SOCKET_SUBDIR;
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_discover_via_biomeos_socket_dir() {
        let dir = tempdir().expect("tempdir for BIOMEOS_SOCKET_DIR test");
        let biomeos_dir = dir.path().join("sockets");
        fs::create_dir_all(&biomeos_dir).expect("create biomeos sockets dir");

        let socket_path = biomeos_dir.join("test_primal.sock");
        fs::File::create(&socket_path).expect("create placeholder socket file");

        let env = DiscoverSocketEnv {
            xdg_runtime_dir: None,
            biomeos_socket_dir: Some(biomeos_dir.to_string_lossy().into_owned()),
            uid: None,
            ipc_namespace: None,
        };

        let result = discover_primal_socket_with("test_primal", &env).await;
        assert!(result.is_ok());
        assert_eq!(
            result.expect("discover primal socket via BIOMEOS_SOCKET_DIR"),
            socket_path
        );
    }

    #[tokio::test]
    async fn test_discover_via_xdg_biomeos() {
        let dir = tempdir().expect("tempdir for XDG biomeos test");
        let biomeos_dir = dir.path().join(BIOMEOS_RUNTIME_SOCKET_SUBDIR);
        fs::create_dir_all(&biomeos_dir).expect("create XDG biomeos dir");

        let socket_path = biomeos_dir.join("test_primal.sock");
        fs::File::create(&socket_path).expect("create placeholder socket file");

        let env = DiscoverSocketEnv {
            xdg_runtime_dir: Some(dir.path().to_string_lossy().into_owned()),
            biomeos_socket_dir: None,
            uid: None,
            ipc_namespace: None,
        };

        let result = discover_primal_socket_with("test_primal", &env).await;
        assert!(result.is_ok());
        assert_eq!(
            result.expect("discover primal socket via XDG_RUNTIME_DIR/biomeos"),
            socket_path
        );
    }

    #[tokio::test]
    async fn test_primal_not_found() {
        let env = DiscoverSocketEnv {
            xdg_runtime_dir: None,
            biomeos_socket_dir: None,
            uid: Some(99999),
            ipc_namespace: None,
        };

        let result = discover_primal_socket_with("nonexistent_primal", &env).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::PrimalNotFound(_))));
    }
}
