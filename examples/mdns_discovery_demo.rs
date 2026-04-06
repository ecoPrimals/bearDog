// SPDX-License-Identifier: AGPL-3.0-or-later
//! mDNS Discovery Demo
//!
//! Demonstrates runtime primal discovery using mDNS/DNS-SD.
//! Zero hardcoded addresses - pure capability-based discovery.
//!
//! # Usage
//!
//! Terminal 1 (Announce service):
//! ```bash
//! cargo run --example mdns_discovery_demo --features mdns -- announce
//! ```
//!
//! Terminal 2 (Discover services):
//! ```bash
//! cargo run --example mdns_discovery_demo --features mdns -- discover
//! ```
//!
//! Without mdns feature:
//! ```bash
//! cargo run --example mdns_discovery_demo
//! # Gracefully shows that mDNS is not enabled
//! ```

use beardog_core::primal_self_knowledge::{PrimalDiscovery, PrimalIdentity};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map_or("discover", std::string::String::as_str);

    match mode {
        "announce" => {
            println!("📢 BearDog mDNS Announcement Demo");
            println!("==================================\n");

            announce_service().await?;

            println!("\n✅ Service announced. Press Ctrl+C to stop.");
            println!("   Other primals can now discover this service.\n");

            // Keep running
            tokio::signal::ctrl_c().await?;
            println!("\n👋 Shutting down...");
        }

        "discover" => {
            println!("🔍 BearDog mDNS Discovery Demo");
            println!("==============================\n");

            discover_primals().await?;
        }

        _ => {
            println!("Usage: mdns_discovery_demo [announce|discover]");
            println!();
            println!("announce  - Announce this primal on mDNS");
            println!("discover  - Discover other primals via mDNS");
        }
    }

    Ok(())
}

/// Announce this primal's services via mDNS
async fn announce_service() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "mdns")]
    {
        use beardog_core::primal_discovery_mdns::MdnsServiceAnnouncer;

        println!("Setting up mDNS announcement...");

        // Create announcer with capabilities
        let announcer = MdnsServiceAnnouncer::new(
            8080,
            vec!["crypto".to_string(), "hsm".to_string(), "audit".to_string()],
        );

        // Announce
        announcer.announce()?;

        println!("\n✅ mDNS service announced!");
        println!("   Service Type: _beardog._tcp.local.");
        println!("   Port: 8080");
        println!("   Capabilities: crypto, hsm, audit");
    }

    #[cfg(not(feature = "mdns"))]
    {
        println!("⚠️  mDNS feature not enabled.");
        println!("    To enable mDNS discovery:");
        println!("    cargo run --example mdns_discovery_demo --features mdns -- announce");
        println!();
        println!("    Without mDNS, primals can still be discovered via:");
        println!("    - HTTP API endpoints (/api/v1/capabilities)");
        println!("    - Service registry (if configured)");
        println!("    - Manual configuration");
    }

    Ok(())
}

/// Discover primals via mDNS
async fn discover_primals() -> Result<(), Box<dyn std::error::Error>> {
    println!("1️⃣ Self-Identity Discovery\n");

    // First, establish our own identity (self-knowledge)
    let identity = PrimalIdentity::from_environment()?;
    println!("✅ I am: {}", identity.name);
    println!("   Type: {}", identity.primal_type);
    println!("   Capabilities: {:?}", identity.capabilities);
    println!();

    println!("2️⃣ Discovering Other Primals\n");

    // Create discovery service
    let discovery = PrimalDiscovery::new(identity);

    // Discover primals with specific capabilities
    let capabilities_to_find = vec!["crypto", "hsm", "storage"];

    for capability in capabilities_to_find {
        println!("🔍 Searching for primals with '{capability}' capability...");

        match discovery.discover_by_capability(capability).await {
            Ok(primals) => {
                if primals.is_empty() {
                    println!("   No primals found with '{capability}' capability");
                } else {
                    println!("   Found {} primal(s):", primals.len());
                    for primal in primals {
                        println!("   ✅ {}", primal.name);
                        println!("      Type: {}", primal.primal_type);
                        println!("      Endpoints: {:?}", primal.endpoints);
                        println!("      Capabilities: {:?}", primal.capabilities);
                        println!("      Discovered: {:?}", primal.discovered_at);
                        println!();
                    }
                }
            }
            Err(e) => {
                println!("   ⚠️  Discovery failed: {e}");
            }
        }
        println!();
    }

    #[cfg(not(feature = "mdns"))]
    {
        println!("ℹ️  mDNS feature not enabled.");
        println!("   To enable mDNS discovery:");
        println!("   cargo run --example mdns_discovery_demo --features mdns -- discover");
        println!();
        println!("   Without mDNS, primals can still be discovered via:");
        println!("   - Service registry (set BEARDOG_SERVICE_REGISTRY_URL)");
        println!("   - HTTP API queries");
        println!("   - Manual configuration");
    }

    Ok(())
}
