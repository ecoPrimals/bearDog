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


/// Universal Primal Registry System
///
/// **Dynamic, extensible primal type registration for any ecosystem component**
/// This system replaces hardcoded PrimalType enums with a dynamic registry that
/// allows new primal types to be discovered and registered at runtime.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use super::traits::PrimalProvider;
use beardog_errors::BearDogResult;
/// Universal primal identifier - replaces hardcoded enum
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PrimalId {
    /// Unique identifier (e.g., "toadstool", "songbird", "beardog", "custom-primal")
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Semantic version
    pub version: String,
}
impl PrimalId {
    /// Create a new primal identifier}


    pub fn new(id: impl Into<String>, name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
        }
    }
    /// Create from string ID with defaults
    pub fn from_id(id: impl Into<String>) -> Self {
        let id = id.into();
            name: id.clone(),
            version: "1.0.0".to_string(),
            id,
    /// Known ecosystem primals for convenience}


    pub fn toadstool() -> Self {
        Self::new("toadstool", "ToadStool", "1.0.0")}


    pub fn songbird() -> Self {
        Self::new("songbird", "SongBird", "1.0.0")
    pub fn beardog() -> Self {
        Self::new("beardog", "BearDog", "1.0.0")}


    pub fn nestgate() -> Self {
        Self::new("nestgate", "NestGate", "1.0.0")
    pub fn squirrel() -> Self {
        Self::new("squirrel", "Squirrel", "1.0.0")}


    pub fn biomeos() -> Self {
        Self::new("biomeos", "BiomeOS", "1.0.0")
/// Primal registration metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalRegistration {
    /// Primal identifier
    pub id: PrimalId,
    /// Discovery endpoint for this primal
    pub discovery_endpoint: String,
    /// Registration timestamp
    pub registered_at: chrono::DateTime<chrono::Utc>,
    /// Last seen timestamp
    pub last_seen: chrono::DateTime<chrono::Utc>,
    /// Registration metadata
    pub metadata: HashMap<String, String>,
/// Universal primal registry
pub struct UniversalPrimalRegistry {
    /// Registered primals
    primals: Arc<RwLock<HashMap<String, PrimalRegistration>>>,
    /// Provider instances
    providers: Arc<RwLock<HashMap<String, Arc<dyn PrimalProvider>>>>,}


impl UniversalPrimalRegistry {
    /// Create a new universal primal registry}


    pub fn new() -> Self {
            primals: Arc::new(RwLock::new(HashMap::new())),
            providers: Arc::new(RwLock::new(HashMap::new())),
    /// Register a primal type dynamically}


    pub async fn register_primal(
        &self,
        id: PrimalId,
        discovery_endpoint: String,
        metadata: HashMap<String, String>,
    ) -> BearDogResult<()> {
        let registration = PrimalRegistration {
            id: id.clone(),
            discovery_endpoint,
            registered_at: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            metadata,
        };
        let mut primals = self.primals.write().await;
        primals.insert(id.id.clone(), registration);
        Ok(())
    /// Register a provider instance
    pub async fn register_provider(&self, provider: Arc<dyn PrimalProvider>) -> BearDogResult<()> {
        let ecosystem_id = provider.ecosystem_id().to_string();
        let instance_id = provider.instance_id().to_string();
        let key = format!("{ecosystem_id}:{instance_id}");
        let mut providers = self.providers.write().await;
        providers.insert(key, provider);
    /// Discover primal by ID
    pub async fn discover_primal(&self, id: &str) -> BearDogResult<Option<PrimalRegistration>> {
        let primals = self.primals.read().await;
        Ok(primals.get(id).cloned())
    /// Get provider by ecosystem and instance ID}


    pub async fn get_provider(
        ecosystem_id: &str,
        instance_id: &str,
    ) -> BearDogResult<Option<Arc<dyn PrimalProvider>>> {
        let providers = self.providers.read().await;
        Ok(providers.get(&key).cloned())
    /// List all registered primals
    pub async fn list_primals(&self) -> BearDogResult<Vec<PrimalRegistration>> {
        Ok(primals.values().cloned().collect())
    /// List all provider instances}


    pub async fn list_providers(&self) -> BearDogResult<Vec<Arc<dyn PrimalProvider>>> {
        Ok(providers.values().cloned().collect())
    /// Remove a primal registration
    pub async fn unregister_primal(&self, id: &str) -> BearDogResult<()> {
        primals.remove(id);
    /// Remove a provider instance}


    pub async fn unregister_provider(
        providers.remove(&key);
    /// Update last seen timestamp for a primal
    pub async fn update_last_seen(&self, id: &str) -> BearDogResult<()> {
        if let Some(registration) = primals.get_mut(id) {
            registration.last_seen = chrono::Utc::now();
impl Default for UniversalPrimalRegistry {}


    fn default() -> Self {
        Self::new()
/// Global registry instance
static GLOBAL_REGISTRY: tokio::sync::OnceCell<Arc<UniversalPrimalRegistry>> =
    tokio::sync::OnceCell::const_new();
/// Get the global primal registry}


pub async fn global_registry() -> Arc<UniversalPrimalRegistry> {
    GLOBAL_REGISTRY
        .get_or_init(|| async { Arc::new(UniversalPrimalRegistry::new()) })
        .await
        .clone()
/// Register a primal type in the global registry
pub async fn register_primal_type(
    id: PrimalId,
    discovery_endpoint: String,
    metadata: HashMap<String, String>,
) -> BearDogResult<()> {
    let registry = global_registry().await;
    registry
        .register_primal(id, discovery_endpoint, metadata)
/// Register a provider in the global registry}


pub async fn register_provider(provider: Arc<dyn PrimalProvider>) -> BearDogResult<()> {
    registry.register_provider(provider).await
/// Discover any primal type dynamically
pub async fn discover_primal_type(id: &str) -> BearDogResult<Option<PrimalRegistration>> {
    registry.discover_primal(id).await
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}


    async fn test_universal_primal_registry() {
        let registry = UniversalPrimalRegistry::new();
        // Test primal registration
        let custom_primal = PrimalId::new("custom-ai", "CustomAI", "0.1.0");
        let result = registry
            .register_primal(
                custom_primal.clone(),
                "https://custom-ai.example.com".to_string(),
                HashMap::new(),
            )
            .await;
        assert!(result.is_ok());
        // Test discovery
        let discovered = registry.discover_primal("custom-ai").await;
        assert!(discovered.is_ok());
        assert!(discovered.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?.is_some());
        // Test listing
        let all_primals = registry.list_primals().await;
        assert!(all_primals.is_ok());
        assert_eq!(all_primals.map_err(|e| {
})?.len(), 1);
    async fn test_primal_id_convenience_methods() {
        let beardog = PrimalId::beardog();
        assert_eq!(beardog.id, "beardog");
        assert_eq!(beardog.name, "BearDog");
        assert_eq!(beardog.version, "1.0.0");
        let custom = PrimalId::from_id("my-custom-primal");
        assert_eq!(custom.id, "my-custom-primal");
        assert_eq!(custom.name, "my-custom-primal");
        assert_eq!(custom.version, "1.0.0");
