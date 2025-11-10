# 🧹 Deprecated Code Removal Plan

**Date**: November 10, 2025  
**Status**: 📋 **PLANNING**  
**Target**: Remove 127 deprecated items

---

## 📊 **Inventory by Crate**

| Crate | Deprecated Items | Priority |
|-------|-----------------|----------|
| **beardog-types** | 74 | 🔴 HIGH |
| **beardog-utils** | 15 | 🟡 MEDIUM |
| **beardog-core** | 11 | 🟡 MEDIUM |
| **beardog-adapters** | 8 | 🟢 LOW |
| **beardog-tunnel** | 6 | 🟢 LOW |
| **beardog-production** | 6 | 🟢 LOW |
| **beardog-monitoring** | 4 | 🟢 LOW |
| **beardog-security** | 3 | 🟢 LOW |
| **Total** | **127** | |

---

## 🎯 **Removal Strategy**

### **Phase 1: Verify No Usage** ✅ (Current)
1. Check for usage in production code
2. Check for usage in tests
3. Identify safe-to-remove candidates

### **Phase 2: Safe Removals**
1. Remove unused deprecated items
2. Update documentation
3. Verify build still works

### **Phase 3: Migrate Usages**
1. Find remaining usages
2. Migrate to new APIs
3. Remove old code

### **Phase 4: Final Cleanup**
1. Remove all remaining deprecated code
2. Update CHANGELOG
3. Bump version to 3.3.0

---

## 🔍 **Analysis: beardog-utils Crypto Functions**

### **Deprecated Functions**
```rust
crates/beardog-utils/src/utils/crypto_utils.rs:
- secure_random_bytes(size) → BearDogCrypto::generate_secure_random
- generate_salt() → BearDogCrypto::generate_secure_random(32)
- generate_nonce(size) → BearDogCrypto::generate_secure_nonce
- sha256_hash(data) → BearDogCrypto::sha256_hash
- hmac_sha256(key, data) → beardog-security HMAC
- verify_hmac_sha256(key, data, sig) → beardog-security HMAC
```

### **Migration Path**
All have clear replacements in `beardog-security` crate.

### **Planned Removal**
v3.3.0 (Q1 2026) - **COMING SOON!**

---

## 🎯 **Quick Wins** (Safe to Remove Now)

### **Category 1: Unused Config Aliases**
Many config type aliases that were deprecated in favor of canonical versions.

**Example**:
```rust
#[deprecated(since = "3.1.0", note = "Use ConnectionPoolConfig instead")]
pub type OldConnectionPoolConfig = ConnectionPoolConfig;
```

**Action**: If no usages found, remove immediately.

### **Category 2: Unused Type Aliases**
Deprecated type aliases in `unified_types.rs`.

**Action**: Check usages, migrate if any, remove.

### **Category 3: Old Monitoring Types**
Deprecated monitoring config types with clear replacements.

**Action**: Migrate remaining usages, remove.

---

## 📝 **Detailed Plan: beardog-utils Crypto**

### **Step 1: Verify Replacements Exist**
```bash
# Check beardog-security has BearDogCrypto
grep -rn "BearDogCrypto" crates/beardog-security/
```

### **Step 2: Find Usages**
```bash
# Find all usages of deprecated functions
grep -rn "secure_random_bytes\|generate_salt" crates/ --include="*.rs"
```

### **Step 3: Migrate Usages**
```rust
// BEFORE
use beardog_utils::utils::crypto_utils::secure_random_bytes;
let bytes = secure_random_bytes(32);

// AFTER
use beardog_security::crypto_utils::BearDogCrypto;
let bytes = BearDogCrypto::generate_secure_random(32);
```

### **Step 4: Remove Functions**
Once no usages remain, delete the deprecated functions.

---

## ⚠️ **Caution Areas**

### **Test Code**
Some deprecated functions might be used in tests. Options:
1. Migrate tests to new APIs (preferred)
2. Keep deprecated code for tests (not ideal)

### **External Dependencies**
Check if any ecosystem projects depend on deprecated APIs:
- Songbird
- Primal
- Other integrations

### **Documentation**
Update all documentation references to deprecated APIs.

---

## 🚀 **Implementation Schedule**

### **Week 1: Analysis** (Current)
- [x] Count deprecated items
- [ ] Categorize by type
- [ ] Identify dependencies
- [ ] Plan removal order

### **Week 2: Safe Removals**
- [ ] Remove unused type aliases
- [ ] Remove unused config types
- [ ] Run full test suite
- [ ] Update documentation

### **Week 3: Usage Migration**
- [ ] Migrate production code
- [ ] Migrate test code
- [ ] Update examples
- [ ] Verify builds

### **Week 4: Final Cleanup**
- [ ] Remove all deprecated code
- [ ] Update CHANGELOG
- [ ] Bump to v3.3.0
- [ ] Create release notes

---

## 📊 **Success Metrics**

```
Target Metrics:
Deprecated Items:    127 → 0     (-100%)
Build Time:          No regression
Test Pass Rate:      100%
Documentation:       100% updated
Breaking Changes:    Documented
```

---

## 💡 **Best Practices**

### **1. Always Check Usages First**
Never remove code without verifying it's unused.

### **2. Migrate Before Removing**
Provide migration path before deletion.

### **3. Comprehensive Testing**
Run full test suite after each batch of removals.

### **4. Clear Communication**
Document all breaking changes in CHANGELOG.

### **5. Gradual Approach**
Remove in batches, not all at once.

---

## 🔗 **Related Work**

- Pure Rust migration (completed)
- OpenSSL elimination (completed)
- SIMD cross-platform (completed)
- Error handling improvements (in progress)
- Idiomatic Rust patterns (in progress)

---

**Status**: 📋 **PLANNING PHASE**  
**Next**: Verify no usages, begin safe removals  
**Timeline**: 4 weeks to v3.3.0  
**Risk**: LOW (well-documented deprecations)

**Let's clean up this technical debt!** 🧹✨

