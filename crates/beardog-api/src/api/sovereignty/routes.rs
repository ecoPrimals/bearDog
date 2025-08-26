

use crate::api::AppState;
use axum::{
    routing::{delete, get, post},
    Router,
};
use super::handlers::*;

pub fn create_sovereignty_routes() -> Router<AppState> {
    Router::new()

        .route("/sharing/request", post(request_resource_sharing))
        .route("/sharing/offers", get(list_sharing_offers))
        .route("/sharing/offers/:offer_id", get(get_sharing_offer))
        .route(
            "/sharing/offers/:offer_id/accept",
            post(accept_sharing_offer),
        )
            "/sharing/offers/:offer_id/decline",
            post(decline_sharing_offer),
        .route("/sharing/active", get(list_active_shares))
        .route("/sharing/active/:share_id", delete(revoke_resource_share))

        .route("/recovery/request", post(request_friend_recovery))
        .route("/recovery/requests", get(list_recovery_requests))
            "/recovery/requests/:request_id/assist",
            post(assist_with_recovery),
            "/recovery/shards/distribute",
            post(distribute_recovery_shards),
        .route("/recovery/shards/collect", post(collect_recovery_shards))

        .route("/identity/keys", get(list_my_identity_keys))
        .route("/identity/keys", post(generate_identity_key))
        .route("/identity/keys/:key_id", delete(revoke_identity_key))
        .route("/identity/verify", post(verify_identity_claim))
        .route("/identity/attest", post(create_identity_attestation))

        .route("/privacy/status", get(privacy_status_check))
        .route("/privacy/audit", get(privacy_audit_trail))
        .route("/privacy/purge", post(privacy_data_purge))
        .route("/privacy/anonymize", post(anonymize_personal_data))

        .route("/consent/active", get(list_active_consents))
        .route("/consent/:consent_id", get(get_consent_details))
        .route("/consent/:consent_id/revoke", post(revoke_consent))
        .route("/consent/grants", get(list_consent_grants))
        .route("/consent/requests", get(list_consent_requests))
}
