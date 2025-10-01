// Unified Identity Trait System
//
// This module provides identity-related traits for the BearDog ecosystem,
// including unique identification, versioning, and metadata management.

// Re-export core identity traits
pub use super::core::{Identifiable, Versionable, Serializable};

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Extended identity trait with additional metadata capabilities
pub trait ExtendedIdentity: Identifiable {
    /// Get creation timestamp
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        None
    }

    /// Get last updated timestamp
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        None
    }

    /// Get tags associated with this entity
    fn tags(&self) -> Vec<String> {
        Vec::new()
    }

    /// Get extended metadata as structured data
    fn extended_metadata(&self) -> HashMap<String, serde_json::Value> {
        HashMap::new()
    }
}

/// Identity with lineage tracking for genetic and evolutionary systems
pub trait IdentityWithLineage: Identifiable {
    /// Get parent identifiers (for entities with inheritance)
    fn parent_ids(&self) -> Vec<String> {
        Vec::new()
    }

    /// Get generation number (for evolutionary systems)
    fn generation(&self) -> u32 {
        0
    }

    /// Get lineage hash for verification
    fn lineage_hash(&self) -> Option<String> {
        None
    }
}

/// Identity information structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdentityInfo {
    /// Unique identifier
    pub id: String,
    
    /// Entity type
    pub entity_type: String,
    
    /// Human-readable name
    pub name: String,
    
    /// Version string
    pub version: String,
    
    /// Creation timestamp
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Last update timestamp
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Associated tags
    pub tags: Vec<String>,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Default for IdentityInfo {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            entity_type: "unknown".to_string(),
            name: "unnamed".to_string(),
            version: "0.1.0".to_string(),
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

/// Identity validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityValidation {
    /// Whether the identity is valid
    pub valid: bool,
    
    /// Validation errors
    pub errors: Vec<String>,
    
    /// Validation warnings
    pub warnings: Vec<String>,
}

/// Identity validator trait
pub trait IdentityValidator: Send + Sync {
    /// Validate an identity
    fn validate(
        &self,
        identity_info: &IdentityInfo,
    ) -> impl std::future::Future<Output = Result<IdentityValidation, BearDogError>> + Send;
    
    /// Validate identity uniqueness
    fn validate_uniqueness(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_info_default() {
        let info = IdentityInfo::default();
        assert!(!info.id.is_empty());
        assert_eq!(info.entity_type, "unknown");
        assert!(info.created_at.is_some());
    }

    #[test]
    fn test_identity_validation() {
        let validation = IdentityValidation {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };
        assert!(validation.valid);
        assert!(validation.errors.is_empty());
    }
}
