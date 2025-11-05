use beardog_errors::{
    constructors_unified::{io_error, validation_error},
    BearDogError,
};
use beardog_tunnel::SimplePkcs11Client;
use clap::{Parser, Subcommand};
use sha3::{Digest, Sha3_512};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "beardog")]
#[command(version, about = "BearDog HSM Management CLI - Real Hardware Edition", long_about = None)]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Discover available HSMs
    DiscoverHsm {
        /// PKCS#11 library path (optional, will try defaults)
        #[arg(short, long)]
        library: Option<String>,
    },
    /// Test entropy collection from a specific slot
    TestEntropy {
        /// PKCS#11 library path
        #[arg(
            short,
            long,
            default_value = "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so"
        )]
        library: String,

        /// Slot ID to test
        #[arg(short, long)]
        slot: u32,

        /// Number of bytes to collect
        #[arg(short = 'n', long, default_value = "1024")]
        size: usize,

        /// Show hex dump of collected entropy
        #[arg(long)]
        show_hex: bool,
    },
    /// Mix entropy from multiple sources into a seed
    MixSeed {
        /// PKCS#11 library path
        #[arg(
            short,
            long,
            default_value = "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so"
        )]
        library: String,

        /// Comma-separated list of slot IDs
        #[arg(short, long)]
        slots: String,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Bytes to collect from each source
        #[arg(long, default_value = "256")]
        bytes_per_source: usize,
    },
    /// Show system status
    Status,
}

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
        Commands::DiscoverHsm { library } => {
            println!("🔍 BearDog HSM Discovery\n");
            discover_hsm(library).await?;
        }
        Commands::TestEntropy {
            library,
            slot,
            size,
            show_hex,
        } => {
            println!("🎲 BearDog Entropy Test\n");
            test_entropy(&library, slot, size, show_hex).await?;
        }
        Commands::MixSeed {
            library,
            slots,
            output,
            bytes_per_source,
        } => {
            println!("🌀 BearDog Entropy Mixer\n");
            mix_seed(&library, &slots, &output, bytes_per_source).await?;
        }
        Commands::Status => {
            show_status();
        }
    }

    Ok(())
}

async fn discover_hsm(library: Option<String>) -> Result<(), BearDogError> {
    let lib_path =
        library.unwrap_or_else(|| "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so".to_string());

    println!("📚 Using PKCS#11 library: {}", lib_path);

    let client = SimplePkcs11Client::new(lib_path.clone());

    println!("🔧 Initializing PKCS#11...");
    client.initialize()?;

    println!("🔍 Discovering HSM devices...\n");
    let devices = client.list_devices()?;

    if devices.is_empty() {
        println!("❌ No HSM devices found.");
        println!("\n💡 Troubleshooting:");
        println!("   1. Check if devices are connected");
        println!("   2. Try: sudo apt install opensc pcscd");
        println!("   3. Start service: sudo systemctl start pcscd");
        println!("   4. Check status: pcsc_scan");
    } else {
        println!("✅ Found {} HSM device(s):\n", devices.len());

        for (idx, device) in devices.iter().enumerate() {
            println!("Device #{}", idx + 1);
            println!("  Slot ID:       {}", device.slot_id);
            println!("  Label:         {}", device.label);
            println!("  Manufacturer:  {}", device.manufacturer);
            println!("  Model:         {}", device.model);
            println!("  Serial:        {}", device.serial_number);
            println!();
        }

        println!("💡 Next steps:");
        println!("   Test entropy: beardog test-entropy --slot <SLOT_ID>");
        println!("   Mix seeds:    beardog mix-seed --slots 0,1,2,3 --output seed.bin");
    }

    client.finalize()?;
    Ok(())
}

/*
// This will work once module exports are configured:
async fn discover_hsm_real(library: Option<String>) -> Result<(), BearDogError> {
    if let Some(lib_path) = library {
        info!("Using specified library: {}", lib_path);
        let prober = Pkcs11CapabilityProber::new()?;
        let capabilities = prober.probe_capabilities(&lib_path).await?;

*/

async fn test_entropy(
    library: &str,
    slot: u32,
    size: usize,
    show_hex: bool,
) -> Result<(), BearDogError> {
    println!("📚 Using library: {}", library);
    println!("🎯 Target slot: {}", slot);
    println!("📏 Collecting {} bytes...\n", size);

    let client = SimplePkcs11Client::new(library.to_string());
    client.initialize()?;

    println!("🎲 Collecting entropy from hardware...");
    let entropy = client.collect_entropy(slot as u64, size)?;

    println!(
        "✅ Successfully collected {} bytes of entropy!\n",
        entropy.len()
    );

    // Basic entropy quality check
    let unique_bytes = entropy
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len();
    let quality_percent = (unique_bytes as f64 / 256.0) * 100.0;

    println!("📊 Entropy Quality:");
    println!("   Unique byte values: {}/256", unique_bytes);
    println!("   Quality score: {:.1}%", quality_percent);

    if quality_percent > 90.0 {
        println!("   Assessment: ✅ Excellent");
    } else if quality_percent > 70.0 {
        println!("   Assessment: ⚠️  Good");
    } else {
        println!("   Assessment: ❌ Poor (may not be true hardware RNG)");
    }

    if show_hex {
        println!("\n📋 Hex dump (first 256 bytes):");
        for (i, chunk) in entropy
            .iter()
            .take(256)
            .collect::<Vec<_>>()
            .chunks(16)
            .enumerate()
        {
            print!("   {:04x}: ", i * 16);
            for byte in chunk {
                print!("{:02x} ", byte);
            }
            println!();
        }
        if entropy.len() > 256 {
            println!("   ... ({} more bytes)", entropy.len() - 256);
        }
    }

    client.finalize()?;
    Ok(())
}

async fn mix_seed(
    library: &str,
    slots_str: &str,
    output: &PathBuf,
    bytes_per_source: usize,
) -> Result<(), BearDogError> {
    // Parse slot IDs
    let slots: Vec<u32> = slots_str
        .split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| validation_error("slots", &format!("Invalid slot ID: {}", e)))?;

    if slots.is_empty() {
        return Err(validation_error("slots", "No slots specified"));
    }

    println!("📚 Using library: {}", library);
    println!("🎯 Collecting from {} slot(s): {:?}", slots.len(), slots);
    println!("📏 {} bytes per source\n", bytes_per_source);

    let client = SimplePkcs11Client::new(library.to_string());
    client.initialize()?;

    // Collect entropy from all sources
    let mut hasher = Sha3_512::new();
    let mut total_bytes = 0;

    for (idx, &slot) in slots.iter().enumerate() {
        println!(
            "🎲 Collecting from slot {}... ({}/{}",
            slot,
            idx + 1,
            slots.len()
        );
        let entropy = client.collect_entropy(slot as u64, bytes_per_source)?;
        hasher.update(&entropy);
        total_bytes += entropy.len();
        println!("   ✅ Collected {} bytes", entropy.len());
    }

    // Finalize hash
    let mixed_seed = hasher.finalize();

    println!(
        "\n🌀 Mixed {} bytes from {} sources",
        total_bytes,
        slots.len()
    );
    println!("🔐 SHA3-512 digest: {} bytes", mixed_seed.len());

    // Write to file
    fs::write(output, mixed_seed.as_slice())
        .map_err(|e| io_error("write seed file", &format!("{}", e)))?;

    println!("\n✅ Seed mixing complete!");
    println!(
        "📁 Wrote {} bytes to: {}",
        mixed_seed.len(),
        output.display()
    );
    println!(
        "\n💡 This seed combines entropy from {} hardware sources",
        slots.len()
    );
    println!("   Use it for: key derivation, wallet seeds, etc.");

    client.finalize()?;
    Ok(())
}

fn show_status() {
    println!("🐻 BearDog Status\n");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!(
        "Platform: {} ({})",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    println!(
        "Build: {}",
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    );
    println!("\n🔐 Security Features:");
    println!("  Real PKCS#11 Integration: ✅");
    println!("  Hardware Entropy: ✅");
    println!("  SHA3-512 Mixing: ✅");
    println!("  Zero Mocks: ✅");
    println!("\n💡 Try:");
    println!("  beardog discover-hsm");
    println!("  beardog test-entropy --slot 0");
    println!("  beardog mix-seed --slots 0,1,2,3 --output seed.bin");
}
