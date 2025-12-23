# 📋 Deprecation Migration Guide - BearDog v4.0.0

**Created**: November 7, 2025  
**Target Version**: v4.0.0 (Q1 2026)  
**Current Status**: 340 deprecation markers identified  
**Migration Strategy**: Gradual, non-breaking

---

## 🎯 OVERVIEW

BearDog v3.0.0 has successfully unified types, traits, configs, and error systems. As part of this unification, **340 items have been deprecated** to guide migration to new canonical locations.

**Good News**: All deprecated items still work! This guide helps you migrate at your own pace.

---

## 📊 DEPRECATION SUMMARY

### By Category

| Category | Count | Priority | Timeline |
|----------|-------|----------|----------|
| **Traits** | ~26 items | HIGH | Migrate by v4.0.0 |
| **Types** | ~40 items | MEDIUM | Migrate by v4.0.0 |
| **Functions** | ~80 items | MEDIUM | Migrate by v4.5.0 |
| **Modules** | ~20 items | LOW | Migrate by v5.0.0 |
| **Notices** | ~174 items | INFO | Documentation only |

**Total**: 340 deprecation markers

---

## 🔴 HIGH PRIORITY: Trait Migrations

### 1. Provider Traits (26 items)

#### From: `beardog-traits/src/canonical/`
**Status**: ⚠️ Deprecated since v3.0.0  
**Remove**: v4.0.0 (Q1 2026)

#### To: `beardog-types/src/canonical/providers_unified/traits/`
**Status**: ✅ Production-ready  
**Available**: Now

#### Migration Example

```rust
// ❌ OLD (deprecated)
use beardog_traits::canonical::{
    BaseProvider,
    SecurityProvider,
    HsmProvider,
};

// ✅ NEW (recommended)
use beardog_types::canonical::providers_unified::traits::{
    BaseProvider,
    SecurityProvider,
    HsmProvider,
};

// Or use the re-export in beardog-traits:
use beardog_traits::unified::{
    BearDogProvider,  // Replaces BaseProvider
    SecurityProvider,  // Same name, better implementation
    HsmProvider,       // Same name, native async
};
```

#### Affected Files (~50 consumers)

**High-traffic files** (update first):
```
crates/beardog-core/src/service_discovery/
crates/beardog-adapters/src/universal/
crates/beardog-security/src/providers/
crates/beardog-monitoring/src/providers/
```

**Lower-traffic files** (update as you touch them):
```
crates/beardog-workflows/
crates/beardog-genetics/
tests/integration/
```

#### Migration Steps

1. **Phase 1** (Week 1-2): Update high-traffic files
   ```bash
   # Find usages
   grep -r "use beardog_traits::canonical" crates/beardog-core/
   
   # Update imports
   # Use search-replace in your editor
   ```

2. **Phase 2** (Week 3-4): Update remaining files
   ```bash
   # Find all usages
   grep -r "use beardog_traits::canonical" crates/
   ```

3. **Phase 3** (Week 5-6): Remove deprecated modules
   ```bash
   # After all consumers updated
   rm -rf crates/beardog-traits/src/canonical/
   ```

---

## 🟡 MEDIUM PRIORITY: Type Migrations

### 2. Type Aliases (~40 items)

#### Common Deprecated Types

```rust
// ❌ OLD locations
use beardog_types::aliases::NodeId;
use beardog_types::aliases::ServiceId;

// ✅ NEW canonical locations
use beardog_types::canonical::network::NodeId;
use beardog_types::canonical::network::ServiceId;
```

#### Migration Strategy

**Approach**: Update as you touch files
- No rush - aliases still work
- Update when refactoring nearby code
- Full migration by v4.0.0

---

### 3. Config Types (~30 items)

#### From: Scattered locations
#### To: `beardog-config/src/domains/`

```rust
// ❌ OLD
use beardog_types::config::TimeoutConfig;

// ✅ NEW
use beardog_config::domains::timeouts::TimeoutConfig;
```

**Note**: Config system is 100% complete. Just update import paths!

---

## 🟢 LOW PRIORITY: Function Migrations

### 4. Utility Functions (~80 items)

#### Crypto Utils

```rust
// ❌ OLD
use beardog_utils::crypto_migration::old_hash_function;

// ✅ NEW
use beardog_security::crypto_utils::unified::hash_function;
```

#### String Utils

```rust
// ❌ OLD
use beardog_utils::string_helpers::old_format;

// ✅ NEW (if still needed, otherwise inline)
use beardog_utils::formatting::format_string;
```

**Strategy**: These are low priority. Migrate when refactoring.

---

## 📝 MIGRATION TIMELINE

### v3.0.0 (Current) - Deprecation Notices Added
**Status**: ✅ Complete  
**All deprecated items still work** with compiler warnings

---

### v3.5.0 (Q4 2025) - Migration Period
**Goal**: 50% of consumers migrated

**Actions**:
- Update high-traffic files
- Provide migration examples
- Document patterns

**Deliverables**:
- Migration guide (this document)
- Example migrations
- Updated documentation

---

### v4.0.0 (Q1 2026) - Remove High Priority Deprecations
**Goal**: Remove deprecated traits and high-priority items

**Breaking Changes**:
- Remove `beardog-traits/src/canonical/` module
- Remove deprecated trait re-exports
- Remove deprecated type aliases

**Migration Required Before**:
- All trait consumers must update imports
- Config import paths must be updated
- Type aliases must use canonical locations

**Backward Compatibility**:
- v3.x will remain supported for 6 months
- Security patches for v3.x until Q3 2026

---

### v4.5.0 (Q2 2026) - Remove Medium Priority Deprecations
**Goal**: Remove deprecated functions

**Changes**:
- Remove deprecated utility functions
- Remove old crypto functions
- Clean up string helpers

---

### v5.0.0 (Q3 2026) - Final Cleanup
**Goal**: Zero deprecated items

**Changes**:
- Remove all deprecated modules
- Clean architecture achieved
- Documentation updated

---

## 🔧 MIGRATION TOOLS

### Automated Search & Replace

```bash
# Find all deprecated trait imports
grep -r "use beardog_traits::canonical::" crates/ --include="*.rs"

# Count by file
grep -r "use beardog_traits::canonical::" crates/ --include="*.rs" | cut -d: -f1 | sort | uniq -c | sort -rn

# Find specific deprecated imports
grep -r "use beardog_traits::canonical::BaseProvider" crates/ --include="*.rs"
```

### Update Script (Example)

```bash
#!/bin/bash
# migrate-traits.sh

# Backup first!
git checkout -b migrate-traits-$(date +%Y%m%d)

# Find and replace BaseProvider
find crates/ -name "*.rs" -type f -exec sed -i \
  's/use beardog_traits::canonical::BaseProvider/use beardog_types::canonical::providers_unified::traits::BaseProvider/g' {} +

# Build and test
cargo build --workspace
cargo test --workspace

# If successful, commit
git add -A
git commit -m "chore: migrate BaseProvider to canonical location"
```

---

## ✅ VERIFICATION

### After Migration, Verify:

```bash
# 1. Build passes
cargo build --workspace

# 2. Tests pass
cargo test --workspace

# 3. No new warnings
cargo build --workspace 2>&1 | grep -i "deprecated"

# 4. Clippy happy
cargo clippy --workspace -- -D warnings
```

---

## 📚 REFERENCE

### Deprecated Trait Locations

```
beardog-traits/src/canonical/
├── base.rs          → beardog-types/canonical/providers_unified/traits/base.rs
├── security.rs      → beardog-types/canonical/providers_unified/traits/security.rs
├── hsm.rs           → beardog-types/canonical/providers_unified/traits/hsm.rs
├── crypto.rs        → beardog-types/canonical/providers_unified/traits/crypto.rs
├── workflow.rs      → beardog-types/canonical/providers_unified/traits/workflow.rs
├── cache.rs         → beardog-types/canonical/providers_unified/traits/cache.rs
├── monitoring.rs    → beardog-types/canonical/providers_unified/traits/monitoring.rs
├── database.rs      → beardog-types/canonical/providers_unified/traits/database.rs
└── ai.rs            → beardog-types/canonical/providers_unified/traits/ai.rs
```

### Type Alias Migrations

```
beardog-types/src/aliases.rs → beardog-types/canonical/*/
├── NodeId      → canonical/network/node.rs
├── ServiceId   → canonical/network/service.rs
├── KeyId       → canonical/security/key.rs
└── ...
```

---

## 💡 BEST PRACTICES

### Do's ✅

1. **Update imports gradually** - No need to rush
2. **Test after each change** - Verify nothing breaks
3. **Commit frequently** - Small, focused changes
4. **Update documentation** - Keep docs in sync
5. **Communicate** - Let team know about migrations

### Don'ts ❌

1. **Don't mass-replace** - Review each change
2. **Don't skip tests** - Always verify
3. **Don't break APIs** - Maintain backward compat
4. **Don't remove without warning** - Deprecate first
5. **Don't rush** - Take time to do it right

---

## 🆘 TROUBLESHOOTING

### Issue: Import not found after migration

**Solution**: Check new location in `beardog-types/canonical/providers_unified/traits/`

```rust
// If old import fails
use beardog_traits::canonical::BaseProvider;  // ❌

// Try new locations
use beardog_types::canonical::providers_unified::traits::BaseProvider;  // ✅
// Or
use beardog_traits::unified::BearDogProvider;  // ✅
```

### Issue: Type mismatch after migration

**Solution**: Ensure all related types are updated together

```rust
// Update ALL related imports at once
use beardog_types::canonical::providers_unified::traits::{
    BaseProvider,
    SecurityProvider,
    HsmProvider,
};
```

### Issue: Tests fail after migration

**Solution**: Update test imports too!

```rust
// In test files
#[cfg(test)]
mod tests {
    use beardog_types::canonical::providers_unified::traits::*;
    // Not the old canonical imports
}
```

---

## 📞 SUPPORT

### Questions?
- Check this guide first
- Review examples in `examples/`
- Read `COMPREHENSIVE_UNIFICATION_REVIEW_NOV_7_2025.md`
- Consult `ARCHITECTURE.md`

### Report Issues
- File GitHub issue with "migration" label
- Include code example
- Describe expected vs actual behavior

---

## 🎯 SUCCESS METRICS

### Migration Complete When:

- [ ] Zero deprecated trait usages
- [ ] Zero deprecated type usages
- [ ] All imports use canonical locations
- [ ] All tests passing
- [ ] Documentation updated
- [ ] Build warnings <10

**Target Date**: v4.0.0 (Q1 2026)

---

## 🎉 CELEBRATION MILESTONES

### 25% Migrated
- High-traffic files updated
- Core modules using new traits
- Tests still passing

### 50% Migrated
- Most consumers updated
- Documentation current
- Performance verified

### 75% Migrated
- Rare usages remaining
- Ready for final push
- v4.0.0-beta released

### 100% Migrated 🏆
- Zero deprecated usages
- Clean architecture achieved
- v4.0.0 released!

---

**Created**: November 7, 2025  
**Target**: v4.0.0 (Q1 2026)  
**Status**: Ready for execution  
**Priority**: Gradual migration (no rush!)

🐻 **BearDog: Smooth Migrations, Zero Breakage** 🚀

