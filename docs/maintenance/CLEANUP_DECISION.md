# 🧹 Archive Code Cleanup - Execution Plan

**Date**: January 13, 2026  
**Status**: Ready to execute

---

## 🎯 **What to Clean**

### **1. Disabled Tests** (DELETE - Superseded)
```
tests/btsp_jsonrpc_chaos_tests.rs.disabled  
tests/btsp_jsonrpc_e2e_tests.rs.disabled
tests/capability_ipc_e2e_tests.rs.disabled
tests/trust_api_e2e_tests.rs.disabled
```

**Reason**: 
- We have modern chaos tests in `tests/unix_socket_chaos_tests.rs`
- We have E2E tests in `tests/e2e/` directory
- These were disabled during refactoring
- Functionality is covered by newer, better tests

**Action**: DELETE (safe - already disabled, not compiled)

---

### **2. Archive Crates** (KEEP AS-IS - Already Excluded)
```
crates/beardog-crypto/      # Minimal stub
crates/beardog-networking/  # Empty stub
```

**Reason**:
- Already excluded in `Cargo.toml`
- Not interfering with builds
- Serve as placeholders for future work

**Action**: KEEP (already properly archived)

---

### **3. Comments** (KEEP - Valuable History)
- TODO comments (81) - Future work tracking
- Legacy mentions (1,263) - Evolution documentation  
- REMOVED/MIGRATED comments (49) - Fossil record

**Action**: KEEP (docs as fossil record per requirement)

---

## ✅ **Safe Deletions**

These files are safe to delete:
1. Already disabled (`.disabled` extension)
2. Not compiled into any build
3. Functionality superseded by modern tests
4. No dependencies on them

---

## 🚀 **Execution**

```bash
# Delete disabled tests
rm tests/btsp_jsonrpc_chaos_tests.rs.disabled
rm tests/btsp_jsonrpc_e2e_tests.rs.disabled
rm tests/capability_ipc_e2e_tests.rs.disabled
rm tests/trust_api_e2e_tests.rs.disabled

# Git commit
git add -A
git commit -m "chore: remove superseded disabled tests

- Removed 4 .disabled test files
- Functionality covered by modern test suite
- tests/unix_socket_chaos_tests.rs covers chaos testing
- tests/e2e/ directory covers end-to-end testing
- Cleanup after Phase 1 completion and pure Rust evolution"

# Push via SSH
git push origin main
```

---

## 📊 **Impact**

**Files Deleted**: 4  
**Lines Removed**: ~2,000 (approx)  
**Build Impact**: NONE (files already disabled)  
**Test Impact**: NONE (superseded by modern tests)  
**Risk**: ZERO (safe cleanup)

---

**Status**: ✅ READY TO EXECUTE  
**Safety**: 💯 SAFE  
**Impact**: 🧹 CLEANER CODEBASE

