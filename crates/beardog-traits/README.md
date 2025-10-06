# `BearDog` Traits

Unified trait system for the `BearDog` distributed security ecosystem.

## ⚠️ Migration Notice

**Current Status**: This crate is in **active migration** to consolidate provider traits.

**Three Provider Hierarchies**:
1. **`canonical/`** - 🟡 **LEGACY** (Deprecated, maintained for compatibility)
2. **`unified/`** - 🟢 **CURRENT** (Active use, recommended for now)
3. **`beardog-types::canonical::providers_unified/`** - 🔵 **TARGET** (Final location)

**Recommendation**: Use `unified::*` traits for new code. Migration to final location in `beardog-types` planned for Week 3-4.

## Overview

This crate provides zero-cost abstractions and type-safe traits that enable seamless integration across all `BearDog` components. The trait system ensures compile-time guarantees while maintaining runtime performance.

## Features

- **Zero-Cost Abstractions**: All traits compile to optimal machine code
- **Type Safety**: Comprehensive compile-time verification
- **Unified Interface**: Consistent API across all `BearDog` components
- **Extensible**: Easy integration for new components and providers

## Core Traits

- `Identifiable`: Unique identification for all entities
- `Configurable`: Type-safe configuration management
- `Serializable`: Efficient serialization with multiple formats
- `Versionable`: Version-aware migration support
- `PolicyEngine`: Security policy evaluation
- `CryptoProvider`: Cryptographic operations interface

## Usage

```rust
use beardog_traits::prelude::*;

#[derive(Identifiable, Configurable)]
struct MyComponent {
    id: String,
    config: MyConfig,
}
```

## Migration Guide

### Current Provider Trait Usage

```rust
// ✅ RECOMMENDED: Use unified traits
use beardog_traits::unified::{BearDogProvider, SecurityProvider};

// ⚠️ LEGACY: Avoid canonical traits (will be removed)
// use beardog_traits::canonical::BaseProvider;  // Deprecated

// 🔵 FUTURE: Eventually migrate to beardog-types
// use beardog_types::canonical::providers_unified::UnifiedProvider;
```

### Migration Timeline

- **Now**: Use `unified::*` traits
- **Week 3-4**: Migrate to `beardog-types::canonical::providers_unified::*`
- **Future**: Remove `canonical/` (legacy) entirely

## License

This project is licensed under the AGPL-3.0 License - see the LICENSE file for details. 