# Benchmark Restoration Notes

**Date**: October 9, 2025  
**Status**: Requires API Migration  
**Estimated Effort**: 8-12 hours (revised from 5-8)

---

## Issue Summary

All 10 benchmark files need API migration to work with the modernized codebase. They were written for older APIs and have compilation errors.

---

## Compilation Errors Found

### 1. API Signature Changes
```rust
// Old API (benchmarks)
let core = BearDogCore::new().await.expect("Core init failed");

// New API (current)
let config = UnifiedBearDogConfig::default();
let core = BearDogCore::new(config);  // Not async anymore
```

**Affected Files**: 
- `hyperoptimized_benchmarks.rs`
- `modernization_performance_validation.rs`

### 2. Missing/Moved Modules
```rust
// Missing modules
use beardog_utils::optimization::OptimizationEngine;  // Module doesn't exist
use beardog_sovereign_science::*;  // Crate not linked

// Changed modules  
use beardog_adapters::universal::*;  // Module structure changed
```

**Affected Files**:
- `clone_optimization_benchmarks.rs`
- `hyperoptimized_benchmarks.rs`
- `sovereign_science_benchmarks.rs`
- `universal_capability_benchmarks.rs`

### 3. Missing Functions/Methods
```rust
// Functions that no longer exist
ResponseTimeMetrics::zero()
LoadMetrics::zero()
ErrorRateMetrics::is_healthy()
generate_performance_report()
```

**Affected Files**:
- `const_optimization_bench.rs`
- `modernization_baseline.rs`

### 4. Type Changes
```rust
// Old
BearDogError::validation("error message".to_string())

// New
BearDogError::validation("error message")  // Takes &str, not String
```

### 5. Missing Dependencies
```rust
use futures::future::join_all;  // futures crate not in dev-dependencies
```

**Affected Files**:
- `production_performance_suite.rs`

### 6. Struct Field Changes
```rust
// Old struct definition
EndpointConfig {
    timeout: Duration::from_secs(30),
    max_retries: 3,
}

// New struct (missing field)
EndpointConfig {
    timeout: Duration::from_secs(30),
    max_retries: 3,
    circuit_breaker: CircuitBreakerConfig::default(),  // NEW REQUIRED FIELD
}
```

---

## Benchmark Files Status

| File | Lines | Issues | Priority |
|------|-------|--------|----------|
| `comprehensive_benchmarks.rs` | 101 | Missing OptimizationEngine | P1 |
| `clone_optimization_benchmarks.rs` | 147 | Missing optimization module | P2 |
| `const_optimization_bench.rs` | 50 | Missing ::zero() methods | P2 |
| `hyperoptimized_benchmarks.rs` | 78 | API signature changes | P1 |
| `modernization_baseline.rs` | 32 | Missing function | P3 |
| `modernization_performance_validation.rs` | 91 | API changes + type errors | P1 |
| `production_performance_suite.rs` | 134 | Missing futures dependency | P1 |
| `sovereign_science_benchmarks.rs` | 54 | Unlinked crate | P3 |
| `unified_modernization_benchmarks.rs` | 209 | Missing types | P1 |
| `universal_capability_benchmarks.rs` | 393 | API changes + fields | P1 |
| `zero_copy_benchmarks.rs` | 80 | Likely OK (check) | P1 |

**Total**: 11 files, ~1,369 lines

---

## Restoration Plan

### Phase 1: Fix High-Priority Benchmarks (4-5 hours)
1. `comprehensive_benchmarks.rs` - Core benchmarks
2. `production_performance_suite.rs` - Production validation
3. `unified_modernization_benchmarks.rs` - Main benchmark suite
4. `hyperoptimized_benchmarks.rs` - Performance critical
5. `zero_copy_benchmarks.rs` - Verify it works

### Phase 2: Fix Medium-Priority Benchmarks (2-3 hours)
6. `universal_capability_benchmarks.rs` - Adapter benchmarks
7. `modernization_performance_validation.rs` - Validation suite
8. `clone_optimization_benchmarks.rs` - Optimization tracking

### Phase 3: Fix Low-Priority Benchmarks (2-3 hours)
9. `const_optimization_bench.rs` - Const evaluation
10. `modernization_baseline.rs` - Baseline measurements
11. `sovereign_science_benchmarks.rs` - Experimental framework

### Phase 4: Integration & CI (1 hour)
- Add benchmarks to CI/CD pipeline
- Create benchmark running documentation
- Set up performance regression tracking

---

## Required Changes Per File

### `comprehensive_benchmarks.rs`
```rust
// Add import
use beardog_core::genetic_optimizer::OptimizationEngine;  // If exists
// Or implement minimal benchmark version

// Fix initialization
let config = UnifiedBearDogConfig::default();
let core = BearDogCore::new(config);
```

### `hyperoptimized_benchmarks.rs`
```rust
// Fix initialization (not async)
let config = UnifiedBearDogConfig::default();
let core = BearDogCore::new(config);
// Remove .await
```

### `production_performance_suite.rs`
```rust
// Add to Cargo.toml [dev-dependencies]
futures = "0.3"

// Or replace with tokio::join!
```

### `universal_capability_benchmarks.rs`
```rust
// Fix ProviderType enum variant
provider_type: ProviderType::External,  // Or appropriate variant

// Add missing field
circuit_breaker: CircuitBreakerConfig::default(),

// Fix AuthConfig
auth_config: AuthConfig::default(),  // Instead of ::None
```

---

## Decision

**Re-disabled all benchmarks** until API migration can be completed properly.

**Priority**: P2 (Medium)  
- Not blocking v1.0.0 Alpha
- Should be done before v1.0.0 Complete
- Lower priority than test coverage and documentation

**Next Steps**:
1. Focus on documentation and test coverage first
2. Return to benchmarks in Sprint 1 or 2
3. Migrate benchmarks systematically using this guide

---

## Notes

- This is actually good validation of the audit findings
- Confirms benchmarks need restoration work
- API has evolved significantly since benchmarks were written
- Some benchmark targets (OptimizationEngine) may not exist anymore
- May need to create new benchmark targets for current architecture

**Status**: Documented and deferred to later sprint

