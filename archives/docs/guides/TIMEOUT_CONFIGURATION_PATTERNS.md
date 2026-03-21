# Timeout Configuration Patterns

**Created**: November 7, 2025  
**Status**: Active  
**Version**: 1.0.0

---

## Overview

All timeout values in BearDog are centralized in `beardog-config::domains::timeouts::TimeoutConfig`. This document describes best practices for using and configuring timeouts throughout the codebase.

---

## Quick Reference

### Using Centralized Timeouts

**✅ GOOD** - Use centralized config:

```rust
use beardog_config::domains::timeouts::TimeoutConfig;

// Load from environment (or use defaults)
let timeout_config = TimeoutConfig::from_env();
let timeout = timeout_config.ai_decision_duration();

// Use the timeout
tokio::time::timeout(timeout, async_operation()).await?;
```

**❌ BAD** - Don't hardcode or inline env var checks:

```rust
// ❌ Don't do this
let timeout = Duration::from_secs(30);

// ❌ Don't do this
let timeout = Duration::from_secs(
    std::env::var("TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30)
);
```

---

## Available Timeouts

### Core Operations

| Timeout | Method | Default | Environment Variable |
|---------|--------|---------|---------------------|
| Health Checks | `health_check_duration()` | 5s | `BEARDOG_HEALTH_CHECK_TIMEOUT_SECS` |
| HSM Operations | `hsm_operation_duration()` | 2s | `BEARDOG_HSM_OPERATION_TIMEOUT_SECS` |
| HSM Probe | `hsm_probe_duration()` | 500ms | `BEARDOG_HSM_PROBE_TIMEOUT_MILLIS` |
| Service Discovery | `discovery_operation_duration()` | 10s | `BEARDOG_DISCOVERY_TIMEOUT_SECS` |

### AI Operations

| Timeout | Method | Default | Environment Variable |
|---------|--------|---------|---------------------|
| AI Decisions | `ai_decision_duration()` | 30s | `BEARDOG_DECISION_TIMEOUT_SECS` |
| AI Requests | `ai_request_timeout_duration()` | 30s | `BEARDOG_AI_REQUEST_TIMEOUT_SECS` |
| AI Batching | `ai_batch_timeout_duration()` | 10ms | `BEARDOG_AI_BATCH_TIMEOUT_MS` |

### Connection Pooling

| Timeout | Method | Default | Environment Variable |
|---------|--------|---------|---------------------|
| Pool Idle | `pool_idle_timeout_duration()` | 300s (5min) | `BEARDOG_POOL_IDLE_TIMEOUT_SECS` |
| Connection Age | `max_connection_age_duration()` | 3600s (1h) | `BEARDOG_MAX_CONNECTION_AGE_SECS` |

---

## Usage Patterns

### Pattern 1: Simple Timeout

For straightforward operations, load config and use the appropriate method:

```rust
use beardog_config::domains::timeouts::TimeoutConfig;
use tokio::time::timeout;

async fn perform_operation() -> Result<Data, BearDogError> {
    let timeout_config = TimeoutConfig::from_env();
    
    let result = timeout(
        timeout_config.hsm_operation_duration(),
        hsm.encrypt(data)
    ).await??;
    
    Ok(result)
}
```

### Pattern 2: Struct Initialization

When initializing structs with timeout fields:

```rust
use beardog_config::domains::timeouts::TimeoutConfig;

impl Default for MyConfig {
    fn default() -> Self {
        let timeout_config = TimeoutConfig::from_env();
        
        Self {
            request_timeout: timeout_config.ai_request_timeout_duration(),
            health_check_interval: timeout_config.health_check_duration(),
            // ... other fields
        }
    }
}
```

### Pattern 3: Builder Pattern

For test-friendly configuration:

```rust
use beardog_config::domains::timeouts::TimeoutConfig;

// In tests - explicit values without env pollution
let config = TimeoutConfig::builder()
    .health_check_secs(1)  // Fast for tests
    .hsm_operation_secs(1)
    .build();

// In production - load from environment
let config = TimeoutConfig::from_env();
```

### Pattern 4: Shared Config

When passing config around:

```rust
pub struct ServiceManager {
    timeout_config: TimeoutConfig,
}

impl ServiceManager {
    pub fn new() -> Self {
        Self {
            timeout_config: TimeoutConfig::from_env(),
        }
    }
    
    pub async fn check_health(&self) -> Result<bool, BearDogError> {
        timeout(
            self.timeout_config.health_check_duration(),
            self.service.health()
        ).await?
    }
}
```

---

## Environment Variable Naming

### Conventions

All timeout environment variables follow these conventions:

1. **Prefix**: Always use `BEARDOG_` prefix
2. **Suffix**:
   - `_TIMEOUT_SECS` for second-based timeouts
   - `_TIMEOUT_MS` or `_TIMEOUT_MILLIS` for millisecond-based timeouts
   - `_INTERVAL_SECS` for periodic intervals
3. **Naming**: Use descriptive names indicating what operation is timed

### Examples

```bash
# ✅ Good naming
export BEARDOG_HSM_OPERATION_TIMEOUT_SECS=5
export BEARDOG_AI_BATCH_TIMEOUT_MS=20
export BEARDOG_HEALTH_CHECK_INTERVAL_SECS=10

# ❌ Bad naming
export TIMEOUT=5                    # Too generic
export BEARDOG_TIMEOUT_HSM=5       # Wrong format
export HSM_TIMEOUT_SECS=5          # Missing prefix
```

---

## Configuration File Support

Timeouts can also be set via TOML configuration files:

```toml
# beardog-config.toml

[timeouts]
health_check_secs = 10
hsm_operation_secs = 5
hsm_probe_millis = 1000
discovery_operation_secs = 15
ai_decision_secs = 60
ai_request_timeout_secs = 45
ai_batch_timeout_millis = 20
pool_idle_timeout_secs = 600
max_connection_age_secs = 7200
```

Load configuration from file:

```rust
use beardog_config::BearDogConfig;

let config = BearDogConfig::from_file("beardog-config.toml")?;
let timeouts = &config.timeouts;

let timeout = timeouts.ai_decision_duration();
```

---

## Validation

TimeoutConfig includes validation to ensure sensible values:

```rust
let config = TimeoutConfig::builder()
    .health_check_secs(100)  // Too long!
    .build();

// Validate returns Err
assert!(config.validate().is_err());
```

### Validation Ranges

| Timeout | Min | Max | Rationale |
|---------|-----|-----|-----------|
| Health Check | 1s | 30s | Fast feedback, not too frequent |
| HSM Operation | 1s | 10s | Hardware operations should complete quickly |
| HSM Probe | 100ms | 5000ms | Fast availability checks |
| Discovery | 1s | 60s | Network latency tolerance |
| AI Decision | 5s | 300s | Complex decisions take time |
| AI Request | 5s | 300s | Inference operations |
| AI Batch | 1ms | 1000ms | Fast batching for throughput |
| Pool Idle | 60s | 3600s | Reasonable idle before cleanup |
| Connection Age | 300s | 86400s | Connections should refresh periodically |

---

## Testing

### Test-Friendly Configuration

Use the builder pattern to avoid environment variable pollution in tests:

```rust
#[test]
fn test_operation_with_timeout() {
    // ✅ Good - no env var pollution
    let config = TimeoutConfig::builder()
        .hsm_operation_secs(1)
        .build();
    
    let result = perform_operation(&config);
    assert!(result.is_ok());
}

#[test]
fn test_timeout_behavior() {
    // ❌ Bad - pollutes environment for other tests
    std::env::set_var("BEARDOG_HSM_OPERATION_TIMEOUT_SECS", "1");
    let config = TimeoutConfig::from_env();
    
    // ... test code
}
```

### Mock Timeouts

For fast tests, use short timeouts:

```rust
#[tokio::test]
async fn test_timeout_expires() {
    let config = TimeoutConfig::builder()
        .hsm_operation_secs(1)  // 1 second for fast test
        .build();
    
    let result = timeout(
        config.hsm_operation_duration(),
        async {
            tokio::time::sleep(Duration::from_secs(10)).await;
            Ok::<_, BearDogError>(())
        }
    ).await;
    
    assert!(result.is_err());  // Should timeout
}
```

---

## Migration Guide

### From Hardcoded Timeouts

**Before**:
```rust
let timeout = Duration::from_secs(30);
tokio::time::timeout(timeout, operation()).await?;
```

**After**:
```rust
use beardog_config::domains::timeouts::TimeoutConfig;

let timeout_config = TimeoutConfig::from_env();
tokio::time::timeout(
    timeout_config.ai_decision_duration(),
    operation()
).await?;
```

### From Environment Variable Checks

**Before**:
```rust
let timeout = Duration::from_secs(
    std::env::var("BEARDOG_DECISION_TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30)
);
```

**After**:
```rust
use beardog_config::domains::timeouts::TimeoutConfig;

let timeout_config = TimeoutConfig::from_env();
let timeout = timeout_config.ai_decision_duration();
```

### From Struct Fields

**Before**:
```rust
pub struct Config {
    pub timeout_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout_secs: std::env::var("TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
        }
    }
}
```

**After**:
```rust
use beardog_config::domains::timeouts::TimeoutConfig;

pub struct Config {
    pub timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        let timeout_config = TimeoutConfig::from_env();
        
        Self {
            timeout: timeout_config.ai_decision_duration(),
        }
    }
}
```

---

## Common Pitfalls

### ❌ Pitfall 1: Forgetting to Load Config

```rust
// ❌ Wrong - uses compile-time defaults always
let config = TimeoutConfig::default();
let timeout = config.ai_decision_duration();  // Always 30s!

// ✅ Correct - loads from environment
let config = TimeoutConfig::from_env();
let timeout = config.ai_decision_duration();  // Respects env vars
```

### ❌ Pitfall 2: Loading Config Multiple Times

```rust
// ❌ Inefficient - loads env vars on every call
pub async fn operation1(&self) -> Result<()> {
    let config = TimeoutConfig::from_env();
    timeout(config.health_check_duration(), ...).await?
}

pub async fn operation2(&self) -> Result<()> {
    let config = TimeoutConfig::from_env();
    timeout(config.health_check_duration(), ...).await?
}

// ✅ Better - load once and share
pub struct Manager {
    timeout_config: TimeoutConfig,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            timeout_config: TimeoutConfig::from_env(),
        }
    }
    
    pub async fn operation1(&self) -> Result<()> {
        timeout(self.timeout_config.health_check_duration(), ...).await?
    }
}
```

### ❌ Pitfall 3: Wrong Timeout for Operation

```rust
// ❌ Wrong - using HSM timeout for AI decision
let config = TimeoutConfig::from_env();
timeout(
    config.hsm_operation_duration(),  // Too short for AI!
    make_ai_decision()
).await?

// ✅ Correct - use appropriate timeout
let config = TimeoutConfig::from_env();
timeout(
    config.ai_decision_duration(),  // Right timeout for AI
    make_ai_decision()
).await?
```

---

## Best Practices

### 1. Load Configuration Early

Load timeout configuration once at startup and share it:

```rust
pub struct Application {
    timeout_config: TimeoutConfig,
    // ... other fields
}

impl Application {
    pub fn new() -> Self {
        Self {
            timeout_config: TimeoutConfig::from_env(),
            // ... initialize other fields
        }
    }
}
```

### 2. Use Appropriate Timeouts

Choose the timeout that matches your operation type:

- **Health checks**: Use `health_check_duration()`
- **HSM operations**: Use `hsm_operation_duration()`
- **AI decisions**: Use `ai_decision_duration()`
- **Network operations**: Use `discovery_operation_duration()`

### 3. Document Timeout Requirements

When creating new operations, document which timeout should be used:

```rust
/// Performs AI-based decision making
///
/// # Timeout
/// Uses `TimeoutConfig::ai_decision_duration()` (default: 30s)
///
/// # Environment Variables
/// - `BEARDOG_DECISION_TIMEOUT_SECS` - Override default timeout
pub async fn make_decision(&self) -> Result<Decision, BearDogError> {
    let timeout_config = TimeoutConfig::from_env();
    
    timeout(
        timeout_config.ai_decision_duration(),
        self.ai_engine.decide()
    ).await?
}
```

### 4. Validate Configuration

Always validate timeout configuration when loading from external sources:

```rust
let config = TimeoutConfig::from_file("config.toml")?;
config.validate()?;  // Ensure all values are sensible
```

---

## See Also

- `beardog-config/src/domains/timeouts.rs` - Implementation
- `CONFIGURATION_SYSTEM_DESIGN.md` - Overall config architecture
- `BEARDOG_CODING_STANDARDS.md` - Coding standards

---

**Last Updated**: November 7, 2025  
**Maintained By**: BearDog Development Team

