// SPDX-License-Identifier: AGPL-3.0-only

//! Configuration schema and parameter validation for provider initialization.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::configuration::ConfigurationValue;

/// Configuration schema for providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSchema {
    /// Schema parameters
    pub parameters: Vec<ConfigurationParameter>,
    
    /// Required parameters
    pub required: Vec<String>,
    
    /// Schema version
    pub version: String,
}

/// Configuration parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationParameter {
    /// Parameter name
    pub name: String,
    
    /// Parameter type
    pub parameter_type: ParameterType,
    
    /// Parameter description
    pub description: Option<String>,
    
    /// Default value
    pub default_value: Option<ConfigurationValue>,
    
    /// Validation rules
    pub validation: Vec<ValidationRule>,
}

/// Parameter types for configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Array(Box<ParameterType>),
    Object(HashMap<String, ParameterType>),
    Custom(String),
}

/// Validation rules for parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Rule type
    pub rule_type: ValidationRuleType,
    
    /// Rule parameters
    pub parameters: HashMap<String, ConfigurationValue>,
    
    /// Error message for validation failure
    pub error_message: String,
}

/// Types of validation rules
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationRuleType {
    Required,
    MinLength,
    MaxLength,
    Pattern,
    Range,
    Custom(String),
}
