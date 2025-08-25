# 🛠️ BearDog Development Guidelines - Post-Modernization

**Version**: 2.1.0+  
**Status**: Production Ready  
**Last Updated**: January 2025  

---

## 🎯 **Core Principles**

### **1. Canonical Types First**
Always use types from `beardog_types::canonical` for new development:

```rust
// ✅ CORRECT - Use canonical types
use beardog_types::canonical::{
    configuration::DatabaseConfig,
    crypto::KeyType,
    providers::ProviderConfig,
    security::SecurityLevel,
};

// ❌ AVOID - Legacy types (deprecated)
use beardog_types::config::DatabaseConfig;  // Will be removed in v3.0
```

### **2. File Size Management**
Keep all files under **2000 lines**:

```bash
# Check file sizes regularly
find crates/ -name "*.rs" -exec wc -l {} + | sort -rn | head -10

# Current largest: 950 lines (well under limit)
```

### **3. Error Handling Standards**
Use proper `Result<>` patterns, avoid `unwrap()`:

```rust
// ✅ CORRECT - Proper error handling
match operation() {
    Ok(value) => handle_success(value),
    Err(e) => return Err(BearDogError::from(e)),
}

// ✅ ACCEPTABLE - Safe unwrap with justification
let config = CONFIG.get().expect("Config initialized at startup");

// ❌ AVOID - Unsafe unwrap
let value = risky_operation().unwrap();  // Could panic!
```

---

## 📋 **Development Checklist**

### **Before Adding New Code**
- [ ] Import types from `beardog_types::canonical`
- [ ] Check file size will stay under 2000 lines
- [ ] Plan error handling strategy
- [ ] Consider provider interface patterns

### **Before Committing**
- [ ] Run `cargo check --workspace`
- [ ] Run `cargo test --workspace`
- [ ] Check no new TODO/FIXME items added
- [ ] Verify no new unwrap patterns introduced

### **Code Review Focus**
- [ ] Canonical types used consistently
- [ ] Error handling follows patterns
- [ ] File organization maintained
- [ ] Documentation updated

---

## 🏗️ **Architecture Patterns**

### **Provider Interface Pattern**
When adding new providers, follow the established pattern:

```rust
use beardog_types::canonical::providers::{ProviderConfig, ProviderCapabilities};

pub struct MyProvider {
    config: ProviderConfig,
    capabilities: ProviderCapabilities,
}

#[async_trait]
impl Provider for MyProvider {
    async fn initialize(&mut self) -> BearDogResult<()> {
        // Implementation
    }
    
    async fn capabilities(&self) -> &ProviderCapabilities {
        &self.capabilities
    }
}
```

### **Configuration Pattern**
Use canonical configuration types:

```rust
use beardog_types::canonical::configuration::*;

pub fn create_config() -> DatabaseConfig {
    DatabaseConfig {
        url: "postgresql://localhost/beardog".to_string(),
        max_connections: 100,
        // ... other fields
    }
}
```

### **Error Handling Pattern**
Consistent error propagation:

```rust
use beardog_errors::{BearDogError, BearDogResult};

pub async fn secure_operation() -> BearDogResult<String> {
    let provider = get_provider()
        .map_err(|e| BearDogError::provider_error("Failed to get provider", e))?;
    
    let result = provider.perform_operation().await
        .map_err(|e| BearDogError::operation_failed("Secure operation failed", e))?;
    
    Ok(result)
}
```

---

## 🚫 **Anti-Patterns to Avoid**

### **Type Duplication**
```rust
// ❌ DON'T create duplicate types
pub struct MyDatabaseConfig {  // Duplicates canonical::DatabaseConfig
    pub url: String,
    // ...
}

// ✅ DO use canonical types
use beardog_types::canonical::configuration::DatabaseConfig;
```

### **Large Files**
```rust
// ❌ DON'T let files grow too large
// If approaching 2000 lines, split into modules:

// lib.rs
pub mod handlers;
pub mod types;
pub mod utils;

// handlers.rs (focused functionality)
// types.rs (type definitions)
// utils.rs (helper functions)
```

### **Unsafe Unwrapping**
```rust
// ❌ DON'T use unsafe unwraps
let value = risky_call().unwrap();

// ✅ DO handle errors properly
let value = risky_call()
    .map_err(|e| BearDogError::internal("Operation failed"))?;
```

---

## 🔄 **Migration Guidelines (v3.0)**

### **Preparing for Legacy Cleanup**
When v3.0 arrives, deprecated types will be removed:

```rust
// Current (v2.1+): Both work, canonical preferred
use beardog_types::canonical::DatabaseConfig;  // ✅ Future-proof
use beardog_types::config::DatabaseConfig;     // ⚠️ Will be removed

// Future (v3.0): Only canonical types available
use beardog_types::canonical::DatabaseConfig;  // ✅ Only option
```

### **Automated Migration Tools**
```bash
# Find deprecated imports
grep -r "use beardog_types::config::" crates/

# Find remaining unwrap patterns
grep -r "\.unwrap()" crates/ --include="*.rs"

# Check file sizes
find crates/ -name "*.rs" -exec wc -l {} + | sort -rn | head -10
```

---

## 📊 **Monitoring & Maintenance**

### **Regular Health Checks**
```bash
# Weekly checks
cargo check --workspace          # Compilation health
cargo test --workspace           # Test health
grep -r "TODO\|FIXME" crates/   # Technical debt
find crates/ -name "*.rs" -exec wc -l {} + | sort -rn | head -5  # File sizes
```

### **Quality Gates**
- **File Size**: No file > 2000 lines
- **Technical Debt**: Keep TODO/FIXME < 5 items
- **Compilation**: Always clean workspace build
- **Tests**: 100% test suite passing
- **Types**: Canonical types for all new code

---

## 🎯 **Success Metrics**

### **Current Baselines (Maintain or Improve)**
- **Files**: 754 Rust files
- **Max Size**: 950 lines (target: < 2000)
- **Debt**: 1 TODO item (target: < 5)
- **Compilation**: Clean success
- **Tests**: 100% passing

### **Quality Indicators**
- ✅ All new code uses canonical types
- ✅ No new unwrap patterns without justification
- ✅ File sizes remain manageable
- ✅ Error handling follows patterns
- ✅ Provider interfaces properly implemented

---

## 🚀 **Future Roadmap**

### **v2.2 (Near Term)**
- Expand provider ecosystem
- Performance optimizations
- Documentation improvements

### **v3.0 (Major)**
- Remove all deprecated types
- Clean final deprecation warnings
- Complete canonical migration

### **v3.1+ (Long Term)**
- Advanced provider features
- Enhanced security capabilities
- Performance benchmarking

---

**Remember**: The modernization success was achieved through disciplined adherence to these principles. Maintaining this excellence requires continued vigilance and commitment to quality! 🏆 