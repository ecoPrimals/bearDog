//! Unit tests for daemon handler

#[cfg(test)]
mod tests {

    use crate::DaemonArgs;

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
