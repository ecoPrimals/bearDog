

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

pub struct PhonebookService {

    config: PhonebookConfig,

    registered_nodes: Arc<RwLock<HashMap<String, PhonebookEntry>>>,

    service_advertisements: Arc<RwLock<HashMap<String, ServiceAdvertisement>>>,

    status: Arc<RwLock<PhonebookStatus>>,

    served_regions: Vec<String>,

    phonebook_network: Arc<RwLock<HashMap<String, PhonebookPeer>>>,
}

#[derive(Debug, Clone)]
pub struct PhonebookEntry {

    pub node_info: NodeInfo,

    pub registered_at: SystemTime,

    pub last_heartbeat: SystemTime,

    pub health_status: ServiceHealthStatus,

    pub region: String,

    pub protocols: Vec<String>,

    pub metadata: HashMap<String, String>,

    pub registration_source: RegistrationSource,

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationSource {

    Direct,

    Federation,

    DHT,

    Manual,

    NetworkScan,

pub struct PhonebookStatus {

    pub active: bool,

    pub registered_nodes: usize,

    pub active_advertisements: usize,

    pub total_discovery_requests: u64,

    pub last_cleanup: chrono::DateTime<chrono::Utc>,

    pub connected_peers: usize,

pub struct PhonebookPeer {

    pub peer_id: String,

    pub endpoints: Vec<String>,

    pub regions: Vec<String>,

    pub capabilities: Vec<String>,

    pub connection_status: PeerConnectionStatus,

    pub last_seen: SystemTime,

    pub trust_level: TrustLevel,

pub enum PeerConnectionStatus {

    Connected,

    Connecting,

    Disconnected,

    Failed,}

impl PhonebookService {

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
            registered_nodes: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            service_advertisements: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            status,
            served_regions: vec!["global".to_string()],
            phonebook_network: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        };

        service.start_background_tasks().await?;
        info!("✅ Phonebook Service initialized successfully");
        Ok(service)
    }

    pub async fn register_node(
        &self,
        node_info: NodeInfo,
        region: &str,
    ) -> BearDogResult<String> {
            "📝 Registering node '{}' from region '{}'",
            node_info.id, region

        {
            let nodes = self.registered_nodes.read().await;
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
            metadata: HashMap::with_capacity(16),
            registration_source: RegistrationSource::Direct,

            let mut nodes = self.registered_nodes.write().await;
            nodes.insert(node_info.id.clone(), entry);

            let mut status = self.status.write().await;
            status.registered_nodes += 1;

        let registration_id = format_args!("reg_{}", uuid::Uuid::new_v4().to_string());
            "✅ Node '{}' registered with ID: {}",
            node_info.id, registration_id
        Ok(registration_id)

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

    pub async fn discover_nodes(
        criteria: &NodeDiscoveryCriteria,
    ) -> BearDogResult<Vec<NodeInfo>> {
        debug!("🔍 Discovering nodes with criteria: {:?}", criteria);

            status.total_discovery_requests += 1;
        let nodes = self.registered_nodes.read().await;
        let mut matching_nodes = Vec::new();
        for entry in nodes.values() {
            if self.entry_matches_criteria(entry, criteria) {
                matching_nodes.push(entry.node_info.clone());

            if matching_nodes.len() >= criteria.max_results {
                break;

        matching_nodes.sort_by(|a, b| {

            b.trust_level.cmp(&a.trust_level)
        });
            "🔍 Found {} nodes matching discovery criteria",
            matching_nodes.len()
        Ok(matching_nodes)

    pub async fn advertise_service(
        advertisement: ServiceAdvertisement,
    ) -> BearDogResult<()> {
        info!("📢 Advertising service: {}", advertisement.service_name);

            let mut ads = self.service_advertisements.write().await;
            ads.insert(advertisement.service_id.clone(), advertisement);
            status.active_advertisements += 1;
        Ok(())

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

    pub async fn get_statistics(&self) -> PhonebookStatus {
        self.status.read().await.clone()

    pub async fn connect_to_phonebook(&self, peer_info: PhonebookPeer) -> BearDogResult<()> {
        info!("📞 Connecting to phonebook peer: {}", peer_info.peer_id);

            let mut peers = self.phonebook_network.write().await;
            peers.insert(peer_info.peer_id.clone(), peer_info);
            status.connected_peers += 1;

    pub async fn unregister_node(&self, node_id: &str) -> BearDogResult<()> {
        info!("🗑️ Unregistering node: {}", node_id);
        let removed = {
            nodes.remove(node_id).is_some()
        if removed {
            status.registered_nodes = status.registered_nodes.saturating_sub(1);

    pub async fn cleanup_expired_entries(&self) -> BearDogResult<()> {
        debug!("🧹 Cleaning up expired phonebook entries");
        let now = SystemTime::now();
        let mut expired_nodes = Vec::new();
        let mut expired_services = Vec::new();

            for (node_id, entry) in nodes.iter() {
                if let Ok(elapsed) = now.duration_since(entry.last_heartbeat) {
                    if elapsed > self.config.node_entry_ttl {
                        expired_nodes.push(node_id.clone());
                    }
                }

            let services = self.service_advertisements.read().await;
            for (service_id, ad) in services.iter() {

                if let Ok(elapsed) = now
                    .duration_since(SystemTime::UNIX_EPOCH + Duration::from_secs(ad.ttl.as_secs()))
                {
                    if elapsed > ad.ttl {
                        expired_services.push(service_id.clone());

        if !expired_nodes.is_empty() {
            for node_id in &expired_nodes {
                nodes.remove(node_id);

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

    async fn start_background_tasks(&self) -> BearDogResult<()> {

    fn entry_matches_criteria(
        entry: &PhonebookEntry,
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
    fn service_matches_criteria(
        ad: &ServiceAdvertisement,

        if let Some(ref service_type) = criteria.service_type {
            if ad.service_type != *service_type {
            if ad.region != *region {
            if !ad.capabilities.contains(required_cap) {
        if ad.health_status == ServiceHealthStatus::Unavailable {

pub struct NodeDiscoveryCriteria {
    pub region: Option<String>,

    pub node_type: Option<String>,

    pub required_capabilities: Vec<String>,

    pub min_trust_level: TrustLevel,

    pub max_results: usize,

pub struct ServiceDiscoveryCriteria {

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
