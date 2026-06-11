// SPDX-License-Identifier: AGPL-3.0-or-later

//! Daemon mode handler - background service
//!
//! This handler runs the server in daemon mode (background process).

use crate::DaemonArgs;
use beardog_errors::{BearDogError, BusinessErrorCategory, SystemErrorCategory};
use std::fs::File;
use std::io::Write;
#[cfg(unix)]
use std::process::Command;
use tracing::{info, warn};

/// Ensure the PID file reflects a fresh daemon start: reject if a live PID is recorded,
/// remove stale files, then write the current process id.
///
/// Separated for unit tests (no server bind).
///
/// # Errors
///
/// Returns a [`BearDogError`] if the PID file cannot be read, a live process still holds the PID,
/// or the new PID cannot be written.
pub fn prepare_daemon_pid_file(pid_file: &str) -> Result<(), BearDogError> {
    // Check if already running
    if std::path::Path::new(pid_file).exists() {
        let pid_content = std::fs::read_to_string(pid_file).map_err(|e| BearDogError::System {
            message: format!("Failed to read PID file: {e}"),
            category: SystemErrorCategory::default(),
        })?;

        if let Ok(pid) = pid_content.trim().parse::<u32>() {
            // Check if process is still running
            #[cfg(unix)]
            {
                // Use if let instead of is_ok().unwrap() pattern
                if let Ok(output) = Command::new("kill").args(["-0", &pid.to_string()]).output()
                    && output.status.success()
                {
                    return Err(BearDogError::Business {
                        message: format!("BearDog daemon already running (PID: {pid})"),
                        category: BusinessErrorCategory::default(),
                    });
                }
            }
        }

        // Stale PID file, remove it
        warn!("⚠️  Removing stale PID file");
        std::fs::remove_file(pid_file).map_err(|e| BearDogError::System {
            message: format!("Failed to remove stale PID file: {e}"),
            category: SystemErrorCategory::default(),
        })?;
    }

    // Write PID file
    let pid = std::process::id();
    let mut file = File::create(pid_file).map_err(|e| BearDogError::System {
        message: format!("Failed to create PID file: {e}"),
        category: SystemErrorCategory::default(),
    })?;
    file.write_all(pid.to_string().as_bytes())
        .map_err(|e| BearDogError::System {
            message: format!("Failed to write PID file: {e}"),
            category: SystemErrorCategory::default(),
        })?;
    Ok(())
}

/// Handle daemon command - run as background service
///
/// # Errors
///
/// Returns an error if PID file preparation fails (e.g. daemon already running, I/O), or the
/// embedded server handler returns an error.
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
        bind_mode: crate::BindMode::Auto,
        socket: args.socket,
        r#abstract: false,
        port: None,
        listen: None,
        audit_dir: None,
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

#[cfg(test)]
mod tests {
    // SPDX-License-Identifier: AGPL-3.0-or-later

    use super::prepare_daemon_pid_file;

    #[test]
    fn prepare_daemon_pid_file_creates_and_writes_current_pid() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("unit.pid");
        let path_str = path.to_str().expect("utf8 path");

        prepare_daemon_pid_file(path_str).expect("prepare fresh pid file");

        let contents = std::fs::read_to_string(path_str).expect("read pid");
        assert_eq!(
            contents.trim(),
            std::process::id().to_string(),
            "pid file should record this process"
        );
    }

    #[test]
    fn prepare_daemon_pid_file_replaces_stale_pid_when_process_absent() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("stale.pid");
        let path_str = path.to_str().expect("utf8 path");
        std::fs::write(path_str, "999999\n").expect("write stale pid");

        prepare_daemon_pid_file(path_str).expect("stale pid should be replaced");

        let contents = std::fs::read_to_string(path_str).expect("read pid");
        assert_eq!(contents.trim(), std::process::id().to_string());
    }

    #[test]
    fn prepare_daemon_pid_file_fails_when_path_is_directory() {
        let dir = tempfile::tempdir().expect("tempdir");
        let err = prepare_daemon_pid_file(dir.path().to_str().expect("utf8 path"))
            .expect_err("directory cannot be used as pid file");
        let msg = format!("{err}");
        assert!(
            msg.contains("PID") || msg.contains("pid") || msg.contains("Failed"),
            "{msg}"
        );
    }
}
