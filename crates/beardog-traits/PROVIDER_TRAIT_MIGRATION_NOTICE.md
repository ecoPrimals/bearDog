# Provider Trait Migration Notice

## ⚠️ DEPRECATION NOTICE

The fragmented provider traits across `beardog-traits` crates are **DEPRECATED** and will be removed in a future release.

## Migration Required

All provider trait implementations should now use the **Unified Canonical Provider System** located at:
- `beardog-types/src/canonical/providers_unified/traits.rs`
- Migration utilities: `beardog-types/src/canonical/providers_unified/trait_migration.rs`

## New Unified Trait Hierarchy

### Root Trait
- **`UnifiedProvider`** - Foundation for all BearDog providers

### Specialized Traits
- **`UnifiedSecurityProvider`** - Replaces `SecurityProvider`, `CryptoProvider`
- **`UnifiedHsmProvider`** - Replaces `HsmProvider`, hardware-specific traits
- **`UnifiedMonitoringProvider`** - Replaces `MonitoringProvider`
- **`UnifiedStorageProvider`** - Replaces `DatabaseProvider`, `CacheProvider`
- **`UnifiedNetworkProvider`** - Replaces network-related provider traits
- **`UnifiedAiProvider`** - Replaces `AiProvider`

## Migration Benefits

✅ **Single Trait Hierarchy**: One coherent provider system eliminates fragmentation  
✅ **Native Async**: Zero-cost native async/await functions (no async_trait!)  
✅ **Enhanced Performance**: Zero-cost abstractions with optimal async support  
✅ **Better Type Safety**: Strongly typed with comprehensive error handling  
✅ **Migration Utilities**: Automated migration from legacy traits  
✅ **Compatibility Shims**: Gradual migration support  

## Migration Timeline

- **Phase 1** (Current): Unified system available, migration utilities provided
- **Phase 2** (Next Release): Legacy traits marked deprecated  
- **Phase 3** (Future Release): Legacy traits removed

## How to Migrate

### Option 1: Use Migration Service (Recommended)

```rust
use beardog_types::canonical::providers_unified::{
    ProviderMigrationService,
    LegacyProviderTrait,
    create_canonical_hsm_legacy_provider,
};

// Create legacy provider metadata
let legacy_hsm = create_canonical_hsm_legacy_provider(
    "MyHsmProvider".to_string(),
    "hardware".to_string(),
    vec!["AES".to_string(), "RSA".to_string()],
);

// Migrate to unified system
let migration_service = ProviderMigrationService::default();
let result = migration_service
    .migrate_provider_traits(vec![legacy_hsm])
    .await?;

// Use migration report for implementation guidance
println!("{}", ProviderMigrationService::create_provider_migration_summary(&result.report));
```

### Option 2: Manual Migration

```rust
use beardog_types::canonical::providers_unified::traits::{
    UnifiedProvider,
    UnifiedHsmProvider,
    ProviderInfo,
    ProviderHealth,
    ProviderMetrics,
};

// Replace your legacy provider trait implementation
pub struct MyProvider {
    // ... your implementation
}

impl UnifiedProvider for MyProvider {
    fn provider_info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "MyProvider".to_string(),
            version: "1.0.0".to_string(),
            provider_type: ProviderType::Security,
            capabilities: vec![
                ProviderCapability::Authentication,
                ProviderCapability::Encryption,
            ],
        }
    }

    async fn health_check(&self) -> Result<ProviderHealth, BearDogError> {
        // Implementation
    }

    async fn metrics(&self) -> Result<ProviderMetrics, BearDogError> {
        // Implementation  
    }

    // ... other required methods
}

impl UnifiedHsmProvider for MyProvider {
    async fn device_info(&self) -> Result<HsmDeviceInfo, BearDogError> {
        // HSM-specific implementation
    }

    async fn generate_hardware_key(&self, spec: HardwareKeySpec) -> Result<HardwareKeyInfo, BearDogError> {
        // Hardware key generation
    }

    // ... other HSM methods
}
```

## Import Path Changes

### Before (Legacy - DEPRECATED)
```rust
// Fragmented across multiple locations
use beardog_traits::canonical::{BaseProvider, HsmProvider, SecurityProvider};
use beardog_traits::unified::{BearDogProvider, CryptoProvider};
use beardog_tunnel::hsm_foundation::traits::HsmProvider; // Different HsmProvider!
```

### After (Unified - CURRENT)
```rust
// Single canonical location
use beardog_types::canonical::providers_unified::traits::{
    UnifiedProvider,
    UnifiedSecurityProvider,
    UnifiedHsmProvider,
    UnifiedMonitoringProvider,
    UnifiedStorageProvider,
    UnifiedNetworkProvider,
    UnifiedAiProvider,
};
```

## Trait Hierarchy Changes

### Legacy Structure (DEPRECATED)
```rust
// Fragmented traits in different crates
trait BaseProvider { /* basic functionality */ }
trait HsmProvider: BaseProvider { /* HSM operations */ }
trait SecurityProvider: BaseProvider { /* security operations */ }
trait BearDogProvider: BearDogCore { /* unified operations */ }

// Multiple HsmProvider definitions in different crates!
// No clear hierarchy or consistency
```

### Unified Structure (CURRENT)
```rust
// Single, coherent hierarchy
trait UnifiedProvider: Send + Sync {
    // Core provider functionality
    fn provider_info(&self) -> ProviderInfo;
    async fn health_check(&self) -> Result<ProviderHealth, BearDogError>;
    async fn metrics(&self) -> Result<ProviderMetrics, BearDogError>;
    // ... other core methods
}

trait UnifiedSecurityProvider: UnifiedProvider {
    // Security-specific functionality
    async fn authenticate(&self, credentials: AuthenticationRequest) -> Result<AuthenticationResponse, BearDogError>;
    async fn authorize(&self, request: AuthorizationRequest) -> Result<AuthorizationResponse, BearDogError>;
    // ... other security methods
}

trait UnifiedHsmProvider: UnifiedSecurityProvider {
    // HSM-specific functionality  
    async fn device_info(&self) -> Result<HsmDeviceInfo, BearDogError>;
    async fn generate_hardware_key(&self, spec: HardwareKeySpec) -> Result<HardwareKeyInfo, BearDogError>;
    // ... other HSM methods
}
```

## Performance Benefits

### Native Async Functions
- **Before**: `async_trait` boxing overhead
- **After**: Zero-cost native async functions

### Single Trait System
- **Before**: Multiple trait definitions, compilation overhead
- **After**: Single canonical system, optimized compilation

### Type Safety
- **Before**: Inconsistent error types across traits
- **After**: Unified `BearDogError` throughout

## Migration Examples by Provider Type

### HSM Provider Migration
```rust
// Before (DEPRECATED)
impl beardog_traits::canonical::HsmProvider for MyHsm {
    // Legacy implementation
}

// After (CURRENT)  
impl UnifiedHsmProvider for MyHsm {
    // Unified implementation with native async
}
```

### Security Provider Migration
```rust
// Before (DEPRECATED)
impl beardog_traits::canonical::SecurityProvider for MySecurity {
    // Legacy implementation
}

// After (CURRENT)
impl UnifiedSecurityProvider for MySecurity {
    // Unified implementation with enhanced capabilities
}
```

### Monitoring Provider Migration
```rust
// Before (DEPRECATED)
impl beardog_traits::canonical::MonitoringProvider for MyMonitoring {
    // Legacy implementation
}

// After (CURRENT)
impl UnifiedMonitoringProvider for MyMonitoring {
    // Unified implementation with comprehensive metrics
}
```

## Support

If you encounter issues during migration:

1. **Use Migration Service**: Automated migration handles most cases
2. **Check Migration Report**: Detailed reports with implementation guidance
3. **Review Documentation**: See `docs/architecture/PROVIDER_TRAIT_ARCHITECTURE.md`
4. **Compatibility Shims**: Gradual migration support available
5. **Custom Mappings**: Handle special cases with custom trait mappings

## Examples

See `examples/provider_trait_migration_demo.rs` for complete migration examples.

---

**This migration is part of the broader BearDog ecosystem unification initiative to eliminate trait fragmentation and provide a single canonical provider system.** 