

use beardog_auth::auth::{BearDogGenetics, NodeCapability};
use beardog_errors::{BearDogError, BearDogResult};
use chrono::{DateTime, Utc};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicU64, Arc, RwLock},
};
use tracing::{debug, info};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct P2PGeneticsConfig {
    pub max_peers: usize,
    pub consensus_threshold: f64,
    pub key_mixing_enabled: bool,
    pub collaboration_timeout_secs: u64,
}

#[derive(Debug, Default)]
pub struct P2PGeneticsStats {
    pub collaborative_spawns: AtomicU64,
    pub key_mixing_operations: AtomicU64,
    pub peer_contributions: AtomicU64,
    pub genetic_sharing_events: AtomicU64,
    pub network_fitness_evaluations: AtomicU64,
    pub recursive_evolutions: AtomicU64,

pub struct GeneticsPoolStats {
    pub genetics_allocated: AtomicU64,
    pub genetics_reused: AtomicU64,
    pub chromosomes_allocated: AtomicU64,
    pub chromosomes_reused: AtomicU64,
    pub capabilities_allocated: AtomicU64,
    pub capabilities_reused: AtomicU64,
    pub pool_capacity: usize,

pub struct NetworkEvaluationState {
    pub active_evaluations: usize,
    pub consensus_reached: usize,
    pub average_confidence: f64,

#[derive(Debug, Clone)]
pub struct KeyShare {
    pub share_index: usize,
    pub share_data: Vec<u8>,
    pub verification_hash: Vec<u8>,
    pub threshold: usize,
    pub created_at: DateTime<Utc>,

pub struct KeySharePool {
    shares: HashMap<String, Vec<KeyShare>>,
    active_mixings: HashMap<String, ActiveKeyMixing>,}

impl KeySharePool {

    pub fn new() -> Self {
        Self {
            shares: HashMap::with_capacity(16),
            active_mixings: HashMap::with_capacity(16),
        }
    }

    pub fn add_share(&mut self, node_id: &str, share: KeyShare) {
        self.shares.entry(node_id).or_default().push(share);

    pub fn start_mixing(&mut self, session_id: &str, mixing: ActiveKeyMixing) {
        self.active_mixings.insert(session_id, mixing);

    pub fn get_shares(&self, node_id: &str) -> Option<&Vec<KeyShare>> {
        self.shares.get(node_id)

    pub fn is_mixing_active(&self, session_id: &str) -> bool {
        self.active_mixings.contains_key(session_id)

pub struct ActiveKeyMixing {
    pub mixing_id: String,
    pub participating_peers: Vec<String>,
    pub collected_shares: HashMap<String, KeyShare>,
    pub target_threshold: u32,
    pub started_at: DateTime<Utc>,

#[derive(Debug)]
pub struct CollaborativeKeys {
    pub mixed_key_id: String,
    pub access_key: Vec<u8>,
    pub verification_key: Vec<u8>,
    pub participants: Vec<String>,

pub enum MixingProtocol {
    ShamirSecretSharing,
    ThresholdSignature,
    MultiPartyComputation,

pub struct SharedGeneticData {
    pub genetics: BearDogGenetics,
    pub contributor_peer: String,
    pub contribution_time: DateTime<Utc>,
    pub access_permissions: Vec<String>,

pub struct PeerGeneticsNode {
    pub peer_id: String,
    pub shared_capabilities: Vec<NodeCapability>,
    pub genetic_reputation: f64,
    pub last_seen: DateTime<Utc>,

pub struct DistributedFitnessResult {
    pub genetics_id: String,
    pub peer_evaluations: HashMap<String, f64>,
    pub consensus_fitness: f64,
    pub confidence_score: f64,

pub struct PeerEvaluator {
    pub evaluation_weight: f64,
    pub historical_accuracy: f64,

pub struct FitnessEvaluatorNode {
    pub node_id: String,
    pub base_fitness: f64,
    pub evaluation_count: usize,
    pub average_response_time: f64,
    pub last_active: chrono::DateTime<chrono::Utc>,}

impl FitnessEvaluatorNode {

    pub fn new(node_id: &str) -> Self {
            node_id,
            base_fitness: 0.5,
            evaluation_count: 0,
            average_response_time: 0.0,
            last_active: chrono::Utc::now(),

    pub fn update_stats(&mut self, response_time: f64) {
        self.evaluation_count += 1;
        self.average_response_time =
            (self.average_response_time * (self.evaluation_count - 1) as f64 + response_time)
                / self.evaluation_count as f64;
        self.last_active = chrono::Utc::now();

pub struct P2PGeneticsNetwork {

    node_registry: Arc<GeneticsNodeRegistry>,

    key_mixer: Arc<CollaborativeKeyMixer>,

    fitness_evaluator: Arc<DistributedFitnessEvaluator>,

    config: P2PGeneticsConfig,

pub struct CollaborativeKeyMixer {
    active_mixings: Arc<RwLock<HashMap<String, ActiveKeyMixing>>>,

    pool: Arc<KeySharePool>,

pub struct DistributedFitnessEvaluator {
    evaluator_pool: HashMap<String, FitnessEvaluatorNode>,
    evaluation_state: NetworkEvaluationState,}

impl Default for CollaborativeKeyMixer {}

    fn default() -> Self {
        Self::new()
impl CollaborativeKeyMixer {

            active_mixings: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            pool: Arc::new(KeySharePool::new()),

    pub async fn start_mixing_session(
        &self,
        session_id: &str,
        participants: Vec<&str>,
    ) -> BearDogResult<()> {
        let mixing = ActiveKeyMixing {
            mixing_id: session_id.clone(),
            participating_peers: participants,
            collected_shares: HashMap::with_capacity(16),
            target_threshold: 2,
            started_at: chrono::Utc::now(),
        };
        let mut active = self
            .active_mixings
            .write()
            .map_err(|_| BearDogError::internal("Failed to acquire write lock on active mixings".to_string(),
            ))?;
        active.insert(session_id, mixing);
        Ok(())

    pub fn get_active_sessions(&self) -> BearDogResult<Vec<String>> {
        let active = self
            .read()
            .map_err(|_| BearDogError::internal("Failed to acquire read lock on active mixings".to_string(),
        Ok(active.keys().cloned().collect())

    pub fn get_pool(&self) -> Arc<KeySharePool> {
        self.pool.clone()
impl DistributedFitnessEvaluator {

            evaluator_pool: HashMap::with_capacity(16),
            evaluation_state: NetworkEvaluationState::default(),

    pub fn add_evaluator_node(&mut self, node_id: &str, node: FitnessEvaluatorNode) {
        self.evaluator_pool.insert(node_id, node);
        self.evaluation_state.active_evaluations += 1;

    pub fn remove_evaluator_node(&mut self, node_id: &str) -> Option<FitnessEvaluatorNode> {
        if let Some(node) = self.evaluator_pool.remove(node_id) {
            if self.evaluation_state.active_evaluations > 0 {
                self.evaluation_state.active_evaluations -= 1;
            }
            Some(node)
        } else {
            None

    pub fn get_active_evaluators_count(&self) -> usize {
        self.evaluator_pool.len()

    pub fn update_evaluation_state(&mut self, consensus_reached: usize, avg_confidence: f64) {
        self.evaluation_state.consensus_reached = consensus_reached;
        self.evaluation_state.average_confidence = avg_confidence;

    pub fn get_evaluation_state(&self) -> &NetworkEvaluationState {
        &self.evaluation_state

    pub fn has_sufficient_evaluators(&self, min_required: usize) -> bool {
        self.evaluator_pool.len() >= min_required

    pub async fn evaluate_fitness_distributed(&self, genetics_data: &str) -> BearDogResult<f64> {
        if self.evaluator_pool.is_empty() {
            return Err(BearDogError::ValidationError(
                "No evaluator nodes available".to_string(),
            ));

        let mut total_fitness = 0.0;
        let mut evaluations_count = 0;
        for (node_id, evaluator) in &self.evaluator_pool {

            let node_fitness = evaluator.base_fitness + (genetics_data.len() as f64 * 0.001);
            total_fitness += node_fitness;
            evaluations_count += 1;
            tracing::debug!("Node {} evaluated fitness: {}", node_id, node_fitness);
        if evaluations_count == 0 {
                "No evaluations completed".to_string(),
        let average_fitness = total_fitness / evaluations_count as f64;
        Ok(average_fitness.clamp(0.0, 1.0))
impl P2PGeneticsNetwork {

    pub fn new(config: P2PGeneticsConfig) -> Self {
            node_registry: Arc::new(GeneticsNodeRegistry::new()),
            key_mixer: Arc::new(CollaborativeKeyMixer::new()),
            fitness_evaluator: Arc::new(DistributedFitnessEvaluator::new()),

            config,

    pub fn get_node_registry(&self) -> Arc<GeneticsNodeRegistry> {
        self.node_registry.clone()

    pub fn get_key_mixer(&self) -> Arc<CollaborativeKeyMixer> {
        self.key_mixer.clone()

    pub fn get_fitness_evaluator(&self) -> Arc<DistributedFitnessEvaluator> {
        self.fitness_evaluator.clone()

    pub fn get_zero_copy_engine(&self) -> Result<String, String> {
        Err("Zero-copy engine temporarily disabled during refactor".to_string())

    pub fn get_config(&self) -> &P2PGeneticsConfig {
        &self.config

    pub async fn connect_node(&self, node: GeneticsNode) -> BearDogResult<()> {
        let node_id = node.node_id.clone();

        info!("Connected to genetics node: {node_id}");

    pub async fn contribute_node_genetics(
        node_id: &str,
        _genetics_data: BearDogGenetics,
    ) -> GeneticsResult<String> {
        info!("🤝 Contributing genetics from node: {node_id}");

        let nodes = self.node_registry.get_active_nodes();
        let _node =
            nodes
                .iter()
                .find(|n| n.node_id == node_id)
                .ok_or_else(|| BearDogError::not_found(format!("Node not found: {node_id}")))?;
        let contribution_id = Uuid::new_v4().to_string();
        info!("✅ Created genetics contribution: {contribution_id}");
        Ok(contribution_id)

    pub async fn retrieve_node_genetics(&self, node_id: &str) -> GeneticsResult<BearDogGenetics> {
        debug!("📥 Retrieving genetics from node: {node_id}");

        Ok(BearDogGenetics::default())

pub struct GeneticsNodeRegistry {
    active_nodes: HashMap<String, GeneticsNode>,
    node_capabilities: HashMap<String, Vec<NodeCapability>>,
    last_heartbeat: HashMap<String, chrono::DateTime<chrono::Utc>>,

pub struct GeneticsNode {
    pub network_address: String,
    pub capabilities: Vec<NodeCapability>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub trust_score: f64,}

impl GeneticsNodeRegistry {

            active_nodes: HashMap::with_capacity(16),
            node_capabilities: HashMap::with_capacity(16),
            last_heartbeat: HashMap::with_capacity(16),

    pub fn register_node(&mut self, node: GeneticsNode) {
        self.node_capabilities
            .insert(node.node_id.clone(), node.capabilities.clone());
        self.last_heartbeat
            .insert(node.node_id.clone(), chrono::Utc::now());
        self.active_nodes.insert(node.node_id.clone(), node);

    pub fn update_heartbeat(&mut self, node_id: &str) -> bool {
        if self.active_nodes.contains_key(node_id) {
            self.last_heartbeat
                .insert(node_id.to_string(), chrono::Utc::now());
            true
            false

    pub fn get_active_nodes(&self) -> Vec<&GeneticsNode> {
        self.active_nodes.values().collect()

    pub fn get_nodes_with_capability(&self, capability: &NodeCapability) -> Vec<&GeneticsNode> {
        self.active_nodes
            .values()
            .filter(|node| node.capabilities.contains(capability))
            .collect()

    pub fn cleanup_inactive_nodes(&mut self) -> usize {
        let cutoff = chrono::Utc::now() - chrono::Duration::minutes(5);
        let mut removed_count = 0;
        let inactive_nodes: Vec<String> = self
            .last_heartbeat
            .iter()
            .filter(|(_, &last_seen)| last_seen < cutoff)
            .map(|(node_id, _)| node_id.clone())
            .collect();
        for node_id in inactive_nodes {
            self.active_nodes.remove(&node_id);
            self.node_capabilities.remove(&node_id);
            self.last_heartbeat.remove(&node_id);
            removed_count += 1;
        removed_count
