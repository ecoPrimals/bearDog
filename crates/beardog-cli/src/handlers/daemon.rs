// SPDX-License-Identifier: AGPL-3.0-only

//! Daemon mode handler - background service
//!
//! This handler runs the server in daemon mode (background process).

use crate::DaemonArgs;
use beardog_errors::BearDogError;
use std::fs::File;
use std::io::Write;
use tracing::{info, warn};

/// Ensure the PID file reflects a fresh daemon start: reject if a live PID is recorded,
/// remove stale files, then write the current process id.
///
/// Separated for unit tests (no server bind).
pub(crate) fn prepare_daemon_pid_file(pid_file: &str) -> Result<(), BearDogError> {
    // Check if already running
    if std::path::Path::new(pid_file).exists() {
        let pid_content = std::fs::read_to_string(pid_file).map_err(|e| BearDogError::System {
            message: format!("Failed to read PID file: {e}"),
            category: Default::default(),
        })?;

        if let Ok(pid) = pid_content.trim().parse::<u32>() {
            // Check if process is still running
            #[cfg(unix)]
            {
                use std::process::Command;
                // Use if let instead of is_ok().unwrap() pattern
                if let Ok(output) = Command::new("kill").args(["-0", &pid.to_string()]).output()
                    && output.status.success()
                {
                    return Err(BearDogError::Business {
                        message: format!("BearDog daemon already running (PID: {pid})"),
                        category: Default::default(),
                    });
                }
            }
        }

        // Stale PID file, remove it
        warn!("⚠️  Removing stale PID file");
        std::fs::remove_file(pid_file).map_err(|e| BearDogError::System {
            message: format!("Failed to remove stale PID file: {e}"),
            category: Default::default(),
        })?;
    }

    // Write PID file
    let pid = std::process::id();
    let mut file = File::create(pid_file).map_err(|e| BearDogError::System {
        message: format!("Failed to create PID file: {e}"),
        category: Default::default(),
    })?;
    file.write_all(pid.to_string().as_bytes())
        .map_err(|e| BearDogError::System {
            message: format!("Failed to write PID file: {e}"),
            category: Default::default(),
        })?;
    Ok(())
}

/// Handle daemon command - run as background service
pub async fn handle_daemon(args: DaemonArgs) -> Result<(), BearDogError> {
    info!("🐻🐕 BearDog Daemon Mode - Starting...");
    info!("   Socket: {}", args.socket);
    info!("   PID file: {}", args.pid_file);
    info!("   Log file: {}", args.log_file);

    prepare_daemon_pid_file(&args.pid_file)?;
    info!(
        "✅ PID file created: {} (PID: {})",
        args.pid_file,
        std::process::id()
    );

    // Redirect logs to file
    // Note: Full daemonization (fork, setsid, etc.) would require additional crates
    // For now, this is a simplified daemon mode that can be run with nohup or systemd
    info!("📝 Logs will be written to: {}", args.log_file);
    info!("💡 Tip: Use systemd or nohup for full background operation");
    info!("");

    // Run server in foreground (systemd/nohup will background it)
    let server_args = crate::ServerArgs {
        socket: args.socket,
        r#abstract: false, // Daemon mode uses filesystem sockets
        listen: None,      // Daemon mode uses Unix sockets
        family_id: args.family_id,
        orchestrator_id: args.orchestrator_id,
    };

    // Call server handler
    let result = super::server::handle_server(server_args).await;

    // Clean up PID file on exit
    if let Err(e) = std::fs::remove_file(&args.pid_file) {
        warn!("⚠️  Failed to remove PID file: {}", e);
    }

    result
}
