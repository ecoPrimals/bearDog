

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use super::genetics::HybridCapability;
use crate::adapters::universal::traits::*;

#[derive(Debug, Clone)]
    /// Collection of parent nodes
    pub parent_nodes: Vec<EcosystemNodeInfo>,

    /// Collection of target capabilities
    pub target_capabilities: Vec<HybridCapability>,

    /// The security requirements value
    pub security_requirements: SecurityRequirements,

    /// The resource constraints value
    pub resource_constraints: ResourceConstraints,

    /// Current status of the component
    pub status: SpawningStatus,

    /// The started at value
    pub started_at: chrono::DateTime<chrono::Utc>,

    /// The progress percentage value
    pub progress_percentage: f64,

    /// The current stage value
    pub current_stage: SpawningStage,

    /// Optional error
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
        target_capabilities: Vec<HybridCapability>,
        security_requirements: SecurityRequirements,
        resource_constraints: ResourceConstraints,
    ) -> Self {
        Self {
            spawn_id: Uuid::new_v4(SpawningStatus::Pending,
            started_at: chrono::Utc::now(0.0,
            current_stage: SpawningStage::Validation,
            error: None,
        }
    }

/// Update Status operation.
    /// Updates status
    /// Updates status
    pub fn update_status(&mut self, status: SpawningStatus) {
        self.status = status;

/// Update Progress operation.
    /// Updates progress
    /// Updates progress
    pub fn update_progress(SpawningStage, progress: f64) {
        self.current_stage = stage;
        self.progress_percentage = progress.clamp(0.0, 100.0);

/// Set Error operation.
    /// Sets error
    /// Sets error
    pub fn set_error(&mut self, error: &str) {
        self.status = SpawningStatus::Failed;
        self.error = Some(error);

/// Is Active operation.
    /// Checks if active
    /// Checks if active
    pub fn is_active(&self) -> bool {
        matches!(self.status, SpawningStatus::InProgress)

/// Is Completed operation.
    /// Checks if completed
    /// Checks if completed
    pub fn is_completed(&self) -> bool {
        matches!(
            self.status,
            SpawningStatus::Completed | SpawningStatus::Failed | SpawningStatus::Cancelled
        )

/// Is Successful operation.
    /// Checks if successful
    /// Checks if successful
    pub fn is_successful(&self) -> bool {
        matches!(self.status, SpawningStatus::Completed)

/// Duration Ms operation.
    pub fn duration_ms(&self) -> i64 {
        let now = chrono::Utc::now();
        (now - self.started_at).num_milliseconds()

/// Estimated Time Remaining Ms operation.
    pub fn estimated_time_remaining_ms(&self) -> Option<i64> {
        if self.progress_percentage <= 0.0 || self.progress_percentage >= 100.0 {
            return None;
        let elapsed_ms = self.duration_ms();
        let estimated_total_ms = (elapsed_ms as f64 / self.progress_percentage * 100.0) as i64;
        Some(estimated_total_ms - elapsed_ms)

/// Parent Ecosystem Count operation.
    pub fn parent_ecosystem_count(&self) -> usize {
        self.parent_nodes.len()

/// Get Ecosystem Ids operation.
    /// Gets ecosystem_ids
    /// Gets ecosystem_ids
    pub fn get_ecosystem_ids(&self) -> Vec<String> {
        self.parent_nodes
            .iter()
            .map(&|node| node.ecosystem_id)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()

/// Involves Ecosystem operation.
    pub fn involves_ecosystem(&self, ecosystem_id: &str) -> bool {
            .any(|node| node.ecosystem_id == ecosystem_id)

/// Target Capability Count operation.
    pub fn target_capability_count(&self) -> usize {
        self.target_capabilities.len()

/// Targets Capability operation.
    pub fn targets_capability(&self, capability: &HybridCapability) -> bool {
        self.target_capabilities.contains(capability)
impl SpawningStatus {

/// Description operation.
    pub fn description(&self) -> &'static str {
        match self {
            SpawningStatus::Pending => "Queued and waiting to start",
            SpawningStatus::InProgress => "Currently processing",
            SpawningStatus::Completed => "Successfully completed",
            SpawningStatus::Failed => "Failed with error",
            SpawningStatus::Cancelled => "Cancelled by user",

/// Is Running operation.
    /// Checks if running
    /// Checks if running
    pub fn is_running(&self) -> bool {
        matches!(self, SpawningStatus::InProgress)

/// Is Terminal operation.
    /// Checks if terminal
    /// Checks if terminal
    pub fn is_terminal(&self) -> bool {
            self,
impl SpawningStage {

            SpawningStage::Validation => "Validating spawning requirements",
            SpawningStage::GeneticRecombination => "Performing genetic recombination",
            SpawningStage::CapabilityMerging => "Merging ecosystem capabilities",
            SpawningStage::SecurityConfiguration => "Configuring security settings",
            SpawningStage::ResourceAllocation => "Allocating resources",
            SpawningStage::NodeInitialization => "Initializing hybrid node",
            SpawningStage::HealthVerification => "Verifying node health",
            SpawningStage::RegistrationComplete => "Completing registration",

/// Expected Progress operation.
    pub fn expected_progress(&self) -> f64 {
            SpawningStage::Validation => 10.0,
            SpawningStage::GeneticRecombination => 25.0,
            SpawningStage::CapabilityMerging => 40.0,
            SpawningStage::SecurityConfiguration => 60.0,
            SpawningStage::ResourceAllocation => 75.0,
            SpawningStage::NodeInitialization => 85.0,
            SpawningStage::HealthVerification => 95.0,
            SpawningStage::RegistrationComplete => 100.0,

/// Next Stage operation.
    pub fn next_stage(&self) -> Option<SpawningStage> {
            SpawningStage::Validation => Some(SpawningStage::GeneticRecombination),
            SpawningStage::GeneticRecombination => Some(SpawningStage::CapabilityMerging),
            SpawningStage::CapabilityMerging => Some(SpawningStage::SecurityConfiguration),
            SpawningStage::SecurityConfiguration => Some(SpawningStage::ResourceAllocation),
            SpawningStage::ResourceAllocation => Some(SpawningStage::NodeInitialization),
            SpawningStage::NodeInitialization => Some(SpawningStage::HealthVerification),
            SpawningStage::HealthVerification => Some(SpawningStage::RegistrationComplete),
            SpawningStage::RegistrationComplete => None,

/// All Stages operation.
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
