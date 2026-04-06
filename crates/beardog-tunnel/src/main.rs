// SPDX-License-Identifier: AGPL-3.0-or-later

//! `BearDog` `UniBin` - Modern Idiomatic Rust Architecture
//!
//! **`UniBin` Architecture**: One binary, multiple modes (ecosystem standard)
//!
//! # Usage
//!
//! ```bash
//! # Show all available commands
//! beardog --help
//!
//! # Start server mode
//! beardog server
//!
//! # Run as daemon
//! beardog daemon
//!
//! # Interactive client
//! beardog client
//!
//! # Health diagnostics
//! beardog doctor --comprehensive
//! ```
//!
//! # Architecture
//!
//! Modern async/concurrent Rust patterns:
//! - ✅ Modern `clap` for CLI (derive API)
//! - ✅ Full async/await (tokio)
//! - ✅ Structured error handling
//! - ✅ Graceful shutdown (signals)
//! - ✅ Self-documenting help
//! - ✅ Professional UX

use beardog_tunnel::modes;
use clap::{Parser, Subcommand};
use tracing::Level;

/// `BearDog` - Security & Cryptography Primal
///
/// `UniBin` architecture: one binary, multiple operational modes.
/// Compliant with ecoPrimals ecosystem standard (Jan 2026).
#[derive(Parser)]
#[command(name = "beardog")]
#[command(about = "🐻 BearDog - Security & Cryptography Primal", long_about = None)]
#[command(version)]
#[command(author = "ecoPrimals Team")]
struct Cli {
    /// Set log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start `BearDog` server mode (Unix socket IPC)
    ///
    /// Primary operational mode for production deployments.
    /// Provides cryptographic services via Unix socket JSON-RPC.
    Server {
        /// Unix socket path (overrides environment)
        #[arg(long)]
        socket: Option<String>,

        /// Run as daemon (detach from terminal)
        #[arg(long, short = 'd')]
        daemon: bool,

        /// Family identifier (for socket path generation)
        #[arg(long)]
        family_id: Option<String>,

        /// Orchestrator identifier
        #[arg(long)]
        orchestrator_id: Option<String>,

        /// Enable HTTP API (deprecated, use Unix sockets)
        #[arg(long)]
        http_enabled: bool,

        /// HTTP bind address (only if HTTP enabled)
        /// Default comes from `BEARDOG_CONFIG` (no hardcoding)
        #[arg(long)]
        bind_addr: Option<String>,
    },

    /// Run as background daemon
    ///
    /// Detaches from terminal and runs server in background.
    /// Equivalent to: beardog server --daemon
    Daemon {
        /// Unix socket path
        #[arg(long)]
        socket: Option<String>,

        /// Family identifier
        #[arg(long)]
        family_id: Option<String>,

        /// Orchestrator identifier
        #[arg(long)]
        orchestrator_id: Option<String>,
    },

    /// Interactive client mode
    ///
    /// Connect to `BearDog` server and perform operations interactively.
    Client {
        /// Server endpoint (Unix socket or HTTP URL)
        /// Default prioritizes Primal IPC Protocol standard: /primal/beardog
        /// Falls back to XDG runtime directory or /tmp based on `SocketConfig`
        #[arg(long)]
        endpoint: Option<String>,

        /// Command to execute (if not provided, enter interactive mode)
        #[arg(long)]
        command: Option<String>,
    },

    /// Health diagnostics and system check
    ///
    /// Verifies `BearDog` installation, dependencies, and runtime health.
    Doctor {
        /// Run comprehensive diagnostics
        #[arg(long, short = 'c')]
        comprehensive: bool,

        /// Check connectivity to server
        #[arg(long)]
        socket: Option<String>,

        /// Output format (text, json)
        #[arg(long, default_value = "text")]
        format: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging based on CLI arg
    let log_level: Level = cli.log_level.parse().unwrap_or(Level::INFO);

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .init();

    // Dispatch to appropriate mode handler
    match cli.command {
        Commands::Server {
            socket,
            daemon,
            family_id,
            orchestrator_id,
            http_enabled,
            bind_addr,
        } => {
            modes::server::run(
                socket,
                daemon,
                family_id,
                orchestrator_id,
                http_enabled,
                bind_addr,
            )
            .await?;
        }

        Commands::Daemon {
            socket,
            family_id,
            orchestrator_id,
        } => {
            // Daemon is just server with daemon flag
            modes::server::run(socket, true, family_id, orchestrator_id, false, None).await?;
        }

        Commands::Client { endpoint, command } => {
            modes::client::run(endpoint, command).await?;
        }

        Commands::Doctor {
            comprehensive,
            socket,
            format,
        } => {
            modes::doctor::run(comprehensive, socket, format).await?;
        }
    }

    Ok(())
}
