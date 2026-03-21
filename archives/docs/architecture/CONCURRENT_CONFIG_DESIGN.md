# 🔒 Concurrent-Safe Configuration Architecture

**Date**: November 7, 2025  
**Status**: Design → Implementation  
**Priority**: High (Technical Debt Elimination)

---

## 🎯 Problem Statement

Current configuration system has **race conditions** due to:
1. `Default` trait reading from environment variables (global mutable state)
2. Tests modifying environment variables with `std::env::set_var()`
3. Tests failing when run in parallel (`cargo test` without `--test-threads=1`)

**This is a technical debt issue and anti-pattern in concurrent systems.**

---

## 🏗️ Modern Architecture

### Principle: **Separate Static Defaults from Dynamic Loading**

```rust
// ❌ ANTI-PATTERN: Default reads global state
impl Default for Config {
    fn default() -> Self {
        Self {
            value: env::var("VALUE").unwrap_or(...),  // WRONG!
        }
    }
}

// ✅ MODERN PATTERN: Default is static, loading is explicit
impl Default for Config {
    fn default() -> Self {
        Self::const_defaults()  // Pure, no side effects
    }
}

impl Config {
    pub fn from_env() -> Self {
        // Explicit environment loading
    }
    
    pub fn builder() -> ConfigBuilder {
        // Flexible construction
    }
}
```

---

## 📐 Design Goals

1. **Concurrent-Safe**: No race conditions, ever
2. **Testable**: Tests use explicit values, not env vars
3. **Explicit**: Clear when environment is read
4. **Backward Compatible**: Existing code continues to work
5. **Zero Cost**: No runtime overhead
6. **Idiomatic Rust**: Follows ecosystem best practices

---

## 🔧 Implementation Strategy

### Phase 1: Refactor Configuration Loading

```rust
pub struct TimeoutConfig {
    pub health_check_secs: u64,
    // ... other fields
}

impl TimeoutConfig {
    /// Pure static defaults (no environment reads)
    pub const fn const_defaults() -> Self {
        Self {
            health_check_secs: 5,
            hsm_operation_secs: 2,
            hsm_probe_millis: 500,
            discovery_operation_secs: 10,
        }
    }
    
    /// Load from environment with fallback to defaults
    pub fn from_env() -> Self {
        Self {
            health_check_secs: env::var("BEARDOG_HEALTH_CHECK_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::const_defaults().health_check_secs),
            // ... other fields
        }
    }
    
    /// Builder for flexible construction
    pub fn builder() -> TimeoutConfigBuilder {
        TimeoutConfigBuilder::new()
    }
}

// Default uses static values (no env reads)
impl Default for TimeoutConfig {
    fn default() -> Self {
        Self::const_defaults()
    }
}
```

### Phase 2: Builder Pattern for Flexibility

```rust
pub struct TimeoutConfigBuilder {
    health_check_secs: Option<u64>,
    hsm_operation_secs: Option<u64>,
    // ... other fields
}

impl TimeoutConfigBuilder {
    pub fn new() -> Self {
        Self {
            health_check_secs: None,
            hsm_operation_secs: None,
            // ...
        }
    }
    
    pub fn health_check_secs(mut self, secs: u64) -> Self {
        self.health_check_secs = Some(secs);
        self
    }
    
    pub fn from_env(mut self) -> Self {
        if let Ok(val) = env::var("BEARDOG_HEALTH_CHECK_TIMEOUT_SECS") {
            if let Ok(secs) = val.parse() {
                self.health_check_secs = Some(secs);
            }
        }
        // ... other fields
        self
    }
    
    pub fn build(self) -> TimeoutConfig {
        let defaults = TimeoutConfig::const_defaults();
        TimeoutConfig {
            health_check_secs: self.health_check_secs
                .unwrap_or(defaults.health_check_secs),
            hsm_operation_secs: self.hsm_operation_secs
                .unwrap_or(defaults.hsm_operation_secs),
            // ...
        }
    }
}
```

### Phase 3: Update ConfigLoader

```rust
impl ConfigLoader {
    pub fn with_env_vars(mut self) -> Self {
        // Explicitly load from environment
        self.config.timeouts = TimeoutConfig::from_env();
        self.config.network = NetworkConfig::from_env();
        // ... other domains
        self
    }
}
```

### Phase 4: Concurrent-Safe Testing

```rust
#[test]
fn test_timeout_custom_values() {
    // ✅ NO environment variable modification
    let config = TimeoutConfig::builder()
        .health_check_secs(15)
        .hsm_operation_secs(3)
        .build();
    
    assert_eq!(config.health_check_secs, 15);
    // No race conditions, can run in parallel!
}

#[test]
fn test_timeout_from_env() {
    // If we MUST test env loading, use serial_test crate
    #[serial]
    fn inner_test() {
        env::set_var("BEARDOG_HEALTH_CHECK_TIMEOUT_SECS", "10");
        let config = TimeoutConfig::from_env();
        assert_eq!(config.health_check_secs, 10);
        env::remove_var("BEARDOG_HEALTH_CHECK_TIMEOUT_SECS");
    }
    inner_test();
}
```

---

## 🎯 Benefits

### Concurrency
- ✅ No race conditions
- ✅ Tests run in parallel safely
- ✅ Thread-safe by design

### Testability
- ✅ No environment variable pollution
- ✅ Explicit value injection
- ✅ Deterministic tests

### Clarity
- ✅ Explicit when environment is read
- ✅ Clear separation of concerns
- ✅ Self-documenting code

### Performance
- ✅ Zero overhead (const defaults)
- ✅ No unnecessary env reads
- ✅ Lazy loading when needed

---

## 📊 Migration Path

### Step 1: Add New API (Non-Breaking)
```rust
// Keep existing Default (for now)
impl Default for TimeoutConfig { ... }

// Add new explicit API
impl TimeoutConfig {
    pub const fn const_defaults() -> Self { ... }
    pub fn from_env() -> Self { ... }
    pub fn builder() -> TimeoutConfigBuilder { ... }
}
```

### Step 2: Update ConfigLoader
```rust
impl ConfigLoader {
    pub fn with_env_vars(mut self) -> Self {
        // Change from relying on Default to explicit loading
        self.config.timeouts = TimeoutConfig::from_env();
        self
    }
}
```

### Step 3: Update Tests
```rust
// Old (uses env vars)
std::env::set_var("KEY", "value");
let config = TimeoutConfig::default();  // Reads env

// New (explicit)
let config = TimeoutConfig::builder()
    .health_check_secs(10)
    .build();  // No env pollution
```

### Step 4: Deprecate Old Pattern (Optional)
```rust
#[deprecated(since = "0.2.0", note = "Use TimeoutConfig::from_env() or builder()")]
impl Default for TimeoutConfig { ... }
```

---

## 🔬 Testing Strategy

### Unit Tests: Pure Values
```rust
#[test]
fn test_const_defaults() {
    let config = TimeoutConfig::const_defaults();
    assert_eq!(config.health_check_secs, 5);
}

#[test]
fn test_builder() {
    let config = TimeoutConfig::builder()
        .health_check_secs(10)
        .build();
    assert_eq!(config.health_check_secs, 10);
}
```

### Integration Tests: Explicit Loading
```rust
#[test]
fn test_full_config_load() {
    // Load with explicit environment reading
    let config = BearDogConfig::builder()
        .with_env_vars()  // Explicit!
        .build()?;
    
    // Test without modifying global state
}
```

### Env Var Tests: Serial When Needed
```rust
use serial_test::serial;

#[test]
#[serial]  // Mark explicitly as serial
fn test_env_var_override() {
    env::set_var("KEY", "value");
    let config = TimeoutConfig::from_env();
    env::remove_var("KEY");
    assert_eq!(config.value, "value");
}
```

---

## 🎓 Modern Rust Patterns Used

1. **Builder Pattern**: Flexible construction
2. **Explicit Loading**: Clear side effects
3. **Const Functions**: Compile-time defaults
4. **Type State**: Enforce correctness at compile time
5. **Dependency Injection**: Testability without mocks

---

## 📚 References

- [Rust API Guidelines - Const Functions](https://rust-lang.github.io/api-guidelines/)
- [tokio::test with serial execution](https://docs.rs/serial_test/)
- [Builder Pattern in Rust](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
- [Effective Rust - Item 15: Avoid global state](https://www.lurklurk.org/effective-rust/)

---

## ✅ Success Criteria

- [ ] All tests pass in parallel (`cargo test` without `--test-threads=1`)
- [ ] No `env::set_var()` in tests (except explicitly serial ones)
- [ ] Clear separation: `Default` vs `from_env()` vs `builder()`
- [ ] Backward compatible (existing code works)
- [ ] Documentation updated
- [ ] All config domains refactored

---

**Next**: Implement this design across all config domains!

🐻 **BearDog: Concurrent-Safe by Design** 🔒

