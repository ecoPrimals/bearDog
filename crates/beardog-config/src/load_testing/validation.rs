//! Performance Validation Configuration
//!
//! This module defines performance validation and data validation
//! configurations for load testing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Performance validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceValidationConfig {
    /// Enable validation
    pub enabled: bool,
    /// Validation rules
    pub rules: Vec<ValidationRule>,
    /// Performance thresholds
    pub thresholds: PerformanceThresholds,
    /// Data validation
    pub data_validation: DataValidationConfig,
}

/// Performance thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Response time thresholds
    pub response_time: ResponseTimeThresholds,
    /// Throughput thresholds
    pub throughput: ThroughputThresholds,
    /// Error rate thresholds
    pub error_rate: ErrorRateThresholds,
    /// Resource utilization thresholds
    pub resource_utilization: ResourceUtilizationThresholds,
}

/// Response time thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimeThresholds {
    /// Mean response time
    pub mean: Duration,
    /// 95th percentile response time
    pub p95: Duration,
    /// 99th percentile response time
    pub p99: Duration,
    /// Maximum response time
    pub max: Duration,
}

/// Throughput thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputThresholds {
    /// Minimum requests per second
    pub min_rps: f64,
    /// Target requests per second
    pub target_rps: f64,
    /// Maximum requests per second
    pub max_rps: f64,
}

/// Error rate thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRateThresholds {
    /// Maximum error rate
    pub max_error_rate: f64,
    /// Warning error rate
    pub warning_error_rate: f64,
    /// Critical error rate
    pub critical_error_rate: f64,
}

/// Resource utilization thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilizationThresholds {
    /// CPU utilization threshold
    pub cpu: f64,
    /// Memory utilization threshold
    pub memory: f64,
    /// Disk utilization threshold
    pub disk: f64,
    /// Network utilization threshold
    pub network: f64,
}

/// Data validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationConfig {
    /// Enable validation
    pub enabled: bool,
    /// Validation rules
    pub rules: Vec<ValidationRule>,
    /// Validation sampling
    pub sampling: ValidationSampling,
}

/// Validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Rule name
    pub name: String,
    /// Rule type
    pub rule_type: ValidationRuleType,
    /// Rule parameters
    pub parameters: HashMap<String, String>,
}

/// Validation rule type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    /// Schema validation
    Schema,
    /// Format validation
    Format,
    /// Range validation
    Range,
    /// Custom validation
    Custom { validator: String },
}

/// Validation sampling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSampling {
    /// Sampling rate
    pub rate: f64,
    /// Sampling method
    pub method: SamplingMethod,
}

/// Sampling method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SamplingMethod {
    /// Random sampling
    Random,
    /// Systematic sampling
    Systematic,
    /// Stratified sampling
    Stratified,
}

impl Default for PerformanceValidationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Vec::new(),
            thresholds: PerformanceThresholds::default(),
            data_validation: DataValidationConfig::default(),
        }
    }
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            response_time: ResponseTimeThresholds::default(),
            throughput: ThroughputThresholds::default(),
            error_rate: ErrorRateThresholds::default(),
            resource_utilization: ResourceUtilizationThresholds::default(),
        }
    }
}

impl Default for ResponseTimeThresholds {
    fn default() -> Self {
        Self {
            mean: Duration::from_millis(100),
            p95: Duration::from_millis(500),
            p99: Duration::from_millis(1000),
            max: Duration::from_millis(5000),
        }
    }
}

impl Default for ThroughputThresholds {
    fn default() -> Self {
        Self {
            min_rps: 10.0,
            target_rps: 100.0,
            max_rps: 1000.0,
        }
    }
}

impl Default for ErrorRateThresholds {
    fn default() -> Self {
        Self {
            max_error_rate: 0.01,
            warning_error_rate: 0.005,
            critical_error_rate: 0.05,
        }
    }
}

impl Default for ResourceUtilizationThresholds {
    fn default() -> Self {
        Self {
            cpu: 80.0,
            memory: 80.0,
            disk: 80.0,
            network: 80.0,
        }
    }
}

impl Default for DataValidationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Vec::new(),
            sampling: ValidationSampling::default(),
        }
    }
}

impl Default for ValidationSampling {
    fn default() -> Self {
        Self {
            rate: 0.1,
            method: SamplingMethod::Random,
        }
    }
}
