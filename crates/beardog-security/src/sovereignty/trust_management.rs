//! # Trust Management
//!
//! This module provides trust relationship management for the BearDog ecosystem.

use crate::BearDogSecurityError;
use beardog_errors::BearDogError;
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

// ============================================================
// Configuration
// ============================================================

/// Trust management configuration
#[derive(Debug, Clone)]
pub struct TrustManagementConfig {
    /// Default trust level for new relationships
    pub default_trust_level: TrustLevel,

    /// Trust expiry in hours (0 = no expiry)
    pub trust_expiry_hours: u64,

    /// Whether verification is required
    pub verification_required: bool,

    /// Maximum trust relationships
    pub max_trust_relationships: usize,
}

impl Default for TrustManagementConfig {
    fn default() -> Self {
        Self {
            default_trust_level: TrustLevel::Limited,
            trust_expiry_hours: 24,
            verification_required: true,
            max_trust_relationships: 1000,
        }
    }
}

// ============================================================
// Trust Types
// ============================================================

/// Trust level for entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TrustLevel {
    /// Untrusted entity
    Untrusted,

    /// Limited trust
    Limited,

    /// Trusted entity
    Trusted,

    /// Highly trusted entity
    HighlyTrusted,

    /// Absolute trust (e.g., self)
    Absolute,
}

impl Default for TrustLevel {
    fn default() -> Self {
        Self::Limited
    }
}

/// Trust relationship with an entity
#[derive(Debug, Clone)]
pub struct TrustRelationship {
    /// Entity identifier
    pub entity_id: String,

    /// Trust level
    pub trust_level: TrustLevel,

    /// When the relationship was established
    pub established_at: DateTime<Utc>,

    /// When the trust expires (if applicable)
    pub expires_at: Option<DateTime<Utc>>,

    /// Cryptographic proof of trust
    pub verification_proof: Vec<u8>,

    /// When trust was last verified
    pub last_verified: DateTime<Utc>,
}

// ============================================================
// Trust Manager
// ============================================================

/// Trust manager service
#[derive(Debug)]
pub struct TrustManager {
    /// Trust relationships by entity ID
    trust_relationships: Arc<RwLock<HashMap<String, TrustRelationship>>>,

    /// Configuration
    config: TrustManagementConfig,
}

impl TrustManager {
    /// Create a new trust manager
    pub fn new(config: TrustManagementConfig) -> Self {
        Self {
            trust_relationships: Arc::new(RwLock::new(HashMap::with_capacity(
                config.max_trust_relationships,
            ))),
            config,
        }
    }

    /// Establish a trust relationship
    ///
    /// # Arguments
    /// * `entity_id` - Entity to establish trust with
    /// * `trust_level` - Level of trust
    /// * `verification_proof` - Cryptographic proof
    pub async fn establish_trust(
        &self,
        entity_id: &str,
        trust_level: TrustLevel,
        verification_proof: Vec<u8>,
    ) -> Result<(), BearDogSecurityError> {
        info!(
            "🤝 Establishing trust relationship with: {} ({:?})",
            entity_id, trust_level
        );

        // Check relationship limit
        let relationships = self.trust_relationships.read().await;
        if relationships.len() >= self.config.max_trust_relationships {
            return Err(BearDogSecurityError::ResourceExhausted(
                "Maximum trust relationships reached".to_string(),
            ));
        }
        drop(relationships);

        // Calculate expiry
        let expires_at = if self.config.trust_expiry_hours > 0 {
            Some(Utc::now() + Duration::hours(self.config.trust_expiry_hours as i64))
        } else {
            None
        };

        let relationship = TrustRelationship {
            entity_id: entity_id.to_string(),
            trust_level,
            established_at: Utc::now(),
            expires_at,
            verification_proof,
            last_verified: Utc::now(),
        };

        let mut relationships = self.trust_relationships.write().await;
        relationships.insert(entity_id.to_string(), relationship);

        info!("✅ Trust established with: {}", entity_id);
        Ok(())
    }

    /// Verify trust level for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Entity to verify
    pub async fn verify_trust(&self, entity_id: &str) -> Result<TrustLevel, BearDogSecurityError> {
        let relationships = self.trust_relationships.read().await;

        if let Some(relationship) = relationships.get(entity_id) {
            // Check if trust has expired
            if let Some(expires_at) = relationship.expires_at {
                if Utc::now() > expires_at {
                    warn!("⏰ Trust expired for: {}", entity_id);
                    return Ok(TrustLevel::Untrusted);
                }
            }

            info!(
                "✅ Trust verified for: {} ({:?})",
                entity_id, relationship.trust_level
            );
            Ok(relationship.trust_level)
        } else {
            warn!("❓ No trust relationship found for: {}", entity_id);
            Ok(TrustLevel::Untrusted)
        }
    }

    /// Revoke trust for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Entity to revoke trust for
    pub async fn revoke_trust(&self, entity_id: &str) -> Result<(), BearDogSecurityError> {
        info!("🚫 Revoking trust relationship with: {}", entity_id);

        let mut relationships = self.trust_relationships.write().await;
        relationships.remove(entity_id);

        info!("✅ Trust revoked for: {}", entity_id);
        Ok(())
    }

    /// List all trusted entities
    pub async fn list_trusted_entities(&self) -> Result<Vec<String>, BearDogSecurityError> {
        let relationships = self.trust_relationships.read().await;
        let now = Utc::now();

        let trusted_entities: Vec<String> = relationships
            .iter()
            .filter(|(_, rel)| rel.trust_level != TrustLevel::Untrusted)
            .filter(|(_, rel)| {
                // Filter out expired relationships
                if let Some(expires_at) = rel.expires_at {
                    now <= expires_at
                } else {
                    true
                }
            })
            .map(|(entity_id, _)| entity_id.clone())
            .collect();

        Ok(trusted_entities)
    }

    /// Get trust relationship details
    pub async fn get_relationship(
        &self,
        entity_id: &str,
    ) -> Result<Option<TrustRelationship>, BearDogSecurityError> {
        let relationships = self.trust_relationships.read().await;
        Ok(relationships.get(entity_id).cloned())
    }

    /// Update trust level for an entity
    pub async fn update_trust_level(
        &self,
        entity_id: &str,
        new_level: TrustLevel,
    ) -> Result<(), BearDogSecurityError> {
        let mut relationships = self.trust_relationships.write().await;

        if let Some(relationship) = relationships.get_mut(entity_id) {
            info!(
                "📝 Updating trust level for {}: {:?} -> {:?}",
                entity_id, relationship.trust_level, new_level
            );
            relationship.trust_level = new_level;
            relationship.last_verified = Utc::now();
            Ok(())
        } else {
            Err(BearDogSecurityError::NotFound(format!(
                "Trust relationship not found: {}",
                entity_id
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_trust_manager() -> Result<(), BearDogError> {
        let manager = TrustManager::new(TrustManagementConfig::default());

        use rand::RngCore;
        let mut proof = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut proof);

        manager
            .establish_trust("test-entity", TrustLevel::Trusted, proof)
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        let trust_level = manager
            .verify_trust("test-entity")
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        assert_eq!(trust_level, TrustLevel::Trusted);
        Ok(())
    }

    #[tokio::test]
    async fn test_trust_revocation() -> Result<(), BearDogError> {
        let manager = TrustManager::new(TrustManagementConfig::default());

        let proof = vec![1, 2, 3, 4];
        manager
            .establish_trust("test-entity", TrustLevel::Trusted, proof)
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        manager
            .revoke_trust("test-entity")
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        let trust_level = manager
            .verify_trust("test-entity")
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        assert_eq!(trust_level, TrustLevel::Untrusted);
        Ok(())
    }

    #[tokio::test]
    async fn test_list_trusted_entities() -> Result<(), BearDogError> {
        let manager = TrustManager::new(TrustManagementConfig::default());

        manager
            .establish_trust("entity1", TrustLevel::Trusted, vec![])
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        manager
            .establish_trust("entity2", TrustLevel::HighlyTrusted, vec![])
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        let entities = manager
            .list_trusted_entities()
            .await
            .map_err(|e| BearDogError::internal(format!("Operation failed: {e:?}")))?;

        assert_eq!(entities.len(), 2);
        Ok(())
    }

    #[test]
    fn test_trust_level_ordering() {
        assert!(TrustLevel::Absolute > TrustLevel::HighlyTrusted);
        assert!(TrustLevel::HighlyTrusted > TrustLevel::Trusted);
        assert!(TrustLevel::Trusted > TrustLevel::Limited);
        assert!(TrustLevel::Limited > TrustLevel::Untrusted);
    }

    #[test]
    fn test_config_default() {
        let config = TrustManagementConfig::default();
        assert_eq!(config.default_trust_level, TrustLevel::Limited);
        assert_eq!(config.trust_expiry_hours, 24);
        assert!(config.verification_required);
    }
}
