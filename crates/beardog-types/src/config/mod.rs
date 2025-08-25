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


/// # BearDog Configuration System
///
/// **UNIFIED CONFIGURATION ARCHITECTURE** ✅ **COMPLETE**
/// This module provides the **single source of truth** for ALL configuration
/// across the BearDog ecosystem. It consolidates all configuration types 
/// into a unified, canonical system.

// ============================================================================
// **FOCUSED CONFIGURATION MODULES** - Under 2000 lines each
/// **PERFORMANCE CONFIGURATION** - Async, memory, concurrency, timeouts
pub mod performance;
/// **MONITORING CONFIGURATION** - Metrics, alerting, health checks, profiling
pub mod monitoring;
/// **SECURITY CONFIGURATION** - Crypto, auth, authorization, HSM
pub mod security_unified;
/// **NETWORK CONFIGURATION** - Ports, TLS, load balancing, connections
pub mod network_unified;
/// **APPLICATION CONFIGURATION** - Core app settings
pub mod app;
/// **DATABASE CONFIGURATION** - Database connection and optimization
pub mod database;
/// **COMPLIANCE CONFIGURATION** - Regulatory and compliance settings
pub mod compliance;
/// **DISCOVERY CONFIGURATION** - Service discovery settings
pub mod discovery;
/// **LOAD TESTING CONFIGURATION** - Performance testing settings
pub mod load_testing;
/// **MANAGER CONFIGURATION** - Configuration management settings
pub mod manager;
/// **NODE REGISTRY CONFIGURATION** - Node registry settings
pub mod node_registry;
/// **PLATFORM CONFIGURATION** - Platform-specific settings
pub mod platform;
/// **PRODUCTION CONFIGURATION** - Production deployment settings
pub mod production;
/// **SECRETS CONFIGURATION** - Secret management settings
pub mod secrets;
/// **TUNNEL CONFIGURATION** - Tunnel and HSM settings
pub mod tunnel;
/// **INTEGRATION CONFIGURATION** - Ecosystem integration settings
pub mod integration;
/// **MIGRATION UTILITIES** - Configuration consolidation helpers
pub mod migration;
/// **CANONICAL CONSOLIDATION** - Unified canonical configurations
pub mod canonical_consolidation;
// **CANONICAL CONFIGURATION EXPORTS** - Direct access to unified types
// Re-export the main configuration types from focused modules
pub use integration::IntegrationConfig;
pub use network_unified::UnifiedNetworkConfig;
pub use performance::GeneralPerformanceConfig as UnifiedPerformanceConfig;
pub use monitoring::BasicMonitoringConfig as UnifiedMonitoringConfig;
pub use production::UnifiedProductionConfig as ProductionConfig;
pub use security_unified::UnifiedSecurityConfig;
// Re-export other configuration types
pub use app::AppConfig;
pub use database::UnifiedDatabaseConfig as DatabaseConfig;
pub use compliance::UnifiedComplianceConfig as ComplianceConfig;
pub use discovery::UnifiedDiscoveryConfig as DiscoveryConfig;
pub use load_testing::UnifiedLoadTestConfig as LoadTestingConfig;
pub use manager::UnifiedConfigManager as ConfigManagerConfig;
pub use node_registry::UnifiedNodeRegistryConfig as NodeRegistryConfig;
pub use platform::UnifiedPlatformConfig as PlatformConfig;

// **CANONICAL CONSOLIDATION EXPORTS** - New unified configurations
pub use canonical_consolidation::{
    CanonicalHsmConfig, CanonicalTunnelConfig, CanonicalAdapterConfig,
    ConfigurationConsolidator, ConsolidationReport
};

// **CANONICAL UNIFIED CONFIGURATION** - Single source of truth
/// **BEARDOG CANONICAL CONFIGURATION** - The definitive configuration structure
/// This is the primary configuration that unifies all domain-specific configurations
/// into a single, canonical system. All other configuration structs should use this
/// as the foundation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[derive(Default)]
pub struct BearDogCanonicalConfig {
    /// Application configuration
    pub app: AppConfig,
    /// Performance optimization configuration  
    pub performance: UnifiedPerformanceConfig,
    /// Security configuration (includes SIMD crypto)
    pub security: UnifiedSecurityConfig,
    /// Database configuration (includes optimization)
    pub database: DatabaseConfig,
    /// Monitoring configuration (includes production)
    pub monitoring: UnifiedMonitoringConfig,
    /// Network configuration
    pub network: UnifiedNetworkConfig,
    /// Production deployment configuration
    pub production: ProductionConfig,
    /// Integration configuration
    pub integration: IntegrationConfig,
    /// Tunnel configuration (includes gaming, genetic healing, and BSTP)
    pub tunnel: tunnel::UnifiedTunnelConfig,
}


/// Configuration performance metrics and validation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfigPerformanceMetrics;

impl ConfigPerformanceMetrics {
    /// Validate configuration for performance issues
    pub fn validate_performance(_config: &BearDogCanonicalConfig) -> Vec<String> {
        // Configuration validation logic would go here
        vec![]
    }
    
    /// Get configuration optimization suggestions
    pub fn get_optimization_suggestions(_config: &BearDogCanonicalConfig) -> Vec<String> {
        // Optimization suggestion logic would go here
        vec![]
    }
}
