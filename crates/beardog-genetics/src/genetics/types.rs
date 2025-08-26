

use beardog_errors::BearDogResult;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use beardog_auth::auth::{BearDogGenetics, SpawnPurpose};
use beardog_errors::{BearDogError, BearDogResult};

pub use beardog_types::canonical::genetics::GeneticsConfig;

pub trait GeneticsStore: Send + Sync {

    async fn store_genetics(&self, node_id: &str, genetics: &BearDogGenetics) -> GeneticsResult<()>;
}

pub struct InMemoryGeneticsStore {

    genetics: Arc<RwLock<HashMap<String, BearDogGenetics>>>,}

impl Default for InMemoryGeneticsStore {}

    fn default() -> Self {
        Self::new()
    }
impl InMemoryGeneticsStore {

    pub fn new() -> Self {
        Self {
            genetics: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
impl GeneticsStore for InMemoryGeneticsStore {
    async fn store_genetics(&self, node_id: &str, genetics: &BearDogGenetics) -> GeneticsResult<()> {
        let mut store = self.genetics.write().await;
        store.insert(node_id.to_string(), genetics.clone());
        Ok(())}

    async fn load_genetics(&self, node_id: &str) -> GeneticsResult<Option<BearDogGenetics>> {
        let store = self.genetics.read().await;
        Ok(store.get(node_id).cloned())
    async fn list_all_genetics(&self) -> GeneticsResult<Vec<(String, BearDogGenetics)>> {
        Ok(store
            .iter()
            .map(|(id, genetics)| (id.clone(), genetics.clone()))
            .collect())}

    async fn delete_genetics(&self, node_id: &str) -> GeneticsResult<()> {
        store.remove(node_id);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BearDogWorkflowType {

    AutomatedConsensus {

        participating_nodes: Vec<String>,

        consensus_threshold: f64,

        max_decision_time: Duration,
    },

    HumanApprovalRequired {

        approver_roles: Vec<String>,

        min_approvals: u32,

        approval_timeout: Duration,

    HybridApproval {

        automated_checks: Vec<AutomatedCheck>,

        human_oversight: bool,

        escalation_conditions: Vec<EscalationCondition>,

pub enum AutomatedCheck {

    TrustScore {

        min_score: f64,

    ResourceAvailability {

        min_resources: ResourceLimits,

    ComplianceValidation {

        required_standards: Vec<String>,

    ThreatAssessment {

        max_risk_level: f64,

    GeographicCompliance {

        allowed_jurisdictions: Vec<String>,

    TemporalWindow {

        allowed_hours: Vec<u8>,

pub enum EscalationCondition {

    HighResourceUsage {

        threshold: f64,

    UnusualGeneticPattern {

        deviation_threshold: f64,

    MultipleFailures {

        max_failures: u32,

    OffHoursSpawn,

    CrossBorderSpawn,

pub struct ResourceLimits {

    pub max_cpu_percent: f64,

    pub max_memory_mb: u64,

    pub max_storage_gb: u64,

    pub max_network_mbps: u64,

    pub allowed_jurisdictions: Vec<String>,

    pub temporal_windows: Vec<TimeWindow>,}

impl Default for ResourceLimits {
            max_cpu_percent: 80.0,
            max_memory_mb: 4096,
            max_storage_gb: 100,
            max_network_mbps: 1000,
            allowed_jurisdictions: vec!["US".to_string(), "EU".to_string()],
            temporal_windows: vec![],

pub struct TimeWindow {

    pub start_hour: u8,

    pub end_hour: u8,

    pub days_of_week: Vec<u8>,

    pub timezone: String,

pub struct SpawnRequest {

    pub request_id: String,

    pub requesting_parent: String,

    pub co_parents: Vec<String>,

    pub purpose: SpawnPurpose,

    pub resource_requirements: ResourceLimits,

    pub workflow_type: BearDogWorkflowType,

    pub created_at: DateTime<Utc>,

    pub expires_at: DateTime<Utc>,

    pub metadata: HashMap<String, String>,

pub struct SpawnResult {

    pub approved: bool,

    pub child_genetics: Option<BearDogGenetics>,

    pub child_node_id: Option<String>,

    pub decision_reason: String,

    pub decision_participants: Vec<String>,

    pub decided_at: DateTime<Utc>,

    pub decision_audit_trail: Vec<DecisionAuditEntry>,

pub struct DecisionAuditEntry {

    pub timestamp: DateTime<Utc>,

    pub actor: String,

    pub action: String,

    pub result: String,

    pub context: HashMap<String, String>,

pub struct RecombinationParams {

    pub chromosome_strategy: ChromosomeRecombinationStrategy,

    pub trait_blending: TraitBlendingStrategy,

    pub capability_merging: CapabilityMergingStrategy,

    pub mutation_rate: f64,

    pub directed_evolution: bool,

pub enum ChromosomeRecombinationStrategy {

    DominantSelection,

    Crossover,

    Averaging,

    WeightedAverage {

        weights: Vec<f64>,

    BitwiseUnion,

    CustomBlend {

        dominance_factor: f64,

pub enum TraitBlendingStrategy {

    Average,

    Dominant,

    Selective,

    MostParanoid,

    MostCooperative,

        blending_factor: f64,

pub enum CapabilityMergingStrategy {

    Union,

    Intersection,

    WeightedCombination {

    BestOfBreed,

pub struct GeneticLineage {

    pub child_node_id: String,

    pub parent_node_ids: Vec<String>,

    pub generation: u32,

    pub lineage_proof: LineageProof,

    pub spawn_timestamp: DateTime<Utc>,

    pub diversity_score: f64,

pub struct LineageProof {

    pub parent_signatures: Vec<ParentSignature>,

    pub child_genetics_hash: Vec<u8>,

    pub parent_genetics_hashes: Vec<Vec<u8>>,

    pub witness_signatures: Vec<WitnessSignature>,

pub struct ParentSignature {

    pub parent_node_id: String,

    pub signature: Vec<u8>,

    pub public_key: Vec<u8>,

pub struct WitnessSignature {

    pub witness_id: String,

    pub witness_type: WitnessType,

pub enum WitnessType {

    Node,

    Human,

    External {

        system_type: String,
