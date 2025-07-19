# Context-Aware Key Configuration - Complete Configuration Guide

## Overview

**ALL VALUES ARE NOW CONFIGURABLE** - No more hard-coded entropy tiers, quality scores, expiry policies, or thresholds. This document shows how to configure every aspect of the context-aware key system.

## Complete Configuration Structure

### 1. **Main Configuration File** (`beardog-key-config.toml`)

```toml
# =============================================================================
# GLOBAL KEY EXPIRY CONFIGURATION
# =============================================================================
[key_expiry]
# Security-first: permanent keys must be explicitly enabled
allow_permanent = false
# Maximum expiry duration (safety limit)
max_expiry_duration = "8760h"  # 1 year (365 days * 24 hours)
# Minimum expiry duration (prevent too-short keys)
min_expiry_duration = "5m"
# Whether to enable entropy-based expiry adjustments
entropy_based_adjustment = true
# Whether to enable genetic spawning for key renewal
enable_genetic_renewal = true

# Default policy for contexts not explicitly configured
[key_expiry.default_policy]
policy_type = "fixed"
duration = "24h"

# =============================================================================
# CONTEXT-SPECIFIC CONFIGURATIONS
# =============================================================================

# Identity keys - Long-term, maximum security
[context_configs.identity]
default_entropy_tier = 3                              # Human entropy required
default_entropy_quality = 0.9                        # High quality required
default_human_source = "biometric"                   # Biometric preferred
is_self_sovereign = true                              # Self-sovereign identity
security_level = "maximum"                           # Maximum security
[context_configs.identity.default_expiry_policy]
policy_type = "genetic_renewal"
generation_duration = "2160h"                        # 90 days per generation
max_generations = 10                                 # Up to 10 generations
generation_decay = 0.9                               # 90% decay per generation

# Signing keys - Long-term, maximum security
[context_configs.signing]
default_entropy_tier = 3
default_entropy_quality = 0.9
default_human_source = "biometric"
is_self_sovereign = true
security_level = "maximum"
[context_configs.signing.default_expiry_policy]
policy_type = "genetic_renewal"
generation_duration = "2160h"                        # 90 days
max_generations = 10
generation_decay = 0.9

# User authentication - Entropy-adaptive
[context_configs.user_auth]
default_entropy_tier = 3
default_entropy_quality = 0.9
default_human_source = "biometric"
is_self_sovereign = true
security_level = "high"
[context_configs.user_auth.default_expiry_policy]
policy_type = "entropy_adaptive"
base_duration = "24h"                                # Base: 1 day
max_duration = "168h"                                # Max: 7 days
entropy_weight = 1.5                                 # Strong entropy influence

# Data encryption - Usage-based
[context_configs.data_encryption]
default_entropy_tier = 3
default_entropy_quality = 0.8
default_human_source = "multimodal"
is_self_sovereign = true
security_level = "high"
[context_configs.data_encryption.default_expiry_policy]
policy_type = "usage_based"
max_uses = 1000                                      # 1000 operations
max_duration = "720h"                                # 30 days max

# Session keys - Activity-based
[context_configs.session]
default_entropy_tier = 2                             # Supervised entropy
default_entropy_quality = 0.7
default_human_source = null                          # No human source required
is_self_sovereign = false
security_level = "medium"
[context_configs.session.default_expiry_policy]
policy_type = "activity_based"
inactivity_timeout = "2h"                           # 2 hours idle = expire
absolute_max_duration = "24h"                       # 24 hours absolute max
extension_duration = "1h"                           # +1 hour per activity

# Temporary keys - Short-lived
[context_configs.temporary]
default_entropy_tier = 2
default_entropy_quality = 0.6
default_human_source = null
is_self_sovereign = false
security_level = "medium"
[context_configs.temporary.default_expiry_policy]
policy_type = "fixed"
duration = "1h"                                      # 1 hour fixed

# Default fallback - Machine entropy
[context_configs.default]
default_entropy_tier = 1                             # Machine entropy
default_entropy_quality = 0.5
default_human_source = null
is_self_sovereign = false
security_level = "basic"
[context_configs.default.default_expiry_policy]
policy_type = "fixed"
duration = "24h"                                     # 24 hours default

# =============================================================================
# ENTROPY ADJUSTMENT CONFIGURATION
# =============================================================================
[entropy_adjustments]
# Duration multipliers by entropy tier (replaces hard-coded 1x, 1.5x, 2x)
[entropy_adjustments.tier_multipliers]
1 = 1.0    # Machine entropy: 1x base duration
2 = 1.5    # Supervised entropy: 1.5x base duration  
3 = 2.0    # Human entropy: 2x base duration

# Genetic renewal thresholds (replaces hard-coded 0.7 and tier 2)
genetic_renewal_quality_threshold = 0.7              # Require 70% quality
genetic_renewal_tier_threshold = 2                   # Require tier 2+

# Security quality thresholds by level
[entropy_adjustments.security_quality_thresholds]
basic = 0.3      # Basic security: 30% quality minimum
medium = 0.5     # Medium security: 50% quality minimum
high = 0.7       # High security: 70% quality minimum
maximum = 0.9    # Maximum security: 90% quality minimum

# =============================================================================
# GENETIC RENEWAL CONFIGURATION
# =============================================================================
[genetic_renewal]
enabled = true
min_entropy_quality = 0.7                           # Minimum quality for renewal
spawning_workflow = "hybrid"                        # Auto with human oversight

# Inheritance rules for genetic renewal
[genetic_renewal.inheritance_rules]
inheritable_contexts = ["user_auth", "data_encryption", "identity"]

# How different properties are inherited
[genetic_renewal.inheritance_rules.constraint_inheritance]
scope = { mode = "evolved", mutation_rate = 0.1 }
expiry = { mode = "evolved", mutation_rate = 0.2 }
security = { mode = "exact" }                       # Security constraints don't mutate

# How entropy is inherited
[genetic_renewal.inheritance_rules.entropy_inheritance]
mode = "mix_with_new"
mix_ratio = 0.7                                     # 70% parent, 30% new entropy

# Mutation parameters for evolution
[genetic_renewal.mutation_params]
context_mutation_rate = 0.05                       # 5% context mutation
constraint_evolution_rate = 0.1                    # 10% constraint evolution
expiry_adaptation_rate = 0.15                      # 15% expiry adaptation
enable_cross_context_breeding = true               # Allow cross-context breeding

# =============================================================================
# KEY LIFECYCLE THRESHOLDS
# =============================================================================
[lifecycle_thresholds]
# Renewal warning threshold (replaces hard-coded 90%)
renewal_warning_threshold = 0.9                    # 90% of lifetime
# Grace period after expiry for renewal
expiry_grace_period = 300                          # 5 minutes
# Maximum renewal attempts before giving up
max_renewal_attempts = 3
# Cleanup delay after expiry
cleanup_delay = 3600                               # 1 hour
# Usage pattern analysis window
usage_analysis_window = 86400                      # 24 hours
```

### 2. **Environment-Specific Overrides**

#### **Development Environment** (`dev-override.toml`)
```toml
# Relaxed settings for development
[key_expiry]
allow_permanent = true                              # Allow permanent keys in dev
min_expiry_duration = "30s"                       # Shorter minimum for testing

[entropy_adjustments]
genetic_renewal_quality_threshold = 0.3            # Lower threshold for dev
genetic_renewal_tier_threshold = 1                 # Allow machine entropy renewal

[lifecycle_thresholds]
renewal_warning_threshold = 0.5                    # 50% warning for faster testing
expiry_grace_period = 60                          # 1 minute grace period
```

#### **Production Environment** (`prod-override.toml`)
```toml
# Strict settings for production
[key_expiry]
allow_permanent = false                            # No permanent keys in prod
max_expiry_duration = "4320h"                     # 6 months max (stricter)

[entropy_adjustments]
genetic_renewal_quality_threshold = 0.8            # Higher threshold for prod
genetic_renewal_tier_threshold = 3                # Require human entropy

[lifecycle_thresholds]
renewal_warning_threshold = 0.95                  # 95% warning (more time)
expiry_grace_period = 60                          # 1 minute grace period
max_renewal_attempts = 1                          # Only 1 attempt in prod
```

### 3. **Custom Context Configuration**

#### **Adding New Context** (`custom-contexts.toml`)
```toml
# API keys - Medium security, long-lived
[context_configs.api_key]
default_entropy_tier = 2
default_entropy_quality = 0.6
default_human_source = null
is_self_sovereign = false
security_level = "medium"
[context_configs.api_key.default_expiry_policy]
policy_type = "usage_based"
max_uses = 10000                                   # 10,000 API calls
max_duration = "8760h"                             # 1 year max

# Database keys - High security, entropy-adaptive
[context_configs.database]
default_entropy_tier = 3
default_entropy_quality = 0.8
default_human_source = "haptic"
is_self_sovereign = true
security_level = "high"
[context_configs.database.default_expiry_policy]
policy_type = "entropy_adaptive"
base_duration = "12h"                              # 12 hour base
max_duration = "72h"                               # 3 days max
entropy_weight = 2.0                               # Strong entropy influence

# Backup keys - Maximum security, genetic renewal
[context_configs.backup]
default_entropy_tier = 3
default_entropy_quality = 0.95
default_human_source = "biometric"
is_self_sovereign = true
security_level = "maximum"
[context_configs.backup.default_expiry_policy]
policy_type = "genetic_renewal"
generation_duration = "4320h"                      # 6 months per generation
max_generations = 5                                # 5 generations max
generation_decay = 0.95                            # 95% decay (very slow)
```

## Configuration Loading Priority

1. **Base Configuration**: Default values from `ContextAwareKeyConfig::default()`
2. **Main Config File**: `beardog-key-config.toml`
3. **Environment Override**: `{env}-override.toml` (dev, staging, prod)
4. **Custom Contexts**: `custom-contexts.toml`
5. **Runtime Overrides**: Environment variables with `BEARDOG_KEY_` prefix

## Environment Variable Overrides

```bash
# Override global settings
export BEARDOG_KEY_ALLOW_PERMANENT=true
export BEARDOG_KEY_MAX_EXPIRY_DURATION=8760h
export BEARDOG_KEY_MIN_EXPIRY_DURATION=1m

# Override context-specific settings
export BEARDOG_KEY_CONTEXT_IDENTITY_ENTROPY_TIER=3
export BEARDOG_KEY_CONTEXT_IDENTITY_ENTROPY_QUALITY=0.9
export BEARDOG_KEY_CONTEXT_IDENTITY_SELF_SOVEREIGN=true

# Override entropy adjustments
export BEARDOG_KEY_ENTROPY_TIER_MULTIPLIER_1=1.0
export BEARDOG_KEY_ENTROPY_TIER_MULTIPLIER_2=1.5
export BEARDOG_KEY_ENTROPY_TIER_MULTIPLIER_3=2.0

# Override lifecycle thresholds
export BEARDOG_KEY_RENEWAL_WARNING_THRESHOLD=0.9
export BEARDOG_KEY_EXPIRY_GRACE_PERIOD=300
export BEARDOG_KEY_MAX_RENEWAL_ATTEMPTS=3
```

## API Configuration Examples

### **Runtime Configuration Updates**
```rust
// Update context configuration at runtime
let mut key_config = ContextAwareKeyConfig::default();

// Add new context
key_config.context_configs.insert("webhook".to_string(), ContextConfig {
    default_entropy_tier: 2,
    default_entropy_quality: 0.7,
    default_human_source: None,
    is_self_sovereign: false,
    default_expiry_policy: KeyExpiryPolicy::Fixed {
        duration: Duration::hours(6),
    },
    security_level: SecurityLevel::Medium,
});

// Update entropy adjustments
key_config.entropy_adjustments.tier_multipliers.insert(4, 3.0); // Tier 4 = 3x multiplier

// Update lifecycle thresholds
key_config.lifecycle_thresholds.renewal_warning_threshold = 0.85; // 85% warning
```

### **Per-Key Configuration Overrides**
```rust
// Override configuration for specific key generation
let custom_policy = KeyExpiryPolicy::Permanent {
    justification: "Root CA signing key for production".to_string(),
    authorized_by: "security-team@company.com".to_string(),
    granted_at: Utc::now(),
};

let key = zfs_manager.generate_owner_encryption_key_with_policy(
    "root_ca",
    "identity",
    Some(custom_policy),
    Some("admin_authorization_token")
).await?;
```

## Benefits of Full Configuration

### **No Hard-Coded Values**
- ✅ **Entropy tiers**: Fully configurable (1-3 default, can add tier 4+)
- ✅ **Quality thresholds**: Configurable per security level
- ✅ **Duration multipliers**: Configurable per entropy tier
- ✅ **Expiry policies**: Configurable per context
- ✅ **Genetic renewal**: All thresholds and parameters configurable
- ✅ **Lifecycle thresholds**: Warning times, grace periods, attempts

### **Environment Flexibility**
- ✅ **Development**: Relaxed settings for testing
- ✅ **Staging**: Production-like with some flexibility
- ✅ **Production**: Strict security requirements
- ✅ **Custom**: Organization-specific requirements

### **Runtime Adaptability**
- ✅ **Hot Updates**: Change configuration without restart
- ✅ **A/B Testing**: Different configurations for different users
- ✅ **Compliance**: Adapt to changing regulatory requirements
- ✅ **Scaling**: Adjust thresholds based on load

### **Organization Customization**
- ✅ **New Contexts**: Add organization-specific key contexts
- ✅ **Custom Policies**: Define organization-specific expiry policies
- ✅ **Compliance Rules**: Configure to meet specific compliance requirements
- ✅ **Security Levels**: Define organization-specific security levels

## Migration Strategy

### **Phase 1: Configuration Infrastructure**
1. Deploy configuration loading system
2. Set defaults to match current hard-coded values
3. Verify no behavior changes

### **Phase 2: Environment Differentiation**
1. Create environment-specific overrides
2. Test in development with relaxed settings
3. Gradually differentiate staging and production

### **Phase 3: Custom Context Rollout**
1. Identify organization-specific contexts
2. Create custom context configurations
3. Migrate existing keys to new contexts

### **Phase 4: Advanced Features**
1. Enable runtime configuration updates
2. Implement A/B testing for configuration changes
3. Add compliance-specific configuration templates

---

**Document Version**: 1.0  
**Last Updated**: 2025-01-11  
**Next Review**: 2025-01-25  
**Owner**: BearDog Engineering Team 