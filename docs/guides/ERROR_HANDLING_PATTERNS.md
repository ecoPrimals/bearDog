# 🛡️ Error Handling Patterns - BearDog Production Standards

**Purpose**: Eliminate all 429 production unwraps with proper error handling  
**Status**: Implementation guide for Week 1-3  
**Goal**: 0 unwraps in production code

---

## ❌ ANTI-PATTERNS (DO NOT USE)

### 1. Naked unwrap() - CRASH RISK!
```rust
// ❌ BAD - Will panic in production!
let value = map.get(&key).unwrap();
let config = load_config().unwrap();
let provider = registry.get("hsm").unwrap();
```

### 2. Expect without context
```rust
// ❌ BAD - Unhelpful panic message
let value = map.get(&key).expect("failed");
```

### 3. Unwrap in library code
```rust
// ❌ BAD - Libraries should never panic
pub fn process(input: &str) -> String {
    let parsed = serde_json::from_str(input).unwrap();
    // ...
}
```

---

## ✅ CORRECT PATTERNS (USE THESE)

### Pattern 1: Propagate with ? operator
```rust
// ✅ GOOD - Propagate error to caller
pub fn load_hsm_config(path: &str) -> Result<HsmConfig, BearDogError> {
    let contents = std::fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}
```

### Pattern 2: Add context with .context()
```rust
use anyhow::Context;

// ✅ GOOD - Provide helpful context
pub fn get_provider(id: &str) -> Result<Provider, BearDogError> {
    providers.get(id)
        .cloned()
        .context(format!("Provider '{}' not found in registry", id))?
}
```

### Pattern 3: Provide safe defaults with unwrap_or_default()
```rust
// ✅ GOOD - Safe fallback for optional values
let timeout = config.timeout_ms.unwrap_or(5000);
let max_retries = config.max_retries.unwrap_or_default(); // 0 for numeric types
```

### Pattern 4: Handle Option with ok_or_else()
```rust
// ✅ GOOD - Convert Option to Result with error
let value = map.get(&key)
    .ok_or_else(|| BearDogError::not_found(format!("Key '{}' not in map", key)))?;
```

### Pattern 5: Custom error types
```rust
// ✅ GOOD - Specific error types for different failures
pub enum ConfigError {
    NotFound { path: String },
    ParseError { message: String },
    ValidationError { field: String, reason: String },
}

impl From<ConfigError> for BearDogError {
    fn from(err: ConfigError) -> Self {
        match err {
            ConfigError::NotFound { path } => 
                BearDogError::not_found(format!("Config file not found: {}", path)),
            // ...
        }
    }
}
```

### Pattern 6: Early return for validation
```rust
// ✅ GOOD - Validate and return early
pub fn validate_key_id(key_id: &str) -> Result<(), BearDogError> {
    if key_id.is_empty() {
        return Err(BearDogError::invalid_input("Key ID cannot be empty"));
    }
    if key_id.len() > 255 {
        return Err(BearDogError::invalid_input("Key ID too long (max 255 chars)"));
    }
    Ok(())
}
```

---

## 🔧 MIGRATION STRATEGY

### Step 1: Identify Risk Level
```bash
# Find all unwraps in production code (not tests)
grep -r "\.unwrap()" crates/ | grep -v test | grep -v "tests/"

# Categorize by risk:
# - CRITICAL: In error paths, initialization, public APIs
# - HIGH: In core logic, crypto operations
# - MEDIUM: In utility functions
# - LOW: In logging, debug code
```

### Step 2: Fix by Priority
1. **Week 1**: Fix 50 CRITICAL unwraps
2. **Week 2**: Fix 100 HIGH unwraps
3. **Week 3**: Fix remaining 279 unwraps

### Step 3: Replace Pattern
```rust
// Before (CRASH RISK!)
let config = SoftwareHsmConfig::default();
let software_hsm = RustSoftwareHsm::new(config).await.unwrap();

// After (SAFE)
let config = SoftwareHsmConfig::default();
let software_hsm = RustSoftwareHsm::new(config)
    .await
    .context("Failed to initialize software HSM")?;
```

---

## 📝 COMMON SCENARIOS

### Scenario 1: HashMap/Map Access
```rust
// ❌ Before
let provider = self.providers.get(&id).unwrap();

// ✅ After
let provider = self.providers.get(&id)
    .ok_or_else(|| BearDogError::not_found(
        format!("Provider '{}' not registered", id)
    ))?;
```

### Scenario 2: Configuration Loading
```rust
// ❌ Before
let config = std::fs::read_to_string(path).unwrap();
let parsed = toml::from_str(&config).unwrap();

// ✅ After
let config = std::fs::read_to_string(path)
    .context(format!("Failed to read config file: {}", path))?;
let parsed = toml::from_str(&config)
    .context("Failed to parse TOML configuration")?;
```

### Scenario 3: Async Operations
```rust
// ❌ Before
let hsm = RustSoftwareHsm::new(config).await.unwrap();

// ✅ After
let hsm = RustSoftwareHsm::new(config)
    .await
    .context("Failed to initialize HSM")?;
```

### Scenario 4: Array/Vec Access
```rust
// ❌ Before
let first = list.first().unwrap();

// ✅ After - Option 1: Return Result
let first = list.first()
    .ok_or_else(|| BearDogError::invalid_state("List is empty"))?;

// ✅ After - Option 2: Safe default
let first = list.first().copied().unwrap_or_default();
```

### Scenario 5: Lock/Mutex Access
```rust
// ❌ Before
let providers = self.providers.lock().unwrap();

// ✅ After
let providers = self.providers.lock()
    .map_err(|e| BearDogError::internal(
        format!("Mutex poisoned: {}", e)
    ))?;
```

### Scenario 6: Testing (Acceptable)
```rust
// ✅ OK in tests only!
#[tokio::test]
async fn test_hsm_initialization() {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await.unwrap(); // OK in tests
    
    assert!(hsm.is_initialized());
}
```

---

## 🎯 WEEK 1 TARGETS

### Files to Fix (Top Priority):
1. **beardog-tunnel/src/tunnel/hsm/unified_provider.rs** (19 unwraps)
   - Provider registration
   - Provider lookup
   - Default provider access

2. **beardog-tunnel/src/tunnel/hsm/software_hsm/types.rs** (18 unwraps)
   - Backend operations
   - Encryption/decryption
   - Storage access

3. **beardog-core/src/zero_knowledge_bootstrap/** (16 unwraps)
   - Bootstrap initialization
   - Service discovery
   - Capability registration

4. **beardog-types/src/canonical/config/** (12 unwraps)
   - Configuration loading
   - Validation
   - Type conversion

### Success Metrics:
- [ ] 50 unwraps fixed → 379 remaining
- [ ] 0 new unwraps introduced
- [ ] All tests still passing
- [ ] Better error messages in logs

---

## 🔍 VERIFICATION

### Check Remaining Unwraps:
```bash
# Production unwraps
grep -r "\.unwrap()" crates/ | grep -v test | wc -l

# Test unwraps (acceptable)
grep -r "\.unwrap()" crates/ | grep test | wc -l
```

### Test Error Handling:
```bash
# Run tests to ensure errors are properly handled
cargo test --all

# Check for panic in logs
cargo test 2>&1 | grep -i "panic\|unwrap"
```

---

## 📚 ADDITIONAL RESOURCES

### Error Handling Libraries:
- `anyhow` - Easy error handling with context
- `thiserror` - Derive macros for error types
- `eyre` - Alternative to anyhow with more features

### BearDog Error Types:
```rust
// Use these from beardog-errors crate
BearDogError::not_found(message)
BearDogError::invalid_input(message)
BearDogError::unavailable(message)
BearDogError::internal(message)
BearDogError::timeout(message)
BearDogError::initialization(message)
```

---

## ✅ CHECKLIST FOR EACH FIX

- [ ] Identified unwrap location and context
- [ ] Determined appropriate error type
- [ ] Added helpful error message
- [ ] Used ? operator for propagation
- [ ] Added .context() for additional info
- [ ] Updated function signature if needed
- [ ] Tested error path
- [ ] Verified no panic possible
- [ ] Documented error in API docs

---

**Status**: Pattern guide complete  
**Next**: Apply patterns to fix top 50 unwraps  
**Timeline**: 50 unwraps in Week 1, 429 total by Week 3

🐻 **Let's build crash-resistant production code!** 🔐

