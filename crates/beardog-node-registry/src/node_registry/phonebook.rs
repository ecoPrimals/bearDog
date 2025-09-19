// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info};
use super::types::{
    PhonebookConfig, ServiceAdvertisement, NodeInfo, ServiceHealthStatus,
    TrustLevel,
};
use crate::{{BearDogError}};

pub struct PhonebookService {

    config: PhonebookConfig,

    registered_nodes: Arc<RwLock<HashMap<String, PhonebookEntry>>>,

    service_advertisements: Arc<RwLock<HashMap<String, ServiceAdvertisement>>>,

    status: Arc<RwLock<PhonebookStatus>>,

    served_regions: Vec<String>,

    phonebook_network: Arc<RwLock<HashMap<String, PhonebookPeer>>>,
}

#[derive(Debug, Clone)]
    /// The registered at value
    pub registered_at: SystemTime,

    /// The last heartbeat value
    pub last_heartbeat: SystemTime,

    /// Current status of the health
    pub health_status: ServiceHealthStatus,

    /// The region value
    pub region: String,

    /// Collection of protocols
    pub protocols: Vec<String>,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,

    /// The registration source value
    pub registration_source: RegistrationSource,

#[derive(Debug, Clone)]
    /// Number of registered_nodes
    pub registered_nodes: usize,

    /// Number of active_advertisements
    pub active_advertisements: usize,

    /// Number of total_discovery_requests
    pub total_discovery_requests: u64,

    /// The last cleanup value
    pub last_cleanup: chrono::DateTime<chrono::Utc>,

    /// Number of connected_peers
    pub connected_peers: usize,

pub struct PhonebookPeer {


    pub peer_id: String,

    /// Collection of endpoints
    pub endpoints: Vec<String>,

    /// Collection of regions
    pub regions: Vec<String>,

    /// Collection of capabilities
    pub capabilities: Vec<String>,

    /// Current status of the connection
    pub connection_status: PeerConnectionStatus,

    /// The last seen value
    pub last_seen: SystemTime,

    /// The trust level value
    pub trust_level: TrustLevel,

pub enum PeerConnectionStatus {


    /// State indicating connected
    Connected,


    /// Currently connecting
    Connecting,


    /// State indicating disconnected
    Disconnected,


    Failed,}
    Failed,}
    Failed,}

impl PhonebookService {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: PhonebookConfig) -> Result<Self, BearDogError> {
        if !config.enabled {
            return Err(BearDogError::config("Phonebook service is disabled"));
        }
        info!(
            "📞 Initializing Phonebook Service on {}:{}",
            config.bind_address, config.port
        );
        let status = Arc::new(RwLock::new(true,
            registered_nodes: 0,
            active_advertisements: 0,
            total_discovery_requests: 0,
            last_cleanup: chrono::Utc::now(ServiceHealthStatus::Healthy,
            connected_peers: 0,
        }));
        let service = Self {
            config,
            registered_nodes: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            service_advertisements: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            status,
            served_regions: vec!["global".to_string()],
            phonebook_network: Arc::new(RwLock::new(HashMap::with_capacity(NodeInfo,
        region: &str,
    ) -> Result<String, BearDogError> {
            "📝 Registering node "{}" from region "{}"",
            node_info.id, region

        {
            let nodes = self.registered_nodes.read();
            if nodes.len() >= self.config.max_tracked_nodes {
                return Err(BearDogError::validation(
                    "registration",
                    "Maximum number of tracked nodes reached",
                ));
            }

        let entry = PhonebookEntry {
            node_info: node_info.clone(),
            registered_at: SystemTime::now(),
            last_heartbeat: SystemTime::now(),
            region,
            protocols: vec!["https".to_string(), "beardog-secure".to_string()],
            metadata: HashMap::with_capacity(RegistrationSource::Direct,

            let mut nodes = self.registered_nodes.write();
            nodes.insert(node_info.id.clone(), entry);

            let mut status = self.status.write();
            status.registered_nodes += 1;

        let registration_id = format!("reg_{}", uuid::Uuid::new_v4({}",
            node_info.id, registration_id
        Ok(registration_id)

/// Heartbeat operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn heartbeat(&self, node_id: &str) -> Result<(), BearDogError> {
        let updated = {
            if let Some(entry) = nodes.get_mut(node_id) {
                entry.last_heartbeat = SystemTime::now();
                entry.health_status = ServiceHealthStatus::Healthy;
                true
            } else {
                false
        if updated {
            debug!("💓 Heartbeat received from node: {}", node_id);
            Ok(())
        } else {
            Err(BearDogError::not_found(&NodeDiscoveryCriteria,
    ) -> Result<Vec<NodeInfo>, BearDogError>> {
        debug!("🔍 Discovering nodes with criteria: {:?}", criteria);

            status.total_discovery_requests += 1;
        let nodes = self.registered_nodes.read();
        let mut matching_nodes = Vec::new(ServiceAdvertisement,
    ) -> Result<(), BearDogError> {
        info!("📢 Advertising service: {}", advertisement.service_name);

            let mut ads = self.service_advertisements.write(&ServiceDiscoveryCriteria,
    ) -> Result<Vec<ServiceAdvertisement>, BearDogError>> {
        debug!("🔍 Discovering services with criteria: {:?}", criteria);
        let ads = self.service_advertisements.read();
        let mut matching_services = Vec::new();
        for ad in ads.values() {
            if self.service_matches_criteria(ad, criteria) {
                matching_services.push(&ad);
            if matching_services.len() >= criteria.max_results {
        Ok(matching_services)

/// Get Statistics operation.
    /// Gets statistics
    /// Gets statistics
    pub fn get_statistics(&self) -> PhonebookStatus {
        self.status.read().clone()

/// Connect To Phonebook operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn connect_to_phonebook(&self, peer_info: PhonebookPeer) -> Result<(), BearDogError> {
        info!("📞 Connecting to phonebook peer: {}", peer_info.peer_id);

            let mut peers = self.phonebook_network.write();
            peers.insert(peer_info.peer_id, peer_info);
            status.connected_peers += 1;

/// Unregister Node operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn unregister_node(&self, node_id: &str) -> Result<(), BearDogError> {
        info!("🗑️ Unregistering node: {}", node_id);
        let removed = {
            nodes.remove(node_id).is_some()
        if removed {
            status.registered_nodes = status.registered_nodes.saturating_sub(1);

/// Cleanup Expired Entries operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Cleans up expired_entries
    /// Cleans up expired_entries
    pub fn cleanup_expired_entries(&self) -> Result<(), BearDogError> {
        debug!("🧹 Cleaning up expired phonebook entries");
        let now = SystemTime::now();
        let mut expired_nodes = Vec::new();
        let mut expired_services = Vec::new();

            for (node_id, entry) in nodes.iter() {
                if let Ok(elapsed) = now.duration_since(entry.last_heartbeat) {
                    if elapsed > self.config.node_entry_ttl {
                        expired_nodes.push(&node_id);
                    }
                }

            let services = self.service_advertisements.read();
            for (service_id, ad) in services.iter() {

                if let Ok(elapsed) = now
                    .duration_since(SystemTime::UNIX_EPOCH + Duration::from_secs(ad.ttl.as_secs()))
                {
                    if elapsed > ad.ttl {
                        expired_services.push(&service_id);

        if !expired_nodes.is_empty() {
            for node_id in &expired_nodes {
                nodes.remove(node_id);

        if !expired_services.is_empty() {
            let mut services = self.service_advertisements.write();
            for service_id in &expired_services {
                services.remove(service_id);
            status.registered_nodes = status.registered_nodes.saturating_sub(expired_nodes.len());
            status.active_advertisements = status
                .active_advertisements
                .saturating_sub(expired_services.len());
            status.last_cleanup = chrono::Utc::now(&PhonebookEntry,
    ) -> bool {

        if let Some(ref region) = criteria.region {
            if entry.region != *region {
                return false;

        if let Some(ref node_type) = criteria.node_type {
            if entry.node_info.node_type != *node_type {

        for required_cap in &criteria.required_capabilities {
            if !entry.node_info.capabilities.contains(required_cap) {

        if entry.node_info.trust_level < criteria.min_trust_level {
            return false;

        if entry.health_status == ServiceHealthStatus::Unavailable {
        true
    fn service_matches_criteria(&ServiceAdvertisement,

        if let Some(ref service_type) = criteria.service_type {
            if ad.service_type != *service_type {
            if ad.region != *region {
            if !ad.capabilities.contains(required_cap) {
        if ad.health_status == ServiceHealthStatus::Unavailable {

pub struct NodeDiscoveryCriteria {
    /// Optional region
    pub region: Option<String>,

    /// Optional node type
    pub node_type: Option<String>,

    /// Collection of required capabilities
    pub required_capabilities: Vec<String>,

    /// The min trust level value
    pub min_trust_level: TrustLevel,

    /// Number of max_results
    pub max_results: usize,

pub struct ServiceDiscoveryCriteria {

    /// Optional service type
    pub service_type: Option<String>,}

impl Default for NodeDiscoveryCriteria {}

    fn default(None,
            node_type: None,
            required_capabilities: Vec::new(TrustLevel::Unknown,
            max_results: 50,
impl Default for ServiceDiscoveryCriteria {
            service_type: None,}

impl Default for PhonebookStatus {
            active: false,
            health_status: ServiceHealthStatus::Unknown,
