

use super::primal_types::*;

#[allow(async_fn_in_trait)]
pub trait EcoPrimal: Send + Sync {

    fn metadata(&self) -> &PrimalMetadata;

    fn capabilities(&self) -> Vec<PrimalCapability>;

    async fn initialize(&self, config: &PrimalIntegrationConfig) -> Result<(), PrimalError>;

    async fn handle_request(&self, request: PrimalRequest) -> Result<PrimalResponse, PrimalError>;

    async fn health_check(&self) -> PrimalHealth;

    async fn shutdown(&self) -> Result<(), PrimalError>;
} 
