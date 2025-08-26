

use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;

pub trait Authentication: Send + Sync {

    fn method(&self) -> &str;

    fn validate(&self, params: &HashMap<&str, &str>) -> BearDogResult<()>;

    fn get_headers(&self) -> HashMap<String, String>;
}

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

    fn validate(&self, _params: &HashMap<&str, &str>) -> BearDogResult<()> {
        Ok(())
    fn get_headers(&self) -> HashMap<String, String> {
        HashMap::with_capacity(16)

pub struct ApiKeyAuthentication {
    api_key: String,
    header_name: String,}

impl ApiKeyAuthentication {}

    pub fn new(api_key: &str, header_name: Option<&str>) -> Self {
        Self {
            api_key,
            header_name: header_name.unwrap_or_else(|| "Authorization".to_string()),
        }
impl Authentication for ApiKeyAuthentication {
        "api_key"
        if self.api_key.is_empty() {
            return Err(BearDogError::configuration("API key cannot be empty".to_string(),
            ));
        let mut headers = HashMap::with_capacity(16);
        headers.insert(self.header_name.clone(), format_args!("Bearer {}", self.api_key).to_string());
        headers

pub struct BearerTokenAuthentication {
    token: String,}

impl BearerTokenAuthentication {}

    pub fn new(token: &str) -> Self {
        Self { token }
impl Authentication for BearerTokenAuthentication {
        "bearer_token"
        if self.token.is_empty() {
            return Err(BearDogError::configuration("Bearer token cannot be empty".to_string(),
        headers.insert(
            "Authorization".to_string(),
            format_args!("Bearer {}", self.token).to_string(),
        );

pub struct KubeconfigAuthentication {
    config_path: String,}

impl Default for KubeconfigAuthentication {}

impl KubeconfigAuthentication {
            config_path: std::env::var("KUBECONFIG")
                .unwrap_or_else(|_| "~/.kube/config".to_string()),
    pub fn with_path(config_path: &str) -> Self {
        Self { config_path }
impl Authentication for KubeconfigAuthentication {
        "kubeconfig"

