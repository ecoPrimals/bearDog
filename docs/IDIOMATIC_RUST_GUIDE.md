# 🦀 Idiomatic Rust Guide for BearDog

**Purpose**: Modernize BearDog codebase to follow Rust best practices  
**Date**: November 14, 2025  
**Status**: Living document

---

## 🎯 Core Principles

1. **Make Invalid States Unrepresentable** - Use type system to prevent bugs
2. **Explicit is Better Than Implicit** - Clear, obvious code over clever tricks
3. **Zero-Cost Abstractions** - High-level without performance cost
4. **Fearless Concurrency** - Safe parallelism through ownership
5. **Memory Safety Without GC** - No runtime overhead

---

## ✅ ERROR HANDLING

### Pattern: Never Use unwrap() in Production

```rust
// ❌ BAD - Can panic
fn load_config() -> Config {
    let file = std::fs::read_to_string("config.toml").unwrap();
    toml::from_str(&file).unwrap()
}

// ✅ GOOD - Proper error handling
fn load_config() -> Result<Config, BearDogError> {
    let file = std::fs::read_to_string("config.toml")
        .map_err(|e| BearDogError::io("Failed to read config file", e))?;
    
    toml::from_str(&file)
        .map_err(|e| BearDogError::config("Invalid config format", e))
}

// ✅ BETTER - With context
fn load_config(path: &Path) -> Result<Config, BearDogError> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Reading config from {}", path.display()))?
        .parse()
        .with_context(|| "Parsing config as TOML")
}
```

### Pattern: Custom Error Types

```rust
// ✅ Rich, actionable errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Config file not found at {path}")]
    NotFound { path: PathBuf },
    
    #[error("Invalid port {port}: {reason}")]
    InvalidPort { port: u16, reason: String },
    
    #[error("Missing required field: {field}")]
    MissingField { field: &'static str },
}
```

---

## ✅ TYPE SAFETY

### Pattern: Newtype for Domain Types

```rust
// ❌ BAD - Easy to mix up
fn connect(api_port: u16, admin_port: u16) { ... }

// ✅ GOOD - Type-safe
#[derive(Debug, Clone, Copy)]
pub struct ApiPort(u16);

#[derive(Debug, Clone, Copy)]
pub struct AdminPort(u16);

fn connect(api: ApiPort, admin: AdminPort) { ... }

// Compile error if you mix them up!
connect(admin_port, api_port); // ❌ Won't compile
```

### Pattern: Phantom Types for State Machines

```rust
// ✅ State encoded in types
struct Connection<State> {
    socket: TcpStream,
    state: PhantomData<State>,
}

struct Disconnected;
struct Connected;
struct Authenticated;

impl Connection<Disconnected> {
    fn connect(self) -> Result<Connection<Connected>> { ... }
}

impl Connection<Connected> {
    fn authenticate(self, creds: &Credentials) -> Result<Connection<Authenticated>> { ... }
}

impl Connection<Authenticated> {
    fn send_secure(&self, data: &[u8]) -> Result<()> { ... }
    // Only available in Authenticated state!
}
```

---

## ✅ ZERO-COPY PATTERNS

### Pattern: Use References

```rust
// ❌ BAD - Unnecessary allocation
fn process_name(name: String) -> String {
    name.to_uppercase()
}

// ✅ GOOD - No allocation
fn process_name(name: &str) -> String {
    name.to_uppercase()
}
```

### Pattern: Cow for Conditional Ownership

```rust
use std::borrow::Cow;

// ✅ Borrow when possible, own when necessary
fn ensure_prefix<'a>(input: &'a str, prefix: &str) -> Cow<'a, str> {
    if input.starts_with(prefix) {
        Cow::Borrowed(input)  // No allocation!
    } else {
        Cow::Owned(format!("{}{}", prefix, input))  // Allocate only if needed
    }
}
```

### Pattern: Arc for Shared Ownership

```rust
use std::sync::Arc;

// ✅ Share expensive data cheaply
let config = Arc::new(load_config()?);

// Cloning Arc is cheap (just ref count)
let config_clone = Arc::clone(&config);
spawn_worker(config_clone);
```

---

## ✅ BUILDER PATTERN

```rust
// ✅ Fluent, type-safe configuration
pub struct HsmConfigBuilder {
    provider: Option<Provider>,
    timeout: Duration,
    retry_count: u32,
}

impl HsmConfigBuilder {
    pub fn new() -> Self {
        Self {
            provider: None,
            timeout: Duration::from_secs(30),
            retry_count: 3,
        }
    }
    
    pub fn provider(mut self, provider: Provider) -> Self {
        self.provider = Some(provider);
        self
    }
    
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    
    pub fn build(self) -> Result<HsmConfig, BuildError> {
        let provider = self.provider.ok_or(BuildError::MissingProvider)?;
        
        Ok(HsmConfig {
            provider,
            timeout: self.timeout,
            retry_count: self.retry_count,
        })
    }
}

// Usage
let config = HsmConfigBuilder::new()
    .provider(Provider::Software)
    .timeout(Duration::from_secs(60))
    .build()?;
```

---

## ✅ ASYNC PATTERNS

### Pattern: Proper Async Error Handling

```rust
// ✅ Async with proper errors
pub async fn fetch_data(url: &str) -> Result<Data, BearDogError> {
    let response = reqwest::get(url)
        .await
        .with_context(|| format!("Fetching {}", url))?;
    
    let data = response
        .json()
        .await
        .with_context(|| "Parsing response JSON")?;
    
    Ok(data)
}
```

### Pattern: Timeouts

```rust
use tokio::time::{timeout, Duration};

// ✅ Always use timeouts for external calls
pub async fn call_with_timeout<T>(
    operation: impl Future<Output = Result<T>>,
    duration: Duration,
) -> Result<T> {
    timeout(duration, operation)
        .await
        .map_err(|_| BearDogError::timeout("Operation timed out"))?
}
```

---

## ✅ TRAIT PATTERNS

### Pattern: Extension Traits

```rust
// ✅ Add methods to existing types
pub trait ResultExt<T> {
    fn with_context<F>(self, f: F) -> Result<T, BearDogError>
    where
        F: FnOnce() -> String;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: std::error::Error + 'static,
{
    fn with_context<F>(self, f: F) -> Result<T, BearDogError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let context = f();
            BearDogError::context(context, e)
        })
    }
}
```

---

## ✅ CONFIGURATION PATTERN

```rust
// ✅ Type-safe configuration
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkConfig {
    #[serde(default = "default_api_port")]
    pub api_port: u16,
    
    #[serde(default = "default_timeout")]
    #[serde(with = "humantime_serde")]
    pub timeout: Duration,
}

fn default_api_port() -> u16 {
    std::env::var("BEARDOG_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080)
}

fn default_timeout() -> Duration {
    Duration::from_secs(30)
}

impl NetworkConfig {
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content).map_err(Into::into)
    }
    
    pub fn validate(&self) -> Result<()> {
        if self.api_port == 0 {
            return Err(ConfigError::InvalidPort {
                port: 0,
                reason: "Port cannot be 0".into(),
            }.into());
        }
        Ok(())
    }
}
```

---

## ✅ TESTING PATTERNS

### Pattern: Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn port_roundtrip(port in 1024u16..65535) {
        let config = NetworkConfig { api_port: port, ..Default::default() };
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: NetworkConfig = toml::from_str(&serialized).unwrap();
        assert_eq!(config.api_port, deserialized.api_port);
    }
}
```

---

## ❌ ANTI-PATTERNS TO AVOID

### 1. unwrap() / expect() in Production
```rust
// ❌ NEVER
let value = option.unwrap();

// ✅ ALWAYS
let value = option.ok_or_else(|| BearDogError::missing("value"))?;
```

### 2. Stringly-Typed APIs
```rust
// ❌ BAD
fn set_log_level(level: &str) { ... }

// ✅ GOOD
enum LogLevel { Debug, Info, Warn, Error }
fn set_log_level(level: LogLevel) { ... }
```

### 3. Ignoring Results
```rust
// ❌ BAD
let _ = dangerous_operation();

// ✅ GOOD
dangerous_operation()
    .with_context(|| "Performing dangerous operation")?;
```

### 4. Excessive Cloning
```rust
// ❌ BAD
fn process(data: Vec<u8>) -> Vec<u8> {
    let copy = data.clone();
    // ...
}

// ✅ GOOD
fn process(data: &[u8]) -> Vec<u8> {
    // ...
}
```

---

## 🎓 QUICK REFERENCE

| Pattern | When to Use | Benefit |
|---------|-------------|---------|
| `Result<T, E>` | Operations that can fail | Explicit error handling |
| `Option<T>` | Optional values | No null pointer errors |
| `Cow<'a, T>` | Sometimes borrow, sometimes own | Zero-copy when possible |
| `Arc<T>` | Shared ownership | Cheap cloning |
| `Newtype` | Domain types | Type safety |
| `PhantomData` | State machines | Compile-time guarantees |
| Builder | Complex construction | Fluent API |
| Extension traits | Add methods | Non-invasive |

---

**🦀 Remember**: The compiler is your friend. If it compiles, it usually works!

**Last Updated**: November 14, 2025

