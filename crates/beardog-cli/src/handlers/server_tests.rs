// SPDX-License-Identifier: AGPL-3.0-only

//! Unit tests for server handler

#[cfg(test)]
mod tests {

    use crate::ServerArgs;
    use crate::handlers::server::resolve_server_socket_path;

    #[test]
    fn test_server_args_creation() {
        let args = ServerArgs {
            socket: "/tmp/test.sock".to_string(),
            r#abstract: false,
            port: None,
            listen: None,
            family_id: Some("test_family".to_string()),
            orchestrator_id: Some("test_orch".to_string()),
        };

        assert_eq!(args.socket, "/tmp/test.sock");
        assert!(!args.r#abstract);
        assert_eq!(args.listen, None);
        assert_eq!(args.family_id, Some("test_family".to_string()));
        assert_eq!(args.orchestrator_id, Some("test_orch".to_string()));
    }

    #[test]
    fn test_server_args_defaults() {
        let args = ServerArgs {
            socket: "/tmp/beardog.sock".to_string(),
            r#abstract: false,
            port: None,
            listen: None,
            family_id: None,
            orchestrator_id: None,
        };

        assert_eq!(args.socket, "/tmp/beardog.sock");
        assert_eq!(args.listen, None);
        assert!(args.family_id.is_none());
        assert!(args.orchestrator_id.is_none());
    }

    #[test]
    fn test_server_args_abstract_socket() {
        let args = ServerArgs {
            socket: "/tmp/beardog.sock".to_string(),
            r#abstract: true,
            port: None,
            listen: None,
            family_id: Some("stun_test".to_string()),
            orchestrator_id: None,
        };

        assert!(args.r#abstract);
        assert_eq!(args.family_id, Some("stun_test".to_string()));
    }

    #[test]
    fn test_resolve_server_socket_path_abstract_default_family() {
        let args = ServerArgs {
            socket: "/tmp/ignored.sock".to_string(),
            r#abstract: true,
            port: None,
            listen: None,
            family_id: None,
            orchestrator_id: None,
        };
        assert_eq!(
            resolve_server_socket_path(&args),
            "@biomeos_beardog_default"
        );
    }

    #[test]
    fn test_resolve_server_socket_path_abstract_named_family() {
        let args = ServerArgs {
            socket: "/tmp/ignored.sock".to_string(),
            r#abstract: true,
            port: None,
            listen: None,
            family_id: Some("alpha".to_string()),
            orchestrator_id: None,
        };
        assert_eq!(resolve_server_socket_path(&args), "@biomeos_beardog_alpha");
    }

    #[test]
    fn test_resolve_server_socket_path_family_scoped_file() {
        let args = ServerArgs {
            socket: "/var/run/beardog.sock".to_string(),
            r#abstract: false,
            port: None,
            listen: None,
            family_id: Some("fam99".to_string()),
            orchestrator_id: None,
        };
        assert_eq!(
            resolve_server_socket_path(&args),
            "/var/run/beardog-fam99.sock"
        );
    }

    #[test]
    fn test_resolve_server_socket_path_explicit_when_no_family() {
        let args = ServerArgs {
            socket: "/tmp/custom.sock".to_string(),
            r#abstract: false,
            port: None,
            listen: None,
            family_id: None,
            orchestrator_id: None,
        };
        assert_eq!(resolve_server_socket_path(&args), "/tmp/custom.sock");
    }

    #[test]
    fn test_server_args_clone() {
        let args1 = ServerArgs {
            socket: "/tmp/test.sock".to_string(),
            r#abstract: true,
            port: None,
            listen: None,
            family_id: Some("family1".to_string()),
            orchestrator_id: Some("orch1".to_string()),
        };

        let args2 = args1.clone();
        assert_eq!(args1.socket, args2.socket);
        assert_eq!(args1.r#abstract, args2.r#abstract);
        assert_eq!(args1.listen, args2.listen);
        assert_eq!(args1.family_id, args2.family_id);
        assert_eq!(args1.orchestrator_id, args2.orchestrator_id);
    }
}
