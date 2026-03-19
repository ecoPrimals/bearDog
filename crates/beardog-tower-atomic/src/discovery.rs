// SPDX-License-Identifier: AGPL-3.0-only

//! Primal discovery via Unix socket paths

use crate::error::{Error, Result};
use std::path::PathBuf;
use tracing::{debug, warn};

/// Discover primal's Unix socket path
///
/// Search order:
/// 1. XDG_RUNTIME_DIR/ecoPrimals/{primal}.sock
/// 2. HOME/.local/share/ecoPrimals/{primal}.sock
/// 3. /var/run/ecoPrimals/{primal}.sock
/// 4. /tmp/ecoPrimals/{primal}.sock
pub async fn discover_primal_socket(primal_name: &str) -> Result<PathBuf> {
    // 1. Check XDG runtime dir (preferred for user services)
    if let Ok(xdg_runtime) = std::env::var("XDG_RUNTIME_DIR") {
        let socket_path = PathBuf::from(format!("{}/ecoPrimals/{}.sock", xdg_runtime, primal_name));

        if socket_path.exists() {
            debug!("✅ Found {} via XDG_RUNTIME_DIR", primal_name);
            return Ok(socket_path);
        }
    }

    // 2. Check home dir (for user-level primals)
    if let Ok(home) = std::env::var("HOME") {
        let socket_path = PathBuf::from(format!(
            "{}/.local/share/ecoPrimals/{}.sock",
            home, primal_name
        ));

        if socket_path.exists() {
            debug!("✅ Found {} via HOME", primal_name);
            return Ok(socket_path);
        }
    }

    // 3. Check /var/run (for system services)
    let socket_path = PathBuf::from(format!("/var/run/ecoPrimals/{}.sock", primal_name));
    if socket_path.exists() {
        debug!("✅ Found {} via /var/run", primal_name);
        return Ok(socket_path);
    }

    // 4. Check /tmp (fallback)
    let socket_path = PathBuf::from(format!("/tmp/ecoPrimals/{}.sock", primal_name));
    if socket_path.exists() {
        debug!("✅ Found {} via /tmp", primal_name);
        return Ok(socket_path);
    }

    // Not found
    warn!("❌ Primal not found: {}", primal_name);
    Err(Error::PrimalNotFound(format!(
        "{} (searched XDG_RUNTIME_DIR, HOME, /var/run, /tmp)",
        primal_name
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_discover_via_home() {
        // Create temporary directory structure
        let dir = tempdir().unwrap();
        let ecoprimals_dir = dir.path().join(".local/share/ecoPrimals");
        fs::create_dir_all(&ecoprimals_dir).unwrap();

        let socket_path = ecoprimals_dir.join("test_primal.sock");
        fs::File::create(&socket_path).unwrap();

        // Set HOME to temp dir
        std::env::set_var("HOME", dir.path());

        // Discover
        let result = discover_primal_socket("test_primal").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), socket_path);
    }

    #[tokio::test]
    async fn test_primal_not_found() {
        // Clear environment to ensure failure
        std::env::remove_var("XDG_RUNTIME_DIR");
        std::env::set_var("HOME", "/nonexistent");

        let result = discover_primal_socket("nonexistent_primal").await;
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::PrimalNotFound(_))));
    }
}
