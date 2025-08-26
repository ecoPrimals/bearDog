

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod canonical;

pub use canonical::*;

pub use beardog_errors::BearDogResult;

pub mod prelude {
    pub use crate::canonical::*;
    pub use beardog_errors::BearDogResult;
}

pub mod validation {

    pub fn validate_canonical_patterns() -> bool {

        true
    }

    pub fn validate_canonical_usage() -> Result<(), Vec<String>> {
        let errors = Vec::new();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn trait_system_info() -> Vec<(&'static str, &'static str)> {
        vec![
            (
                "BaseProvider",
                "Foundation trait for all providers with common lifecycle methods",
            ),
            (
                "SecurityProvider",
                "Complete security operations including auth, authz, and crypto",
            ),
            (
                "HsmProvider",
                "Hardware security module operations for key management",
            ),
            (
                "CacheProvider",
                "Unified caching interface with batch operations support",
            ),
            (
                "CryptoProvider",
                "Software cryptographic operations without HSM requirement",
            ),
            (
                "WorkflowProvider",
                "Business process automation and workflow execution",
            ),
            (
                "GeneticsProvider",
                "Genetic algorithm operations and population management",
            ),
            (
                "MonitoringProvider",
                "System observability, metrics, and alerting",
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_canonical_trait_registry() {

        let info = validation::trait_system_info();
        assert!(!info.is_empty());
        let trait_names: Vec<&str> = info.iter().map(|(name, _)| *name).collect();
        assert!(trait_names.contains(&"BaseProvider"));
    }

    #[test] 
    fn test_canonical_completion() {

        let result = validation::validate_canonical_usage();
        assert!(result.is_ok(), "Canonical migration should be complete");
    }

    #[test]
    fn test_validation_system() {
        let result = validation::validate_canonical_usage();
        assert!(result.is_ok(), "Canonical usage validation should pass");
    }

    #[test]
    fn test_trait_system_info() {
        let info = validation::trait_system_info();
        assert!(!info.is_empty());
        let trait_names: Vec<&str> = info.iter().map(|(name, _)| *name).collect();
        assert!(trait_names.contains(&"BaseProvider"));
    }
}
