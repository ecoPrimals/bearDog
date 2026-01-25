//! Client Mode - Interactive BearDog client
//!
//! Connect to BearDog server and perform operations.

use beardog_core::socket_config::SocketConfig;
use tracing::info;

/// Run BearDog in client mode
///
/// Interactive client for connecting to BearDog server.
/// Uses intelligent socket discovery following Primal IPC Protocol.
pub async fn run(endpoint: Option<String>, command: Option<String>) -> anyhow::Result<()> {
    info!("🐻 BearDog Client v{}", env!("CARGO_PKG_VERSION"));
    
    // Discover server endpoint using same logic as server
    let endpoint = endpoint.unwrap_or_else(|| {
        let config = SocketConfig::from_env();
        let path = format!("unix://{}", config.socket_path_string());
        info!("Auto-discovered endpoint: {}", config.description());
        path
    });
    
    info!("Endpoint: {}\n", endpoint);

    if let Some(cmd) = command {
        // Execute single command
        info!("Executing command: {}", cmd);
        println!("Command execution not yet implemented");
        println!("Coming soon: JSON-RPC client for BearDog operations");
    } else {
        // Interactive mode
        println!("╔════════════════════════════════════════════════════════════════════╗");
        println!("║                                                                    ║");
        println!("║              🐻 BearDog Interactive Client 🐻                     ║");
        println!("║                                                                    ║");
        println!("╚════════════════════════════════════════════════════════════════════╝\n");

        println!("Endpoint: {}", endpoint);
        println!("\nAvailable commands:");
        println!("  • tunnel establish <peer>   - Establish BTSP tunnel");
        println!("  • tunnel status <id>        - Check tunnel status");
        println!("  • tunnel close <id>         - Close tunnel");
        println!("  • health                    - Server health check");
        println!("  • help                      - Show this help");
        println!("  • quit                      - Exit client\n");

        println!("⚠️  Interactive mode coming soon!");
        println!("Use Unix socket JSON-RPC for now (see examples/btsp_unix_socket_client.rs)\n");
    }

    Ok(())
}
