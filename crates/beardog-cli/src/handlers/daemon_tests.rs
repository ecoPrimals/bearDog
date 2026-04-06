// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unit tests for daemon handler

#[cfg(test)]
mod tests {

    use crate::DaemonArgs;
    use crate::handlers::daemon::prepare_daemon_pid_file;
    use beardog_errors::BearDogError;
    use tempfile::TempDir;

    #[test]
    fn test_prepare_daemon_pid_file_creates_expected_pid() {
        let dir = TempDir::new().unwrap();
        let pid_path = dir.path().join("beardog.pid");
        let p = pid_path.to_str().unwrap();
        prepare_daemon_pid_file(p).unwrap();
        let on_disk = std::fs::read_to_string(p).unwrap();
        assert_eq!(on_disk.trim(), std::process::id().to_string());
    }

    #[test]
    fn test_prepare_daemon_pid_file_removes_unparseable_stale() {
        let dir = TempDir::new().unwrap();
        let pid_path = dir.path().join("beardog.pid");
        let p = pid_path.to_str().unwrap();
        std::fs::write(p, "not-a-number\n").unwrap();
        prepare_daemon_pid_file(p).unwrap();
        assert_eq!(
            std::fs::read_to_string(p).unwrap().trim(),
            std::process::id().to_string()
        );
    }

    #[test]
    fn test_prepare_daemon_pid_file_removes_dead_pid() {
        let dir = TempDir::new().unwrap();
        let pid_path = dir.path().join("beardog.pid");
        let p = pid_path.to_str().unwrap();
        // Very unlikely to exist as a process on Unix/Linux
        std::fs::write(p, "4194303\n").unwrap();
        prepare_daemon_pid_file(p).unwrap();
        assert_eq!(
            std::fs::read_to_string(p).unwrap().trim(),
            std::process::id().to_string()
        );
    }

    /// When the PID file lists our own live PID, `kill -0` succeeds and we must refuse to start.
    #[cfg(unix)]
    #[test]
    fn test_prepare_daemon_pid_file_rejects_live_pid() {
        let dir = TempDir::new().unwrap();
        let pid_path = dir.path().join("beardog.pid");
        let p = pid_path.to_str().unwrap();
        std::fs::write(p, format!("{}\n", std::process::id())).unwrap();
        let err = prepare_daemon_pid_file(p).unwrap_err();
        match err {
            BearDogError::Business { message, .. } => {
                assert!(
                    message.contains("already running"),
                    "unexpected message: {message}"
                );
            }
            other => panic!("expected Business error, got {other:?}"),
        }
    }

    /// PID path exists but is not a readable file (directory) → read fails with System error.
    #[test]
    fn test_prepare_daemon_pid_file_fails_when_pid_path_is_directory() {
        let dir = TempDir::new().unwrap();
        let pid_path = dir.path().join("beardog.pid");
        std::fs::create_dir(&pid_path).unwrap();
        let p = pid_path.to_str().unwrap();
        let err = prepare_daemon_pid_file(p).unwrap_err();
        assert!(
            matches!(err, BearDogError::System { .. }),
            "expected System error, got {err:?}"
        );
    }

    #[test]
    fn test_daemon_args_creation() {
        let args = DaemonArgs {
            socket: "/tmp/test.sock".to_string(),
            pid_file: "/tmp/test.pid".to_string(),
            log_file: "/tmp/test.log".to_string(),
            family_id: Some("test_family".to_string()),
            orchestrator_id: Some("test_orch".to_string()),
        };

        assert_eq!(args.socket, "/tmp/test.sock");
        assert_eq!(args.pid_file, "/tmp/test.pid");
        assert_eq!(args.log_file, "/tmp/test.log");
        assert_eq!(args.family_id, Some("test_family".to_string()));
        assert_eq!(args.orchestrator_id, Some("test_orch".to_string()));
    }

    #[test]
    fn test_daemon_args_defaults() {
        let args = DaemonArgs {
            socket: "/tmp/beardog.sock".to_string(),
            pid_file: "/tmp/beardog.pid".to_string(),
            log_file: "/tmp/beardog.log".to_string(),
            family_id: None,
            orchestrator_id: None,
        };

        assert_eq!(args.socket, "/tmp/beardog.sock");
        assert_eq!(args.pid_file, "/tmp/beardog.pid");
        assert_eq!(args.log_file, "/tmp/beardog.log");
        assert!(args.family_id.is_none());
        assert!(args.orchestrator_id.is_none());
    }

    #[test]
    fn test_daemon_args_clone() {
        let args1 = DaemonArgs {
            socket: "/tmp/test.sock".to_string(),
            pid_file: "/tmp/test.pid".to_string(),
            log_file: "/tmp/test.log".to_string(),
            family_id: Some("family1".to_string()),
            orchestrator_id: Some("orch1".to_string()),
        };

        let args2 = args1.clone();
        assert_eq!(args1.socket, args2.socket);
        assert_eq!(args1.pid_file, args2.pid_file);
        assert_eq!(args1.log_file, args2.log_file);
        assert_eq!(args1.family_id, args2.family_id);
        assert_eq!(args1.orchestrator_id, args2.orchestrator_id);
    }
}
