// BearDog CLI - Main Entry Point
// Vendor-agnostic, primal-agnostic, algorithm-agnostic, transport-agnostic

use beardog_errors::BearDogError;
use clap::{Parser, Subcommand};

mod ecosystem_discovery_adapter;
mod handlers;

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

#[derive(Subcommand)]
enum Commands {
    /// Entropy collection and seed generation
    #[command(subcommand)]
    Entropy(EntropyCommands),

    /// Key management operations
    #[command(subcommand)]
    Key(KeyCommands),

    /// Encryption operations
    Encrypt(EncryptArgs),

    /// Decryption operations
    Decrypt(DecryptArgs),

    /// HSM operations
    #[command(subcommand)]
    Hsm(HsmCommands),

    /// Cross-primal secure messaging (Workflow 3)
    #[command(name = "cross-primal")]
    CrossPrimal(handlers::cross_primal::CrossPrimalCommand),

    /// Show system status
    Status,
}

// ============================================================================
// ENTROPY COMMANDS
// ============================================================================

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
}

// ============================================================================
// ENCRYPT/DECRYPT COMMANDS
// ============================================================================

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
// HSM COMMANDS
// ============================================================================

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
        Commands::Key(key_cmd) => match key_cmd {
            KeyCommands::Generate {
                key_id,
                algorithm,
                hsm,
                seed,
            } => {
                handlers::key::handle_key_generate(&key_id, &algorithm, &hsm, seed.as_deref())
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
        },
        Commands::Encrypt(args) => {
            handlers::encrypt::handle_encrypt(&args.key, &args.input, &args.output, args.genetic)
                .await?;
        }
        Commands::Decrypt(args) => {
            handlers::decrypt::handle_decrypt(&args.key, &args.input, &args.output).await?;
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
    }

    Ok(())
}
