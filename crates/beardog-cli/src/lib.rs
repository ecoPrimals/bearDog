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
    /// Unix socket path (default mode)
    #[arg(long, default_value = "/tmp/beardog.sock")]
    pub socket: String,

    /// TCP listen address (alternative to Unix socket for Android/Windows)
    /// Example: --listen 127.0.0.1:9900
    #[arg(long)]
    pub listen: Option<String>,

    /// Family ID for BirdSong
    #[arg(long)]
    pub family_id: Option<String>,

    /// Orchestrator ID
    #[arg(long)]
    pub orchestrator_id: Option<String>,
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
