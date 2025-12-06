# 🧪 Concurrent-Safe Testing Guide for BearDog

**Status**: Production Pattern  
**Established**: November 22, 2025  
**Philosophy**: "Solve debt, don't hide it" - No serial_test crutches

---

## 🎯 WHY CONCURRENT-SAFE TESTING?

### Problems with Traditional Approaches

**❌ BAD: Global State Pollution**
```rust
#[test]
fn test_config_from_env() {
    std::env::set_var("PORT", "8080");
    let config = load_config();
    assert_eq!(config.port, 8080);
    std::env::remove_var("PORT"); // Cleanup might fail!
}
```

**Problems**:
- 🔴 Tests can't run in parallel (race conditions)
- 🔴 Cleanup might not run (if test panics)
- 🔴 Pollutes global environment
- 🔴 Flaky CI builds
- 🔴 Slower test execution

**❌ BAD: serial_test Band-Aid**
```rust
use serial_test::serial;

#[test]
#[serial] // Forces sequential execution
fn test_config_from_env() {
    std::env::set_var("PORT", "8080");
    let config = load_config();
    assert_eq!(config.port, 8080);
}
```

**Problems**:
- 🟡 Hides the problem instead of solving it
- 🟡 Slower CI (no parallelism)
- 🟡 Extra dependency
- 🟡 Doesn't fix the architecture issue

---

## ✅ THE BEARDOG WAY: Dependency Injection

### Pattern: Environment Provider

**Production Code**:
```rust
use std::collections::HashMap;

pub struct Config {
    pub port: u16,
    pub host: String,
    pub timeout: u64,
}

impl Config {
    /// Load configuration from environment variables
    /// 
    /// This is the public API - unchanged for users
    pub fn from_env() -> Self {
        Self::from_env_provider(|key| std::env::var(key).ok())
    }

    /// Load configuration from a custom environment provider
    ///
    /// This enables concurrent-safe testing by accepting any function
    /// that provides environment variable values, without touching global state.
    ///
    /// # Arguments
    ///
    /// * `env_provider` - A function that takes a variable name and returns its value
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut test_env = HashMap::new();
    /// test_env.insert("PORT", "8080");
    ///
    /// let config = Config::from_env_provider(|key| {
    ///     test_env.get(key).map(|s| s.to_string())
    /// });
    ///
    /// assert_eq!(config.port, 8080);
    /// ```
    pub fn from_env_provider<F>(env_provider: F) -> Self
    where
        F: Fn(&str) -> Option<String>,
    {
        let port = env_provider("PORT")
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080);

        let host = env_provider("HOST")
            .unwrap_or_else(|| "localhost".to_string());

        let timeout = env_provider("TIMEOUT")
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);

        Self { port, host, timeout }
    }
}
```

**Test Code**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_config_from_env_with_overrides() {
        // ✅ Zero global state - fully concurrent-safe!
        let mut test_env = HashMap::new();
        test_env.insert("PORT", "9000");
        test_env.insert("HOST", "0.0.0.0");
        test_env.insert("TIMEOUT", "60");

        let config = Config::from_env_provider(|key| {
            test_env.get(key).map(|s| s.to_string())
        });

        assert_eq!(config.port, 9000);
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.timeout, 60);
        
        // No cleanup needed! test_env is local to this test.
    }

    #[test]
    fn test_config_defaults() {
        // ✅ Empty env - tests defaults
        let config = Config::from_env_provider(|_| None);

        assert_eq!(config.port, 8080);
        assert_eq!(config.host, "localhost");
        assert_eq!(config.timeout, 30);
    }

    #[test]
    fn test_config_partial_overrides() {
        // ✅ Can test any combination
        let mut test_env = HashMap::new();
        test_env.insert("PORT", "3000");
        // HOST and TIMEOUT will use defaults

        let config = Config::from_env_provider(|key| {
            test_env.get(key).map(|s| s.to_string())
        });

        assert_eq!(config.port, 3000);
        assert_eq!(config.host, "localhost"); // default
        assert_eq!(config.timeout, 30); // default
    }
}
```

---

## 🏗️ MIGRATION GUIDE

### Step 1: Add from_env_provider() Method

**Before**:
```rust
impl MyConfig {
    pub fn from_env() -> Self {
        Self {
            field1: std::env::var("FIELD1").unwrap_or_default(),
            field2: std::env::var("FIELD2").unwrap_or_default(),
        }
    }
}
```

**After**:
```rust
impl MyConfig {
    // Public API unchanged
    pub fn from_env() -> Self {
        Self::from_env_provider(|key| std::env::var(key).ok())
    }

    // New testable method
    pub fn from_env_provider<F>(env_provider: F) -> Self
    where
        F: Fn(&str) -> Option<String>,
    {
        Self {
            field1: env_provider("FIELD1").unwrap_or_default(),
            field2: env_provider("FIELD2").unwrap_or_default(),
        }
    }
}
```

### Step 2: Update Tests

**Before** (Global state):
```rust
#[test]
fn test_my_config() {
    std::env::set_var("FIELD1", "value1");
    std::env::set_var("FIELD2", "value2");
    
    let config = MyConfig::from_env();
    assert_eq!(config.field1, "value1");
    
    // Cleanup
    std::env::remove_var("FIELD1");
    std::env::remove_var("FIELD2");
}
```

**After** (Concurrent-safe):
```rust
#[test]
fn test_my_config() {
    use std::collections::HashMap;
    
    let mut test_env = HashMap::new();
    test_env.insert("FIELD1", "value1");
    test_env.insert("FIELD2", "value2");
    
    let config = MyConfig::from_env_provider(|key| {
        test_env.get(key).map(|s| s.to_string())
    });
    
    assert_eq!(config.field1, "value1");
    // No cleanup needed!
}
```

---

## 🎨 ADVANCED PATTERNS

### Pattern 1: Test Fixtures

**Create Reusable Test Environments**:
```rust
#[cfg(test)]
mod test_helpers {
    use std::collections::HashMap;

    pub fn production_env() -> HashMap<&'static str, &'static str> {
        let mut env = HashMap::new();
        env.insert("PORT", "8080");
        env.insert("HOST", "0.0.0.0");
        env.insert("TIMEOUT", "30");
        env
    }

    pub fn development_env() -> HashMap<&'static str, &'static str> {
        let mut env = HashMap::new();
        env.insert("PORT", "3000");
        env.insert("HOST", "localhost");
        env.insert("TIMEOUT", "60");
        env
    }

    pub fn minimal_env() -> HashMap<&'static str, &'static str> {
        HashMap::new() // All defaults
    }
}

#[test]
fn test_production_config() {
    let env = test_helpers::production_env();
    let config = Config::from_env_provider(|k| env.get(k).map(|s| s.to_string()));
    assert_eq!(config.port, 8080);
}
```

### Pattern 2: Error Case Testing

```rust
#[test]
fn test_invalid_port_uses_default() {
    let mut test_env = HashMap::new();
    test_env.insert("PORT", "not_a_number");

    let config = Config::from_env_provider(|key| {
        test_env.get(key).map(|s| s.to_string())
    });

    // Should fallback to default when parsing fails
    assert_eq!(config.port, 8080);
}
```

### Pattern 3: Builder Pattern Integration

```rust
impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

pub struct ConfigBuilder {
    env_provider: Box<dyn Fn(&str) -> Option<String>>,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            env_provider: Box::new(|key| std::env::var(key).ok()),
        }
    }
}

impl ConfigBuilder {
    /// Use a custom environment provider (for testing)
    pub fn with_env_provider<F>(mut self, provider: F) -> Self
    where
        F: Fn(&str) -> Option<String> + 'static,
    {
        self.env_provider = Box::new(provider);
        self
    }

    /// Use actual environment variables
    pub fn from_env(self) -> Self {
        self.with_env_provider(|key| std::env::var(key).ok())
    }

    pub fn build(self) -> Config {
        Config::from_env_provider(&self.env_provider)
    }
}

// Usage in tests
#[test]
fn test_builder_pattern() {
    let mut test_env = HashMap::new();
    test_env.insert("PORT", "9000");

    let config = Config::builder()
        .with_env_provider(move |key| test_env.get(key).map(|s| s.to_string()))
        .build();

    assert_eq!(config.port, 9000);
}
```

---

## 📦 WHEN TO USE serial_test

**Reserve for Resource-Constrained Tests ONLY**:

```rust
use serial_test::serial;

// ✅ GOOD: Resource constraint (actual hardware)
#[test]
#[serial]
fn test_physical_usb_device() {
    // Only one test can access USB device at a time
}

// ✅ GOOD: Chaos testing (intentional system disruption)
#[test]
#[serial]
fn chaos_test_high_cpu_load() {
    // Intentionally max out CPU
}

// ✅ GOOD: File system limits
#[test]
#[serial]
fn test_max_open_files() {
    // Opens OS file descriptor limit
}

// ❌ BAD: Environment variables
#[test]
#[serial]  // ← Don't do this!
fn test_config() {
    std::env::set_var("KEY", "value");
    // Use from_env_provider() instead!
}
```

---

## 🏆 BENEFITS

### Performance
- ✅ **Parallel CI**: All tests run concurrently
- ✅ **Faster local dev**: `cargo test` runs at full speed
- ✅ **Scales**: Works on 1 core or 128 cores

### Reliability
- ✅ **Zero flakiness**: No race conditions
- ✅ **Deterministic**: Same result every time
- ✅ **Panic-safe**: No cleanup needed

### Architecture
- ✅ **Testable by design**: Dependency injection
- ✅ **Flexible**: Works with any config source
- ✅ **Future-proof**: Easy to add remote config, etc.

### Team
- ✅ **No `serial_test` dependency**: One less dep
- ✅ **Clear patterns**: Easy to understand
- ✅ **Idiomatic Rust**: Follows language conventions

---

## 📚 REAL EXAMPLES FROM BEARDOG

### Example 1: CapacityConfig

**File**: `crates/beardog-config/src/domains/capacity.rs`

```rust
impl CapacityConfig {
    pub fn from_env() -> Self {
        Self::from_env_provider(|key| std::env::var(key).ok())
    }

    pub fn from_env_provider<F>(env_provider: F) -> Self
    where
        F: Fn(&str) -> Option<String>,
    {
        let mut config = Self::default();

        if let Some(val) = env_provider("BEARDOG_CHANNEL_BUFFER") {
            if let Ok(parsed) = val.parse() {
                config.default_channel_buffer = parsed;
            }
        }

        if let Some(val) = env_provider("BEARDOG_MAX_CONNECTIONS") {
            if let Ok(parsed) = val.parse() {
                config.max_connections = parsed;
            }
        }

        // ... more fields

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_capacity_config_from_env_with_overrides() {
        let mut test_env = HashMap::new();
        test_env.insert("BEARDOG_CHANNEL_BUFFER", "2000");
        test_env.insert("BEARDOG_MAX_CONNECTIONS", "200");

        let config = CapacityConfig::from_env_provider(|key| {
            test_env.get(key).map(|s| s.to_string())
        });

        assert_eq!(config.default_channel_buffer, 2000);
        assert_eq!(config.max_connections, 200);
    }
}
```

### Example 2: TimeoutConfig (Builder Pattern)

**File**: `crates/beardog-config/src/domains/timeouts.rs`

Already concurrent-safe using builder pattern:

```rust
#[test]
fn test_custom_timeouts() {
    let config = TimeoutConfig::builder()
        .health_check_secs(15)
        .hsm_operation_secs(5)
        .discovery_operation_secs(30)
        .build();

    assert_eq!(config.health_check_secs, 15);
    assert_eq!(config.hsm_operation_secs, 5);
}
```

---

## 🎓 COMPARISON

| Aspect | Global Env | serial_test | from_env_provider |
|--------|-----------|-------------|-------------------|
| **Parallel Tests** | ❌ No | ❌ No | ✅ Yes |
| **Zero Flakiness** | ❌ No | ✅ Yes | ✅ Yes |
| **Clean Architecture** | ❌ No | ❌ No | ✅ Yes |
| **Extra Dependencies** | ✅ No | ❌ Yes | ✅ No |
| **Test Speed** | 🟡 Medium | 🟡 Slow | ✅ Fast |
| **Idiomatic Rust** | ❌ No | 🟡 Workaround | ✅ Yes |

---

## 📖 SUMMARY

### The BearDog Testing Philosophy

1. **Solve debt, don't hide it** - Fix architecture, not symptoms
2. **Concurrent-safe by default** - All tests run in parallel
3. **Zero global state** - Use dependency injection
4. **Idiomatic Rust** - Follow language patterns
5. **Reserve serial_test** - Only for hardware/chaos tests

### Quick Reference

**Add this to your config modules**:
```rust
pub fn from_env_provider<F>(env_provider: F) -> Self
where F: Fn(&str) -> Option<String>
```

**Use this in your tests**:
```rust
let mut test_env = HashMap::new();
test_env.insert("KEY", "value");
let config = Config::from_env_provider(|k| test_env.get(k).map(|s| s.to_string()));
```

**Result**:
- ✅ Zero global state
- ✅ Fully concurrent
- ✅ Zero flakiness
- ✅ Faster CI

---

**Established**: November 22, 2025  
**Status**: Production pattern across BearDog  
**Philosophy**: "We don't hide debt with serial_test - we solve it with idiomatic Rust."

🐻🐕 **BearDog: Concurrent-Safe by Design** 🐻🐕

