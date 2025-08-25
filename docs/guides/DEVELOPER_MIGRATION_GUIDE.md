# BearDog Idiomatic Error Evolution - Developer Migration Guide

**Version**: 1.0  
**Date**: January 19, 2025  
**Target Audience**: BearDog Development Team  

---

## 🎯 **Overview**

This guide provides comprehensive instructions for migrating from `BearDogResult<T>` to idiomatic `Result<T, E>` patterns in the BearDog ecosystem. The migration preserves and enhances all rich error context while providing better type safety and domain-specific error handling.

---

## 📋 **Quick Reference**

### Before and After Comparison

```rust
// ❌ OLD: Generic error handling
fn authenticate_user(token: &str) -> BearDogResult<Session> {
    // Generic error with limited context
    Err(BearDogError::Security(SecurityError::AuthenticationFailed { 
        message: "Invalid token".to_string() 
    }))
}

// ✅ NEW: Domain-specific rich error handling
fn authenticate_user(token: &str) -> SecurityResult<Session> {
    // Rich error with metadata, AI insights, compliance context
    Err(SecurityError::Authentication {
        reason: "Invalid JWT signature".to_string(),
        context: create_security_context(),
        metadata: SecurityMetadata {
            threat_level: ThreatLevel::Medium,
            attack_vector: Some(AttackVector::TokenManipulation),
            compliance_impact: ComplianceImpact::DataProtection,
        },
        ai_remediation: Some(SecurityRemediation {
            suggested_actions: vec!["Rotate signing keys", "Audit token issuance"],
            confidence: 0.92,
            threat_assessment: ThreatAssessment::Moderate,
        }),
    })
}
```

---

## 🚀 **Migration Process**

### Phase 1: Planning and Analysis

1. **Analyze Your Module**
   ```bash
   python3 scripts/idiomatic_migration_analyzer.py crates/your-module/ --output analysis.json
   ```

2. **Review Migration Report**
   - Check domain type suggestions
   - Identify high-confidence patterns
   - Note manual intervention points

3. **Plan Migration Strategy**
   - Start with highest confidence files
   - Group related functionality
   - Plan testing approach

### Phase 2: Implementation

#### Step 1: Import New Types
```rust
// Add to your module's imports
use beardog_errors::{
    // Domain-specific result types
    SecurityResult, GeneticsResult, NetworkResult, WorkflowResult,
    
    // Domain-specific error types
    SecurityError, GeneticsError, NetworkError, WorkflowError,
    
    // Migration helpers (for gradual transition)
    migrate_security_result, migrate_genetics_result,
    migrate_network_result, migrate_workflow_result,
};
```

#### Step 2: Update Function Signatures
```rust
// ❌ Before
fn process_genetic_data(data: &GeneticData) -> BearDogResult<LineageInfo> {
    // ...
}

// ✅ After
fn process_genetic_data(data: &GeneticData) -> GeneticsResult<LineageInfo> {
    // ...
}
```

#### Step 3: Update Error Construction
```rust
// ❌ Before
Err(BearDogError::Genetics(GeneticsError::InvalidLineage { 
    message: "Lineage validation failed".to_string() 
}))

// ✅ After
Err(GeneticsError::InvalidLineage {
    lineage_id: lineage.id.clone(),
    reason: "Genetic diversity threshold not met".to_string(),
    context: create_genetics_context(),
    metadata: LineageMetadata {
        diversity_score: 0.23,
        generation: lineage.generation,
        parent_lineages: lineage.parents.clone(),
    },
    improvement: Some(DiversityImprovement {
        suggested_crossings: vec!["Lineage-A", "Lineage-C"],
        expected_diversity: 0.78,
        confidence: 0.95,
    }),
})
```

### Phase 3: Gradual Migration

For incremental migration, use compatibility helpers:

```rust
// Existing function returning BearDogResult
fn legacy_function() -> BearDogResult<T> { /* ... */ }

// New function using domain-specific error
fn new_function() -> SecurityResult<T> {
    let legacy_result = legacy_function();
    migrate_security_result(legacy_result)
}
```

---

## 🎨 **Domain-Specific Error Types**

### SecurityError - Authentication, Authorization, Encryption
```rust
fn authenticate_user(credentials: &Credentials) -> SecurityResult<Session> {
    Err(SecurityError::Authentication {
        reason: "MFA required".to_string(),
        user_id: Some(credentials.user_id.clone()),
        context: create_security_context(),
        metadata: SecurityMetadata {
            threat_level: ThreatLevel::Low,
            authentication_method: AuthMethod::Password,
            compliance_impact: ComplianceImpact::AccessControl,
        },
        ai_remediation: Some(SecurityRemediation {
            suggested_actions: vec!["Enable MFA", "Review access patterns"],
            confidence: 0.87,
            threat_assessment: ThreatAssessment::Low,
        }),
    })
}
```

### GeneticsError - Spawning, Lineage, Diversity
```rust
fn spawn_offspring(parents: &[LineageId]) -> GeneticsResult<Offspring> {
    Err(GeneticsError::InsufficientDiversity {
        required_diversity: 0.6,
        actual_diversity: 0.3,
        context: create_genetics_context(),
        metadata: LineageMetadata {
            diversity_score: 0.3,
            generation: 5,
            parent_lineages: parents.to_vec(),
        },
        improvement: Some(DiversityImprovement {
            suggested_crossings: vec!["high-diversity-lineage-x"],
            expected_diversity: 0.75,
            confidence: 0.92,
        }),
    })
}
```

### NetworkError - Connectivity, Timeout, Protocol
```rust
fn establish_connection(endpoint: &Endpoint) -> NetworkResult<Connection> {
    Err(NetworkError::ConnectionTimeout {
        endpoint: endpoint.clone(),
        timeout_duration: Duration::from_secs(30),
        context: create_network_context(),
        network_context: NetworkContext {
            protocol: Protocol::Https,
            retry_count: 3,
            last_error: Some("DNS resolution failed".to_string()),
        },
    })
}
```

### WorkflowError - Processing, Approval, State
```rust
fn execute_workflow(workflow_id: &str) -> WorkflowResult<ExecutionResult> {
    Err(WorkflowError::ProcessingFailed {
        workflow_id: workflow_id.to_string(),
        step: "approval_gate".to_string(),
        workflow_context: WorkflowContext {
            workflow_type: "security_review".to_string(),
            current_step: 3,
        },
        context: create_workflow_context(),
    })
}
```

---

## 🔧 **Migration Patterns**

### Pattern 1: Simple Error Conversion
```rust
// ❌ Before
return Err(BearDogError::validation("Invalid input"));

// ✅ After
return Err(SecurityError::ValidationFailed {
    field: "input_parameter".to_string(),
    reason: "Format validation failed".to_string(),
    context: create_security_context(),
    metadata: SecurityMetadata::default(),
    ai_remediation: None,
});
```

### Pattern 2: Error Propagation
```rust
// ❌ Before
fn outer_function() -> BearDogResult<T> {
    inner_function()?;
    // ...
}

// ✅ After
fn outer_function() -> SecurityResult<T> {
    inner_function().map_err(|e| SecurityError::DependencyFailed {
        dependency: "inner_function".to_string(),
        underlying_error: e.to_string(),
        context: create_security_context(),
        metadata: SecurityMetadata::default(),
        ai_remediation: None,
    })?;
    // ...
}
```

### Pattern 3: Cross-Domain Error Handling
```rust
// When calling functions from different domains
fn complex_operation() -> SecurityResult<T> {
    // Genetics operation
    let genetics_result: GeneticsResult<Data> = spawn_offspring(&parents);
    let data = genetics_result.map_err(|genetics_error| {
        SecurityError::DependencyFailed {
            dependency: "genetics_spawning".to_string(),
            underlying_error: genetics_error.to_string(),
            context: create_security_context(),
            metadata: SecurityMetadata::default(),
            ai_remediation: None,
        }
    })?;
    
    // Continue with security-specific processing
    Ok(process_securely(data))
}
```

---

## 🧪 **Testing Patterns**

### Unit Tests with Domain-Specific Errors
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_authentication_failure() {
        let result = authenticate_user(&invalid_credentials());
        
        match result {
            Err(SecurityError::Authentication { reason, metadata, .. }) => {
                assert_eq!(reason, "Invalid credentials");
                assert_eq!(metadata.threat_level, ThreatLevel::Medium);
            }
            _ => panic!("Expected AuthenticationFailed error"),
        }
    }
    
    #[test]
    fn test_error_context_preservation() {
        let result = complex_security_operation();
        
        if let Err(error) = result {
            // Verify rich context is preserved
            assert!(error.context().operation_id.is_some());
            assert!(error.metadata().compliance_impact != ComplianceImpact::None);
        }
    }
}
```

### Integration Tests with Migration Helpers
```rust
#[test]
fn test_backward_compatibility() {
    // Test that old BearDogResult can be converted to new domain types
    let legacy_result: BearDogResult<Data> = legacy_function();
    let new_result: SecurityResult<Data> = migrate_security_result(legacy_result);
    
    assert!(new_result.is_ok());
}
```

---

## 📊 **Best Practices**

### 1. **Error Construction**
- Always provide meaningful `reason` messages
- Include relevant context and metadata
- Use AI remediation when actionable suggestions are available
- Maintain consistency in error messages within your domain

### 2. **Error Propagation**
- Use `?` operator for same-domain errors
- Use explicit mapping for cross-domain errors
- Preserve original error information in `underlying_error` fields

### 3. **Testing Strategy**
- Test both success and error paths
- Verify error context and metadata
- Test cross-domain error handling
- Include backward compatibility tests during migration

### 4. **Performance Considerations**
- Domain-specific errors have zero runtime overhead
- Rich context is only created when errors occur
- Use `Option<T>` for optional metadata to minimize memory usage

---

## 🔄 **Migration Checklist**

### Pre-Migration
- [ ] Run analysis script on your module
- [ ] Review migration confidence scores
- [ ] Identify manual intervention points
- [ ] Plan testing strategy
- [ ] Coordinate with dependent modules

### During Migration
- [ ] Update function signatures
- [ ] Convert error construction sites
- [ ] Add appropriate imports
- [ ] Update error handling patterns
- [ ] Add rich context and metadata

### Post-Migration
- [ ] Run `cargo check` and fix compilation errors
- [ ] Update unit tests
- [ ] Run integration tests
- [ ] Verify backward compatibility
- [ ] Update documentation
- [ ] Run migration validator script

### Validation Commands
```bash
# Check compilation
cargo check --manifest-path crates/your-module/Cargo.toml

# Run tests
cargo test --manifest-path crates/your-module/Cargo.toml

# Validate migration
python3 scripts/migration_validator.py crates/your-module --verbose

# Format code
cargo fmt

# Check linting
cargo clippy
```

---

## 🆘 **Troubleshooting**

### Common Issues

#### 1. **Import Errors**
```rust
// ❌ Problem
error[E0432]: unresolved import `beardog_errors::SecurityResult`

// ✅ Solution
use beardog_errors::{SecurityResult, SecurityError};
```

#### 2. **Type Mismatch**
```rust
// ❌ Problem
expected SecurityResult<T>, found BearDogResult<T>

// ✅ Solution
use beardog_errors::migrate_security_result;
let security_result = migrate_security_result(beardog_result);
```

#### 3. **Missing Error Variants**
```rust
// ❌ Problem
no variant named `CustomError` found for enum `SecurityError`

// ✅ Solution: Use existing variant or add to domain_errors.rs
Err(SecurityError::ValidationFailed {
    field: "custom_field".to_string(),
    reason: "Custom validation failed".to_string(),
    // ... other fields
})
```

#### 4. **Cross-Domain Dependencies**
```rust
// ❌ Problem: Genetics function returning SecurityResult
fn genetics_operation() -> SecurityResult<T> { /* ... */ }

// ✅ Solution: Use correct domain type
fn genetics_operation() -> GeneticsResult<T> { /* ... */ }

// Or wrap in appropriate domain
fn security_wrapper() -> SecurityResult<T> {
    genetics_operation().map_err(|e| SecurityError::DependencyFailed {
        dependency: "genetics".to_string(),
        underlying_error: e.to_string(),
        // ... other fields
    })
}
```

---

## 📈 **Migration Metrics and Goals**

### Target Metrics
- **Automation Success**: 95%+ automated conversion
- **Compilation**: Zero errors after migration
- **Test Coverage**: Maintain existing coverage
- **Performance**: Zero runtime impact
- **Documentation**: 100% error types documented

### Success Criteria
- [ ] All function signatures updated
- [ ] All error construction sites converted
- [ ] Rich context preserved and enhanced
- [ ] Backward compatibility maintained
- [ ] Tests pass with new error types
- [ ] Documentation updated

---

## 🎯 **Module Rollout Priority**

Based on analysis and domain concentration:

1. **✅ beardog-security** (Complete - 378 usages, 95.8% security)
2. **🔄 beardog-genetics** (Next - High genetics concentration)
3. **📋 beardog-core** (High impact, multiple domains)
4. **📋 beardog-workflows** (Clear workflow domain)
5. **📋 beardog-api** (Mixed domains, careful handling needed)

---

## 💡 **Advanced Topics**

### Custom Error Metadata
```rust
// Extending error types with custom metadata
impl SecurityError {
    pub fn with_custom_metadata(mut self, key: String, value: String) -> Self {
        if let SecurityError::Authentication { ref mut metadata, .. } = &mut self {
            metadata.custom_fields.insert(key, value);
        }
        self
    }
}
```

### Error Analytics Integration
```rust
// Integrating with monitoring and analytics
impl SecurityError {
    pub fn report_to_analytics(&self) {
        analytics::report_security_event(SecurityEvent {
            error_type: self.error_type(),
            threat_level: self.metadata().threat_level,
            context: self.context().clone(),
        });
    }
}
```

---

## 📞 **Support and Resources**

- **Migration Scripts**: `scripts/idiomatic_migration_*`
- **Validation Tools**: `scripts/migration_validator.py`
- **Documentation**: `specs/IDIOMATIC_RESULT_EVOLUTION_SPECIFICATION.md`
- **Examples**: `examples/idiomatic_evolution_demo.rs`
- **Team Chat**: #beardog-error-evolution

---

**Happy Migrating! 🚀**

*Remember: The goal is not just to change types, but to create more expressive, actionable, and maintainable error handling that enhances the entire BearDog ecosystem.* 