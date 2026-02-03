//! BearDog CLI Library
//!
//! This library provides the core functionality for the BearDog CLI,
//! including command handlers and argument definitions.

use clap::Parser;

pub mod ecosystem_discovery_adapter;
pub mod handlers;

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
    /// - Android: @biomeos_beardog (abstract socket, bypasses SELinux)
    /// - Linux/macOS: /tmp/beardog.sock (filesystem Unix socket)
    /// - Windows: \\.\pipe\biomeos_beardog (named pipe)
    ///
    /// Override with --socket for custom path
    #[arg(long, default_value_t = default_socket_path())]
    pub socket: String,

    /// TCP listen address (Tier 2 - Universal fallback)
    /// Example: --listen 127.0.0.1:9900
    ///
    /// Use for:
    /// - Cross-device communication
    /// - When native IPC unavailable
    /// - Testing/development
    #[arg(long)]
    pub listen: Option<String>,

    /// Family ID for BirdSong
    #[arg(long)]
    pub family_id: Option<String>,

    /// Orchestrator ID
    #[arg(long)]
    pub orchestrator_id: Option<String>,
}

/// Get platform-native default socket path
///
/// **Deep Debt Principle #4 & #5**: Agnostic + Runtime Discovery
///
/// Automatically selects optimal IPC mechanism per platform
fn default_socket_path() -> String {
    // Platform detection at compile-time, returns appropriate default
    #[cfg(target_os = "android")]
    {
        "@biomeos_beardog".to_string()
    }
    #[cfg(all(unix, not(target_os = "android")))]
    {
        "/tmp/beardog.sock".to_string()
    }
    #[cfg(windows)]
    {
        r"\\.\pipe\biomeos_beardog".to_string()
    }
    #[cfg(target_os = "ios")]
    {
        "com.ecoprimals.beardog".to_string()
    }
    #[cfg(target_family = "wasm")]
    {
        "beardog".to_string()
    }
}

/// Daemon mode arguments
#[derive(Parser, Debug, Clone)]
pub struct DaemonArgs {
    /// Unix socket path
    #[arg(long, default_value = "/tmp/beardog.sock")]
    pub socket: String,

    /// PID file path
    #[arg(long, default_value = "/tmp/beardog.pid")]
    pub pid_file: String,

    /// Log file path
    #[arg(long, default_value = "/tmp/beardog.log")]
    pub log_file: String,

    /// Family ID for BirdSong
    #[arg(long)]
    pub family_id: Option<String>,

    /// Orchestrator ID
    #[arg(long)]
    pub orchestrator_id: Option<String>,
}

/// Client mode arguments
#[derive(Parser, Debug, Clone)]
pub struct ClientArgs {
    /// Unix socket path to connect to
    #[arg(long, default_value = "/tmp/beardog.sock")]
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
