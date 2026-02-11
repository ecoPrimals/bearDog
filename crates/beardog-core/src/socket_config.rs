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

/// Socket configuration with 3-tier fallback logic
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
    /// Create socket configuration from environment variables
    ///
    /// Implements 5-tier fallback (Primal IPC Protocol compliant):
    /// 1. `BEARDOG_SOCKET` env var (primal-specific, highest priority)
    /// 2. `BIOMEOS_SOCKET_PATH` or `BIOMEOS_SOCKET_DIR` env var (generic orchestrator, e.g., Neural API)
    /// 3. `/primal/beardog` (Primal IPC Protocol standard namespace)
    /// 4. `/run/user/<uid>/biomeos/beardog.sock` (XDG Runtime Directory, biomeOS standard)
    /// 5. `/tmp/beardog-<family>-<node>.sock` (fallback)
    #[must_use]
    pub fn from_env() -> Self {
        let family_id = std::env::var("BEARDOG_FAMILY_ID")
            .or_else(|_| std::env::var("FAMILY_ID"))
            .unwrap_or_else(|_| "default".to_string());

        let node_id = std::env::var("BEARDOG_NODE_ID")
            .or_else(|_| std::env::var("NODE_ID"))
            .unwrap_or_else(|_| "default".to_string());

        // Tier 1: Check for primal-specific BEARDOG_SOCKET env var (highest priority)
        if let Ok(socket_path) = std::env::var("BEARDOG_SOCKET") {
            // Validate: reject empty paths (fail fast, no hanging)
            if socket_path.is_empty() {
                // Skip to next tier instead of using empty path
                // This prevents production hangs discovered in testing
            } else {
                return Self {
                    socket_path: PathBuf::from(socket_path),
                    family_id,
                    node_id,
                    source: SocketPathSource::PrimalEnvVar,
                };
            }
        }

        // Tier 2: Check for generic orchestrator BIOMEOS_SOCKET_PATH or BIOMEOS_SOCKET_DIR
        // This allows Neural API to set a standard path for all primals
        // BIOMEOS_SOCKET_PATH: Full path to socket file
        // BIOMEOS_SOCKET_DIR: Directory where beardog.sock will be created
        if let Ok(socket_path) = std::env::var("BIOMEOS_SOCKET_PATH") {
            // Validate: reject empty paths (fail fast, no hanging)
            if !socket_path.is_empty() {
                return Self {
                    socket_path: PathBuf::from(socket_path),
                    family_id,
                    node_id,
                    source: SocketPathSource::OrchestratorEnvVar,
                };
            }
        } else if let Ok(socket_dir) = std::env::var("BIOMEOS_SOCKET_DIR") {
            // Validate: reject empty paths
            if !socket_dir.is_empty() {
                return Self {
                    socket_path: PathBuf::from(socket_dir).join("beardog.sock"),
                    family_id,
                    node_id,
                    source: SocketPathSource::OrchestratorEnvVar,
                };
            }
        }

        // Tier 3: Try Primal IPC Protocol standard namespace (/primal/{primal-name})
        // Per PRIMAL_IPC_PROTOCOL.md: Standard Path Format: /primal/{primal-name}
        // Uses PRIMAL_NAME env var for self-knowledge, defaults to "beardog"
        if Path::new("/primal").exists() {
            let primal_name =
                std::env::var("PRIMAL_NAME").unwrap_or_else(|_| "beardog".to_string());
            return Self {
                socket_path: PathBuf::from(format!("/primal/{}", primal_name)),
                family_id,
                node_id,
                source: SocketPathSource::PrimalNamespace,
            };
        }

        // Tier 4: Try XDG Runtime Directory (more secure, per-user)
        if let Some(xdg_path) = Self::try_xdg_runtime(&family_id) {
            return Self {
                socket_path: xdg_path,
                family_id,
                node_id,
                source: SocketPathSource::XdgRuntime,
            };
        }

        // Tier 5: Fallback to /tmp (last resort)
        // Uses PRIMAL_NAME env var for self-knowledge, defaults to "beardog"
        let primal_name = std::env::var("PRIMAL_NAME").unwrap_or_else(|_| "beardog".to_string());
        let tmp_path = format!("/tmp/{}-{family_id}-{node_id}.sock", primal_name);
        Self {
            socket_path: PathBuf::from(tmp_path),
            family_id,
            node_id,
            source: SocketPathSource::TempDir,
        }
    }

    /// Try to use XDG Runtime Directory
    ///
    /// Returns `Some(path)` if `/run/user/<uid>/` exists, otherwise `None`
    ///
    /// Creates socket at `/run/user/<uid>/biomeos/beardog.sock` for biomeOS integration.
    /// The `/biomeos/` subdirectory groups all biomeOS primal sockets together for
    /// easy discovery and management.
    fn try_xdg_runtime(_family_id: &str) -> Option<PathBuf> {
        // Get current user ID
        let uid = Self::get_uid();

        // Check if XDG runtime directory exists
        let xdg_runtime_dir = format!("/run/user/{uid}");
        if Path::new(&xdg_runtime_dir).exists() {
            // Use biomeOS subdirectory for ecosystem integration
            // Path: /run/user/<uid>/biomeos/beardog.sock
            Some(PathBuf::from(format!(
                "{xdg_runtime_dir}/biomeos/beardog.sock"
            )))
        } else {
            None
        }
    }

    /// Get current user ID
    ///
    /// Uses fallback parsing from environment or defaults to 1000
    #[cfg(unix)]
    fn get_uid() -> u32 {
        std::env::var("UID")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1000) // Default to 1000 (common for first user)
    }

    #[cfg(not(unix))]
    fn get_uid() -> u32 {
        1000 // Default UID for non-Unix systems
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
    pub fn source(&self) -> SocketPathSource {
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
        if let Some(parent) = self.socket_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| {
                    format!(
                        "Failed to create socket directory {}: {}",
                        parent.display(),
                        e
                    )
                })?;
            }
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
    use std::sync::Mutex;

    // Global mutex to serialize env var tests
    // Modern Rust: Tests that mutate global state (env vars) must be serialized
    // This is a deep debt solution: explicit serialization for correctness
    static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_env_var_override_takes_priority() {
        // Lock to prevent concurrent env var modification
        let _lock = ENV_TEST_LOCK.lock().unwrap();

        // Clean slate - remove any existing variables
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::remove_var("BEARDOG_FAMILY_ID");
        std::env::remove_var("FAMILY_ID");
        std::env::remove_var("BEARDOG_NODE_ID");
        std::env::remove_var("NODE_ID");

        // Set explicit override
        std::env::set_var("BEARDOG_SOCKET", "/tmp/custom-override.sock");
        std::env::set_var("BEARDOG_FAMILY_ID", "test0");

        let config = SocketConfig::from_env();

        // Deep debt solution: Assert what we set, not assumptions about XDG
        // The env var should take priority regardless of system configuration
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

        // Cleanup
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::remove_var("BEARDOG_FAMILY_ID");
    }

    #[test]
    fn test_empty_socket_path_rejected() {
        // Lock to prevent concurrent env var modification
        let _lock = ENV_TEST_LOCK.lock().unwrap();

        // Test: Empty socket paths should fall through to next tier
        // This prevents production hangs discovered in integration testing

        // Clean slate
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::remove_var("BIOMEOS_SOCKET_PATH");
        std::env::remove_var("BEARDOG_FAMILY_ID");
        std::env::remove_var("BEARDOG_NODE_ID");

        // Set empty socket path (should be rejected)
        std::env::set_var("BEARDOG_SOCKET", "");
        std::env::set_var("BEARDOG_FAMILY_ID", "test");

        let config = SocketConfig::from_env();

        // Should NOT use empty path - should fall through to tier 3, 4, or 5
        assert_ne!(config.socket_path_string(), "");
        assert_ne!(config.source(), SocketPathSource::PrimalEnvVar);

        // Should use Primal IPC namespace, XDG, or /tmp fallback
        assert!(
            config.source() == SocketPathSource::PrimalNamespace
                || config.source() == SocketPathSource::XdgRuntime
                || config.source() == SocketPathSource::TempDir
        );

        // Cleanup
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::remove_var("BEARDOG_FAMILY_ID");
    }

    #[test]
    fn test_empty_biomeos_socket_rejected() {
        // Lock to prevent concurrent env var modification
        let _lock = ENV_TEST_LOCK.lock().unwrap();

        // Test: Empty BIOMEOS_SOCKET_PATH should also be rejected

        // Clean slate
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::remove_var("BIOMEOS_SOCKET_PATH");
        std::env::remove_var("BEARDOG_FAMILY_ID");

        // Set empty orchestrator socket (should be rejected)
        std::env::set_var("BIOMEOS_SOCKET_PATH", "");
        std::env::set_var("BEARDOG_FAMILY_ID", "test");

        let config = SocketConfig::from_env();

        // Should NOT use empty path - should fall through to tier 3, 4, or 5
        assert_ne!(config.socket_path_string(), "");
        assert_ne!(config.source(), SocketPathSource::OrchestratorEnvVar);

        // Should use Primal IPC namespace, XDG, or /tmp fallback
        assert!(
            config.source() == SocketPathSource::PrimalNamespace
                || config.source() == SocketPathSource::XdgRuntime
                || config.source() == SocketPathSource::TempDir
        );

        // Cleanup
        std::env::remove_var("BIOMEOS_SOCKET_PATH");
        std::env::remove_var("BEARDOG_FAMILY_ID");
    }

    #[test]
    fn test_biomeos_socket_path_tier2() {
        // Lock to prevent concurrent env var modification
        let _lock = ENV_TEST_LOCK.lock().unwrap();

        // Clean slate - IMPORTANT: Remove ALL relevant env vars for concurrent test safety
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::remove_var("BIOMEOS_SOCKET_PATH");
        std::env::remove_var("BEARDOG_FAMILY_ID");
        std::env::remove_var("FAMILY_ID");
        std::env::remove_var("BEARDOG_NODE_ID");
        std::env::remove_var("NODE_ID");

        // Set BIOMEOS_SOCKET_PATH (Tier 2 - Neural API orchestrator)
        std::env::set_var("BIOMEOS_SOCKET_PATH", "/tmp/beardog-default-default.sock");
        std::env::set_var("BEARDOG_FAMILY_ID", "nat0");

        let config = SocketConfig::from_env();

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

        // Cleanup - IMPORTANT: Clean ALL vars we touched
        std::env::remove_var("BIOMEOS_SOCKET_PATH");
        std::env::remove_var("BEARDOG_FAMILY_ID");
    }

    #[test]
    fn test_beardog_socket_overrides_biomeos_socket_path() {
        // Lock to prevent concurrent env var modification
        let _lock = ENV_TEST_LOCK.lock().unwrap();
        // Clean slate
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::remove_var("BIOMEOS_SOCKET_PATH");

        // Set both - BEARDOG_SOCKET should win (Tier 1 > Tier 2)
        std::env::set_var("BEARDOG_SOCKET", "/custom/beardog-specific.sock");
        std::env::set_var("BIOMEOS_SOCKET_PATH", "/tmp/biomeos-generic.sock");

        let config = SocketConfig::from_env();

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

        // Cleanup
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::remove_var("BIOMEOS_SOCKET_PATH");
    }

    #[test]
    fn test_xdg_runtime_preferred_over_tmp() {
        let _lock = ENV_TEST_LOCK.lock().unwrap();
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::remove_var("BIOMEOS_SOCKET_PATH");
        std::env::set_var("BEARDOG_FAMILY_ID", "xdg-test");

        let config = SocketConfig::from_env();

        // Should use Primal IPC namespace, XDG, or /tmp depending on system
        match config.source() {
            SocketPathSource::PrimalNamespace => {
                assert_eq!(config.socket_path_string(), "/primal/beardog");
            }
            SocketPathSource::XdgRuntime => {
                assert!(config.socket_path_string().contains("/run/user/"));
                // biomeOS standard: /run/user/$UID/biomeos/beardog.sock
                assert!(config.socket_path_string().contains("biomeos/beardog.sock"));
            }
            SocketPathSource::TempDir => {
                assert!(config.socket_path_string().starts_with("/tmp/beardog-"));
            }
            _ => panic!("Unexpected source: {:?}", config.source()),
        }

        std::env::remove_var("BEARDOG_FAMILY_ID");
    }

    #[test]
    fn test_fallback_to_tmp_with_node_id() {
        let _lock = ENV_TEST_LOCK.lock().unwrap();
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::set_var("BEARDOG_FAMILY_ID", "fallback");
        std::env::set_var("BEARDOG_NODE_ID", "node123");

        let config = SocketConfig::from_env();

        // If Primal namespace, XDG, or /tmp fallback is used
        if config.source() == SocketPathSource::TempDir {
            assert_eq!(
                config.socket_path_string(),
                "/tmp/beardog-fallback-node123.sock"
            );
        } else if config.source() == SocketPathSource::PrimalNamespace {
            assert_eq!(config.socket_path_string(), "/primal/beardog");
        }
        // XDG would have different path, which is fine

        std::env::remove_var("BEARDOG_FAMILY_ID");
        std::env::remove_var("BEARDOG_NODE_ID");
    }

    #[test]
    fn test_default_family_and_node_ids() {
        let _lock = ENV_TEST_LOCK.lock().unwrap();
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::remove_var("BEARDOG_FAMILY_ID");
        std::env::remove_var("FAMILY_ID");
        std::env::remove_var("BEARDOG_NODE_ID");
        std::env::remove_var("NODE_ID");

        let config = SocketConfig::from_env();

        assert_eq!(config.family_id(), "default");
        assert_eq!(config.node_id(), "default");

        // Should generate a valid path - could be Primal namespace, XDG, or /tmp
        if config.source() == SocketPathSource::TempDir {
            assert_eq!(
                config.socket_path_string(),
                "/tmp/beardog-default-default.sock"
            );
        } else if config.source() == SocketPathSource::PrimalNamespace {
            assert_eq!(config.socket_path_string(), "/primal/beardog");
        }
        // XDG would have different path, which is fine
    }

    #[test]
    fn test_description_format() {
        let _lock = ENV_TEST_LOCK.lock().unwrap();
        std::env::set_var("BEARDOG_SOCKET", "/custom/socket.sock");
        let config = SocketConfig::from_env();

        let desc = config.description();
        assert!(desc.contains("/custom/socket.sock"));
        assert!(desc.contains("BEARDOG_SOCKET"));

        std::env::remove_var("BEARDOG_SOCKET");
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
        fs::create_dir_all(&test_dir).unwrap();

        let socket_path = test_dir.join("test-socket.sock");

        // Create an old socket file
        fs::write(&socket_path, b"old socket").unwrap();
        assert!(socket_path.exists());

        let config =
            SocketConfig::custom(socket_path.clone(), "test".to_string(), "test".to_string());

        // Prepare should remove the old socket
        config.prepare().unwrap();
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
        config.prepare().unwrap();
        assert!(socket_path.parent().unwrap().exists());

        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }
}
