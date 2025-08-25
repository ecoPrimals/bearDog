# `BearDog` Traits

Unified trait system for the `BearDog` distributed security ecosystem.

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

## License

This project is licensed under the AGPL-3.0 License - see the LICENSE file for details. 