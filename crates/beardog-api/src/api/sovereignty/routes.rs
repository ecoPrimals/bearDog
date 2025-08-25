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


/// Individual Sovereignty API Routes
///
/// Routes for empowering individuals to control their compute, data, and identity
/// through consent-based peer-to-peer interactions.

use crate::api::AppState;
use axum::{
    routing::{delete, get, post},
    Router,
};
use super::handlers::*;
/// Create individual sovereignty API routes
/// These routes embody BearDog's core mission: empowering individuals
/// to control their own resources and share them with friends through
/// explicit consent mechanisms.
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
            "/sharing/offers/:offer_id/decline",
            post(decline_sharing_offer),
        .route("/sharing/active", get(list_active_shares))
        .route("/sharing/active/:share_id", delete(revoke_resource_share))
        // 🆘 FRIEND-BASED RECOVERY
        .route("/recovery/request", post(request_friend_recovery))
        .route("/recovery/requests", get(list_recovery_requests))
            "/recovery/requests/:request_id/assist",
            post(assist_with_recovery),
            "/recovery/shards/distribute",
            post(distribute_recovery_shards),
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
