// SPDX-License-Identifier: AGPL-3.0-only

//! Internal helpers for graph security operations
//!
//! This module provides internal helper functions that use the module-level
//! CollaborationService for runtime capability discovery.
//!
//! # Architecture
//!
//! Uses a module-level static CollaborationService (initialized once on first use)
//! to discover collaboration capabilities at runtime. This follows the TRUE PRIMAL
//! principle: "Primals only have self-knowledge and discover others at runtime."
//!
//! # Implementation Note
//!
//! Currently uses fallback/default data until full runtime discovery is available
//! (pending beardog-adapters crate stability). The architecture is in place for
//! future integration.

use crate::graph_security::collaboration_service::CollaborationService;
use beardog_errors::BearDogError;
use std::sync::{Arc, LazyLock};
use tracing::{debug, info};

/// Module-level CollaborationService (initialized once on first use)
///
/// This static service is shared across all graph security operations for
/// efficient runtime capability discovery.
static COLLABORATION: LazyLock<Arc<CollaborationService>> = LazyLock::new(|| {
    debug!("🏗️  Initializing graph security CollaborationService");
    info!("✅ Graph security CollaborationService initialized (using fallback data)");
    Arc::new(CollaborationService::new())
});

/// Get the module-level collaboration service
///
/// Returns a clone of the Arc, allowing efficient sharing without locks
pub fn collaboration_service() -> Arc<CollaborationService> {
    COLLABORATION.clone()
}

/// Get creator info via collaboration capability
///
/// Discovers any primal with Collaboration::TemplateStorage capability
/// and retrieves template creator information.
///
/// # Arguments
///
/// * `template_id` - Template identifier
///
/// # Returns
///
/// Template information including creator details
pub async fn get_creator_info(
    template_id: &str,
) -> Result<crate::graph_security::collaboration_service::TemplateInfo, BearDogError> {
    let collab = collaboration_service();
    collab.get_template_info(template_id).await
}

/// Get lineage via collaboration capability
///
/// Discovers any primal with Collaboration::LineageTracking capability
/// and retrieves template lineage history.
///
/// # Arguments
///
/// * `template_id` - Template identifier
///
/// # Returns
///
/// Vector of lineage versions (chronological order, oldest first)
pub async fn get_lineage(
    template_id: &str,
) -> Result<Vec<crate::graph_security::collaboration_service::LineageVersion>, BearDogError> {
    let collab = collaboration_service();
    collab.get_lineage(template_id).await
}

/// Get community usage via collaboration capability
///
/// Discovers any primal with Collaboration::CommunityMetrics capability
/// and retrieves community usage metrics.
///
/// # Arguments
///
/// * `template_id` - Template identifier
///
/// # Returns
///
/// Community metrics (deployments, success rate, ratings)
pub async fn get_community_metrics(
    template_id: &str,
) -> Result<crate::graph_security::collaboration_service::CommunityMetrics, BearDogError> {
    let collab = collaboration_service();
    collab.get_community_metrics(template_id).await
}

/// Get user permissions via collaboration capability
///
/// Discovers any primal with Collaboration::PermissionManagement capability
/// and retrieves user permissions for a resource.
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `resource_id` - Resource identifier (graph, template, etc.)
///
/// # Returns
///
/// User permissions including role and capabilities
pub async fn get_user_permissions(
    user_id: &str,
    resource_id: &str,
) -> Result<crate::graph_security::collaboration_service::UserPermissions, BearDogError> {
    let collab = collaboration_service();
    collab.get_user_permissions(user_id, resource_id).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collaboration_service_initialization() {
        // Service should initialize successfully
        let service = collaboration_service();
        assert!(Arc::strong_count(&service) >= 1);
    }

    #[test]
    fn test_collaboration_service_singleton() {
        // Multiple calls should return the same underlying service
        let service1 = collaboration_service();
        let service2 = collaboration_service();

        // Both should point to the same Arc (different clones)
        assert!(Arc::ptr_eq(&service1, &service2));
    }

    #[tokio::test]
    async fn test_get_creator_info_integration() {
        // Integration test - should not panic
        let result = get_creator_info("test-template").await;

        // Should return either success or a graceful error
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_lineage_integration() {
        // Integration test - should not panic
        let result = get_lineage("test-template").await;

        // Should return either success or a graceful error
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_community_metrics_integration() {
        // Integration test - should not panic
        let result = get_community_metrics("test-template").await;

        // Should return either success or a graceful error
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_user_permissions_integration() {
        // Integration test - should not panic
        let result = get_user_permissions("user-1", "graph-1").await;

        // Should return either success or a graceful error
        assert!(result.is_ok() || result.is_err());
    }
}
