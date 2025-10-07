// Unified Error Helpers for Testing Framework
//
// This module provides helper functions to create BearDogError instances
// for testing framework operations, replacing the custom error types.

use beardog_errors::{BearDogError, TestingErrorCategory};

/// Helper functions for creating testing errors with appropriate categories
pub struct TestingErrorHelpers;

impl TestingErrorHelpers {
    /// Create a property-based testing error
    pub fn property_error(message: &str) -> BearDogError {
        BearDogError::testing_with_category(message, TestingErrorCategory::Property)
    }

    /// Create a mutation testing error
    pub fn mutation_error(message: &str) -> BearDogError {
        BearDogError::testing_with_category(message, TestingErrorCategory::Mutation)
    }

    /// Create an invariant validation error
    pub fn invariant_error(message: &str) -> BearDogError {
        BearDogError::testing_with_category(message, TestingErrorCategory::Invariant)
    }

    /// Create a test monitoring error
    pub fn monitoring_error(message: &str) -> BearDogError {
        BearDogError::testing_with_category(message, TestingErrorCategory::Monitoring)
    }

    /// Create a coverage analysis error
    pub fn coverage_error(message: &str) -> BearDogError {
        BearDogError::testing_with_category(message, TestingErrorCategory::Coverage)
    }

    /// Create a boundary testing error
    pub fn boundary_error(message: &str) -> BearDogError {
        BearDogError::testing_with_category(message, TestingErrorCategory::Boundary)
    }

    /// Create a quantum security testing error
    pub fn quantum_error(message: &str) -> BearDogError {
        BearDogError::testing_with_category(message, TestingErrorCategory::Quantum)
    }

    /// Create an attack simulation error
    pub fn attack_error(message: &str) -> BearDogError {
        BearDogError::testing_with_category(message, TestingErrorCategory::Attack)
    }

    /// Create a readiness testing error
    pub fn readiness_error(message: &str) -> BearDogError {
        BearDogError::testing_with_category(message, TestingErrorCategory::Readiness)
    }

    /// Create a resistance testing error
    pub fn resistance_error(message: &str) -> BearDogError {
        BearDogError::testing_with_category(message, TestingErrorCategory::Resistance)
    }

    /// Create a simulation framework error
    pub fn simulation_error(message: &str) -> BearDogError {
        BearDogError::testing_with_category(message, TestingErrorCategory::Simulation)
    }

    /// Create a validation framework error
    pub fn validation_error(message: &str) -> BearDogError {
        BearDogError::testing_with_category(message, TestingErrorCategory::Validation)
    }
}

/// Type aliases for unified testing results
pub type PropertyResult = Result<bool, BearDogError>;
pub type MutationTestResult = Result<MutationScore, BearDogError>;
pub type InvariantValidationResult = Result<bool, BearDogError>;
pub type InvariantMonitoringResult = Result<MonitoringReport, BearDogError>;
pub type PathCoverageResult = Result<CoverageReport, BearDogError>;
pub type BoundaryTestResult = Result<BoundaryReport, BearDogError>;
pub type QuantumSecurityResult = Result<SecurityAssessment, BearDogError>;
pub type QuantumAttackResult = Result<AttackResistance, BearDogError>;
pub type QuantumReadinessScore = Result<ReadinessReport, BearDogError>;
pub type QuantumResistanceResult = Result<ResistanceReport, BearDogError>;
pub type QuantumAttackSimulation = Result<SimulationReport, BearDogError>;
pub type ErrorValidationResults = Result<Vec<ErrorValidation>, BearDogError>;

// Re-export the types that are still needed from the traits module
pub use super::traits::{
    MutationScore, MonitoringReport, CoverageReport, BoundaryReport,
    SecurityAssessment, AttackResistance, ReadinessReport, ResistanceReport,
    SimulationReport, ErrorValidation
}; 