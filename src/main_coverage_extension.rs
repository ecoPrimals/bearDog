// SPDX-License-Identifier: AGPL-3.0-only
//! Deep coverage for `main.rs`: CLI parsing edge cases, `init_tracing`, and `dispatch` branches.

#[cfg(test)]
mod main_coverage_deep_tests {
    #![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

    use crate::{BirdsongAction, Cli, Commands, EntropyAction, HsmAction, KeyAction, init_tracing};
    use beardog_cli::handlers::cross_primal::CrossPrimalCommand;
    use beardog_cli::{ClientArgs, DaemonArgs, DoctorArgs, ServerArgs};
    use clap::Parser;

    fn parse_cli(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(args)
    }

    #[test]
    fn init_tracing_is_idempotent() {
        init_tracing();
        init_tracing();
    }

    #[test]
    fn cli_parses_encrypt_with_genetic_flag() {
        let cli = parse_cli(&[
            "beardog",
            "encrypt",
            "--key",
            "k",
            "--input",
            "i",
            "--output",
            "o",
            "--genetic",
        ])
        .expect("parse encrypt --genetic");
        match cli.command {
            Commands::Encrypt { genetic, .. } => assert!(genetic),
            _ => panic!("expected Encrypt"),
        }
    }

    #[test]
    fn cli_parses_birdsong_decrypt() {
        let cli = parse_cli(&[
            "beardog", "birdsong", "decrypt", "--input", "/tmp/in", "--key-id", "kid",
        ])
        .expect("parse birdsong decrypt");
        assert!(matches!(
            cli.command,
            Commands::Birdsong {
                action: BirdsongAction::Decrypt { .. }
            }
        ));
    }

    #[test]
    fn cli_parses_key_generate_with_seed_and_algorithm() {
        let cli = parse_cli(&[
            "beardog",
            "key",
            "generate",
            "--key-id",
            "x",
            "--algorithm",
            "chacha20-poly1305",
            "--seed",
            "/tmp/seed.bin",
        ])
        .expect("parse key generate with seed");
        match cli.command {
            Commands::Key {
                action:
                    KeyAction::Generate {
                        key_id,
                        algorithm,
                        seed,
                        ..
                    },
            } => {
                assert_eq!(key_id, "x");
                assert_eq!(algorithm, "chacha20-poly1305");
                assert_eq!(seed.as_deref(), Some("/tmp/seed.bin"));
            }
            _ => panic!("expected Key Generate"),
        }
    }

    #[test]
    fn cli_parses_key_list_with_hsm_and_verbose() {
        let cli = parse_cli(&["beardog", "key", "list", "--hsm", "software", "--verbose"])
            .expect("parse key list verbose");
        match cli.command {
            Commands::Key {
                action: KeyAction::List { hsm, verbose },
            } => {
                assert_eq!(hsm.as_deref(), Some("software"));
                assert!(verbose);
            }
            _ => panic!("expected Key List"),
        }
    }

    #[test]
    fn cli_parses_key_revoke_with_options() {
        let cli = parse_cli(&[
            "beardog",
            "key",
            "revoke",
            "--key-id",
            "k1",
            "--reason",
            "rotated",
            "--effective-at",
            "2026-01-01",
            "--cascade",
        ])
        .expect("parse key revoke");
        match cli.command {
            Commands::Key {
                action:
                    KeyAction::Revoke {
                        key_id,
                        reason,
                        effective_at,
                        cascade,
                    },
            } => {
                assert_eq!(key_id, "k1");
                assert_eq!(reason.as_deref(), Some("rotated"));
                assert_eq!(effective_at.as_deref(), Some("2026-01-01"));
                assert!(cascade);
            }
            _ => panic!("expected Key Revoke"),
        }
    }

    #[test]
    fn cli_parses_entropy_collect_with_identity_and_tier() {
        let cli = parse_cli(&[
            "beardog",
            "entropy",
            "collect",
            "--human-input",
            "--device",
            "software",
            "--quality-tier",
            "4",
            "--output",
            "/tmp/out.json",
            "--identity",
            "node-a",
        ])
        .expect("parse entropy collect");
        match cli.command {
            Commands::Entropy {
                action:
                    EntropyAction::Collect {
                        human_input,
                        device,
                        quality_tier,
                        output,
                        identity,
                    },
            } => {
                assert!(human_input);
                assert_eq!(device, "software");
                assert_eq!(quality_tier, 4);
                assert_eq!(output, "/tmp/out.json");
                assert_eq!(identity.as_deref(), Some("node-a"));
            }
            _ => panic!("expected Entropy Collect"),
        }
    }

    #[test]
    fn cli_parses_hsm_discover_verbose() {
        let cli = parse_cli(&["beardog", "hsm", "discover", "--verbose"]).expect("parse");
        match cli.command {
            Commands::Hsm {
                action: HsmAction::Discover { verbose },
            } => assert!(verbose),
            _ => panic!("expected hsm discover"),
        }
    }

    #[test]
    fn cli_parses_hsm_test_iterations() {
        let cli = parse_cli(&[
            "beardog",
            "hsm",
            "test",
            "--hsm-id",
            "h1",
            "--iterations",
            "3",
        ])
        .expect("parse");
        match cli.command {
            Commands::Hsm {
                action: HsmAction::Test { hsm_id, iterations },
            } => {
                assert_eq!(hsm_id, "h1");
                assert_eq!(iterations, 3);
            }
            _ => panic!("expected hsm test"),
        }
    }

    #[test]
    fn cli_parses_server_with_listen_and_ids() {
        let cli = parse_cli(&[
            "beardog",
            "server",
            "--socket",
            "/tmp/s.sock",
            "--listen",
            "127.0.0.1:0",
            "--family-id",
            "fam",
            "--orchestrator-id",
            "orch",
        ])
        .expect("parse server");
        match cli.command {
            Commands::Server(args) => {
                assert_eq!(args.socket, "/tmp/s.sock");
                assert_eq!(args.listen.as_deref(), Some("127.0.0.1:0"));
                assert_eq!(args.family_id.as_deref(), Some("fam"));
                assert_eq!(args.orchestrator_id.as_deref(), Some("orch"));
            }
            _ => panic!("expected Server"),
        }
    }

    #[test]
    fn cli_parses_server_abstract_socket() {
        let cli = parse_cli(&["beardog", "server", "--abstract"]).expect("parse abstract");
        match cli.command {
            Commands::Server(args) => assert!(args.r#abstract),
            _ => panic!("expected Server"),
        }
    }

    #[test]
    fn cli_parses_daemon_with_paths() {
        let cli = parse_cli(&[
            "beardog",
            "daemon",
            "--socket",
            "/tmp/d.sock",
            "--pid-file",
            "/tmp/p.pid",
            "--log-file",
            "/tmp/l.log",
        ])
        .expect("parse daemon");
        match cli.command {
            Commands::Daemon(DaemonArgs {
                socket,
                pid_file,
                log_file,
                ..
            }) => {
                assert_eq!(socket, "/tmp/d.sock");
                assert!(pid_file.ends_with("p.pid"));
                assert!(log_file.ends_with("l.log"));
            }
            _ => panic!("expected Daemon"),
        }
    }

    #[test]
    fn cli_parses_client_with_command() {
        let cli = parse_cli(&[
            "beardog",
            "client",
            "--socket",
            "/tmp/c.sock",
            "--command",
            "health.ping",
        ])
        .expect("parse client");
        match cli.command {
            Commands::Client(ClientArgs { socket, command }) => {
                assert_eq!(socket, "/tmp/c.sock");
                assert_eq!(command.as_deref(), Some("health.ping"));
            }
            _ => panic!("expected Client"),
        }
    }

    #[test]
    fn cli_parses_doctor_comprehensive_json() {
        let cli = parse_cli(&[
            "beardog",
            "doctor",
            "--comprehensive",
            "--format",
            "json",
            "--component",
            "ipc",
        ])
        .expect("parse doctor");
        match cli.command {
            Commands::Doctor(DoctorArgs {
                comprehensive,
                format,
                component,
            }) => {
                assert!(comprehensive);
                assert_eq!(format, "json");
                assert_eq!(component.as_deref(), Some("ipc"));
            }
            _ => panic!("expected Doctor"),
        }
    }

    #[test]
    fn cli_parses_cross_primal_discover_primals() {
        let cli = parse_cli(&[
            "beardog",
            "cross-primal",
            "discover-primals",
            "--capability",
            "compute",
        ])
        .expect("parse full argv");
        assert!(
            matches!(cli.command, Commands::CrossPrimal(_)),
            "{:?}",
            cli.command
        );
        let dbg = format!("{:?}", cli.command);
        assert!(
            dbg.contains("compute"),
            "expected capability in debug: {dbg}"
        );
    }

    #[tokio::test]
    async fn dispatch_cross_primal_discover_primals_runs() {
        let cmd = CrossPrimalCommand::try_parse_from([
            "cross-primal",
            "discover-primals",
            "--capability",
            "network",
        ])
        .expect("parse");
        let r = crate::dispatch(Commands::CrossPrimal(cmd)).await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn dispatch_client_fails_on_missing_socket() {
        let args = ClientArgs {
            socket: "/nonexistent/beardog-test-client.sock".to_string(),
            command: Some(r#"{"jsonrpc":"2.0","method":"x","id":1}"#.to_string()),
        };
        let r = crate::dispatch(Commands::Client(args)).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn dispatch_hsm_list_succeeds() {
        let r = crate::dispatch(Commands::Hsm {
            action: HsmAction::List,
        })
        .await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn dispatch_hsm_discover_verbose_succeeds() {
        let r = crate::dispatch(Commands::Hsm {
            action: HsmAction::Discover { verbose: true },
        })
        .await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn dispatch_hsm_capabilities_unknown_id_ok() {
        let r = crate::dispatch(Commands::Hsm {
            action: HsmAction::Capabilities {
                hsm_id: "___cli_coverage_hsm___".to_string(),
            },
        })
        .await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn dispatch_hsm_test_unknown_id_ok() {
        let r = crate::dispatch(Commands::Hsm {
            action: HsmAction::Test {
                hsm_id: "___cli_coverage_hsm___".to_string(),
                iterations: 1,
            },
        })
        .await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn dispatch_key_list_succeeds() {
        let r = crate::dispatch(Commands::Key {
            action: KeyAction::List {
                hsm: None,
                verbose: false,
            },
        })
        .await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn dispatch_key_info_succeeds() {
        let r = crate::dispatch(Commands::Key {
            action: KeyAction::Info {
                key_id: "coverage-key-id".to_string(),
            },
        })
        .await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn dispatch_doctor_comprehensive_runs() {
        let args = DoctorArgs {
            comprehensive: true,
            format: "text".to_string(),
            component: Some("crypto".to_string()),
        };
        let r = crate::dispatch(Commands::Doctor(args)).await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn dispatch_stream_decrypt_missing_input_fails() {
        let r = crate::dispatch(Commands::StreamDecrypt {
            input: "/nonexistent/beardog-stream-in.bin".to_string(),
            output: "/tmp/beardog-stream-out.bin".to_string(),
        })
        .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn dispatch_birdsong_decrypt_missing_file_fails() {
        let r = crate::dispatch(Commands::Birdsong {
            action: BirdsongAction::Decrypt {
                input: "/nonexistent/birdsong.enc".to_string(),
                key_id: "k".to_string(),
            },
        })
        .await;
        assert!(r.is_err());
    }

    /// `Server` and `Daemon` dispatch into long-running handlers — only assert CLI structs round-trip.
    #[test]
    fn server_and_daemon_args_parse_for_dispatch_shape() {
        let s = parse_cli(&["beardog", "server"]).expect("server");
        assert!(matches!(s.command, Commands::Server(ServerArgs { .. })));
        let d = parse_cli(&["beardog", "daemon"]).expect("daemon");
        assert!(matches!(d.command, Commands::Daemon(DaemonArgs { .. })));
    }
}
