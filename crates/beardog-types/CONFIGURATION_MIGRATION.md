# 🔄 Configuration System Migration Guide

**BearDog v3.1.0 Configuration Migration**

This guide helps you migrate from the legacy configuration system to the new canonical configuration architecture.

## 🎯 **Migration Overview**

The BearDog ecosystem has consolidated configuration systems to eliminate fragmentation and provide a single source of truth.

### **What's Changing**

- **Legacy System**: `beardog_types::configuration::*` (deprecated)
- **New System**: `beardog_types::canonical::config::*` (canonical)

## 📋 **Step-by-Step Migration**

### **1. Update Imports**

**OLD (deprecated)**:
```rust
use beardog_types::configuration::{
    BearDogConfig,
    UnifiedBearDogConfig,
    AppConfig,
    NetworkConfig,
    SecurityConfig,
    HsmConfig,
    MonitoringConfig,
};
```

**NEW (canonical)**:
```rust
use beardog_types::canonical::config::{
    unified::UnifiedBearDogConfig,
    app::UnifiedAppConfig,
    network::UnifiedNetworkConfig,
    security::UnifiedSecurityConfig,
    hsm::UnifiedHsmConfig,
    monitoring::UnifiedMonitoringConfig,
};
```

### **2. Update Configuration Types**

**OLD**:
```rust
let config = UnifiedBearDogConfig {
    app: AppConfig::default(),
    network: NetworkConfig::default(),
    security: SecurityConfig::default(),
    hsm: HsmConfig::default(),
    monitoring: MonitoringConfig::default(),
    metadata: ConfigMetadata::default(),
};
```

**NEW**:
```rust
let config = UnifiedBearDogConfig {
    metadata: SystemMetadata::default(),
    app: UnifiedAppConfig::default(),
    network: UnifiedNetworkConfig::default(),
    security: UnifiedSecurityConfig::default(),
    hsm: UnifiedHsmConfig::default(),
    monitoring: UnifiedMonitoringConfig::default(),
    // Additional domains now available:
    genetics: UnifiedGeneticsConfig::default(),
    workflows: UnifiedWorkflowConfig::default(),
    compliance: UnifiedComplianceConfig::default(),
    performance: UnifiedPerformanceConfig::default(),
    production: UnifiedProductionConfig::default(),
    deployment: UnifiedDeploymentConfig::default(),
    testing: UnifiedTestingConfig::default(),
    development: UnifiedDevelopmentConfig::default(),
    adapters: UnifiedAdapterConfig::default(),
    tunnel: UnifiedTunnelConfig::default(),
    federation: UnifiedFederationConfig::default(),
    ecosystem: UnifiedEcosystemConfig::default(),
};
```

### **3. Update Trait Implementations**

**OLD**:
```rust
impl BearDogConfig for MyConfig {
    fn validate(&self) -> Result<(), BearDogError> { /* ... */ }
    fn merge(&mut self, other: Self) -> Result<(), BearDogError> { /* ... */ }
    fn from_env() -> Result<Self, BearDogError> { /* ... */ }
}
```

**NEW**:
```rust
// Canonical configs implement validation automatically
// Custom validation can be added via the ValidatedConfig trait
use beardog_types::canonical::config::validation::ValidatedConfig;

impl ValidatedConfig for MyConfig {
    fn custom_validate(&self) -> Result<(), BearDogError> { /* ... */ }
}
```

### **4. Configuration Loading**

**OLD**:
```rust
let config = AppConfig::from_env()?;
config.validate()?;
```

**NEW**:
```rust
use beardog_types::canonical::config::unified_simple::WorkingUnifiedConfig;

// Use the working implementation for immediate deployment
let config = WorkingUnifiedConfig::from_environment()?;
// Validation is built-in and automatic
```

## 🔄 **Automated Migration Tools**

### **Migration Utility**

```rust
use beardog_types::canonical::config::migration::ConfigurationMigrator;

// Migrate existing configuration
let migrator = ConfigurationMigrator::new();
let new_config = migrator.migrate_from_legacy(old_config)?;

// Validate migration
migrator.validate_migration(&old_config, &new_config)?;
```

### **Compatibility Layer**

During migration, you can use the compatibility layer:

```rust
use beardog_types::canonical::config::compat::LegacyConfigAdapter;

// Temporary adapter for gradual migration
let adapter = LegacyConfigAdapter::new(legacy_config);
let canonical_config = adapter.to_canonical()?;
```

## ⚡ **Benefits of Migration**

### **Before (Legacy)**
- Multiple configuration systems
- Fragmented config types across crates
- Inconsistent validation patterns
- Manual configuration merging

### **After (Canonical)**
- Single source of truth
- Unified validation system
- Automatic environment loading
- Built-in configuration merging
- Comprehensive domain coverage

## 🚨 **Breaking Changes**

### **Removed Types**
- `ConfigMetadata` → `SystemMetadata`
- `ConfigSource` → Built into unified system

### **Renamed Fields**
- `metadata` → `metadata` (same name, different type)
- Configuration field names remain consistent

### **New Required Fields**
The canonical system includes additional domains that may require configuration:
- `genetics`: Genetic algorithm configuration
- `workflows`: Workflow engine configuration
- `compliance`: Compliance monitoring configuration
- `performance`: Performance optimization settings

## 📝 **Migration Checklist**

- [ ] Update imports to canonical config system
- [ ] Replace legacy config types with unified types
- [ ] Update configuration initialization code
- [ ] Test configuration loading and validation
- [ ] Update documentation and examples
- [ ] Remove deprecated imports after migration
- [ ] Validate all configuration-dependent functionality

## 🆘 **Migration Support**

### **Automated Detection**
```bash
# Find usage of deprecated configuration types
grep -r "beardog_types::configuration" src/
```

### **Validation**
```rust
// Ensure migration is complete
use beardog_types::canonical::config::validation::MigrationValidator;

let validator = MigrationValidator::new();
validator.check_codebase_migration("src/")?;
```

## 🎯 **Timeline**

- **v3.1.0**: Legacy system deprecated, canonical system available
- **v3.2.0**: Migration warnings increased
- **v3.3.0**: Legacy system will be removed

**Migrate now to avoid disruption!**

## 📞 **Support**

If you encounter issues during migration:

1. Check the canonical config documentation
2. Use the migration utilities provided
3. Review the examples in `examples/config_migration/`
4. File issues for migration-specific problems

---

**Status**: Migration guide complete - ready for v3.1.0 deployment 