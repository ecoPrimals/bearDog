

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use super::traits::PrimalProvider;
use beardog_errors::BearDogResult;

#[derive(Debug, Clone)]
pub struct DefaultPrimalProvider {
    ecosystem_id: String,
    instance_id: String,
}

impl PrimalProvider for DefaultPrimalProvider {
    fn ecosystem_id(&self) -> &str { &self.ecosystem_id }
    fn instance_id(&self) -> &str { &self.instance_id }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PrimalId {

    pub id: String,

    pub name: String,

    pub version: String,
}
impl PrimalId {

    pub fn new(id: impl Into<&str>, name: impl Into<&str>, version: impl Into<&str>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
        }
    }

    pub fn from_id(id: impl Into<&str>) -> Self {
        let id = id.into();
        Self {
            name: id.clone(),
            version: "1.0.0".to_string(),
            id,
        }
    }

    pub fn toadstool() -> Self {
        Self::new("toadstool", "ToadStool", "1.0.0")
    }

    pub fn songbird() -> Self {
        Self::new("songbird", "SongBird", "1.0.0")
    }

    pub fn beardog() -> Self {
        Self::new("beardog", "BearDog", "1.0.0")
    }

    pub fn nestgate() -> Self {
        Self::new("nestgate", "NestGate", "1.0.0")
    }

    pub fn squirrel() -> Self {
        Self::new("squirrel", "Squirrel", "1.0.0")
    }

    pub fn biomeos() -> Self {
        Self::new("biomeos", "BiomeOS", "1.0.0")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalRegistration {

    pub id: PrimalId,

    pub discovery_endpoint: String,

    pub registered_at: chrono::DateTime<chrono::Utc>,

    pub last_seen: chrono::DateTime<chrono::Utc>,

    pub metadata: HashMap<String, String>,
}

pub struct UniversalPrimalRegistry<P: PrimalProvider = DefaultPrimalProvider> {

    primals: Arc<RwLock<HashMap<String, PrimalRegistration>>>,

    providers: Arc<RwLock<HashMap<String, Arc<P>>>>,
}

impl<P: PrimalProvider> UniversalPrimalRegistry<P> {

    pub fn new() -> Self {
        Self {
            primals: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            providers: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
    }

    pub async fn register_primal(
        &self,
        id: PrimalId,
        discovery_endpoint: &str,
        metadata: HashMap<&str, &str>,
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
    }

    pub async fn register_provider(&self, provider: Arc<P>) -> BearDogResult<()> {
        let ecosystem_id = provider.ecosystem_id().to_string();
        let instance_id = provider.instance_id().to_string();
        let key = format!("{ecosystem_id}:{instance_id}");
        let mut providers = self.providers.write().await;
        providers.insert(key, provider);
        Ok(())
    }

    pub async fn discover_primal(&self, id: &str) -> BearDogResult<Option<PrimalRegistration>> {
        let primals = self.primals.read().await;
        Ok(primals.get(id).cloned())
    }

    pub async fn get_provider(
        &self,
        ecosystem_id: &str,
        instance_id: &str,
    ) -> BearDogResult<Option<Arc<P>>> {
        let key = format!("{ecosystem_id}:{instance_id}");
        let providers = self.providers.read().await;
        Ok(providers.get(&key).cloned())
    }

    pub async fn list_primals(&self) -> BearDogResult<Vec<PrimalRegistration>> {
        let primals = self.primals.read().await;
        Ok(primals.values().cloned().collect())
    }

    pub async fn list_providers(&self) -> BearDogResult<Vec<Arc<P>>> {
        let providers = self.providers.read().await;
        Ok(providers.values().cloned().collect())
    }

    pub async fn unregister_primal(&self, id: &str) -> BearDogResult<()> {
        let mut primals = self.primals.write().await;
        primals.remove(id);
        Ok(())
    }

    pub async fn unregister_provider(
        &self,
        ecosystem_id: &str,
        instance_id: &str,
    ) -> BearDogResult<()> {
        let key = format!("{ecosystem_id}:{instance_id}");
        let mut providers = self.providers.write().await;
        providers.remove(&key);
        Ok(())
    }

    pub async fn update_last_seen(&self, id: &str) -> BearDogResult<()> {
        let mut primals = self.primals.write().await;
        if let Some(registration) = primals.get_mut(id) {
            registration.last_seen = chrono::Utc::now();
        }
        Ok(())
    }
}

impl<P: PrimalProvider> Default for UniversalPrimalRegistry<P> {
    fn default() -> Self {
        Self::new()
    }
}

static GLOBAL_REGISTRY: tokio::sync::OnceCell<Arc<UniversalPrimalRegistry>> =
    tokio::sync::OnceCell::const_new();

pub async fn global_registry() -> Arc<UniversalPrimalRegistry> {
    GLOBAL_REGISTRY
        .get_or_init(|| async { Arc::new(UniversalPrimalRegistry::new()) })
        .await
        .clone()
}

pub async fn register_primal_type(
    id: PrimalId,
    discovery_endpoint: &str,
    metadata: HashMap<&str, &str>,
) -> BearDogResult<()> {
    let registry = global_registry().await;
    registry
        .register_primal(id, discovery_endpoint, metadata)
        .await
}

pub async fn register_provider(provider: Arc<DefaultPrimalProvider>) -> BearDogResult<()> {
    let registry = global_registry().await;
    registry.register_provider(provider).await
}

pub async fn discover_primal_type(id: &str) -> BearDogResult<Option<PrimalRegistration>> {
    let registry = global_registry().await;
    registry.discover_primal(id).await
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_universal_primal_registry() {
        let registry = UniversalPrimalRegistry::new();

        let custom_primal = PrimalId::new("custom-ai", "CustomAI", "0.1.0");
        let result = registry
            .register_primal(
                custom_primal.clone(),
                "https://custom-ai.example.com".to_string(),
                HashMap::with_capacity(16),
            )
            .await;
        assert!(result.is_ok());

        let discovered = registry.discover_primal("custom-ai").await;
        assert!(discovered.is_ok());
        let discovered = discovered.map_err(|e| BearDogError::internal(format_args!("Failed to discover primal: {}", e).to_string()))?;
        assert!(discovered.is_some());

        let all_primals = registry.list_primals().await;
        assert!(all_primals.is_ok());
        assert_eq!(all_primals.map_err(|e| BearDogError::internal(format_args!("Failed to list primals: {}", e).to_string()))?.len(), 1);
    }

    #[tokio::test]
    async fn test_primal_id_convenience_methods() {
        let beardog = PrimalId::beardog();
        assert_eq!(beardog.id, "beardog");
        assert_eq!(beardog.name, "BearDog");
        assert_eq!(beardog.version, "1.0.0");
        
        let custom = PrimalId::from_id("my-custom-primal");
        assert_eq!(custom.id, "my-custom-primal");
        assert_eq!(custom.name, "my-custom-primal");
        assert_eq!(custom.version, "1.0.0");
    }

    #[tokio::test]
    async fn test_zero_cost_provider_registration() {
        let registry = UniversalPrimalRegistry::new();

        let provider = Arc::new(DefaultPrimalProvider {
            ecosystem_id: "test-ecosystem".to_string(),
            instance_id: "test-instance".to_string(),
        });

        let result = registry.register_provider(provider.clone()).await;
        assert!(result.is_ok());

        let retrieved = registry.get_provider("test-ecosystem", "test-instance").await;
        assert!(retrieved.is_ok());
        assert!(retrieved.map_err(|e| BearDogError::internal(format_args!("Failed to retrieve primal: {}", e).to_string()))?.is_some());

        let all_providers = registry.list_providers().await;
        assert!(all_providers.is_ok());
        assert_eq!(all_providers.map_err(|e| BearDogError::internal(format_args!("Failed to list providers: {}", e).to_string()))?.len(), 1);
    }
}
