// SPDX-License-Identifier: AGPL-3.0-or-later



use super::types::*;
use beardog_errors::BearDogError;
use chrono::Utc;
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use uuid::Uuid;
use zeroize::Zeroize;
impl EntropySeed {

/// New Human Entropy operation.
    /// Creates a new instance
    pub fn new_human_entropy(EntropyClass,
        lifetime_policy: SeedLifetimePolicy,
        owner_identity: HumanIdentity,
        seed_bytes: Vec<u8>,
        entropy_manager: &crate::genetics::entropy_hierarchy::EntropyHierarchyManager,
    ) -> Result<Self, BearDogError> {
        let seed_id = Uuid::new_v4();

        let ownership_proof = entropy_manager
            .generate_ownership_proof(&owner_identity, &seed_bytes)
            ?;

        let irreproducibility_proof = entropy_manager
            .generate_irreproducibility_proof(&seed_bytes, &entropy_class)

        let usage_policy = match &entropy_class {
            EntropyClass::HumanLivedExperience { .. } => SeedUsagePolicy {
                allowed_operations: vec![
                    "key_derivation".to_string(),
            entropy_class,
            generation_time: Utc::now(SeedOwnership::HumanOwned {
                owner_identity,
                ownership_proof,
                transfer_count: 0,
            irreproducibility_proof,
            usage_policy,
            usage_history: Vec::new(None,
        })
    }

/// New Event Seed operation.
    /// Creates a new instance
    pub fn new_event_seed(SocialContext,
        sharing_policy: SharingPolicy,

        let location_clone = &event_context.location;
        let timestamp = event_context.event_timestamp;
        let biometric_data = format!(
            "{}:{}:{}",
            timestamp.timestamp(),
            location_clone.unwrap_or_else(|| "Unknown".to_string()),
            0.5 // Default quality score during refactor
        );

        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(biometric_data.as_bytes());
        let biometric_hash = hasher.finalize().to_vec();

        let entropy_class = EntropyClass::HumanLivedExperience {
            source_type: HumanEntropySource::MultiModalHuman {
                sources: vec![], // Would be populated with actual sources
                fusion_algorithm: FusionAlgorithm::CryptographicMixing,
                confidence_score: 0.9,
            capture_timestamp: event_context.event_timestamp,
            biometric_signature: BiometricHash(biometric_hash),
            ownership_proof: ownership_proof.clone(),

        let event_context_clone = event_context.clone();
        let lifetime_policy = SeedLifetimePolicy::EventBased {
            event_id: format!("event_{seed_id}"),
            event_type: &event_context.event_type: event_type.to_string(),
            sharing_policy: sharing_policy.clone(sharing_policy.sharing_expiration,
            ownership: SeedOwnership::SharedOwnership {
                primary_owner: owner_identity,
                shared_with: &event_context.participants,
                sharing_terms: SharingTerms {
                    max_participants: sharing_policy.max_shares,
                    expiration: sharing_policy.sharing_expiration,
                    permissions: &sharing_policy.allowed_operations,
                },
            usage_policy: SeedUsagePolicy {
                allowed_operations: sharing_policy.allowed_operations,
                max_uses: None,
                requires_approval: sharing_policy.require_permission,
            social_context: Some(event_context_clone), // Use cloned context

/// Is Valid operation.
    /// Checks if valid
    pub fn is_valid(&self) -> bool {

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

/// Transfer Ownership operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn transfer_ownership(&mut self, new_owner: HumanIdentity) -> Result<(), BearDogError> {

                ownership_transfer_allowed,
            } => {
                if !ownership_transfer_allowed {
                    return Err(BearDogError::internal("Ownership transfer not allowed for this seed"));
                }
            }
            SeedLifetimePolicy::SelfSovereign {
                transfer_permissions,
                if !transfer_permissions.transferable {
                        message: "This self-sovereign seed is not transferable".to_string(),
            _ => {
                return Err(BearDogError::internal("Ownership transfer not supported for this seed type"));

        let transfer_context = format!("Transferred to {}", new_owner.identity_id);

        match &mut self.ownership {
            SeedOwnership::HumanOwned { transfer_count, .. } => {
                *transfer_count += 1;
                self.ownership = SeedOwnership::HumanOwned {
                    owner_identity: new_owner, // Move happens here
                    ownership_proof: OwnershipProof {
                        signature: vec![0u8; 64], // Would be generated properly
                        timestamp: Utc::now(vec![0u8; 32],
                    },
                    transfer_count: *transfer_count,
                };
                    message: "Cannot transfer ownership from non-human owner".to_string(),

        self.usage_history.push(SeedUsageEvent {
            timestamp: Utc::now(),
            operation: "ownership_transfer".to_string(), // Would be hash of transfer
        });
        Ok(())

/// Expire Ownership operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn expire_ownership(&mut self) -> Result<(), BearDogError> {
        let previous_owner = match &self.ownership {
            SeedOwnership::HumanOwned { owner_identity, .. } => Some(owner_identity),
            SeedOwnership::SharedOwnership { primary_owner, .. } => Some(primary_owner),
            _ => None,
        self.ownership = SeedOwnership::MachineOwned {
            previous_owner,
            ownership_transition: OwnershipTransition::OwnershipExpired,
            self_sovereign: matches!(
                self.lifetime_policy,
                SeedLifetimePolicy::SelfSovereign { .. }
            ),

            operation: "ownership_expiration".to_string(),
            context: "Ownership expired due to policy".to_string(), operation: &str) -> Result<Vec<u8>, BearDogError>> {

        if !self.is_valid() {
            return Err(BearDogError::internal("Seed has expired and cannot be used"));

        if !self
            .usage_policy
            .allowed_operations
            .contains(&operation.to_string())
        {
                message: format!("Operation '{operation}' not allowed for this seed"),

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

        let mut hasher = Sha3_256::new();
        hasher.update(self.seed_bytes.as_bytes());
        hasher.update(operation.as_bytes());
        hasher.update(Utc::now().timestamp().to_le_bytes());
        let result = hasher.finalize().to_vec();

        let result_hash = {
            let mut hash_hasher = Sha3_256::new();
            hash_hasher.update(&result);
            hash_hasher.finalize().to_vec()
            operation: operation.to_string(),
            context: "Cryptographic operation".to_string(),
            result_hash,
        Ok(result)

/// Destroy operation.
    pub fn destroy(&mut self) {
        self.seed_bytes.zeroize();
        self.irreproducibility_proof.zeroize();

            operation: "destroy".to_string(),
            context: "Seed securely destroyed".to_string(),

/// Get Entropy Tier operation.
    /// Gets entropy_tier
    pub fn get_entropy_tier(&self) -> u8 {
        match &self.entropy_class {
            EntropyClass::HumanLivedExperience { .. } => 3,
            EntropyClass::HumanSupervisedMachine { .. } => 2,
            EntropyClass::StoreBoughtMachine { .. } => 1,

/// Can Be Shared operation.
    pub fn can_be_shared(&self) -> bool {
            SeedLifetimePolicy::EventBased { sharing_policy, .. } => {
                sharing_policy.max_shares.is_some()
            } => transfer_permissions.transferable,
            _ => false,

/// Get Current Owner operation.
    /// Gets current_owner
    pub fn get_current_owner(&self) -> Option<&HumanIdentity> {
        match &self.ownership {
            SeedOwnership::HumanOwned { owner_identity, .. } => Some(owner_identity),
            SeedOwnership::SharedOwnership { primary_owner, .. } => Some(primary_owner),
            SeedOwnership::CommunityOwned { .. } => None, // Community ownership
            SeedOwnership::MachineOwned { .. } => None,   // Machine ownership

/// Get Usage Stats operation.
    /// Gets usage_stats
    pub fn get_usage_stats(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::with_capacity(16);
        for event in &self.usage_history {
            *stats.entry(event.operation).or_insert(0) += 1;
        stats

/// Requires Approval operation.
    pub fn requires_approval(&self) -> bool {
        self.usage_policy.requires_approval

/// Get Remaining Uses operation.
    /// Gets remaining_uses
    pub fn get_remaining_uses(&self, operation: &str) -> Option<u32> {
        self.usage_policy.max_uses.map(|max_uses| {
            let used = self
            max_uses.saturating_sub(used)
}
