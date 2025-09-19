use beardog_errors::BearDogError;
use beardog_types::canonical::config::CanonicalAppConfig;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🚀 BearDog Configuration Manager Demo");

    // Demo 1: Basic Configuration
    demo_basic_configuration()?;

    // Demo 2: Environment Configuration
    demo_environment_configuration()?;

    // Demo 3: Dynamic Updates
    demo_dynamic_updates()?;

    // Demo 4: Configuration Validation
    demo_configuration_validation()?;

    // Demo 5: Service Integration
    demo_service_integration()?;

    info!("✅ Configuration Manager Demo Complete!");
    Ok(())
}

async fn demo_basic_configuration() -> Result<(), BearDogError> {
    info!("📋 Demo 1: Basic Configuration");

    let config = CanonicalAppConfig::default();

    info!("   Version: {}", config.version);
    info!("   Environment: {}", config.environment);
    info!(
        "   Health Check Interval: {}s",
        config.monitoring.health_checks.interval_seconds
    );

    Ok(())
}

async fn demo_environment_configuration() -> Result<(), BearDogError> {
    info!("🌍 Demo 2: Environment Configuration");

    // Set environment variables
    std::env::set_var("BEARDOG_VERSION", "3.0.0");
    std::env::set_var("BEARDOG_ENVIRONMENT", "production");

    let config = CanonicalAppConfig::from_env()?;

    info!("   Version from env: {}", config.version);
    info!("   Environment from env: {}", config.environment);

    Ok(())
}

async fn demo_dynamic_updates() -> Result<(), BearDogError> {
    info!("🔄 Demo 3: Dynamic Configuration Updates");

    let mut config = CanonicalAppConfig::default();

    // Simulate runtime configuration update
    config.monitoring.enabled = true;
    config.monitoring.metrics.enabled = true;

    info!("   Updated monitoring configuration");
    info!("   Monitoring enabled: {}", config.monitoring.enabled);
    info!("   Metrics enabled: {}", config.monitoring.metrics.enabled);

    Ok(())
}

async fn demo_configuration_validation() -> Result<(), BearDogError> {
    info!("✅ Demo 4: Configuration Validation");

    let config = CanonicalAppConfig::default();
    let validation_result = config.validate();

    match validation_result {
        Ok(_) => info!("   Configuration validation: PASSED"),
        Err(e) => info!("   Configuration validation: FAILED - {}", e),
    }

    Ok(())
}

async fn demo_service_integration() -> Result<(), BearDogError> {
    info!("🔗 Demo 5: Service Integration");

    let config = CanonicalAppConfig::default();

    // Display service configuration
    if config.genetics.enabled {
        info!("   [OK] Genetic Optimization: ENABLED");
    } else {
        info!("   ⏸️  Genetic Optimization: DISABLED");
    }

    if config.threat_detection.enabled {
        info!("   [OK] Threat Detection: ENABLED");
    }

    // Display security configuration
    info!("   Security Configuration:");
    info!("   - Encryption: {}", config.security.encryption.algorithm);
    if config.security.distributed_mode {
        info!("   [OK] Distributed Mode: ENABLED");
    }

    info!(
        "   Session Timeout: {}s",
        config.security.session_timeout_seconds
    );
    info!(
        "   Key Rotation Interval: {}s",
        config.security.key_rotation_interval_seconds
    );
    info!(
        "   Encryption Key Size: {} bytes",
        config.security.encryption.key_size_bytes
    );

    Ok(())
}
