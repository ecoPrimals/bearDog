

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
use uuid::Uuid;
use super::models::{
    ActiveShare, ResourceAmount, ResourceType, ResourceUsageStats, SharingOffer, UsageDataPoint,
};

pub struct ResourceSharingEngine {

    active_offers: Arc<RwLock<HashMap<String, SharingOfferInternal>>>,

    active_shares: Arc<RwLock<HashMap<String, ActiveShareInternal>>>,

    friend_network: Arc<RwLock<HashMap<String, FriendNode>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SharingOfferInternal {
    pub offer_id: String,
    pub from_node_id: String,
    pub from_display_name: String,
    pub resource_type: ResourceType,
    pub resource_amount: ResourceAmount,
    pub personal_message: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub terms: Vec<String>,
    pub consent_required: bool,
    pub friends_notified: Vec<String>,
    pub workflow_id: Option<String>,
struct ActiveShareInternal {
    pub share_id: String,
    pub friend_node_id: String,
    pub friend_display_name: String,
    pub usage_stats: ResourceUsageStatsInternal,
    pub started_at: DateTime<Utc>,
    pub revocable: bool,
    pub consent_record_id: String,
struct ResourceUsageStatsInternal {
    pub allocated: u64,
    pub current_usage: u64,
    pub peak_usage: u64,
    pub usage_history: Vec<UsageDataPointInternal>,
    pub fair_share_score: f64,
struct UsageDataPointInternal {
    pub timestamp: DateTime<Utc>,
    pub usage_amount: u64,
    pub efficiency_score: f64,
struct FriendNode {
    pub node_id: String,
    pub display_name: String,
    pub public_key: String,
    pub last_seen: DateTime<Utc>,
    pub trust_score: f64,
    pub shared_resources: Vec<ResourceType>,
    pub usage_history: Vec<String>, // Share IDs
impl Default for ResourceSharingEngine {}

    fn default() -> Self {
        Self::new()
    }
impl ResourceSharingEngine {

    pub fn new() -> Self {
        info!("🤝 Initializing peer-to-peer resource sharing engine");
        Self {
            active_offers: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            active_shares: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            friend_network: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }

    pub async fn create_sharing_offer(
        &self,
        from_node_id: &str,
        from_display_name: &str,
        resource_type: ResourceType,
        resource_amount: ResourceAmount,
        personal_message: &str,
        expires_hours: Option<i64>,
    ) -> Result<SharingOffer, Box<dyn std::error::Error + Send + Sync>> {
        info!(
            "🎁 Creating resource sharing offer from {}",
            from_display_name
        );
        let offer_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = expires_hours.map(|h| now + Duration::hours(h));

        let temp_offer = SharingOffer {
            offer_id: offer_id.clone(),
            from_node_id: from_node_id.clone(),
            from_display_name: from_display_name.clone(),
            resource_type: resource_type.clone(),
            resource_amount: resource_amount.clone(),
            personal_message: personal_message.clone(),
            created_at: now.to_rfc3339(),
            expires_at: expires_at.map(|dt| dt.to_rfc3339()),
            terms: self.generate_sharing_terms(&resource_type),
        };

        let workflow_id = self.create_consent_workflow(&temp_offer).await?;
        let offer_internal = SharingOfferInternal {
            created_at: now,
            expires_at,
            consent_required: true,
            friends_notified: Vec::new(),
            workflow_id: Some(workflow_id),

        {
            let mut offers = self.active_offers.write().await;
            offers.insert(offer_id.clone(), offer_internal);

        self.notify_friends_of_offer(&offer_id).await?;

        let terms = self.generate_sharing_terms(&resource_type);
        Ok(SharingOffer {
            offer_id,
            from_node_id,
            from_display_name,
            resource_type,
            resource_amount,
            personal_message,
            terms,
        })

    pub async fn accept_sharing_offer(
        offer_id: &str,
        _accepting_node_id: &str,
        _accepting_display_name: &str,
    ) -> Result<ActiveShare, Box<dyn std::error::Error + Send + Sync>> {
        info!("✅ Accepting resource sharing offer: {}", offer_id);

        let offer = {
            let offers = self.active_offers.read().await;
            offers
                .get(offer_id)
                .cloned()
                .ok_or("Sharing offer not found")?

        if let Some(expires_at) = offer.expires_at {
            if Utc::now() > expires_at {
                return Err("Sharing offer has expired".into());
            }

        let share_id = Uuid::new_v4().to_string();
        let consent_record_id = Uuid::new_v4().to_string();
        let active_share_internal = ActiveShareInternal {
            share_id: share_id.clone(),
            friend_node_id: offer.from_node_id.clone(),
            friend_display_name: offer.from_display_name.clone(),
            resource_type: offer.resource_type.clone(),
            usage_stats: ResourceUsageStatsInternal {
                allocated: offer.resource_amount.maximum,
                current_usage: 0,
                peak_usage: 0,
                usage_history: Vec::new(),
                fair_share_score: 1.0,
            },
            started_at: Utc::now(),
            expires_at: offer.expires_at,
            revocable: true,
            consent_record_id: consent_record_id.clone(),

            let mut shares = self.active_shares.write().await;
            shares.insert(share_id.clone(), active_share_internal);

            offers.remove(offer_id);

        self.update_friend_trust_score(&offer.from_node_id, 0.1)
            .await?;
        info!("🎉 Resource sharing activated: {}", share_id);

        Ok(ActiveShare {
            share_id,
            friend_node_id: offer.from_node_id,
            friend_display_name: offer.from_display_name,
            resource_type: offer.resource_type,
            usage_stats: ResourceUsageStats {
            started_at: Utc::now().to_rfc3339(),
            expires_at: offer.expires_at.map(|dt| dt.to_rfc3339()),

    pub async fn list_available_offers(
    ) -> Result<Vec<SharingOffer>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("📋 Listing available sharing offers from friend network");
        let offers = self.active_offers.read().await;
        let mut result = Vec::new();
        for offer in offers.values() {

            if let Some(expires_at) = offer.expires_at {
                if Utc::now() > expires_at {
                    continue;
                }
            result.push(SharingOffer {
                offer_id: offer.offer_id.clone(),
                from_node_id: offer.from_node_id.clone(),
                from_display_name: offer.from_display_name.clone(),
                resource_type: offer.resource_type.clone(),
                resource_amount: offer.resource_amount.clone(),
                personal_message: offer.personal_message.clone(),
                created_at: offer.created_at.to_rfc3339(),
                expires_at: offer.expires_at.map(|dt| dt.to_rfc3339()),
                terms: offer.terms.clone(),
            });
        Ok(result)

    pub async fn list_active_shares(
    ) -> Result<Vec<ActiveShare>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("📊 Listing active resource shares");
        let shares = self.active_shares.read().await;
        for share in shares.values() {
            result.push(ActiveShare {
                share_id: share.share_id.clone(),
                friend_node_id: share.friend_node_id.clone(),
                friend_display_name: share.friend_display_name.clone(),
                resource_type: share.resource_type.clone(),
                usage_stats: ResourceUsageStats {
                    allocated: share.usage_stats.allocated,
                    current_usage: share.usage_stats.current_usage,
                    peak_usage: share.usage_stats.peak_usage,
                    usage_history: share
                        .usage_stats
                        .usage_history
                        .iter()
                        .map(|dp| UsageDataPoint {
                            timestamp: dp.timestamp.to_rfc3339(),
                            usage: dp.usage_amount,
                            usage_amount: dp.usage_amount,
                        })
                        .collect(),
                },
                started_at: share.started_at.to_rfc3339(),
                expires_at: share.expires_at.map(|dt| dt.to_rfc3339()),
                revocable: share.revocable,

    pub async fn revoke_share(
        share_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("🚫 Revoking resource share: {}", share_id);
        let mut shares = self.active_shares.write().await;
        if shares.remove(share_id).is_some() {
            info!("✅ Resource share successfully revoked: {}", share_id);
            Ok(())
        } else {
            Err("Resource share not found".into())

    pub async fn add_friend(
        node_id: &str,
        display_name: &str,
        public_key: &str,
        shared_resources: Vec<ResourceType>,
        info!("👥 Adding friend to network: {}", display_name);
        let friend = FriendNode {
            node_id: node_id.clone(),
            display_name,
            public_key,
            last_seen: Utc::now(),
            trust_score: 1.0, // Start with neutral trust
            shared_resources,
            usage_history: Vec::new(),
        let mut network = self.friend_network.write().await;
        network.insert(node_id, friend);
        Ok(())

    pub async fn update_usage_stats(
        current_usage: u64,
        debug!("📈 Updating usage statistics for share: {}", share_id);
        if let Some(share) = shares.get_mut(share_id) {
            share.usage_stats.current_usage = current_usage;
            if current_usage > share.usage_stats.peak_usage {
                share.usage_stats.peak_usage = current_usage;

            share
                .usage_stats
                .usage_history
                .push(UsageDataPointInternal {
                    timestamp: Utc::now(),
                    usage_amount: current_usage,
                    efficiency_score: self
                        .calculate_efficiency_score(current_usage, share.usage_stats.allocated),
                });

            share.usage_stats.fair_share_score =
                self.calculate_fair_share_score(&share.usage_stats);

    async fn create_consent_workflow(
        resource_offer: &SharingOffer,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        debug!("⚖️ Creating consent workflow for resource sharing");

        let workflow_id = Uuid::new_v4().to_string();
            "📋 Created sharing consent workflow {} for offer {}",
            workflow_id, resource_offer.offer_id
            "   Resource: {:?}, Friend: {}",
            resource_offer.resource_type, resource_offer.from_display_name
        Ok(workflow_id)

    fn generate_sharing_terms(&self, resource_type: &ResourceType) -> Vec<String> {
        let mut terms = vec![
            "Explicit consent required for all usage".to_string(),
            "Resources can be revoked at any time".to_string(),
            "Usage will be monitored for fair allocation".to_string(),
            "All data processed remains encrypted".to_string(),
        ];
        match resource_type {
            ResourceType::Compute { .. } => {
                terms.push("Compute resources subject to availability".to_string());
                terms.push("No cryptocurrency mining allowed".to_string());
            ResourceType::Storage { encrypted, .. } => {
                if *encrypted {
                    terms.push("All stored data is encrypted at rest".to_string());
                terms.push("No illegal content allowed".to_string());
            ResourceType::Network { .. } => {
                terms.push("Network usage subject to bandwidth limits".to_string());
                terms.push("No illegal network activities allowed".to_string());
        terms

    async fn notify_friends_of_offer(
            "📢 Notifying friend network about sharing offer: {}",
            offer_id

    async fn update_friend_trust_score(
        friend_id: &str,
        delta: f64,
        if let Some(friend) = network.get_mut(friend_id) {
            friend.trust_score = (friend.trust_score + delta).clamp(0.0, 5.0);
            friend.last_seen = Utc::now();

    fn calculate_efficiency_score(&self, used: u64, allocated: u64) -> f64 {
        if allocated == 0 {
            return 0.0;
        let usage_ratio = used as f64 / allocated as f64;

        if usage_ratio <= 0.8 {
            usage_ratio / 0.8
            1.0 - ((usage_ratio - 0.8) / 0.2) * 0.5

    fn calculate_fair_share_score(&self, stats: &ResourceUsageStatsInternal) -> f64 {
        if stats.usage_history.is_empty() {
            return 1.0;
        let avg_efficiency: f64 = stats
            .usage_history
            .iter()
            .map(|dp| dp.efficiency_score)
            .sum::<f64>()
            / stats.usage_history.len() as f64;

        let usage_ratio = stats.current_usage as f64 / stats.allocated as f64;
        if usage_ratio < 0.1 && stats.usage_history.len() > 10 {
            avg_efficiency * 0.7
            avg_efficiency
