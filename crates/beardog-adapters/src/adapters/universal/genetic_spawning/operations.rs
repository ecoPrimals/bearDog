

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use super::genetics::HybridCapability;
use crate::adapters::universal::traits::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawningOperation {

    pub spawn_id: Uuid,

    pub parent_nodes: Vec<EcosystemNodeInfo>,

    pub target_capabilities: Vec<HybridCapability>,

    pub security_requirements: SecurityRequirements,

    pub resource_constraints: ResourceConstraints,

    pub status: SpawningStatus,

    pub started_at: chrono::DateTime<chrono::Utc>,

    pub progress_percentage: f64,

    pub current_stage: SpawningStage,

    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpawningStatus {

    Pending,

    InProgress,

    Completed,

    Failed,

    Cancelled,

pub enum SpawningStage {

    Validation,

    GeneticRecombination,

    CapabilityMerging,

    SecurityConfiguration,

    ResourceAllocation,

    NodeInitialization,

    HealthVerification,

    RegistrationComplete,
impl SpawningOperation {

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

    pub fn update_status(&mut self, status: SpawningStatus) {
        self.status = status;

    pub fn update_progress(&mut self, stage: SpawningStage, progress: f64) {
        self.current_stage = stage;
        self.progress_percentage = progress.clamp(0.0, 100.0);

    pub fn set_error(&mut self, error: &str) {
        self.status = SpawningStatus::Failed;
        self.error = Some(error);

    pub fn is_active(&self) -> bool {
        matches!(self.status, SpawningStatus::InProgress)

    pub fn is_completed(&self) -> bool {
        matches!(
            self.status,
            SpawningStatus::Completed | SpawningStatus::Failed | SpawningStatus::Cancelled
        )

    pub fn is_successful(&self) -> bool {
        matches!(self.status, SpawningStatus::Completed)

    pub fn duration_ms(&self) -> i64 {
        let now = chrono::Utc::now();
        (now - self.started_at).num_milliseconds()

    pub fn estimated_time_remaining_ms(&self) -> Option<i64> {
        if self.progress_percentage <= 0.0 || self.progress_percentage >= 100.0 {
            return None;
        let elapsed_ms = self.duration_ms();
        let estimated_total_ms = (elapsed_ms as f64 / self.progress_percentage * 100.0) as i64;
        Some(estimated_total_ms - elapsed_ms)

    pub fn parent_ecosystem_count(&self) -> usize {
        self.parent_nodes.len()

    pub fn get_ecosystem_ids(&self) -> Vec<String> {
        self.parent_nodes
            .iter()
            .map(|node| node.ecosystem_id.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()

    pub fn involves_ecosystem(&self, ecosystem_id: &str) -> bool {
            .any(|node| node.ecosystem_id == ecosystem_id)

    pub fn target_capability_count(&self) -> usize {
        self.target_capabilities.len()

    pub fn targets_capability(&self, capability: &HybridCapability) -> bool {
        self.target_capabilities.contains(capability)
impl SpawningStatus {

    pub fn description(&self) -> &'static str {
        match self {
            SpawningStatus::Pending => "Queued and waiting to start",
            SpawningStatus::InProgress => "Currently processing",
            SpawningStatus::Completed => "Successfully completed",
            SpawningStatus::Failed => "Failed with error",
            SpawningStatus::Cancelled => "Cancelled by user",

    pub fn is_running(&self) -> bool {
        matches!(self, SpawningStatus::InProgress)

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

    pub fn expected_progress(&self) -> f64 {
            SpawningStage::Validation => 10.0,
            SpawningStage::GeneticRecombination => 25.0,
            SpawningStage::CapabilityMerging => 40.0,
            SpawningStage::SecurityConfiguration => 60.0,
            SpawningStage::ResourceAllocation => 75.0,
            SpawningStage::NodeInitialization => 85.0,
            SpawningStage::HealthVerification => 95.0,
            SpawningStage::RegistrationComplete => 100.0,

    pub fn next_stage(&self) -> Option<SpawningStage> {
            SpawningStage::Validation => Some(SpawningStage::GeneticRecombination),
            SpawningStage::GeneticRecombination => Some(SpawningStage::CapabilityMerging),
            SpawningStage::CapabilityMerging => Some(SpawningStage::SecurityConfiguration),
            SpawningStage::SecurityConfiguration => Some(SpawningStage::ResourceAllocation),
            SpawningStage::ResourceAllocation => Some(SpawningStage::NodeInitialization),
            SpawningStage::NodeInitialization => Some(SpawningStage::HealthVerification),
            SpawningStage::HealthVerification => Some(SpawningStage::RegistrationComplete),
            SpawningStage::RegistrationComplete => None,

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
