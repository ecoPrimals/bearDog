//! External Primal Service
//!
//! Tarpc service interface for external primal communication

use super::core_types::*;
use beardog_errors::BearDogResult;
use async_trait::async_trait;

/// Tarpc service for external primal communication
/// This is BearDog's interface to communicate with Songbird, ToadStool, and other primals
#[async_trait]
pub trait ExternalPrimalService {
    async fn register_service(&self, req: RegistrationRequest) -> BearDogResult<RegistrationResult>;
    async fn request_capabilities(&self, req: CapabilityRequest) -> BearDogResult<CapabilityResponse>;
    async fn conduct_key_ceremony(&self, req: KeyCeremonyRequest) -> BearDogResult<KeyCeremonyResult>;
    async fn submit_metrics(&self, metrics: HsmMetricsSnapshot) -> BearDogResult<MetricsAckResult>;
    async fn health_check(&self) -> BearDogResult<ServiceHealthStatus>;
}

// TODO: Extract complete external primal service implementation
pub struct RegistrationRequest;
pub struct RegistrationResult;
pub struct CapabilityRequest;
pub struct CapabilityResponse;
pub struct KeyCeremonyRequest;
pub struct KeyCeremonyResult;
pub struct HsmMetricsSnapshot;
pub struct MetricsAckResult;
pub struct ServiceHealthStatus; 