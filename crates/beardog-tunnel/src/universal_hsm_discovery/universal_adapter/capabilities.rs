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


/// BearDog Capabilities
///
/// **CANONICAL CAPABILITY SYSTEM** - Complete implementation for capability definitions and management
/// This module provides comprehensive capability management for external primal communication,
/// consolidating patterns from beardog-types capabilities and beardog-adapters capability management.

use super::core_types::*;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::capabilities::{CapabilityRequirements as CanonicalCapabilityRequirements, CapabilityType};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime};
use tracing::{debug, info, warn};
use uuid::Uuid;
/// **BEARDOG CAPABILITY** - Core capability representation for external primal communication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BearDogCapability {
    /// Unique capability identifier
    pub capability_id: Uuid,
    
    /// Capability name
    pub name: String,
    /// Capability category
    pub category: BearDogCapabilityCategory,
    /// Capability version
    pub version: String,
    /// Capability description
    pub description: String,
    /// Required security level
    pub security_level: SecurityLevel,
    /// Performance requirements
    pub performance_requirements: PerformanceRequirements,
    /// Security requirements
    pub security_requirements: SecurityRequirements,
    /// Capability status
    pub status: CapabilityStatus,
    /// Supported protocols
    pub supported_protocols: Vec<String>,
    /// Capability metadata
    pub metadata: HashMap<String, String>,
    /// Dependencies on other capabilities
    pub dependencies: Vec<Uuid>,
    /// Capability registration timestamp
    pub registered_at: SystemTime,
    /// Last updated timestamp
    pub updated_at: SystemTime,
}
/// **BEARDOG CAPABILITY CATEGORY** - Categories of BearDog capabilities
pub enum BearDogCapabilityCategory {
    /// Cryptographic operations
    Cryptography,
    /// Hardware Security Module operations
    HardwareSecurityModule,
    /// Key management operations
    KeyManagement,
    /// Digital signature operations
    DigitalSignature,
    /// Data encryption/decryption
    DataEncryption,
    /// Network communication
    NetworkCommunication,
    /// Service discovery
    ServiceDiscovery,
    /// Distributed operations
    DistributedOperations,
    /// Genetic algorithm operations
    GeneticOperations,
    /// AI/ML inference
    ArtificialIntelligence,
    /// Monitoring and observability
    Monitoring,
    /// Storage operations
    Storage,
    /// Authentication and authorization
    Authentication,
    /// Workflow orchestration
    WorkflowOrchestration,
    /// Custom capability
    Custom {
        /// Custom category name
        category_name: String,
    },
/// **CAPABILITY STATUS** - Status of a capability
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CapabilityStatus {
    /// Capability is active and available
    Active,
    /// Capability is inactive
    Inactive,
    /// Capability is under maintenance
    Maintenance,
    /// Capability is experimental
    Experimental,
    /// Capability has failed
    Failed,
/// **SECURITY LEVEL** - Security levels for capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]}


pub enum SecurityLevel {
    /// Public access (no authentication required)
    Public,
    /// Basic authentication required
    Basic,
    /// Standard security level
    Standard,
    /// High security level
    High,
    /// Critical security level
    Critical,
    /// Maximum security level
    Maximum,
/// **EXTERNAL PRIMAL COMMUNICATION REQUIREMENTS** - Requirements for communicating with external primals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirements {
    /// Required capabilities
    pub required_capabilities: Vec<BearDogCapability>,
    /// Optional capabilities
    pub optional_capabilities: Vec<BearDogCapability>,
    /// Minimum security level
    pub min_security_level: SecurityLevel,
    /// Network requirements
    pub network_requirements: NetworkRequirements,
    /// Compatibility requirements
    pub compatibility_requirements: CompatibilityRequirements,
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
/// **PERFORMANCE REQUIREMENTS** - Performance requirements for capabilities
pub struct PerformanceRequirements {
    /// Maximum acceptable latency in milliseconds
    pub max_latency_ms: u64,
    /// Minimum required throughput (operations per second)
    pub min_throughput_ops_per_sec: u64,
    /// Maximum memory usage in bytes
    pub max_memory_bytes: u64,
    /// Maximum CPU usage percentage
    pub max_cpu_percent: f64,
    /// Required availability percentage
    pub required_availability_percent: f64,
    /// Maximum response time in milliseconds
    pub max_response_time_ms: u64,
    /// Concurrent operation limit
    pub max_concurrent_operations: u32,
    /// Performance tier requirement
    pub performance_tier: PerformanceTier,
/// **PERFORMANCE TIER** - Performance tiers for capabilities
pub enum PerformanceTier {
    /// Basic performance tier
    /// Standard performance tier
    /// High performance tier
    /// Premium performance tier
    Premium,
    /// Enterprise performance tier
    Enterprise,
/// **SECURITY REQUIREMENTS** - Security requirements for capabilities}


pub struct SecurityRequirements {
    /// Required encryption algorithms
    pub required_encryption: Vec<String>,
    /// Required signature algorithms
    pub required_signatures: Vec<String>,
    /// Required authentication methods
    pub required_auth_methods: Vec<String>,
    /// Required security certifications
    pub required_certifications: Vec<String>,
    /// Minimum key length requirements
    pub min_key_lengths: HashMap<String, u32>,
    /// Required security protocols
    pub required_protocols: Vec<String>,
    /// Audit requirements
    pub audit_requirements: AuditRequirements,
    /// Compliance requirements
    pub compliance_requirements: Vec<String>,
/// **NETWORK REQUIREMENTS** - Network requirements for capabilities
pub struct NetworkRequirements {
    /// Supported network protocols
    /// Required bandwidth in bytes per second
    pub required_bandwidth_bps: u64,
    /// Maximum acceptable network latency in milliseconds
    pub max_network_latency_ms: u64,
    /// Required network security features
    pub required_security_features: Vec<String>,
    /// Supported transport protocols
    pub transport_protocols: Vec<String>,
    /// Port requirements
    pub port_requirements: Vec<PortRequirement>,
/// **PORT REQUIREMENT** - Network port requirements
pub struct PortRequirement {
    /// Port number
    pub port: u16,
    /// Protocol (TCP/UDP)
    pub protocol: String,
    /// Port purpose description
    pub purpose: String,
    /// Whether the port is required or optional
    pub required: bool,
/// **COMPATIBILITY REQUIREMENTS** - Compatibility requirements for capabilities
pub struct CompatibilityRequirements {
    /// Supported operating systems
    pub supported_os: Vec<String>,
    /// Supported architectures
    pub supported_architectures: Vec<String>,
    /// Required software versions
    pub required_versions: HashMap<String, String>,
    /// Incompatible systems
    pub incompatible_systems: Vec<String>,
    /// Required runtime environments
    pub required_runtimes: Vec<String>,
/// **RESOURCE REQUIREMENTS** - Resource requirements for capabilities
pub struct ResourceRequirements {
    /// Minimum RAM in bytes
    pub min_memory_bytes: u64,
    /// Minimum storage in bytes
    pub min_storage_bytes: u64,
    /// Minimum CPU cores
    pub min_cpu_cores: u32,
    /// Required GPU capabilities
    pub gpu_requirements: Option<GpuRequirements>,
    /// Required hardware features
    pub hardware_features: Vec<String>,
    /// Resource allocation priority
    pub priority: ResourcePriority,
/// **GPU REQUIREMENTS** - GPU requirements for capabilities
pub struct GpuRequirements {
    /// Minimum GPU memory in bytes
    pub min_gpu_memory_bytes: u64,
    /// Required GPU compute capability
    pub min_compute_capability: String,
    /// Supported GPU vendors
    pub supported_vendors: Vec<String>,
    /// Required GPU features
    pub required_features: Vec<String>,
/// **RESOURCE PRIORITY** - Resource allocation priority
pub enum ResourcePriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    /// Critical priority
/// **AUDIT REQUIREMENTS** - Audit requirements for capabilities}


pub struct AuditRequirements {
    /// Required audit events
    pub required_events: Vec<String>,
    /// Audit retention period in days
    pub retention_days: u32,
    /// Required audit formats
    pub required_formats: Vec<String>,
    /// Audit encryption requirements
    pub encryption_required: bool,
    /// Real-time audit requirements
    pub realtime_required: bool,
/// **CAPABILITY MANAGER** - Manages BearDog capabilities
#[derive(Debug)]
pub struct CapabilityManager {
    /// Registered capabilities
    capabilities: HashMap<Uuid, BearDogCapability>,
    /// Capability index by category
    by_category: HashMap<BearDogCapabilityCategory, HashSet<Uuid>>,
    /// Capability index by name
    by_name: HashMap<String, Uuid>,
    /// Active capability requirements
    active_requirements: HashMap<Uuid, CapabilityRequirements>,}


impl CapabilityManager {
    /// **CREATE NEW CAPABILITY MANAGER**}


    pub fn new() -> Self {
        info!("🎯 Initializing BearDog Capability Manager");
        
        Self {
            capabilities: HashMap::new(),
            by_category: HashMap::new(),
            by_name: HashMap::new(),
            active_requirements: HashMap::new(),
        }
    }
    /// **REGISTER CAPABILITY** - Register a new capability
    pub fn register_capability(&mut self, capability: BearDogCapability) -> BearDogResult<()> {
        info!("📝 Registering capability: {}", capability.name);
        // Check for name conflicts
        if self.by_name.contains_key(&capability.name) {
            return Err(BearDogError::validation(format!(
                "Capability with name '{}' already exists", capability.name
            )));
        let capability_id = capability.capability_id;
        let name = capability.name.clone();
        let category = capability.category.clone();
        // Update indices
        self.by_name.insert(name, capability_id);
        self.by_category
            .entry(category)
            .or_insert_with(HashSet::new)
            .insert(capability_id);
        // Store capability
        self.capabilities.insert(capability_id, capability);
        debug!("✅ Capability registered: {}", capability_id);
        Ok(())
    /// **GET CAPABILITY** - Get capability by ID
    pub fn get_capability(&self, capability_id: Uuid) -> BearDogResult<&BearDogCapability> {
        self.capabilities.get(&capability_id)
            .ok_or_else(|| BearDogError::not_found(format!("Capability not found: {}", capability_id)))
    /// **GET CAPABILITY BY NAME** - Get capability by name
    pub fn get_capability_by_name(&self, name: &str) -> BearDogResult<&BearDogCapability> {
        let capability_id = self.by_name.get(name)
            .ok_or_else(|| BearDogError::not_found(format!("Capability not found: {}", name)))?;
        self.get_capability(*capability_id)
    /// **LIST CAPABILITIES BY CATEGORY** - List all capabilities in a category
    pub fn list_capabilities_by_category(&self, category: &BearDogCapabilityCategory) -> Vec<&BearDogCapability> {
        if let Some(capability_ids) = self.by_category.get(category) {
            capability_ids.iter()
                .filter_map(|id| self.capabilities.get(id))
                .collect()
        } else {
            Vec::new()
    /// **LIST ALL CAPABILITIES** - List all registered capabilities
    pub fn list_all_capabilities(&self) -> Vec<&BearDogCapability> {
        self.capabilities.values().collect()
    /// **MATCH REQUIREMENTS** - Find capabilities that match requirements}


    pub fn match_requirements(&self, requirements: &CapabilityRequirements) -> BearDogResult<Vec<&BearDogCapability>> {
        let mut matches = Vec::new();
        for capability in self.capabilities.values() {
            if self.capability_matches_requirements(capability, requirements)? {
                matches.push(capability);
            }
        Ok(matches)
    /// **CAPABILITY MATCHES REQUIREMENTS** - Check if a capability matches requirements
    fn capability_matches_requirements(
        &self,
        capability: &BearDogCapability,
        requirements: &CapabilityRequirements,
    ) -> BearDogResult<bool> {
        // Check security level
        if capability.security_level < requirements.min_security_level {
            return Ok(false);
        // Check performance requirements
        if !self.performance_requirements_met(&capability.performance_requirements, &requirements.performance_requirements) {
        // Check status
        if capability.status != CapabilityStatus::Active {
        // Additional matching logic would go here
        Ok(true)
    /// **PERFORMANCE REQUIREMENTS MET** - Check if performance requirements are met}


    fn performance_requirements_met(
        capability_perf: &PerformanceRequirements,
        required_perf: &PerformanceRequirements,
    ) -> bool {
        capability_perf.max_latency_ms <= required_perf.max_latency_ms &&
        capability_perf.min_throughput_ops_per_sec >= required_perf.min_throughput_ops_per_sec &&
        capability_perf.required_availability_percent >= required_perf.required_availability_percent
    /// **UPDATE CAPABILITY STATUS** - Update the status of a capability
    pub fn update_capability_status(&mut self, capability_id: Uuid, status: CapabilityStatus) -> BearDogResult<()> {
        if let Some(capability) = self.capabilities.get_mut(&capability_id) {
            capability.status = status;
            capability.updated_at = SystemTime::now();
            
            debug!("🔄 Updated capability status: {} -> {:?}", capability_id, capability.status);
            Ok(())
            Err(BearDogError::not_found(format!("Capability not found: {}", capability_id)))
    /// **HEALTH CHECK** - Check health of all capabilities
    pub fn health_check(&self) -> BearDogResult<HashMap<Uuid, CapabilityStatus>> {
        debug!("🏥 Performing capability health check");
        let mut health_status = HashMap::new();
        for (id, capability) in &self.capabilities {
            health_status.insert(*id, capability.status.clone());
        Ok(health_status)
