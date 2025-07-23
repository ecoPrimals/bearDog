//! Individual Sovereignty API Routes
//!
//! Routes for empowering individuals to control their compute, data, and identity
//! through consent-based peer-to-peer interactions.

use crate::api::AppState;
use axum::{
    routing::{delete, get, post},
    Router,
};

use super::handlers::*;

/// Create individual sovereignty API routes
///
/// These routes embody BearDog's core mission: empowering individuals
/// to control their own resources and share them with friends through
/// explicit consent mechanisms.
///
/// # Route Structure
/// - `/sharing` - Peer-to-peer resource sharing with consent
/// - `/recovery` - Friend-based account recovery workflows  
/// - `/identity` - Self-sovereign identity management
/// - `/privacy` - Anti-surveillance privacy protections
/// - `/consent` - Consent management for all interactions
pub fn create_sovereignty_routes() -> Router<AppState> {
    Router::new()
        // 🤝 PEER-TO-PEER RESOURCE SHARING
        .route("/sharing/request", post(request_resource_sharing))
        .route("/sharing/offers", get(list_sharing_offers))
        .route("/sharing/offers/:offer_id", get(get_sharing_offer))
        .route(
            "/sharing/offers/:offer_id/accept",
            post(accept_sharing_offer),
        )
        .route(
            "/sharing/offers/:offer_id/decline",
            post(decline_sharing_offer),
        )
        .route("/sharing/active", get(list_active_shares))
        .route("/sharing/active/:share_id", delete(revoke_resource_share))
        // 🆘 FRIEND-BASED RECOVERY
        .route("/recovery/request", post(request_friend_recovery))
        .route("/recovery/requests", get(list_recovery_requests))
        .route(
            "/recovery/requests/:request_id/assist",
            post(assist_with_recovery),
        )
        .route(
            "/recovery/shards/distribute",
            post(distribute_recovery_shards),
        )
        .route("/recovery/shards/collect", post(collect_recovery_shards))
        // 🎭 SELF-SOVEREIGN IDENTITY
        .route("/identity/keys", get(list_my_identity_keys))
        .route("/identity/keys", post(generate_identity_key))
        .route("/identity/keys/:key_id", delete(revoke_identity_key))
        .route("/identity/verify", post(verify_identity_claim))
        .route("/identity/attest", post(create_identity_attestation))
        // 🔒 ANTI-SURVEILLANCE PRIVACY
        .route("/privacy/status", get(privacy_status_check))
        .route("/privacy/audit", get(privacy_audit_trail))
        .route("/privacy/purge", post(privacy_data_purge))
        .route("/privacy/anonymize", post(anonymize_personal_data))
        // ✋ CONSENT MANAGEMENT
        .route("/consent/active", get(list_active_consents))
        .route("/consent/:consent_id", get(get_consent_details))
        .route("/consent/:consent_id/revoke", post(revoke_consent))
        .route("/consent/grants", get(list_consent_grants))
        .route("/consent/requests", get(list_consent_requests))
}
