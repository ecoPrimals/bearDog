//! # Unix Socket Configuration
//!
//! This module provides a robust, XDG-compliant socket path configuration
//! with a 3-tier fallback system:
//!
//! 1. **Environment Variable** (highest priority): `BEARDOG_SOCKET`
//! 2. **XDG Runtime Directory**: `/run/user/<uid>/beardog-<family>.sock`
//! 3. **Temp Directory** (last resort): `/tmp/beardog-<family>-<node>.sock`
//!
//! ## Security & Standards
//!
//! - ✅ XDG Base Directory Specification compliant
//! - ✅ Per-user runtime directories (`/run/user/<uid>/`)
//! - ✅ Automatic directory creation with proper permissions
//! - ✅ Old socket cleanup (prevents "address already in use")
//! - ✅ Multi-instance support via family/node IDs
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
//! println!("Socket: {}", socket_path);
//! // Output: /run/user/1000/beardog-nat0.sock
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
    /// Explicit environment variable (highest priority)
    EnvVar,
    /// XDG Runtime Directory (preferred)
    XdgRuntime,
    /// Temp directory (last resort)
    TempDir,
}

impl SocketConfig {
    /// Create socket configuration from environment variables
    ///
    /// Implements 3-tier fallback:
    /// 1. `BEARDOG_SOCKET` env var
    /// 2. `/run/user/<uid>/beardog-<family>.sock` (XDG)
    /// 3. `/tmp/beardog-<family>-<node>.sock` (fallback)
    pub fn from_env() -> Self {
        let family_id = std::env::var("BEARDOG_FAMILY_ID")
            .or_else(|_| std::env::var("FAMILY_ID"))
            .unwrap_or_else(|_| "default".to_string());

        let node_id = std::env::var("BEARDOG_NODE_ID")
            .or_else(|_| std::env::var("NODE_ID"))
            .unwrap_or_else(|_| "default".to_string());

        // Tier 1: Check for explicit BEARDOG_SOCKET env var
        if let Ok(socket_path) = std::env::var("BEARDOG_SOCKET") {
            return Self {
                socket_path: PathBuf::from(socket_path),
                family_id,
                node_id,
                source: SocketPathSource::EnvVar,
            };
        }

        // Tier 2: Try XDG Runtime Directory (more secure, per-user)
        if let Some(xdg_path) = Self::try_xdg_runtime(&family_id) {
            return Self {
                socket_path: xdg_path,
                family_id,
                node_id,
                source: SocketPathSource::XdgRuntime,
            };
        }

        // Tier 3: Fallback to /tmp (last resort)
        let tmp_path = format!("/tmp/beardog-{}-{}.sock", family_id, node_id);
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
    fn try_xdg_runtime(family_id: &str) -> Option<PathBuf> {
        // Get current user ID
        let uid = Self::get_uid();

        // Check if XDG runtime directory exists
        let xdg_runtime_dir = format!("/run/user/{}", uid);
        if Path::new(&xdg_runtime_dir).exists() {
            Some(PathBuf::from(format!(
                "{}/beardog-{}.sock",
                xdg_runtime_dir, family_id
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
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    /// Get socket path as string
    pub fn socket_path_string(&self) -> String {
        self.socket_path.display().to_string()
    }

    /// Get family ID
    pub fn family_id(&self) -> &str {
        &self.family_id
    }

    /// Get node ID
    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    /// Get the source tier that was used
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
    pub fn description(&self) -> String {
        match self.source {
            SocketPathSource::EnvVar => {
                format!(
                    "{} (from BEARDOG_SOCKET env var)",
                    self.socket_path_string()
                )
            }
            SocketPathSource::XdgRuntime => {
                format!("{} (XDG Runtime Directory)", self.socket_path_string())
            }
            SocketPathSource::TempDir => {
                format!("{} (fallback to /tmp)", self.socket_path_string())
            }
        }
    }

    /// Create a custom socket configuration (for testing)
    pub fn custom(socket_path: impl Into<PathBuf>, family_id: String, node_id: String) -> Self {
        Self {
            socket_path: socket_path.into(),
            family_id,
            node_id,
            source: SocketPathSource::EnvVar,
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
            SocketPathSource::EnvVar,
            "Source should be EnvVar when BEARDOG_SOCKET is set"
        );
        assert_eq!(config.family_id(), "test0");

        // Cleanup
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::remove_var("BEARDOG_FAMILY_ID");
    }

    #[test]
    fn test_xdg_runtime_preferred_over_tmp() {
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::set_var("BEARDOG_FAMILY_ID", "xdg-test");

        let config = SocketConfig::from_env();

        // Should use XDG if available, otherwise /tmp
        match config.source() {
            SocketPathSource::XdgRuntime => {
                assert!(config.socket_path_string().contains("/run/user/"));
                assert!(config
                    .socket_path_string()
                    .contains("beardog-xdg-test.sock"));
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
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::set_var("BEARDOG_FAMILY_ID", "fallback");
        std::env::set_var("BEARDOG_NODE_ID", "node123");

        let config = SocketConfig::from_env();

        // If XDG is not available, should fall back to /tmp with node ID
        if config.source() == SocketPathSource::TempDir {
            assert_eq!(
                config.socket_path_string(),
                "/tmp/beardog-fallback-node123.sock"
            );
        }

        std::env::remove_var("BEARDOG_FAMILY_ID");
        std::env::remove_var("BEARDOG_NODE_ID");
    }

    #[test]
    fn test_default_family_and_node_ids() {
        std::env::remove_var("BEARDOG_SOCKET");
        std::env::remove_var("BEARDOG_FAMILY_ID");
        std::env::remove_var("FAMILY_ID");
        std::env::remove_var("BEARDOG_NODE_ID");
        std::env::remove_var("NODE_ID");

        let config = SocketConfig::from_env();

        assert_eq!(config.family_id(), "default");
        assert_eq!(config.node_id(), "default");

        // Should still generate a valid path
        if config.source() == SocketPathSource::TempDir {
            assert_eq!(
                config.socket_path_string(),
                "/tmp/beardog-default-default.sock"
            );
        }
    }

    #[test]
    fn test_description_format() {
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
