// SPDX-License-Identifier: AGPL-3.0-only

//! # Unix Socket Configuration
//!
//! This module provides a robust, Primal IPC Protocol compliant socket path configuration
//! with a 5-tier fallback system (TRUE PRIMAL architecture):
//!
//! 1. **Primal-Specific** (highest priority): `BEARDOG_SOCKET`
//! 2. **Generic Orchestrator**: `BIOMEOS_SOCKET_PATH` or `BIOMEOS_SOCKET_DIR`
//! 3. **Primal IPC Protocol Standard**: `/primal/{PRIMAL_NAME}` (discovery-based)
//! 4. **XDG Runtime Directory**: `/run/user/<uid>/biomeos/{PRIMAL_NAME}.sock` (biomeOS standard)
//! 5. **Temp Directory** (last resort): `/tmp/{PRIMAL_NAME}-<family>-<node>.sock`
//!
//! ## Self-Knowledge via PRIMAL_NAME
//!
//! The primal name is discovered via the `PRIMAL_NAME` environment variable (defaults to "beardog").
//! This follows the TRUE PRIMAL principle: primals only know themselves and discover others at runtime.
//!
//! ## Primal IPC Protocol Compliance
//!
//! Per `/wateringHole/PRIMAL_IPC_PROTOCOL.md`:
//! - Standard namespace: `/primal/{primal-name}`
//! - Path is constructed from PRIMAL_NAME (capability-based, not hardcoded)
//!
//! ## Security & Standards
//!
//! - ✅ Primal IPC Protocol compliant
//! - ✅ XDG Base Directory Specification compliant
//! - ✅ Per-user runtime directories (`/run/user/<uid>/`)
//! - ✅ Automatic directory creation with proper permissions
//! - ✅ Old socket cleanup (prevents "address already in use")
//! - ✅ Multi-instance support via family/node IDs
//! - ✅ Neural API orchestration support (BIOMEOS_SOCKET_PATH)
//!
//! ## Usage
//!
//! ```rust
//! use beardog_core::socket_config::SocketConfig;
//!
//! // Get socket path using environment-driven configuration
//! let config = SocketConfig::from_env();
//! let socket_path = config.socket_path();
//!
//! println!("Socket: {}", socket_path.display());
//! // Output: /tmp/beardog-default-default.sock (or as set by Neural API)
//! ```

use std::fs;
use std::path::{Path, PathBuf};

/// All inputs needed to resolve a [`SocketConfig`] without reading the process environment.
///
/// Use [`SocketPathInputs::from_env`] at process boundaries; tests should construct values
/// directly for concurrency-safe, deterministic behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocketPathInputs {
    /// Tier 1: `BEARDOG_SOCKET` (empty string is treated as unset).
    pub beardog_socket: Option<String>,
    /// Tier 2: `BIOMEOS_SOCKET_PATH`
    pub biomeos_socket_path: Option<String>,
    /// Tier 2: `BIOMEOS_SOCKET_DIR` (joins `beardog.sock`)
    pub biomeos_socket_dir: Option<String>,
    /// Used for tier 3–5 path construction; defaults to `"beardog"` in resolution.
    pub primal_name: Option<String>,
    /// Resolved family id (`BEARDOG_FAMILY_ID` / `FAMILY_ID`); default `"default"`.
    pub family_id: Option<String>,
    /// Resolved node id (`BEARDOG_NODE_ID` / `NODE_ID`); default `"default"`.
    pub node_id: Option<String>,
    /// User id for tier 4 (`/run/user/<uid>/...`); defaults to `1000` in [`Default`].
    pub uid: u32,
    /// When `true`, tier 3 uses `/primal/<primal_name>` if tier 1–2 do not apply.
    pub primal_namespace_root_exists: bool,
}

impl Default for SocketPathInputs {
    fn default() -> Self {
        Self {
            beardog_socket: None,
            biomeos_socket_path: None,
            biomeos_socket_dir: None,
            primal_name: None,
            family_id: None,
            node_id: None,
            uid: 1000,
            primal_namespace_root_exists: false,
        }
    }
}

impl SocketPathInputs {
    /// Read configuration from the process environment (read-only, thread-safe `std::env::var`).
    #[must_use]
    pub fn from_env() -> Self {
        let family_id = std::env::var("BEARDOG_FAMILY_ID")
            .ok()
            .or_else(|| std::env::var("FAMILY_ID").ok());
        let node_id = std::env::var("BEARDOG_NODE_ID")
            .ok()
            .or_else(|| std::env::var("NODE_ID").ok());
        let uid = std::env::var("UID")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1000);

        Self {
            beardog_socket: std::env::var("BEARDOG_SOCKET").ok(),
            biomeos_socket_path: std::env::var("BIOMEOS_SOCKET_PATH").ok(),
            biomeos_socket_dir: std::env::var("BIOMEOS_SOCKET_DIR").ok(),
            primal_name: std::env::var("PRIMAL_NAME").ok(),
            family_id,
            node_id,
            uid,
            primal_namespace_root_exists: Path::new("/primal").exists(),
        }
    }
}

/// Socket configuration with 5-tier fallback logic
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocketConfig {
    /// Final resolved socket path
    socket_path: PathBuf,
    /// Family ID for this instance
    family_id: String,
    /// Node ID for this instance
    node_id: String,
    /// Which tier was used (for diagnostics)
    source: SocketPathSource,
}

/// Indicates which tier of fallback logic was used
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketPathSource {
    /// Primal-specific environment variable (`BEARDOG_SOCKET`, highest priority)
    PrimalEnvVar,
    /// Generic orchestrator environment variable (`BIOMEOS_SOCKET_PATH`)
    OrchestratorEnvVar,
    /// Primal IPC Protocol standard namespace (/primal/beardog)
    PrimalNamespace,
    /// XDG Runtime Directory (preferred)
    XdgRuntime,
    /// Temp directory (last resort)
    TempDir,
}

impl SocketConfig {
    /// Resolve socket configuration from explicit inputs (no environment reads).
    ///
    /// Implements 5-tier fallback (Primal IPC Protocol compliant):
    /// 1. `beardog_socket` (primal-specific, highest priority)
    /// 2. `biomeos_socket_path` or `biomeos_socket_dir` (generic orchestrator)
    /// 3. `/primal/{primal-name}` when `primal_namespace_root_exists`
    /// 4. `/run/user/<uid>/biomeos/beardog.sock` when the XDG runtime dir exists
    /// 5. `/tmp/{primal-name}-{family}-{node}.sock` (fallback)
    #[must_use]
    pub fn from_inputs(inputs: &SocketPathInputs) -> Self {
        let family_id = inputs
            .family_id
            .clone()
            .unwrap_or_else(|| "default".to_string());
        let node_id = inputs
            .node_id
            .clone()
            .unwrap_or_else(|| "default".to_string());

        // Tier 1: primal-specific BEARDOG_SOCKET
        if let Some(ref socket_path) = inputs.beardog_socket
            && !socket_path.is_empty()
        {
            return Self {
                socket_path: PathBuf::from(socket_path),
                family_id,
                node_id,
                source: SocketPathSource::PrimalEnvVar,
            };
        }

        // Tier 2: BIOMEOS_SOCKET_PATH or BIOMEOS_SOCKET_DIR
        if let Some(ref socket_path) = inputs.biomeos_socket_path {
            if !socket_path.is_empty() {
                return Self {
                    socket_path: PathBuf::from(socket_path),
                    family_id,
                    node_id,
                    source: SocketPathSource::OrchestratorEnvVar,
                };
            }
        } else if let Some(ref socket_dir) = inputs.biomeos_socket_dir
            && !socket_dir.is_empty()
        {
            return Self {
                socket_path: PathBuf::from(socket_dir).join("beardog.sock"),
                family_id,
                node_id,
                source: SocketPathSource::OrchestratorEnvVar,
            };
        }

        let primal_name = inputs
            .primal_name
            .clone()
            .unwrap_or_else(|| "beardog".to_string());

        // Tier 3: Primal IPC Protocol standard namespace
        if inputs.primal_namespace_root_exists {
            return Self {
                socket_path: PathBuf::from(format!("/primal/{primal_name}")),
                family_id,
                node_id,
                source: SocketPathSource::PrimalNamespace,
            };
        }

        // Tier 4: XDG Runtime Directory
        if let Some(xdg_path) = Self::try_xdg_runtime(inputs.uid) {
            return Self {
                socket_path: xdg_path,
                family_id,
                node_id,
                source: SocketPathSource::XdgRuntime,
            };
        }

        // Tier 5: /tmp fallback
        let tmp_path = format!("/tmp/{primal_name}-{family_id}-{node_id}.sock");
        Self {
            socket_path: PathBuf::from(tmp_path),
            family_id,
            node_id,
            source: SocketPathSource::TempDir,
        }
    }

    /// Create socket configuration from environment variables.
    ///
    /// Thin wrapper: [`SocketPathInputs::from_env`] then [`SocketConfig::from_inputs`].
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_inputs(&SocketPathInputs::from_env())
    }

    /// Try to use XDG Runtime Directory
    ///
    /// Returns `Some(path)` if `/run/user/<uid>/` exists, otherwise `None`
    ///
    /// Creates socket at `/run/user/<uid>/biomeos/beardog.sock` for biomeOS integration.
    /// The `/biomeos/` subdirectory groups all biomeOS primal sockets together for
    /// easy discovery and management.
    fn try_xdg_runtime(uid: u32) -> Option<PathBuf> {
        let xdg_runtime_dir = format!("/run/user/{uid}");
        if Path::new(&xdg_runtime_dir).exists() {
            Some(PathBuf::from(format!(
                "{xdg_runtime_dir}/biomeos/beardog.sock"
            )))
        } else {
            None
        }
    }

    /// Get the resolved socket path
    #[must_use]
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    /// Get socket path as string
    #[must_use]
    pub fn socket_path_string(&self) -> String {
        self.socket_path.display().to_string()
    }

    /// Get family ID
    #[must_use]
    pub fn family_id(&self) -> &str {
        &self.family_id
    }

    /// Get node ID
    #[must_use]
    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    /// Get the source tier that was used
    #[must_use]
    pub const fn source(&self) -> SocketPathSource {
        self.source
    }

    /// Prepare the socket path for binding
    ///
    /// This ensures:
    /// 1. Parent directory exists (creates if needed)
    /// 2. Old socket file is removed (prevents "address already in use")
    ///
    /// Call this before binding to the socket.
    pub fn prepare(&self) -> Result<(), String> {
        // Ensure parent directory exists
        if let Some(parent) = self.socket_path.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent).map_err(|e| {
                format!(
                    "Failed to create socket directory {}: {}",
                    parent.display(),
                    e
                )
            })?;
        }

        // Remove old socket file if it exists
        if self.socket_path.exists() {
            fs::remove_file(&self.socket_path).map_err(|e| {
                format!(
                    "Failed to remove old socket {}: {}",
                    self.socket_path.display(),
                    e
                )
            })?;
        }

        Ok(())
    }

    /// Get a descriptive string for logging
    #[must_use]
    pub fn description(&self) -> String {
        match self.source {
            SocketPathSource::PrimalEnvVar => {
                format!(
                    "{} (from BEARDOG_SOCKET env var ⭐ Tier 1)",
                    self.socket_path_string()
                )
            }
            SocketPathSource::OrchestratorEnvVar => {
                format!(
                    "{} (from BIOMEOS_SOCKET_PATH env var ⭐ Tier 2 - Neural API)",
                    self.socket_path_string()
                )
            }
            SocketPathSource::PrimalNamespace => {
                format!(
                    "{} (Primal IPC Protocol standard namespace - Tier 3)",
                    self.socket_path_string()
                )
            }
            SocketPathSource::XdgRuntime => {
                format!(
                    "{} (XDG Runtime Directory - Tier 4)",
                    self.socket_path_string()
                )
            }
            SocketPathSource::TempDir => {
                format!("{} (fallback to /tmp - Tier 5)", self.socket_path_string())
            }
        }
    }

    /// Create a custom socket configuration (for testing)
    pub fn custom(socket_path: impl Into<PathBuf>, family_id: String, node_id: String) -> Self {
        Self {
            socket_path: socket_path.into(),
            family_id,
            node_id,
            source: SocketPathSource::PrimalEnvVar,
        }
    }
}

impl std::fmt::Display for SocketConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_var_override_takes_priority() {
        let config = SocketConfig::from_inputs(&SocketPathInputs {
            beardog_socket: Some("/tmp/custom-override.sock".to_string()),
            family_id: Some("test0".to_string()),
            ..Default::default()
        });

        assert_eq!(
            config.socket_path_string(),
            "/tmp/custom-override.sock",
            "BEARDOG_SOCKET env var must take highest priority (Tier 1)"
        );
        assert_eq!(
            config.source(),
            SocketPathSource::PrimalEnvVar,
            "Source should be PrimalEnvVar when BEARDOG_SOCKET is set"
        );
        assert_eq!(config.family_id(), "test0");
    }

    #[test]
    fn test_empty_socket_path_rejected() {
        let config = SocketConfig::from_inputs(&SocketPathInputs {
            beardog_socket: Some(String::new()),
            family_id: Some("test".to_string()),
            primal_namespace_root_exists: false,
            ..Default::default()
        });

        assert_ne!(config.socket_path_string(), "");
        assert_ne!(config.source(), SocketPathSource::PrimalEnvVar);

        assert!(
            config.source() == SocketPathSource::XdgRuntime
                || config.source() == SocketPathSource::TempDir
        );
    }

    #[test]
    fn test_empty_biomeos_socket_rejected() {
        let config = SocketConfig::from_inputs(&SocketPathInputs {
            biomeos_socket_path: Some(String::new()),
            family_id: Some("test".to_string()),
            primal_namespace_root_exists: false,
            ..Default::default()
        });

        assert_ne!(config.socket_path_string(), "");
        assert_ne!(config.source(), SocketPathSource::OrchestratorEnvVar);

        assert!(
            config.source() == SocketPathSource::XdgRuntime
                || config.source() == SocketPathSource::TempDir
        );
    }

    #[test]
    fn test_biomeos_socket_path_tier2() {
        let config = SocketConfig::from_inputs(&SocketPathInputs {
            biomeos_socket_path: Some("/tmp/beardog-default-default.sock".to_string()),
            family_id: Some("nat0".to_string()),
            primal_namespace_root_exists: false,
            ..Default::default()
        });

        assert_eq!(
            config.socket_path_string(),
            "/tmp/beardog-default-default.sock",
            "BIOMEOS_SOCKET_PATH should be honored (Tier 2)"
        );
        assert_eq!(
            config.source(),
            SocketPathSource::OrchestratorEnvVar,
            "Source should be OrchestratorEnvVar when BIOMEOS_SOCKET_PATH is set"
        );
    }

    #[test]
    fn test_beardog_socket_overrides_biomeos_socket_path() {
        let config = SocketConfig::from_inputs(&SocketPathInputs {
            beardog_socket: Some("/custom/beardog-specific.sock".to_string()),
            biomeos_socket_path: Some("/tmp/biomeos-generic.sock".to_string()),
            ..Default::default()
        });

        assert_eq!(
            config.socket_path_string(),
            "/custom/beardog-specific.sock",
            "BEARDOG_SOCKET (Tier 1) should override BIOMEOS_SOCKET_PATH (Tier 2)"
        );
        assert_eq!(
            config.source(),
            SocketPathSource::PrimalEnvVar,
            "Source should be PrimalEnvVar when BEARDOG_SOCKET is set"
        );
    }

    #[test]
    fn test_xdg_runtime_preferred_over_tmp() {
        let config = SocketConfig::from_inputs(&SocketPathInputs {
            family_id: Some("xdg-test".to_string()),
            primal_namespace_root_exists: false,
            ..Default::default()
        });

        match config.source() {
            SocketPathSource::XdgRuntime => {
                assert!(config.socket_path_string().contains("/run/user/"));
                assert!(config.socket_path_string().contains("biomeos/beardog.sock"));
            }
            SocketPathSource::TempDir => {
                assert!(config.socket_path_string().starts_with("/tmp/beardog-"));
            }
            _ => panic!("Unexpected source: {:?}", config.source()),
        }
    }

    #[test]
    fn test_fallback_to_tmp_with_node_id() {
        let config = SocketConfig::from_inputs(&SocketPathInputs {
            family_id: Some("fallback".to_string()),
            node_id: Some("node123".to_string()),
            primal_namespace_root_exists: false,
            ..Default::default()
        });

        if config.source() == SocketPathSource::TempDir {
            assert_eq!(
                config.socket_path_string(),
                "/tmp/beardog-fallback-node123.sock"
            );
        } else if config.source() == SocketPathSource::XdgRuntime {
            // XDG path when /run/user/<uid> exists
            assert!(config.socket_path_string().contains("/run/user/"));
        }
    }

    #[test]
    fn test_default_family_and_node_ids() {
        let config = SocketConfig::from_inputs(&SocketPathInputs {
            primal_namespace_root_exists: false,
            ..Default::default()
        });

        assert_eq!(config.family_id(), "default");
        assert_eq!(config.node_id(), "default");

        if config.source() == SocketPathSource::TempDir {
            assert_eq!(
                config.socket_path_string(),
                "/tmp/beardog-default-default.sock"
            );
        }
    }

    #[test]
    fn test_primal_namespace_tier3_when_root_exists() {
        let config = SocketConfig::from_inputs(&SocketPathInputs {
            primal_namespace_root_exists: true,
            primal_name: None,
            ..Default::default()
        });
        assert_eq!(config.source(), SocketPathSource::PrimalNamespace);
        assert_eq!(config.socket_path_string(), "/primal/beardog");
    }

    #[test]
    fn test_description_format() {
        let config = SocketConfig::from_inputs(&SocketPathInputs {
            beardog_socket: Some("/custom/socket.sock".to_string()),
            ..Default::default()
        });

        let desc = config.description();
        assert!(desc.contains("/custom/socket.sock"));
        assert!(desc.contains("BEARDOG_SOCKET"));
    }

    #[test]
    fn test_custom_config() {
        let config = SocketConfig::custom(
            "/test/custom.sock",
            "family1".to_string(),
            "node1".to_string(),
        );

        assert_eq!(config.socket_path_string(), "/test/custom.sock");
        assert_eq!(config.family_id(), "family1");
        assert_eq!(config.node_id(), "node1");
    }

    #[test]
    fn test_prepare_removes_old_socket() {
        // Create a temporary directory for testing
        let test_dir = std::env::temp_dir().join("beardog-socket-test");
        fs::create_dir_all(&test_dir).expect("Failed to create test directory");

        let socket_path = test_dir.join("test-socket.sock");

        // Create an old socket file
        fs::write(&socket_path, b"old socket").expect("Failed to write test socket file");
        assert!(socket_path.exists());

        let config =
            SocketConfig::custom(socket_path.clone(), "test".to_string(), "test".to_string());

        // Prepare should remove the old socket
        config.prepare().expect("prepare() should succeed for test");
        assert!(!socket_path.exists());

        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_prepare_creates_parent_directory() {
        let test_dir = std::env::temp_dir().join("beardog-socket-test-nested");
        let socket_path = test_dir.join("subdir").join("test.sock");

        // Ensure the directory doesn't exist
        let _ = fs::remove_dir_all(&test_dir);

        let config =
            SocketConfig::custom(socket_path.clone(), "test".to_string(), "test".to_string());

        // Prepare should create parent directory
        config
            .prepare()
            .expect("prepare() should succeed for nested test");
        assert!(
            socket_path
                .parent()
                .expect("socket path should have parent")
                .exists()
        );

        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
}
