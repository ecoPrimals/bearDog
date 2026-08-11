// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used, reason = "expect/unwrap acceptable for invariant failures in tests and bootstrap code"))]
#![cfg_attr(test, allow(clippy::float_cmp, reason = "float equality acceptable for metrics thresholds and test assertions"))]

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

/// guideStone P1/P4: Standard bind mode for primal startup contract.
///
/// Replaces per-primal transport flags (`--abstract`, `--no-unix`, `--no-uds`)
/// with a single ecosystem-standard enum. Set via `--bind-mode` or
/// `PRIMAL_BIND_MODE` env var.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum BindMode {
    /// Auto-detect transport from platform capabilities (default).
    /// Linux/macOS → filesystem UDS + optional TCP.
    /// Android → abstract socket + optional TCP.
    Auto,
    /// Filesystem Unix domain socket (explicit).
    Filesystem,
    /// Linux abstract namespace socket (kernel-only, no disk path).
    Abstract,
    /// TCP only — skip Unix socket entirely. Requires `--port` or `PORT`.
    Tcp,
}

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
    /// Transport bind mode (guideStone P1/P4 standard startup contract).
    ///
    /// Replaces per-primal transport flags with a single ecosystem-standard
    /// enum. `auto` uses platform detection; `abstract` forces Linux abstract
    /// namespace; `tcp` disables UDS entirely.
    ///
    /// Set via `--bind-mode` or `PRIMAL_BIND_MODE` env var.
    #[arg(long, value_enum, env = "PRIMAL_BIND_MODE", default_value_t = BindMode::Auto)]
    pub bind_mode: BindMode,

    /// Socket path (platform-native default)
    ///
    /// Defaults:
    /// - Android: @`biomeos_beardog` (abstract socket, bypasses `SELinux`)
    /// - Linux/macOS: `{temp}/beardog.sock` (platform temp dir + primal name)
    /// - Windows: `\\.\pipe\biomeos_beardog` (named pipe)
    ///
    /// Override with --socket for custom path. Ignored when `--bind-mode tcp`.
    #[arg(long, default_value_t = default_socket_path())]
    pub socket: String,

    /// Use abstract socket (Linux/Android SELinux-safe)
    ///
    /// **Deprecated**: Prefer `--bind-mode abstract`. Kept for backward compatibility.
    /// Forces abstract socket mode regardless of platform detection.
    #[arg(long)]
    pub r#abstract: bool,

    /// TCP port for JSON-RPC listener.
    ///
    /// Binds a newline-delimited JSON-RPC server on `0.0.0.0:<PORT>`.
    /// Also reads unprefixed `PORT` env var (guideStone P4 standard).
    /// Override bind address with --listen.
    #[arg(long, env = "PORT")]
    pub port: Option<u16>,

    /// TCP listen address (overrides --port with full addr:port)
    ///
    /// Example: --listen 127.0.0.1:9100
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

    /// Override path for the plaintext health socket.
    ///
    /// On Unix, a secondary UDS listener is **always** bound alongside the main
    /// socket (unless `--bind-mode tcp`). It runs in plaintext mode — no BTSP,
    /// no riboCipher — and only exposes a minimal alive/version response.
    ///
    /// cellMembrane and monitoring tools connect here for lightweight probes
    /// without needing riboCipher or BTSP handshake knowledge.
    ///
    /// Default: `<primal>-health.sock` in the same directory as the main socket.
    /// Env: `BEARDOG_HEALTH_SOCKET`
    #[arg(long, env = "BEARDOG_HEALTH_SOCKET")]
    pub health_socket: Option<String>,
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
mod coverage_expansion_cli_wave;
