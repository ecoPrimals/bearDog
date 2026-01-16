//! Advanced Authentication and Authorization Services (Phase 2)
//!
//! Implements JWT token generation, `OAuth2` integration, and RBAC/ABAC authorization.
//! This module provides production-grade authentication and authorization capabilities
//! for the `BearDog` ecosystem.

use beardog_errors::BearDogError;
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

// HMAC-SHA256 type alias for JWT signatures (Pure Rust!)
type HmacSha256 = Hmac<Sha256>;

// ============================================================================
// JWT Token Management
// ============================================================================

/// JWT Header structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct JwtHeader {
    alg: String,
    typ: String,
}

/// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Subject (user ID)
    pub sub: String,
    /// Issued at (Unix timestamp)
    pub iat: i64,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issuer
    pub iss: String,
    /// Audience
    pub aud: String,
    /// Custom claims (roles, permissions, etc.)
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,
}

/// JWT Token Manager (Pure Rust implementation using RustCrypto!)
pub struct JwtTokenManager {
    secret: Vec<u8>,
    issuer: String,
    audience: String,
    default_expiry: Duration,
}

impl JwtTokenManager {
    /// Create a new JWT token manager
    ///
    /// # Arguments
    /// * `secret` - Secret key for signing tokens (should be loaded from secure config)
    /// * `issuer` - Token issuer identifier
    /// * `audience` - Token audience identifier
    /// * `expiry_hours` - Default token expiry in hours
    #[must_use]
    pub fn new(secret: &[u8], issuer: String, audience: String, expiry_hours: i64) -> Self {
        Self {
            secret: secret.to_vec(),
            issuer,
            audience,
            default_expiry: Duration::hours(expiry_hours),
        }
    }

    /// Base64 URL-safe encode (JWT standard)
    fn base64url_encode(data: &[u8]) -> String {
        use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
        URL_SAFE_NO_PAD.encode(data)
    }

    /// Base64 URL-safe decode (JWT standard)
    fn base64url_decode(data: &str) -> Result<Vec<u8>, BearDogError> {
        use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
        URL_SAFE_NO_PAD
            .decode(data)
            .map_err(|e| BearDogError::security(format!("Base64 decode failed: {e}")))
    }

    /// Sign data with HMAC-SHA256 (Pure Rust!)
    fn sign(&self, data: &str) -> Result<Vec<u8>, BearDogError> {
        let mut mac = HmacSha256::new_from_slice(&self.secret)
            .map_err(|e| BearDogError::security(format!("HMAC initialization failed: {e}")))?;
        mac.update(data.as_bytes());
        Ok(mac.finalize().into_bytes().to_vec())
    }

    /// Verify HMAC-SHA256 signature (Pure Rust!)
    fn verify(&self, data: &str, signature: &[u8]) -> Result<(), BearDogError> {
        let mut mac = HmacSha256::new_from_slice(&self.secret)
            .map_err(|e| BearDogError::security(format!("HMAC initialization failed: {e}")))?;
        mac.update(data.as_bytes());
        mac.verify_slice(signature)
            .map_err(|_| BearDogError::security("JWT signature verification failed".to_string()))
    }

    /// Generate a JWT token for a user (Pure Rust implementation!)
    ///
    /// # Arguments
    /// * `user_id` - User identifier
    /// * `custom_claims` - Additional custom claims to include in the token
    ///
    /// # Returns
    /// Signed JWT token string
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub fn generate_token(
        &self,
        user_id: &str,
        custom_claims: HashMap<String, serde_json::Value>,
    ) -> Result<String, BearDogError> {
        let now = Utc::now();
        let expiry = now + self.default_expiry;

        // Create JWT header
        let header = JwtHeader {
            alg: "HS256".to_string(),
            typ: "JWT".to_string(),
        };

        // Create JWT claims with custom fields
        let mut claims = JwtClaims {
            sub: user_id.to_string(),
            iat: now.timestamp(),
            exp: expiry.timestamp(),
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            custom: custom_claims,
        };

        // Serialize header and claims
        let header_json = serde_json::to_string(&header)
            .map_err(|e| BearDogError::security(format!("Failed to serialize header: {e}")))?;
        let claims_json = serde_json::to_string(&claims)
            .map_err(|e| BearDogError::security(format!("Failed to serialize claims: {e}")))?;

        // Base64url encode header and claims
        let header_b64 = Self::base64url_encode(header_json.as_bytes());
        let claims_b64 = Self::base64url_encode(claims_json.as_bytes());

        // Create signing input
        let signing_input = format!("{}.{}", header_b64, claims_b64);

        // Sign with HMAC-SHA256 (Pure Rust!)
        let signature = self.sign(&signing_input)?;
        let signature_b64 = Self::base64url_encode(&signature);

        // Create final JWT token
        Ok(format!("{}.{}", signing_input, signature_b64))
    }

    /// Validate and decode a JWT token (Pure Rust implementation!)
    ///
    /// # Arguments
    /// * `token` - JWT token string to validate
    ///
    /// # Returns
    /// Decoded JWT claims if valid
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub fn validate_token(&self, token: &str) -> Result<JwtClaims, BearDogError> {
        // Split token into parts
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(BearDogError::security(
                "Invalid JWT format: expected 3 parts".to_string(),
            ));
        }

        let header_b64 = parts[0];
        let claims_b64 = parts[1];
        let signature_b64 = parts[2];

        // Verify signature (Pure Rust!)
        let signing_input = format!("{}.{}", header_b64, claims_b64);
        let signature = Self::base64url_decode(signature_b64)?;
        self.verify(&signing_input, &signature)?;

        // Decode and parse header
        let header_json = Self::base64url_decode(header_b64)?;
        let header: JwtHeader = serde_json::from_slice(&header_json)
            .map_err(|e| BearDogError::security(format!("Failed to parse header: {e}")))?;

        // Verify algorithm
        if header.alg != "HS256" {
            return Err(BearDogError::security(format!(
                "Unsupported algorithm: {}",
                header.alg
            )));
        }

        // Decode and parse claims
        let claims_json = Self::base64url_decode(claims_b64)?;
        let claims: JwtClaims = serde_json::from_slice(&claims_json)
            .map_err(|e| BearDogError::security(format!("Failed to parse claims: {e}")))?;

        // Verify issuer
        if claims.iss != self.issuer {
            return Err(BearDogError::security(format!(
                "Invalid issuer: expected {}, got {}",
                self.issuer, claims.iss
            )));
        }

        // Verify audience
        if claims.aud != self.audience {
            return Err(BearDogError::security(format!(
                "Invalid audience: expected {}, got {}",
                self.audience, claims.aud
            )));
        }

        // Verify expiration
        let now = Utc::now().timestamp();
        if claims.exp < now {
            return Err(BearDogError::security("Token has expired".to_string()));
        }

        Ok(claims)
    }

    /// Extract user ID from token without full validation (for logging/debugging)
    #[must_use]
    pub fn extract_user_id(&self, token: &str) -> Option<String> {
        // Split token and decode claims (skip signature verification for debugging)
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return None;
        }

        let claims_b64 = parts[1];
        let claims_json = Self::base64url_decode(claims_b64).ok()?;
        let claims: JwtClaims = serde_json::from_slice(&claims_json).ok()?;

        Some(claims.sub)
    }
}

// ============================================================================
// RBAC (Role-Based Access Control)
// ============================================================================

/// User role definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    /// System administrator with full access
    Admin,
    /// Regular user with standard permissions
    User,
    /// Guest with read-only access
    Guest,
    /// Service account for automated processes
    Service,
    /// Auditor with read and audit permissions
    Auditor,
    /// Custom role with specific permissions
    Custom(String),
}

/// Permission definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// Read operations
    Read,
    /// Write operations
    Write,
    /// Delete operations
    Delete,
    /// Execute operations
    Execute,
    /// Administrative operations
    Admin,
    /// Audit operations
    Audit,
    /// Custom permission
    Custom(String),
}

/// Resource definition for access control
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Resource {
    /// Type of resource (e.g., "key", "workflow", "primal")
    pub resource_type: String,
    /// Optional unique identifier for the specific resource instance
    pub resource_id: Option<String>,
}

/// RBAC Policy
#[derive(Debug, Clone)]
pub struct RbacPolicy {
    role_permissions: HashMap<Role, HashSet<Permission>>,
    user_roles: Arc<RwLock<HashMap<String, HashSet<Role>>>>,
}

impl RbacPolicy {
    /// Create a new RBAC policy with default role-permission mappings
    #[must_use]
    pub fn new() -> Self {
        let mut role_permissions = HashMap::new();

        // Admin role: all permissions
        role_permissions.insert(
            Role::Admin,
            vec![
                Permission::Read,
                Permission::Write,
                Permission::Delete,
                Permission::Execute,
                Permission::Admin,
                Permission::Audit,
            ]
            .into_iter()
            .collect(),
        );

        // User role: read, write, execute
        role_permissions.insert(
            Role::User,
            vec![Permission::Read, Permission::Write, Permission::Execute]
                .into_iter()
                .collect(),
        );

        // Guest role: read only
        role_permissions.insert(Role::Guest, vec![Permission::Read].into_iter().collect());

        // Service role: read, write, execute
        role_permissions.insert(
            Role::Service,
            vec![Permission::Read, Permission::Write, Permission::Execute]
                .into_iter()
                .collect(),
        );

        // Auditor role: read and audit
        role_permissions.insert(
            Role::Auditor,
            vec![Permission::Read, Permission::Audit]
                .into_iter()
                .collect(),
        );

        Self {
            role_permissions,
            user_roles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Assign a role to a user
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn assign_role(&self, user_id: &str, role: Role) -> Result<(), BearDogError> {
        let mut user_roles = self.user_roles.write().await;
        user_roles
            .entry(user_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(role);
        Ok(())
    }

    /// Remove a role from a user
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn remove_role(&self, user_id: &str, role: &Role) -> Result<(), BearDogError> {
        let mut user_roles = self.user_roles.write().await;
        if let Some(roles) = user_roles.get_mut(user_id) {
            roles.remove(role);
        }
        Ok(())
    }

    /// Check if a user has permission for an operation
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn has_permission(
        &self,
        user_id: &str,
        permission: &Permission,
    ) -> Result<bool, BearDogError> {
        let user_roles = self.user_roles.read().await;

        if let Some(roles) = user_roles.get(user_id) {
            for role in roles {
                if let Some(permissions) = self.role_permissions.get(role) {
                    if permissions.contains(permission) {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    /// Get all permissions for a user
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn get_user_permissions(
        &self,
        user_id: &str,
    ) -> Result<HashSet<Permission>, BearDogError> {
        let user_roles = self.user_roles.read().await;
        let mut permissions = HashSet::new();

        if let Some(roles) = user_roles.get(user_id) {
            for role in roles {
                if let Some(role_perms) = self.role_permissions.get(role) {
                    permissions.extend(role_perms.clone());
                }
            }
        }

        Ok(permissions)
    }

    /// Get all roles for a user
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn get_user_roles(&self, user_id: &str) -> Result<HashSet<Role>, BearDogError> {
        let user_roles = self.user_roles.read().await;
        Ok(user_roles.get(user_id).cloned().unwrap_or_default())
    }
}

impl Default for RbacPolicy {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// OAuth2 Integration
// ============================================================================

/// `OAuth2` provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    /// `OAuth2` client identifier provided by the authorization server
    pub client_id: String,
    /// `OAuth2` client secret for authenticating with the authorization server
    pub client_secret: String,
    /// Authorization endpoint URL for initiating `OAuth2` flow
    pub auth_url: String,
    /// Token endpoint URL for exchanging authorization codes for access tokens
    pub token_url: String,
    /// Redirect URI where the authorization server sends responses
    pub redirect_uri: String,
    /// List of `OAuth2` scopes to request (e.g., "read", "write")
    pub scopes: Vec<String>,
}

/// `OAuth2` token response from authorization server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2TokenResponse {
    /// The access token issued by the authorization server
    pub access_token: String,
    /// Token type (typically "Bearer")
    pub token_type: String,
    /// Lifetime of the access token in seconds
    pub expires_in: i64,
    /// Optional refresh token for obtaining new access tokens
    pub refresh_token: Option<String>,
    /// Optional space-separated list of granted scopes
    pub scope: Option<String>,
}

/// `OAuth2` client for external authentication
pub struct OAuth2Client {
    config: OAuth2Config,
    http_client: reqwest::Client,
}

impl OAuth2Client {
    /// Create a new `OAuth2` client
    #[must_use]
    pub fn new(config: OAuth2Config) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }

    /// Generate authorization URL for user to authenticate
    #[must_use]
    pub fn get_authorization_url(&self, state: &str) -> String {
        let scopes = self.config.scopes.join(" ");
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            self.config.auth_url,
            urlencoding::encode(&self.config.client_id),
            urlencoding::encode(&self.config.redirect_uri),
            urlencoding::encode(&scopes),
            urlencoding::encode(state)
        )
    }

    /// Exchange authorization code for access token
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn exchange_code(&self, code: &str) -> Result<OAuth2TokenResponse, BearDogError> {
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &self.config.redirect_uri),
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
        ];

        let response = self
            .http_client
            .post(&self.config.token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| BearDogError::security(format!("OAuth2 token exchange failed: {e}")))?;

        if !response.status().is_success() {
            return Err(BearDogError::security(format!(
                "OAuth2 token exchange returned status: {}",
                response.status()
            )));
        }

        response
            .json::<OAuth2TokenResponse>()
            .await
            .map_err(|e| BearDogError::security(format!("Failed to parse OAuth2 response: {e}")))
    }

    /// Refresh an access token using a refresh token
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<OAuth2TokenResponse, BearDogError> {
        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
        ];

        let response = self
            .http_client
            .post(&self.config.token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| BearDogError::security(format!("OAuth2 token refresh failed: {e}")))?;

        if !response.status().is_success() {
            return Err(BearDogError::security(format!(
                "OAuth2 token refresh returned status: {}",
                response.status()
            )));
        }

        response
            .json::<OAuth2TokenResponse>()
            .await
            .map_err(|e| BearDogError::security(format!("Failed to parse OAuth2 response: {e}")))
    }
}

// ============================================================================
// Unified Auth Manager (combines JWT, RBAC, OAuth2)
// ============================================================================

/// Global auth manager instance
static AUTH_MANAGER: OnceLock<Arc<RwLock<AuthManager>>> = OnceLock::new();

/// Unified authentication and authorization manager
pub struct AuthManager {
    jwt_manager: JwtTokenManager,
    rbac_policy: RbacPolicy,
    oauth2_clients: HashMap<String, OAuth2Client>,
}

impl AuthManager {
    /// Create a new auth manager
    #[must_use]
    pub fn new(jwt_secret: &[u8], issuer: String, audience: String) -> Self {
        Self {
            jwt_manager: JwtTokenManager::new(jwt_secret, issuer, audience, 24), // 24h default
            rbac_policy: RbacPolicy::new(),
            oauth2_clients: HashMap::new(),
        }
    }

    /// Register an `OAuth2` provider
    pub fn register_oauth2_provider(&mut self, name: String, config: OAuth2Config) {
        self.oauth2_clients.insert(name, OAuth2Client::new(config));
    }

    /// Generate a JWT token with role-based claims
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn authenticate_user(&self, user_id: &str) -> Result<String, BearDogError> {
        let roles = self.rbac_policy.get_user_roles(user_id).await?;
        let permissions = self.rbac_policy.get_user_permissions(user_id).await?;

        let mut custom_claims = HashMap::new();
        custom_claims.insert(
            "roles".to_string(),
            serde_json::to_value(&roles)
                .map_err(|e| BearDogError::security(format!("Failed to serialize roles: {e}")))?,
        );
        custom_claims.insert(
            "permissions".to_string(),
            serde_json::to_value(&permissions).map_err(|e| {
                BearDogError::security(format!("Failed to serialize permissions: {e}"))
            })?,
        );

        self.jwt_manager.generate_token(user_id, custom_claims)
    }

    /// Validate token and check permission
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn authorize_operation(
        &self,
        token: &str,
        permission: &Permission,
    ) -> Result<bool, BearDogError> {
        let claims = self.jwt_manager.validate_token(token)?;
        self.rbac_policy
            .has_permission(&claims.sub, permission)
            .await
    }

    /// Get `OAuth2` client by provider name
    #[must_use]
    pub fn get_oauth2_client(&self, provider: &str) -> Option<&OAuth2Client> {
        self.oauth2_clients.get(provider)
    }

    /// Access JWT manager
    #[must_use]
    pub fn jwt(&self) -> &JwtTokenManager {
        &self.jwt_manager
    }

    /// Access RBAC policy
    #[must_use]
    pub fn rbac(&self) -> &RbacPolicy {
        &self.rbac_policy
    }
}

/// Initialize the global auth manager
pub fn init_auth_manager(
    jwt_secret: &[u8],
    issuer: String,
    audience: String,
) -> Arc<RwLock<AuthManager>> {
    let manager = Arc::new(RwLock::new(AuthManager::new(jwt_secret, issuer, audience)));
    AUTH_MANAGER.get_or_init(|| manager.clone());
    manager
}

/// Get the global auth manager instance
pub fn get_auth_manager() -> Option<Arc<RwLock<AuthManager>>> {
    AUTH_MANAGER.get().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_token_generation() {
        let secret = b"test_secret_key_at_least_32_bytes_long!";
        let manager = JwtTokenManager::new(
            secret,
            "beardog-test".to_string(),
            "beardog-api".to_string(),
            1,
        );

        let mut custom_claims = HashMap::new();
        custom_claims.insert("role".to_string(), serde_json::json!("admin"));

        let token = manager
            .generate_token("user123", custom_claims)
            .expect("Token generation should succeed");

        assert!(!token.is_empty());
        assert!(
            token.contains('.'),
            "JWT should have 3 parts separated by dots"
        );
    }

    #[test]
    fn test_jwt_token_validation() {
        let secret = b"test_secret_key_at_least_32_bytes_long!";
        let manager = JwtTokenManager::new(
            secret,
            "beardog-test".to_string(),
            "beardog-api".to_string(),
            1,
        );

        let token = manager
            .generate_token("user123", HashMap::new())
            .expect("Token generation should succeed");

        let claims = manager
            .validate_token(&token)
            .expect("Token validation should succeed");

        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.iss, "beardog-test");
        assert_eq!(claims.aud, "beardog-api");
    }

    #[tokio::test]
    async fn test_rbac_role_assignment() {
        let policy = RbacPolicy::new();

        policy
            .assign_role("user1", Role::Admin)
            .await
            .expect("Role assignment should succeed");

        let roles = policy
            .get_user_roles("user1")
            .await
            .expect("Get roles should succeed");

        assert!(roles.contains(&Role::Admin));
    }

    #[tokio::test]
    async fn test_rbac_permission_check() {
        let policy = RbacPolicy::new();

        policy
            .assign_role("user1", Role::User)
            .await
            .expect("Role assignment should succeed");

        let has_read = policy
            .has_permission("user1", &Permission::Read)
            .await
            .expect("Permission check should succeed");

        let has_admin = policy
            .has_permission("user1", &Permission::Admin)
            .await
            .expect("Permission check should succeed");

        assert!(has_read, "User role should have Read permission");
        assert!(!has_admin, "User role should not have Admin permission");
    }

    #[tokio::test]
    async fn test_rbac_multiple_roles() {
        let policy = RbacPolicy::new();

        policy
            .assign_role("user1", Role::User)
            .await
            .expect("Role assignment should succeed");

        policy
            .assign_role("user1", Role::Auditor)
            .await
            .expect("Role assignment should succeed");

        let permissions = policy
            .get_user_permissions("user1")
            .await
            .expect("Get permissions should succeed");

        assert!(permissions.contains(&Permission::Read));
        assert!(permissions.contains(&Permission::Write));
        assert!(permissions.contains(&Permission::Audit));
    }

    #[test]
    fn test_oauth2_authorization_url() {
        let config = OAuth2Config {
            client_id: "test_client".to_string(),
            client_secret: "test_secret".to_string(),
            auth_url: "https://provider.com/oauth/authorize".to_string(),
            token_url: "https://provider.com/oauth/token".to_string(),
            redirect_uri: "http://localhost:8080/callback".to_string(),
            scopes: vec!["read".to_string(), "write".to_string()],
        };

        let client = OAuth2Client::new(config);
        let url = client.get_authorization_url("random_state_123");

        assert!(url.contains("client_id=test_client"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("state=random_state_123"));
        // URL encoding converts spaces to %20, so "read write" becomes "read%20write"
        assert!(url.contains("scope=read%20write") || url.contains("scope=read+write"));
    }

    #[tokio::test]
    async fn test_auth_manager_integration() {
        let secret = b"test_secret_key_at_least_32_bytes_long!";
        let manager = AuthManager::new(
            secret,
            "beardog-test".to_string(),
            "beardog-api".to_string(),
        );

        // Assign role
        manager
            .rbac()
            .assign_role("user1", Role::Admin)
            .await
            .expect("Role assignment should succeed");

        // Generate token
        let token = manager
            .authenticate_user("user1")
            .await
            .expect("Token generation should succeed");

        // Validate token and check permission
        let authorized = manager
            .authorize_operation(&token, &Permission::Admin)
            .await
            .expect("Authorization check should succeed");

        assert!(authorized, "Admin should have admin permission");
    }
}
