# 🌱 Entropy Hierarchy Guide

**Version**: 1.0.0  
**Status**: IMPLEMENTED ✅  
**Module**: `beardog-genetics::entropy_hierarchy`

---

## Overview

The Entropy Hierarchy system implements **human-centric entropy management** with sophisticated quality assessment and lifecycle management.

## Entropy Classification

### Hierarchy (Highest to Lowest Quality)

1. **Human Lived Experience** (Tier 3 - Highest)
   - Source: Multi-modal human input
   - Quality: 0.9 base score
   - Features: Biometric verification, ownership proof

2. **Human Supervised Machine** (Tier 2 - Medium)  
   - Source: Machine generation with human oversight
   - Quality: 0.7 base score
   - Features: Human validator, validation timestamp

3. **Store Bought Machine** (Tier 1 - Lowest)
   - Source: Pure machine generation
   - Quality: 0.4 base score
   - Features: Reproducibility index, generation metadata

## Usage Examples

```rust
use beardog_genetics::entropy_hierarchy::{EntropyHierarchyManager, EntropyClass};

// Initialize entropy manager
let mut entropy_manager = EntropyHierarchyManager::default();

// Create human entropy seed (highest quality)
let human_identity = HumanIdentity {
    identity_id: "user123".to_string(),
    verification_level: VerificationLevel::Biometric,
    biometric_hash: Some(biometric_data),
};

let seed_id = entropy_manager.create_human_seed(
    EntropyClass::HumanLivedExperience { /* ... */ },
    SeedLifetimePolicy::Persistent,
    human_identity,
    entropy_bytes,
).await?;

// Assess entropy quality
let assessment = entropy_manager.assess_seed_quality(&seed);
println!("Quality: {} (tier {})", assessment.quality_score, assessment.entropy_tier);
```

## Quality Assessment

### Factors Considered
- **Base Quality**: Based on entropy class (Human > Supervised > Machine)
- **Age Factor**: Fresher entropy scores higher
- **Usage Factor**: Less-used entropy scores higher
- **Weighted Score**: Final score with entropy class weighting

### Quality Tiers
- **Tier 3**: Quality > 0.8 (Excellent)
- **Tier 2**: Quality > 0.6 (Good)  
- **Tier 1**: Quality ≤ 0.6 (Basic)

## Lifecycle Management

### Seed States
- **Active**: Available for use
- **Expired**: Past lifetime limits
- **Exhausted**: Usage limits reached
- **Compromised**: Security concerns

### Cleanup Policies
- **Automatic**: Expired seeds cleaned automatically
- **Manual**: On-demand cleanup available
- **Secure**: Cryptographic zeroization

---

**Status**: ✅ **OPERATIONAL - HUMAN-CENTRIC DESIGN** 