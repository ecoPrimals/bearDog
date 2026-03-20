// SPDX-License-Identifier: AGPL-3.0-only

//! Primal discovery via Unix socket paths

use crate::error::{Error, Result};
use std::path::PathBuf;
use tracing::{debug, warn};

/// Environment paths used by [`discover_primal_socket_with`].
#[derive(Debug, Clone, Default)]
pub struct DiscoverSocketEnv {
    /// `XDG_RUNTIME_DIR` when present.
    pub xdg_runtime_dir: Option<String>,
    /// User home directory (`HOME`).
    pub home: Option<String>,
}

impl DiscoverSocketEnv {
    /// Read `XDG_RUNTIME_DIR` and `HOME` from the process environment.
    #[must_use]
    pub fn from_process_env() -> Self {
        Self {
            xdg_runtime_dir: std::env::var("XDG_RUNTIME_DIR").ok(),
            home: std::env::var("HOME").ok(),
        }
    }
}

/// Discover primal's Unix socket path using explicit environment paths (testable).
///
/// Search order:
/// 1. XDG_RUNTIME_DIR/ecoPrimals/{primal}.sock
/// 2. HOME/.local/share/ecoPrimals/{primal}.sock
/// 3. /var/run/ecoPrimals/{primal}.sock
/// 4. /tmp/ecoPrimals/{primal}.sock
pub async fn discover_primal_socket_with(
    primal_name: &str,
    env: &DiscoverSocketEnv,
) -> Result<PathBuf> {
    // 1. Check XDG runtime dir (preferred for user services)
    if let Some(xdg_runtime) = &env.xdg_runtime_dir {
        let socket_path = PathBuf::from(format!("{xdg_runtime}/ecoPrimals/{primal_name}.sock"));

        if socket_path.exists() {
            debug!("✅ Found {} via XDG_RUNTIME_DIR", primal_name);
            return Ok(socket_path);
        }
    }

    // 2. Check home dir (for user-level primals)
    if let Some(home) = &env.home {
        let socket_path =
            PathBuf::from(format!("{home}/.local/share/ecoPrimals/{primal_name}.sock"));

        if socket_path.exists() {
            debug!("✅ Found {} via HOME", primal_name);
            return Ok(socket_path);
        }
    }

    // 3. Check /var/run (for system services)
    let socket_path = PathBuf::from(format!("/var/run/ecoPrimals/{primal_name}.sock"));
    if socket_path.exists() {
        debug!("✅ Found {} via /var/run", primal_name);
        return Ok(socket_path);
    }

    // 4. Check /tmp (fallback)
    let socket_path = PathBuf::from(format!("/tmp/ecoPrimals/{primal_name}.sock"));
    if socket_path.exists() {
        debug!("✅ Found {} via /tmp", primal_name);
        return Ok(socket_path);
    }

    // Not found
    warn!("❌ Primal not found: {}", primal_name);
    Err(Error::PrimalNotFound(format!(
        "{primal_name} (searched XDG_RUNTIME_DIR, HOME, /var/run, /tmp)"
    )))
}

/// Discover primal's Unix socket path using [`DiscoverSocketEnv::from_process_env`].
pub async fn discover_primal_socket(primal_name: &str) -> Result<PathBuf> {
    discover_primal_socket_with(primal_name, &DiscoverSocketEnv::from_process_env()).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_discover_via_home() {
        let dir = tempdir().unwrap();
        let ecoprimals_dir = dir.path().join(".local/share/ecoPrimals");
        fs::create_dir_all(&ecoprimals_dir).unwrap();

        let socket_path = ecoprimals_dir.join("test_primal.sock");
        fs::File::create(&socket_path).unwrap();

        let env = DiscoverSocketEnv {
            xdg_runtime_dir: None,
            home: Some(dir.path().to_string_lossy().into_owned()),
        };

        let result = discover_primal_socket_with("test_primal", &env).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), socket_path);
    }

    #[tokio::test]
    async fn test_primal_not_found() {
        let env = DiscoverSocketEnv {
            xdg_runtime_dir: None,
            home: Some("/nonexistent".to_string()),
        };

        let result = discover_primal_socket_with("nonexistent_primal", &env).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::PrimalNotFound(_))));
    }
}
