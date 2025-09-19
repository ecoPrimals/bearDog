// Comprehensive Configuration Examples for BearDog
//
// This file demonstrates various configuration patterns and use cases
// for the unified BearDog configuration system.

use beardog_errors::BearDogError;
use beardog_types::canonical::config::{
    AuthMethod, BearDogMasterConfig, CanonicalAppConfig, CanonicalHsmConfig,
    CanonicalMonitoringConfig, CanonicalNetworkConfig, CanonicalSecurityConfig,
    EncryptionAlgorithm, EvictionPolicy, HsmProviderType, LoadBalancingStrategy, LogLevel,
    ProductionEnvironment, SecurityLevel,
};
use std::collections::HashMap;
use std::time::Duration;

/// Example 1: Basic Development Configuration
///
/// This example shows how to create a basic development configuration
/// with minimal security and debugging enabled.
pub fn create_development_config() -> Result<BearDogMasterConfig, BearDogError> {
    let mut config = BearDogMasterConfig::new();

    // Application settings for development
    config.app.name = "BearDog-Dev".to_string();
    config.app.version = "3.0.0-dev".to_string();
    config.app.environment = "development".to_string();
    config.app.debug = true;
    config.app.log_level = LogLevel::Debug;

    // Add development feature flags
    config.app.features.insert("hot_reload".to_string(), true);
    config.app.features.insert("debug_ui".to_string(), true);
    config.app.features.insert("mock_hsm".to_string(), true);

    // Network settings for local development
    config.network.enabled = true;
    config.network.bind_address = "127.0.0.1".to_string();
    config.network.port = 3000;
    config.network.timeout = Duration::from_secs(30);
    config.network.max_connections = 100;
    config.network.tls.enabled = false; // Disabled for local dev

    // Minimal security for development
    config.security.enabled = true;
    config.security.level = SecurityLevel::Low;
    config.security.authentication.enabled = false; // Disabled for dev

    // Basic monitoring
    config.monitoring.enabled = true;
    config.monitoring.metrics.enabled = true;
    config.monitoring.logging.enabled = true;
    config.monitoring.logging.level = LogLevel::Debug;

    // Mock HSM for development
    config.hsm.enabled = true;
    config.hsm.provider.provider_type = HsmProviderType::Software;

    // Development environment settings
    config.production.environment = ProductionEnvironment::Development;
    config.production.health.enabled = true;

    // Validate and return
    config.validate()?;
    Ok(config)
}

/// Example 2: Production Configuration
///
/// This example demonstrates a production-ready configuration with
/// full security, monitoring, and HSM integration.
pub fn create_production_config() -> Result<BearDogMasterConfig, BearDogError> {
    let mut config = BearDogMasterConfig::new();

    // Production application settings
    config.app.name = "BearDog".to_string();
    config.app.version = "3.0.0".to_string();
    config.app.environment = "production".to_string();
    config.app.debug = false;
    config.app.log_level = LogLevel::Info;

    // Production metadata
    config
        .app
        .metadata
        .insert("deployment_id".to_string(), "prod-001".to_string());
    config
        .app
        .metadata
        .insert("region".to_string(), "us-east-1".to_string());
    config
        .app
        .metadata
        .insert("cluster".to_string(), "primary".to_string());

    // Production network settings
    config.network.enabled = true;
    config.network.bind_address = "0.0.0.0".to_string();
    config.network.port = 443;
    config.network.timeout = Duration::from_secs(60);
    config.network.max_connections = 10000;

    // TLS configuration
    config.network.tls.enabled = true;
    config.network.tls.cert_path = Some("/etc/ssl/certs/beardog.crt".to_string());
    config.network.tls.key_path = Some("/etc/ssl/private/beardog.key".to_string());
    config.network.tls.ca_path = Some("/etc/ssl/certs/ca.crt".to_string());

    // Load balancing
    config.network.load_balancing.enabled = true;
    config.network.load_balancing.strategy = LoadBalancingStrategy::LeastConnections;

    // Rate limiting
    config.network.rate_limiting.enabled = true;
    config.network.rate_limiting.requests_per_second = 1000.0;
    config.network.rate_limiting.burst_size = 100;

    // Production security settings
    config.security.enabled = true;
    config.security.level = SecurityLevel::Critical;

    // Authentication
    config.security.authentication.enabled = true;
    config.security.authentication.methods = vec![AuthMethod::Certificate, AuthMethod::Bearer];
    config.security.authentication.token_expiry = Duration::from_secs(3600);
    config.security.authentication.mfa_required = true;

    // Authorization
    config.security.authorization.enabled = true;
    config.security.authorization.rbac_enabled = true;

    // Encryption
    config.security.encryption.enabled = true;
    config.security.encryption.algorithm = EncryptionAlgorithm::Aes256;
    config.security.encryption.key_size = 256;
    config.security.encryption.key_rotation_enabled = true;

    // Session management
    config.security.session.timeout = Duration::from_secs(1800);
    config.security.session.max_sessions = 1000;
    config.security.session.secure_cookies = true;

    // Audit logging
    config.security.audit.enabled = true;
    config.security.audit.log_all_events = true;
    config.security.audit.retention_days = 90;

    // Comprehensive monitoring
    config.monitoring.enabled = true;

    // Metrics
    config.monitoring.metrics.enabled = true;
    config.monitoring.metrics.collection_interval = Duration::from_secs(30);
    config.monitoring.metrics.retention_days = 30;
    config.monitoring.metrics.export_endpoints = vec![
        "https://prometheus.example.com:9090".to_string(),
        "https://grafana.example.com:3000".to_string(),
    ];

    // Logging
    config.monitoring.logging.enabled = true;
    config.monitoring.logging.level = LogLevel::Info;
    config.monitoring.logging.rotation_enabled = true;
    config.monitoring.logging.max_file_size_mb = 100;

    // Alerting
    config.monitoring.alerting.enabled = true;
    config.monitoring.alerting.notification_channels = vec![
        "slack://security-alerts".to_string(),
        "email://ops@example.com".to_string(),
    ];
    config.monitoring.alerting.escalation_enabled = true;

    // Health checks
    config.monitoring.health_checks.enabled = true;
    config.monitoring.health_checks.check_interval = Duration::from_secs(30);
    config.monitoring.health_checks.timeout = Duration::from_secs(10);

    // Hardware HSM configuration
    config.hsm.enabled = true;
    config.hsm.provider.provider_type = HsmProviderType::Hardware;
    config.hsm.provider.endpoint = Some("https://hsm.example.com:443".to_string());
    config.hsm.provider.credentials = Some("/etc/hsm/credentials.json".to_string());

    // HSM connection settings
    config.hsm.connection.timeout = Duration::from_secs(30);
    config.hsm.connection.max_connections = 10;
    config.hsm.connection.retry_attempts = 3;

    // HSM security
    config.hsm.security.authentication_required = true;
    config.hsm.security.access_control_enabled = true;
    config.hsm.security.audit_logging_enabled = true;

    // HSM performance
    config.hsm.performance.batch_operations_enabled = true;
    config.hsm.performance.connection_pooling_enabled = true;
    config.hsm.performance.cache_enabled = true;

    // HSM monitoring
    config.hsm.monitoring.health_checks_enabled = true;
    config.hsm.monitoring.metrics_enabled = true;
    config.hsm.monitoring.alerting_enabled = true;

    // Production environment settings
    config.production.enabled = true;
    config.production.environment = ProductionEnvironment::Production;

    // Production health monitoring
    config.production.health.enabled = true;
    config.production.health.check_interval = Duration::from_secs(30);
    config.production.health.failure_threshold = 3;

    // Production metrics
    config.production.metrics.enabled = true;
    config.production.metrics.collection_interval = Duration::from_secs(15);
    config.production.metrics.aggregation_enabled = true;

    // Observability
    config.production.observability.tracing_enabled = true;
    config.production.observability.distributed_tracing = true;
    config.production.observability.correlation_id_enabled = true;

    // Optimization
    config.production.optimization.auto_scaling_enabled = true;
    config.production.optimization.resource_limits_enabled = true;
    config.production.optimization.performance_tuning = true;

    // Telemetry
    config.production.telemetry.enabled = true;
    config.production.telemetry.collection_interval = Duration::from_secs(60);

    // Performance settings
    config.performance.enabled = true;
    config.performance.caching_enabled = true;
    config.performance.parallel_processing = true;
    config.performance.resource_limits.max_memory_mb = 8192;
    config.performance.resource_limits.max_cpu_cores = 8;
    config.performance.resource_limits.max_connections = 10000;

    // Authentication providers
    config.auth.enabled = true;
    config.auth.session_timeout = Duration::from_secs(3600);
    config.auth.max_login_attempts = 5;

    // Compliance settings
    config.compliance.enabled = true;
    config.compliance.gdpr_enabled = true;
    config.compliance.hipaa_enabled = false;
    config.compliance.pci_dss_enabled = true;
    config.compliance.data_retention_days = 2555; // 7 years

    // Cache configuration
    config.cache.enabled = true;
    config.cache.max_size_mb = 1024;
    config.cache.ttl = Duration::from_secs(3600);
    config.cache.eviction_policy = EvictionPolicy::Lru;

    // Database configuration
    config.database.enabled = true;
    config.database.connection_string =
        "postgresql://beardog:***@db.example.com:5432/beardog".to_string();
    config.database.max_connections = 100;
    config.database.connection_timeout = Duration::from_secs(30);
    config.database.query_timeout = Duration::from_secs(60);

    // Validate the entire configuration
    config.validate()?;
    Ok(config)
}

/// Example 3: Environment-based Configuration Loading
///
/// This example shows how to load configuration from environment variables
/// and merge with defaults.
pub fn create_environment_config() -> Result<BearDogMasterConfig, BearDogError> {
    // Start with base configuration
    let mut base_config = create_development_config()?;

    // Load environment-specific overrides
    let env_config = BearDogMasterConfig::from_env()?;

    // Merge environment config with base
    base_config.merge(env_config)?;

    // Validate merged configuration
    base_config.validate()?;
    Ok(base_config)
}

/// Example 4: Microservice Configuration
///
/// This example shows how to configure a microservice with specific
/// requirements for a distributed architecture.
pub fn create_microservice_config(service_name: &str) -> Result<BearDogMasterConfig, BearDogError> {
    let mut config = BearDogMasterConfig::new();

    // Service-specific application settings
    config.app.name = format!("BearDog-{}", service_name);
    config.app.version = "3.0.0".to_string();
    config.app.environment = "production".to_string();
    config.app.debug = false;
    config.app.log_level = LogLevel::Info;

    // Service metadata
    config
        .app
        .metadata
        .insert("service_type".to_string(), "microservice".to_string());
    config
        .app
        .metadata
        .insert("service_name".to_string(), service_name.to_string());

    // Network settings optimized for microservices
    config.network.enabled = true;
    config.network.bind_address = "0.0.0.0".to_string();
    config.network.port = 8080; // Standard microservice port
    config.network.timeout = Duration::from_secs(30);
    config.network.max_connections = 1000;

    // Circuit breaker for resilience
    config.network.circuit_breaker.enabled = true;
    config.network.circuit_breaker.failure_threshold = 5;
    config.network.circuit_breaker.timeout = Duration::from_secs(60);

    // Security appropriate for internal services
    config.security.enabled = true;
    config.security.level = SecurityLevel::High;
    config.security.authentication.enabled = true;
    config.security.authentication.methods = vec![AuthMethod::Bearer];

    // Monitoring for observability
    config.monitoring.enabled = true;
    config.monitoring.metrics.enabled = true;
    config.monitoring.logging.enabled = true;
    config.monitoring.health_checks.enabled = true;

    // Service mesh integration
    config.monitoring.metrics.export_endpoints = vec!["http://prometheus-service:9090".to_string()];

    // Performance tuning for microservices
    config.performance.enabled = true;
    config.performance.resource_limits.max_memory_mb = 512; // Conservative for microservices
    config.performance.resource_limits.max_cpu_cores = 2;
    config.performance.resource_limits.max_connections = 1000;

    // Cache for performance
    config.cache.enabled = true;
    config.cache.max_size_mb = 64;
    config.cache.ttl = Duration::from_secs(300);

    config.validate()?;
    Ok(config)
}

/// Example 5: Configuration Validation and Error Handling
///
/// This example demonstrates comprehensive configuration validation
/// and error handling patterns.
pub fn validate_configuration_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Configuration Validation Examples ===");

    // Example 1: Valid configuration
    println!("1. Testing valid configuration...");
    let valid_config = create_development_config()?;
    match valid_config.validate() {
        Ok(()) => println!("   [OK] Configuration is valid"),
        Err(e) => println!("   [X] Unexpected validation error: {}", e),
    }

    // Example 2: Invalid network configuration
    println!("2. Testing invalid network configuration...");
    let mut invalid_network = BearDogMasterConfig::new();
    invalid_network.network.enabled = true;
    invalid_network.network.port = 0; // Invalid port

    match invalid_network.validate() {
        Ok(()) => println!("   [X] Should have failed validation"),
        Err(e) => println!("   [OK] Caught validation error: {}", e),
    }

    // Example 3: Production security requirements
    println!("3. Testing production security requirements...");
    let mut prod_config = BearDogMasterConfig::new();
    prod_config.production.environment = ProductionEnvironment::Production;
    prod_config.security.enabled = false; // Invalid for production

    match prod_config.validate() {
        Ok(()) => println!("   [X] Should have failed validation"),
        Err(e) => println!("   [OK] Caught validation error: {}", e),
    }

    // Example 4: HSM configuration validation
    println!("4. Testing HSM configuration validation...");
    let mut hsm_config = BearDogMasterConfig::new();
    hsm_config.hsm.enabled = true;
    hsm_config.hsm.provider.provider_type = HsmProviderType::Hardware;
    // Missing endpoint - should fail validation

    match hsm_config.validate() {
        Ok(()) => println!("   [X] Should have failed validation"),
        Err(e) => println!("   [OK] Caught validation error: {}", e),
    }

    Ok(())
}

/// Example 6: Configuration Serialization and Persistence
///
/// This example shows how to save and load configurations from files.
pub fn configuration_persistence_examples() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    println!("=== Configuration Persistence Examples ===");

    // Create a sample configuration
    let config = create_production_config()?;

    // Save as JSON
    println!("1. Saving configuration as JSON...");
    let json = serde_json::to_string_pretty(&config)?;
    fs::write("config.json", &json)?;
    println!("   [OK] Saved to config.json");

    // Load from JSON
    println!("2. Loading configuration from JSON...");
    let loaded_json = fs::read_to_string("config.json")?;
    let loaded_config: BearDogMasterConfig = serde_json::from_str(&loaded_json)?;
    loaded_config.validate()?;
    println!("   [OK] Loaded and validated from config.json");

    // Configuration summary
    println!("3. Configuration summary:");
    let summary = loaded_config.summary();
    println!("   Environment: {}", summary.environment);
    println!("   Debug Mode: {}", summary.debug_mode);
    println!("   Network Enabled: {}", summary.network_enabled);
    println!("   Security Level: {}", summary.security_level);
    println!("   HSM Enabled: {}", summary.hsm_enabled);
    println!("   Production Mode: {}", summary.production_mode);
    println!("   Total Domains: {}", summary.total_domains);

    // Cleanup
    fs::remove_file("config.json").ok();

    Ok(())
}

/// Example 7: Configuration Templates
///
/// This example provides template functions for common deployment scenarios.
pub mod templates {
    use super::*;

    /// Template for single-node deployment
    pub fn single_node() -> Result<BearDogMasterConfig, BearDogError> {
        let mut config = BearDogMasterConfig::new();

        config.app.name = "BearDog-SingleNode".to_string();
        config.app.environment = "production".to_string();

        // Single node network settings
        config.network.enabled = true;
        config.network.port = 8080;
        config.network.max_connections = 1000;

        // Moderate security
        config.security.enabled = true;
        config.security.level = SecurityLevel::High;

        // Basic monitoring
        config.monitoring.enabled = true;
        config.monitoring.metrics.enabled = true;
        config.monitoring.logging.enabled = true;

        // Software HSM for simplicity
        config.hsm.enabled = true;
        config.hsm.provider.provider_type = HsmProviderType::Software;

        config.validate()?;
        Ok(config)
    }

    /// Template for high-availability cluster
    pub fn high_availability_cluster() -> Result<BearDogMasterConfig, BearDogError> {
        let mut config = create_production_config()?;

        // HA-specific settings
        config.app.name = "BearDog-HA-Cluster".to_string();
        config
            .app
            .metadata
            .insert("deployment_mode".to_string(), "ha_cluster".to_string());

        // Enhanced load balancing
        config.network.load_balancing.enabled = true;
        config.network.load_balancing.strategy = LoadBalancingStrategy::LeastConnections;

        // Multiple HSM for redundancy
        config.hsm.performance.connection_pooling_enabled = true;
        config.hsm.connection.max_connections = 20;

        // Enhanced monitoring for cluster
        config.monitoring.alerting.escalation_enabled = true;

        config.validate()?;
        Ok(config)
    }

    /// Template for development environment
    pub fn development() -> Result<BearDogMasterConfig, BearDogError> {
        create_development_config()
    }

    /// Template for testing environment
    pub fn testing() -> Result<BearDogMasterConfig, BearDogError> {
        let mut config = create_development_config()?;

        config.app.name = "BearDog-Test".to_string();
        config.app.environment = "testing".to_string();

        // Test-specific features
        config.app.features.insert("test_mode".to_string(), true);
        config
            .app
            .features
            .insert("mock_external_services".to_string(), true);

        // Faster timeouts for testing
        config.network.timeout = Duration::from_secs(5);
        config.hsm.connection.timeout = Duration::from_secs(5);

        config.validate()?;
        Ok(config)
    }
}

/// Main function demonstrating all examples
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🐻🐕 BearDog Configuration Examples");

    // Run validation examples
    validate_configuration_examples()?;

    println!("" + "=".repeat(50).as_str() + "");

    // Run persistence examples
    configuration_persistence_examples()?;

    println!("" + "=".repeat(50).as_str() + "");

    // Test templates
    println!("=== Configuration Templates ===");

    let single_node = templates::single_node()?;
    println!(
        "[OK] Single node template: {} domains configured",
        single_node.summary().total_domains
    );

    let ha_cluster = templates::high_availability_cluster()?;
    println!(
        "[OK] HA cluster template: {} domains configured",
        ha_cluster.summary().total_domains
    );

    let dev_config = templates::development()?;
    println!(
        "[OK] Development template: {} domains configured",
        dev_config.summary().total_domains
    );

    let test_config = templates::testing()?;
    println!(
        "[OK] Testing template: {} domains configured",
        test_config.summary().total_domains
    );

    println!("[PARTY] All configuration examples completed successfully!");

    Ok(())
}
