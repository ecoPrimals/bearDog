// SPDX-License-Identifier: AGPL-3.0-or-later

//! Entropy hierarchy: classify sources, mix seeds, monitor health, and validate policy.

/// Runtime manager coordinating validation, mixing, and seed lifecycle.
pub mod engine;
/// Validates that entropy feeds are live human input rather than scripted or replayed data.
pub mod live_feed_validator;
/// Metrics, analytics, and cleanup for active entropy seeds.
pub mod monitoring;
/// Fusion and mixing of multiple entropy buffers into classified seeds.
pub mod sources;
/// Core types: [`EntropyClass`], seeds, identities, and configuration.
pub mod types;
/// Quality, age, biometric, and ownership checks on entropy material.
pub mod validation;

// Re-export all types
pub use engine::EntropyHierarchyManager;
pub use live_feed_validator::{LiveFeedConfig, LiveFeedValidationResult, LiveFeedValidator};
pub use monitoring::{
    EntropyAnalytics, EntropyHealthStatus, EntropyHierarchyStats, EntropyMonitor,
    EntropyMonitoringConfig, PerformanceMetrics,
};
pub use sources::{EntropyMixingEngine, EntropySourceManager};
pub use types::{
    BiometricHash, EntropyClass, EntropyHierarchyConfig, EntropySeed, FusionAlgorithm,
    HumanEntropySource, HumanEntropyType, HumanIdentity, MachineEntropySource, MachineSourceType,
    MixingStrategy, OwnershipProof, SeedMetadata, VerificationLevel,
};
pub use validation::EntropyValidator;

// Note: BearDogError, Deserialize, and Serialize imports removed as they are unused in this module
// They can be re-added when needed for future implementations

#[cfg(test)]
#[path = "entropy_hierarchy_tests.rs"]
mod tests;
