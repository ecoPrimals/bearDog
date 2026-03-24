// SPDX-License-Identifier: AGPL-3.0-only

// Safety checking for external functions

use super::types::{ExternalFunction, FunctionParameter, FunctionValue, SecurityClearance};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

// Note: PolicyEngine, PolicyContext, and UnifiedTraitError traits are not yet
// defined in the unified trait system. These would need to be added to
// beardog-types/src/canonical/providers_unified/traits/ if the functionality is needed.

/// A parameter with its associated value for safety validation
///
/// Pairs function parameter definitions with their runtime values for safety checks.
#[derive(Debug, Clone)]
pub struct ParameterValue {
    /// Function parameter definition including type and constraints
    pub parameter: FunctionParameter,
    /// Actual value provided for this parameter
    pub value: FunctionValue,
}

/// Safety policy for external function calls
///
/// Defines security requirements and validation rules for calling external functions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyPolicy {
    /// Unique policy identifier
    pub policy_id: String,
    /// Function name patterns this policy applies to (glob patterns)
    pub function_patterns: Vec<String>,
    /// Minimum security clearance required to call these functions
    pub required_clearance: SecurityClearance,
    /// Whether to validate parameters before calling
    pub parameter_validation: bool,
}

/// Result of a safety check for an external function call
///
/// Indicates whether a call is allowed and what actions must be taken.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyDecision {
    /// Whether the function call is allowed
    pub allowed: bool,
    /// Reason for the decision
    pub reason: String,
    /// Actions that must be taken before/after the call
    pub required_actions: Vec<String>,
}

/// Safety checker for external function calls
///
/// Validates external function calls against security policies and clearance levels.
#[derive(Debug)]
pub struct SafetyChecker {
    /// Current security clearance level
    security_clearance: SecurityClearance,
    /// Loaded safety policies by policy ID
    loaded_policies: HashMap<String, SafetyPolicy>,
}

impl SafetyChecker {
    /// Create new safety checker
    /// Creates a new instance
    #[must_use]
    pub fn new(security_clearance: SecurityClearance) -> Self {
        Self {
            security_clearance,
            loaded_policies: HashMap::new(),
        }
    }

    /// Check if function call is safe
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Insufficient security clearance
    /// - Parameter validation fails
    pub fn check_function_call(
        &self,
        function: &ExternalFunction,
        parameters: &[ParameterValue],
    ) -> Result<(), BearDogError> {
        debug!("Checking safety for function: {}", function.name);

        // Check security clearance
        if function.metadata.security.clearance_required > self.security_clearance {
            return Err(BearDogError::security(
                "Insufficient security clearance for function call".to_string(),
            ));
        }

        // Validate parameters
        for param_value in parameters {
            Self::validate_parameter(param_value)?;
        }

        Ok(())
    }

    /// Validate a single parameter
    /// Validates parameter
    fn validate_parameter(param_value: &ParameterValue) -> Result<(), BearDogError> {
        // For now, just basic validation - can be extended
        if matches!(&param_value.value, FunctionValue::Null) && param_value.parameter.required {
            return Err(BearDogError::validation(
                "Required parameter cannot be null",
            ));
        }

        Ok(())
    }

    /// Load a safety policy
    ///
    /// # Errors
    ///
    /// Returns an error if policy validation fails
    pub fn load_policy(&mut self, policy: SafetyPolicy) -> Result<(), BearDogError> {
        // Validate policy before loading
        if policy.policy_id.is_empty() {
            return Err(BearDogError::validation("Policy ID cannot be empty"));
        }
        if policy.function_patterns.is_empty() {
            return Err(BearDogError::validation(
                "Policy must have at least one function pattern",
            ));
        }

        self.loaded_policies
            .insert(policy.policy_id.clone(), policy);
        Ok(())
    }

    /// List all loaded policy IDs
    ///
    /// # Errors
    ///
    /// Returns an error if policy retrieval fails
    pub fn list_policies(&self) -> Result<Vec<String>, BearDogError> {
        Ok(self.loaded_policies.keys().cloned().collect())
    }

    /// Remove a policy by ID
    ///
    /// # Errors
    ///
    /// Returns an error if policy doesn't exist
    pub fn remove_policy(&mut self, policy_id: &str) -> Result<(), BearDogError> {
        if self.loaded_policies.remove(policy_id).is_none() {
            return Err(BearDogError::validation("Policy not found"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::external_functions::types::{
        ExternalFunction, FunctionMetadata, FunctionParameter, FunctionSignature, FunctionValue,
        ParameterType, PerformanceInfo, ReturnType, SecurityClearance, SecurityInfo,
    };

    fn make_function(clearance: SecurityClearance) -> ExternalFunction {
        ExternalFunction {
            id: "test-id".to_string(),
            name: "test_func".to_string(),
            signature: FunctionSignature {
                parameters: vec![],
                return_type: ReturnType::Type(ParameterType::Int32),
                calling_convention: crate::external_functions::types::CallingConvention::C,
                attributes: vec![],
            },
            metadata: FunctionMetadata {
                description: "test".to_string(),
                safety_level: crate::external_functions::types::SafetyLevel::Safe,
                performance: PerformanceInfo::default(),
                security: SecurityInfo {
                    clearance_required: clearance,
                    audit_required: false,
                    sandbox_required: false,
                    access_restrictions: vec![],
                },
                custom: std::collections::HashMap::new(),
            },
            library_id: "lib".to_string(),
        }
    }

    #[test]
    fn test_safety_checker_insufficient_clearance() {
        let checker = SafetyChecker::new(SecurityClearance::Public);
        let function = make_function(SecurityClearance::Secret);
        let param_values = vec![];
        let result = checker.check_function_call(&function, &param_values);
        assert!(result.is_err());
    }

    #[test]
    fn test_safety_checker_sufficient_clearance() {
        let checker = SafetyChecker::new(SecurityClearance::Secret);
        let function = make_function(SecurityClearance::Public);
        let param_values = vec![];
        let result = checker.check_function_call(&function, &param_values);
        assert!(result.is_ok());
    }

    #[test]
    fn test_safety_checker_required_param_null() {
        let checker = SafetyChecker::new(SecurityClearance::Public);
        let function = make_function(SecurityClearance::Public);
        let param_values = vec![ParameterValue {
            parameter: FunctionParameter {
                name: "req".to_string(),
                param_type: ParameterType::Int32,
                required: true,
                default_value: None,
            },
            value: FunctionValue::Null,
        }];
        let result = checker.check_function_call(&function, &param_values);
        assert!(result.is_err());
    }

    #[test]
    fn test_safety_checker_load_policy_empty_id() {
        let mut checker = SafetyChecker::new(SecurityClearance::Public);
        let policy = SafetyPolicy {
            policy_id: String::new(),
            function_patterns: vec!["*".to_string()],
            required_clearance: SecurityClearance::Public,
            parameter_validation: true,
        };
        let result = checker.load_policy(policy);
        assert!(result.is_err());
    }

    #[test]
    fn test_safety_checker_load_policy_empty_patterns() {
        let mut checker = SafetyChecker::new(SecurityClearance::Public);
        let policy = SafetyPolicy {
            policy_id: "test".to_string(),
            function_patterns: vec![],
            required_clearance: SecurityClearance::Public,
            parameter_validation: true,
        };
        let result = checker.load_policy(policy);
        assert!(result.is_err());
    }

    #[test]
    fn test_safety_checker_remove_policy_not_found() {
        let mut checker = SafetyChecker::new(SecurityClearance::Public);
        let result = checker.remove_policy("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_safety_checker_list_and_remove_policy() {
        let mut checker = SafetyChecker::new(SecurityClearance::Public);
        let policy = SafetyPolicy {
            policy_id: "p1".to_string(),
            function_patterns: vec!["f*".to_string()],
            required_clearance: SecurityClearance::Public,
            parameter_validation: true,
        };
        checker
            .load_policy(policy)
            .expect("valid test policy should load");
        let policies = checker.list_policies().expect("list policies after load");
        assert!(policies.contains(&"p1".to_string()));
        let result = checker.remove_policy("p1");
        assert!(result.is_ok());
        let policies = checker.list_policies().expect("list policies after remove");
        assert!(!policies.contains(&"p1".to_string()));
    }
}

// PolicyEngine trait implementation commented out until traits are added to unified system
// PHASE-2(Policy): Re-enable when PolicyEngine, PolicyContext, and UnifiedTraitError are added to
// beardog-types/src/canonical/providers_unified/traits/
/*
impl PolicyEngine for SafetyChecker {
    type Policy = SafetyPolicy;
    type Decision = SafetyDecision;

    async fn evaluate(
        &self,
        policy: &Self::Policy,
        _context: PolicyContext,
    ) -> Result<Self::Decision, UnifiedTraitError> {
        // Basic policy evaluation logic
        let allowed = self.security_clearance >= policy.required_clearance;

        let decision = SafetyDecision {
            allowed,
            reason: if allowed {
                "Security clearance sufficient".to_string()
            } else {
                "Insufficient security clearance".to_string()
            },
            required_actions: if allowed {
                vec![]
            } else {
                vec!["Upgrade security clearance".to_string()]
            },
        };

        Ok(decision)
    }

    async fn load_policy(&mut self, policy: Self::Policy) -> Result<(), UnifiedTraitError> {
        self.loaded_policies
            .insert(policy.policy_id.clone(), policy);
        Ok(())
    }

    async fn list_policies(&self) -> Result<Vec<String>, UnifiedTraitError> {
        Ok(self.loaded_policies.keys().cloned().collect())
    }

    async fn remove_policy(&mut self, policy_id: &str) -> Result<(), UnifiedTraitError> {
        self.loaded_policies.remove(policy_id);
        Ok(())
    }

    async fn validate_policy(&self, policy: &Self::Policy) -> Result<bool, UnifiedTraitError> {
        // Basic policy validation
        Ok(!policy.policy_id.is_empty() && !policy.function_patterns.is_empty())
    }
}
*/
