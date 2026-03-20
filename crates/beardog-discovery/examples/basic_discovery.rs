// SPDX-License-Identifier: AGPL-3.0-only
//! Basic usage example for beardog-discovery
//!
//! This demonstrates capability-based service discovery

use beardog_discovery::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🐻 BearDog Capability-Based Discovery Example\n");

    // Load configuration (path relative to this crate, not process CWD)
    let config_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../configs/beardog-primal-capabilities.toml");
    println!("Loading configuration from: {}", config_path.display());

    let discovery = CapabilityDiscovery::from_config(config_path).await?;
    println!("✅ Discovery initialized\n");

    // Example 1: Discover orchestration capability
    println!("🔍 Discovering services with 'orchestration' capability...");
    let orchestrators = discovery.find_by_capability("orchestration").await?;

    if orchestrators.is_empty() {
        println!("   No orchestrators found (try setting CAPABILITY_ORCHESTRATION_ENDPOINT)");
        println!("   Example: export CAPABILITY_ORCHESTRATION_ENDPOINT=\"http://localhost:8080\"");
    } else {
        println!("   Found {} orchestrator(s):", orchestrators.len());
        for service in &orchestrators {
            println!(
                "     - {} at {}",
                service.display_name, service.endpoint.primary_url
            );
        }

        // Select best service based on QoS
        if let Some(best) = discovery.select_best(&orchestrators) {
            println!(
                "\n   ✅ Best service: {} (score based on QoS metrics)",
                best.display_name
            );
        }
    }

    println!("\n---\n");

    // Example 2: Discover ANY primal announcing itself
    println!("🔍 Discovering all services via environment...");
    println!("   Set PRIMAL_*_ENDPOINT and PRIMAL_*_CAPABILITIES to announce services");
    println!("   Example:");
    println!("     export PRIMAL_MYORCH_ENDPOINT=\"http://localhost:8080\"");
    println!("     export PRIMAL_MYORCH_CAPABILITIES=\"orchestration,coordination\"");

    println!("\n✅ Discovery example complete!");
    println!("\nKey Points:");
    println!("  1. BearDog discovers services by CAPABILITY, not by a fixed peer name");
    println!("  2. Any service can provide 'orchestration' if it advertises the capability");
    println!("  3. Services self-announce via environment/mDNS/service registry");
    println!("  4. Zero hardcoded knowledge - pure runtime discovery");

    Ok(())
}
