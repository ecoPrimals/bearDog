#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

use super::*;
use clap::Parser;

fn parse_cli(args: &[&str]) -> Result<Cli, clap::Error> {
    Cli::try_parse_from(args)
}

#[test]
fn cli_parses_version() {
    let cli = parse_cli(&["beardog", "version"]);
    assert!(cli.is_ok());
    assert!(matches!(cli.unwrap().command, Commands::Version));
}

#[test]
fn cli_parses_capabilities() {
    let cli = parse_cli(&["beardog", "capabilities"]);
    assert!(cli.is_ok());
    assert!(matches!(cli.unwrap().command, Commands::Capabilities));
}

#[test]
fn cli_parses_server() {
    let cli = parse_cli(&["beardog", "server"]);
    assert!(cli.is_ok());
    assert!(matches!(cli.unwrap().command, Commands::Server(_)));
}

#[test]
fn cli_parses_client() {
    let cli = parse_cli(&["beardog", "client"]);
    assert!(cli.is_ok());
    assert!(matches!(cli.unwrap().command, Commands::Client(_)));
}

#[test]
fn cli_parses_doctor() {
    let cli = parse_cli(&["beardog", "doctor"]);
    assert!(cli.is_ok());
    assert!(matches!(cli.unwrap().command, Commands::Doctor(_)));
}

#[test]
fn cli_parses_daemon() {
    let cli = parse_cli(&["beardog", "daemon"]);
    assert!(cli.is_ok());
    assert!(matches!(cli.unwrap().command, Commands::Daemon(_)));
}

#[test]
fn cli_parses_key_generate() {
    let cli = parse_cli(&["beardog", "key", "generate", "--key-id", "test-key"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Key {
            action: KeyAction::Generate { .. }
        }
    ));
}

#[test]
fn cli_parses_key_list() {
    let cli = parse_cli(&["beardog", "key", "list"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Key {
            action: KeyAction::List { .. }
        }
    ));
}

#[test]
fn cli_parses_key_info() {
    let cli = parse_cli(&["beardog", "key", "info", "--key-id", "k1"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Key {
            action: KeyAction::Info { .. }
        }
    ));
}

#[test]
fn cli_parses_key_delete() {
    let cli = parse_cli(&["beardog", "key", "delete", "--key-id", "k1", "--yes"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Key {
            action: KeyAction::Delete { .. }
        }
    ));
}

#[test]
fn cli_parses_key_derive() {
    let cli = parse_cli(&[
        "beardog",
        "key",
        "derive",
        "--master-key",
        "m",
        "--purpose",
        "p",
        "--output",
        "o",
    ]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Key {
            action: KeyAction::Derive { .. }
        }
    ));
}

#[test]
fn cli_parses_key_mix() {
    let cli = parse_cli(&[
        "beardog", "key", "mix", "--key1", "a", "--key2", "b", "--output", "o",
    ]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Key {
            action: KeyAction::Mix { .. }
        }
    ));
}

#[test]
fn cli_parses_key_lineage() {
    let cli = parse_cli(&["beardog", "key", "lineage", "--key-id", "k1"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Key {
            action: KeyAction::Lineage { .. }
        }
    ));
}

#[test]
fn cli_parses_key_export() {
    let cli = parse_cli(&[
        "beardog", "key", "export", "--key-id", "k1", "--output", "o",
    ]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Key {
            action: KeyAction::Export { .. }
        }
    ));
}

#[test]
fn cli_parses_key_import() {
    let cli = parse_cli(&["beardog", "key", "import", "--input", "i"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Key {
            action: KeyAction::Import { .. }
        }
    ));
}

#[test]
fn cli_parses_key_revoke() {
    let cli = parse_cli(&["beardog", "key", "revoke", "--key-id", "k1"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Key {
            action: KeyAction::Revoke { .. }
        }
    ));
}

#[test]
fn cli_parses_entropy_collect() {
    let cli = parse_cli(&["beardog", "entropy", "collect", "--output", "/tmp/seed"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Entropy {
            action: EntropyAction::Collect { .. }
        }
    ));
}

#[test]
fn cli_parses_entropy_info() {
    let cli = parse_cli(&["beardog", "entropy", "info", "--seed", "/tmp/seed"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Entropy {
            action: EntropyAction::Info { .. }
        }
    ));
}

#[test]
fn cli_parses_hsm_discover() {
    let cli = parse_cli(&["beardog", "hsm", "discover"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Hsm {
            action: HsmAction::Discover { .. }
        }
    ));
}

#[test]
fn cli_parses_hsm_list() {
    let cli = parse_cli(&["beardog", "hsm", "list"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Hsm {
            action: HsmAction::List
        }
    ));
}

#[test]
fn cli_parses_hsm_capabilities() {
    let cli = parse_cli(&["beardog", "hsm", "capabilities", "--hsm-id", "h1"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Hsm {
            action: HsmAction::Capabilities { .. }
        }
    ));
}

#[test]
fn cli_parses_hsm_test() {
    let cli = parse_cli(&["beardog", "hsm", "test", "--hsm-id", "h1"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::Hsm {
            action: HsmAction::Test { .. }
        }
    ));
}

#[test]
fn cli_parses_encrypt() {
    let cli = parse_cli(&[
        "beardog", "encrypt", "--key", "k1", "--input", "i", "--output", "o",
    ]);
    assert!(cli.is_ok());
    assert!(matches!(cli.unwrap().command, Commands::Encrypt { .. }));
}

#[test]
fn cli_parses_decrypt() {
    let cli = parse_cli(&[
        "beardog", "decrypt", "--key", "k1", "--input", "i", "--output", "o",
    ]);
    assert!(cli.is_ok());
    assert!(matches!(cli.unwrap().command, Commands::Decrypt { .. }));
}

#[test]
fn cli_parses_birdsong_encrypt() {
    let cli = parse_cli(&[
        "beardog",
        "birdsong",
        "encrypt",
        "--message",
        "hello",
        "--hint",
        "h",
        "--root-id",
        "r",
    ]);
    assert!(cli.is_ok());
    assert!(matches!(cli.unwrap().command, Commands::Birdsong { .. }));
}

#[test]
fn cli_parses_cross_primal() {
    let cli = parse_cli(&["beardog", "cross-primal", "--help"]);
    assert!(cli.is_err(), "cross-primal --help should exit via clap");
}

#[test]
fn cli_parses_stream_encrypt() {
    let cli = parse_cli(&[
        "beardog",
        "stream-encrypt",
        "--key",
        "k",
        "--input",
        "i",
        "--output",
        "o",
    ]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::StreamEncrypt { .. }
    ));
}

#[test]
fn cli_parses_stream_decrypt() {
    let cli = parse_cli(&["beardog", "stream-decrypt", "--input", "i", "--output", "o"]);
    assert!(cli.is_ok());
    assert!(matches!(
        cli.unwrap().command,
        Commands::StreamDecrypt { .. }
    ));
}

#[test]
fn cli_rejects_empty_args() {
    let cli = parse_cli(&["beardog"]);
    assert!(cli.is_err());
}

#[test]
fn cli_rejects_unknown_subcommand() {
    let cli = parse_cli(&["beardog", "does-not-exist"]);
    assert!(cli.is_err());
}

#[tokio::test]
async fn dispatch_version_succeeds() {
    let result = dispatch(Commands::Version).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn dispatch_capabilities_succeeds() {
    let result = dispatch(Commands::Capabilities).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn dispatch_doctor_succeeds() {
    let args = DoctorArgs {
        comprehensive: false,
        format: "text".to_string(),
        component: None,
    };
    let result = dispatch(Commands::Doctor(args)).await;
    assert!(result.is_ok());
}
