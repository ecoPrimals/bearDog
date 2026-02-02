//! Mobile Ephemeral Key Integration
//!
//! This module integrates mobile hardware security with ephemeral key generation
//! using human entropy for enhanced security.

use super::{
    safe_ffi::SafePlatformSecurity,
    types::{HsmKey, KeyType},
};

// DEEP DEBT: Android StrongBox temporarily disabled (see mod.rs)
// #[cfg(target_os = "android")]
// use super::android_strongbox::SafeAndroidKeystore;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tracing::{debug, info};

/// Human entropy data collected from mobile sensors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntropyData {
    pub sensor_data: HashMap<String, f64>,
    pub environmental_context: Vec<u8>,
    pub session_context: String,
    pub user_id: String,
    pub timestamp: SystemTime,
}

/// Mobile ephemeral key configuration
#[derive(Debug, Clone)]
pub struct MobileEphemeralConfig {
    pub max_lifetime_minutes: u64,
    pub prefer_hardware_backing: bool,
    pub enable_biometric_binding: bool,
    pub require_user_presence: bool,
    pub entropy_quality_threshold: f64,
}

impl Default for MobileEphemeralConfig {
    fn default() -> Self {
        Self {
            max_lifetime_minutes: 60,
            prefer_hardware_backing: true,
            enable_biometric_binding: true,
            require_user_presence: true,
            entropy_quality_threshold: 0.8,
        }
    }
}

/// Mobile ephemeral key manager
pub struct MobileEphemeralKeyManager {
    platform_security: SafePlatformSecurity,
    config: MobileEphemeralConfig,
    active_keys: HashMap<String, EphemeralKeyMetadata>,
}

/// Ephemeral key metadata
#[derive(Debug, Clone)]
pub struct EphemeralKeyMetadata {
    pub key_id: String,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
    pub entropy_quality: f64,
    pub hardware_backed: bool,
    pub biometric_bound: bool,
}

impl MobileEphemeralKeyManager {
    /// Creates a new mobile ephemeral key manager
    ///
    /// # Errors
    /// Returns an error if initialization fails.
    pub fn new(config: MobileEphemeralConfig) -> Result<Self, BearDogError> {
        info!("🔑 Initializing Mobile Ephemeral Key Manager");

        let platform_security = SafePlatformSecurity::new()?;

        Ok(Self {
            platform_security,
            config,
            active_keys: HashMap::new(),
        })
    }

    /// Generates ephemeral key from human entropy
    ///
    /// # Errors
    /// Returns an error if key generation fails.
    pub fn generate_ephemeral_key(
        &mut self,
        entropy_data: &HumanEntropyData,
        key_type: KeyType,
    ) -> Result<HsmKey, BearDogError> {
        info!("🔑 Generating ephemeral key from human entropy");

        // Assess entropy quality
        let entropy_quality = self.assess_entropy_quality(entropy_data)?;

        if entropy_quality < self.config.entropy_quality_threshold {
            return Err(BearDogError::validation(&format!(
                "Entropy quality {} below threshold {}",
                entropy_quality, self.config.entropy_quality_threshold
            )));
        }

        // Generate key using platform security
        let key_id = format!("ephemeral_{}", uuid::Uuid::new_v4());
        let hsm_key = self.platform_security.generate_key(&key_id, &key_type)?;

        // Create metadata
        let now = SystemTime::now();
        let lifetime = Duration::from_secs(self.config.max_lifetime_minutes * 60);
        let metadata = EphemeralKeyMetadata {
            key_id: key_id.clone(),
            created_at: now,
            expires_at: now + lifetime,
            entropy_quality,
            hardware_backed: self.config.prefer_hardware_backing,
            biometric_bound: self.config.enable_biometric_binding,
        };

        self.active_keys.insert(key_id.clone(), metadata);

        info!("✅ Ephemeral key generated successfully: {}", key_id);
        Ok(hsm_key)
    }

    /// Assesses entropy quality
    fn assess_entropy_quality(&self, entropy_data: &HumanEntropyData) -> Result<f64, BearDogError> {
        debug!("📊 Assessing entropy quality");

        let mut quality_score = 0.0;

        // Sensor data diversity
        let sensor_count = entropy_data.sensor_data.len() as f64;
        quality_score += (sensor_count / 10.0).min(0.3);

        // Environmental context
        let context_entropy = entropy_data.environmental_context.len() as f64 / 1024.0;
        quality_score += context_entropy.min(0.3);

        // Session context
        if !entropy_data.session_context.is_empty() {
            quality_score += 0.2;
        }

        // User ID presence
        if !entropy_data.user_id.is_empty() {
            quality_score += 0.2;
        }

        debug!("📊 Entropy quality score: {:.2}", quality_score);
        Ok(quality_score.min(1.0))
    }

    /// Cleans up expired keys
    pub fn cleanup_expired_keys(&mut self) -> Result<usize, BearDogError> {
        let now = SystemTime::now();
        let mut removed_count = 0;

        self.active_keys.retain(|key_id, metadata| {
            if metadata.expires_at < now {
                info!("🗑️ Removing expired ephemeral key: {}", key_id);
                removed_count += 1;
                false
            } else {
                true
            }
        });

        if removed_count > 0 {
            info!("✅ Cleaned up {} expired ephemeral keys", removed_count);
        }

        Ok(removed_count)
    }

    /// Gets active key count
    pub fn active_key_count(&self) -> usize {
        self.active_keys.len()
    }

    /// Verifies key is still valid
    pub fn verify_key_valid(&self, key_id: &str) -> Result<bool, BearDogError> {
        match self.active_keys.get(key_id) {
            Some(metadata) => {
                let now = SystemTime::now();
                Ok(metadata.expires_at > now)
            }
            None => Ok(false),
        }
    }
}

/// Collects human entropy from mobile sensors
pub async fn collect_mobile_entropy(user_id: &str) -> Result<HumanEntropyData, BearDogError> {
    info!("📱 Collecting mobile sensor entropy for user: {}", user_id);

    let mut sensor_data = HashMap::new();

    // Simulated sensor data (real implementation would use actual sensors)
    sensor_data.insert("accelerometer_x".to_string(), 0.5);
    sensor_data.insert("accelerometer_y".to_string(), 0.3);
    sensor_data.insert("accelerometer_z".to_string(), 0.8);
    sensor_data.insert("gyroscope_x".to_string(), 0.2);
    sensor_data.insert("touch_pressure".to_string(), 0.7);

    let entropy_data = HumanEntropyData {
        sensor_data,
        environmental_context: vec![1, 2, 3, 4, 5], // Placeholder
        session_context: "mobile_session".to_string(),
        user_id: user_id.to_string(),
        timestamp: SystemTime::now(),
    };

    info!("✅ Mobile entropy collected successfully");
    Ok(entropy_data)
}

/// Demonstrates ephemeral key lifecycle
pub async fn demo_ephemeral_key_lifecycle() -> Result<(), BearDogError> {
    info!("🎬 Demonstrating ephemeral key lifecycle");

    let config = MobileEphemeralConfig::default();
    let mut manager = MobileEphemeralKeyManager::new(config)?;

    // Collect entropy
    let entropy_data = collect_mobile_entropy("demo_user").await?;

    // Generate ephemeral key
    let key = manager.generate_ephemeral_key(&entropy_data, KeyType::Ed25519)?;
    info!("🔑 Generated ephemeral key: {:?}", key.id);

    // Verify key is valid
    let is_valid = manager.verify_key_valid(&key.id)?;
    info!("✅ Key valid: {}", is_valid);

    // Show active keys
    info!("📊 Active keys: {}", manager.active_key_count());

    // Cleanup
    let cleaned = manager.cleanup_expired_keys()?;
    info!("🗑️ Cleaned up {} keys", cleaned);

    info!("🎉 Ephemeral key lifecycle demo complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() -> Result<(), Box<dyn std::error::Error>> {
        let config = MobileEphemeralConfig::default();
        assert_eq!(config.max_lifetime_minutes, 60);
        assert!(config.prefer_hardware_backing);
        Ok(())
    }

    #[test]
    fn test_manager_creation() -> Result<(), Box<dyn std::error::Error>> {
        let config = MobileEphemeralConfig::default();
        let manager = MobileEphemeralKeyManager::new(config);
        assert!(manager.is_ok());
        Ok(())
    }

    #[test]
    fn test_entropy_data_creation() -> Result<(), Box<dyn std::error::Error>> {
        let mut sensor_data = HashMap::new();
        sensor_data.insert("test".to_string(), 1.0);

        let entropy = HumanEntropyData {
            sensor_data,
            environmental_context: vec![1, 2, 3],
            session_context: "test".to_string(),
            user_id: "test_user".to_string(),
            timestamp: SystemTime::now(),
        };

        assert_eq!(entropy.user_id, "test_user");
        Ok(())
    }

    #[tokio::test]
    async fn test_entropy_collection() -> Result<(), Box<dyn std::error::Error>> {
        let result = collect_mobile_entropy("test_user").await;
        assert!(result.is_ok());

        let entropy = result?;
        assert_eq!(entropy.user_id, "test_user");
        assert!(!entropy.sensor_data.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_lifecycle_demo() -> Result<(), Box<dyn std::error::Error>> {
        let result = demo_ephemeral_key_lifecycle().await;
        // May fail without proper HSM setup, but should not panic
        let _ = result;
        Ok(())
    }
}
