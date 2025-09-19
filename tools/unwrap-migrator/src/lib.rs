use beardog_errors::BearDogError;

pub mod systematic_migrator;
pub mod enhanced_migrator;
pub mod refined_migrator;

pub use systematic_migrator::{SystematicUnwrapMigrator, MigratorResult};
pub use enhanced_migrator::{EnhancedUnwrapMigrator, EnhancedMigratorResult};
pub use refined_migrator::{RefinedBearDogMigrator, RefinedResult};

pub mod patterns {

    pub const BEARDOG_ERROR_PATTERN: &str = r#".map_err(|e| BearDogError::Internal { 
        message: format!("Operation failed: {:?}", e) 
    })?"#;

    pub const CONFIG_ERROR_PATTERN: &str = r#".map_err(|e| BearDogError::Configuration { 
        message: format!("Configuration error: {}", e) 
    })?"#;

    pub const NETWORK_ERROR_PATTERN: &str = r#".map_err(|e| BearDogError::Network { 
        message: format!("Network operation failed: {}", e) 
    })?"#;

    pub const VALIDATION_ERROR_PATTERN: &str = r#".map_err(|e| BearDogError::Validation { 
        message: format!("Validation failed: {}", e) 
    })?"#;
}

pub mod utils {
    use std::path::Path;

    pub fn is_test_file(path: &Path) -> bool {
        path.to_string_lossy().contains("test") ||
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with("test_") || n.ends_with("_test.rs"))
            .unwrap_or(false)
    }

    pub fn is_example_file(path: &Path) -> bool {
        path.to_string_lossy().contains("example") ||
        path.ancestors().any(|p| p.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n == "examples")
            .unwrap_or(false))
    }

    pub fn is_benchmark_file(path: &Path) -> bool {
        path.to_string_lossy().contains("bench") ||
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with("bench_") || n.contains("benchmark"))
            .unwrap_or(false)
    }
}
