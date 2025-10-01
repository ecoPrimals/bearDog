# 🛡️ BearDog Coding Standards

**Version**: 3.0.0 Production  
**Status**: ✅ **ESTABLISHED STANDARDS**  
**Purpose**: Maintain code quality for the ecoPrimals security provider

---

## 🎯 **Overview**

BearDog maintains high coding standards as the security provider for the ecoPrimals ecosystem. These standards ensure reliability, maintainability, and security in all code contributions.

---

## 🏗️ **Architecture Standards**

### **File Organization**
- ✅ **File Size Limit**: Maximum 2000 lines per file (currently compliant - largest: 1,046 lines)
- ✅ **Module Structure**: Logical separation of concerns with clear module boundaries
- ✅ **Crate Organization**: 22 focused crates with single responsibilities

### **Type System Standards**
- ✅ **Canonical Types**: Use `beardog-types::canonical::*` for all type definitions
- ✅ **Unified Traits**: Use `beardog-traits::unified::*` for all trait definitions
- ✅ **Error Handling**: Use `beardog_errors::BearDogError` for all error cases

---

## 🔧 **Configuration Standards**

### **Configuration Naming Conventions**

**Canonical Configuration Types**
- ✅ **Primary Pattern**: `Canonical{Domain}Config` for all domain configs
  - Examples: `CanonicalAppConfig`, `CanonicalSecurityConfig`, `CanonicalNetworkConfig`
- ✅ **Unified Pattern**: `Unified{Domain}Config` for consolidated multi-domain configs
  - Examples: `UnifiedBearDogConfig`, `UnifiedHsmConfig`, `UnifiedProductionConfig`
- ✅ **Simplified Pattern**: `Simplified{Domain}Config` for developer-friendly configs
  - Examples: `SimplifiedBearDogConfig`

**Type Alias Guidelines**
- ✅ **Domain Aliases**: Use short aliases for frequently used configs
  - Pattern: `{Domain}Config = Canonical{Domain}Config`
  - Examples: `AppConfig`, `AuthConfig`, `DatabaseConfig`
  - Purpose: Backwards compatibility and convenience
- ❌ **Avoid Master/Global Aliases**: Don't create `MasterConfig`, `GlobalConfig`, `UnifiedConfig`
  - Reason: Ambiguous, use specific type names directly
- ❌ **Avoid Duplicate Aliases**: Each config should have ONE canonical name
  - Use `grep "pub type.*Config.*=" -r crates/` to audit

**Configuration Organization**
```rust
// ✅ GOOD: Clear, canonical pattern
pub struct CanonicalAppConfig { ... }
pub type AppConfig = CanonicalAppConfig;  // Backwards compat

// ✅ GOOD: Unified pattern for consolidated configs
pub struct UnifiedBearDogConfig { ... }
// No alias needed - use the type directly

// ❌ BAD: Multiple aliases for same type
pub type MasterConfig = UnifiedBearDogConfig;
pub type GlobalConfig = UnifiedBearDogConfig;
pub type Config = UnifiedBearDogConfig;
```

**Configuration Location**
- ✅ **Primary Location**: `beardog-types/src/canonical/config/{domain}.rs`
- ✅ **Domain Modules**: Split large configs into `{domain}/mod.rs` with submodules
- ✅ **Unified Config**: `beardog-types/src/canonical/config/unified.rs`
- ❌ **Avoid**: Scattered config definitions across multiple crates

**Deprecation Strategy**
```rust
// When deprecating config types, use this pattern:
#[deprecated(since = "3.2.0", note = "Use UnifiedBearDogConfig instead")]
pub struct OldConfig { ... }

// Allow deprecated code to use itself
#[allow(deprecated)]
impl Default for OldConfig { ... }
```

---

## 🔒 **Security Standards**

### **Memory Safety**
- ✅ **Zero Unsafe Code**: No `unsafe` blocks in production code
- ✅ **Memory Management**: Prefer stack allocation and zero-copy patterns
- ✅ **Input Validation**: Validate all external inputs at boundaries

### **Cryptographic Standards**
- ✅ **Ed25519 Signatures**: Use production-ready signature verification
- ✅ **HSM Integration**: Support hardware security modules for key operations
- ✅ **Key Management**: Secure key generation, rotation, and lifecycle management

---

## 🧪 **Code Quality Standards**

### **Compilation Standards**
```bash
# All code must pass these checks
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
cargo test --workspace
```

### **Linting Configuration**
```toml
# Pedantic clippy configuration (see .clippy.toml)
pedantic = "warn"
nursery = "warn"
unwrap_used = "deny"
expect_used = "warn"
panic = "deny"
todo = "deny"
```

### **Documentation Standards**
- ✅ **API Documentation**: All public APIs must have comprehensive documentation
- ✅ **Examples**: Include usage examples for complex APIs
- ✅ **Architecture Docs**: Maintain up-to-date architecture documentation

---

## 🚀 **Performance Standards**

### **Zero-Cost Abstractions**
- ✅ **Generic Types**: Prefer compile-time generics over runtime dispatch
- ✅ **Enum Dispatch**: Use enum-based dispatch instead of `Box<dyn>`
- ✅ **Const Generics**: Use const generics for compile-time optimizations

### **Async Standards**
- ✅ **Native Async**: Use native `async fn` instead of `async_trait`
- ✅ **Async Patterns**: Follow modern async/await patterns
- ✅ **Error Propagation**: Use `?` operator for clean error handling

---

## 🔄 **Development Workflow**

### **Git Standards**
```bash
# Commit message format
git commit -m "feat: add HSM integration for Ed25519 signatures

- Implement hardware security module support
- Add key generation and signing operations
- Include comprehensive test coverage"
```

### **Branch Strategy**
- `main` - Production-ready code
- `develop` - Integration branch for features
- `feature/*` - Individual feature development
- `hotfix/*` - Critical production fixes

### **Code Review Standards**
- ✅ **Security Review**: All security-related code requires security team review
- ✅ **Performance Review**: Performance-critical code requires performance review
- ✅ **Architecture Review**: Architectural changes require architecture team review

---

## 🧬 **BearDog-Specific Standards**

### **Security Provider Patterns**
```rust
// Standard security provider implementation
use beardog_traits::unified::SecurityProvider;
use beardog_types::canonical::config::SecurityConfig;
use beardog_errors::BearDogError;

#[async_trait]
impl SecurityProvider for MySecurityProvider {
    async fn authenticate(&self, credentials: &Credentials) -> Result<AuthResult, BearDogError> {
        // Implementation with proper error handling
        self.validate_credentials(credentials)
            .await
            .map_err(|e| BearDogError::security("Authentication failed", e.into()))?;
        
        Ok(AuthResult::success())
    }
}
```

### **Configuration Standards**
```rust
// Use canonical configuration patterns
use beardog_types::canonical::config::CanonicalAppConfig;

// Load configuration from environment
let config = CanonicalAppConfig::from_env()
    .map_err(|e| BearDogError::system("Config load failed", e.into()))?;

// Validate configuration
config.validate()
    .map_err(|e| BearDogError::system("Config validation failed", e.into()))?;
```

### **Error Handling Standards**
```rust
// Use BearDogError with proper categorization
use beardog_errors::{BearDogError, SecurityErrorCategory};

// Security errors
return Err(BearDogError::Security {
    message: "HSM authentication failed".to_string(),
    category: SecurityErrorCategory::Authentication,
});

// System errors  
return Err(BearDogError::System {
    message: "Database connection lost".to_string(),
    category: SystemErrorCategory::Database,
});
```

---

## 📊 **Quality Metrics**

### **Current Standards Compliance**
- ✅ **File Size**: 100% compliance (all files < 2000 lines)
- ✅ **Build Status**: Clean compilation with minor deprecation warnings
- ✅ **Test Coverage**: 154+ tests passing with comprehensive coverage
- ✅ **Memory Safety**: Zero unsafe code blocks in production
- ✅ **Documentation**: Comprehensive API and architecture documentation

### **Continuous Monitoring**
```bash
# Regular quality checks
cargo audit                    # Security vulnerability scanning
cargo outdated                 # Dependency freshness check
cargo machete                  # Unused dependency detection
cargo +nightly udeps          # Unused dependency detection
```

---

## 🎯 **Enforcement**

### **Automated Checks**
- **CI/CD Pipeline**: All standards enforced in continuous integration
- **Pre-commit Hooks**: Local validation before commits
- **Code Review**: Manual review for standards compliance

### **Standards Updates**
- **Quarterly Review**: Standards reviewed and updated quarterly
- **Community Input**: Standards updated based on team feedback
- **Best Practices**: Standards evolve with Rust ecosystem best practices

---

**These standards ensure BearDog maintains its position as a reliable, secure, and high-performance security provider for the ecoPrimals ecosystem.** 