//! Commercial Extraction Detection Module
//!
//! Revolutionary system for distinguishing human users from commercial extraction attempts.
//! Implements the core principle: "Open gates for humans, locked tight for commercial extraction"

pub mod detector;
pub mod implementation;

// Re-export specific types to avoid UniversalRequest ambiguity
pub use detector::{
    AccessLevel, CommercialClassification, CommercialExtractionDetector, EntropyHistory,
    ExtractionRisk, GeneticKeyEvolutionEngine, HumanEntropyUsage, UsagePattern,
};
// Note: UniversalRequest is intentionally not re-exported to avoid ambiguity
