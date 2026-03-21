# 🧹 Archive Code Cleanup Plan

**Date**: January 13, 2026  
**Goal**: Remove archive code, keep docs as fossil record  
**Found**: 4 disabled tests, minimal actual archive code

---

## 📊 **Findings Summary**

### **Disabled Tests** (4 files in `tests/`)
```
tests/btsp_jsonrpc_chaos_tests.rs.disabled
tests/btsp_jsonrpc_e2e_tests.rs.disabled  
tests/capability_ipc_e2e_tests.rs.disabled
tests/trust_api_e2e_tests.rs.disabled
```

**Status**: Review if these should be re-enabled or deleted

### **Comments to Review**
- **TODO/FIXME**: 81 matches across 62 files
- **Legacy mentions**: 1,263 matches across 247 files (mostly comments)

### **Excluded Crates** (Already in .gitignore)
```
crates/beardog-crypto/      # Excluded workspace member
crates/beardog-networking/  # Excluded workspace member
```

**Status**: These are ALREADY excluded from builds

---

## ✅ **What's Actually Clean**

### **No .deprecated Files**
- Zero `.deprecated` files found (good!)
- Previous `.deprecated` files were already cleaned up

### **Archive Properly Excluded**
- `beardog-crypto` and `beardog-networking` in workspace exclude list
- Not compiled, not interfering with builds

### **Good Comment Hygiene**  
Most "legacy" mentions are:
- Documentation of evolution (EVOLVED:, MIGRATED:)
- Explanations of changes (REMOVED:, CLEANED:)
- Historical context (valuable for fossil record!)

---

## 🎯 **Recommended Actions**

### **1. Review Disabled Tests** (Priority: MEDIUM)

**Check Each File**:
```bash
# Review and decide: Re-enable or delete?
tests/btsp_jsonrpc_chaos_tests.rs.disabled
tests/btsp_jsonrpc_e2e_tests.rs.disabled
tests/capability_ipc_e2e_tests.rs.disabled
tests/trust_api_e2e_tests.rs.disabled
```

**Questions**:
- Were these disabled temporarily or permanently?
- Do we have equivalent tests now?
- Should we re-enable with modern patterns?

**Recommendation**: DELETE if superseded, RE-ENABLE if still relevant

---

### **2. TODO Audit** (Priority: LOW)

**81 TODOs found** - Most are legitimate:
- Future improvements
- Phase 2/3 work
- Documentation reminders

**Action**: Keep TODOs (they're tracked in evolution plan)

---

### **3. Keep Archive Crates Excluded** (Priority: DONE ✅)

**Current State**: Already excluded in `Cargo.toml`:
```toml
exclude = [
    "crates/beardog-crypto",
    "crates/beardog-networking",
    ...
]
```

**Action**: NO CHANGE needed (working as intended)

---

## 🔍 **Specific Disabled Tests Review**

### **Test 1: btsp_jsonrpc_chaos_tests.rs.disabled**
**Lines**: Check first 20 lines to understand

### **Test 2: btsp_jsonrpc_e2e_tests.rs.disabled**  
**Lines**: Check first 20 lines to understand

### **Test 3: capability_ipc_e2e_tests.rs.disabled**
**Purpose**: Unknown (needs review)

### **Test 4: trust_api_e2e_tests.rs.disabled**
**Purpose**: Unknown (needs review)

---

## 💡 **Decision Framework**

For each disabled test:

### **Re-Enable If**:
- ✅ Still testing valid functionality
- ✅ No equivalent modern test exists
- ✅ Can be updated to modern patterns (no sleeps, concurrent)

### **Delete If**:
- ❌ Functionality removed
- ❌ Superseded by better tests
- ❌ Would require major rewrite

---

## 🎯 **Quick Wins**

### **Immediate Actions** (5-10 minutes)
1. Review 4 disabled test files
2. Delete if obviously superseded
3. Document decision in commit message

### **No Action Needed**
- ✅ Archive crates (already excluded)
- ✅ Most TODO comments (tracked in evolution)
- ✅ Legacy mentions in comments (fossil record!)
- ✅ Documentation files (keep all as requested)

---

## 📝 **Cleanup Checklist**

- [ ] Review `btsp_jsonrpc_chaos_tests.rs.disabled`
- [ ] Review `btsp_jsonrpc_e2e_tests.rs.disabled`
- [ ] Review `capability_ipc_e2e_tests.rs.disabled`
- [ ] Review `trust_api_e2e_tests.rs.disabled`
- [ ] Decision: Delete or re-enable each
- [ ] Document reasoning
- [ ] Git commit with clear message
- [ ] Push via SSH

---

## 🚀 **Next Steps**

1. **Review disabled tests** (check content below)
2. **Make delete/re-enable decision**
3. **Execute cleanup**
4. **Git commit + push**

**Time Estimate**: 10-15 minutes total

---

**Status**: ⏸️ **PENDING REVIEW**  
**Priority**: MEDIUM (non-blocking)  
**Complexity**: LOW (straightforward decisions)

🧹 **Clean codebase = happy developers!**

