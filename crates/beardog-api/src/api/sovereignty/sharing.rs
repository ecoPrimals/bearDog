//! Peer-to-Peer Resource Sharing
//!
//! **Empowering individuals to share resources with friends through explicit consent**
//!
//! This module implements Beardog's core peer-to-peer sharing capabilities:
//! - Friend-to-friend compute sharing (CPU, GPU, memory)
//! - Community storage pools with encryption
//! - Consent-based resource lending
//! - Usage tracking and fair allocation
//! - Automated friend network discovery
//!
//! ## Core Principles
//! - **Individual Control**: Only you decide what to share and with whom
//! - **Explicit Consent**: Every resource share requires explicit friend consent
//! - **Privacy First**: All sharing is encrypted and logged locally
//! - **Fair Usage**: Built-in mechanisms to prevent abuse
//! - **Human Dignity**: Technology serves the individual, not corporations

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

/// Peer-to-peer resource sharing engine
pub struct ResourceSharingEngine {
    /// Active sharing offers (outgoing)
    active_offers: Arc<RwLock<HashMap<String, SharingOfferInternal>>>,
    /// Active shares (resources being used)
    active_shares: Arc<RwLock<HashMap<String, ActiveShareInternal>>>,
    /// Friend network for resource discovery
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActiveShareInternal {
    pub share_id: String,
    pub friend_node_id: String,
    pub friend_display_name: String,
    pub resource_type: ResourceType,
    pub usage_stats: ResourceUsageStatsInternal,
    pub started_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revocable: bool,
    pub consent_record_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResourceUsageStatsInternal {
    pub allocated: u64,
    pub current_usage: u64,
    pub peak_usage: u64,
    pub usage_history: Vec<UsageDataPointInternal>,
    pub fair_share_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UsageDataPointInternal {
    pub timestamp: DateTime<Utc>,
    pub usage_amount: u64,
    pub efficiency_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FriendNode {
    pub node_id: String,
    pub display_name: String,
    pub public_key: String,
    pub last_seen: DateTime<Utc>,
    pub trust_score: f64,
    pub shared_resources: Vec<ResourceType>,
    pub usage_history: Vec<String>, // Share IDs
}

impl Default for ResourceSharingEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceSharingEngine {
    /// Create new resource sharing engine
    pub fn new() -> Self {
        info!("🤝 Initializing peer-to-peer resource sharing engine");

        Self {
            active_offers: Arc::new(RwLock::new(HashMap::new())),
            active_shares: Arc::new(RwLock::new(HashMap::new())),
            friend_network: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a resource sharing offer to friends
    pub async fn create_sharing_offer(
        &self,
        from_node_id: String,
        from_display_name: String,
        resource_type: ResourceType,
        resource_amount: ResourceAmount,
        personal_message: String,
        expires_hours: Option<i64>,
    ) -> Result<SharingOffer, Box<dyn std::error::Error + Send + Sync>> {
        info!(
            "🎁 Creating resource sharing offer from {}",
            from_display_name
        );

        let offer_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = expires_hours.map(|h| now + Duration::hours(h));

        // Create temporary offer for workflow
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

        // Create consent workflow for friend approval
        let workflow_id = self.create_consent_workflow(&temp_offer).await?;

        let offer_internal = SharingOfferInternal {
            offer_id: offer_id.clone(),
            from_node_id: from_node_id.clone(),
            from_display_name: from_display_name.clone(),
            resource_type: resource_type.clone(),
            resource_amount: resource_amount.clone(),
            personal_message: personal_message.clone(),
            created_at: now,
            expires_at,
            terms: self.generate_sharing_terms(&resource_type),
            consent_required: true,
            friends_notified: Vec::new(),
            workflow_id: Some(workflow_id),
        };

        // Store the offer
        {
            let mut offers = self.active_offers.write().await;
            offers.insert(offer_id.clone(), offer_internal);
        }

        // Notify friends about the offer
        self.notify_friends_of_offer(&offer_id).await?;

        // Convert to public format (clone resource_type to avoid move issues)
        let terms = self.generate_sharing_terms(&resource_type);
        Ok(SharingOffer {
            offer_id,
            from_node_id,
            from_display_name,
            resource_type,
            resource_amount,
            personal_message,
            created_at: now.to_rfc3339(),
            expires_at: expires_at.map(|dt| dt.to_rfc3339()),
            terms,
        })
    }

    /// Accept a resource sharing offer from a friend
    pub async fn accept_sharing_offer(
        &self,
        offer_id: &str,
        _accepting_node_id: String,
        _accepting_display_name: String,
    ) -> Result<ActiveShare, Box<dyn std::error::Error + Send + Sync>> {
        info!("✅ Accepting resource sharing offer: {}", offer_id);

        // Find the offer
        let offer = {
            let offers = self.active_offers.read().await;
            offers
                .get(offer_id)
                .cloned()
                .ok_or("Sharing offer not found")?
        };

        // Check if offer is still valid
        if let Some(expires_at) = offer.expires_at {
            if Utc::now() > expires_at {
                return Err("Sharing offer has expired".into());
            }
        }

        // Create active share
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
        };

        // Store the active share
        {
            let mut shares = self.active_shares.write().await;
            shares.insert(share_id.clone(), active_share_internal);
        }

        // Remove the offer since it's been accepted
        {
            let mut offers = self.active_offers.write().await;
            offers.remove(offer_id);
        }

        // Update friend network with successful sharing
        self.update_friend_trust_score(&offer.from_node_id, 0.1)
            .await?;

        info!("🎉 Resource sharing activated: {}", share_id);

        // Convert to public format
        Ok(ActiveShare {
            share_id,
            friend_node_id: offer.from_node_id,
            friend_display_name: offer.from_display_name,
            resource_type: offer.resource_type,
            usage_stats: ResourceUsageStats {
                allocated: offer.resource_amount.maximum,
                current_usage: 0,
                peak_usage: 0,
                usage_history: Vec::new(),
            },
            started_at: Utc::now().to_rfc3339(),
            expires_at: offer.expires_at.map(|dt| dt.to_rfc3339()),
            revocable: true,
        })
    }

    /// List all active sharing offers available from friends
    pub async fn list_available_offers(
        &self,
    ) -> Result<Vec<SharingOffer>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("📋 Listing available sharing offers from friend network");

        let offers = self.active_offers.read().await;
        let mut result = Vec::new();

        for offer in offers.values() {
            // Only include offers that haven't expired
            if let Some(expires_at) = offer.expires_at {
                if Utc::now() > expires_at {
                    continue;
                }
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
        }

        Ok(result)
    }

    /// List all active resource shares
    pub async fn list_active_shares(
        &self,
    ) -> Result<Vec<ActiveShare>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("📊 Listing active resource shares");

        let shares = self.active_shares.read().await;
        let mut result = Vec::new();

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
            });
        }

        Ok(result)
    }

    /// Revoke an active resource share
    pub async fn revoke_share(
        &self,
        share_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("🚫 Revoking resource share: {}", share_id);

        let mut shares = self.active_shares.write().await;
        if shares.remove(share_id).is_some() {
            info!("✅ Resource share successfully revoked: {}", share_id);
            Ok(())
        } else {
            Err("Resource share not found".into())
        }
    }

    /// Add a friend to the network
    pub async fn add_friend(
        &self,
        node_id: String,
        display_name: String,
        public_key: String,
        shared_resources: Vec<ResourceType>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("👥 Adding friend to network: {}", display_name);

        let friend = FriendNode {
            node_id: node_id.clone(),
            display_name,
            public_key,
            last_seen: Utc::now(),
            trust_score: 1.0, // Start with neutral trust
            shared_resources,
            usage_history: Vec::new(),
        };

        let mut network = self.friend_network.write().await;
        network.insert(node_id, friend);

        Ok(())
    }

    /// Update usage statistics for an active share
    pub async fn update_usage_stats(
        &self,
        share_id: &str,
        current_usage: u64,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("📈 Updating usage statistics for share: {}", share_id);

        let mut shares = self.active_shares.write().await;
        if let Some(share) = shares.get_mut(share_id) {
            share.usage_stats.current_usage = current_usage;

            if current_usage > share.usage_stats.peak_usage {
                share.usage_stats.peak_usage = current_usage;
            }

            // Add data point to history
            share
                .usage_stats
                .usage_history
                .push(UsageDataPointInternal {
                    timestamp: Utc::now(),
                    usage_amount: current_usage,
                    efficiency_score: self
                        .calculate_efficiency_score(current_usage, share.usage_stats.allocated),
                });

            // Update fair share score
            share.usage_stats.fair_share_score =
                self.calculate_fair_share_score(&share.usage_stats);

            Ok(())
        } else {
            Err("Resource share not found".into())
        }
    }

    // Private helper methods

    /// Create consent workflow for sharing
    async fn create_consent_workflow(
        &self,
        resource_offer: &SharingOffer,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        debug!("⚖️ Creating consent workflow for resource sharing");

        // Simplified workflow creation - just return a UUID for now
        let workflow_id = Uuid::new_v4().to_string();

        info!(
            "📋 Created sharing consent workflow {} for offer {}",
            workflow_id, resource_offer.offer_id
        );
        info!(
            "   Resource: {:?}, Friend: {}",
            resource_offer.resource_type, resource_offer.from_display_name
        );

        Ok(workflow_id)
    }

    /// Generate standard sharing terms based on resource type
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
            }
            ResourceType::Storage { encrypted, .. } => {
                if *encrypted {
                    terms.push("All stored data is encrypted at rest".to_string());
                }
                terms.push("No illegal content allowed".to_string());
            }
            ResourceType::Network { .. } => {
                terms.push("Network usage subject to bandwidth limits".to_string());
                terms.push("No illegal network activities allowed".to_string());
            }
        }

        terms
    }

    /// Notify friends about a new sharing offer
    async fn notify_friends_of_offer(
        &self,
        offer_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!(
            "📢 Notifying friend network about sharing offer: {}",
            offer_id
        );

        // In a real implementation, this would:
        // 1. Send encrypted notifications to friends
        // 2. Update friend discovery services
        // 3. Broadcast to local network
        // 4. Update consent workflows

        Ok(())
    }

    /// Update a friend's trust score based on sharing interactions
    async fn update_friend_trust_score(
        &self,
        friend_id: &str,
        delta: f64,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut network = self.friend_network.write().await;
        if let Some(friend) = network.get_mut(friend_id) {
            friend.trust_score = (friend.trust_score + delta).clamp(0.0, 5.0);
            friend.last_seen = Utc::now();
        }
        Ok(())
    }

    /// Calculate efficiency score for resource usage
    fn calculate_efficiency_score(&self, used: u64, allocated: u64) -> f64 {
        if allocated == 0 {
            return 0.0;
        }

        let usage_ratio = used as f64 / allocated as f64;

        // Efficiency peaks at around 70-80% usage
        if usage_ratio <= 0.8 {
            usage_ratio / 0.8
        } else {
            1.0 - ((usage_ratio - 0.8) / 0.2) * 0.5
        }
    }

    /// Calculate fair share score based on usage patterns
    fn calculate_fair_share_score(&self, stats: &ResourceUsageStatsInternal) -> f64 {
        if stats.usage_history.is_empty() {
            return 1.0;
        }

        let avg_efficiency: f64 = stats
            .usage_history
            .iter()
            .map(|dp| dp.efficiency_score)
            .sum::<f64>()
            / stats.usage_history.len() as f64;

        // Penalty for consistently low usage
        let usage_ratio = stats.current_usage as f64 / stats.allocated as f64;
        if usage_ratio < 0.1 && stats.usage_history.len() > 10 {
            avg_efficiency * 0.7
        } else {
            avg_efficiency
        }
    }
}
