//! BearDog Unified API Server with UPA Registration
//!
//! Demonstrates:
//! - Starting the unified API server
//! - Registering with Songbird's Universal Port Authority (UPA)
//! - Maintaining heartbeat for service discovery
//! - Exposing BTSP, Genesis, BirdSong, and Lineage capabilities

use std::sync::Arc;

use beardog_genetics::EcosystemGeneticEngine;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use beardog_tunnel::{
    BearDogApiServer, BearDogApiServerConfig, BeardogBtspProvider, UpaClient, UpaClientConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("╔══════════════════════════════════════════════════════════════════════════╗");
    println!("║                                                                          ║");
    println!("║        🐻 BearDog Unified API Server with UPA Integration 🐻          ║");
    println!("║                                                                          ║");
    println!("╚══════════════════════════════════════════════════════════════════════════╝");
    println!();

    // Step 1: Initialize HSM Manager
    println!("📦 Initializing HSM Manager...");
    let hsm_manager = Arc::new(HsmManager::new());

    // Step 2: Initialize Genetic Engine
    println!("🧬 Initializing Ecosystem Genetic Engine...");
    let genetic_engine = Arc::new(EcosystemGeneticEngine::new()?);

    // Step 3: Initialize BTSP Provider
    println!("🔒 Initializing BTSP Provider...");
    let btsp_provider = Arc::new(
        BeardogBtspProvider::new(Arc::clone(&hsm_manager), Arc::clone(&genetic_engine)).await?,
    );

    // Step 4: Create API Server
    println!("🚀 Creating Unified API Server...");
    let api_config = BearDogApiServerConfig {
        bind_addr: "127.0.0.1:9000".parse()?,
        enable_cors: true,
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    let api_server = BearDogApiServer::new(api_config.clone(), Arc::clone(&btsp_provider)).await?;

    // Step 5: Create UPA Client
    println!("📡 Creating UPA Client...");
    let upa_config = UpaClientConfig {
        upa_base_url: "https://localhost:8080".to_string(),
        service_name: "beardog".to_string(),
        service_version: env!("CARGO_PKG_VERSION").to_string(),
        api_bind_addr: format!("https://{}", api_config.bind_addr),
        heartbeat_interval_secs: 30,
        connection_timeout_secs: 10,
    };

    let upa_client = UpaClient::new(upa_config)?;

    // Step 6: Register with Songbird UPA (optional - Songbird may not be running)
    println!("🎵 Attempting UPA registration with Songbird...");
    match upa_client.register().await {
        Ok(service_id) => {
            println!("✅ Registered with UPA: service_id={}", service_id);
            println!("🔁 Starting heartbeat loop...");
            upa_client.start_heartbeat();
        }
        Err(e) => {
            println!("⚠️  UPA registration failed (Songbird may not be running): {}", e);
            println!("   API server will continue without UPA registration.");
        }
    }

    // Step 7: Start API Server
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎉 BearDog API Server Starting!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("Capabilities:");
    println!("  🔒 BTSP   - Secure Tunnels       → http://{}/btsp/*", api_config.bind_addr);
    println!("  🔐 Genesis - Physical Bootstrap  → http://{}/genesis/*", api_config.bind_addr);
    println!("  🎵 BirdSong - Encrypted Broadcasts → http://{}/birdsong/*", api_config.bind_addr);
    println!("  🧬 Lineage - Cryptographic Proofs → http://{}/lineage/*", api_config.bind_addr);
    println!();
    println!("Health Check: http://{}/health", api_config.bind_addr);
    println!();

    if upa_client.is_registered() {
        println!("🎵 Multi-Primal Status:");
        println!("  ✅ Registered with Songbird UPA");
        println!("  🔁 Heartbeat active (30s interval)");
        println!();
    }

    println!("Press Ctrl+C to stop the server");
    println!();

    // Start server (blocks until shutdown)
    api_server.serve().await?;

    // Cleanup
    println!();
    println!("🛑 Shutting down...");
    upa_client.stop_heartbeat().await;
    println!("✅ Cleanup complete");

    Ok(())
}

