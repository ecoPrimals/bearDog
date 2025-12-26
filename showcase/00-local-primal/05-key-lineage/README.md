# 🧬 Key Lineage - Track Key Ancestry

**Level**: 0 (Local Primal)  
**Category**: Key Management  
**Time**: 10 minutes  
**Dependencies**: None

---

## 🎯 What This Demo Shows

Track the **complete ancestry** of keys:
- ✅ Parent-child relationships
- ✅ Inheritance of constraints
- ✅ Evolution tracking
- ✅ Lineage visualization
- ✅ Audit trail

BearDog keys have **genetic lineage** - you can trace any key back to its origins.

---

## 🚀 Running the Demo

```bash
./run.sh
```

---

## 📊 Expected Output

```
🧬 BearDog - Key Lineage Demo
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Creating a key family tree...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Generation 1: Genesis Key
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
ID: genesis_key_abc123
Type: Ed25519
Parent: None (root key)
Constraints:
  - Expires: 2026-12-31
  - Max Uses: 10000
Created: 2025-12-24T20:00:00Z

✓ Genesis key created

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Generation 2: Derived Child
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
ID: child_key_def456
Type: Ed25519
Parent: genesis_key_abc123
Inherited Constraints:
  - Expires: 2026-12-31 (from parent)
  - Max Uses: 10000 (from parent)
New Constraints:
  + Usage: Sign only
  + Witness: alice@example.com
Created: 2025-12-24T20:01:00Z

✓ Child key derived from parent

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Generation 3: Grandchild
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
ID: grandchild_key_ghi789
Type: Ed25519
Parent: child_key_def456
Grandparent: genesis_key_abc123
Inherited Constraints:
  - All parent constraints
  - All grandparent constraints
New Constraints:
  + Max Uses: 100 (stricter than parent)
Created: 2025-12-24T20:02:00Z

✓ Grandchild key created

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Lineage Visualization
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

genesis_key_abc123 (Gen 1)
  ├─ Expires: 2026-12-31
  └─ Max Uses: 10000
     │
     └──> child_key_def456 (Gen 2)
           ├─ Inherits: parent constraints
           ├─ + Sign only
           └─ + Witness required
              │
              └──> grandchild_key_ghi789 (Gen 3)
                    ├─ Inherits: all ancestor constraints
                    └─ + Max Uses: 100 (stricter)

Total Generations: 3
Total Keys in Family: 3
Constraint Evolution: Progressive restriction

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Lineage Queries
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Query: Who is the parent of grandchild_key?
Answer: child_key_def456 ✓

Query: List all ancestors of grandchild_key
Answer:
  - child_key_def456 (parent)
  - genesis_key_abc123 (grandparent)
✓

Query: List all descendants of genesis_key
Answer:
  - child_key_def456 (child)
  - grandchild_key_ghi789 (grandchild)
✓

Query: What constraints did grandchild inherit?
Answer:
  - Expires: 2026-12-31 (from grandparent)
  - Max Uses: 100 (own, stricter than ancestors)
  - Sign only (from parent)
  - Witness required (from parent)
✓

🧬 Complete lineage tracking working!
```

---

## 🧠 Understanding Key Lineage

### Why Track Lineage?

**Audit Trail**:
- Know where every key came from
- Trace security incidents
- Compliance reporting

**Constraint Evolution**:
- See how constraints changed over generations
- Understand permission inheritance
- Debug access issues

**Key Management**:
- Revoke entire families
- Update related keys together
- Organize by lineage

### Lineage Rules

**Inheritance**:
- Children inherit ALL parent constraints
- Children can ADD new constraints
- Children can make constraints STRICTER
- Children CANNOT make constraints LOOSER

**Example**:
```
Parent: Max Uses = 1000
Child: Max Uses = 100 ✓ (stricter, allowed)
Child: Max Uses = 2000 ✗ (looser, forbidden)
```

---

## 🎓 Key Concepts

### 1. Genesis Keys
- No parent (root of family)
- Start of lineage
- Can have multiple children

### 2. Derived Keys
- Have a parent
- Inherit parent constraints
- Can add new constraints
- Cannot violate parent rules

### 3. Constraint Evolution
```
Gen 1: Expires 2026
Gen 2: Expires 2026 + Sign only
Gen 3: Expires 2026 + Sign only + Max 100 uses
```

Each generation adds restrictions!

### 4. Lineage Graph
```
         Genesis
        /    |    \
    Child1 Child2 Child3
      /              \
  GrandChild1    GrandChild2
```

Forms a tree structure.

---

## 🚀 Next Steps

### Experiment
```rust
// Create different family structures
// Try cyclic inheritance (should fail!)
// Test constraint violations
// Query complex lineages
```

### Next Demos
1. **06-btsp-tunnel** - Secure encrypted tunnels
2. **01-hardware-integration/** - Real HSM usage
3. **02-ecosystem-integration/** - Cross-primal lineage

---

## 📚 Related Documentation

- **03-key-constraints** - Understand constraints first
- **specs/current/architecture/PRIMAL_SOVEREIGNTY_ARCHITECTURE.md**
- **CAPABILITY_ARCHITECTURE_EVOLUTION_PLAN.md**

---

**Demo Status**: ✅ Complete  
**Difficulty**: 🟡 Intermediate  
**Time Required**: 10 minutes

🧬 **BearDog: Every Key Has a Story!** 🔐

