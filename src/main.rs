// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! `BearDog` `UniBin` — one binary, multiple operational modes.
//!
//! Subcommands cover the full CLI surface: service lifecycle, crypto operations,
//! key management, HSM discovery, entropy, and diagnostics.

use beardog_cli::handlers::cross_primal::CrossPrimalCommand;
use beardog_cli::handlers::{
    birdsong, client, cross_primal, daemon, decrypt, doctor, encrypt, entropy, hsm, key,
    key_derive, key_export, key_lineage, key_mix, key_revoke, server, status, streaming,
};
use beardog_cli::{ClientArgs, DaemonArgs, DoctorArgs, ServerArgs};
use clap::{Parser, Subcommand};
use color_eyre::eyre::{Result, eyre};
use tracing_subscriber::{EnvFilter, fmt};

#[derive(Debug, Parser)]
#[command(name = "beardog")]
#[command(version, about = "BearDog - Sovereign Genetic Cryptography", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Long-running IPC / multi-transport service
    Server(ServerArgs),
    /// Background daemon (PID file, logging)
    Daemon(DaemonArgs),
    /// Interactive client (JSON-RPC over Unix socket)
    Client(ClientArgs),
    /// Health diagnostics
    Doctor(DoctorArgs),
    /// Print version string
    Version,
    /// Security and capability summary
    Capabilities,

    /// Key management operations
    Key {
        #[command(subcommand)]
        action: KeyAction,
    },
    /// Entropy collection and seed generation
    Entropy {
        #[command(subcommand)]
        action: EntropyAction,
    },
    /// HSM Discovery and management
    Hsm {
        #[command(subcommand)]
        action: HsmAction,
    },
    /// Encrypt a file
    Encrypt {
        /// Key ID to use
        #[arg(long)]
        key: String,
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Use genetic algorithm (if supported by key)
        #[arg(long)]
        genetic: bool,
    },
    /// Decrypt a file
    Decrypt {
        /// Key ID to use
        #[arg(long)]
        key: String,
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
    },
    /// `BirdSong` lineage-based encryption
    Birdsong {
        #[command(subcommand)]
        action: BirdsongAction,
    },
    /// Cross-primal secure messaging (capability-based discovery)
    #[command(name = "cross-primal")]
    CrossPrimal(CrossPrimalCommand),
    /// Streaming encryption for large files
    #[command(name = "stream-encrypt")]
    StreamEncrypt {
        /// Key ID to use
        #[arg(long)]
        key: String,
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
    },
    /// Streaming decryption for large files
    #[command(name = "stream-decrypt")]
    StreamDecrypt {
        /// Input file path (encrypted)
        #[arg(short, long)]
        input: String,
        /// Output file path (decrypted)
        #[arg(short, long)]
        output: String,
    },
}

#[derive(Debug, Subcommand)]
enum KeyAction {
    /// Generate a new key
    Generate {
        #[arg(long)]
        key_id: String,
        #[arg(long, default_value = "aes256-gcm")]
        algorithm: String,
        #[arg(long, default_value = "auto")]
        hsm: String,
        /// Optional entropy seed file path
        #[arg(long)]
        seed: Option<String>,
    },
    /// List all keys
    List {
        #[arg(long)]
        hsm: Option<String>,
        #[arg(long)]
        verbose: bool,
    },
    /// Show key info
    Info {
        #[arg(long)]
        key_id: String,
    },
    /// Delete a key
    Delete {
        #[arg(long)]
        key_id: String,
        #[arg(long)]
        yes: bool,
    },
    /// Derive a child key
    Derive {
        #[arg(long)]
        master_key: String,
        #[arg(long)]
        purpose: String,
        #[arg(long)]
        output: String,
        #[arg(long)]
        expires_in: Option<String>,
    },
    /// Mix two keys
    Mix {
        #[arg(long)]
        key1: String,
        #[arg(long)]
        key2: String,
        #[arg(long)]
        output: String,
        #[arg(long, default_value = "2-of-2")]
        threshold: String,
        #[arg(long)]
        expires_in: Option<String>,
    },
    /// Show key lineage
    Lineage {
        #[arg(long)]
        key_id: String,
        #[arg(long)]
        json: bool,
    },
    /// Export a key
    Export {
        #[arg(long)]
        key_id: String,
        #[arg(long)]
        output: String,
        /// Encrypt the exported key with a password (interactive)
        #[arg(long)]
        encrypt: bool,
    },
    /// Import a key
    Import {
        #[arg(long)]
        input: String,
        #[arg(long)]
        key_id: Option<String>,
        /// Decrypt the imported key with a password (interactive)
        #[arg(long)]
        decrypt: bool,
    },
    /// Revoke a key
    Revoke {
        #[arg(long)]
        key_id: String,
        #[arg(long)]
        reason: Option<String>,
        #[arg(long)]
        effective_at: Option<String>,
        #[arg(long)]
        cascade: bool,
    },
}

#[derive(Debug, Subcommand)]
enum EntropyAction {
    /// Collect entropy from sources
    Collect {
        #[arg(long)]
        human_input: bool,
        #[arg(long, default_value = "auto")]
        device: String,
        #[arg(long, default_value = "2")]
        quality_tier: u8,
        #[arg(short, long)]
        output: String,
        #[arg(long)]
        identity: Option<String>,
    },
    /// Show entropy seed information
    Info {
        #[arg(short, long)]
        seed: String,
    },
}

#[derive(Debug, Subcommand)]
enum HsmAction {
    /// Discover available HSMs
    Discover {
        #[arg(long)]
        verbose: bool,
    },
    /// List registered HSMs
    List,
    /// Show HSM capabilities
    Capabilities {
        #[arg(long)]
        hsm_id: String,
    },
    /// Test HSM operations
    Test {
        #[arg(long)]
        hsm_id: String,
        #[arg(long, default_value = "10")]
        iterations: usize,
    },
}

#[derive(Debug, Subcommand)]
enum BirdsongAction {
    /// Encrypt with `BirdSong` lineage
    Encrypt {
        #[arg(long)]
        message: String,
        #[arg(long)]
        hint: String,
        #[arg(long)]
        root_id: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Decrypt with `BirdSong` lineage
    Decrypt {
        #[arg(short, long)]
        input: String,
        #[arg(long)]
        key_id: String,
    },
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // Idempotent: tests may call `init_tracing` more than once; ignore duplicate installs.
    let _ = fmt().with_env_filter(filter).with_target(false).try_init();
}

async fn dispatch(command: Commands) -> Result<()> {
    let err = |e: beardog_errors::BearDogError| eyre!(e);

    match command {
        Commands::Server(args) => server::handle_server(args).await.map_err(err),
        Commands::Daemon(args) => daemon::handle_daemon(args).await.map_err(err),
        Commands::Client(args) => client::handle_client(args).await.map_err(err),
        Commands::Doctor(args) => doctor::handle_doctor(args).await.map_err(err),
        Commands::Version => status::handle_version().await.map_err(err),
        Commands::Capabilities => status::handle_status(false).await.map_err(err),

        Commands::Key { action } => match action {
            KeyAction::Generate {
                key_id,
                algorithm,
                hsm,
                seed,
            } => key::handle_key_generate_v2(
                &key_id,
                &algorithm,
                &hsm,
                seed.as_deref(),
                "argon2",
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .map_err(err),
            KeyAction::List { hsm, verbose } => key::handle_key_list(hsm.as_deref(), verbose)
                .await
                .map_err(err),
            KeyAction::Info { key_id } => key::handle_key_info(&key_id).await.map_err(err),
            KeyAction::Delete { key_id, yes } => {
                key::handle_key_delete(&key_id, yes).await.map_err(err)
            }
            KeyAction::Derive {
                master_key,
                purpose,
                output,
                expires_in,
            } => {
                key_derive::handle_key_derive(&master_key, &purpose, &output, expires_in.as_deref())
                    .await
                    .map_err(err)
            }
            KeyAction::Mix {
                key1,
                key2,
                output,
                threshold,
                expires_in,
            } => key_mix::handle_key_mix(&key1, &key2, &output, &threshold, expires_in.as_deref())
                .await
                .map_err(err),
            KeyAction::Lineage { key_id, json } => key_lineage::handle_key_lineage(&key_id, json)
                .await
                .map_err(err),
            KeyAction::Export {
                key_id,
                output,
                encrypt,
            } => key_export::handle_key_export(&key_id, &output, encrypt)
                .await
                .map_err(err),
            KeyAction::Import {
                input,
                key_id,
                decrypt,
            } => key_export::handle_key_import(&input, key_id.as_deref(), decrypt)
                .await
                .map_err(err),
            KeyAction::Revoke {
                key_id,
                reason,
                effective_at,
                cascade,
            } => key_revoke::handle_key_revoke(
                &key_id,
                reason.as_deref(),
                effective_at.as_deref(),
                cascade,
            )
            .await
            .map_err(err),
        },

        Commands::Entropy { action } => match action {
            EntropyAction::Collect {
                human_input,
                device,
                quality_tier,
                output,
                identity,
            } => entropy::handle_entropy_collect(
                human_input,
                &device,
                quality_tier,
                &output,
                identity.as_deref(),
            )
            .await
            .map_err(err),
            EntropyAction::Info { seed } => entropy::handle_entropy_info(&seed).await.map_err(err),
        },

        Commands::Hsm { action } => match action {
            HsmAction::Discover { verbose } => hsm::handle_hsm_discover(verbose).await.map_err(err),
            HsmAction::List => hsm::handle_hsm_list().await.map_err(err),
            HsmAction::Capabilities { hsm_id } => {
                hsm::handle_hsm_capabilities(&hsm_id).await.map_err(err)
            }
            HsmAction::Test { hsm_id, iterations } => {
                hsm::handle_hsm_test(&hsm_id, iterations).await.map_err(err)
            }
        },

        Commands::Encrypt {
            key,
            input,
            output,
            genetic,
        } => encrypt::handle_encrypt(&key, &input, &output, genetic)
            .await
            .map_err(err),
        Commands::Decrypt { key, input, output } => decrypt::handle_decrypt(&key, &input, &output)
            .await
            .map_err(err),
        Commands::Birdsong { action } => match action {
            BirdsongAction::Encrypt {
                message,
                hint,
                root_id,
                output,
            } => birdsong::handle_birdsong_encrypt(&message, &hint, &root_id, output.as_deref())
                .await
                .map_err(err),
            BirdsongAction::Decrypt { input, key_id } => {
                birdsong::handle_birdsong_decrypt(&input, &key_id)
                    .await
                    .map_err(err)
            }
        },
        Commands::CrossPrimal(cmd) => cross_primal::handle_cross_primal(cmd).await.map_err(err),
        Commands::StreamEncrypt { key, input, output } => {
            streaming::handle_streaming_encrypt(&key, &input, &output)
                .await
                .map_err(err)
        }
        Commands::StreamDecrypt { input, output } => {
            streaming::handle_streaming_decrypt(&input, &output)
                .await
                .map_err(err)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    init_tracing();

    dispatch(cli.command).await
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

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
}

#[cfg(test)]
#[path = "main_coverage_extension.rs"]
mod main_coverage_extension;
