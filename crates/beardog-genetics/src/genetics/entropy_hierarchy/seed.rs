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


/// Entropy Seed Operations
///
/// This module implements the EntropySeed struct with all its lifecycle methods
/// including creation, validation, ownership transfer, and operations.

use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use chrono::Utc;
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use uuid::Uuid;
use zeroize::Zeroize;
impl EntropySeed {
    /// Create a new human entropy seed with cryptographic proofs
    pub async fn new_human_entropy(
        entropy_class: EntropyClass,
        lifetime_policy: SeedLifetimePolicy,
        owner_identity: HumanIdentity,
        seed_bytes: Vec<u8>,
        entropy_manager: &crate::genetics::entropy_hierarchy::EntropyHierarchyManager,
    ) -> BearDogResult<Self> {
        let seed_id = Uuid::new_v4();
        // Generate ownership proof
        let ownership_proof = entropy_manager
            .generate_ownership_proof(&owner_identity, &seed_bytes)
            .await?;
        // Generate irreproducibility proof
        let irreproducibility_proof = entropy_manager
            .generate_irreproducibility_proof(&seed_bytes, &entropy_class)
        // Create usage policy based on entropy class
        let usage_policy = match &entropy_class {
            EntropyClass::HumanLivedExperience { .. } => SeedUsagePolicy {
                allowed_operations: vec![
                    "key_derivation".to_string(),
                    "signing".to_string(),
                    "encryption".to_string(),
                ],
                max_uses: None, // No limit for human entropy
                requires_approval: false,
            },
            _ => SeedUsagePolicy {
                allowed_operations: vec!["key_derivation".to_string()],
                max_uses: Some(100),
                requires_approval: true,
        };
        Ok(EntropySeed {
            id: seed_id,
            seed_bytes: SecretBytes::new(seed_bytes),
            entropy_class,
            generation_time: Utc::now(),
            lifetime_policy,
            ownership: SeedOwnership::HumanOwned {
                owner_identity,
                ownership_proof,
                transfer_count: 0,
            irreproducibility_proof,
            usage_policy,
            usage_history: Vec::new(),
            social_context: None,
        })
    }
    /// Create a new event-based seed for social contexts
    pub async fn new_event_seed(
        event_context: SocialContext,
        sharing_policy: SharingPolicy,
        // Generate biometric signature from event context (clone needed values first)
        let location_clone = event_context.location.clone();
        let timestamp = event_context.event_timestamp;
        let biometric_data = format!(
            "{}:{}:{}",
            timestamp.timestamp(),
            location_clone.unwrap_or_else(|| "Unknown".to_string()),
            0.5 // Default quality score during refactor
        );
        // Create cryptographic hash of biometric-like data
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(biometric_data.as_bytes());
        let biometric_hash = hasher.finalize().to_vec();
        // Create entropy class for event seed
        let entropy_class = EntropyClass::HumanLivedExperience {
            source_type: HumanEntropySource::MultiModalHuman {
                sources: vec![], // Would be populated with actual sources
                fusion_algorithm: FusionAlgorithm::CryptographicMixing,
                confidence_score: 0.9,
            capture_timestamp: event_context.event_timestamp,
            biometric_signature: BiometricHash(biometric_hash),
            ownership_proof: ownership_proof.clone(),
        // Create event-based lifetime policy
        // Clone event_context before using it
        let event_context_clone = event_context.clone();
        let lifetime_policy = SeedLifetimePolicy::EventBased {
            event_id: format!("event_{seed_id}"),
            event_type: event_context.event_type.clone(),
            sharing_policy: sharing_policy.clone(),
            event_expiration: sharing_policy.sharing_expiration,
            ownership: SeedOwnership::SharedOwnership {
                primary_owner: owner_identity,
                shared_with: event_context.participants.clone(),
                sharing_terms: SharingTerms {
                    max_participants: sharing_policy.max_shares,
                    expiration: sharing_policy.sharing_expiration,
                    permissions: sharing_policy.allowed_operations.clone(),
                },
            usage_policy: SeedUsagePolicy {
                allowed_operations: sharing_policy.allowed_operations,
                max_uses: None,
                requires_approval: sharing_policy.require_permission,
            social_context: Some(event_context_clone), // Use cloned context
    /// Validate that the seed is still valid and usable
    pub fn is_valid(&self) -> bool {
        // Check expiration based on lifetime policy
        match &self.lifetime_policy {
            SeedLifetimePolicy::Ephemeral {
                expiration_time, ..
            } => Utc::now() < *expiration_time,
            SeedLifetimePolicy::Persistent {
                ownership_expiration,
                ..
            } => match ownership_expiration {
                None => true,
                Some(exp) => Utc::now() < *exp,
            SeedLifetimePolicy::EventBased {
                event_expiration, ..
            } => match event_expiration {
            SeedLifetimePolicy::SelfSovereign { .. } => true, // Self-sovereign seeds don't expire
        }
    /// Transfer ownership of the seed to a new owner
    pub fn transfer_ownership(&mut self, new_owner: HumanIdentity) -> BearDogResult<()> {
        // Check if transfer is allowed
                ownership_transfer_allowed,
            } => {
                if !ownership_transfer_allowed {
                    return Err(BearDogError::internal("Ownership transfer not allowed for this seed".to_string(),
                    ));
                }
            }
            SeedLifetimePolicy::SelfSovereign {
                transfer_permissions,
                if !transfer_permissions.transferable {
                        message: "This self-sovereign seed is not transferable".to_string(),
            _ => {
                return Err(BearDogError::internal("Ownership transfer not supported for this seed type".to_string(),
                ));
        // Record the transfer before moving new_owner
        let transfer_context = format!("Transferred to {}", new_owner.identity_id);
        // Update ownership
        match &mut self.ownership {
            SeedOwnership::HumanOwned { transfer_count, .. } => {
                *transfer_count += 1;
                self.ownership = SeedOwnership::HumanOwned {
                    owner_identity: new_owner, // Move happens here
                    ownership_proof: OwnershipProof {
                        signature: vec![0u8; 64], // Would be generated properly
                        timestamp: Utc::now(),
                        verification_key: vec![0u8; 32],
                    },
                    transfer_count: *transfer_count,
                };
                    message: "Cannot transfer ownership from non-human owner".to_string(),
        // Record usage event using the saved context
        self.usage_history.push(SeedUsageEvent {
            timestamp: Utc::now(),
            operation: "ownership_transfer".to_string(),
            context: transfer_context,  // Use the saved context
            result_hash: vec![0u8; 32], // Would be hash of transfer
        });
        Ok(())
    /// Expire ownership and transition to machine ownership
    pub fn expire_ownership(&mut self) -> BearDogResult<()> {
        let previous_owner = match &self.ownership {
            SeedOwnership::HumanOwned { owner_identity, .. } => Some(owner_identity.clone()),
            SeedOwnership::SharedOwnership { primary_owner, .. } => Some(primary_owner.clone()),
            _ => None,
        self.ownership = SeedOwnership::MachineOwned {
            previous_owner,
            ownership_transition: OwnershipTransition::OwnershipExpired,
            self_sovereign: matches!(
                self.lifetime_policy,
                SeedLifetimePolicy::SelfSovereign { .. }
            ),
        // Record the expiration
            operation: "ownership_expiration".to_string(),
            context: "Ownership expired due to policy".to_string(),
            result_hash: vec![0u8; 32],
    /// Use the seed for a cryptographic operation
    pub fn use_for_operation(&mut self, operation: &str) -> BearDogResult<Vec<u8>> {
        // Check if the seed is valid
        if !self.is_valid() {
            return Err(BearDogError::internal("Seed has expired and cannot be used".to_string(),
            ));
        // Check if operation is allowed
        if !self
            .usage_policy
            .allowed_operations
            .contains(&operation.to_string())
        {
                message: format!("Operation '{operation}' not allowed for this seed"),
        // Check usage limits
        if let Some(max_uses) = self.usage_policy.max_uses {
            let operation_count = self
                .usage_history
                .iter()
                .filter(|event| event.operation == operation)
                .count() as u32;
            if operation_count >= max_uses {
                    message: format!(
                        "Maximum usage limit ({max_uses}) reached for operation '{operation}'"
                    ),
        // Perform the operation (simplified - would use actual crypto)
        let mut hasher = Sha3_256::new();
        hasher.update(self.seed_bytes.as_bytes());
        hasher.update(operation.as_bytes());
        hasher.update(Utc::now().timestamp().to_le_bytes());
        let result = hasher.finalize().to_vec();
        // Record the usage
        let result_hash = {
            let mut hash_hasher = Sha3_256::new();
            hash_hasher.update(&result);
            hash_hasher.finalize().to_vec()
            operation: operation.to_string(),
            context: "Cryptographic operation".to_string(),
            result_hash,
        Ok(result)
    /// Securely destroy the seed}


    pub fn destroy(&mut self) {
        self.seed_bytes.zeroize();
        self.irreproducibility_proof.zeroize();
        // Mark as destroyed in usage history
            operation: "destroy".to_string(),
            context: "Seed securely destroyed".to_string(),
    /// Get the entropy classification level}


    pub fn get_entropy_tier(&self) -> u8 {
        match &self.entropy_class {
            EntropyClass::HumanLivedExperience { .. } => 3,
            EntropyClass::HumanSupervisedMachine { .. } => 2,
            EntropyClass::StoreBoughtMachine { .. } => 1,
    /// Check if seed can be shared with others
    pub fn can_be_shared(&self) -> bool {
            SeedLifetimePolicy::EventBased { sharing_policy, .. } => {
                sharing_policy.max_shares.is_some()
            } => transfer_permissions.transferable,
            _ => false,
    /// Get current owner identity if human-owned
    pub fn get_current_owner(&self) -> Option<&HumanIdentity> {
        match &self.ownership {
            SeedOwnership::HumanOwned { owner_identity, .. } => Some(owner_identity),
            SeedOwnership::SharedOwnership { primary_owner, .. } => Some(primary_owner),
            SeedOwnership::CommunityOwned { .. } => None, // Community ownership
            SeedOwnership::MachineOwned { .. } => None,   // Machine ownership
    /// Get usage statistics for this seed
    pub fn get_usage_stats(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        for event in &self.usage_history {
            *stats.entry(event.operation.clone()).or_insert(0) += 1;
        stats
    /// Check if seed requires approval for operations}


    pub fn requires_approval(&self) -> bool {
        self.usage_policy.requires_approval
    /// Get remaining usage count for operations with limits
    pub fn get_remaining_uses(&self, operation: &str) -> Option<u32> {
        self.usage_policy.max_uses.map(|max_uses| {
            let used = self
            max_uses.saturating_sub(used)
}
