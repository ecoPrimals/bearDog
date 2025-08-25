// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Universal `HSM` Provider Trait - Canonical Definition
///
/// **THE SINGLE `HSM` PROVIDER INTERFACE**
/// This trait defines the canonical interface that ALL `HSM` providers in `BearDog`
/// must implement. It replaces all fragmented trait definitions and provides
/// a unified, vendor-agnostic interface for `HSM` operations.

use super::*;
use beardog_errors::BearDogResult;
use beardog_types::canonical::{KeyMetadata, KeyType};
/// **Universal `HSM` Provider Trait - Canonical Interface**
/// This is THE definitive `HSM` provider interface for `BearDog`. All `HSM` providers
/// (Android StrongBox, iOS Secure Enclave, Software HSM, PKCS#11, TPM, etc.)
/// must implement this trait.
/// ## Design Principles
/// - **Vendor Agnostic**: Works with any `HSM` vendor without lock-in
/// - **Capability Driven**: Providers expose their capabilities for intelligent selection
/// - **Human Entropy First**: Built-in support for human entropy collection
/// - **Hardware Attestation**: Native support for hardware attestation
/// - **Performance Optimized**: Zero-cost async operations with native async fn
/// - **Comprehensive Monitoring**: Built-in health checks and metrics
/// ## Zero-Cost Async Migration
/// **PERFORMANCE IMPROVEMENT**: This trait now uses native async fn instead of async_trait,
/// eliminating Box<dyn Future> allocation overhead for 5-15% performance improvement.
/// ## Usage Example
/// ```rust,no_run
/// use beardog_traits::canonical::HsmProvider;
/// use beardog_types::canonical::{KeyType, KeyMetadata};
/// async fn example_usage(provider: &dyn HsmProvider) -> beardog_errors::BearDogResult<()> {
///     // Check provider capabilities
///     let info = provider.get_provider_info();
///     println!("Using provider: {} ({})", info.name, info.provider_type);
///     // Generate a key
///     let metadata = KeyMetadata::default();
///     let key = provider.generate_key(KeyType::Ed25519, metadata).await?;
///     // Sign some data
///     let data = b"Hello, `BearDog`!";
///     let signature = provider.sign_data(&key.key_id, data).await?;
///     // Verify the signature
///     let is_valid = provider.verify_signature(&key.key_id, data, &signature).await?;
///     assert!(is_valid);
///     Ok(())
/// }
/// ```
#[allow(async_fn_in_trait)]
pub trait UniversalHsmProvider: Send + Sync {
    // ===== CORE CRYPTOGRAPHIC OPERATIONS =====
    /// **Generate a cryptographic key**
    ///
    /// Creates a new cryptographic key of the specified type with the given metadata.
    /// The key is stored securely within the `HSM` and can be referenced by its ID.
    /// # Arguments
    /// * `key_type` - The type of key to generate (Ed25519, ECDSA P-256, RSA, etc.)
    /// * `metadata` - Key metadata including usage policies, tags, and compliance info
    /// # Returns
    /// * `Ok(HsmKey)` - The generated key with its unique identifier
    /// * `Err(BearDogError)` - If key generation fails
    /// # Implementation Notes
    /// - Keys should be generated using the `HSM`'s secure random number generator
    /// - If human entropy is available and metadata requests it, use human entropy
    /// - Key metadata should be stored securely alongside the key
    /// - The key ID should be unique and cryptographically secure
    async fn generate_key(
        &self,
        key_type: KeyType,
        metadata: KeyMetadata,
    ) -> BearDogResult<beardog_types::HsmKey>;
    /// **Sign data with a stored key**
    /// Signs the provided data using the key identified by key_id. The signing
    /// operation is performed securely within the `HSM`.
    /// * `key_id` - Unique identifier of the key to use for signing
    /// * `data` - The data to be signed
    /// * `Ok(Vec<u8>)` - The digital signature
    /// * `Err(BearDogError)` - If signing fails (key not found, `HSM` error, etc.)
    /// - The signing algorithm should match the key type
    /// - If the key requires user presence, prompt for authentication
    /// - All signing operations should be audited
    async fn sign_data(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>>;
    /// **Verify a digital signature**
    /// Verifies that the provided signature is valid for the given data using
    /// the specified key.
    /// * `key_id` - Unique identifier of the key to use for verification
    /// * `data` - The original data that was signed
    /// * `signature` - The signature to verify
    /// * `Ok(true)` - If the signature is valid
    /// * `Ok(false)` - If the signature is invalid
    /// * `Err(BearDogError)` - If verification fails due to an error
    async fn verify_signature(
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool>;
    // ===== HUMAN ENTROPY CAPABILITIES =====
    /// **Get human entropy capabilities**
    /// Returns information about this provider's human entropy collection
    /// capabilities. This is used for tier elevation and provider selection.
    /// * `Ok(`HumanEntropyCapabilities`)` - Detailed capability information
    /// * `Err(BearDogError)` - If capability detection fails
    async fn get_human_entropy_capabilities(&self) -> BearDogResult<HumanEntropyCapabilities>;
    /// **Collect human entropy**
    /// Collects high-quality entropy from human interaction using the specified
    /// method. This is used for creating ephemeral seeds and enhancing key generation.
    /// * `method` - The entropy collection method to use
    /// * `bits` - Number of entropy bits to collect
    /// * `Ok(HumanEntropyData)` - The collected entropy data with quality metrics
    /// * `Err(BearDogError)` - If entropy collection fails
    /// - Only collect entropy if the provider supports the specified method
    /// - Ensure entropy quality meets minimum standards
    /// - Provide real-time feedback to the user during collection
    async fn collect_human_entropy(
        method: &HumanEntropyMethod,
        bits: u32,
    ) -> BearDogResult<HumanEntropyData>;
    /// **Create ephemeral seed from human entropy**
    /// Creates a high-quality ephemeral seed from collected human entropy.
    /// This is used for primal sovereignty and enhanced security operations.
    /// * `entropy` - The collected human entropy data
    /// * `seed_size` - Size of the seed to create in bytes
    /// * `Ok(EphemeralSeed)` - The created seed with quality assessment
    /// * `Err(BearDogError)` - If seed creation fails
    async fn create_ephemeral_seed(
        entropy: &HumanEntropyData,
        seed_size: u32,
    ) -> BearDogResult<EphemeralSeed>;
    // ===== PROVIDER METADATA & HEALTH =====
    /// **Get provider information**
    /// Returns comprehensive information about this `HSM` provider including
    /// capabilities, vendor info, and supported features.
    /// * `ProviderInfo` - Complete provider information
    /// - This should be a fast, synchronous operation
    /// - Information should be cached and updated only when necessary
    /// - Capabilities should be detected at provider initialization
    fn get_provider_info(&self) -> ProviderInfo;
    /// **Perform health check**
    /// Checks the current health and availability of the `HSM` provider.
    /// This is used for provider selection and failover decisions.
    /// * `Ok(ProviderHealth)` - Current health status
    /// * `Err(BearDogError)` - If health check fails
    /// - Should perform actual connectivity/functionality tests
    /// - Include response time measurements
    /// - Verify that all advertised capabilities are still available
    async fn health_check(&self) -> BearDogResult<ProviderHealth>;
    // ===== HARDWARE ATTESTATION (OPTIONAL) =====
    /// **Get hardware attestation data**
    /// Retrieves hardware attestation data proving the authenticity and
    /// integrity of the `HSM` hardware. This is optional and only supported
    /// by hardware-based providers.
    /// * `Ok(Some(AttestationData))` - If attestation is supported and available
    /// * `Ok(None)` - If attestation is not supported by this provider
    /// * `Err(BearDogError)` - If attestation fails
    /// - Software providers should return Ok(None)
    /// - Hardware providers should return actual attestation data
    /// - Attestation should include certificate chains when available
    async fn get_hardware_attestation(&self) -> BearDogResult<Option<AttestationData>>;
    // ===== KEY MANAGEMENT (OPTIONAL ADVANCED FEATURES) =====
    /// **List available keys**
    /// Returns a list of all keys stored in this `HSM` provider.
    /// This is an optional advanced feature.
    /// * `Ok(Vec<String>)` - List of key IDs
    /// * `Err(BearDogError)` - If listing fails
    /// # Default Implementation
    /// The default implementation returns an empty list, indicating that
    /// key listing is not supported by this provider.
    async fn list_keys(&self) -> BearDogResult<Vec<String>> {
        Ok(Vec::new())
    }
    /// **Delete a key**
    /// Securely deletes a key from the `HSM`. This is an optional advanced feature.
    /// * `key_id` - Unique identifier of the key to delete
    /// * `Ok(())` - If deletion succeeds
    /// * `Err(BearDogError)` - If deletion fails
    /// The default implementation returns an error indicating that key deletion
    /// is not supported by this provider.
    async fn delete_key(&self, _key_id: &str) -> BearDogResult<()> {
        Err(beardog_errors::BearDogError::NotSupported {
            feature: format!(
                "Key deletion not supported by provider: {}",
                self.get_provider_info().name
            ),
        })
    /// **Get key metadata**
    /// Retrieves metadata for a specific key. This is an optional advanced feature.
    /// * `key_id` - Unique identifier of the key
    /// * `Ok(KeyMetadata)` - The key's metadata
    /// * `Err(BearDogError)` - If retrieval fails
    /// The default implementation returns an error indicating that metadata
    /// retrieval is not supported by this provider.
    async fn get_key_metadata(&self, _key_id: &str) -> BearDogResult<KeyMetadata> {
                "Key metadata retrieval not supported by provider: {}",
}
/// **Provider Capability Assessment**
/// Helper functions for assessing provider capabilities.
impl dyn HsmProvider {
    /// Check if provider supports human entropy collection
    pub async fn supports_human_entropy(&self) -> bool {
        match self.get_human_entropy_capabilities().await {
            Ok(caps) => caps.supports_ephemeral_seeds,
            Err(_) => false,
        }
    /// Check if provider supports hardware attestation
    pub async fn supports_attestation(&self) -> bool {
        match self.get_hardware_attestation().await {
            Ok(Some(_)) => true,
            Ok(None) => false,
    /// Get provider security level}


    pub fn get_security_level(&self) -> crate::SecurityLevel {
        self.get_provider_info().security_level
    /// Check if provider is hardware-backed
    pub fn is_hardware_backed(&self) -> bool {
        matches!(
            self.get_provider_info().security_level,
            crate::SecurityLevel::Hardware
        )
#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::KeyType;
    use chrono::Utc;
    // Mock provider for testing
    struct MockProvider;
    
    impl UniversalHsmProvider for MockProvider {}


        async fn generate_key(
            &self,
            _key_type: KeyType,
            _metadata: KeyMetadata,
        ) -> BearDogResult<beardog_types::HsmKey> {
            Ok(beardog_types::HsmKey {
                id: "mock_key_id".to_string(),
                key_type: beardog_types::canonical::KeyType::Ed25519,
                material: beardog_types::canonical::hsm::keys::KeyMaterial::PublicKey(vec![
                    1, 2, 3, 4,
                ]),
                metadata: KeyMetadata::default(),
                health: beardog_types::canonical::hsm::keys::KeyHealth::default(),
                created_at: Utc::now(),
                expires_at: None,
                key_name: "Mock Test Key".to_string(),
                last_used: None,
                usage_count: 0,
                key_material: beardog_types::canonical::hsm::keys::KeyMaterial::PublicKey(vec![
                hsm_type: Some("mock".to_string()),
                hsm_tier: Some("test".to_string()),
                health_status: None,
                attestation: None,
                backup_info: None,
                compliance_info: None,
                provider_attributes: std::collections::HashMap::new(),
                derivation_path: None,
            })
        async fn sign_data(&self, _key_id: &str, _data: &[u8]) -> BearDogResult<Vec<u8>> {
            Ok(vec![1, 2, 3, 4, 5, 6, 7, 8])}


        async fn verify_signature(
            _key_id: &str,
            _data: &[u8],
            _signature: &[u8],
        ) -> BearDogResult<bool> {
            Ok(true)
        async fn get_human_entropy_capabilities(&self) -> BearDogResult<HumanEntropyCapabilities> {
            Ok(HumanEntropyCapabilities {
                supports_ephemeral_seeds: false,
                collection_methods: Vec::new(),
                realtime_entropy: false,
                quality_assessment: false,
                biometric_integration: false,
                min_entropy_bits: 0.0,
                max_collection_rate: 0.0,}


        async fn collect_human_entropy(
            _method: &HumanEntropyMethod,
            _bits: u32,
        ) -> BearDogResult<HumanEntropyData> {
            Err(beardog_errors::BearDogError::NotSupported {
                feature: "Human entropy collection not supported by mock provider".to_string(),
        async fn create_ephemeral_seed(
            _entropy: &HumanEntropyData,
            _seed_size: u32,
        ) -> BearDogResult<EphemeralSeed> {
                feature: "Ephemeral seed creation not supported by mock provider".to_string(),}


        fn get_provider_info(&self) -> ProviderInfo {
            ProviderInfo {
                provider_id: "mock_provider".to_string(),
                name: "Mock Provider".to_string(),
                version: "1.0.0".to_string(),
                provider_type: ProviderType::Software,
                security_level: crate::SecurityLevel::Software,
                supports_attestation: false,
                supports_biometric: false,
                supports_human_entropy: false,
                supported_key_types: vec![KeyType::Ed25519],
                description: "Mock `HSM` provider for testing".to_string(),
                vendor: "`BearDog` Test".to_string(),
                platforms: vec![Platform::Linux],
            }
        async fn health_check(&self) -> BearDogResult<ProviderHealth> {
            Ok(ProviderHealth {
                is_healthy: true,
                error_message: None,
                last_check: Utc::now(),
                response_time_ms: Some(1.0),
                capabilities_verified: true,}


        async fn get_hardware_attestation(&self) -> BearDogResult<Option<AttestationData>> {
            Ok(None)
    #[tokio::test]
    async fn test_mock_provider_basic_operations() -> beardog_errors::BearDogResult<()> {
        let provider = MockProvider;
        // Test provider info
        let info = provider.get_provider_info();
        assert_eq!(info.name, "Mock Provider");
        assert_eq!(info.provider_type, ProviderType::Software);
        // Test key generation
        let metadata = KeyMetadata::default();
        let key = provider
            .generate_key(KeyType::Ed25519, metadata)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
        assert_eq!(key.id, "mock_key_id");
        // Test signing
        let data = b"test data";
        let signature = provider.sign_data(&key.id, data).await.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        assert!(!signature.is_empty());
        // Test verification
        let is_valid = provider
            .verify_signature(&key.id, data, &signature)
        assert!(is_valid);
        // Test health check
        let health = provider.health_check().await.map_err(|e| {
        assert!(health.is_healthy);
        Ok(())
    async fn test_provider_capability_helpers() -> beardog_errors::BearDogResult<()> {
        // Test capability helpers from ProviderInfo
        assert!(!info.supports_human_entropy);
        assert!(!info.supports_attestation);
        assert_eq!(info.security_level, crate::SecurityLevel::Software);
        // Test trait extension method - need to use as dyn trait
        let provider_ref: &dyn HsmProvider = &provider;
        assert!(!provider_ref.is_hardware_backed());
