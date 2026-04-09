// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]
#![cfg_attr(test, allow(clippy::float_cmp))]

//! `BearDog` CLI Library
//!
//! This library provides the core functionality for the `BearDog` CLI,
//! including command handlers and argument definitions.
//!
//! # Socket Path Discovery (TRUE PRIMAL)
//!
//! All socket paths use `SocketConfig::from_env()` which implements
//! 5-tier fallback per Primal IPC Protocol:
//!
//! 1. `BEARDOG_SOCKET` env var (primal-specific)
//! 2. `BIOMEOS_SOCKET_PATH` env var (orchestrator)
//! 3. `/primal/beardog` (Primal IPC Protocol standard)
//! 4. XDG Runtime Directory
//! 5. Temp directory (last resort)

use beardog_core::socket_config::SocketConfig;
use clap::Parser;

pub mod ecosystem_discovery_adapter;
pub mod handlers;

#[cfg(test)]
#[doc(hidden)]
pub mod __cli_test_env {
    use std::sync::Mutex;

    /// Serialize tests that mutate `HOME` or other process environment variables.
    pub static HOME: Mutex<()> = Mutex::new(());
}

// ============================================================================
// UNIBIN OPERATIONAL MODE ARGUMENTS
// ============================================================================

/// Server mode arguments
#[derive(Parser, Debug, Clone)]
pub struct ServerArgs {
    /// Socket path (platform-native default)
    ///
    /// **Deep Debt Evolution**: Platform-agnostic runtime discovery!
    ///
    /// Defaults:
    /// - Android: @`biomeos_beardog` (abstract socket, bypasses `SELinux`)
    /// - Linux/macOS: `{temp}/beardog.sock` (platform temp dir + primal name; override dir with `BEARDOG_SOCKET_TMP_DIR` / `BEARDOG_LOCAL_SOCKET_DIR` in client handler)
    /// - Windows: `\\.\pipe\biomeos_beardog` (named pipe)
    ///
    /// Override with --socket for custom path
    #[arg(long, default_value_t = default_socket_path())]
    pub socket: String,

    /// Use abstract socket (Linux/Android SELinux-safe)
    ///
    /// Forces abstract socket mode regardless of platform detection.
    /// Abstract sockets bypass `SELinux` restrictions on Android.
    /// Format: @`biomeos_beardog`_{`family_id`}
    ///
    /// Use when deploying to Android with aarch64-linux-musl target.
    #[arg(long)]
    pub r#abstract: bool,

    /// TCP port for JSON-RPC listener (`UniBin` v1.1 mandatory)
    ///
    /// Binds a newline-delimited JSON-RPC server on `0.0.0.0:<PORT>`.
    /// Required by `PRIMAL_IPC_PROTOCOL` and `UNIBIN_ARCHITECTURE_STANDARD` v1.1.
    /// Override bind address with --listen.
    #[arg(long)]
    pub port: Option<u16>,

    /// TCP listen address (overrides --port with full addr:port)
    ///
    /// Example: --listen 127.0.0.1:9900
    /// Use for cross-device communication or when native IPC is unavailable.
    #[arg(long, conflicts_with = "port")]
    pub listen: Option<String>,

    /// Directory for audit logs (also `BEARDOG_AUDIT_DIR` env var).
    ///
    /// Defaults to `$TMPDIR/beardog` when unset.  Required on mobile
    /// / container substrates where CWD may be read-only.
    #[arg(long, env = "BEARDOG_AUDIT_DIR")]
    pub audit_dir: Option<std::path::PathBuf>,

    /// Family ID for `BirdSong`
    #[arg(long)]
    pub family_id: Option<String>,

    /// Orchestrator ID
    #[arg(long)]
    pub orchestrator_id: Option<String>,
}

/// Get platform-native default socket path via `SocketConfig`
///
/// **TRUE PRIMAL**: Uses `SocketConfig`'s 5-tier discovery instead of hardcoding
///
/// Discovery order (per Primal IPC Protocol):
/// 1. `BEARDOG_SOCKET` env var
/// 2. `BIOMEOS_SOCKET_PATH` env var (Neural API orchestration)
/// 3. `/primal/beardog` (Primal IPC Protocol standard)
/// 4. XDG Runtime Directory
/// 5. Temp directory (platform-specific fallback)
fn default_socket_path() -> String {
    match SocketConfig::from_env() {
        Ok(config) => config.socket_path().to_string_lossy().to_string(),
        Err(e) => {
            eprintln!("FATAL: {e}");
            std::process::exit(1);
        }
    }
}

/// Get platform-native PID file path
///
/// Uses `std::env::temp_dir()` for cross-platform compatibility
fn default_pid_path() -> String {
    let mut path = std::env::temp_dir();
    path.push("beardog.pid");
    path.to_string_lossy().to_string()
}

/// Get platform-native log file path
///
/// Uses `std::env::temp_dir()` for cross-platform compatibility
fn default_log_path() -> String {
    let mut path = std::env::temp_dir();
    path.push("beardog.log");
    path.to_string_lossy().to_string()
}

/// Daemon mode arguments
#[derive(Parser, Debug, Clone)]
pub struct DaemonArgs {
    /// Socket path (uses `SocketConfig` discovery)
    ///
    /// **TRUE PRIMAL**: Automatic 5-tier discovery via `SocketConfig`
    #[arg(long, default_value_t = default_socket_path())]
    pub socket: String,

    /// PID file path
    ///
    /// Uses platform temp directory by default
    #[arg(long, default_value_t = default_pid_path())]
    pub pid_file: String,

    /// Log file path
    ///
    /// Uses platform temp directory by default
    #[arg(long, default_value_t = default_log_path())]
    pub log_file: String,

    /// Family ID for `BirdSong`
    #[arg(long)]
    pub family_id: Option<String>,

    /// Orchestrator ID
    #[arg(long)]
    pub orchestrator_id: Option<String>,
}

/// Client mode arguments
#[derive(Parser, Debug, Clone)]
pub struct ClientArgs {
    /// Socket path to connect to (uses `SocketConfig` discovery)
    ///
    /// **TRUE PRIMAL**: Automatic 5-tier discovery via `SocketConfig`
    #[arg(long, default_value_t = default_socket_path())]
    pub socket: String,

    /// Command to execute (if not provided, starts interactive mode)
    #[arg(long)]
    pub command: Option<String>,
}

/// Doctor mode arguments
#[derive(Parser, Debug, Clone)]
pub struct DoctorArgs {
    /// Comprehensive health check
    #[arg(long)]
    pub comprehensive: bool,

    /// Output format (text, json)
    #[arg(long, default_value = "text")]
    pub format: String,

    /// Check specific component
    #[arg(long)]
    pub component: Option<String>,
}

#[cfg(test)]
mod coverage_expansion_march_2026;
