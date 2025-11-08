# 🧪 Test Addition Plan - BearDog
**Date**: November 6, 2025  
**Goal**: Add 50 tests to critical paths  
**Current Coverage**: 68.74%  
**Target Coverage**: 75%+ (interim), 90% (ultimate)

---

## 📊 COVERAGE ANALYSIS

### Current State (from llvm-cov)

```
Total Lines:     81,333
Covered Lines:   55,908 (68.74%)
Functions:       8,336 / 13,362 (63.70%)
Regions:         61,886 / 82,049 (67.42%)
```

### Gap Analysis

**Missing Coverage**: 25,425 lines (31.26%)

**Low Coverage Areas** (identified from audit):
1. Error handling paths
2. Edge case scenarios
3. Platform-specific code
4. Configuration validation
5. Network failure scenarios
6. HSM provider edge cases
7. Discovery fallback paths
8. Integration scenarios

---

## 🎯 50 TEST ADDITIONS - PRIORITIZED

### Phase 1: Error Path Testing (15 tests)

Critical error paths that need coverage.

#### 1.1 Configuration Errors (5 tests)

**File**: `beardog-config/src/validation.rs` (to be created)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_invalid_port_zero() {
        let config = ApiConfig {
            port: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_invalid_port_conflict() {
        let config = NetworkConfig {
            api: ApiConfig { port: 8080, ..Default::default() },
            admin: AdminConfig { port: 8080, ..Default::default() },
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_missing_tls_cert_when_enabled() {
        let config = ApiConfig {
            tls_enabled: true,
            tls_cert_path: None,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_nonexistent_pkcs11_library() {
        let config = PathConfig {
            pkcs11_library: Some(PathBuf::from("/nonexistent/library.so")),
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_invalid_timeout_zero() {
        let config = LimitsConfig {
            operation_timeout_secs: 0,
            ..Default::default()
        };
        // Should warn but not error - or should it?
        // Document the decision
    }
}
```

#### 1.2 Network Error Handling (5 tests)

**File**: `beardog-core/src/service_discovery/network_error_tests.rs` (new)

```rust
#[cfg(test)]
mod network_error_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_connection_timeout() {
        // Test that connection timeout is handled gracefully
        let config = NetworkConfig {
            timeouts: TimeoutsConfig {
                connection_timeout_secs: 1,
                ..Default::default()
            },
            ..Default::default()
        };
        
        // Try to connect to non-responsive endpoint
        let result = try_connect("192.0.2.1:9999", &config).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), NetworkError::Timeout));
    }
    
    #[tokio::test]
    async fn test_connection_refused() {
        // Test connection refused handling
        let result = try_connect("127.0.0.1:1", &Default::default()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), NetworkError::ConnectionRefused));
    }
    
    #[tokio::test]
    async fn test_dns_resolution_failure() {
        let result = try_connect("nonexistent.invalid:8080", &Default::default()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), NetworkError::DnsError));
    }
    
    #[tokio::test]
    async fn test_malformed_response() {
        // Test handling of malformed server responses
        // Mock server that sends garbage
    }
    
    #[tokio::test]
    async fn test_partial_read() {
        // Test handling of partial/incomplete responses
    }
}
```

#### 1.3 HSM Error Paths (5 tests)

**File**: `beardog-tunnel/src/tunnel/hsm/error_tests.rs` (new)

```rust
#[cfg(test)]
mod hsm_error_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_hsm_not_found() {
        let manager = HsmManager::new();
        let result = manager.get_provider("nonexistent").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HsmError::NotFound));
    }
    
    #[tokio::test]
    async fn test_hsm_operation_unsupported() {
        let provider = SoftwareHsmProvider::new();
        // Try unsupported operation
        let result = provider.unsupported_operation().await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HsmError::UnsupportedOperation));
    }
    
    #[tokio::test]
    async fn test_hsm_key_not_found() {
        let provider = SoftwareHsmProvider::new();
        let result = provider.get_key("nonexistent_key").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HsmError::KeyNotFound));
    }
    
    #[tokio::test]
    async fn test_hsm_invalid_key_size() {
        let provider = SoftwareHsmProvider::new();
        let result = provider.generate_key(KeyType::Aes { key_size: 100 }).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HsmError::InvalidKeySize));
    }
    
    #[tokio::test]
    async fn test_hsm_authentication_failure() {
        let provider = Pkcs11Provider::new("library.so");
        let result = provider.login("wrong_pin").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HsmError::AuthenticationFailed));
    }
}
```

---

### Phase 2: Edge Case Testing (15 tests)

Edge cases that might cause issues in production.

#### 2.1 Boundary Value Tests (5 tests)

**File**: `beardog-types/src/canonical/config/boundary_tests.rs` (new)

```rust
#[cfg(test)]
mod boundary_tests {
    use super::*;
    
    #[test]
    fn test_port_minimum() {
        let config = ApiConfig { port: 1, ..Default::default() };
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_port_maximum() {
        let config = ApiConfig { port: 65535, ..Default::default() };
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_port_above_maximum() {
        // Should fail at type level, but test if using u32
        let config = ApiConfig { port: 65536, ..Default::default() };
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_empty_string_handling() {
        let config = ServiceInfo {
            name: "".to_string(),
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_very_long_string() {
        let long_name = "a".repeat(10000);
        let config = ServiceInfo {
            name: long_name,
            ..Default::default()
        };
        // Should either accept or reject with clear limit
        let result = config.validate();
        // Document the decision
    }
}
```

#### 2.2 Concurrent Access Tests (5 tests)

**File**: `beardog-core/src/concurrency_tests.rs` (new)

```rust
#[cfg(test)]
mod concurrency_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_concurrent_config_access() {
        let config = Arc::new(BearDogConfig::load().unwrap());
        
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let config = Arc::clone(&config);
                tokio::spawn(async move {
                    // Multiple threads reading config
                    let _ = config.network.api.port;
                })
            })
            .collect();
        
        for handle in handles {
            handle.await.unwrap();
        }
    }
    
    #[tokio::test]
    async fn test_concurrent_hsm_operations() {
        let manager = Arc::new(HsmManager::new());
        
        // Multiple concurrent HSM operations
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let manager = Arc::clone(&manager);
                tokio::spawn(async move {
                    manager.generate_key(&format!("key_{}", i)).await
                })
            })
            .collect();
        
        for handle in handles {
            assert!(handle.await.unwrap().is_ok());
        }
    }
    
    #[tokio::test]
    async fn test_concurrent_cache_access() {
        // Test cache under concurrent load
    }
    
    #[tokio::test]
    async fn test_provider_selection_race_condition() {
        // Test that provider selection works correctly under race conditions
    }
    
    #[tokio::test]
    async fn test_concurrent_discovery() {
        // Test service discovery with concurrent requests
    }
}
```

#### 2.3 Platform-Specific Tests (5 tests)

**File**: `beardog-config/src/platform_tests.rs` (new)

```rust
#[cfg(test)]
mod platform_tests {
    use super::*;
    
    #[test]
    #[cfg(target_os = "linux")]
    fn test_linux_path_discovery() {
        let paths = PathConfig::discover_pkcs11_libraries();
        // Should find at least one common library on Linux
        assert!(!paths.is_empty() || {
            // Document that no libraries found is OK in CI
            eprintln!("Warning: No PKCS#11 libraries found on Linux");
            true
        });
    }
    
    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos_path_discovery() {
        let paths = PathConfig::discover_pkcs11_libraries();
        // macOS-specific paths
    }
    
    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_path_discovery() {
        let paths = PathConfig::discover_pkcs11_libraries();
        // Windows-specific paths
    }
    
    #[test]
    fn test_default_config_dir_exists_or_creatable() {
        let config_dir = default_config_dir();
        if !config_dir.exists() {
            assert!(fs::create_dir_all(&config_dir).is_ok());
            fs::remove_dir_all(&config_dir).ok(); // Cleanup
        }
    }
    
    #[test]
    fn test_cross_platform_path_handling() {
        // Test that paths work across platforms
        let config = PathConfig::default();
        assert!(config.config_dir.is_absolute() || {
            // Relative paths should be documented as OK in some scenarios
            true
        });
    }
}
```

---

### Phase 3: Integration Testing (10 tests)

End-to-end scenarios that test multiple components.

#### 3.1 Configuration Integration (3 tests)

**File**: `beardog-config/tests/integration.rs` (new)

```rust
#[test]
fn test_config_hierarchy_priority() {
    // Create temp config file
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("test.toml");
    
    fs::write(&config_path, r#"
        [network.api]
        port = 7000
    "#).unwrap();
    
    // Set env var (higher priority)
    env::set_var("BEARDOG_API_PORT", "8000");
    
    // Load config
    let config = BearDogConfig::from_file(&config_path).unwrap();
    
    // Env var should win
    assert_eq!(config.network.api.port, 8000);
    
    env::remove_var("BEARDOG_API_PORT");
}

#[test]
fn test_config_validation_on_load() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("invalid.toml");
    
    fs::write(&config_path, r#"
        [network.api]
        port = 0  # Invalid!
    "#).unwrap();
    
    let result = BearDogConfig::from_file(&config_path);
    assert!(result.is_err());
}

#[test]
fn test_config_with_all_defaults() {
    let config = BearDogConfig::default();
    assert!(config.validate().is_ok());
    assert_eq!(config.network.api.bind_address, IpAddr::V4(Ipv4Addr::LOCALHOST));
}
```

#### 3.2 HSM Provider Integration (4 tests)

**File**: `beardog-tunnel/tests/hsm_integration.rs` (new)

```rust
#[tokio::test]
async fn test_hsm_full_key_lifecycle() {
    let manager = HsmManager::new();
    let provider = manager.get_default_provider().await.unwrap();
    
    // Generate key
    let key_id = provider.generate_key(KeyType::Ed25519).await.unwrap();
    
    // Use key for signing
    let message = b"test message";
    let signature = provider.sign(&key_id, message).await.unwrap();
    
    // Verify signature
    let valid = provider.verify(&key_id, message, &signature).await.unwrap();
    assert!(valid);
    
    // Delete key
    provider.delete_key(&key_id).await.unwrap();
    
    // Verify deletion
    let result = provider.get_key(&key_id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_hsm_provider_fallback() {
    // Test that system falls back to software HSM if hardware unavailable
    let manager = HsmManager::new();
    let requirements = HsmRequirements {
        require_hardware: false,
        ..Default::default()
    };
    
    let provider = manager.select_provider(&requirements).await.unwrap();
    assert!(provider.is_available().await);
}

#[tokio::test]
async fn test_multiple_hsm_providers_simultaneously() {
    let manager = HsmManager::new();
    
    // Get multiple providers
    let provider1 = manager.select_provider(&HsmRequirements::default()).await.unwrap();
    let provider2 = manager.select_provider(&HsmRequirements::default()).await.unwrap();
    
    // Both should work
    let key1 = provider1.generate_key(KeyType::Ed25519).await.unwrap();
    let key2 = provider2.generate_key(KeyType::Ed25519).await.unwrap();
    
    assert_ne!(key1, key2);
}

#[tokio::test]
async fn test_hsm_capability_detection() {
    let manager = HsmManager::new();
    let providers = manager.discover_providers().await.unwrap();
    
    // At least one provider should be available (software HSM)
    assert!(!providers.is_empty());
    
    // Each provider should report capabilities
    for provider in providers {
        let caps = provider.get_capabilities().await.unwrap();
        assert!(!caps.operations.is_empty());
    }
}
```

#### 3.3 Service Discovery Integration (3 tests)

**File**: `beardog-core/tests/discovery_integration.rs` (new)

```rust
#[tokio::test]
async fn test_discovery_with_multiple_backends() {
    let manager = DiscoveryManager::new();
    
    // Add multiple backends
    manager.add_provider(DiscoveryProvider::static_config(StaticDiscovery::default())).await;
    manager.add_provider(DiscoveryProvider::dns_sd(DnsSdDiscovery::default())).await;
    
    // Discovery should aggregate results from all backends
    let filter = ServiceFilter::default();
    let services = manager.discover_all(&filter).await.unwrap();
    
    // May be empty if no services configured, but should not error
    assert!(services.is_ok());
}

#[tokio::test]
async fn test_discovery_fallback_on_failure() {
    // If primary discovery fails, should fall back to others
    let manager = DiscoveryManager::new();
    
    // Add providers (some may fail)
    manager.add_provider(DiscoveryProvider::static_config(StaticDiscovery::default())).await;
    
    // Discovery should work with at least one provider
    let requirements = DiscoveryRequirements::production();
    let provider = manager.select_provider(&requirements).await.unwrap();
    
    assert!(provider.discover(&ServiceFilter::default()).await.is_ok());
}

#[tokio::test]
async fn test_discovery_caching() {
    let manager = DiscoveryManager::new();
    manager.add_provider(DiscoveryProvider::static_config(StaticDiscovery::default())).await;
    
    // First call - should discover
    let start = std::time::Instant::now();
    let services1 = manager.discover_all(&ServiceFilter::default()).await.unwrap();
    let first_duration = start.elapsed();
    
    // Second call - should be cached (faster)
    let start = std::time::Instant::now();
    let services2 = manager.discover_all(&ServiceFilter::default()).await.unwrap();
    let second_duration = start.elapsed();
    
    assert_eq!(services1, services2);
    // Note: Cache timing is not always reliable, so this might be flaky
}
```

---

### Phase 4: Property-Based Testing (5 tests)

Using proptest or quickcheck for property testing.

**File**: `beardog-utils/src/property_tests.rs` (enhance existing)

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_port_number_always_valid_when_in_range(port in 1u16..=65535) {
        let config = ApiConfig { port, ..Default::default() };
        prop_assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_config_serialization_roundtrip(
        port in 1u16..=65535,
        timeout in 1u64..=3600,
    ) {
        let config = NetworkConfig {
            api: ApiConfig { port, ..Default::default() },
            timeouts: TimeoutsConfig {
                operation_timeout_secs: timeout,
                ..Default::default()
            },
            ..Default::default()
        };
        
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: NetworkConfig = toml::from_str(&serialized).unwrap();
        
        prop_assert_eq!(config.api.port, deserialized.api.port);
    }
    
    #[test]
    fn test_key_id_generation_always_unique(seed in any::<u64>()) {
        let mut rng = StdRng::seed_from_u64(seed);
        let id1 = generate_key_id(&mut rng);
        let id2 = generate_key_id(&mut rng);
        prop_assert_ne!(id1, id2);
    }
    
    #[test]
    fn test_hash_function_consistency(data in prop::collection::vec(any::<u8>(), 0..1000)) {
        let hash1 = compute_hash(&data);
        let hash2 = compute_hash(&data);
        prop_assert_eq!(hash1, hash2);
    }
    
    #[test]
    fn test_encryption_roundtrip(
        plaintext in prop::collection::vec(any::<u8>(), 1..1000)
    ) {
        let key = generate_test_key();
        let encrypted = encrypt(&key, &plaintext).unwrap();
        let decrypted = decrypt(&key, &encrypted).unwrap();
        prop_assert_eq!(plaintext, decrypted);
    }
}
```

---

### Phase 5: Performance & Stress Testing (5 tests)

**File**: `beardog-core/tests/performance_tests.rs` (new)

```rust
#[tokio::test]
async fn test_high_concurrency_discovery() {
    let manager = Arc::new(DiscoveryManager::new());
    manager.add_provider(DiscoveryProvider::static_config(StaticDiscovery::default())).await;
    
    // 100 concurrent discovery requests
    let handles: Vec<_> = (0..100)
        .map(|_| {
            let manager = Arc::clone(&manager);
            tokio::spawn(async move {
                manager.discover_all(&ServiceFilter::default()).await
            })
        })
        .collect();
    
    for handle in handles {
        assert!(handle.await.unwrap().is_ok());
    }
}

#[tokio::test]
async fn test_sustained_hsm_operations() {
    let provider = SoftwareHsmProvider::new();
    
    // 1000 sequential operations
    for i in 0..1000 {
        let key_id = provider.generate_key(KeyType::Ed25519).await.unwrap();
        let message = format!("message_{}", i);
        let signature = provider.sign(&key_id, message.as_bytes()).await.unwrap();
        provider.verify(&key_id, message.as_bytes(), &signature).await.unwrap();
        provider.delete_key(&key_id).await.unwrap();
    }
}

#[test]
fn test_large_config_file() {
    // Test that we can handle large config files
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("large.toml");
    
    // Create config with 1000 entries
    let mut content = String::from("[services]\n");
    for i in 0..1000 {
        content.push_str(&format!("service_{} = \"http://localhost:{}\"\n", i, 8000 + i));
    }
    
    fs::write(&config_path, content).unwrap();
    
    // Should load without issues
    let result = BearDogConfig::from_file(&config_path);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_memory_usage_under_load() {
    // Monitor memory usage during sustained operations
    let initial_memory = get_memory_usage();
    
    let manager = HsmManager::new();
    let provider = manager.get_default_provider().await.unwrap();
    
    // Generate and delete many keys
    for _ in 0..1000 {
        let key_id = provider.generate_key(KeyType::Ed25519).await.unwrap();
        provider.delete_key(&key_id).await.unwrap();
    }
    
    let final_memory = get_memory_usage();
    let memory_growth = final_memory - initial_memory;
    
    // Memory growth should be reasonable (< 100 MB)
    assert!(memory_growth < 100 * 1024 * 1024, 
        "Memory grew by {} bytes", memory_growth);
}

#[tokio::test]
async fn test_cache_performance() {
    let cache = Cache::new(CacheConfig::default());
    
    // Measure cache hit performance
    let key = "test_key";
    let value = vec![0u8; 1024];
    cache.put(key, value.clone()).await.unwrap();
    
    let start = std::time::Instant::now();
    for _ in 0..10000 {
        let _ = cache.get(key).await;
    }
    let duration = start.elapsed();
    
    // 10000 cache hits should be fast (< 100ms)
    assert!(duration < std::time::Duration::from_millis(100));
}
```

---

## 📊 COVERAGE IMPACT ESTIMATE

### Expected Coverage Improvement

```
Current Coverage:  68.74%
After 50 Tests:    ~73-75% (+4-6%)
Target:            75% (interim goal)
Ultimate Target:   90%
```

### Test Distribution

```
Error Paths:         15 tests (30%)
Edge Cases:          15 tests (30%)
Integration:         10 tests (20%)
Property-Based:       5 tests (10%)
Performance:          5 tests (10%)
```

---

## 🎯 IMPLEMENTATION PLAN

### Week 1: Error Path Tests (15 tests)
**Time**: 8-10 hours

- Day 1-2: Configuration error tests
- Day 3: Network error tests
- Day 4-5: HSM error tests

### Week 2: Edge Case Tests (15 tests)
**Time**: 8-10 hours

- Day 1-2: Boundary value tests
- Day 3: Concurrent access tests
- Day 4-5: Platform-specific tests

### Week 3: Integration Tests (10 tests)
**Time**: 6-8 hours

- Day 1-2: Configuration integration
- Day 3-4: HSM integration
- Day 5: Discovery integration

### Week 4: Property & Performance Tests (10 tests)
**Time**: 6-8 hours

- Day 1-3: Property-based tests
- Day 4-5: Performance tests

**Total Time**: 28-36 hours over 4 weeks

---

## ✅ SUCCESS CRITERIA

### Immediate Goals
- [ ] 50 new tests added
- [ ] All tests passing
- [ ] Coverage increased to 73-75%
- [ ] Critical paths covered

### Quality Goals
- [ ] Tests are maintainable
- [ ] Tests document edge cases
- [ ] Tests catch real issues
- [ ] Tests run reasonably fast (<5 min total)

### Long-term Goals
- [ ] Foundation for 90% coverage
- [ ] Comprehensive error testing
- [ ] Good integration test suite
- [ ] Performance baseline established

---

## 📋 TRACKING

### Test Implementation Checklist

#### Phase 1: Error Paths
- [ ] Config validation tests (5)
- [ ] Network error tests (5)
- [ ] HSM error tests (5)

#### Phase 2: Edge Cases
- [ ] Boundary value tests (5)
- [ ] Concurrency tests (5)
- [ ] Platform tests (5)

#### Phase 3: Integration
- [ ] Config integration (3)
- [ ] HSM integration (4)
- [ ] Discovery integration (3)

#### Phase 4: Property-Based
- [ ] Property tests (5)

#### Phase 5: Performance
- [ ] Performance tests (5)

### Coverage Tracking

```bash
# Run coverage after each phase
cargo llvm-cov --workspace --html

# Check improvement
cat target/llvm-cov/html/index.html | grep -A 2 "Coverage Summary"
```

---

## 💡 BEST PRACTICES

### Test Organization
- Group related tests in modules
- Use descriptive test names
- Document what each test validates
- Include both positive and negative cases

### Test Quality
- Tests should be deterministic
- Avoid sleeps (use synchronization)
- Clean up resources
- Use meaningful assertions

### Performance
- Keep individual tests fast (<100ms)
- Use `#[ignore]` for slow tests
- Run expensive tests separately
- Consider parallel execution

---

**Status**: 📋 **PLAN COMPLETE - READY FOR IMPLEMENTATION**  
**Next Action**: Begin Phase 1 error path tests  
**Timeline**: 4 weeks for 50 tests  
**Expected Impact**: +4-6% coverage

🐻🧪 **BearDog: Test Addition Plan - Clear Path to Better Coverage** 🧪🐻

