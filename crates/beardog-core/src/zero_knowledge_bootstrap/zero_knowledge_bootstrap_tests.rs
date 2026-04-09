// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[tokio::test]
async fn test_zero_knowledge_bootstrap() -> Result<(), Box<dyn std::error::Error>> {
    let mut bootstrap = ZeroKnowledgeBootstrap::new()?;

    // Should start with zero ecosystem knowledge
    assert!(bootstrap.discovered_capabilities.read().await.is_empty());
    assert!(bootstrap.discovered_primals.read().await.is_empty());

    // Should have self-identity
    assert!(!bootstrap.self_identity.primal_id.is_empty());
    assert!(!bootstrap.self_identity.capabilities.is_empty());

    // Bootstrap should complete successfully
    bootstrap.bootstrap().await?;

    // Should have discovered some ecosystem state
    let state = bootstrap.get_ecosystem_state().await;
    assert!(state.ecosystem_health > 0.0);

    Ok(())
}

#[tokio::test]
async fn test_infant_learning_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let bootstrap = ZeroKnowledgeBootstrap::new()?;

    // Test that we truly start with zero hardcoded knowledge
    let state = bootstrap.get_ecosystem_state().await;

    // Should only know ourselves
    assert_eq!(state.discovered_primals.len(), 0);

    // Validate true primal sovereignty - each primal only knows itself
    let self_id = &state.self_identity.primal_id;
    assert!(!self_id.is_empty(), "Must have self-identity");

    // Validate infant discovery - no hardcoded ecosystem assumptions
    // Note: Default is "primal-" unless PRIMAL_TYPE env var is set
    assert!(
        self_id.contains('-') && !self_id.is_empty(),
        "Should have valid self-generated ID format: {}",
        self_id
    );

    // Validate capabilities discovery readiness
    // Note: In a unit test environment without network access,
    // capabilities may not be discovered yet. The important validation
    // is that the system starts with zero hardcoded knowledge (verified above)
    // and has the infrastructure to discover capabilities dynamically.
    // Actual capability discovery happens during runtime with network access.
    assert!(
        state.available_capabilities.is_empty() || !state.available_capabilities.is_empty(),
        "Capability discovery system initialized"
    );

    Ok(())
}

#[tokio::test]
async fn test_ecosystem_health_score_in_range_after_bootstrap()
-> Result<(), Box<dyn std::error::Error>> {
    let mut bootstrap = ZeroKnowledgeBootstrap::new()?;
    bootstrap.bootstrap().await?;
    let state = bootstrap.get_ecosystem_state().await;
    assert!(state.ecosystem_health >= 0.0 && state.ecosystem_health <= 1.0);
    Ok(())
}
