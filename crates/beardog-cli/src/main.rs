// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]
#![cfg_attr(test, allow(clippy::float_cmp))]

//! `BearDog` command-line binary (`beardog`).
//!
//! Parses subcommands and dispatches to internal `handlers` modules for entropy, keys, `BirdSong`,
//! encryption, HSM discovery, cross-primal messaging, and server/daemon modes.
//! Shared argument types (`ServerArgs`, `DaemonArgs`, etc.) live in the
//! `beardog-cli` library crate.

use beardog_cli::{BindMode, ClientArgs, DaemonArgs, DoctorArgs, ServerArgs};
use beardog_errors::BearDogError;
use clap::{Parser, Subcommand};

#[cfg(test)]
#[doc(hidden)]
mod __cli_test_env {
    use std::sync::Mutex;

    pub static HOME: Mutex<()> = Mutex::new(());
}

mod ecosystem_discovery_adapter;
mod handlers;

/// Root CLI parser: global flags and subcommand dispatch.
#[derive(Parser)]
#[command(name = "beardog")]
#[command(version, about = "BearDog - Sovereign Genetic Cryptography", long_about = None)]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

/// Top-level `beardog` subcommands.
#[derive(Subcommand)]
enum Commands {
    /// Entropy collection and seed generation
    #[command(subcommand)]
    Entropy(EntropyCommands),

    /// Key management operations
    #[command(subcommand)]
    Key(KeyCommands),

    /// `BirdSong` lineage-based encryption (privacy-preserving)
    #[command(subcommand)]
    Birdsong(BirdSongCommands),

    /// Encryption operations
    Encrypt(EncryptArgs),

    /// Decryption operations
    Decrypt(DecryptArgs),

    /// Streaming encryption for large files (100GB+)
    #[command(name = "stream-encrypt")]
    StreamEncrypt {
        /// Key ID to use for encryption
        #[arg(long)]
        key: String,
        /// Input file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: String,
    },

    /// Streaming decryption for large files (100GB+)
    #[command(name = "stream-decrypt")]
    StreamDecrypt {
        /// Input file path (encrypted)
        #[arg(short, long)]
        input: String,
        /// Output file path (decrypted)
        #[arg(short, long)]
        output: String,
    },

    /// HSM operations
    #[command(subcommand)]
    Hsm(HsmCommands),

    /// Cross-primal secure messaging (Workflow 3)
    #[command(name = "cross-primal")]
    CrossPrimal(handlers::cross_primal::CrossPrimalCommand),

    /// Show system status
    Status,

    /// Start `BearDog` server (long-running service mode)
    Server(ServerArgs),

    /// Run as daemon (background service)
    Daemon(DaemonArgs),

    /// Interactive client mode
    Client(ClientArgs),

    /// Health diagnostics
    Doctor(DoctorArgs),
}

// ============================================================================
// ENTROPY COMMANDS
// ============================================================================

/// `beardog entropy` subcommands.
#[derive(Subcommand)]
enum EntropyCommands {
    /// Collect human entropy and generate seed
    Collect {
        /// Enable human input collection (multi-modal)
        #[arg(long)]
        human_input: bool,

        /// Device preference (auto, software, mobile, usb, hardware)
        /// Auto = discover best available HSM
        #[arg(long, default_value = "auto")]
        device: String,

        /// Quality tier (1-5, where 1 is highest quality)
        #[arg(long, default_value = "2")]
        quality_tier: u8,

        /// Output file path for seed
        #[arg(short, long)]
        output: String,

        /// Human identity (optional, for sovereign seeds)
        #[arg(long)]
        identity: Option<String>,
    },

    /// Show entropy seed information
    Info {
        /// Seed file path
        #[arg(short, long)]
        seed: String,
    },
}

// ============================================================================
// KEY COMMANDS
// ============================================================================

/// `beardog key` subcommands.
#[derive(Subcommand)]
enum KeyCommands {
    /// Generate a new cryptographic key
    Generate {
        /// Key identifier (unique name)
        #[arg(long)]
        key_id: String,

        /// Algorithm (aes256-gcm, chacha20-poly1305, ed25519, rsa4096, genetic-aes256)
        #[arg(long)]
        algorithm: String,

        /// HSM preference (auto, software, hardware, mobile, usb)
        #[arg(long, default_value = "auto")]
        hsm: String,

        /// Use entropy seed (optional)
        #[arg(long)]
        seed: Option<String>,

        /// Key derivation function (pbkdf2, argon2, hkdf)
        #[arg(long, default_value = "argon2")]
        kdf: String,

        /// KDF iterations (for PBKDF2, default: 100000)
        #[arg(long)]
        kdf_iterations: Option<u32>,

        /// KDF memory cost in KiB (for Argon2, default: 65536)
        #[arg(long)]
        kdf_memory: Option<u32>,

        /// KDF time cost (for Argon2, default: 3)
        #[arg(long)]
        kdf_time: Option<u32>,

        /// Expiry duration (e.g., "24h", "30d", "1y")
        #[arg(long)]
        expires_in: Option<String>,

        /// Key purpose/description
        #[arg(long)]
        purpose: Option<String>,

        /// Usage restrictions (encrypt-only, decrypt-only, sign-only, all)
        #[arg(long, default_value = "all")]
        usage: String,
    },

    /// List available keys
    List {
        /// Filter by HSM type
        #[arg(long)]
        hsm: Option<String>,

        /// Show detailed information
        #[arg(long)]
        verbose: bool,
    },

    /// Show key information
    Info {
        /// Key identifier
        #[arg(long)]
        key_id: String,
    },

    /// Delete a key
    Delete {
        /// Key identifier
        #[arg(long)]
        key_id: String,

        /// Skip confirmation prompt
        #[arg(long)]
        yes: bool,
    },

    /// Export a key to a file (for inter-primal sharing)
    Export {
        /// Key identifier to export
        #[arg(long)]
        key_id: String,

        /// Output file path
        #[arg(long)]
        output: String,

        /// Encrypt the exported key with a password (recommended)
        #[arg(long)]
        encrypt: bool,
    },

    /// Import a key from a file (from another primal/tower)
    Import {
        /// Input file path
        #[arg(long)]
        input: String,

        /// Key identifier for the imported key (optional, uses file's `key_id` if not provided)
        #[arg(long)]
        key_id: Option<String>,

        /// Decrypt the imported key with a password
        #[arg(long)]
        decrypt: bool,
    },

    /// Derive a new key from an existing master key
    Derive {
        /// Master key identifier to derive from
        #[arg(long)]
        master_key: String,

        /// Purpose/context for the derived key (used in derivation)
        #[arg(long)]
        purpose: String,

        /// Output key identifier for the derived key
        #[arg(long)]
        output: String,

        /// Expiry duration (e.g., "24h", "30d", "1y")
        #[arg(long)]
        expires_in: Option<String>,
    },

    /// Mix two keys cryptographically for shared access
    Mix {
        /// First key identifier
        #[arg(long)]
        key1: String,

        /// Second key identifier
        #[arg(long)]
        key2: String,

        /// Output key identifier for mixed key
        #[arg(long)]
        output: String,

        /// Threshold scheme (e.g., "2-of-2", "1-of-2")
        #[arg(long, default_value = "2-of-2")]
        threshold: String,

        /// Expiry duration (e.g., "24h", "30d", "1y")
        #[arg(long)]
        expires_in: Option<String>,
    },

    /// Show key lineage (parent-child relationships)
    Lineage {
        /// Key identifier to show lineage for
        #[arg(long)]
        key_id: String,

        /// Output in JSON format (machine-readable)
        #[arg(long)]
        json: bool,
    },

    /// Delegate key with time and resource constraints
    Delegate {
        /// Master key to delegate from
        #[arg(long)]
        master_key: String,

        /// Delegate to (identity or key ID)
        #[arg(long)]
        delegate_to: String,

        /// Output delegated key identifier
        #[arg(long)]
        output: String,

        /// Time range (e.g., "9:00-17:00")
        #[arg(long)]
        time_range: Option<String>,

        /// Allowed weekdays (e.g., "mon-fri" or "mon,wed,fri")
        #[arg(long)]
        weekdays: Option<String>,

        /// CPU quota (0-100%)
        #[arg(long)]
        cpu_quota: Option<u8>,

        /// Memory quota (e.g., "8GB", "512MB")
        #[arg(long)]
        memory_quota: Option<String>,

        /// Expiry duration (required, e.g., "30d")
        #[arg(long)]
        expires_in: String,
    },

    /// Revoke a key (sovereign revocation)
    Revoke {
        /// Key identifier to revoke
        #[arg(long)]
        key_id: String,

        /// Reason for revocation
        #[arg(long)]
        reason: Option<String>,

        /// Effective-at timestamp (ISO 8601 format, e.g., "2025-12-20T00:00:00Z")
        #[arg(long)]
        effective_at: Option<String>,

        /// Cascade revocation to child keys
        #[arg(long)]
        cascade: bool,
    },

    /// Check if a key is revoked
    CheckRevocation {
        /// Key identifier to check
        #[arg(long)]
        key_id: String,
    },

    /// List all revoked keys
    ListRevocations,
}

// ============================================================================
// ENCRYPT/DECRYPT COMMANDS
// ============================================================================

/// Arguments for `beardog encrypt`.
#[derive(Parser)]
struct EncryptArgs {
    /// Key ID to use for encryption
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
}

/// Arguments for `beardog decrypt`.
#[derive(Parser)]
struct DecryptArgs {
    /// Key ID to use for decryption
    #[arg(long)]
    key: String,

    /// Input file path (encrypted)
    #[arg(short, long)]
    input: String,

    /// Output file path (plaintext)
    #[arg(short, long)]
    output: String,
}

// ============================================================================
// BIRDSONG COMMANDS
// ============================================================================

/// `beardog birdsong` subcommands.
#[derive(Subcommand)]
enum BirdSongCommands {
    /// Encrypt message for lineage only (privacy-preserving)
    Encrypt {
        /// Message to encrypt
        #[arg(long)]
        message: String,

        /// Lineage hint type (`DirectAncestors`, `AllDescendants`, `RootOnly`, Depth:min-max)
        #[arg(long)]
        hint: String,

        /// Root lineage ID
        #[arg(long)]
        root_id: String,

        /// Output file path (default: encrypted.birdsong)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Decrypt `BirdSong` message (if in lineage)
    Decrypt {
        /// Input file path (encrypted broadcast)
        #[arg(short, long)]
        input: String,

        /// Key ID to use for decryption
        #[arg(long)]
        key_id: String,
    },
}

// ============================================================================
// HSM COMMANDS
// ============================================================================

/// `beardog hsm` subcommands.
#[derive(Subcommand)]
enum HsmCommands {
    /// Discover available HSMs
    Discover {
        /// Show detailed capabilities
        #[arg(long)]
        verbose: bool,
    },

    /// Show HSM capabilities
    Capabilities {
        /// HSM identifier (from discover command)
        #[arg(long)]
        hsm_id: String,
    },

    /// Test HSM functionality
    Test {
        /// HSM identifier
        #[arg(long)]
        hsm_id: String,

        /// Number of test iterations
        #[arg(long, default_value = "10")]
        iterations: usize,
    },
}

// ============================================================================
// MAIN FUNCTION
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .with_target(false)
        .init();

    match cli.command {
        Commands::Entropy(entropy_cmd) => match entropy_cmd {
            EntropyCommands::Collect {
                human_input,
                device,
                quality_tier,
                output,
                identity,
            } => {
                handlers::entropy::handle_entropy_collect(
                    human_input,
                    &device,
                    quality_tier,
                    &output,
                    identity.as_deref(),
                )
                .await?;
            }
            EntropyCommands::Info { seed } => {
                handlers::entropy::handle_entropy_info(&seed).await?;
            }
        },
        Commands::Birdsong(birdsong_cmd) => match birdsong_cmd {
            BirdSongCommands::Encrypt {
                message,
                hint,
                root_id,
                output,
            } => {
                handlers::birdsong::handle_birdsong_encrypt(
                    &message,
                    &hint,
                    &root_id,
                    output.as_deref(),
                )
                .await?;
            }
            BirdSongCommands::Decrypt { input, key_id } => {
                handlers::birdsong::handle_birdsong_decrypt(&input, &key_id).await?;
            }
        },
        Commands::Key(key_cmd) => match key_cmd {
            KeyCommands::Generate {
                key_id,
                algorithm,
                hsm,
                seed,
                kdf,
                kdf_iterations,
                kdf_memory,
                kdf_time,
                usage,
                expires_in,
                purpose,
            } => {
                handlers::key::handle_key_generate_v2(
                    &key_id,
                    &algorithm,
                    &hsm,
                    seed.as_deref(),
                    &kdf,
                    kdf_iterations,
                    kdf_memory,
                    kdf_time,
                    Some(usage.as_str()), // usage is String with default
                    expires_in.as_deref(),
                    purpose.as_deref(),
                )
                .await?;
            }
            KeyCommands::List { hsm, verbose } => {
                handlers::key::handle_key_list(hsm.as_deref(), verbose).await?;
            }
            KeyCommands::Info { key_id } => {
                handlers::key::handle_key_info(&key_id).await?;
            }
            KeyCommands::Delete { key_id, yes } => {
                handlers::key::handle_key_delete(&key_id, yes).await?;
            }
            KeyCommands::Export {
                key_id,
                output,
                encrypt,
            } => {
                handlers::key_export::handle_key_export(&key_id, &output, encrypt).await?;
            }
            KeyCommands::Import {
                input,
                key_id,
                decrypt,
            } => {
                handlers::key_export::handle_key_import(&input, key_id.as_deref(), decrypt).await?;
            }
            KeyCommands::Derive {
                master_key,
                purpose,
                output,
                expires_in,
            } => {
                handlers::key_derive::handle_key_derive(
                    &master_key,
                    &purpose,
                    &output,
                    expires_in.as_deref(),
                )
                .await?;
            }
            KeyCommands::Mix {
                key1,
                key2,
                output,
                threshold,
                expires_in,
            } => {
                handlers::key_mix::handle_key_mix(
                    &key1,
                    &key2,
                    &output,
                    &threshold,
                    expires_in.as_deref(),
                )
                .await?;
            }
            KeyCommands::Lineage { key_id, json } => {
                handlers::key_lineage::handle_key_lineage(&key_id, json).await?;
            }
            KeyCommands::Delegate {
                master_key,
                delegate_to,
                output,
                time_range,
                weekdays,
                cpu_quota,
                memory_quota,
                expires_in,
            } => {
                handlers::key_delegate::handle_key_delegate(
                    &master_key,
                    &delegate_to,
                    &output,
                    time_range.as_deref(),
                    weekdays.as_deref(),
                    cpu_quota,
                    memory_quota.as_deref(),
                    &expires_in,
                )
                .await?;
            }
            KeyCommands::Revoke {
                key_id,
                reason,
                effective_at,
                cascade,
            } => {
                handlers::key_revoke::handle_key_revoke(
                    &key_id,
                    reason.as_deref(),
                    effective_at.as_deref(),
                    cascade,
                )
                .await?;
            }
            KeyCommands::CheckRevocation { key_id } => {
                handlers::key_revoke::handle_key_check_revocation(&key_id).await?;
            }
            KeyCommands::ListRevocations => {
                handlers::key_revoke::handle_key_list_revocations().await?;
            }
        },
        Commands::Encrypt(args) => {
            handlers::encrypt::handle_encrypt(&args.key, &args.input, &args.output, args.genetic)
                .await?;
        }
        Commands::Decrypt(args) => {
            handlers::decrypt::handle_decrypt(&args.key, &args.input, &args.output).await?;
        }
        Commands::StreamEncrypt { key, input, output } => {
            handlers::streaming::handle_streaming_encrypt(&key, &input, &output).await?;
        }
        Commands::StreamDecrypt { input, output } => {
            handlers::streaming::handle_streaming_decrypt(&input, &output).await?;
        }
        Commands::Hsm(hsm_cmd) => match hsm_cmd {
            HsmCommands::Discover { verbose } => {
                handlers::hsm::handle_hsm_discover(verbose).await?;
            }
            HsmCommands::Capabilities { hsm_id } => {
                handlers::hsm::handle_hsm_capabilities(&hsm_id).await?;
            }
            HsmCommands::Test { hsm_id, iterations } => {
                handlers::hsm::handle_hsm_test(&hsm_id, iterations).await?;
            }
        },
        Commands::CrossPrimal(cmd) => {
            handlers::cross_primal::handle_cross_primal(cmd).await?;
        }
        Commands::Status => {
            handlers::status::show_status().await?;
        }
        Commands::Server(args) => {
            handlers::server::handle_server(args).await?;
        }
        Commands::Daemon(args) => {
            handlers::daemon::handle_daemon(args).await?;
        }
        Commands::Client(args) => {
            handlers::client::handle_client(args).await?;
        }
        Commands::Doctor(args) => {
            handlers::doctor::handle_doctor(args).await?;
        }
    }

    Ok(())
}
