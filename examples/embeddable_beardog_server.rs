//! Embeddable BearDog Server Example
//!
//! This example demonstrates how to embed BearDog into your application
//! (e.g., biomeOS towers) with proper HSM initialization and lineage support.
//!
//! # Features
//!
//! - Pure Rust software HSM (no external dependencies)
//! - Automatic HSM provider registration via `auto_initialize()`
//! - Environment-driven configuration (zero hardcoding)
//! - Genetic lineage support for family-based trust
//! - Unix socket IPC + HTTP API
//!
//! # Usage
//!
//! ```bash
//! # Set environment variables
//! export BEARDOG_HSM_MODE=software
//! export BEARDOG_BIND_ADDR=127.0.0.1:9000
//! export BEARDOG_FAMILY_SEED=$(cat /path/to/usb/seed.txt)  # Optional
//!
//! # Run the server
//! cargo run --example embeddable_beardog_server
//! ```
//!
//! # Integration with biomeOS
//!
//! biomeOS towers can embed this pattern directly:
//!
//! ```rust,ignore
//! use beardog_tunnel::{BeardogBtspProvider, HsmManager};
//! use beardog_genetics::EcosystemGeneticEngine;
//! use beardog_tunnel::api::BearDogApiServer;
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 1. Auto-initialize HSM (reads BEARDOG_HSM_MODE env var)
//!     let hsm = Arc::new(HsmManager::auto_initialize().await?);
//!
//!     // 2. Initialize genetic engine
//!     let genetics = Arc::new(EcosystemGeneticEngine::new()?);
//!
//!     // 3. Create BTSP provider
//!     let btsp_provider = Arc::new(
//!         BeardogBtspProvider::new(hsm, genetics).await?
//!     );
//!
//!     // 4. Create API server
//!     let config = BearDogApiServerConfig::default();  // Reads from env
//!     let server = BearDogApiServer::new(config, btsp_provider).await?;
//!
//!     // 5. Serve
//!     server.serve().await?;
//!     Ok(())
//! }
//! ```

use beardog_genetics::EcosystemGeneticEngine;
use beardog_tunnel::api::{BearDogApiServer, BearDogApiServerConfig};
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::HsmManager;
use std::sync::Arc;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();

    info!("╔════════════════════════════════════════════════════════════════════╗");
    info!("║                                                                    ║");
    info!("║         🐻 BearDog Embeddable Server Example                       ║");
    info!("║                                                                    ║");
    info!("║              For biomeOS and Other Sovereign Systems               ║");
    info!("║                                                                    ║");
    info!("╚════════════════════════════════════════════════════════════════════╝");
    info!("");

    // Step 1: Auto-initialize HSM Manager
    // This reads BEARDOG_HSM_MODE from environment and registers the appropriate provider
    info!("🔐 Step 1: Initializing HSM Manager...");
    info!("   Reading BEARDOG_HSM_MODE from environment");
    info!("   Supported modes: software (pure Rust), hardware, android, ios");
    
    let hsm = Arc::new(HsmManager::auto_initialize().await?);
    info!("✅ HSM Manager initialized successfully");
    info!("");

    // Step 2: Initialize Genetic Engine
    info!("🧬 Step 2: Initializing Genetic Engine...");
    let genetics = Arc::new(EcosystemGeneticEngine::new()?);
    info!("✅ Genetic Engine initialized");
    info!("");

    // Step 3: Create BTSP Provider
    info!("🛡️  Step 3: Creating BTSP Provider...");
    info!("   This integrates HSM + Genetics for secure tunnels");
    let btsp_provider = Arc::new(BeardogBtspProvider::new(hsm, genetics).await?);
    info!("✅ BTSP Provider created");
    info!("");

    // Step 4: Create API Server
    info!("🚀 Step 4: Creating API Server...");
    let config = BearDogApiServerConfig::default(); // Reads from environment
    info!("   Bind address: {}", config.bind_addr);
    info!("   CORS enabled: {}", config.enable_cors);
    info!("   Version: {}", config.version);
    
    let server = BearDogApiServer::new(config.clone(), btsp_provider).await?;
    info!("✅ API Server created");
    info!("");

    // Step 5: Display family lineage info (if USB seed present)
    if let Ok(family_seed) = std::env::var("BEARDOG_FAMILY_SEED") {
        let family_id: String = family_seed
            .chars()
            .filter(|c| c.is_alphanumeric())
            .take(4)
            .collect();
        info!("👨‍👩‍👧‍👦 Family Lineage Detected:");
        info!("   Family ID: {}", family_id.to_lowercase());
        info!("   Genetic siblings will auto-trust this family");
        info!("");
    } else {
        info!("ℹ️  No family seed detected (BEARDOG_FAMILY_SEED not set)");
        info!("   Running in standalone mode");
        info!("");
    }

    // Step 6: Serve
    info!("╔════════════════════════════════════════════════════════════════════╗");
    info!("║                                                                    ║");
    info!("║         ✅ BearDog Server Ready!                                   ║");
    info!("║                                                                    ║");
    info!("║  HTTP API: http://{}                              ", config.bind_addr);
    info!("║                                                                    ║");
    info!("║  Try:                                                              ║");
    info!("║    curl http://{}/health                         ", config.bind_addr);
    info!("║    curl http://{}/api/v1/lineage/create          ", config.bind_addr);
    info!("║                                                                    ║");
    info!("╚════════════════════════════════════════════════════════════════════╝");
    info!("");

    server.serve().await?;

    Ok(())
}

