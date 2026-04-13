# BearDog Canonical Type System Migration Guide

**Version**: 1.0  
**Date**: January 2025  
**Status**: ✅ **ACTIVE GUIDE FOR CANONICAL TYPE ADOPTION**  
**Audience**: **Developers, Contributors, Integration Partners**

---

## 🎯 **Executive Summary**

This guide provides comprehensive instructions for working with BearDog's **unified canonical type system**. The transformation consolidates fragmented type definitions into a single source of truth, providing enterprise-grade type safety and maintainability across the entire 217k+ line codebase.

### **📋 Quick Migration Checklist**
- ✅ **Replace duplicate imports** with canonical paths
- ✅ **Use Default::default()** and field overrides for instantiation
- ✅ **Add compatibility fields** to struct instantiations
- ✅ **Update pattern matching** to be exhaustive
- ✅ **Leverage pub use** statements for module exports

---

## 🏗️ **Canonical Type Architecture**

### **Import Paths**

```rust
// ✅ CORRECT: Canonical HSM types
use beardog_types::canonical::hsm::{
    KeyMetadata, KeyUsagePolicy, HsmKey, KeyHealth
};

// ✅ CORRECT: Platform-specific HSM types
use beardog_types::hsm::{
    android::AttestationConfig,
    ios::SecureEnclaveConfig,
    software::SoftwareHsmConfig,
};

// ✅ CORRECT: Canonical capabilities
use beardog_types::canonical::capabilities::{
    HumanEntropyCapabilities, SecurityCapabilities
};

// ❌ INCORRECT: Don't use tunnel-specific duplicates
// use crate::tunnel::hsm::types::{KeyMetadata, KeyUsagePolicy}; // REMOVED
```

### **Type Hierarchy**

```rust
beardog-types/
├── canonical/           // SINGLE SOURCE OF TRUTH
│   ├── hsm/
│   │   ├── keys.rs     // KeyMetadata (25+ fields), KeyUsagePolicy (10+ fields)
│   │   ├── config.rs   // HsmConfig, unified configurations
│   │   └── operations.rs // KeyOperation, KeyHealth enums
│   ├── crypto/         // Cryptographic type definitions
│   └── capabilities.rs // Unified capability types
├── hsm/                // PLATFORM-SPECIFIC IMPLEMENTATIONS
│   ├── android.rs      // AndroidStrongBoxHsm, AttestationConfig
│   ├── ios.rs         // iOSSecureEnclaveHsm
│   └── software.rs    // SoftwareHsm implementations
└── config/            // Configuration management
```

---

## 🔄 **Migration Patterns**

### **1. Struct Instantiation - Use Default + Override**

```rust
// ✅ CORRECT: Use Default::default() and override specific fields
use beardog_types::canonical::hsm::{KeyMetadata, KeyUsagePolicy};

let metadata = KeyMetadata {
    created_by: "my_service".to_string(),
    purpose: "encryption_key".to_string(),
    algorithm: "Ed25519".to_string(),
    is_hardware_backed: true,
    user_presence_required: true,
    ..KeyMetadata::default() // Use defaults for other 20+ fields
};

let policy = KeyUsagePolicy {
    allowed_operations: vec![KeyOperation::Encrypt, KeyOperation::Decrypt],
    can_encrypt: true,
    can_decrypt: true,
    exportable: false,
    ..KeyUsagePolicy::default() // Use defaults for other compatibility fields
};

// ❌ INCORRECT: Don't manually specify all fields
// let metadata = KeyMetadata {
//     created_by: "my_service".to_string(),
//     purpose: "encryption_key".to_string(),
//     usage_policy: policy,
//     tags: vec![],
//     compliance_info: None,
//     backup_info: None,
//     // ... 20+ more fields manually specified
// };
```

### **2. Module Exports - Use pub use**

```rust
// ✅ CORRECT: Replace duplicate definitions with pub use
// In your module (e.g., crates/beardog-tunnel/src/tunnel/hsm/types/key.rs)

// Remove duplicate definitions and use canonical types
pub use beardog_types::canonical::hsm::{
    KeyMetadata, KeyUsagePolicy, HsmKey, KeyHealth
};

// Keep module-specific types that don't have canonical equivalents
pub struct TunnelSpecificConfig {
    pub tunnel_id: String,
    pub connection_params: HashMap<String, String>,
}

// ❌ INCORRECT: Don't maintain duplicate definitions
// pub struct KeyMetadata {
//     pub created_by: String,
//     // ... duplicate fields
// }
```

### **3. Pattern Matching - Exhaustive Coverage**

```rust
// ✅ CORRECT: Handle all enum variants
use beardog_types::canonical::crypto::EntropyCollectionMethod;

match entropy_method {
    EntropyCollectionMethod::Keyboard => handle_keyboard(),
    EntropyCollectionMethod::TouchScreen => handle_touch(),
    EntropyCollectionMethod::MouseMovement => handle_mouse(),
    EntropyCollectionMethod::Custom { method, .. } => handle_custom(method),
    // All variants covered - no compiler warnings
}

// ✅ CORRECT: Add new variants when extending enums
#[derive(Debug, Clone)]
pub enum ExtendedAlgorithm {
    Canonical(beardog_types::canonical::crypto::KeyType),
    PostQuantum(String),
    Custom(String),
}

// ❌ INCORRECT: Non-exhaustive patterns
// match entropy_method {
//     EntropyCollectionMethod::Keyboard => handle_keyboard(),
//     EntropyCollectionMethod::TouchScreen => handle_touch(),
//     // Missing MouseMovement and Custom variants - compiler error!
// }
```

### **4. Configuration Management**

```rust
// ✅ CORRECT: Use canonical configuration types
use beardog_types::hsm::android::AttestationConfig;

let attestation_config = AttestationConfig {
    require_hardware_attestation: true,
    accepted_attestation_levels: vec!["StrongBox".to_string()],
    attestation_timeout: Duration::from_secs(30),
    // Tunnel compatibility fields
    enabled: true,
    challenge_timeout_secs: 30,
    cache_results: true,
    cache_ttl_secs: 3600,
};

// ✅ CORRECT: Extend canonical types for specific needs
#[derive(Debug, Clone)]
pub struct TunnelAttestationConfig {
    pub base: AttestationConfig,
    pub tunnel_specific_field: String,
}

// ❌ INCORRECT: Don't duplicate canonical configurations
// pub struct MyAttestationConfig {
//     pub require_hardware_attestation: bool,
//     // ... duplicate fields
// }
```

---

## 🛠️ **Common Migration Scenarios**

### **Scenario 1: HSM Key Management**

```rust
// BEFORE: Fragmented types
use crate::tunnel::hsm::types::{KeyMetadata as TunnelKeyMetadata, KeyUsagePolicy};
use beardog_types::{KeyMetadata as CoreKeyMetadata};

// AFTER: Unified canonical types
use beardog_types::canonical::hsm::{KeyMetadata, KeyUsagePolicy, HsmKey};

// Generate key with canonical types
let key_request = GenerateKeyRequest {
    key_id: format!("key_{}", uuid::Uuid::new_v4()),
    key_type: KeyType::Ed25519,
    usage_policy: KeyUsagePolicy {
        allowed_operations: vec![KeyOperation::Sign],
        can_sign: true,
        can_verify: true,
        ..KeyUsagePolicy::default()
    },
    metadata: KeyMetadata {
        created_by: "hsm_service".to_string(),
        purpose: "signing_key".to_string(),
        is_hardware_backed: true,
        algorithm: "Ed25519".to_string(),
        ..KeyMetadata::default()
    },
    // Compatibility fields for tunnel operations
    attestation_challenge: None,
    require_user_presence: false,
    generate_attestation: true,
    target_hsm_tier: "StrongBox".to_string(),
};
```

### **Scenario 2: Security Capabilities**

```rust
// BEFORE: Multiple capability definitions
use crate::tunnel::types::SecurityCapabilities as TunnelCapabilities;
use beardog_types::SecurityCapabilities as CoreCapabilities;

// AFTER: Unified canonical capabilities
use beardog_types::canonical::capabilities::SecurityCapabilities;

let capabilities = SecurityCapabilities {
    encryption_algorithms: vec!["AES-256".to_string(), "ChaCha20".to_string()],
    signature_algorithms: vec!["Ed25519".to_string(), "ECDSA".to_string()],
    key_derivation_functions: vec!["HKDF".to_string(), "PBKDF2".to_string()],
    // Tunnel compatibility fields
    physical_security_level: "Level3".to_string(),
    security_level: "High".to_string(),
    audit_logging: true,
    attestation_support: true,
    ..SecurityCapabilities::default()
};
```

### **Scenario 3: Android StrongBox Integration**

```rust
// BEFORE: Platform-specific duplicate types
use crate::tunnel::hsm::android_strongbox::types::AndroidHealthMonitor;

// AFTER: Canonical types with platform-specific implementations
use beardog_types::canonical::hsm::{KeyMetadata, HsmKey};
use beardog_types::hsm::android::{AndroidStrongBoxHsm, AttestationConfig};

impl HsmProvider for AndroidStrongBoxHsm {
    async fn generate_key(&self, request: GenerateKeyRequest) -> BearDogResult<HsmKey> {
        // Use canonical HsmKey with all compatibility fields
        let hsm_key = HsmKey {
            id: request.key_id.clone(),
            key_type: request.key_type,
            material: KeyMaterial::HardwareReference {
                handle: hardware_handle,
                hsm_type: "AndroidStrongBox".to_string(),
            },
            metadata: KeyMetadata {
                created_by: "AndroidStrongBox".to_string(),
                purpose: "StrongBox generated key".to_string(),
                is_hardware_backed: true,
                attestation_available: true,
                hsm_type: "AndroidStrongBox".to_string(),
                hsm_tier: "StrongBox".to_string(),
                ..request.metadata
            },
            // Compatibility fields for legacy support
            key_material: Some(hardware_handle.clone()),
            attestation: generate_attestation().await?,
            provider_attributes: get_strongbox_attributes(),
            ..HsmKey::default()
        };
        
        Ok(hsm_key)
    }
}
```

---

## 📊 **Migration Benefits**

### **Type Safety Improvements**
- **Single Source of Truth**: No more conflicting type definitions
- **Compile-time Validation**: Catch type mismatches at build time
- **Field Completeness**: All use cases supported in canonical definitions
- **Pattern Matching**: 100% exhaustive coverage prevents runtime errors

### **Developer Experience Enhancements**
- **Clear Import Paths**: Always know where to import types from
- **Consistent APIs**: Same patterns across all modules
- **Future-Proof**: Easy extension through compatibility fields
- **Documentation**: Comprehensive field explanations and usage examples

### **Maintenance Benefits**
- **Reduced Duplication**: No more manual synchronization of duplicate types
- **Easier Evolution**: Add fields once in canonical definitions
- **Backward Compatibility**: Legacy code continues to work
- **Testing Simplification**: Test canonical types once, use everywhere

---

## 🚨 **Common Pitfalls & Solutions**

### **Pitfall 1: Missing Compatibility Fields**

```rust
// ❌ PROBLEM: Compilation error - missing fields
let policy = KeyUsagePolicy {
    allowed_operations: vec![KeyOperation::Sign],
    exportable: false,
    // Missing: can_sign, can_verify, user_presence_required, etc.
};

// ✅ SOLUTION: Use Default::default()
let policy = KeyUsagePolicy {
    allowed_operations: vec![KeyOperation::Sign],
    can_sign: true,
    exportable: false,
    ..KeyUsagePolicy::default()
};
```

### **Pitfall 2: Non-exhaustive Pattern Matching**

```rust
// ❌ PROBLEM: Compiler error - missing variants
match algorithm {
    Algorithm::Ed25519 => handle_ed25519(),
    Algorithm::Rsa2048 => handle_rsa(),
    // Missing: EcdsaP384, Rsa1024, etc.
}

// ✅ SOLUTION: Handle all variants
match algorithm {
    Algorithm::Ed25519 => handle_ed25519(),
    Algorithm::Rsa2048 | Algorithm::Rsa1024 | Algorithm::Rsa3072 => handle_rsa(),
    Algorithm::EcdsaP384 => handle_ecdsa(),
    Algorithm::Custom(name) => handle_custom(&name),
}
```

### **Pitfall 3: Incorrect Import Paths**

```rust
// ❌ PROBLEM: Import from removed duplicate
use crate::tunnel::hsm::types::KeyMetadata; // No longer exists

// ✅ SOLUTION: Use canonical import
use beardog_types::canonical::hsm::KeyMetadata;

// ✅ ALTERNATIVE: Use pub use in your module
pub use beardog_types::canonical::hsm::KeyMetadata;
```

---

## 🎯 **Best Practices**

### **1. Canonical-First Development**
- Always check `beardog-types::canonical` first for existing types
- Extend canonical types rather than creating new ones
- Use compatibility fields for backward compatibility

### **2. Default-Driven Instantiation**
- Use `Default::default()` and override specific fields
- Avoid manually specifying all fields in large structs
- Leverage the compiler to catch missing fields

### **3. Exhaustive Pattern Matching**
- Always handle all enum variants
- Use `_` wildcard only when absolutely necessary
- Add new variants to enums rather than using strings

### **4. Documentation Standards**
- Document compatibility fields and their purpose
- Explain migration paths in module documentation
- Provide examples of canonical type usage

### **5. Testing Approach**
- Test canonical types in isolation
- Verify compatibility with legacy code
- Include pattern matching tests for enums

---

## 🚀 **Future Evolution**

### **Planned Enhancements**
1. **Automatic Migration Tools** - CLI tools to assist with type migration
2. **Compile-time Validation** - Macros to validate canonical type usage
3. **Performance Optimizations** - Zero-cost abstractions in canonical types
4. **Documentation Generation** - Auto-generate API docs from canonical types

### **Contribution Guidelines**
- **New Types**: Add to canonical hierarchy when they serve multiple modules
- **Compatibility**: Always maintain backward compatibility through compatibility fields
- **Documentation**: Update this guide when adding new canonical types
- **Testing**: Include comprehensive tests for new canonical types

---

## 📚 **Additional Resources**

- **Canonical Type System Specification**: `CANONICAL_TYPE_SYSTEM_SPECIFICATION.md`
- **Architecture Documentation**: `../../../ARCHITECTURE.md`
- **API Interfaces**: *(see README.md method table)*
- **Code Examples**: `examples/` directory with canonical type usage
- **Test Suite**: `tests/` directory with canonical type validation

---

**This migration guide represents the foundation for working with BearDog's world-class canonical type system. Following these patterns ensures type safety, maintainability, and consistency across the entire ecosystem.** 🎉 