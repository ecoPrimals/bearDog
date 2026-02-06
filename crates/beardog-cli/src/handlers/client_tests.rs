//! Unit tests for client handler

#[cfg(test)]
mod tests {

    use crate::ClientArgs;

    #[test]
    fn test_client_args_creation() {
        let args = ClientArgs {
            socket: "/tmp/test.sock".to_string(),
            command: Some("help".to_string()),
        };

        assert_eq!(args.socket, "/tmp/test.sock");
        assert_eq!(args.command, Some("help".to_string()));
    }

    #[test]
    fn test_client_args_defaults() {
        let args = ClientArgs {
            socket: "/tmp/beardog.sock".to_string(),
            command: None,
        };

        assert_eq!(args.socket, "/tmp/beardog.sock");
        assert!(args.command.is_none());
    }

    #[test]
    fn test_client_args_with_command() {
        let commands = vec!["help", "crypto.sign_ed25519", "discovery.capabilities"];

        for cmd in commands {
            let args = ClientArgs {
                socket: "/tmp/beardog.sock".to_string(),
                command: Some(cmd.to_string()),
            };
            assert_eq!(args.command, Some(cmd.to_string()));
        }
    }

    #[test]
    fn test_client_args_clone() {
        let args1 = ClientArgs {
            socket: "/tmp/test.sock".to_string(),
            command: Some("help".to_string()),
        };

        let args2 = args1.clone();
        assert_eq!(args1.socket, args2.socket);
        assert_eq!(args1.command, args2.command);
    }
}
