// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Configuration Manager Demo
//!
//! Demonstrates how to use the BearDog Configuration Manager to eliminate
//! hardcoded values throughout the codebase.

use beardog_types::config::{ConfigManager, ConfigSource, NewBearDogConfig};
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::info;

#[tokio::main]
async fn main() -> BearDogResult<()> {
    info!("🔧 BearDog Configuration Manager Demo");

    // 1. Basic Configuration Manager Usage
    demo_basic_usage().await?;

    // 2. Environment Variable Configuration
    demo_environment_config().await?;

    // 3. File-Based Configuration
    demo_file_config().await?;

    // 4. Runtime Configuration Updates
    demo_runtime_updates().await?;

    // 5. Integration with Services
    demo_service_integration().await?;

    info!("✅ Configuration Manager Demo completed successfully!");
    Ok(())
}

/// Demonstrate basic configuration manager usage
async fn demo_basic_usage() -> BearDogResult<()> {
    info!("📝 Demo 1: Basic Configuration Manager Usage");

    // Create a configuration manager
    let mut config_manager = ConfigManager::new();
    
    // Load configuration from default sources
    config_manager.load().await?;
    
    // Get current configuration
    let config = config_manager.get_config();
    
    info!("🌐 Network Configuration:");
    info!("   API Bind Address: {}", config.network.api_bind_address);
    info!("   API Port: {}", config.network.ports.api);
    info!("   Metrics Port: {}", config.network.ports.metrics);
    info!("   Health Port: {}", config.network.ports.health);
    
    info!("🔐 Security Configuration:");
    info!("   Session Timeout: {}s", config.security.session_timeout_secs);
    info!("   Max Failed Attempts: {}", config.security.max_failed_attempts);
    info!("   Key Cache Size: {}", config.security.key_cache_size);
    
    info!("⚡ Performance Configuration:");
    info!("   Max Connections: {}", config.performance.max_connections);
    info!("   Worker Threads: {}", config.performance.worker_threads);
    info!("   Rate Limit: {}/min", config.performance.rate_limit_per_minute);

    Ok(())
}

/// Demonstrate environment variable configuration
async fn demo_environment_config() -> BearDogResult<()> {
    info!("🌍 Demo 2: Environment Variable Configuration");

    // Set some environment variables
    std::env::set_var("BEARDOG_API_PORT", "9080");
    std::env::set_var("BEARDOG_METRICS_PORT", "9190");
    std::env::set_var("SONGBIRD_ENDPOINT", "https://songbird-dev.beardog.local:8443");
    std::env::set_var("BEARDOG_SESSION_TIMEOUT", "7200");

    // Create and load configuration
    let mut config_manager = ConfigManager::new();
    config_manager.load().await?;
    
    let config = config_manager.get_config();
    
    info!("📊 Configuration loaded from environment:");
    info!("   API Port: {} (from BEARDOG_API_PORT)", config.network.ports.api);
    info!("   Metrics Port: {} (from BEARDOG_METRICS_PORT)", config.network.ports.metrics);
    info!("   SongBird Endpoint: {} (from SONGBIRD_ENDPOINT)", config.external_services.songbird_endpoint);
    info!("   Session Timeout: {}s (from BEARDOG_SESSION_TIMEOUT)", config.security.session_timeout_secs);

    // Clean up environment variables
    std::env::remove_var("BEARDOG_API_PORT");
    std::env::remove_var("BEARDOG_METRICS_PORT");
    std::env::remove_var("SONGBIRD_ENDPOINT");
    std::env::remove_var("BEARDOG_SESSION_TIMEOUT");

    Ok(())
}

/// Demonstrate file-based configuration
async fn demo_file_config() -> BearDogResult<()> {
    info!("📄 Demo 3: File-Based Configuration");

    // Create a temporary configuration file
    let config_content = r#"
[network]
api_bind_address = "127.0.0.1:8081"
metrics_bind_address = "127.0.0.1:9091"

[network.ports]
api = 8081
metrics = 9091
health = 8082
admin = 9998

[security]
session_timeout_secs = 1800
max_failed_attempts = 3
key_cache_size = 2000

[performance]
max_connections = 200
worker_threads = 8
rate_limit_per_minute = 120

[external_services]
songbird_endpoint = "https://songbird-prod.beardog.local:8443"
nestgate_endpoint = "https://nestgate-prod.beardog.local:8443"

[features]
enable_genetic_optimization = false
enable_threat_detection = true
enable_distributed_mode = true
"#;

    // Write to a temporary file
    let temp_file = "/tmp/beardog-demo-config.toml";
    tokio::fs::write(temp_file, config_content).await?;

    // Create configuration manager with file source
    let mut config_manager = ConfigManager::new();
    config_manager.add_source(ConfigSource::File(temp_file.to_string()));
    config_manager.load().await?;
    
    let config = config_manager.get_config();
    
    info!("📋 Configuration loaded from file:");
    info!("   API Bind Address: {}", config.network.api_bind_address);
    info!("   API Port: {}", config.network.ports.api);
    info!("   Session Timeout: {}s", config.security.session_timeout_secs);
    info!("   Max Connections: {}", config.performance.max_connections);
    info!("   Worker Threads: {}", config.performance.worker_threads);
    info!("   SongBird Endpoint: {}", config.external_services.songbird_endpoint);
    info!("   Genetic Optimization: {}", config.features.enable_genetic_optimization);
    info!("   Distributed Mode: {}", config.features.enable_distributed_mode);

    // Clean up
    tokio::fs::remove_file(temp_file).await.ok();

    Ok(())
}

/// Demonstrate runtime configuration updates
async fn demo_runtime_updates() -> BearDogResult<()> {
    info!("⚡ Demo 4: Runtime Configuration Updates");

    let config_manager = ConfigManager::new();
    
    // Show initial configuration
    let initial_config = config_manager.get_config();
    info!("🔧 Initial API Port: {}", initial_config.network.ports.api);
    info!("🔧 Initial Max Connections: {}", initial_config.performance.max_connections);

    // Update configuration at runtime
    config_manager.update_config(|config| {
        config.network.ports.api = 8085;
        config.performance.max_connections = 500;
        config.features.enable_distributed_mode = true;
        Ok(())
    }).await?;

    // Show updated configuration
    let updated_config = config_manager.get_config();
    info!("✅ Updated API Port: {}", updated_config.network.ports.api);
    info!("✅ Updated Max Connections: {}", updated_config.performance.max_connections);
    info!("✅ Distributed Mode: {}", updated_config.features.enable_distributed_mode);

    // Demonstrate runtime overrides
    let mut overrides = HashMap::new();
    overrides.insert("api_port".to_string(), "8086".to_string());
    overrides.insert("songbird_endpoint".to_string(), 
                    "https://songbird-staging.beardog.local:8443".to_string());

    let mut override_manager = ConfigManager::new();
    override_manager.add_source(ConfigSource::Runtime(overrides));
    override_manager.load().await?;

    let override_config = override_manager.get_config();
    info!("🔄 Runtime Override API Port: {}", override_config.network.ports.api);
    info!("🔄 Runtime Override SongBird: {}", override_config.external_services.songbird_endpoint);

    Ok(())
}

/// Demonstrate integration with services
async fn demo_service_integration() -> BearDogResult<()> {
    info!("🔗 Demo 5: Service Integration");

    let config_manager = ConfigManager::new();
    let config = config_manager.get_config();

    // Example: Starting an API server with configuration
    info!("🚀 Starting API Server with configuration:");
    let api_bind_address = config.get_bind_address("api");
    let api_port = config.get_port("api");
    info!("   Binding API server to: {}", api_bind_address);
    info!("   API Port: {}", api_port);

    // Example: Configuring database connection
    info!("🗄️  Configuring Database Connection:");
    info!("   Max Connections: {}", config.database.max_connections);
    info!("   Connection Timeout: {}s", config.database.connection_timeout_secs);
    info!("   Idle Timeout: {}s", config.database.idle_timeout_secs);

    // Example: Setting up monitoring
    info!("📊 Setting up Monitoring:");
    let metrics_port = config.get_port("metrics");
    let health_port = config.get_port("health");
    info!("   Metrics Port: {}", metrics_port);
    info!("   Health Check Port: {}", health_port);
    info!("   Health Check Interval: {}s", config.monitoring.health_check_interval_secs);

    // Example: Configuring external services
    info!("🌐 Configuring External Services:");
    info!("   SongBird: {}", config.external_services.songbird_endpoint);
    info!("   NestGate: {}", config.external_services.nestgate_endpoint);
    info!("   Squirrel: {}", config.external_services.squirrel_endpoint);
    info!("   ToadStool: {}", config.external_services.toadstool_endpoint);

    // Example: Feature flag usage
    info!("🎛️  Feature Flags:");
    if config.features.enable_genetic_optimization {
        info!("   ✅ Genetic Optimization: ENABLED");
        info!("      Population Size: {}", config.performance.population_size);
        info!("      Mutation Rate: {}", config.performance.mutation_rate);
    } else {
        info!("   ⏸️  Genetic Optimization: DISABLED");
    }

    if config.features.enable_threat_detection {
        info!("   ✅ Threat Detection: ENABLED");
        info!("      Threat Threshold: {}", config.performance.threat_threshold);
        info!("      Anomaly Threshold: {}", config.performance.anomaly_threshold);
    }

    if config.features.enable_distributed_mode {
        info!("   ✅ Distributed Mode: ENABLED");
        info!("      P2P Discovery Port: {}", config.get_port("p2p"));
    }

    // Example: Security configuration
    info!("🔐 Security Configuration:");
    info!("   Session Timeout: {}s", config.security.session_timeout_secs);
    info!("   Key Rotation Interval: {}s", config.security.key_rotation_interval_secs);
    info!("   Encryption Key Size: {} bytes", config.security.encryption_key_size);
    info!("   HSM Library Path: {}", config.security.hsm_library_path);

    Ok(())
}

/// Example of how to use configuration in an actual service
pub struct ApiServer {
    config: NewBearDogConfig,
}

impl ApiServer {
    pub fn new(config_manager: &ConfigManager) -> Self {
        Self {
            config: config_manager.get_config(),
        }
    }

    pub async fn start(&self) -> BearDogResult<()> {
        let bind_address = &self.config.network.api_bind_address;
        let max_connections = self.config.performance.max_connections;
        let request_timeout = self.config.performance.request_timeout_secs;

        info!("🚀 Starting API Server");
        info!("   Bind Address: {}", bind_address);
        info!("   Max Connections: {}", max_connections);
        info!("   Request Timeout: {}s", request_timeout);

        // In a real implementation, you would:
        // 1. Create the HTTP server with these settings
        // 2. Configure connection pools
        // 3. Set up middleware with the configuration
        // 4. Start listening on the configured address

        info!("✅ API Server started successfully (simulated)");
        Ok(())
    }

    pub async fn configure_cors(&self) -> Vec<String> {
        // Use configured CORS origins instead of hardcoded values
        self.config.network.cors_origins.clone()
    }

    pub fn get_rate_limit(&self) -> u32 {
        // Use configured rate limit instead of hardcoded value
        self.config.performance.rate_limit_per_minute
    }

    pub fn should_enable_feature(&self, feature: &str) -> bool {
        // Use feature flags instead of hardcoded feature switches
        match feature {
            "genetic_optimization" => self.config.features.enable_genetic_optimization,
            "threat_detection" => self.config.features.enable_threat_detection,
            "distributed_mode" => self.config.features.enable_distributed_mode,
            "advanced_caching" => self.config.features.enable_advanced_caching,
            _ => false,
        }
    }
} 