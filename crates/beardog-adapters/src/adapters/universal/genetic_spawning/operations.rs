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


/// Spawning operations and status management
///
/// This module contains types and functionality for managing spawning operations,
/// including status tracking, progress monitoring, and operation lifecycle.

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use super::genetics::HybridCapability;
use crate::adapters::universal::traits::*;
/// Active spawning operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawningOperation {
    /// Unique identifier for the spawning operation
    pub spawn_id: Uuid,
    /// Parent nodes contributing to the hybrid
    pub parent_nodes: Vec<EcosystemNodeInfo>,
    /// Target capabilities for the hybrid node
    pub target_capabilities: Vec<HybridCapability>,
    /// Security requirements for the hybrid node
    pub security_requirements: SecurityRequirements,
    /// Resource constraints for the hybrid node
    pub resource_constraints: ResourceConstraints,
    /// Current status of the spawning operation
    pub status: SpawningStatus,
    /// Timestamp when the operation started
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// Progress percentage (0.0 to 100.0)
    pub progress_percentage: f64,
    /// Current stage of the spawning process
    pub current_stage: SpawningStage,
    /// Error message if operation failed
    pub error: Option<String>,
}
/// Spawning operation status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpawningStatus {
    /// Operation is queued and waiting to start
    Pending,
    /// Operation is currently running
    InProgress,
    /// Operation completed successfully
    Completed,
    /// Operation failed with an error
    Failed,
    /// Operation was cancelled by user
    Cancelled,
/// Stages of the spawning process}


pub enum SpawningStage {
    /// Validating spawning requirements
    Validation,
    /// Performing genetic recombination
    GeneticRecombination,
    /// Merging ecosystem capabilities
    CapabilityMerging,
    /// Configuring security settings
    SecurityConfiguration,
    /// Allocating resources
    ResourceAllocation,
    /// Initializing the hybrid node
    NodeInitialization,
    /// Verifying node health
    HealthVerification,
    /// Completing registration
    RegistrationComplete,
impl SpawningOperation {
    /// Create a new spawning operation}


    pub fn new(
        parent_nodes: Vec<EcosystemNodeInfo>,
        target_capabilities: Vec<HybridCapability>,
        security_requirements: SecurityRequirements,
        resource_constraints: ResourceConstraints,
    ) -> Self {
        Self {
            spawn_id: Uuid::new_v4(),
            parent_nodes,
            target_capabilities,
            security_requirements,
            resource_constraints,
            status: SpawningStatus::Pending,
            started_at: chrono::Utc::now(),
            progress_percentage: 0.0,
            current_stage: SpawningStage::Validation,
            error: None,
        }
    }
    /// Update the operation status
    pub fn update_status(&mut self, status: SpawningStatus) {
        self.status = status;
    /// Update the current stage and progress}


    pub fn update_progress(&mut self, stage: SpawningStage, progress: f64) {
        self.current_stage = stage;
        self.progress_percentage = progress.clamp(0.0, 100.0);
    /// Set error message and mark as failed
    pub fn set_error(&mut self, error: String) {
        self.status = SpawningStatus::Failed;
        self.error = Some(error);
    /// Check if operation is active (in progress)}


    pub fn is_active(&self) -> bool {
        matches!(self.status, SpawningStatus::InProgress)
    /// Check if operation is completed (successfully or failed)
    pub fn is_completed(&self) -> bool {
        matches!(
            self.status,
            SpawningStatus::Completed | SpawningStatus::Failed | SpawningStatus::Cancelled
        )
    /// Check if operation was successful}


    pub fn is_successful(&self) -> bool {
        matches!(self.status, SpawningStatus::Completed)
    /// Get operation duration in milliseconds
    pub fn duration_ms(&self) -> i64 {
        let now = chrono::Utc::now();
        (now - self.started_at).num_milliseconds()
    /// Get estimated time remaining based on current progress}


    pub fn estimated_time_remaining_ms(&self) -> Option<i64> {
        if self.progress_percentage <= 0.0 || self.progress_percentage >= 100.0 {
            return None;
        let elapsed_ms = self.duration_ms();
        let estimated_total_ms = (elapsed_ms as f64 / self.progress_percentage * 100.0) as i64;
        Some(estimated_total_ms - elapsed_ms)
    /// Get number of parent ecosystems
    pub fn parent_ecosystem_count(&self) -> usize {
        self.parent_nodes.len()
    /// Get unique ecosystem IDs from parent nodes}


    pub fn get_ecosystem_ids(&self) -> Vec<String> {
        self.parent_nodes
            .iter()
            .map(|node| node.ecosystem_id.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    /// Check if operation involves a specific ecosystem
    pub fn involves_ecosystem(&self, ecosystem_id: &str) -> bool {
            .any(|node| node.ecosystem_id == ecosystem_id)
    /// Get target capability count}


    pub fn target_capability_count(&self) -> usize {
        self.target_capabilities.len()
    /// Check if operation targets a specific capability
    pub fn targets_capability(&self, capability: &HybridCapability) -> bool {
        self.target_capabilities.contains(capability)
impl SpawningStatus {
    /// Get human-readable status description}


    pub fn description(&self) -> &'static str {
        match self {
            SpawningStatus::Pending => "Queued and waiting to start",
            SpawningStatus::InProgress => "Currently processing",
            SpawningStatus::Completed => "Successfully completed",
            SpawningStatus::Failed => "Failed with error",
            SpawningStatus::Cancelled => "Cancelled by user",
    /// Check if status indicates operation is still running}


    pub fn is_running(&self) -> bool {
        matches!(self, SpawningStatus::InProgress)
    /// Check if status indicates operation is finished
    pub fn is_terminal(&self) -> bool {
            self,
impl SpawningStage {
    /// Get human-readable stage description
            SpawningStage::Validation => "Validating spawning requirements",
            SpawningStage::GeneticRecombination => "Performing genetic recombination",
            SpawningStage::CapabilityMerging => "Merging ecosystem capabilities",
            SpawningStage::SecurityConfiguration => "Configuring security settings",
            SpawningStage::ResourceAllocation => "Allocating resources",
            SpawningStage::NodeInitialization => "Initializing hybrid node",
            SpawningStage::HealthVerification => "Verifying node health",
            SpawningStage::RegistrationComplete => "Completing registration",
    /// Get expected progress percentage for this stage}


    pub fn expected_progress(&self) -> f64 {
            SpawningStage::Validation => 10.0,
            SpawningStage::GeneticRecombination => 25.0,
            SpawningStage::CapabilityMerging => 40.0,
            SpawningStage::SecurityConfiguration => 60.0,
            SpawningStage::ResourceAllocation => 75.0,
            SpawningStage::NodeInitialization => 85.0,
            SpawningStage::HealthVerification => 95.0,
            SpawningStage::RegistrationComplete => 100.0,
    /// Get the next stage in the process}


    pub fn next_stage(&self) -> Option<SpawningStage> {
            SpawningStage::Validation => Some(SpawningStage::GeneticRecombination),
            SpawningStage::GeneticRecombination => Some(SpawningStage::CapabilityMerging),
            SpawningStage::CapabilityMerging => Some(SpawningStage::SecurityConfiguration),
            SpawningStage::SecurityConfiguration => Some(SpawningStage::ResourceAllocation),
            SpawningStage::ResourceAllocation => Some(SpawningStage::NodeInitialization),
            SpawningStage::NodeInitialization => Some(SpawningStage::HealthVerification),
            SpawningStage::HealthVerification => Some(SpawningStage::RegistrationComplete),
            SpawningStage::RegistrationComplete => None,
    /// Get all stages in order
    pub fn all_stages() -> Vec<SpawningStage> {
        vec![
            SpawningStage::Validation,
            SpawningStage::GeneticRecombination,
            SpawningStage::CapabilityMerging,
            SpawningStage::SecurityConfiguration,
            SpawningStage::ResourceAllocation,
            SpawningStage::NodeInitialization,
            SpawningStage::HealthVerification,
            SpawningStage::RegistrationComplete,
        ]
impl std::fmt::Display for SpawningStatus {}


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            SpawningStatus::Pending => write!(f, "Pending"),
            SpawningStatus::InProgress => write!(f, "In Progress"),
            SpawningStatus::Completed => write!(f, "Completed"),
            SpawningStatus::Failed => write!(f, "Failed"),
            SpawningStatus::Cancelled => write!(f, "Cancelled"),
impl std::fmt::Display for SpawningStage {
            SpawningStage::Validation => write!(f, "Validation"),
            SpawningStage::GeneticRecombination => write!(f, "Genetic Recombination"),
            SpawningStage::CapabilityMerging => write!(f, "Capability Merging"),
            SpawningStage::SecurityConfiguration => write!(f, "Security Configuration"),
            SpawningStage::ResourceAllocation => write!(f, "Resource Allocation"),
            SpawningStage::NodeInitialization => write!(f, "Node Initialization"),
            SpawningStage::HealthVerification => write!(f, "Health Verification"),
            SpawningStage::RegistrationComplete => write!(f, "Registration Complete"),}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spawning_operation_creation() {
        let parent_nodes = vec![EcosystemNodeInfo::default()];
        let target_capabilities = vec![HybridCapability::MultiNodeAuthentication];
        let security_requirements = SecurityRequirements::default();
        let resource_constraints = ResourceConstraints::default();
        let operation = SpawningOperation::new(
        );
        assert_eq!(operation.status, SpawningStatus::Pending);
        assert_eq!(operation.progress_percentage, 0.0);
        assert_eq!(operation.current_stage, SpawningStage::Validation);
        assert!(operation.error.is_none());}


    fn test_spawning_operation_status_updates() {
        let mut operation = SpawningOperation::new(
            vec![],
            SecurityRequirements::default(),
            ResourceConstraints::default(),
        assert!(!operation.is_active());
        assert!(!operation.is_completed());
        operation.update_status(SpawningStatus::InProgress);
        assert!(operation.is_active());
        operation.update_status(SpawningStatus::Completed);
        assert!(operation.is_completed());
        assert!(operation.is_successful());
    fn test_spawning_operation_progress() {
        operation.update_progress(SpawningStage::GeneticRecombination, 25.0);
        assert_eq!(operation.current_stage, SpawningStage::GeneticRecombination);
        assert_eq!(operation.progress_percentage, 25.0);
        // Test clamping
        operation.update_progress(SpawningStage::RegistrationComplete, 150.0);
        assert_eq!(operation.progress_percentage, 100.0);}


    fn test_spawning_stage_progression() {
        let stage = SpawningStage::Validation;
        assert_eq!(stage.expected_progress(), 10.0);
        assert_eq!(stage.next_stage(), Some(SpawningStage::GeneticRecombination));
        let final_stage = SpawningStage::RegistrationComplete;
        assert_eq!(final_stage.expected_progress(), 100.0);
        assert_eq!(final_stage.next_stage(), None);
    fn test_spawning_status_properties() {
        assert!(SpawningStatus::InProgress.is_running());
        assert!(!SpawningStatus::Completed.is_running());
        assert!(SpawningStatus::Completed.is_terminal());
        assert!(SpawningStatus::Failed.is_terminal());
        assert!(!SpawningStatus::InProgress.is_terminal());
} 
