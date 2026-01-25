# 🧹 Archive Code Cleanup Plan - January 25, 2026

**Goal**: Remove archived/deprecated code files while preserving docs as fossil record  
**Principle**: Docs stay, code goes (if truly orphaned/deprecated)

---

## 📋 **FOUND ARCHIVE/DEPRECATED CODE**

### 1. **Deprecated Code Files** (1 file)
```
crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto/tls.rs.deprecated_jan24_2026
```
- **Size**: 2,175 lines
- **Status**: Complete TLS 1.3 implementation (superseded)
- **Action**: ✅ **DELETE** - Already deprecated, not in use
- **Reason**: Marked as `.deprecated_jan24_2026`, comprehensive tests exist for replacement

### 2. **Orphaned Code in Archives** (2 files)
```
archives/orphaned_code_jan_24_2026/provider_dispatch.rs
archives/orphaned_code_jan_24_2026/safe_keystore_replacement.rs
```
- **Status**: Already archived, historical reference
- **Action**: ✅ **KEEP** - Already in archives, maintains audit trail
- **Reason**: Part of fossil record, small size, shows evolution

### 3. **Disabled Test Files** (3 files)
```
./crates/beardog-tunnel/tests/hardware_pkcs11_tests.rs.disabled
./tests/multi_protocol_e2e_tests.rs.disabled
./tests/birdsong_v2_api_unit_tests.rs.disabled
```
- **Status**: Disabled, not running in test suite
- **Action**: 🔍 **REVIEW** - Check if still needed
- **Options**:
  - Re-enable if hardware available
  - Move to archives if obsolete
  - Delete if redundant

---

## 📊 **TODO/FIXME ANALYSIS**

**Total**: 415 instances across 125 files

### Top Categories:
1. **Documentation TODOs** (~40%): Missing docs, expand examples
2. **Feature TODOs** (~30%): Future enhancements, optimizations
3. **Technical Debt** (~20%): Refactoring opportunities
4. **Cleanup TODOs** (~10%): Old comments, legacy code references

### High-Priority Review Areas:
1. `beardog-security/src/key_rotation_manager.rs` - 26 TODOs
2. `beardog-security/src/key_rotation_manager_tests.rs` - 13 TODOs
3. `beardog-types/src/canonical/config/discovery.rs` - 18 TODOs
4. `beardog-types/src/canonical/traits/tls.rs` - 14 TODOs

---

## ✅ **RECOMMENDED ACTIONS**

### Immediate (This Session)
1. **Delete deprecated TLS file** (already superseded)
   ```bash
   rm crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto/tls.rs.deprecated_jan24_2026
   ```

### Near-Term (Next Session)
2. **Review disabled tests**
   - Check if hardware tests still needed (PKCS#11)
   - Evaluate multi-protocol tests (might be superseded)
   - Check BirdSong v2 tests (might be obsolete)

3. **Triage TODO/FIXMEs**
   - Create GitHub issues for legitimate future work
   - Remove outdated TODOs
   - Convert high-priority items to action items

### Long-Term (Phase 3)
4. **Deep TODO Review**
   - key_rotation_manager.rs (39 TODOs total)
   - discovery.rs (18 TODOs)
   - Consolidate or resolve

---

## 🎯 **FALSE POSITIVES TO IGNORE**

### Test Code (Acceptable)
- Hardcoded `127.0.0.1` in test files
- `/tmp/` paths in test fixtures
- Mock implementations in `tests/` directory

### Documentation
- TODOs for future documentation expansion
- Example code with simplified values

### Comments
- Historical context comments
- Design decision notes

---

## 📝 **EXECUTION PLAN**

### Step 1: Safe Deletion ✅
```bash
# Delete deprecated TLS file (superseded, not in use)
rm crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto/tls.rs.deprecated_jan24_2026
```

### Step 2: Verify No References
```bash
# Ensure nothing imports the deleted file
grep -r "tls.rs.deprecated" crates/
grep -r "handlers::crypto::tls" crates/ | grep -v ".rs:" | grep -v "test"
```

### Step 3: Test Build
```bash
# Verify workspace builds cleanly
cargo build --workspace
cargo test --workspace --lib
```

### Step 4: Document Cleanup
```bash
# Update cleanup report
echo "Removed 1 deprecated file (2,175 lines)" >> CLEANUP_LOG.md
```

---

## 🚫 **WHAT NOT TO DELETE**

### Keep All:
1. **Documentation** (`.md` files) - Fossil record
2. **Archives folder** - Historical audit trail
3. **Orphaned code in archives/** - Already preserved
4. **Test fixtures** - May be used in property tests
5. **Legacy script runners** - May be called by CI

---

## 📊 **IMPACT ASSESSMENT**

### Before Cleanup
- Deprecated code: 2,175 lines
- Disabled tests: 3 files
- TODOs: 415 instances

### After Immediate Cleanup
- **Saved**: 2,175 lines removed
- **Build impact**: None (file already deprecated)
- **Test impact**: None (not in test suite)
- **Risk**: **ZERO** (file explicitly marked deprecated)

---

## ✅ **EXECUTION READY**

**Confidence**: ✅ **HIGH** - Safe to proceed  
**Risk**: ✅ **ZERO** - File explicitly deprecated  
**Verification**: ✅ **Build + test after deletion**

**Next**: Execute Step 1 (delete deprecated file)

