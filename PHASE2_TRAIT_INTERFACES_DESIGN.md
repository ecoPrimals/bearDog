# 🎨 Phase 2: Trait-Based Config Interfaces Design

**Date**: November 8, 2025  
**Status**: 🚀 **READY TO IMPLEMENT**  
**Estimated Time**: 12-16 hours  
**Priority**: HIGH (architectural improvement)

---

## 🎯 OBJECTIVE

Instead of forcing config consolidation (which breaks domain boundaries), create **trait-based interfaces** that enable polymorphism while preserving domain-specific configs.

**Benefits**:
- ✅ Keep domain-specific configs intact
- ✅ Enable generic algorithms
- ✅ Maintain type safety
- ✅ Better architecture than forced mergers
- ✅ Easy to extend

---

## 🏗️ TRAIT 1: RetryStrategy (Priority: HIGH)

### Design

```rust
/// Trait for retry strategy configuration
/// 
/// Provides a common interface for different retry implementations
/// while allowing domain-specific customization.
pub trait RetryStrategy: Send + Sync {
    /// Maximum number of retry attempts
    fn max_attempts(&self) -> u32;
    
    /// Calculate delay for a specific attempt (1-indexed)
    fn delay_for_attempt(&self, attempt: u32) -> Duration;
    
    /// Determine if an error should trigger a retry
    fn should_retry_error(&self, error: &(dyn std::error::Error + Send + Sync)) -> bool;
    
    /// Get backoff multiplier (for exponential backoff)
    fn backoff_multiplier(&self) -> f64 {
        2.0 // Default exponential backoff
    }
    
    /// Check if retry limit reached
    fn is_limit_reached(&self, attempt: u32) -> bool {
        attempt >= self.max_attempts()
    }
}
```

### Implementations

#### 1. CanonicalRetryConfig

```rust
impl RetryStrategy for CanonicalRetryConfig {
    fn max_attempts(&self) -> u32 {
        self.max_attempts
    }
    
    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if self.enable_exponential_backoff {
            let delay_ms = (self.initial_delay.as_millis() as f64 
                * self.backoff_multiplier.powi(attempt as i32 - 1)) as u64;
            Duration::from_millis(delay_ms.min(self.max_delay.as_millis() as u64))
        } else {
            self.initial_delay
        }
    }
    
    fn should_retry_error(&self, _error: &(dyn std::error::Error + Send + Sync)) -> bool {
        true // Retry all errors by default
    }
    
    fn backoff_multiplier(&self) -> f64 {
        self.backoff_multiplier
    }
}
```

#### 2. NetworkRetryConfiguration

```rust
impl RetryStrategy for NetworkRetryConfiguration {
    fn max_attempts(&self) -> u32 {
        self.max_retries
    }
    
    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        // Network-specific: includes jitter
        let base_delay = self.base_delay_ms as f64;
        let jittered = base_delay * (1.0 + (rand::random::<f64>() - 0.5) * self.jitter_factor);
        Duration::from_millis(jittered as u64)
    }
    
    fn should_retry_error(&self, error: &(dyn std::error::Error + Send + Sync)) -> bool {
        // Network-specific: check for retryable status codes
        if let Some(network_err) = error.downcast_ref::<NetworkError>() {
            network_err.is_retryable()
        } else {
            true
        }
    }
}
```

#### 3. ResilienceRetryConfig

```rust
impl RetryStrategy for ResilienceRetryConfig {
    fn max_attempts(&self) -> u32 {
        if self.enabled {
            self.max_attempts
        } else {
            0 // No retries if disabled
        }
    }
    
    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        self.base_delay * attempt // Linear backoff for resilience
    }
    
    fn should_retry_error(&self, error: &(dyn std::error::Error + Send + Sync)) -> bool {
        // Check against configured error list
        let error_str = error.to_string();
        self.retry_on_errors.iter().any(|pattern| error_str.contains(pattern))
    }
}
```

### Generic Retry Executor

```rust
/// Generic retry executor that works with any RetryStrategy
pub async fn execute_with_retry<F, T, E, S>(
    strategy: &S,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>,
    E: std::error::Error + Send + Sync + 'static,
    S: RetryStrategy,
{
    let mut attempt = 0;
    
    loop {
        attempt += 1;
        
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) if strategy.is_limit_reached(attempt) => {
                return Err(error);
            }
            Err(error) if !strategy.should_retry_error(&error) => {
                return Err(error);
            }
            Err(_) => {
                let delay = strategy.delay_for_attempt(attempt);
                tokio::time::sleep(delay).await;
            }
        }
    }
}
```

### Usage Example

```rust
// Works with ANY RetryStrategy implementation!
let retry_config = CanonicalRetryConfig::default();

let result = execute_with_retry(&retry_config, || {
    Box::pin(async {
        // Your async operation here
        perform_network_request().await
    })
}).await?;
```

---

## 🏗️ TRAIT 2: TlsConfiguration (Priority: HIGH)

### Design

```rust
/// Trait for TLS configuration
pub trait TlsConfiguration: Send + Sync {
    /// Whether TLS is enabled
    fn is_enabled(&self) -> bool;
    
    /// Path to certificate file
    fn cert_path(&self) -> Option<&Path>;
    
    /// Path to private key file
    fn key_path(&self) -> Option<&Path>;
    
    /// Path to CA certificate file
    fn ca_path(&self) -> Option<&Path>;
    
    /// Whether to verify peer certificates
    fn verify_peer(&self) -> bool {
        true // Secure by default
    }
    
    /// Minimum TLS version
    fn min_tls_version(&self) -> TlsVersion {
        TlsVersion::Tls12 // Secure default
    }
}

/// TLS version enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TlsVersion {
    Tls10,
    Tls11,
    Tls12,
    Tls13,
}
```

### Implementations

```rust
// Discovery TLS Config
impl TlsConfiguration for DiscoveryTlsConfig {
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn cert_path(&self) -> Option<&Path> {
        self.cert_file.as_deref()
    }
    
    fn key_path(&self) -> Option<&Path> {
        self.key_file.as_deref()
    }
    
    fn ca_path(&self) -> Option<&Path> {
        self.ca_file.as_deref()
    }
    
    fn verify_peer(&self) -> bool {
        self.verify_peer
    }
}

// Production TLS Config (with cipher suites)
impl TlsConfiguration for ProductionTlsConfig {
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn cert_path(&self) -> Option<&Path> {
        Some(&self.cert_path) // Required in production
    }
    
    fn key_path(&self) -> Option<&Path> {
        Some(&self.key_path) // Required in production
    }
    
    fn ca_path(&self) -> Option<&Path> {
        Some(&self.ca_path)
    }
    
    fn verify_peer(&self) -> bool {
        true // Always verify in production
    }
    
    fn min_tls_version(&self) -> TlsVersion {
        TlsVersion::Tls13 // Production uses TLS 1.3
    }
}
```

---

## 🏗️ TRAIT 3: TimeoutPolicy (Priority: MEDIUM)

### Design

```rust
/// Trait for timeout policies
pub trait TimeoutPolicy: Send + Sync {
    /// Connection timeout
    fn connection_timeout(&self) -> Duration;
    
    /// Operation-specific timeout
    fn operation_timeout(&self, operation: &str) -> Duration;
    
    /// Check if elapsed time exceeds timeout for operation
    fn should_timeout(&self, elapsed: Duration, operation: &str) -> bool {
        elapsed >= self.operation_timeout(operation)
    }
    
    /// Global timeout (max for any operation)
    fn global_timeout(&self) -> Option<Duration> {
        None // No global timeout by default
    }
}
```

### Implementations

```rust
// Network Timeout Config
impl TimeoutPolicy for NetworkTimeoutConfig {
    fn connection_timeout(&self) -> Duration {
        self.connect_timeout
    }
    
    fn operation_timeout(&self, operation: &str) -> Duration {
        match operation {
            "read" => self.read_timeout,
            "write" => self.write_timeout,
            _ => self.default_timeout,
        }
    }
}

// HSM Timeout Config (different timeouts for crypto ops)
impl TimeoutPolicy for HsmTimeoutConfig {
    fn connection_timeout(&self) -> Duration {
        self.init_timeout
    }
    
    fn operation_timeout(&self, operation: &str) -> Duration {
        match operation {
            "sign" => self.sign_timeout,
            "verify" => self.verify_timeout,
            "encrypt" => self.encrypt_timeout,
            "decrypt" => self.decrypt_timeout,
            _ => Duration::from_secs(30),
        }
    }
    
    fn global_timeout(&self) -> Option<Duration> {
        Some(Duration::from_secs(300)) // 5 minute max for HSM
    }
}
```

---

## 🏗️ TRAIT 4: CacheStrategy (Priority: MEDIUM)

### Design

```rust
/// Trait for cache strategies
pub trait CacheStrategy: Send + Sync {
    /// Maximum number of cache entries
    fn max_entries(&self) -> usize;
    
    /// Time-to-live for cache entries
    fn ttl(&self) -> Duration;
    
    /// Eviction policy when cache is full
    fn eviction_policy(&self) -> EvictionPolicy;
    
    /// Check if entry should be evicted
    fn should_evict(&self, age: Duration, access_count: usize) -> bool {
        age >= self.ttl()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionPolicy {
    /// Least Recently Used
    Lru,
    /// Least Frequently Used
    Lfu,
    /// First In First Out
    Fifo,
    /// Random
    Random,
}
```

---

## 🏗️ TRAIT 5: MonitoringConfig (Priority: LOW)

### Design

```rust
/// Trait for monitoring configuration
pub trait MonitoringConfig: Send + Sync {
    /// Whether monitoring is enabled
    fn is_enabled(&self) -> bool;
    
    /// Metrics endpoint URL
    fn metrics_endpoint(&self) -> &str;
    
    /// Reporting interval
    fn reporting_interval(&self) -> Duration;
    
    /// Whether to include detailed metrics
    fn detailed_metrics(&self) -> bool {
        false // Summary by default
    }
}
```

---

## 📋 IMPLEMENTATION PLAN

### Step 1: Create Trait Module (1 hour)

**File**: `crates/beardog-types/src/canonical/traits/config_traits.rs`

```rust
//! Configuration trait interfaces
//!
//! Provides common interfaces for config families to enable
//! polymorphic usage while preserving domain-specific implementations.

pub mod retry;
pub mod tls;
pub mod timeout;
pub mod cache;
pub mod monitoring;

// Re-export all traits
pub use retry::RetryStrategy;
pub use tls::TlsConfiguration;
pub use timeout::TimeoutPolicy;
pub use cache::CacheStrategy;
pub use monitoring::MonitoringConfig;
```

### Step 2: Implement RetryStrategy (4 hours)

1. Define trait (30 min)
2. Implement for CanonicalRetryConfig (1 hour)
3. Implement for NetworkRetryConfiguration (1 hour)
4. Implement for ResilienceRetryConfig (1 hour)
5. Create generic executor (30 min)
6. Write tests (1 hour)

### Step 3: Implement TlsConfiguration (3 hours)

1. Define trait (30 min)
2. Implement for 3-4 TLS configs (1.5 hours)
3. Create helper functions (30 min)
4. Write tests (30 min)

### Step 4: Implement TimeoutPolicy (3 hours)

Similar pattern to above

### Step 5: Implement CacheStrategy (2 hours)

Similar pattern to above

### Step 6: Implement MonitoringConfig (2 hours)

Similar pattern to above

### Step 7: Documentation (2 hours)

1. Update CONFIG_ARCHITECTURE_AND_RATIONALE.md
2. Create trait usage examples
3. Add to TRAIT_HIERARCHY_GUIDE.md

---

## 📈 EXPECTED OUTCOMES

### Grade Impact

**After completion**: +0.5 points (95.0 → 95.5)

**Rationale**:
- Significant architectural improvement
- Enables polymorphism without breaking domains
- Demonstrates sophisticated design patterns
- Future-proof extensibility

### Code Quality Impact

1. **Reduced Coupling**: Generic code doesn't depend on specific config types
2. **Increased Testability**: Easy to create test implementations
3. **Better Maintainability**: Clear interfaces, easy to extend
4. **Type Safety**: Compile-time guarantees

### Example Impact

**Before** (tightly coupled):
```rust
fn retry_operation(config: &CanonicalRetryConfig) { ... }
// Can't use with NetworkRetryConfiguration!
```

**After** (polymorphic):
```rust
fn retry_operation<S: RetryStrategy>(strategy: &S) { ... }
// Works with ANY RetryStrategy implementation!
```

---

## ✅ SUCCESS CRITERIA

- [ ] All 5 traits defined and documented
- [ ] At least 3 implementations per trait
- [ ] Generic executors/helpers created
- [ ] Comprehensive test coverage
- [ ] Documentation updated
- [ ] Builds cleanly
- [ ] All tests pass

---

## 🚀 READY TO IMPLEMENT

**Next Steps**:
1. Create trait module structure
2. Start with RetryStrategy (highest value)
3. Test thoroughly
4. Document pattern
5. Apply to other config families

**Estimated Total Time**: 12-16 hours  
**Priority**: HIGH  
**Grade Impact**: +0.5

---

**Status**: 🚀 **DESIGN COMPLETE, READY TO CODE**  
**Next**: Create `config_traits` module and implement RetryStrategy

🐻 **BearDog: Trait-Based Architecture FTW!** 🎨

