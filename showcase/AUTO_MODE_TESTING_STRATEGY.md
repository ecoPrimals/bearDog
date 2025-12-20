# 🚦 Auto-Mode Demo Testing Strategy

**Date:** December 20, 2025  
**Purpose:** Define expected behavior for all demos in `--auto` mode

---

## ✅ **Demos That Should PASS in Auto Mode**

### **1. Crypto Verification Demos**
- ✅ `03-songbird-integration/demos/02-live-crypto-proof.sh`
  - Genetic mixing
  - Wrong key rejection
  - Data integrity proof
  - **Expected:** PASS

- ✅ `04-hsm-vendor-agnostic/demos/01-discover-all-hsms.sh`
  - HSM discovery
  - Zero configuration
  - **Expected:** PASS

- ✅ `04-hsm-vendor-agnostic/demos/02-runtime-hsm-switch.sh`
  - Runtime HSM switching
  - Configuration changes
  - **Expected:** PASS

### **2. Service Integration Demos**
- ✅ `03-songbird-integration/demos/01-service-registration.sh`
  - Service registration
  - Capability discovery
  - **Expected:** PASS (simulated)

### **3. Genetic Demos (Non-Interactive)**
- ✅ `02-hardware-integration/demo-genetic-realistic.sh`
  - Hierarchical keys
  - Key mixing
  - **Expected:** PASS

- ✅ `03-constraint-demos/demo-constraints.sh`
  - Time constraints
  - Resource constraints
  - **Expected:** PASS

---

## ⚠️ **Demos That Should FAIL/SKIP in Auto Mode (Expected!)**

### **Human Entropy Demos**

These demos **MUST** fail or skip in auto mode because:
1. ✅ **Entropy Hierarchy Enforcement** - No simulated human entropy
2. ✅ **LiveFeedValidator** - Detects non-live input
3. ✅ **Security Principle** - "Integrity Over Features"

**List:**
- ⚠️ `02-hardware-integration/demo-human-entropy-interactive.sh`
  - **Expected:** SKIP (requires real human input)
  - **Reason:** Cannot simulate keyboard/mouse dynamics
  - **Auto Behavior:** Should detect auto mode and skip gracefully

- ⚠️ `showcase/run-entropy-interactive.sh`
  - **Expected:** SKIP (requires real human input)
  - **Reason:** Interactive entropy collection
  - **Auto Behavior:** Should detect auto mode and skip gracefully

- ⚠️ `showcase/entropy-mixing-real-human.sh`
  - **Expected:** SKIP or use device-only entropy
  - **Reason:** Human entropy component cannot be simulated
  - **Auto Behavior:** Should skip human portion, use device entropy only

---

## 🔄 **Demos That Need Auto-Mode Updates**

### **Need --auto Support:**
1. 📋 `01-local-basics/demo.sh`
2. 📋 `02-hardware-integration/demo-real-crypto.sh`
3. 📋 `02-hardware-integration/demo-hybrid.sh`
4. 📋 `03-constraint-demos/demo-mobile-context.sh`
5. 📋 `03-constraint-demos/demo-secure-lab.sh`
6. 📋 `03-constraint-demos/demo-tower-sharing.sh`

---

## 📊 **Auto-Mode Test Matrix**

| Demo | Auto Support | Expected Result | Reason |
|------|--------------|-----------------|--------|
| **HSM Discovery** | ✅ Yes | ✅ PASS | No user input needed |
| **HSM Switching** | ✅ Yes | ✅ PASS | Automated operation |
| **Live Crypto Proof** | ✅ Yes | ✅ PASS | Automated crypto ops |
| **Service Registration** | ✅ Yes | ✅ PASS | Simulated services |
| **Genetic Realistic** | 🔄 Partial | ✅ PASS | Uses device entropy |
| **Constraints** | 🔄 Partial | ✅ PASS | Automated constraints |
| **Human Entropy Interactive** | ⚠️ No | ⚠️ SKIP | Requires real human |
| **Entropy Mixing (Human)** | ⚠️ No | ⚠️ SKIP | Requires real human |

---

## 🎯 **Correct Behavior for Human Entropy in Auto Mode**

### **Option 1: Graceful Skip (Recommended)**
```bash
if [ "$AUTO_MODE" = true ]; then
    log_info "Auto-mode detected: Skipping human entropy collection"
    log_info "Reason: Human entropy cannot be simulated (entropy hierarchy)"
    log_info "Run in interactive mode for real human entropy: ./demo.sh"
    return 0  # Skip, not fail
fi
```

### **Option 2: Device-Only Fallback**
```bash
if [ "$AUTO_MODE" = true ]; then
    log_warning "Auto-mode: Using device entropy only"
    log_warning "Human entropy component skipped (cannot simulate)"
    # Proceed with device entropy only
fi
```

### **Option 3: Fail with Clear Message**
```bash
if [ "$AUTO_MODE" = true ] && [ "$REQUIRES_HUMAN_ENTROPY" = true ]; then
    log_error "This demo requires real human entropy"
    log_error "Cannot run in auto mode (entropy hierarchy violation)"
    log_info "Run interactively: ./demo.sh"
    exit 1  # Clear failure, not a bug
fi
```

---

## ✅ **Test Plan**

### **Phase 1: Update All Demos**
```bash
# Add --auto support to each demo
for demo in showcase/**/*.sh; do
    # Add library sourcing
    # Add auto-mode flag parsing
    # Update wait_for_user calls
done
```

### **Phase 2: Test Each Demo**
```bash
# Test in auto mode
./demo.sh --auto

# Expected results:
#   - Crypto demos: PASS
#   - Service demos: PASS
#   - Human entropy: SKIP (correct!)
```

### **Phase 3: Comprehensive Test**
```bash
# Run all demos
./RUN_ALL_SHOWCASES.sh --auto

# Generate report:
#   - X demos passed
#   - Y demos skipped (expected)
#   - Z demos failed (investigate)
```

---

## 🏆 **Success Criteria**

**A demo is "auto-mode ready" when:**
1. ✅ Supports `--auto` flag
2. ✅ Sources `robust_demo_functions.sh`
3. ✅ Uses `wait_for_user()` for pauses
4. ✅ Has clear behavior in auto mode:
   - PASS (for automated ops)
   - SKIP (for human-required ops)
   - FAIL (only for real errors)

**A human entropy demo is "correct" when:**
1. ✅ Detects auto mode
2. ✅ Skips gracefully OR fails with clear message
3. ✅ Does NOT simulate human entropy
4. ✅ Maintains entropy hierarchy integrity
5. ✅ Provides helpful message for interactive mode

---

## 📝 **Documentation for Users**

### **README Addition:**
```markdown
## Running Demos

### Interactive Mode (Default)
$ ./demo.sh

### Automatic Mode (AI/CI)
$ ./demo.sh --auto

Note: Human entropy demos require interactive mode.
They will skip in auto mode to maintain entropy hierarchy integrity.
```

---

## 🎯 **Expected Test Results**

### **When Running `RUN_ALL_SHOWCASES.sh --auto`:**

```
╔══════════════════════════════════════════════════════════════╗
║           Showcase Test Results                              ║
╚══════════════════════════════════════════════════════════════╝

✅ PASSED: 15 demos
   • All crypto operations
   • All service integrations
   • All genetic operations (device entropy)

⏭️  SKIPPED: 3 demos (Expected)
   • Human entropy interactive
   • Human entropy mixing
   • Multi-modal entropy collection
   Reason: Requires real human input (cannot simulate)

❌ FAILED: 0 demos

Overall: ✅ ALL TESTS BEHAVED CORRECTLY
```

---

## 🔐 **Entropy Hierarchy Compliance**

**Critical Principle:**
> "Any attempt to simulate human entropy is an entropy hierarchy violation"

**Auto-Mode Behavior:**
- ✅ **CORRECT:** Skip human entropy demos in auto mode
- ✅ **CORRECT:** Use device-only entropy when appropriate
- ✅ **CORRECT:** Fail with clear message about human requirement
- ❌ **WRONG:** Simulate human entropy to "pass" tests
- ❌ **WRONG:** Fake keyboard/mouse timing
- ❌ **WRONG:** Use random data as "human" entropy

**The fact that human entropy demos skip/fail in auto mode is PROOF that entropy hierarchy is being enforced correctly!**

---

**🐻 BearDog: Integrity Over Convenience**

*If a demo requires human entropy, it should NOT work in auto mode. That's a feature, not a bug.*

