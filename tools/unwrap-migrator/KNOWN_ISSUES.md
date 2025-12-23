# Known Issues - BearDog Unwrap Migrator

## Issue #1: Test File with Thread Spawning

**File**: `crates/beardog-utils/src/memory_pools_safe.rs`

**Problem**: The migrator keeps attempting to migrate this test file even with `--exclude-tests` flag.

**Details**:
- Test function returns `Result<(), Box<dyn std::error::Error>>`
- Contains `thread::spawn()` closures that don't return Result
- Contains `handle.join()` which returns `Result<_, Box<dyn Any + Send>>`
- Tool incorrectly converts these to use `?` operator

**Error**:
```
error[E0277]: `?` couldn't convert the error
   --> crates/beardog-utils/src/memory_pools_safe.rs:399:26
    |
399 |             handle.join()?;
    |                    ------^ doesn't have a size known at compile-time
```

**Root Cause**:
1. File-level detection sees test module but migrates anyway
2. Doesn't check that spawned closures return `()`
3. Thread `join()` returns incompatible error type

**Workaround**: Manual revert with `git checkout`

**Fix Needed**:
1. Better test module detection
2. Check closure return types before migration
3. Special handling for thread `join()` patterns
4. Don't migrate if error types are incompatible

**Status**: Known limitation - manual review required for files with threading

---

## Lessons Learned

### What the Tool Does Well
- ✅ High confidence migrations (90%+)
- ✅ Production code without threading
- ✅ Simple unwrap → ? conversions
- ✅ Fast analysis

### What Needs Improvement
- ⚠️ Test code detection (still migrates some tests)
- ⚠️ Thread pattern handling
- ⚠️ Closure return type checking
- ⚠️ Error type compatibility checking

---

**Recommendation**: Always run tests after migration and be prepared to revert specific files.

