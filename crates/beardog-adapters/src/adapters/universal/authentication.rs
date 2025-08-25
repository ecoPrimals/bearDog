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


/// Authentication Implementations for Universal Adapter
///
/// Authentication-agnostic layer for external systems
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;

/// Authentication trait for universal adapter
pub trait Authentication: Send + Sync {
    /// Authentication method name
    fn method(&self) -> &str;
    /// Validate authentication parameters
    fn validate(&self, params: &HashMap<String, String>) -> BearDogResult<()>;
    /// Get authentication headers for HTTP requests
    fn get_headers(&self) -> HashMap<String, String>;
}
/// No authentication - for systems that don't require auth
pub struct NoAuthentication;
impl Default for NoAuthentication {}


    fn default() -> Self {
        Self::new()
    }
impl NoAuthentication {}


    pub fn new() -> Self {
        Self
impl Authentication for NoAuthentication {}


    fn method(&self) -> &str {
        "none"}


    fn validate(&self, _params: &HashMap<String, String>) -> BearDogResult<()> {
        Ok(())
    fn get_headers(&self) -> HashMap<String, String> {
        HashMap::new()
/// API Key authentication
pub struct ApiKeyAuthentication {
    api_key: String,
    header_name: String,}


impl ApiKeyAuthentication {}


    pub fn new(api_key: String, header_name: Option<String>) -> Self {
        Self {
            api_key,
            header_name: header_name.unwrap_or_else(|| "Authorization".to_string()),
        }
impl Authentication for ApiKeyAuthentication {
        "api_key"
        if self.api_key.is_empty() {
            return Err(BearDogError::configuration("API key cannot be empty".to_string(),
            ));
        let mut headers = HashMap::new();
        headers.insert(self.header_name.clone(), format!("Bearer {}", self.api_key));
        headers
/// Bearer token authentication
pub struct BearerTokenAuthentication {
    token: String,}


impl BearerTokenAuthentication {}


    pub fn new(token: String) -> Self {
        Self { token }
impl Authentication for BearerTokenAuthentication {
        "bearer_token"
        if self.token.is_empty() {
            return Err(BearDogError::configuration("Bearer token cannot be empty".to_string(),
        headers.insert(
            "Authorization".to_string(),
            format!("Bearer {}", self.token),
        );
/// Kubernetes config-based authentication
pub struct KubeconfigAuthentication {
    config_path: String,}


impl Default for KubeconfigAuthentication {}


impl KubeconfigAuthentication {
            config_path: std::env::var("KUBECONFIG")
                .unwrap_or_else(|_| "~/.kube/config".to_string()),
    pub fn with_path(config_path: String) -> Self {
        Self { config_path }
impl Authentication for KubeconfigAuthentication {
        "kubeconfig"
        // Could validate kubeconfig exists and is readable
        // kubectl handles authentication internally
