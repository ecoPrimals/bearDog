// SPDX-License-Identifier: AGPL-3.0-or-later

//! Concrete service-discovery backends (enum dispatch).

use super::core::{
    DiscoveryCapabilities, DiscoveryError, DiscoveryHealthStatus, ServiceDescriptor,
    ServiceDiscoveryCapability,
};
use super::kubernetes::KubernetesDiscovery;
use super::providers::DnsHttpDiscovery;
use crate::canonical::capabilities::ServiceCapabilityType;
use crate::canonical::types::ids::RegistrationId;

/// Resolved service-discovery backend (no `dyn` dispatch).
#[derive(Debug, Clone)]
pub enum ServiceDiscoveryBackend {
    /// DNS + HTTP fallback discovery.
    DnsHttp(DnsHttpDiscovery),
    /// Kubernetes API–oriented discovery.
    Kubernetes(KubernetesDiscovery),
}

impl ServiceDiscoveryCapability for ServiceDiscoveryBackend {
    async fn discover_by_capability(
        &self,
        capability: ServiceCapabilityType,
    ) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        match self {
            Self::DnsHttp(d) => d.discover_by_capability(capability).await,
            Self::Kubernetes(k) => k.discover_by_capability(capability).await,
        }
    }

    async fn discover_by_name(
        &self,
        service_name: &str,
    ) -> Result<Vec<ServiceDescriptor>, DiscoveryError> {
        match self {
            Self::DnsHttp(d) => d.discover_by_name(service_name).await,
            Self::Kubernetes(k) => k.discover_by_name(service_name).await,
        }
    }

    async fn register_service(
        &self,
        descriptor: ServiceDescriptor,
    ) -> Result<RegistrationId, DiscoveryError> {
        match self {
            Self::DnsHttp(d) => d.register_service(descriptor).await,
            Self::Kubernetes(k) => k.register_service(descriptor).await,
        }
    }

    async fn unregister_service(
        &self,
        registration_id: &RegistrationId,
    ) -> Result<(), DiscoveryError> {
        match self {
            Self::DnsHttp(d) => d.unregister_service(registration_id).await,
            Self::Kubernetes(k) => k.unregister_service(registration_id).await,
        }
    }

    async fn renew_registration(
        &self,
        registration_id: &RegistrationId,
    ) -> Result<(), DiscoveryError> {
        match self {
            Self::DnsHttp(d) => d.renew_registration(registration_id).await,
            Self::Kubernetes(k) => k.renew_registration(registration_id).await,
        }
    }

    async fn health_check(&self) -> Result<DiscoveryHealthStatus, DiscoveryError> {
        match self {
            Self::DnsHttp(d) => d.health_check().await,
            Self::Kubernetes(k) => k.health_check().await,
        }
    }

    fn provider_name(&self) -> &str {
        match self {
            Self::DnsHttp(d) => d.provider_name(),
            Self::Kubernetes(k) => k.provider_name(),
        }
    }

    fn capabilities(&self) -> DiscoveryCapabilities {
        match self {
            Self::DnsHttp(d) => d.capabilities(),
            Self::Kubernetes(k) => k.capabilities(),
        }
    }
}
