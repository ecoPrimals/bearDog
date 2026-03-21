# 🏷️ Test Categorization & Tagging Plan

**Date:** October 30, 2025  
**Purpose:** Organize 3,896 tests with proper categories and tags

---

## 📊 CURRENT STATE

### Test Count
- **Total Tests:** 3,896
- **Passing:** 3,879 (99.6%)
- **Ignored:** 17 (0.4%)
- **Hidden:** ~1,139 (test markers not running)

### Current Organization
- ✅ Good: 84 test suites logically organized by crate
- ⚠️ Missing: Test category tags (unit, integration, e2e, etc.)
- ⚠️ Missing: Test domain tags (security, networking, etc.)
- ⚠️ Missing: Test priority tags (critical, important, nice-to-have)

---

## 🎯 PROPOSED CATEGORIZATION

### Category Tags
```rust
/// TEST_CATEGORY: unit
/// TEST_CATEGORY: integration
/// TEST_CATEGORY: e2e
/// TEST_CATEGORY: chaos
/// TEST_CATEGORY: property
/// TEST_CATEGORY: benchmark
/// TEST_CATEGORY: smoke
```

### Domain Tags
```rust
/// TEST_DOMAIN: core
/// TEST_DOMAIN: security
/// TEST_DOMAIN: hsm
/// TEST_DOMAIN: networking
/// TEST_DOMAIN: workflows
/// TEST_DOMAIN: genetics
/// TEST_DOMAIN: monitoring
/// TEST_DOMAIN: adapters
/// TEST_DOMAIN: config
```

### Priority Tags
```rust
/// TEST_PRIORITY: critical    // Production blockers
/// TEST_PRIORITY: important   // Core functionality
/// TEST_PRIORITY: normal      // Standard tests
/// TEST_PRIORITY: optional    // Nice-to-have
```

### Speed Tags
```rust
/// TEST_SPEED: fast    // <100ms
/// TEST_SPEED: medium  // 100ms-1s
/// TEST_SPEED: slow    // >1s
```

---

## 📋 TAGGING STRATEGY

### Phase 1: Automatic Tagging (Week 1)
**Based on file location and naming:**

1. **Category by location:**
   - `crates/*/src/*.rs` → unit
   - `crates/*/tests/*.rs` → integration
   - `tests/e2e/*.rs` → e2e
   - `tests/chaos/*.rs` → chaos
   - `benches/*.rs` → benchmark

2. **Domain by crate:**
   - `beardog-security/` → security
   - `beardog-tunnel/` → hsm
   - `beardog-networking/` → networking
   - `beardog-workflows/` → workflows
   - etc.

3. **Speed by measurement:**
   - Run tests with `--report-time`
   - Auto-tag based on duration

### Phase 2: Manual Refinement (Week 2)
**Review and adjust:**

1. Verify auto-tags
2. Add priority tags
3. Add special markers
4. Document test purposes

### Phase 3: Enforcement (Week 3)
**CI/CD integration:**

1. Require tags on new tests
2. Run tests by category
3. Parallel execution by speed
4. Coverage by domain

---

## 🔧 IMPLEMENTATION

### Step 1: Create Tagging Script
```python
#!/usr/bin/env python3
"""Auto-tag tests based on location and patterns"""

import re
import os
from pathlib import Path

def categorize_test_file(filepath):
    """Determine test category from file path"""
    if '/tests/e2e/' in filepath:
        return 'e2e'
    elif '/tests/chaos/' in filepath:
        return 'chaos'
    elif '/benches/' in filepath:
        return 'benchmark'
    elif '/tests/' in filepath:
        return 'integration'
    else:
        return 'unit'

def domain_from_crate(filepath):
    """Extract domain from crate name"""
    match = re.search(r'beardog-(\w+)/', filepath)
    if match:
        return match.group(1)
    return 'core'

def add_tags_to_test(content, category, domain):
    """Add doc comment tags above #[test]"""
    # Pattern: find #[test] or #[tokio::test]
    # Insert tags as doc comments above
    pass  # Implementation details

# Process all test files...
```

### Step 2: Tag Format Examples
```rust
// Before (no tags)
#[test]
fn test_api_creation() {
    // ...
}

// After (with tags)
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: security
/// TEST_PRIORITY: critical
#[test]
fn test_api_creation() {
    // ...
}
```

### Step 3: Test Selection
```bash
# Run only unit tests
cargo test --workspace -- --test-threads=1 \
    $(rg -l "TEST_CATEGORY: unit" crates/)

# Run security domain tests
cargo test --workspace -- --test-threads=1 \
    $(rg -l "TEST_DOMAIN: security" crates/)

# Run fast tests only
cargo test --workspace -- --test-threads=1 \
    $(rg -l "TEST_SPEED: fast" crates/)
```

---

## 📊 EXPECTED DISTRIBUTION

### By Category (Estimated)
```
Unit:         ~3,400 tests (87%)
Integration:  ~400 tests (10%)
E2E:          ~50 tests (1.3%)
Chaos:        ~30 tests (0.8%)
Property:     ~10 tests (0.3%)
Benchmark:    ~6 tests (0.2%)
```

### By Domain (Estimated)
```
Security/HSM: ~1,100 tests (28%)
Core/Types:   ~1,000 tests (26%)
Workflows:    ~500 tests (13%)
Networking:   ~400 tests (10%)
Genetics:     ~300 tests (8%)
Other:        ~596 tests (15%)
```

### By Priority (To be determined)
```
Critical:     ~500 tests (13%) - Must pass
Important:    ~1,500 tests (38%) - Core functionality
Normal:       ~1,700 tests (44%) - Standard
Optional:     ~196 tests (5%) - Edge cases
```

---

## 🎯 BENEFITS

### For Development
- ✅ Run relevant tests quickly
- ✅ Skip slow tests during iteration
- ✅ Focus on critical tests first
- ✅ Better test organization

### For CI/CD
- ✅ Parallel test execution
- ✅ Targeted test runs
- ✅ Faster feedback loops
- ✅ Better resource utilization

### For Coverage
- ✅ Coverage by domain
- ✅ Coverage by category
- ✅ Identify gaps easily
- ✅ Track improvements

### For Maintenance
- ✅ Find related tests
- ✅ Understand test purpose
- ✅ Refactor with confidence
- ✅ Document test intent

---

## 📅 TIMELINE

### Week 1: Foundation
- Day 1-2: Create tagging script
- Day 3-4: Auto-tag all tests
- Day 5: Verify and test

### Week 2: Refinement
- Day 1-3: Manual review and adjustment
- Day 4-5: Add priority tags
- Day 5: Documentation

### Week 3: Integration
- Day 1-2: CI/CD integration
- Day 3-4: Test selection scripts
- Day 5: Team training

### Ongoing
- Add tags to new tests
- Maintain consistency
- Update as needed

---

## 🔧 TOOLS NEEDED

1. **Tagging Script** - Python or Rust
2. **Verification Script** - Check tag consistency
3. **Selection Script** - Run tests by tag
4. **CI Integration** - GitHub Actions / GitLab CI
5. **Documentation** - Tag guide for team

---

## 📋 NEXT STEPS

### Immediate
1. ✅ Plan created (this document)
2. ⏳ Write tagging script
3. ⏳ Test on sample files
4. ⏳ Run on full codebase

### This Week
1. Auto-tag all 3,896 tests
2. Verify tagging accuracy
3. Create test selection scripts
4. Document usage

### Next Week
1. Manual refinement pass
2. Add priority tags
3. CI/CD integration
4. Team rollout

---

**Status:** ✅ PLAN COMPLETE  
**Ready for:** Implementation  
**Impact:** Better test organization and execution

🏷️ **Organizing 3,896 Tests for Maximum Efficiency!**

