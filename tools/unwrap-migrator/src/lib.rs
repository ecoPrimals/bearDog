// BearDog Unwrap Migrator Library
//
// Pure Rust tool for migrating unwrap()/expect() to proper Result handling

pub mod refined_migrator;

pub use refined_migrator::{
    RefinedBearDogMigrator,
    MigratorConfig,
    SafetyLevel,
    AnalysisStats,
    MigrationResult,
    RefinedMigratorError,
    RefinedResult,
};
