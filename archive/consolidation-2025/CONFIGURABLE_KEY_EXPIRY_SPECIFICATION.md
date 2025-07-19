# Configurable Key Expiry System - BearDog New Age Crypto

## Executive Summary

**FIXED**: Hard-coded 24-hour key expiry replaced with **configurable, entropy-aware, genetically renewable key lifecycle management**. Keys can now be permanent (when authorized), adaptive based on entropy quality, and leverage genetic spawning for autonomous renewal.

## Key Features

### ✅ **Configurable Expiry Policies**
- **Fixed Duration**: Traditional time-based expiry
- **Entropy-Adaptive**: Duration adjusts based on entropy quality
- **Usage-Based**: Expires after N operations
- **Activity-Based**: Extends on use, expires on inactivity  
- **Genetic Renewal**: Keys spawn new generations
- **Permanent**: Never expires (requires explicit authorization)

### ✅ **Entropy Hierarchy Integration**
- **Human Entropy (Tier 3)**: Gets 2x longer expiry duration
- **Supervised Entropy (Tier 2)**: Gets 1.5x longer expiry duration  
- **Machine Entropy (Tier 1)**: Gets base expiry duration
- **Quality-Based Adjustment**: Higher quality = longer expiry
- **Self-Sovereign Benefits**: Enhanced genetic renewal capabilities

### ✅ **Genetic Spawning Integration**
- **Evolutionary Renewal**: Keys spawn evolved versions of themselves
- **Inheritance Rules**: Configure what properties are passed down
- **Mutation Parameters**: Control how keys evolve over generations
- **Cross-Context Breeding**: Keys can inherit from multiple contexts

## Configuration Examples

### 1. **Default Configuration**
```toml
[key_expiry]
# Security-first: no permanent keys by default
allow_permanent = false
max_expiry_duration = "365d"  # 1 year maximum
min_expiry_duration = "5m"    # 5 minute minimum
entropy_based_adjustment = true
enable_genetic_renewal = true

[key_expiry.default_policy]
policy_type = "fixed"
duration = "24h"
```

### 2. **Entropy-Adaptive Configuration**
```toml
[key_expiry.context_policies.user_auth]
policy_type = "entropy_adaptive"
base_duration = "24h"         # Base for lowest entropy
max_duration = "168h"         # 7 days for highest entropy  
entropy_weight = 1.5          # Entropy influence factor
```

### 3. **Genetic Renewal Configuration**
```toml
[key_expiry.context_policies.identity]
policy_type = "genetic_renewal"
generation_duration = "90d"   # 3 months per generation
max_generations = 10          # Up to 10 generations
generation_decay = 0.9        # Each generation 90% of previous

[genetic_renewal]
enabled = true
min_entropy_quality = 0.7     # Require high entropy for renewal
spawning_workflow = "hybrid"  # Auto with human oversight

[genetic_renewal.inheritance_rules]
inheritable_contexts = ["user_auth", "data_encryption"]

[genetic_renewal.inheritance_rules.constraint_inheritance]
scope = { mode = "evolved", mutation_rate = 0.1 }
expiry = { mode = "evolved", mutation_rate = 0.2 }

[genetic_renewal.mutation_params]
context_mutation_rate = 0.05
constraint_evolution_rate = 0.1
enable_cross_context_breeding = true
```

### 4. **Permanent Key Configuration** (High Security)
```toml
[key_expiry]
allow_permanent = true        # Must be explicitly enabled

# Example permanent key via CLI/API (not config file)
# beardog key create --context identity --policy permanent \
#   --justification "Root CA signing key" \
#   --authorized-by "security-team@company.com"
```

### 5. **Activity-Based Session Keys**
```toml
[key_expiry.context_policies.session]
policy_type = "activity_based"
inactivity_timeout = "2h"     # Expire after 2h inactive
absolute_max_duration = "24h" # Never exceed 24h total
extension_duration = "1h"     # Extend 1h on each use
```

## Context-Specific Expiry Policies

### **Identity Keys** (Long-term, Genetic Renewal)
```rust
KeyExpiryPolicy::GeneticRenewal {
    generation_duration: Duration::days(90),    // 3 months
    max_generations: 10,
    generation_decay: 0.9,
}
```

### **Authentication Keys** (Entropy-Adaptive)
```rust
KeyExpiryPolicy::EntropyAdaptive {
    base_duration: Duration::hours(24),         // 1 day minimum
    max_duration: Duration::days(7),            // 1 week maximum
    entropy_weight: 1.5,                        // Strong entropy influence
}
```

### **Session Keys** (Activity-Based)
```rust
KeyExpiryPolicy::ActivityBased {
    inactivity_timeout: Duration::hours(2),     // 2h idle = expire
    absolute_max_duration: Duration::hours(24), // 24h absolute limit
    extension_duration: Duration::hours(1),     // +1h per use
}
```

### **Temporary Keys** (Fixed, Short)
```rust
KeyExpiryPolicy::Fixed {
    duration: Duration::hours(1),               // 1 hour fixed
}
```

### **Data Encryption Keys** (Usage-Based)
```rust
KeyExpiryPolicy::UsageBased {
    max_uses: 1000,                             // 1000 operations
    max_duration: Some(Duration::days(30)),     // 30 day time limit
}
```

## Entropy-Based Duration Calculation

### **Human Entropy (Tier 3)**
- **Fixed Policy**: 2x base duration
- **Adaptive Policy**: Full entropy weight applied
- **Genetic Renewal**: Enabled by default

### **Supervised Entropy (Tier 2)** 
- **Fixed Policy**: 1.5x base duration
- **Adaptive Policy**: Partial entropy weight
- **Genetic Renewal**: Enabled for quality > 0.7

### **Machine Entropy (Tier 1)**
- **Fixed Policy**: 1x base duration  
- **Adaptive Policy**: Minimal entropy weight
- **Genetic Renewal**: Disabled by default

## Genetic Spawning Workflow

### **Automatic Renewal Process**
1. **Approach Expiry**: Key reaches 90% of lifetime
2. **Eligibility Check**: Entropy quality ≥ 0.7, tier ≥ 2
3. **Genetic Analysis**: Determine inheritance and mutations
4. **Spawn Request**: Create new generation with evolved traits
5. **Approval Process**: Auto/human/hybrid based on configuration
6. **Seamless Transition**: New key replaces old without interruption

### **Evolution Characteristics**
- **Context Evolution**: Keys can adapt to new usage patterns
- **Constraint Mutation**: Security constraints evolve based on usage
- **Entropy Inheritance**: Mix parent entropy with new entropy sources
- **Cross-Context Breeding**: Inherit traits from multiple contexts

## Security Considerations

### **Permanent Key Safeguards**
- ✅ **Explicit Authorization Required**: Must be manually approved
- ✅ **Audit Trail**: Full justification and authorization tracking
- ✅ **Role-Based Access**: Only authorized personnel can create permanent keys
- ✅ **Regular Review**: Permanent keys require periodic validation

### **Genetic Renewal Security**
- ✅ **Quality Gates**: Minimum entropy quality requirements
- ✅ **Approval Workflows**: Human oversight for sensitive renewals
- ✅ **Inheritance Validation**: Verify evolved constraints are secure
- ✅ **Rollback Capability**: Ability to revert to previous generation

### **Expiry Enforcement**
- ✅ **Hard Expiry**: Keys become unusable after expiration
- ✅ **Grace Period**: Brief renewal window for critical operations
- ✅ **Automatic Cleanup**: Expired keys are securely destroyed
- ✅ **Usage Tracking**: Monitor key usage patterns for anomalies

## API Examples

### **Create Context Key with Custom Expiry**
```rust
let expiry_policy = KeyExpiryPolicy::EntropyAdaptive {
    base_duration: Duration::hours(12),
    max_duration: Duration::days(3),
    entropy_weight: 2.0,
};

let key = zfs_manager.generate_owner_encryption_key_with_policy(
    "user123",
    "user_auth",
    Some(expiry_policy)
).await?;
```

### **Create Permanent Key** (Requires Authorization)
```rust
let permanent_policy = KeyExpiryPolicy::Permanent {
    justification: "Root CA signing key for production".to_string(),
    authorized_by: "security-team@company.com".to_string(),
    granted_at: Utc::now(),
};

let key = zfs_manager.generate_owner_encryption_key_with_authorization(
    "root_ca",
    "identity", 
    permanent_policy,
    "admin_user_id"
).await?;
```

### **Check Key Renewal Status**
```rust
let should_renew = zfs_manager.check_key_renewal(&context_key).await?;

if should_renew && context_key.genetic_renewal.is_some() {
    let new_key = genetic_spawning_engine.spawn_evolved_key(&context_key).await?;
    // Seamless transition to new generation
}
```

## Migration from Hard-Coded Expiry

### **Backward Compatibility**
- **Default Behavior**: 24-hour fixed expiry (same as before)
- **Gradual Migration**: Can be enabled per context
- **Configuration Override**: Admin can set organization-wide policies

### **Migration Strategy**
1. **Phase 1**: Enable configurable expiry with 24h default
2. **Phase 2**: Introduce entropy-adaptive policies for high-security contexts
3. **Phase 3**: Enable genetic renewal for identity keys
4. **Phase 4**: Full rollout with context-specific policies

## Benefits

### **Security Improvements**
- ✅ **Adaptive Security**: Key lifetime matches entropy quality
- ✅ **Zero-Maintenance Renewal**: Genetic spawning provides seamless renewal
- ✅ **Fine-Grained Control**: Per-context expiry policies
- ✅ **Audit Compliance**: Full lifecycle tracking and justification

### **Operational Benefits**
- ✅ **Reduced Key Management**: Automatic renewal reduces manual operations
- ✅ **Context Awareness**: Keys adapt to their usage context
- ✅ **Scalable Configuration**: Organization-wide policy management
- ✅ **Future-Proof**: Genetic evolution adapts to changing requirements

### **User Experience**
- ✅ **Transparent Operation**: Genetic renewal happens automatically
- ✅ **Reduced Interruptions**: Longer expiry for high-quality entropy
- ✅ **Flexible Authorization**: Permanent keys when justified
- ✅ **Predictable Behavior**: Clear expiry policies per context

## Integration Points

### **Entropy Hierarchy System**
- Keys automatically inherit expiry bonuses based on entropy tier
- Self-sovereign entropy enables enhanced genetic capabilities
- Human entropy sources provide extended key lifetime

### **Genetic Spawning Engine**  
- Seamless integration with existing spawning workflows
- Key evolution follows same patterns as node spawning
- Cross-context breeding enables innovation

### **Configuration Management**
- TOML-based configuration for all expiry policies
- Runtime policy updates without system restart
- Environment-specific policy overrides

---

**Document Version**: 1.0  
**Last Updated**: 2025-01-11  
**Next Review**: 2025-01-25  
**Owner**: BearDog Engineering Team 