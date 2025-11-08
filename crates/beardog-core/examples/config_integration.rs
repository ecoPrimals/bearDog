//! Configuration system integration example
//!
//! This example demonstrates how to use the beardog-config system
//! with beardog-core components.

use beardog_config::BearDogConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🔧 BearDog Configuration System Integration Example\n");

    // Load configuration with full hierarchy
    println!("Loading configuration...");
    let config = BearDogConfig::load()?;
    
    // Validate configuration
    println!("Validating configuration...");
    config.validate()?;
    
    println!("✅ Configuration loaded and validated successfully!\n");

    // Display network configuration
    println!("📡 Network Configuration:");
    println!("  API:");
    println!("    Bind Address: {}", config.network.api.bind_address);
    println!("    Port: {}", config.network.api.port);
    println!("    TLS Enabled: {}", config.network.api.tls_enabled);
    println!("    Max Connections: {}", config.network.api.max_connections);
    
    println!("\n  Discovery:");
    println!("    Port: {}", config.network.discovery.port);
    println!("    Multicast: {}", config.network.discovery.multicast_address);
    println!("    Backends: {:?}", config.network.discovery.backends);
    println!("    Interval: {}s", config.network.discovery.interval_secs);
    
    println!("\n  Admin:");
    println!("    Bind Address: {}", config.network.admin.bind_address);
    println!("    Port: {}", config.network.admin.port);
    println!("    Enabled: {}", config.network.admin.enabled);

    // Display HSM configuration
    println!("\n🔐 HSM Configuration:");
    println!("  Auto Detect: {}", config.hsm.auto_detect);
    println!("  Prefer Hardware: {}", config.hsm.prefer_hardware);
    println!("  Enabled Providers: {:?}", config.hsm.get_enabled_providers());

    // Display paths
    println!("\n📁 Paths:");
    println!("  Config: {}", config.paths.config_dir.display());
    println!("  Data: {}", config.paths.data_dir.display());
    println!("  Logs: {}", config.paths.log_dir.display());
    
    if let Some(lib) = config.paths.get_pkcs11_library() {
        println!("  PKCS#11: {}", lib.display());
    } else {
        println!("  PKCS#11: (none found)");
    }

    // Display crypto settings
    println!("\n🔒 Cryptographic Settings:");
    println!("  RSA Key Size: {} bits", config.crypto.rsa_key_size);
    println!("  EC Curve: {}", config.crypto.ec_curve);
    println!("  AES Key Size: {} bits", config.crypto.aes_key_size);
    println!("  Hash Algorithm: {}", config.crypto.hash_algorithm);
    println!("  PBKDF2 Iterations: {}", config.crypto.pbkdf2_iterations);

    // Display limits
    println!("\n⏱️  Limits:");
    println!("  Operation Timeout: {}s", config.limits.operation_timeout_secs);
    println!("  Connection Timeout: {}s", config.limits.connection_timeout_secs);
    println!("  Max Retries: {}", config.limits.max_retries);
    println!("  Max Concurrent Ops: {}", config.limits.max_concurrent_operations);

    // Display monitoring
    println!("\n📊 Monitoring:");
    println!("  Log Level: {}", config.monitoring.log_level);
    println!("  Log Format: {}", config.monitoring.log_format);
    println!("  Metrics Port: {}", config.monitoring.metrics_port);
    println!("  Health Port: {}", config.monitoring.health_check_port);

    println!("\n✅ Integration example complete!");
    println!("\nTry setting environment variables to override defaults:");
    println!("  export BEARDOG_API_PORT=9000");
    println!("  export BEARDOG_LOG_LEVEL=debug");
    println!("  cargo run --example config_integration");

    Ok(())
}

