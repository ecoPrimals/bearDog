# 🚀 BearDog Unification Quick Reference Guide

**For**: Daily development and code review  
**Purpose**: Enforce unification patterns and prevent fragmentation  
**Status**: Living document - update as patterns evolve

---

## ⚡ **QUICK RULES**

### **The Five Golden Rules**
1. **Types** → `beardog-types/src/canonical/`
2. **Configs** → `beardog-types/src/canonical/config/domains/`
3. **Constants** → `beardog-types/src/constants/domains/`
4. **Errors** → `Result<T, BearDogError>` (NO type aliases!)
5. **File Size** → Max 2000 lines (prefer <1000)

---

## 📦 **IMPORT PATTERNS**

### **✅ CORRECT Imports**
```rust
// Types
use beardog_types::canonical::{
    UnifiedBearDogConfig,
    HealthStatus,
    SecurityContext,
};

// Configs
use beardog_types::canonical::config::{
    NetworkConfig,
    SecuritySettings,
    HsmConfig,
};

// Constants
use beardog_types::constants::domains::{
    network::DEFAULT_TIMEOUT,
    buffers::BUFFER_SIZE_MEDIUM,
};

// Errors
use beardog_errors::BearDogError;
fn my_function() -> Result<Data, BearDogError> { ... }
```

### **❌ INCORRECT Imports (Anti-patterns)**
```rust
// ❌ Using type aliases (deprecated)
use beardog_types::BearDogResult;
fn my_function() -> BearDogResult<Data> { ... }  // NO!

// ❌ Importing from non-canonical locations
use beardog_networking::config::NetworkConfig;  // NO! Use canonical

// ❌ Local config definitions
pub struct MyConfig { ... }  // NO! Put in canonical location

// ❌ Duplicate constants
const DEFAULT_TIMEOUT: u64 = 30;  // NO! Use from constants module
```

---

## 🎨 **CODE PATTERNS**

### **Error Handling (Idiomatic Rust)**

#### **✅ CORRECT Pattern**
```rust
use beardog_errors::{BearDogError, ResultExt};

fn load_config() -> Result<Config, BearDogError> {
    let content = std::fs::read_to_string("config.toml")
        .system_context("Failed to load configuration")?;
    
    let config: Config = toml::from_str(&content)
        .system_context("Failed to parse configuration")?;
    
    Ok(config)
}
```

#### **❌ INCORRECT Patterns**
```rust
// ❌ Type alias (deprecated)
fn load_config() -> BearDogResult<Config> { ... }

// ❌ Unwrap (panics!)
fn load_config() -> Result<Config, BearDogError> {
    let content = std::fs::read_to_string("config.toml").unwrap();  // NO!
    // ...
}

// ❌ No context
fn load_config() -> Result<Config, BearDogError> {
    std::fs::read_to_string("config.toml")?;  // NO! Add context
    // ...
}
```

---

### **Async Traits (Native, Zero-Cost)**

#### **✅ CORRECT Pattern**
```rust
// Modern Rust - zero overhead
pub trait DataProcessor {
    fn process(&self, data: Data) -> impl Future<Output = Result<Output, BearDogError>> + Send;
}

impl DataProcessor for MyProcessor {
    fn process(&self, data: Data) -> impl Future<Output = Result<Output, BearDogError>> + Send {
        async move {
            // Your async code here
            Ok(output)
        }
    }
}
```

#### **❌ INCORRECT Pattern**
```rust
// ❌ Old pattern with async_trait (5-15% overhead)
use async_trait::async_trait;

#[async_trait]
pub trait DataProcessor {
    async fn process(&self, data: Data) -> Result<Output, BearDogError>;
}
```

---

### **Configuration (Unified System)**

#### **✅ CORRECT Pattern**
```rust
use beardog_types::canonical::config::UnifiedBearDogConfig;

// Load configuration
fn load() -> Result<UnifiedBearDogConfig, BearDogError> {
    UnifiedBearDogConfig::from_env()
}

// Access specific configs
fn use_config(config: &UnifiedBearDogConfig) {
    println!("Port: {}", config.network.port);
    println!("HSM enabled: {}", config.hsm.enabled);
}
```

#### **❌ INCORRECT Patterns**
```rust
// ❌ Defining local config (creates fragmentation)
pub struct MyNetworkConfig {
    pub port: u16,
    pub timeout: u64,
}

// ❌ Loading from multiple sources
let app_config = AppConfig::load()?;
let network_config = NetworkConfig::load()?;
let security_config = SecurityConfig::load()?;
// Too fragmented!
```

---

### **Constants (Domain-Organized)**

#### **✅ CORRECT Pattern**
```rust
use beardog_types::constants::domains::{
    network::{DEFAULT_TIMEOUT, MAX_RETRIES},
    buffers::{BUFFER_SIZE_MEDIUM, BUFFER_SIZE_LARGE},
    security::{MIN_KEY_SIZE, ENTROPY_BYTES},
};

fn setup() {
    let timeout = DEFAULT_TIMEOUT;
    let buffer = vec![0u8; BUFFER_SIZE_MEDIUM];
}
```

#### **❌ INCORRECT Patterns**
```rust
// ❌ Local constants (duplication)
const DEFAULT_TIMEOUT: u64 = 30;  // NO! Use from constants module
const BUFFER_SIZE: usize = 4096;  // NO! Use canonical constant

// ❌ Magic numbers
fn setup() {
    let timeout = 30;     // NO! Use constant
    let buffer = vec![0u8; 4096];  // NO! Use constant
}
```

---

## 🔍 **CODE REVIEW CHECKLIST**

### **Before Committing**
- [ ] All imports are from canonical locations
- [ ] No `BearDogResult<T>` type aliases
- [ ] No `#[async_trait]` on new traits
- [ ] No magic numbers (use constants)
- [ ] No duplicate config definitions
- [ ] All files < 2000 lines
- [ ] Error context added with `.context()`
- [ ] Tests included

### **When Adding New Code**
- [ ] Is this type already in canonical? (Check first!)
- [ ] Is this config already defined? (No duplicates!)
- [ ] Is this constant already defined? (No duplication!)
- [ ] Should this be in canonical location? (If reusable: YES!)

---

## 🛠️ **COMMON MIGRATIONS**

### **Migrate Type Alias → Idiomatic Result**
```bash
# Before:
fn my_func() -> BearDogResult<Data>

# After:
fn my_func() -> Result<Data, BearDogError>

# Find and replace:
sed -i 's/-> BearDogResult</-> Result</g' file.rs
sed -i 's/: BearDogResult</: Result</g' file.rs
```

### **Migrate async_trait → Native Async**
```rust
// Before:
#[async_trait]
pub trait MyTrait {
    async fn method(&self) -> Result<T, E>;
}

// After:
pub trait MyTrait {
    fn method(&self) -> impl Future<Output = Result<T, E>> + Send;
}

// Implementation:
impl MyTrait for MyStruct {
    fn method(&self) -> impl Future<Output = Result<T, E>> + Send {
        async move {
            // implementation
        }
    }
}
```

### **Move Config to Canonical**
```bash
# 1. Identify current location
grep -rn "pub struct MyConfig" crates

# 2. Choose canonical location
# beardog-types/src/canonical/config/domains/my_domain.rs

# 3. Move definition
# (manually move the struct)

# 4. Update imports
sed -i 's/use my_crate::config::MyConfig/use beardog_types::canonical::config::MyConfig/g' crates/**/*.rs

# 5. Remove old definition

# 6. Test
cargo check --workspace
```

---

## 📊 **VERIFICATION COMMANDS**

### **Check for Violations**
```bash
# Find BearDogResult usage (should be 0 in production)
grep -r "BearDogResult" crates --include="*.rs" | grep -v "deprecated" | grep -v "test"

# Find async_trait usage (should be 0)
grep -r "#\[async_trait\]" crates --include="*.rs"

# Find large files (all should be <2000 lines)
find crates -name "*.rs" -exec wc -l {} \; | sort -rn | head -20

# Find magic numbers (should use constants)
grep -rn "= [0-9]\{3,\}" crates --include="*.rs" | grep -v "test" | head

# Find potential duplicate configs
grep -r "pub struct.*Config" crates --include="*.rs" | \
    awk -F: '{print $2}' | sort | uniq -c | sort -rn | head -20
```

### **Track Progress**
```bash
# Run the progress dashboard
./scripts/track_unification_progress.sh

# Expected output:
# Type System: 100% (0 violations)
# Config System: 95% canonical
# Legacy Code: <50 files
# Overall: 98%+
```

---

## 🎯 **FILE ORGANIZATION**

### **Canonical Structure**
```
crates/beardog-types/src/
├── canonical/               # ← ALL types go here
│   ├── mod.rs              # Central re-exports
│   ├── config/             # ← ALL configs
│   │   ├── mod.rs
│   │   ├── app.rs
│   │   ├── network.rs
│   │   ├── security.rs
│   │   └── domains/        # Specialized configs
│   │       ├── adapter.rs
│   │       ├── discovery.rs
│   │       └── workflow.rs
│   ├── providers.rs        # Provider traits & types
│   ├── security.rs         # Security types
│   ├── hsm/                # HSM-specific types
│   └── ...
├── constants/              # ← ALL constants
│   ├── mod.rs
│   └── domains/
│       ├── network.rs      # Network constants
│       ├── buffers.rs      # Buffer sizes
│       ├── security.rs     # Security constants
│       └── ...
└── unified_types.rs        # Type aliases (mostly deprecated)
```

---

## 💡 **PATTERNS TO ADOPT**

### **Arc<[T]> over Arc<Vec<T>>**
```rust
// ✅ BETTER (immutable, efficient)
let data: Arc<[u8]> = Arc::from(vec![1, 2, 3]);

// ❌ WORSE (unnecessary indirection)
let data: Arc<Vec<u8>> = Arc::new(vec![1, 2, 3]);
```

### **Cow<'_, str> for Flexible Strings**
```rust
// ✅ FLEXIBLE (no allocation if borrowed)
fn process(name: impl Into<Cow<'_, str>>) -> String {
    let name: Cow<str> = name.into();
    format!("Hello, {}", name)
}

// Works with both:
process("static");           // No allocation
process(owned_string);        // No extra allocation
```

### **Const Generics for Compile-Time Configuration**
```rust
// ✅ ZERO-COST configuration
pub struct Buffer<const SIZE: usize> {
    data: [u8; SIZE],
}

let small: Buffer<1024> = Buffer::new();    // 1 KB
let large: Buffer<65536> = Buffer::new();   // 64 KB
```

---

## 🚫 **ANTI-PATTERNS TO AVOID**

### **1. Type Aliases for Result**
```rust
// ❌ AVOID
pub type MyResult<T> = Result<T, MyError>;

// ✅ USE DIRECTLY
fn my_func() -> Result<T, MyError> { ... }
```

### **2. Duplicate Config Definitions**
```rust
// ❌ AVOID (creates fragmentation)
// File: crates/my-crate/src/config.rs
pub struct NetworkConfig { ... }

// ✅ USE CANONICAL
use beardog_types::canonical::config::NetworkConfig;
```

### **3. Magic Numbers**
```rust
// ❌ AVOID
let timeout = 30;
let buffer = vec![0u8; 4096];

// ✅ USE CONSTANTS
use beardog_types::constants::domains::{network, buffers};
let timeout = network::DEFAULT_TIMEOUT;
let buffer = vec![0u8; buffers::BUFFER_SIZE_MEDIUM];
```

### **4. Unwrap in Production Code**
```rust
// ❌ AVOID (panics!)
let config = load_config().unwrap();

// ✅ USE PROPER ERROR HANDLING
let config = load_config()
    .context("Failed to load configuration")?;
```

---

## 📚 **LEARNING RESOURCES**

### **Internal Documentation**
- Canonical Type Spec: `specs/current/architecture/CANONICAL_TYPE_SYSTEM_SPECIFICATION.md`
- Error Handling Guide: `specs/current/architecture/IDIOMATIC_ERROR_HANDLING_MIGRATION.md`
- Config Architecture: `docs/guides/CONFIG_ARCHITECTURE_AND_RATIONALE.md`
- This Quick Reference: `UNIFICATION_QUICK_REFERENCE.md`

### **Rust Best Practices**
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Async Traits RFC](https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html)

---

## 🎯 **REMEMBER**

1. **Check canonical first** before creating new types/configs/constants
2. **Use idiomatic Rust** - no unnecessary abstractions
3. **Add context to errors** - make debugging easier
4. **Keep files small** - max 2000 lines, prefer <1000
5. **Follow the patterns** - consistency is key

---

**Last Updated**: November 10, 2025  
**Maintained By**: BearDog Architecture Team  
**Questions**: Review audit and action plan documents

---

**This is a LIVING DOCUMENT** - Update as patterns evolve!

