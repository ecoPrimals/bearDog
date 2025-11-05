

use crate::BearDogSecurityError;
use std::collections::HashMap;
use tracing::{info, warn};
use beardog_errors::BearDogError;

#[derive(std::sync::Arc<tokio::sync::RwLock<HashMap<String, TrustRelationship>>>,
    config: TrustManagementConfig,
}

#[derive(Debug, Clone)]
    /// Number of trust_expiry_hours
    pub trust_expiry_hours: u64,
    /// Whether verification_required is enabled
    pub verification_required: bool,
    /// Number of max_trust_relationships
    pub max_trust_relationships: usize,
}

impl Default for TrustManagementConfig {
    fn default(TrustLevel::Limited,
            trust_expiry_hours: 24,
            verification_required: true,
            max_trust_relationships: 1000,
}

#[derive(Debug, Clone)]
    /// The trust level value
    pub trust_level: TrustLevel,
    /// The established at value
    pub established_at: chrono::DateTime<chrono::Utc>,
    /// Optional expires at
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Collection of verification proof
    pub verification_proof: Vec<u8>,
    /// The last verified value
    pub last_verified: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq)]

pub enum TrustLevel {
    /// State indicating untrusted
    Untrusted,
    /// State indicating limited
    Limited,
    /// State indicating trusted
    Trusted,
    /// State indicating highlytrusted
    HighlyTrusted,
    /// Represents absolute variant
    Absolute,
}

impl TrustManager {

/// New operation.
    /// Creates a new instance
    pub fn new(config: TrustManagementConfig) -> Self {
        Self {
            trust_relationships: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::with_capacity(&str,
        trust_level: TrustLevel,
        verification_proof: Vec<u8>,
    ) -> Result<(), BearDogSecurityError> {
        info!(
            "🤝 Establishing trust relationship with: {} ({:?})",
            entity_id, trust_level
        );

        let expires_at = if self.config.trust_expiry_hours > 0 {
            Some(
                chrono::Utc::now() + chrono::Duration::hours(self.config.trust_expiry_hours as i64),
            )
        } else {
            None
        };

        let relationship = TrustRelationship {
            entity_id: entity_id.to_string(),
            trust_level,
            established_at: chrono::Utc::now(),
            expires_at,
            verification_proof,
            last_verified: chrono::Utc::now({}", entity_id);
        Ok(())
    }

/// Verify Trust operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn verify_trust(&self, entity_id: &str) -> Result<TrustLevel, BearDogSecurityError> {
        let relationships = self.trust_relationships.read();

        if let Some(relationship) = relationships.get(entity_id) {

            if let Some(expires_at) = relationship.expires_at {
                if chrono::Utc::now({}", entity_id);
                    return Ok(TrustLevel::Untrusted);
            }

            info!(
                "✅ Trust verified for: {} ({:?})",
                entity_id, relationship.trust_level
            );
            Ok({}", entity_id);
            Ok(TrustLevel::Untrusted)
    }

/// Revoke Trust operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn revoke_trust(&self, entity_id: &str) -> Result<(), BearDogSecurityError> {
        info!("🚫 Revoking trust relationship with: {}", entity_id);

        let mut relationships = self.trust_relationships.write({}", entity_id);
        Ok(())
    }

/// List Trusted Entities operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn list_trusted_entities(&self) -> Result<Vec<String>, BearDogSecurityError> {
        let relationships = self.trust_relationships.read();
        let trusted_entities: Vec<String> = relationships
            .iter()
            .filter(|(_, rel)| rel.trust_level != TrustLevel::Untrusted)
            .filter(|(_, rel)| {

                if let Some(expires_at) = rel.expires_at {
                    chrono::Utc::now() <= expires_at
                } else {
                    true
                }
            })
            .map(|(entity_id, _)| entity_id.clone())
            .collect();

        Ok(trusted_entities)
}

#[cfg(test)]
mod tests {
    use super::*;}

    #[tokio::test]
    fn test_trust_manager() -> Result<(), beardog_errors::BearDogError> {
        let manager = TrustManager::new(TrustManagementConfig::default());
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: security
 // TEST_PRIORITY: normal

        use rand::RngCore;
        let mut proof = vec![0u8; 32]; // 256-bit proof
        rand::thread_rng().fill_bytes(&mut proof);
        manager
            .establish_trust("test-entity", TrustLevel::Trusted, proof)
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(
                    format!("Operation failed: {e:?}"))
            })?;

        let trust_level = manager.verify_trust("test-entity").map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(
                format!("Operation failed: {e:?}"))
        })?;

        assert_eq!(trust_level, TrustLevel::Trusted);
        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[tokio::test]
    fn test_trust_revocation() -> Result<(), beardog_errors::BearDogError> {
        let manager = TrustManager::new(TrustManagementConfig::default());

        let proof = vec![1, 2, 3, 4];
        manager
            .establish_trust("test-entity", TrustLevel::Trusted, proof)
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal(
                    format!("Operation failed: {e:?}"))
            })?;

        manager.revoke_trust("test-entity").map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(
                format!("Operation failed: {e:?}"))
        })?;

        let trust_level = manager.verify_trust("test-entity").map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(
                format!("Operation failed: {e:?}"))
        })?;

        assert_eq!(trust_level, TrustLevel::Untrusted);
        Ok(())
}
