// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod detector;
pub mod implementation;

pub use detector::{
    AccessLevel, CommercialClassification, CommercialExtractionDetector, EntropyHistory,
    ExtractionRisk, GeneticKeyEvolutionEngine, HumanEntropyUsage, UsagePattern,
};
