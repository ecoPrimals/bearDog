// Safety checking for external functions

use super::types::{ExternalFunction, FunctionParameter, FunctionValue, SecurityClearance};
use beardog_errors::BearDogError;
use beardog_traits::unified::security::PolicyContext;
use beardog_traits::unified::{PolicyEngine, UnifiedTraitError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct ParameterValue {
    /// Function parameter definition
    /// The parameter value
    pub parameter: FunctionParameter,
    /// The value value
    pub value: FunctionValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyPolicy {
    pub policy_id: String,
    /// Patterns of function names this policy applies to
    /// Collection of function patterns
    pub function_patterns: Vec<String>,
    /// Minimum security clearance required
    /// The required clearance value
    pub required_clearance: SecurityClearance,
    /// Whether parameter validation is required
    pub parameter_validation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyDecision {
    /// Whether allowed is enabled
    pub allowed: bool,
    /// The reason value
    pub reason: String,
    /// Collection of required actions
    pub required_actions: Vec<String>,
}

#[derive(Debug)]
pub struct SafetyChecker {
    security_clearance: SecurityClearance,
    loaded_policies: HashMap<String, SafetyPolicy>,
}

impl SafetyChecker {
    /// Create new safety checker
    /// Creates a new instance
    pub fn new(security_clearance: SecurityClearance) -> Self {
        Self {
            security_clearance,
            loaded_policies: HashMap::new(),
        }
    }

    /// Check if function call is safe
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
            self.validate_parameter(param_value)?;
        }

        Ok(())
    }

    /// Validate a single parameter
    /// Validates parameter
    fn validate_parameter(&self, param_value: &ParameterValue) -> Result<(), BearDogError> {
        // For now, just basic validation - can be extended
        match &param_value.value {
            FunctionValue::Null => {
                if param_value.parameter.required {
                    return Err(BearDogError::validation(
                        "Required parameter cannot be null",
                    ));
                }
            }
            _ => {} // Other validations can be added here
        }

        Ok(())
    }
}

impl PolicyEngine for SafetyChecker {
    type Policy = SafetyPolicy;
    type Decision = SafetyDecision;

    fn evaluate(
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

    /// Loads policy
    fn load_policy(&mut self, policy: Self::Policy) -> Result<(), UnifiedTraitError> {
        self.loaded_policies
            .insert(policy.policy_id.clone(), policy);
        Ok(())
    }

    fn list_policies(&self) -> Result<Vec<String>, UnifiedTraitError> {
        Ok(self.loaded_policies.keys().cloned().collect())
    }

    /// Removes policy
    fn remove_policy(&mut self, policy_id: &str) -> Result<(), UnifiedTraitError> {
        self.loaded_policies.remove(policy_id);
        Ok(())
    }

    /// Validates policy
    fn validate_policy(&self, policy: &Self::Policy) -> Result<bool, UnifiedTraitError> {
        // Basic policy validation
        Ok(!policy.policy_id.is_empty() && !policy.function_patterns.is_empty())
    }
}
