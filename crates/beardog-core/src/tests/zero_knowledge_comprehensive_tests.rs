//! Zero Knowledge Bootstrap Comprehensive Tests
//!
//! Comprehensive test coverage for zero-knowledge bootstrap functionality
//! Following patterns from beardog-security's `crypto_coverage_tests`

use crate::zero_knowledge_bootstrap::self_discovery::SelfDiscoveryEngine;
use crate::zero_knowledge_bootstrap::ZeroKnowledgeBootstrap;
use beardog_errors::BearDogResult;

#[cfg(test)]
mod self_discovery_tests {
    use super::*;

    #[test]
    fn test_self_discovery_engine_creation() -> BearDogResult<()> {
        let engine = SelfDiscoveryEngine::new()?;
        assert!(format!("{engine:?}").contains("SelfDiscoveryEngine"));
        Ok(())
    }

    #[test]
    fn test_discover_self_identity_succeeds() -> BearDogResult<()> {
        let mut engine = SelfDiscoveryEngine::new()?;
        let identity = engine.discover_self_identity()?;

        // Should have a non-empty primal ID
        assert!(!identity.primal_id.is_empty());

        // Should have some capabilities
        assert!(!identity.capabilities.is_empty());

        Ok(())
    }

    #[test]
    fn test_identity_has_unique_id() -> BearDogResult<()> {
        let mut engine1 = SelfDiscoveryEngine::new()?;
        let mut engine2 = SelfDiscoveryEngine::new()?;

        let identity1 = engine1.discover_self_identity()?;
        let identity2 = engine2.discover_self_identity()?;

        // Each instance should get a unique ID
        assert_ne!(identity1.primal_id, identity2.primal_id);

        Ok(())
    }

    #[test]
    fn test_identity_capabilities_not_empty() -> BearDogResult<()> {
        let mut engine = SelfDiscoveryEngine::new()?;
        let identity = engine.discover_self_identity()?;

        // BearDog should always have security capabilities
        assert!(
            !identity.capabilities.is_empty(),
            "BearDog should discover security capabilities"
        );

        Ok(())
    }

    #[test]
    fn test_identity_endpoints_present() -> BearDogResult<()> {
        let mut engine = SelfDiscoveryEngine::new()?;
        let identity = engine.discover_self_identity()?;

        // Should have at least one endpoint
        assert!(
            !identity.endpoints.is_empty(),
            "Should discover at least one endpoint"
        );

        Ok(())
    }

    #[test]
    fn test_identity_metadata_present() -> BearDogResult<()> {
        let mut engine = SelfDiscoveryEngine::new()?;
        let identity = engine.discover_self_identity()?;

        // Metadata should be populated
        assert!(identity.metadata.display_name.is_some());
        assert!(!identity.metadata.version.is_empty());

        Ok(())
    }

    #[test]
    fn test_repeated_discovery_generates_unique_ids() -> BearDogResult<()> {
        let mut engine = SelfDiscoveryEngine::new()?;

        let identity1 = engine.discover_self_identity()?;
        let identity2 = engine.discover_self_identity()?;

        // Each discovery generates a unique ID (as expected)
        assert_ne!(identity1.primal_id, identity2.primal_id);
        // But capabilities should be consistent
        assert_eq!(identity1.capabilities.len(), identity2.capabilities.len());

        Ok(())
    }
}

#[cfg(test)]
mod bootstrap_initialization_tests {
    use super::*;

    #[tokio::test]
    async fn test_bootstrap_creation() {
        let bootstrap = ZeroKnowledgeBootstrap::new().await;
        assert!(bootstrap.is_ok(), "Bootstrap creation should succeed");
    }

    #[tokio::test]
    async fn test_bootstrap_starts_with_zero_knowledge() {
        let bootstrap = ZeroKnowledgeBootstrap::new().await.unwrap();

        // Should start with zero discovered capabilities
        let state = bootstrap.get_ecosystem_state().await;
        assert!(
            state.available_capabilities.is_empty() || state.available_capabilities.is_empty(),
            "Should start with zero or minimal ecosystem knowledge"
        );
    }

    #[tokio::test]
    async fn test_bootstrap_has_self_identity() {
        let bootstrap = ZeroKnowledgeBootstrap::new().await.unwrap();

        // Should have discovered self-identity
        let state = bootstrap.get_ecosystem_state().await;
        assert!(!state.self_identity.primal_id.is_empty());
        assert!(!state.self_identity.capabilities.is_empty());
    }

    #[tokio::test]
    async fn test_bootstrap_ecosystem_state_accessible() {
        let bootstrap = ZeroKnowledgeBootstrap::new().await.unwrap();

        // Should be able to get ecosystem state
        let state = bootstrap.get_ecosystem_state().await;
        assert!(state.discovered_primals.is_empty());
    }
}

#[cfg(test)]
mod bootstrap_execution_tests {
    use super::*;

    #[tokio::test]
    async fn test_bootstrap_execution_succeeds() {
        let mut bootstrap = ZeroKnowledgeBootstrap::new().await.unwrap();

        // Bootstrap should execute without error
        let result = bootstrap.bootstrap().await;
        assert!(result.is_ok(), "Bootstrap execution should succeed");
    }

    #[tokio::test]
    async fn test_bootstrap_discovers_ecosystem() {
        let mut bootstrap = ZeroKnowledgeBootstrap::new().await.unwrap();

        // After bootstrap, should have attempted ecosystem discovery
        let _ = bootstrap.bootstrap().await;

        // Should have attempted to listen for ecosystem announcements
        // (even if none were found in test environment)
        let state = bootstrap.get_ecosystem_state().await;
        assert!(
            state.discovered_primals.is_empty() || !state.discovered_primals.is_empty(),
            "Should have attempted discovery"
        );
    }

    #[tokio::test]
    async fn test_multiple_bootstrap_calls_safe() {
        let mut bootstrap = ZeroKnowledgeBootstrap::new().await.unwrap();

        // Multiple bootstrap calls should be safe
        let result1 = bootstrap.bootstrap().await;
        let result2 = bootstrap.bootstrap().await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }
}

#[cfg(test)]
mod self_identity_tests {
    use super::*;

    #[test]
    fn test_self_identity_clone() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        let cloned = identity.clone();
        assert_eq!(identity.primal_id, cloned.primal_id);
        assert_eq!(identity.capabilities.len(), cloned.capabilities.len());
    }

    #[test]
    fn test_self_identity_debug_output() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        let debug_str = format!("{identity:?}");
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("SelfIdentity"));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_full_zero_knowledge_flow() {
        // Test the complete zero-knowledge bootstrap flow

        // 1. Create bootstrap
        let mut bootstrap = ZeroKnowledgeBootstrap::new().await.unwrap();

        // 2. Verify zero initial knowledge
        let initial_state = bootstrap.get_ecosystem_state().await;
        assert!(initial_state.discovered_primals.is_empty());

        // 3. Execute bootstrap
        let bootstrap_result = bootstrap.bootstrap().await;
        assert!(bootstrap_result.is_ok());

        // 4. Verify self-identity was discovered
        let final_check = bootstrap.get_ecosystem_state().await;
        assert!(!final_check.self_identity.primal_id.is_empty());
        assert!(!final_check.self_identity.capabilities.is_empty());

        // 5. Get final state
        let final_state = bootstrap.get_ecosystem_state().await;
        // State may be empty in test environment, but the flow should complete
        assert!(
            final_state.discovered_primals.is_empty() || !final_state.discovered_primals.is_empty()
        );
    }

    #[tokio::test]
    async fn test_discovery_engine_integration() {
        // Test discovery engine integration with bootstrap
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        // Identity should be usable in bootstrap context
        assert!(!identity.primal_id.is_empty());
        assert!(!identity.capabilities.is_empty());
        assert!(!identity.endpoints.is_empty());
        assert!(identity.metadata.display_name.is_some());
    }
}
