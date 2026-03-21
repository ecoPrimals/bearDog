// SPDX-License-Identifier: AGPL-3.0-only

// TRAIT IMPLEMENTATIONS: MultiCredentialHsmProvider

use super::Fido2MultiCredentialProvider;
use beardog_errors::BearDogError;
use beardog_traits::unified::{
    CredentialHierarchy, CredentialInfo, CredentialNode, CredentialRequest, HsmProtocol,
    MultiCredentialCapabilities, MultiCredentialHsmProvider,
};
use chrono::Utc;
use std::collections::HashMap;
use tracing::{info, warn};

impl MultiCredentialHsmProvider for Fido2MultiCredentialProvider {
    type Error = BearDogError;
    async fn create_credential(
        &self,
        request: CredentialRequest,
    ) -> Result<CredentialInfo, Self::Error> {
        info!("Creating credential with role: {}", request.role);

        // Validate algorithm
        let algorithm = request.algorithm.as_deref().unwrap_or("ES256");
        if !self
            .device_info
            .capabilities
            .supported_algorithms
            .contains(&algorithm.to_string())
        {
            return Err(BearDogError::system(format!(
                "Algorithm '{}' not supported by device. Supported: {:?}",
                algorithm, self.device_info.capabilities.supported_algorithms
            )));
        }

        // Validate permissions against parent (if hierarchical)
        if let Some(parent_id) = &request.parent_credential {
            let creds = self.credentials.read().await;
            if let Some(parent) = creds.get(parent_id) {
                // Ensure child permissions are subset of parent
                for perm in &request.permissions {
                    if !parent.permissions.contains(perm) {
                        warn!(
                            "Child credential requests permission '{}' not granted to parent",
                            perm
                        );
                    }
                }
            } else {
                return Err(BearDogError::system(format!(
                    "Parent credential '{parent_id}' not found"
                )));
            }
        }

        // Send CTAP2 MakeCredential command
        let (credential_id, public_key) = self.ctap2_make_credential(&request).await?;

        // Create credential info
        let credential_info = CredentialInfo {
            credential_id: Self::credential_id_to_string(&credential_id),
            role: request.role.clone(),
            display_name: request.display_name.clone(),
            permissions: request.permissions.clone(),
            public_key,
            algorithm: algorithm.to_string(),
            created_at: Utc::now(),
            last_used: None,
            use_count: 0,
            parent_credential_id: request.parent_credential.clone(),
            metadata: request.metadata.clone(),
            requires_user_presence: request.require_user_presence,
            requires_user_verification: request.require_user_verification,
        };

        // Store in cache
        let mut creds = self.credentials.write().await;
        creds.insert(
            credential_info.credential_id.clone(),
            credential_info.clone(),
        );

        info!("✅ Created credential: {}", credential_info.credential_id);
        Ok(credential_info)
    }

    async fn list_credentials(&self) -> Result<Vec<CredentialInfo>, Self::Error> {
        self.ctap2_enumerate_credentials().await
    }

    async fn delete_credential(&self, credential_id: &str) -> Result<(), Self::Error> {
        info!("Deleting credential: {}", credential_id);

        let cred_id_bytes = Self::string_to_credential_id(credential_id)?;
        self.ctap2_delete_credential(&cred_id_bytes).await?;

        // Remove from cache
        let mut creds = self.credentials.write().await;
        creds.remove(credential_id);

        info!("✅ Deleted credential: {}", credential_id);
        Ok(())
    }

    async fn get_credential_info(
        &self,
        credential_id: &str,
    ) -> Result<CredentialInfo, Self::Error> {
        let creds = self.credentials.read().await;
        creds
            .get(credential_id)
            .cloned()
            .ok_or_else(|| BearDogError::system(format!("Credential '{credential_id}' not found")))
    }

    async fn sign_with_credential(
        &self,
        credential_id: &str,
        data: &[u8],
        require_user_presence: bool,
    ) -> Result<Vec<u8>, Self::Error> {
        info!(
            "Signing {} bytes with credential: {}",
            data.len(),
            credential_id
        );

        let cred_id_bytes = Self::string_to_credential_id(credential_id)?;
        let signature = self
            .ctap2_get_assertion(&cred_id_bytes, data, require_user_presence)
            .await?;

        // Update use count
        let mut creds = self.credentials.write().await;
        if let Some(cred) = creds.get_mut(credential_id) {
            cred.last_used = Some(Utc::now());
            cred.use_count += 1;
        }

        Ok(signature)
    }

    async fn generate_hardware_entropy(&self, size: usize) -> Result<Vec<u8>, Self::Error> {
        let max_size = self.device_info.capabilities.max_entropy_size.unwrap_or(64);
        if size > max_size {
            return Err(BearDogError::system(format!(
                "Requested {size} bytes exceeds device maximum of {max_size} bytes"
            )));
        }

        self.ctap2_hmac_secret_entropy(size).await
    }

    async fn derive_child_credential(
        &self,
        parent_credential_id: &str,
        mut child_request: CredentialRequest,
    ) -> Result<CredentialInfo, Self::Error> {
        info!(
            "Deriving child credential from parent: {}",
            parent_credential_id
        );

        // Set parent relationship
        child_request.parent_credential = Some(parent_credential_id.to_string());

        // Create the child credential
        self.create_credential(child_request).await
    }

    async fn get_credential_hierarchy(&self) -> Result<CredentialHierarchy, Self::Error> {
        let creds = self.credentials.read().await;

        // Build hierarchy tree
        let mut roots = Vec::new();
        let mut children_map: HashMap<String, Vec<CredentialInfo>> = HashMap::new();

        // Organize credentials by parent
        for cred in creds.values() {
            if let Some(parent_id) = &cred.parent_credential_id {
                children_map
                    .entry(parent_id.clone())
                    .or_default()
                    .push(cred.clone());
            }
        }

        // Build tree starting from roots
        fn build_node(
            cred: &CredentialInfo,
            children_map: &HashMap<String, Vec<CredentialInfo>>,
        ) -> CredentialNode {
            let children = children_map
                .get(&cred.credential_id)
                .map(|kids| {
                    kids.iter()
                        .map(|kid| build_node(kid, children_map))
                        .collect()
                })
                .unwrap_or_default();

            CredentialNode {
                credential: cred.clone(),
                children,
            }
        }

        for cred in creds.values() {
            if cred.parent_credential_id.is_none() {
                roots.push(build_node(cred, &children_map));
            }
        }

        Ok(CredentialHierarchy { roots })
    }

    fn get_multi_credential_capabilities(&self) -> MultiCredentialCapabilities {
        MultiCredentialCapabilities {
            max_credentials: self.device_info.capabilities.max_resident_keys,
            current_credentials: 0, // PHASE-2(CTAP2): Query actual count from device via getInfo
            supports_hierarchical_credentials: true,
            supports_deterministic_derivation: self.device_info.capabilities.hmac_secret,
            supports_hardware_entropy: self.device_info.capabilities.hmac_secret,
            max_entropy_bytes: self.device_info.capabilities.max_entropy_size,
            supported_algorithms: self.device_info.capabilities.supported_algorithms.clone(),
            supports_user_presence: true,
            supports_user_verification: self.device_info.capabilities.user_verification,
            supports_metadata: false, // FIDO2 has limited metadata support
            protocol: HsmProtocol::Fido2,
        }
    }

    async fn prepare_credential_replication(
        &self,
        credential_id: &str,
        shared_entropy: &[u8],
    ) -> Result<beardog_traits::unified::CredentialReplicationData, Self::Error> {
        info!(
            "Preparing credential {} for replication using {} bytes of shared entropy",
            credential_id,
            shared_entropy.len()
        );

        // Get source credential info
        let source_cred = self.get_credential_info(credential_id).await?;

        // Create entropy hash for verification
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(shared_entropy);
        let entropy_hash = hasher.finalize().to_vec();

        // Create derivation path (for deterministic key derivation)
        // Use HMAC of role + timestamp as path
        let derivation_path = vec![0, 1, 2]; // PHASE-2: Implement proper BIP32-style derivation

        // Create credential request for target device
        let target_request = CredentialRequest {
            role: source_cred.role.clone(),
            display_name: source_cred.display_name.clone(),
            permissions: source_cred.permissions.clone(),
            require_user_presence: source_cred.requires_user_presence,
            require_user_verification: source_cred.requires_user_verification,
            parent_credential: None, // Don't replicate parent relationship
            metadata: source_cred.metadata.clone(),
            algorithm: Some(source_cred.algorithm.clone()),
        };

        info!("✅ Prepared replication data for credential");
        Ok(beardog_traits::unified::CredentialReplicationData {
            source_credential: source_cred,
            derivation_path,
            entropy_hash,
            target_request,
        })
    }
}
