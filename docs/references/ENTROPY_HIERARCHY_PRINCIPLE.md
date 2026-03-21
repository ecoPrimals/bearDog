# Entropy Hierarchy Principle

**Date**: December 19, 2025  
**Status**: **CORE DESIGN PRINCIPLE**  
**Severity**: **CRITICAL - SECURITY & TRUST MODEL**

---

## 🎯 The Principle

> **"Never simulate human entropy - it violates the trust model."**

This is a **fundamental design constraint** for BearDog, not a feature request.

---

## 📊 Entropy Hierarchy

### Valid Entropy Sources (In Order of Trust)

```
┌─────────────────────────────────────────────────────────┐
│  TIER 1: Hardware HSMs (Highest Quality)               │
│  ────────────────────────────────────────────────────   │
│  • YubiKey, SoloKeys, StrongBox                         │
│  • Cryptographic quality: ★★★★★                         │
│  • Uniqueness: ★★★★☆                                    │
│  • Sovereignty: ★★★☆☆                                   │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│  TIER 2: System Entropy (High Quality)                 │
│  ────────────────────────────────────────────────────   │
│  • /dev/urandom, kernel entropy pool                    │
│  • Cryptographic quality: ★★★★★                         │
│  • Uniqueness: ★★★★☆                                    │
│  • Sovereignty: ★★☆☆☆                                   │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│  TIER 3: Real Human Input (Non-Fungible)               │
│  ────────────────────────────────────────────────────   │
│  • Keyboard timing, mouse jitter, YOUR behavior         │
│  • Cryptographic quality: ★★★☆☆                         │
│  • Uniqueness: ★★★★★  (NON-FUNGIBLE)                   │
│  • Sovereignty: ★★★★★  (YOU control it)                 │
└─────────────────────────────────────────────────────────┘

╔═════════════════════════════════════════════════════════╗
║  TIER VIOLATION: Simulated Human Entropy               ║
║  ═════════════════════════════════════════════════════  ║
║  • Pseudo-random "human-like" data                      ║
║  • Cryptographic quality: ★★☆☆☆  (Weak)                ║
║  • Uniqueness: ★☆☆☆☆  (Fungible, reproducible)         ║
║  • Sovereignty: ☆☆☆☆☆  (Fake)                           ║
║  • Trust: ☆☆☆☆☆  (VIOLATED)                             ║
║                                                         ║
║  ❌ MUST BE REJECTED BY BEARDOG                         ║
╚═════════════════════════════════════════════════════════╝
```

---

## 🚨 Why Simulation is a Violation

### 1. **Trust Model Broken**
```
User believes: "My key is unique to ME"
Reality with simulation: "Key is reproducible by anyone"
→ TRUST DESTROYED
```

### 2. **Non-Fungibility Lost**
```
Goal: Non-fungible human entropy (one-of-a-kind)
Simulation: Fungible, reproducible, copyable
→ PURPOSE DEFEATED
```

### 3. **Sovereignty Compromised**
```
Promise: "You contribute to your key"
Simulation: "BearDog generates everything"
→ SOVEREIGNTY ILLUSION
```

### 4. **Security Theater**
```
Appearance: "Human-enhanced security"
Reality: "Standard pseudo-random"
→ FALSE SENSE OF SECURITY
```

---

## ✅ Correct Behaviors

### Device Entropy
```rust
// ✅ VALID: Real device entropy
let device_entropy = hsm.collect_entropy()?;
// Quality: HIGH ✓
// Source: Hardware ✓
// Trust: Valid ✓
```

### Real Human Entropy
```rust
// ✅ VALID: Real human keyboard/mouse timing
let human_entropy = collect_real_human_input()?;
// Quality: MEDIUM (but real) ✓
// Uniqueness: HIGH (non-fungible) ✓
// Trust: Valid ✓
```

### Mixed Entropy (Goal)
```rust
// ✅ VALID: Mix real device + real human
let mixed = mix_entropy(
    device_entropy,  // 60% - quality
    human_entropy,   // 40% - uniqueness
)?;
// Quality: HIGH (from device) ✓
// Uniqueness: HIGH (from human) ✓
// Trust: Valid ✓
```

---

## ❌ Violations to Reject

### Simulated Human Entropy
```rust
// ❌ VIOLATION: Faking human input
let fake_human = simulate_keyboard_timing();
// Quality: LOW ✗
// Uniqueness: NONE (fungible) ✗
// Trust: VIOLATED ✗

// BearDog MUST reject this!
```

### Disguised Device Entropy
```rust
// ❌ VIOLATION: Pretending device entropy is human
let device_entropy = hsm.collect_entropy()?;
let fake_human = device_entropy.clone();  // Lie!
// Trust: VIOLATED ✗

// BearDog MUST detect and reject this!
```

### Low-Quality Acceptance
```rust
// ❌ VIOLATION: Accepting low-quality as "human"
let poor_entropy = collect_weak_input()?;
// Quality: 10% (below threshold)
// BearDog accepted it anyway ✗

// BearDog MUST enforce quality thresholds!
```

---

## 🔒 Enforcement Mechanisms

### 1. Source Provenance Tracking
```rust
pub enum EntropySource {
    Hardware {
        device_id: String,
        vendor: String,
        verified: bool,
    },
    System {
        source: String,  // "/dev/urandom", etc.
        verified: bool,
    },
    Human {
        collection_method: HumanCollectionMethod,
        quality_score: f64,
        timing_entropy: f64,
        verified: bool,
        // CRITICAL: Cannot be created without real collection
    },
}

// ❌ No public constructor for Human variant with fake data
// ✅ Only internal collector can create Human variant
```

### 2. Quality Validation
```rust
impl EntropySource {
    pub fn validate(&self) -> Result<(), EntropyViolation> {
        match self {
            Self::Human { quality_score, timing_entropy, .. } => {
                // Enforce minimum quality
                if *quality_score < 0.3 {
                    return Err(EntropyViolation::QualityTooLow {
                        actual: *quality_score,
                        required: 0.3,
                    });
                }
                
                // Verify timing entropy exists
                if *timing_entropy < 0.5 {
                    return Err(EntropyViolation::InsufficientTimingEntropy {
                        actual: *timing_entropy,
                        required: 0.5,
                    });
                }
                
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
```

### 3. Simulation Detection
```rust
pub fn detect_simulation(entropy: &[u8]) -> Result<(), EntropyViolation> {
    // Check for patterns indicating simulation
    
    // 1. Too uniform (not human-like)
    if entropy_uniformity(entropy) > 0.95 {
        return Err(EntropyViolation::SuspectedSimulation {
            reason: "Entropy too uniform for human input",
        });
    }
    
    // 2. No timing variations
    if timing_variance(entropy) < 0.1 {
        return Err(EntropyViolation::SuspectedSimulation {
            reason: "No natural timing variations detected",
        });
    }
    
    // 3. Matches known PRNG patterns
    if matches_prng_pattern(entropy) {
        return Err(EntropyViolation::SuspectedSimulation {
            reason: "Matches pseudo-random number generator pattern",
        });
    }
    
    Ok(())
}
```

### 4. Audit Logging
```rust
pub struct EntropyCollectionAudit {
    pub timestamp: DateTime<Utc>,
    pub source: EntropySource,
    pub quality_score: f64,
    pub verified: bool,
    pub violations: Vec<EntropyViolation>,
    pub accepted: bool,
    
    // CRITICAL: Full provenance
    pub collection_method: String,
    pub user_interaction: bool,
    pub simulation_detected: bool,
}

// Every entropy collection is audited
// Violations are logged but may not block (with warnings)
// Simulations are ALWAYS logged as violations
```

### 5. User Warnings
```rust
pub fn collect_with_warnings(source: EntropySource) -> Result<Entropy, EntropyError> {
    match source.validate() {
        Ok(()) => {
            // Valid entropy
            Ok(collect_internal(source)?)
        }
        Err(violation) => {
            match violation {
                EntropyViolation::SuspectedSimulation { .. } => {
                    // HARD REJECT
                    eprintln!("❌ REJECTED: Simulated entropy detected");
                    eprintln!("   {}", violation);
                    eprintln!("   BearDog refuses to use simulated human entropy");
                    Err(EntropyError::SimulationViolation(violation))
                }
                EntropyViolation::QualityTooLow { .. } => {
                    // WARNING + Option to continue
                    eprintln!("⚠️  WARNING: Low quality entropy");
                    eprintln!("   {}", violation);
                    eprintln!("   Consider using higher quality source");
                    
                    // Allow but log
                    log_violation(violation);
                    Ok(collect_internal(source)?)
                }
                _ => Err(EntropyError::Violation(violation)),
            }
        }
    }
}
```

---

## 📋 Implementation Checklist

### Phase 1: Detection & Warnings (Current)
- [x] `--human-input` flag exists
- [x] Quality scoring implemented
- [x] Demo refuses to simulate (showcase integrity)
- [ ] **Internal simulation detection**
- [ ] **Provenance tracking in entropy struct**
- [ ] **User warnings on violations**

### Phase 2: Interactive Collection (Next)
- [ ] Terminal UI for typing/mouse input
- [ ] Keystroke dynamics capture
- [ ] Timing entropy measurement
- [ ] Real-time quality feedback
- [ ] **Simulation rejection built-in**

### Phase 3: Cryptographic Mixing (Next)
- [ ] SHA3-512 mixing function
- [ ] Configurable ratios (60/40 default)
- [ ] Quality preservation guarantees
- [ ] **Provenance in mixed entropy**

### Phase 4: Enforcement (Critical)
- [ ] **Hard reject simulated human entropy**
- [ ] **Audit logging of all entropy sources**
- [ ] **Receipt generation with full provenance**
- [ ] **CLI warnings/errors on violations**

---

## 🎓 Design Rationale

### Why 60% Device / 40% Human?

**60% Device**: Ensures cryptographic quality
- Protects against weak human input
- Meets security thresholds
- Provides cryptographic strength baseline

**40% Human**: Adds uniqueness without compromising quality
- Enough to make keys non-fungible
- Your contribution is meaningful
- Not enough to weaken if human input is poor

**SHA3-512 Mixing**: Cryptographically combines sources
- No information leakage
- Quality preservation
- Collision resistance

### Why Reject Simulation?

**Trust**: Users trust BearDog to be honest
- Simulation is dishonest
- Breaks the social contract
- Destroys sovereignty promise

**Non-Fungibility**: The entire point
- Simulation makes keys fungible
- Defeats the purpose of human entropy
- No uniqueness without real input

**Security**: Simulation is weak
- Pseudo-random is predictable
- Real entropy is unpredictable
- Security through real randomness

---

## 🚀 Recommended Next Steps

### 1. Add Internal Checks
```rust
// In entropy collection code
if source.is_human() && detect_simulation(&data).is_err() {
    return Err(EntropyError::SimulationViolation);
}
```

### 2. Add Provenance to Receipts
```json
{
  "receipt_id": "...",
  "operation": "entropy_collection",
  "entropy_source": {
    "type": "human",
    "verified": true,
    "quality_score": 0.75,
    "simulation_detected": false,
    "collection_method": "keyboard_timing"
  }
}
```

### 3. Add CLI Warnings
```bash
$ beardog entropy collect --human-input
⚠️  Warning: Human input quality low (32%)
⚠️  Consider re-collecting with more varied input
✓  Collected, but quality is below optimal threshold
```

### 4. Document Principle
- Add to README.md
- Add to security docs
- Add to API docs
- Make it a core value

---

## ✅ Current Status (Dec 19, 2025)

**What We Did Right**:
- ✅ Showcase demo **refused to simulate**
- ✅ Maintained entropy hierarchy integrity
- ✅ Documented the principle
- ✅ Showed architecture for future

**What's Next**:
- [ ] Implement detection internally
- [ ] Add provenance tracking
- [ ] Enforce with hard rejects
- [ ] Complete interactive collection

---

## 🏆 Conclusion

**Entropy hierarchy is not a feature - it's a principle.**

BearDog must:
1. **NEVER simulate human entropy**
2. **ALWAYS track provenance**
3. **REJECT violations with clear errors**
4. **DOCUMENT everything in receipts**

This is what makes BearDog **sovereign** and **trustworthy**.

**Status**: ✅ Principle established, enforcement in progress

---

🐻🐕 **BearDog: Integrity Over Features. Always.**

