//! Core Genetic Spawning Engine
//!
//! This module provides the central genetic spawning engine that orchestrates
//! the entire spawning process including workflow management, genetic recombination,
//! and lineage tracking.

use super::super::types::*;
use beardog_auth::auth::BearDogGenetics;
// use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use beardog_errors::{BearDogError, BearDogResult};
use sha3::Digest;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Core genetic spawning engine with HSM integration
pub struct GeneticSpawningEngine {
    /// Storage for genetic data
    pub(crate) genetics_store: Arc<dyn GeneticsStore>,
    /// HSM manager for cryptographic operations
    // pub(crate) hsm_manager: Arc<HsmManager>,
    /// Configuration settings
    pub(crate) config: GeneticsConfig,
    /// Active spawn requests being processed
    pub(crate) active_spawn_requests: Arc<RwLock<HashMap<String, SpawnRequest>>>,
    /// Lineage tracking store
    pub(crate) lineage_store: Arc<RwLock<HashMap<String, GeneticLineage>>>,
}

impl GeneticSpawningEngine {
    /// Create a new genetic spawning engine
    pub fn new(
        genetics_store: Arc<dyn GeneticsStore>,
        // hsm_manager: Arc<HsmManager>,
        config: GeneticsConfig,
    ) -> Self {
        Self {
            genetics_store,
            // hsm_manager,
            config,
            active_spawn_requests: Arc::new(RwLock::new(HashMap::new())),
            lineage_store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Process a spawn request through the appropriate workflow
    pub async fn process_spawn_request(&self, request: SpawnRequest) -> BearDogResult<SpawnResult> {
        info!("Processing spawn request: {}", request.request_id);

        // Store the request
        self.active_spawn_requests
            .write()
            .await
            .insert(request.request_id.clone(), request.clone());

        // Process based on workflow type
        let result = match &request.workflow_type {
            BearDogWorkflowType::AutomatedConsensus {
                participating_nodes,
                consensus_threshold,
                max_decision_time,
            } => {
                super::workflows::process_automated_consensus(
                    self,
                    &request,
                    participating_nodes,
                    *consensus_threshold,
                    *max_decision_time,
                )
                .await?
            }
            BearDogWorkflowType::HumanApprovalRequired {
                approver_roles,
                min_approvals,
                approval_timeout,
            } => {
                super::workflows::process_human_approval(
                    self,
                    &request,
                    approver_roles,
                    *min_approvals,
                    *approval_timeout,
                )
                .await?
            }
            BearDogWorkflowType::HybridApproval {
                automated_checks,
                human_oversight,
                escalation_conditions,
            } => {
                super::workflows::process_hybrid_approval(
                    self,
                    &request,
                    automated_checks,
                    *human_oversight,
                    escalation_conditions,
                )
                .await?
            }
        };

        // Remove from active requests
        self.active_spawn_requests
            .write()
            .await
            .remove(&request.request_id);

        // If approved, create lineage record
        if result.approved {
            if let Some(ref child_genetics) = result.child_genetics {
                if let Some(ref child_node_id) = result.child_node_id {
                    super::lineage::create_lineage_record(
                        self,
                        &request,
                        child_node_id,
                        child_genetics,
                    )
                    .await?;
                }
            }
        }

        Ok(result)
    }

    /// Perform genetic recombination to create child genetics
    pub async fn perform_genetic_recombination(
        &self,
        request: &SpawnRequest,
    ) -> BearDogResult<BearDogGenetics> {
        info!(
            "Performing genetic recombination for spawn request: {}",
            request.request_id
        );

        // Load parent genetics
        let mut parent_genetics = Vec::new();

        // Add requesting parent
        if let Some(genetics) = self
            .genetics_store
            .load_genetics(&request.requesting_parent)
            .await?
        {
            parent_genetics.push(genetics);
        } else {
            return Err(BearDogError::NotFound {
                resource_type: "parent_genetics".to_string(),
                id: request.requesting_parent.clone(),
            });
        }

        // Add co-parents
        for co_parent in &request.co_parents {
            if let Some(genetics) = self.genetics_store.load_genetics(co_parent).await? {
                parent_genetics.push(genetics);
            }
        }

        if parent_genetics.is_empty() {
            return Err(BearDogError::InvalidInput {
                message: "No parent genetics found for recombination".to_string(),
            });
        }

        // Apply recombination based on configuration
        let recombination_params = RecombinationParams {
            chromosome_strategy: ChromosomeRecombinationStrategy::DominantSelection,
            trait_blending: TraitBlendingStrategy::WeightedAverage {
                weights: vec![0.6, 0.4], // Favor first parent slightly
            },
            capability_merging: CapabilityMergingStrategy::Union,
            mutation_rate: self.config.base_mutation_rate,
            directed_evolution: self.config.enable_directed_evolution,
        };

        super::recombination::recombine_genetics(
            self,
            &parent_genetics,
            &recombination_params,
            &request.purpose,
        )
        .await
    }
}
