# BearDog Unification Migration Guide

## Provider Trait Migrations

### Deprecated → Unified Mappings

- `UniversalPrimalProvider` → `UniversalProvider`
- `ExternalSystemProvider` → `UniversalProvider`
- `UniversalServiceProvider` → `UniversalProvider`
- `SafeHardwareProvider` → `PlatformProvider`
- `SafeIOSProvider` → `PlatformProvider`
- `GamingSecurityProvider` → `PlatformProvider`
- `SimpleCacheProvider` → `EnhancedCacheProvider`

## Configuration Struct Migrations

### Deprecated → Unified Mappings

- `SecurityProcessorConfig` → `UnifiedProcessorConfig`
- `PolicyProcessorConfig` → `UnifiedProcessorConfig`
- `SystemProcessorConfig` → `UnifiedProcessorConfig`
- `KeyManagementConfig` → `UnifiedProcessorConfig`
- `RegistryProcessorConfig` → `UnifiedProcessorConfig`
- `RecoveryConfig` → `UnifiedRecoveryConfig`
- `SocialRecoveryConfig` → `UnifiedRecoveryConfig`
- `FederationRecoveryConfig` → `UnifiedRecoveryConfig`
- `LoadTestConfiguration` → `UnifiedTestingConfig`
- `StressTestConfiguration` → `UnifiedTestingConfig`
- `PenetrationTestConfiguration` → `UnifiedTestingConfig`
- `AuthConfig` → `UnifiedAuthConfig`
- `VerificationConfig` → `UnifiedAuthConfig`
- `SessionConfig` → `UnifiedAuthConfig`

## Migration Steps

1. **Update Imports**: Change import statements to use new unified types
2. **Update Type Annotations**: Replace deprecated types with unified equivalents  
3. **Update Implementations**: Migrate trait implementations to new unified traits
4. **Test Thoroughly**: Ensure all functionality works with new unified system
5. **Remove Deprecated**: After migration period, remove deprecated types

## Example Migration

```rust
// Before
use beardog_traits::canonical::UniversalPrimalProvider;
use beardog_types::config::SecurityProcessorConfig;

impl UniversalPrimalProvider for MyProvider {
    // implementation
}

// After  
use beardog_traits::canonical::UniversalProvider;
use beardog_types::config::UnifiedProcessorConfig;

impl UniversalProvider for MyProvider {
    // updated implementation
}
```
