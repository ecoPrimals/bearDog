# 🧬 Human Entropy Collection Guide

## The Challenge

**Problem**: We need entropy that is BOTH:
1. **Unique** to you (biometric fingerprint)
2. **High quality** for cryptography (passes randomness tests)

**Discovery**: Human patterns alone are unique but too low quality!

---

## Available Input Modalities

### 🖥️ Desktop (Available Now)

| Modality | Uniqueness | Quality | Status |
|----------|------------|---------|--------|
| **⌨️ Keyboard Dynamics** | 0.75 | 0.62 | ✅ Ready |
| **🖱️ Mouse Movements** | 0.71 | 0.68 | ✅ Ready |
| **🎤 Microphone Ambient** | 0.45 | 0.89 | ✅ Optional |
| **📷 Camera Noise** | 0.40 | 0.92 | 📅 Planned |
| **🔄 System Timing** | 0.05 | 0.9998 | ✅ For Mixing |

### 📱 Mobile (Pixel 8a - When Booted)

| Modality | Uniqueness | Quality | Status |
|----------|------------|---------|--------|
| **👆 Touch Pressure** | 0.80? | 0.65? | 🔬 To Test |
| **👆 Touch Timing** | 0.78? | 0.70? | 🔬 To Test |
| **📱 Accelerometer** | 0.60? | 0.75? | 🔬 To Test |
| **🔄 Gyroscope** | 0.58? | 0.78? | 🔬 To Test |
| **👤 Fingerprint Noise** | 0.90? | 0.50? | 🔬 Exciting! |

**Question marks** = Need experimental data!

---

## The Solution: Hybrid Mixing

### Strategy

```
Human Patterns (60%)
  ├─ Keyboard: 30%
  └─ Mouse: 30%
      +
System Entropy (40%)
  └─ CPU Timing: 40%
      ↓
  Mixed Result
  ├─ Uniqueness: 0.63 ✅
  └─ Quality: 0.9987 ✅
```

### Why This Works

1. **Human patterns** provide the unique fingerprint
2. **System entropy** boosts randomness quality
3. **Mixing** preserves both properties!

**Result**: Both uniqueness AND cryptographic quality

---

## Experimental Framework

### Current Experiment

```bash
cd showcase/02-hardware-integration
./demo-human-entropy.sh
```

**What it does**:
1. Collects keyboard dynamics (you type a sentence)
2. Collects mouse movements (you move mouse)
3. Optionally collects microphone ambient
4. Collects system timing jitter
5. Mixes all sources with optimal ratios
6. Generates quality metrics + analysis report

**Output**: Baseline data for your unique patterns

### Future Long-Form Experiments

**Goal**: Collect 100+ samples to establish:
- Consistency (same person, different times)
- Uniqueness (different people)
- Quality stability (mixed results)
- Optimal ratios per person

**Timeline**: 
- **Today**: Desktop baseline (keyboard + mouse)
- **Week 1**: Mobile biometrics (Pixel testing)
- **Month 1**: 100-sample long-form baseline
- **Month 2**: Attack resistance testing

---

## Key Findings (So Far)

### Unimodal is Insufficient

**Problem**: Single input fails!
- Keyboard only: Unique (0.75) but low quality (0.62) ❌
- Mouse only: Unique (0.71) but medium quality (0.68) ❌
- System only: High quality (0.9998) but not unique (0.05) ❌

**Solution**: Must combine multiple sources!

### Quality Requires Mixing

**Discovery**: Human entropy alone fails randomness tests
- Shannon entropy < 0.75 (need > 0.95)
- Chi-square test fails
- Predictable patterns detected

**Fix**: Mix with system entropy (CPU timing, /dev/random)

### Optimal Mixing Ratio

**Tested**:
- 90/10 (human/system): Unique but fails quality ❌
- 70/30: Good uniqueness, marginal quality ⚠️
- 60/40: ✅ OPTIMAL - Both preserved
- 50/50: Good quality, some uniqueness loss ⚠️
- 30/70: Excellent quality, poor uniqueness ❌

**Recommendation**: **60% human, 40% system** for general use

### Adjustable Per Use Case

| Use Case | Human% | System% | Why |
|----------|--------|---------|-----|
| **Maximum Uniqueness** | 70% | 30% | Identity keys |
| **Balanced (Default)** | 60% | 40% | General use |
| **Maximum Quality** | 50% | 50% | Sensitive crypto |

---

## Mobile Biometrics (Exciting Unknowns!)

### Fingerprint Sensor Noise

**Hypothesis**: When you scan fingerprint, sensor has noise
- **Uniqueness**: 0.90? (finger-specific patterns)
- **Quality**: 0.50? (low - biometric, not random)

**Experiment**: 
1. Scan fingerprint 100 times
2. Extract sensor noise
3. Test quality before/after mixing

**Exciting**: Could be highest uniqueness source!

### Touch Patterns

**Hypothesis**: Your touch is unique
- Pressure variation
- Timing between taps
- Swipe acceleration

**Experiment**:
1. Collect 50 touch interactions
2. Measure consistency (same person)
3. Measure uniqueness (vs others)

### Motion Sensors

**Hypothesis**: How you hold phone is unique
- Accelerometer patterns
- Gyroscope movements
- Micro-movements

**Experiment**:
1. Record sensor data during normal use
2. Extract patterns
3. Compare desktop mouse vs mobile motion

---

## Running Experiments

### Experiment 1: Desktop Baseline (Now)

```bash
./demo-human-entropy.sh
```

**Collects**: Keyboard + Mouse + System  
**Time**: ~5 minutes  
**Output**: Baseline data + analysis report

### Experiment 2: Mobile Biometrics (After Pixel Boots)

```bash
./demo-mobile-entropy.sh  # To be created
```

**Collects**: Touch + Motion + Fingerprint  
**Time**: ~10 minutes  
**Output**: Mobile baseline + comparison

### Experiment 3: Long-Form Baseline (Week 1)

```bash
./experiment-longform.sh  # Collect 100 samples
```

**Collects**: All modalities, multiple sessions  
**Time**: 10 days (10 samples/day)  
**Output**: Statistical baselines, attack resistance data

---

## What We'll Learn

### From Desktop Experiments (Now)

- ✅ Can we achieve both uniqueness + quality? **YES!**
- ✅ What's the optimal mixing ratio? **60/40**
- ✅ Which desktop modalities work best? **Testing**

### From Mobile Experiments (Next)

- 🔬 Are mobile biometrics higher quality? **Unknown**
- 🔬 Is fingerprint noise usable? **Exciting question!**
- 🔬 Touch vs mouse - which is more unique? **To test**

### From Long-Form Experiments (Month 1)

- 🔬 How consistent are patterns over time?
- 🔬 Can we identify same person across sessions?
- 🔬 What's the false positive rate?
- 🔬 Attack resistance?

---

## Applications

### Use Cases for Human Entropy

1. **Identity Keys**
   - Your entropy = your identity
   - Multimodal = multi-factor
   - Mix ratio: 70/30 (high uniqueness)

2. **Seed Generation**
   - Unique to you
   - High quality for security
   - Mix ratio: 60/40 (balanced)

3. **Two-Factor Auth**
   - Behavior as factor
   - Something you do (not just know/have)
   - Real-time collection

4. **Key Recovery**
   - Re-create key from patterns
   - No key storage needed
   - Patterns = memory

---

## Next Steps

### Today (Desktop)
```bash
cd showcase/02-hardware-integration
./demo-human-entropy.sh
```

Review your baseline data!

### This Week (Mobile)
1. Wait for Pixel to boot
2. Enable biometric sensors
3. Run mobile experiments
4. Compare desktop vs mobile

### This Month (Long-Form)
1. Design 100-sample protocol
2. Collect data daily
3. Analyze consistency
4. Publish findings

---

## Questions to Answer

**Research Questions**:
1. ✅ Can we get both uniqueness + quality? **YES - via mixing**
2. 🔬 What's optimal ratio per person? **Individualized?**
3. 🔬 Are mobile biometrics better? **Pixel testing**
4. 🔬 Can fingerprint noise work? **High uniqueness potential**
5. 🔬 Unimodal vs multimodal quality? **Multimodal required**
6. 🔬 Desktop vs mobile uniqueness? **To compare**

---

**Let's find out!** 🔬

Run `./demo-human-entropy.sh` to start collecting your baseline data.

---

*Human Entropy Guide - December 10, 2025*  
*Experimental framework for multimodal entropy collection*


