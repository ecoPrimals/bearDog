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


/// Authentication & Authorization API Module
///
/// Comprehensive REST API for BearDog's authentication, authorization, and user
/// management capabilities supporting multiple authentication methods, MFA, and RBAC.

use super::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
// Re-export types and handlers from sub-modules
pub use authentication::*;
pub use authorization::*;
pub use mfa::*;
pub use models::*;
pub use users::*;
// Sub-modules with focused responsibilities
pub mod authentication; // Core auth: login, logout, tokens, sessions
pub mod authorization; // Roles, permissions, API keys
pub mod mfa; // Multi-factor authentication
pub mod models; // All request/response models
pub mod users; // User management CRUD operations
/// Create authentication API routes
pub fn create_routes() -> Router<AppState> {
    Router::new()
        // Authentication routes
        .route("/login", post(authenticate_user))
        .route("/logout", post(logout_user))
        .route("/refresh", post(refresh_token))
        .route("/validate", post(validate_token))
        .route("/session", get(get_current_session))
        .route("/session", delete(invalidate_session))
        // Multi-Factor Authentication routes
        .route("/mfa/setup", post(setup_mfa))
        .route("/mfa/verify", post(verify_mfa))
        .route("/mfa/backup-codes", get(get_backup_codes))
        .route("/mfa/backup-codes", post(generate_backup_codes))
        .route("/mfa/status", get(get_mfa_status))
        // User Management routes
        .route("/users", get(list_users))
        .route("/users", post(create_user))
        .route("/users/:user_id", get(get_user))
        .route("/users/:user_id", put(update_user))
        .route("/users/:user_id", delete(delete_user))
        .route("/users/:user_id/activate", post(activate_user))
        .route("/users/:user_id/deactivate", post(deactivate_user))
        .route("/users/:user_id/reset-password", post(reset_password))
        // Role-Based Access Control routes
        .route("/roles", get(list_roles))
        .route("/roles", post(create_role))
        .route("/roles/:role_id", get(get_role))
        .route("/roles/:role_id", put(update_role))
        .route("/roles/:role_id", delete(delete_role))
        .route("/roles/:role_id/permissions", get(get_role_permissions))
        .route("/roles/:role_id/permissions", put(update_role_permissions))
        // User-Role Assignment routes
        .route("/users/:user_id/roles", get(get_user_roles))
        .route("/users/:user_id/roles", post(assign_user_roles))
        .route("/users/:user_id/roles/:role_id", delete(remove_user_role))
        // Permissions & Authorization routes
        .route("/permissions", get(list_permissions))
        .route("/permissions/check", post(check_permission))
        .route("/permissions/bulk-check", post(bulk_check_permissions))
        .route("/users/:user_id/permissions", get(get_user_permissions))
        // API Keys & Service Accounts routes
        .route("/api-keys", get(list_api_keys))
        .route("/api-keys", post(create_api_key))
        .route("/api-keys/:key_id", get(get_api_key))
        .route("/api-keys/:key_id", delete(revoke_api_key))
        .route("/api-keys/:key_id/rotate", post(rotate_api_key))
        // Audit & Security routes
        .route("/audit/login-attempts", get(get_login_attempts))
        .route("/audit/security-events", get(get_security_events))
        .route("/audit/user-activity", get(get_user_activity))
        .route("/security/password-policy", get(get_password_policy))
        .route("/security/password-policy", put(update_password_policy))
        .route("/security/lockout-policy", get(get_lockout_policy))
        .route("/security/lockout-policy", put(update_lockout_policy))
}
