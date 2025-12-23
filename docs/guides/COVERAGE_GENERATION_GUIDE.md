# Test Coverage Generation Guide

**BearDog Project** - November 22, 2025  
**Estimated Current Coverage**: ~80%  
**Target Coverage**: 85-90%

---

## 🎯 Quick Start

### Generate Coverage Report
```bash
# Generate HTML coverage report (recommended)
cargo llvm-cov --workspace --html

# Open report in browser
open target/llvm-cov/html/index.html  # macOS
xdg-open target/llvm-cov/html/index.html  # Linux
```

### Generate LCOV Format
```bash
# For CI/CD integration
cargo llvm-cov --workspace --lcov --output-path lcov.info

# Upload to Codecov (if configured)
bash <(curl -s https://codecov.io/bash)
```

---

## 📊 Current Status

### Verified Test Counts (Nov 22, 2025)
| Crate | Tests | Status |
|-------|-------|--------|
| beardog-types | 1,265 | ✅ 100% pass |
| beardog-security | 876 | ✅ 100% pass |
| beardog-core | 551 | ✅ 100% pass |
| **Total Verified** | **2,692+** | **✅** |

### Estimated Coverage
- **Current**: ~80% (based on previous audit)
- **Target**: 85-90%
- **Grade Impact**: A- (95/100) → A (96-97/100)

---

## 🔧 Commands Reference

### Basic Coverage
```bash
# Text summary
cargo llvm-cov --workspace

# HTML report (best for analysis)
cargo llvm-cov --workspace --html

# JSON format (for tooling)
cargo llvm-cov --workspace --json --output-path coverage.json
```

### Advanced Options
```bash
# Exclude test code from coverage
cargo llvm-cov --workspace --html --ignore-filename-regex tests

# Specific crate
cargo llvm-cov --package beardog-core --html

# With features
cargo llvm-cov --workspace --all-features --html

# Clean before run
cargo llvm-cov clean && cargo llvm-cov --workspace --html
```

### Per-Crate Analysis
```bash
# Core crates
cargo llvm-cov --package beardog-core --html
cargo llvm-cov --package beardog-security --html
cargo llvm-cov --package beardog-tunnel --html
cargo llvm-cov --package beardog-types --html

# Supporting crates
cargo llvm-cov --package beardog-adapters --html
cargo llvm-cov --package beardog-monitoring --html
cargo llvm-cov --package beardog-auth --html
```

---

## 📈 Interpreting Results

### Coverage Goals by Crate Type

#### Critical Crates (Target: 85%+)
- `beardog-security`: Security operations
- `beardog-core`: Core business logic
- `beardog-auth`: Authentication/authorization
- `beardog-tunnel`: Secure communications

#### Standard Crates (Target: 75%+)
- `beardog-types`: Type definitions
- `beardog-adapters`: Integration adapters
- `beardog-monitoring`: Observability
- `beardog-utils`: Utilities

#### Support Crates (Target: 60%+)
- `beardog-deploy`: Deployment tools
- `beardog-cli`: Command-line interface
- `beardog-benchmarks`: Performance tests

### Quality Thresholds
| Coverage | Grade | Status |
|----------|-------|--------|
| 90%+ | A+ | Exceptional |
| 85-89% | A | Excellent |
| 75-84% | A- | Very Good |
| 65-74% | B+ | Good |
| < 65% | B or lower | Needs Work |

**Current Status**: ~80% = **A-** ✅

---

## 🎯 Improving Coverage

### Priority Areas

#### 1. Error Paths (Highest Impact)
```rust
// Add tests for error conditions
#[test]
fn test_network_timeout_error() {
    // Test timeout scenarios
}

#[test]
fn test_invalid_config_error() {
    // Test config validation
}
```

#### 2. Edge Cases
```rust
// Test boundary conditions
#[test]
fn test_maximum_capacity() {
    // Test at capacity limits
}

#[test]
fn test_zero_elements() {
    // Test empty scenarios
}
```

#### 3. Concurrent Scenarios
```rust
// Test thread safety
#[tokio::test]
async fn test_concurrent_access() {
    // Test race conditions
}
```

---

## 🚀 CI/CD Integration

### GitHub Actions Example
```yaml
name: Coverage

on: [push, pull_request]

jobs:
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install llvm-cov
        run: cargo install cargo-llvm-cov
      
      - name: Generate coverage
        run: cargo llvm-cov --workspace --lcov --output-path lcov.info
      
      - name: Upload to Codecov
        uses: codecov/codecov-action@v3
        with:
          files: lcov.info
```

---

## 📝 Best Practices

### 1. Regular Monitoring
- Generate coverage reports weekly
- Track trends over time
- Set coverage gates in CI/CD

### 2. Focus on Critical Paths
- Prioritize security-sensitive code
- Ensure error paths are tested
- Cover edge cases and boundaries

### 3. Quality Over Quantity
- **80% with good tests > 100% with weak tests**
- Focus on meaningful assertions
- Test behavior, not implementation

### 4. Incremental Improvement
- Set realistic targets (5% per sprint)
- Add tests with each new feature
- Don't sacrifice test quality for coverage %

---

## 🐛 Common Issues

### Issue: Slow Coverage Generation
**Solution**: Run coverage on specific crates
```bash
# Instead of --workspace
cargo llvm-cov --package beardog-core --html
```

### Issue: Out of Memory
**Solution**: Exclude large test fixtures
```bash
cargo llvm-cov --workspace --html -- --test-threads=1
```

### Issue: Flaky Tests
**Solution**: Fix flaky tests before measuring coverage
```bash
# Run tests multiple times
for i in {1..10}; do cargo test; done
```

---

## 📊 Sample Report Interpretation

### Example Output
```
Filename                      Coverage   Lines   Missed
----------------------------------------------------------------
beardog-core/src/bootstrap.rs    92.5%    400      30
beardog-core/src/discovery.rs    87.3%    320      41
beardog-core/src/registry.rs     95.2%    210      10
----------------------------------------------------------------
Total                            91.7%    930      81
```

### Analysis
- **92.5% bootstrap.rs**: Excellent, minor edge cases missed
- **87.3% discovery.rs**: Good, focus on uncovered lines
- **95.2% registry.rs**: Outstanding, minimal gaps

### Action Items
1. Review `discovery.rs` uncovered lines
2. Add tests for missed edge cases
3. Verify error paths are covered

---

## 🎯 Current Recommendations

### For A Grade (96-97/100)
**Target**: 85% overall coverage

**Focus Areas**:
1. `beardog-tunnel`: Add HSM error path tests
2. `beardog-core`: Add bootstrap edge case tests
3. `beardog-adapters`: Add adapter chain failure tests

**Estimated Effort**: 4-6 hours

### For A+ Grade (100/100)
**Target**: 90% overall coverage

**Additional Requirements**:
1. Comprehensive chaos tests
2. Fault injection tests
3. Performance regression tests
4. Documentation examples

**Estimated Effort**: 12-16 hours

---

## 📚 Resources

### Official Documentation
- [cargo-llvm-cov docs](https://github.com/taiki-e/cargo-llvm-cov)
- [LLVM coverage mapping](https://llvm.org/docs/CoverageMappingFormat.html)

### BearDog Documentation
- `TESTING_GUIDE.md`: Test writing guidelines
- `MODERN_CONCURRENT_TEST_PATTERNS.md`: Async test patterns
- `CHAOS_AND_FAULT_TESTING_GUIDE.md`: Chaos testing
- `CONCURRENT_SAFE_TESTING_GUIDE.md`: Thread safety tests

---

## ✅ Quick Verification

After making coverage improvements:

```bash
# 1. Generate report
cargo llvm-cov --workspace --html

# 2. Open report
open target/llvm-cov/html/index.html

# 3. Check summary
cargo llvm-cov --workspace | grep "Total"

# 4. Verify tests pass
cargo test --workspace

# 5. Check for regressions
cargo clippy --workspace
```

---

**Last Updated**: November 22, 2025  
**Current Coverage**: ~80% (A-)  
**Status**: ✅ Production-ready, improvements optional

**Note**: Coverage generation can take 5-15 minutes for the full workspace.

