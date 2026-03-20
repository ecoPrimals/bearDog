// SPDX-License-Identifier: AGPL-3.0-only

//! Permission and authorization logic for graph operations
//!
//! This module implements RBAC (Role-Based Access Control) for graph modifications.

use crate::graph_security::types::{Graph, GraphModification, ModificationAction, UserId};
use beardog_errors::BearDogError;

/// User role in relation to a graph
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    /// Graph owner - full control
    Owner,
    /// Collaborator - limited modifications
    Collaborator,
    /// Viewer - read-only
    Viewer,
    /// Public - template access only
    Public,
}

/// Check if a user has permission to perform a modification
pub async fn check_permission(
    user_id: &UserId,
    graph: &Graph,
    modification: &GraphModification,
) -> Result<bool, BearDogError> {
    // Determine user role
    let role = determine_user_role(user_id, graph).await?;

    // Check if role allows the action
    Ok(is_action_allowed(role, &modification.action))
}

/// Determine the user's role for a graph
async fn determine_user_role(user_id: &UserId, graph: &Graph) -> Result<UserRole, BearDogError> {
    // Check if user is the owner
    if user_id == &graph.owner {
        return Ok(UserRole::Owner);
    }

    // Check collaborator status via collaboration capability (runtime discovery)
    match crate::graph_security::internal::get_user_permissions(user_id, &graph.id).await {
        Ok(permissions) => {
            // Map permissions role to UserRole
            match permissions.role.as_str() {
                "owner" => Ok(UserRole::Owner),
                "collaborator" | "editor" => Ok(UserRole::Collaborator),
                "viewer" | "reader" => Ok(UserRole::Viewer),
                _ => Ok(UserRole::Public),
            }
        }
        Err(e) => {
            // Fallback: If collaboration capability not available, default to Viewer
            tracing::warn!(
                "⚠️  Could not determine user role via collaboration capability: {}",
                e
            );
            Ok(UserRole::Viewer)
        }
    }
}

/// Check if a role allows a specific action
const fn is_action_allowed(role: UserRole, action: &ModificationAction) -> bool {
    match role {
        UserRole::Owner => true, // Owner can do everything
        UserRole::Collaborator => {
            // Collaborators can modify but not delete
            matches!(
                action,
                ModificationAction::AddNode
                    | ModificationAction::ModifyNode
                    | ModificationAction::AddEdge
            )
        }
        UserRole::Viewer | UserRole::Public => false, // No modifications allowed
    }
}

/// Verify graph ownership
pub fn verify_ownership(user_id: &UserId, graph: &Graph) -> bool {
    user_id == &graph.owner
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_graph(owner: &str) -> Graph {
        Graph {
            id: "test-graph".to_string(),
            owner: owner.to_string(),
            nodes: vec![],
            edges: vec![],
            metadata: Some(HashMap::new()),
        }
    }

    #[test]
    fn test_is_action_allowed_owner() {
        assert!(is_action_allowed(
            UserRole::Owner,
            &ModificationAction::AddNode
        ));
        assert!(is_action_allowed(
            UserRole::Owner,
            &ModificationAction::RemoveNode
        ));
        assert!(is_action_allowed(
            UserRole::Owner,
            &ModificationAction::ModifyNode
        ));
    }

    #[test]
    fn test_is_action_allowed_collaborator() {
        assert!(is_action_allowed(
            UserRole::Collaborator,
            &ModificationAction::AddNode
        ));
        assert!(is_action_allowed(
            UserRole::Collaborator,
            &ModificationAction::ModifyNode
        ));
        assert!(!is_action_allowed(
            UserRole::Collaborator,
            &ModificationAction::RemoveNode
        ));
    }

    #[test]
    fn test_is_action_allowed_viewer() {
        assert!(!is_action_allowed(
            UserRole::Viewer,
            &ModificationAction::AddNode
        ));
        assert!(!is_action_allowed(
            UserRole::Viewer,
            &ModificationAction::RemoveNode
        ));
    }

    #[test]
    fn test_verify_ownership() {
        let graph = create_test_graph("alice");
        assert!(verify_ownership(&"alice".to_string(), &graph));
        assert!(!verify_ownership(&"bob".to_string(), &graph));
    }
}
