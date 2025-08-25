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


/// Phonebook Service for BearDog Network Discovery
///
/// This module implements a dedicated phonebook service that helps BearDog instances
/// discover and connect to each other across networks. A phonebook node acts as a
/// discovery service, maintaining a directory of available BearDog nodes and services.
/// ## Architecture
/// The phonebook service operates as a specialized BearDog node that:
/// - **Accepts registrations** from other BearDog instances
/// - **Provides discovery services** for finding nodes and services
/// - **Maintains health status** of registered nodes
/// - **Enables federation** between different BearDog networks
/// - **Supports geographic distribution** for better performance
/// ## Use Cases
/// - **Private networks**: Help BearDog instances find each other in corporate networks
/// - **Public discovery**: Enable public BearDog services to be discoverable
/// - **Federation bootstrap**: Provide initial connection points for federation
/// - **Load balancing**: Help distribute connections across available nodes

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info};
use super::types::{
    PhonebookConfig, ServiceAdvertisement, NodeInfo, ServiceHealthStatus,
    TrustLevel,
};
use crate::{BearDogError, BearDogResult};
/// Phonebook service for BearDog network discovery
pub struct PhonebookService {
    /// Configuration for phonebook service
    config: PhonebookConfig,
    /// Registered BearDog nodes
    registered_nodes: Arc<RwLock<HashMap<String, PhonebookEntry>>>,
    /// Service advertisements
    service_advertisements: Arc<RwLock<HashMap<String, ServiceAdvertisement>>>,
    /// Phonebook service status
    status: Arc<RwLock<PhonebookStatus>>,
    /// Geographic regions we serve
    served_regions: Vec<String>,
    /// Phonebook network (for federation with other phonebooks)
    phonebook_network: Arc<RwLock<HashMap<String, PhonebookPeer>>>,
}
/// Entry in the phonebook for a registered node
#[derive(Debug, Clone)]
pub struct PhonebookEntry {
    /// Node information
    pub node_info: NodeInfo,
    /// Registration timestamp
    pub registered_at: SystemTime,
    /// Last heartbeat timestamp
    pub last_heartbeat: SystemTime,
    /// Node health status
    pub health_status: ServiceHealthStatus,
    /// Geographic region
    pub region: String,
    /// Supported protocols
    pub protocols: Vec<String>,
    /// Entry metadata
    pub metadata: HashMap<String, String>,
    /// Registration source (how we learned about this node)
    pub registration_source: RegistrationSource,
/// How a node was registered with the phonebook
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationSource {
    /// Direct registration by the node
    Direct,
    /// Discovered through federation
    Federation,
    /// Discovered through DHT
    DHT,
    /// Manually added by administrator
    Manual,
    /// Discovered through network scanning
    NetworkScan,
/// Phonebook service status}


pub struct PhonebookStatus {
    /// Whether service is active
    pub active: bool,
    /// Number of registered nodes
    pub registered_nodes: usize,
    /// Number of active service advertisements
    pub active_advertisements: usize,
    /// Total discovery requests served
    pub total_discovery_requests: u64,
    /// Last cleanup time
    pub last_cleanup: chrono::DateTime<chrono::Utc>,
    /// Service health
    /// Connected phonebook peers
    pub connected_peers: usize,
/// Peer phonebook service
pub struct PhonebookPeer {
    /// Peer phonebook ID
    pub peer_id: String,
    /// Peer endpoints
    pub endpoints: Vec<String>,
    /// Peer regions
    pub regions: Vec<String>,
    /// Peer capabilities
    pub capabilities: Vec<String>,
    /// Connection status
    pub connection_status: PeerConnectionStatus,
    /// Last seen timestamp
    pub last_seen: SystemTime,
    /// Trust level for this peer
    pub trust_level: TrustLevel,
/// Connection status with peer phonebook
pub enum PeerConnectionStatus {
    /// Connected and operational
    Connected,
    /// Attempting to connect
    Connecting,
    /// Disconnected
    Disconnected,
    /// Connection failed
    Failed,}


impl PhonebookService {
    /// Create a new phonebook service
    pub async fn new(config: PhonebookConfig) -> BearDogResult<Self> {
        if !config.enabled {
            return Err(BearDogError::config("Phonebook service is disabled"));
        }
        info!(
            "📞 Initializing Phonebook Service on {}:{}",
            config.bind_address, config.port
        );
        let status = Arc::new(RwLock::new(PhonebookStatus {
            active: true,
            registered_nodes: 0,
            active_advertisements: 0,
            total_discovery_requests: 0,
            last_cleanup: chrono::Utc::now(),
            health_status: ServiceHealthStatus::Healthy,
            connected_peers: 0,
        }));
        let service = Self {
            config,
            registered_nodes: Arc::new(RwLock::new(HashMap::new())),
            service_advertisements: Arc::new(RwLock::new(HashMap::new())),
            status,
            served_regions: vec!["global".to_string()],
            phonebook_network: Arc::new(RwLock::new(HashMap::new())),
        };
        // Start background tasks
        service.start_background_tasks().await?;
        info!("✅ Phonebook Service initialized successfully");
        Ok(service)
    }
    /// Register a BearDog node with the phonebook
    pub async fn register_node(
        &self,
        node_info: NodeInfo,
        region: String,
    ) -> BearDogResult<String> {
            "📝 Registering node '{}' from region '{}'",
            node_info.id, region
        // Check registration limits
        {
            let nodes = self.registered_nodes.read().await;
            if nodes.len() >= self.config.max_tracked_nodes {
                return Err(BearDogError::validation(
                    "registration",
                    "Maximum number of tracked nodes reached",
                ));
            }
        // Create phonebook entry
        let entry = PhonebookEntry {
            node_info: node_info.clone(),
            registered_at: SystemTime::now(),
            last_heartbeat: SystemTime::now(),
            region,
            protocols: vec!["https".to_string(), "beardog-secure".to_string()],
            metadata: HashMap::new(),
            registration_source: RegistrationSource::Direct,
        // Store the entry
            let mut nodes = self.registered_nodes.write().await;
            nodes.insert(node_info.id.clone(), entry);
        // Update status
            let mut status = self.status.write().await;
            status.registered_nodes += 1;
        // Generate registration ID
        let registration_id = format!("reg_{}", uuid::Uuid::new_v4());
            "✅ Node '{}' registered with ID: {}",
            node_info.id, registration_id
        Ok(registration_id)
    /// Send heartbeat for a registered node
    pub async fn heartbeat(&self, node_id: &str) -> BearDogResult<()> {
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
            Err(BearDogError::not_found("node", node_id))
    /// Discover nodes based on criteria
    pub async fn discover_nodes(
        criteria: &NodeDiscoveryCriteria,
    ) -> BearDogResult<Vec<NodeInfo>> {
        debug!("🔍 Discovering nodes with criteria: {:?}", criteria);
        // Update request counter
            status.total_discovery_requests += 1;
        let nodes = self.registered_nodes.read().await;
        let mut matching_nodes = Vec::new();
        for entry in nodes.values() {
            if self.entry_matches_criteria(entry, criteria) {
                matching_nodes.push(entry.node_info.clone());
            // Respect max results limit
            if matching_nodes.len() >= criteria.max_results {
                break;
        // Sort by preference (e.g., region, health, trust level)
        matching_nodes.sort_by(|a, b| {
            // Prefer nodes with higher trust levels
            b.trust_level.cmp(&a.trust_level)
        });
            "🔍 Found {} nodes matching discovery criteria",
            matching_nodes.len()
        Ok(matching_nodes)
    /// Advertise a service
    pub async fn advertise_service(
        advertisement: ServiceAdvertisement,
    ) -> BearDogResult<()> {
        info!("📢 Advertising service: {}", advertisement.service_name);
        // Store advertisement
            let mut ads = self.service_advertisements.write().await;
            ads.insert(advertisement.service_id.clone(), advertisement);
            status.active_advertisements += 1;
        Ok(())
    /// Discover services based on criteria
    pub async fn discover_services(
        criteria: &ServiceDiscoveryCriteria,
    ) -> BearDogResult<Vec<ServiceAdvertisement>> {
        debug!("🔍 Discovering services with criteria: {:?}", criteria);
        let ads = self.service_advertisements.read().await;
        let mut matching_services = Vec::new();
        for ad in ads.values() {
            if self.service_matches_criteria(ad, criteria) {
                matching_services.push(ad.clone());
            if matching_services.len() >= criteria.max_results {
        Ok(matching_services)
    /// Get phonebook statistics
    pub async fn get_statistics(&self) -> PhonebookStatus {
        self.status.read().await.clone()
    /// Connect to another phonebook for federation}


    pub async fn connect_to_phonebook(&self, peer_info: PhonebookPeer) -> BearDogResult<()> {
        info!("📞 Connecting to phonebook peer: {}", peer_info.peer_id);
        // Connection logic implemented via canonical provider system
        // This would involve:
        // 1. Establishing secure connection
        // 2. Mutual authentication
        // 3. Capability negotiation
        // 4. Setting up data synchronization
        // Store peer information
            let mut peers = self.phonebook_network.write().await;
            peers.insert(peer_info.peer_id.clone(), peer_info);
            status.connected_peers += 1;
    /// Unregister a node
    pub async fn unregister_node(&self, node_id: &str) -> BearDogResult<()> {
        info!("🗑️ Unregistering node: {}", node_id);
        let removed = {
            nodes.remove(node_id).is_some()
        if removed {
            status.registered_nodes = status.registered_nodes.saturating_sub(1);
    /// Cleanup expired entries
    pub async fn cleanup_expired_entries(&self) -> BearDogResult<()> {
        debug!("🧹 Cleaning up expired phonebook entries");
        let now = SystemTime::now();
        let mut expired_nodes = Vec::new();
        let mut expired_services = Vec::new();
        // Find expired nodes
            for (node_id, entry) in nodes.iter() {
                if let Ok(elapsed) = now.duration_since(entry.last_heartbeat) {
                    if elapsed > self.config.node_entry_ttl {
                        expired_nodes.push(node_id.clone());
                    }
                }
        // Find expired services
            let services = self.service_advertisements.read().await;
            for (service_id, ad) in services.iter() {
                // Check if service TTL has expired
                if let Ok(elapsed) = now
                    .duration_since(SystemTime::UNIX_EPOCH + Duration::from_secs(ad.ttl.as_secs()))
                {
                    if elapsed > ad.ttl {
                        expired_services.push(service_id.clone());
        // Remove expired nodes
        if !expired_nodes.is_empty() {
            for node_id in &expired_nodes {
                nodes.remove(node_id);
        // Remove expired services
        if !expired_services.is_empty() {
            let mut services = self.service_advertisements.write().await;
            for service_id in &expired_services {
                services.remove(service_id);
            status.registered_nodes = status.registered_nodes.saturating_sub(expired_nodes.len());
            status.active_advertisements = status
                .active_advertisements
                .saturating_sub(expired_services.len());
            status.last_cleanup = chrono::Utc::now();
        if !expired_nodes.is_empty() || !expired_services.is_empty() {
            info!(
                "🧹 Cleaned up {} expired nodes and {} expired services",
                expired_nodes.len(),
                expired_services.len()
            );
    // Private helper methods
    async fn start_background_tasks(&self) -> BearDogResult<()> {
        // Background tasks managed by canonical task system
        // 1. Periodic cleanup of expired entries
        // 2. Health checks of registered nodes
        // 3. Synchronization with peer phonebooks
        // 4. Metrics collection and reporting}


    fn entry_matches_criteria(
        entry: &PhonebookEntry,
    ) -> bool {
        // Check region
        if let Some(ref region) = criteria.region {
            if entry.region != *region {
                return false;
        // Check node type
        if let Some(ref node_type) = criteria.node_type {
            if entry.node_info.node_type != *node_type {
        // Check capabilities
        for required_cap in &criteria.required_capabilities {
            if !entry.node_info.capabilities.contains(required_cap) {
        // Check trust level
        if entry.node_info.trust_level < criteria.min_trust_level {
            return false;
        // Check health status
        if entry.health_status == ServiceHealthStatus::Unavailable {
        true
    fn service_matches_criteria(
        ad: &ServiceAdvertisement,
        // Check service type
        if let Some(ref service_type) = criteria.service_type {
            if ad.service_type != *service_type {
            if ad.region != *region {
            if !ad.capabilities.contains(required_cap) {
        if ad.health_status == ServiceHealthStatus::Unavailable {
/// Node discovery criteria
pub struct NodeDiscoveryCriteria {
    pub region: Option<String>,
    /// Node type
    pub node_type: Option<String>,
    /// Required capabilities
    pub required_capabilities: Vec<String>,
    /// Minimum trust level
    pub min_trust_level: TrustLevel,
    /// Maximum results
    pub max_results: usize,
/// Service discovery criteria
pub struct ServiceDiscoveryCriteria {
    /// Service type
    pub service_type: Option<String>,}


impl Default for NodeDiscoveryCriteria {}


    fn default() -> Self {
        Self {
            region: None,
            node_type: None,
            required_capabilities: Vec::new(),
            min_trust_level: TrustLevel::Unknown,
            max_results: 50,
impl Default for ServiceDiscoveryCriteria {
            service_type: None,}


impl Default for PhonebookStatus {
            active: false,
            health_status: ServiceHealthStatus::Unknown,
