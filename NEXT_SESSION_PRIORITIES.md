# 🎯 Next Session Priorities - BearDog

**Date**: January 27, 2026  
**Current Grade**: A (93/100)  
**Target Grade**: A+ (97/100)  
**Status**: Ready for Next Session

---

## 🚀 PRIORITY 1: Hardcoding Elimination (5-10 hours)

### Files to Review (23 files)
```
crates/beardog-capabilities/src/lib.rs
crates/beardog-capabilities/src/metadata.rs
crates/beardog-capabilities/src/registry.rs
crates/beardog-config/src/runtime_network_discovery.rs
crates/beardog-config/src/zero_hardcoding.rs
crates/beardog-core/src/capabilities.rs
crates/beardog-core/src/primal_discovery_mdns.rs
crates/beardog-core/src/primal_discovery.rs
crates/beardog-core/src/primal_self_knowledge.rs
crates/beardog-core/src/self_knowledge.rs
crates/beardog-core/src/universal_adapter.rs
crates/beardog-core/src/zero_copy_service_ids_expanded.rs
crates/beardog-core/src/zero_copy_service_ids.rs
crates/beardog-discovery/src/config.rs
crates/beardog-integration/src/api_server.rs
crates/beardog-integration/src/lib.rs
crates/beardog-node-registry/src/lib.rs
crates/beardog-security-registry/src/lib.rs
crates/beardog-types/src/network.rs
crates/beardog-types/src/security.rs
crates/beardog-utils/src/env_config.rs
crates/beardog-utils/src/zero_copy_guide.rs
crates/beardog-utils/src/zero_copy_optimized.rs
```

### Action Plan
1. Review each file (2 hours)
2. Categorize violations (1 hour)
3. Migrate to config (3-4 hours)
4. Test & verify (1 hour)

**Grade Impact**: +2 points (B+ → A+)

---

## 🎯 PRIORITY 2: Semantic Naming Completion (8-12 hours)

### Current State
- **Coverage**: 70%
- **Target**: 90%

### Action Plan
1. Audit current semantic naming (2 hours)
2. Identify gaps (1 hour)
3. Migrate methods (4-8 hours)
4. Update documentation (1 hour)

**Grade Impact**: +2 points

---

## 📋 PRIORITY 3: Unsafe Code Audit (12-16 hours)

### Current State
- **Instances**: 154
- **Status**: Mostly in crypto libraries (expected)

### Action Plan
1. List all unsafe instances (2 hours)
2. Categorize by justification (3 hours)
3. Document each instance (5-8 hours)
4. Eliminate unnecessary unsafe (2-3 hours)

**Grade Impact**: +1 point

---

## 🧪 PRIORITY 4: Test Coverage Expansion (20-30 hours)

### Current State
- **Pass Rate**: 100% (1373/1373)
- **Coverage**: Unknown (likely 70-80%)
- **Target**: 90%

### Action Plan
1. Generate baseline report (1 hour)
2. Identify gaps (2 hours)
3. Add unit tests (10-15 hours)
4. Add E2E tests (7-10 hours)

**Grade Impact**: +1 point

---

## ✅ QUICK WINS (2-4 hours)

### Fix Interactive Test Guards
```rust
#[cfg_attr(not(feature = "interactive"), ignore)]
#[test]
fn test_entropy_collection_workflow() { ... }
```

### Clean Up Documentation
- Consolidate audit documents
- Update CURRENT_STATUS.md
- Create final roadmap

---

**Next Session Goal**: A (93/100) → A (95/100)  
**Timeline**: 4-6 hours  
**Focus**: Hardcoding elimination

🐻 Ready for Next Session! 🐕
