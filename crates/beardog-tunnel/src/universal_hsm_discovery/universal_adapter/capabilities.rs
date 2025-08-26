

use super::core_types::*;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::capabilities::{CapabilityRequirements as CanonicalCapabilityRequirements, CapabilityType};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime};
use tracing::{debug, info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BearDogCapability {

    pub capability_id: Uuid,

    pub name: String,

    pub category: BearDogCapabilityCategory,

    pub version: String,

    pub description: String,

    pub security_level: SecurityLevel,

    pub performance_requirements: PerformanceRequirements,

    pub security_requirements: SecurityRequirements,

    pub status: CapabilityStatus,

    pub supported_protocols: Vec<String>,

    pub metadata: HashMap<String, String>,

    pub dependencies: Vec<Uuid>,

    pub registered_at: SystemTime,

    pub updated_at: SystemTime,
}

pub enum BearDogCapabilityCategory {

    Cryptography,

    HardwareSecurityModule,

    KeyManagement,

    DigitalSignature,

    DataEncryption,

    NetworkCommunication,

    ServiceDiscovery,

    DistributedOperations,

    GeneticOperations,

    ArtificialIntelligence,

    Monitoring,

    Storage,

    Authentication,

    WorkflowOrchestration,

    Custom {

        category_name: String,
    },

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CapabilityStatus {

    Active,

    Inactive,

    Maintenance,

    Experimental,

    Failed,

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]}

pub enum SecurityLevel {

    Public,

    Basic,

    Standard,

    High,

    Critical,

    Maximum,

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirements {

    pub required_capabilities: Vec<BearDogCapability>,

    pub optional_capabilities: Vec<BearDogCapability>,

    pub min_security_level: SecurityLevel,

    pub network_requirements: NetworkRequirements,

    pub compatibility_requirements: CompatibilityRequirements,

    pub resource_requirements: ResourceRequirements,

pub struct PerformanceRequirements {

    pub max_latency_ms: u64,

    pub min_throughput_ops_per_sec: u64,

    pub max_memory_bytes: u64,

    pub max_cpu_percent: f64,

    pub required_availability_percent: f64,

    pub max_response_time_ms: u64,

    pub max_concurrent_operations: u32,

    pub performance_tier: PerformanceTier,

pub enum PerformanceTier {

    Premium,

    Enterprise,

pub struct SecurityRequirements {

    pub required_encryption: Vec<String>,

    pub required_signatures: Vec<String>,

    pub required_auth_methods: Vec<String>,

    pub required_certifications: Vec<String>,

    pub min_key_lengths: HashMap<String, u32>,

    pub required_protocols: Vec<String>,

    pub audit_requirements: AuditRequirements,

    pub compliance_requirements: Vec<String>,

pub struct NetworkRequirements {

    pub required_bandwidth_bps: u64,

    pub max_network_latency_ms: u64,

    pub required_security_features: Vec<String>,

    pub transport_protocols: Vec<String>,

    pub port_requirements: Vec<PortRequirement>,

pub struct PortRequirement {

    pub port: u16,

    pub protocol: String,

    pub purpose: String,

    pub required: bool,

pub struct CompatibilityRequirements {

    pub supported_os: Vec<String>,

    pub supported_architectures: Vec<String>,

    pub required_versions: HashMap<String, String>,

    pub incompatible_systems: Vec<String>,

    pub required_runtimes: Vec<String>,

pub struct ResourceRequirements {

    pub min_memory_bytes: u64,

    pub min_storage_bytes: u64,

    pub min_cpu_cores: u32,

    pub gpu_requirements: Option<GpuRequirements>,

    pub hardware_features: Vec<String>,

    pub priority: ResourcePriority,

pub struct GpuRequirements {

    pub min_gpu_memory_bytes: u64,

    pub min_compute_capability: String,

    pub supported_vendors: Vec<String>,

    pub required_features: Vec<String>,

pub enum ResourcePriority {

    Low,

    Normal,

pub struct AuditRequirements {

    pub required_events: Vec<String>,

    pub retention_days: u32,

    pub required_formats: Vec<String>,

    pub encryption_required: bool,

    pub realtime_required: bool,

#[derive(Debug)]
pub struct CapabilityManager {

    capabilities: HashMap<Uuid, BearDogCapability>,

    by_category: HashMap<BearDogCapabilityCategory, HashSet<Uuid>>,

    by_name: HashMap<String, Uuid>,

    active_requirements: HashMap<Uuid, CapabilityRequirements>,}

impl CapabilityManager {

    pub fn new() -> Self {
        info!("🎯 Initializing BearDog Capability Manager");
        
        Self {
            capabilities: HashMap::with_capacity(16),
            by_category: HashMap::with_capacity(16),
            by_name: HashMap::with_capacity(16),
            active_requirements: HashMap::with_capacity(16),
        }
    }

    pub fn register_capability(&mut self, capability: BearDogCapability) -> BearDogResult<()> {
        info!("📝 Registering capability: {}", capability.name);

        if self.by_name.contains_key(&capability.name) {
            return Err(BearDogError::validation(format!(
                "Capability with name '{}' already exists", capability.name
            )));
        let capability_id = capability.capability_id;
        let name = capability.name.clone();
        let category = capability.category.clone();

        self.by_name.insert(name, capability_id);
        self.by_category
            .entry(category)
            .or_insert_with(HashSet::new)
            .insert(capability_id);

        self.capabilities.insert(capability_id, capability);
        debug!("✅ Capability registered: {}", capability_id);
        Ok(())

    pub fn get_capability(&self, capability_id: Uuid) -> BearDogResult<&BearDogCapability> {
        self.capabilities.get(&capability_id)
            .ok_or_else(|| BearDogError::not_found(format_args!("Capability not found: {}", capability_id).to_string()))

    pub fn get_capability_by_name(&self, name: &str) -> BearDogResult<&BearDogCapability> {
        let capability_id = self.by_name.get(name)
            .ok_or_else(|| BearDogError::not_found(format_args!("Capability not found: {}", name).to_string()))?;
        self.get_capability(*capability_id)

    pub fn list_capabilities_by_category(&self, category: &BearDogCapabilityCategory) -> Vec<&BearDogCapability> {
        if let Some(capability_ids) = self.by_category.get(category) {
            capability_ids.iter()
                .filter_map(|id| self.capabilities.get(id))
                .collect()
        } else {
            Vec::new()

    pub fn list_all_capabilities(&self) -> Vec<&BearDogCapability> {
        self.capabilities.values().collect()

    pub fn match_requirements(&self, requirements: &CapabilityRequirements) -> BearDogResult<Vec<&BearDogCapability>> {
        let mut matches = Vec::new();
        for capability in self.capabilities.values() {
            if self.capability_matches_requirements(capability, requirements)? {
                matches.push(capability);
            }
        Ok(matches)

    fn capability_matches_requirements(
        &self,
        capability: &BearDogCapability,
        requirements: &CapabilityRequirements,
    ) -> BearDogResult<bool> {

        if capability.security_level < requirements.min_security_level {
            return Ok(false);

        if !self.performance_requirements_met(&capability.performance_requirements, &requirements.performance_requirements) {

        if capability.status != CapabilityStatus::Active {

        Ok(true)

    fn performance_requirements_met(
        capability_perf: &PerformanceRequirements,
        required_perf: &PerformanceRequirements,
    ) -> bool {
        capability_perf.max_latency_ms <= required_perf.max_latency_ms &&
        capability_perf.min_throughput_ops_per_sec >= required_perf.min_throughput_ops_per_sec &&
        capability_perf.required_availability_percent >= required_perf.required_availability_percent

    pub fn update_capability_status(&mut self, capability_id: Uuid, status: CapabilityStatus) -> BearDogResult<()> {
        if let Some(capability) = self.capabilities.get_mut(&capability_id) {
            capability.status = status;
            capability.updated_at = SystemTime::now();
            
            debug!("🔄 Updated capability status: {} -> {:?}", capability_id, capability.status);
            Ok(())
            Err(BearDogError::not_found(format_args!("Capability not found: {}", capability_id).to_string()))

    pub fn health_check(&self) -> BearDogResult<HashMap<Uuid, CapabilityStatus>> {
        debug!("🏥 Performing capability health check");
        let mut health_status = HashMap::with_capacity(16);
        for (id, capability) in &self.capabilities {
            health_status.insert(*id, capability.status.clone());
        Ok(health_status)
