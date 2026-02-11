//! Generic `SecureTunnelProvider` Implementation
//!
//! This is the primary interface for BearDog's secure tunnel capability.
//! It delegates to the underlying `BtspProvider` implementation while providing
//! the generic capability types required for primal sovereignty.
//!
//! **Deep Debt Principle #6**: Modern production implementation!

use async_trait::async_trait;

use super::types::{Direction, SecurityContext};
use super::{BeardogBtspProvider, BtspProvider};
use beardog_capabilities::traits::{
    PeerEndpoint, SecureTunnelProvider, TunnelHandle as CapabilityTunnelHandle,
    TunnelStatus as CapabilityTunnelStatus,
};
use beardog_errors::BearDogError;

#[allow(deprecated)] // Delegates to BtspProvider internally for backward compat
#[async_trait]
impl SecureTunnelProvider for BeardogBtspProvider {
    async fn establish_tunnel(
        &self,
        peer: PeerEndpoint,
    ) -> Result<CapabilityTunnelHandle, BearDogError> {
        // Delegate to BtspProvider implementation (backward compat)
        let handle = <Self as BtspProvider>::establish_tunnel(self, &peer).await?;

        // No conversion needed - TunnelHandle is the same type
        Ok(handle)
    }

    async fn tunnel_encrypt(
        &self,
        handle: &CapabilityTunnelHandle,
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        let context = SecurityContext {
            tunnel_id: handle.id.clone(),
            direction: Direction::Outbound,
        };
        <Self as BtspProvider>::encrypt(self, data, &context).await
    }

    async fn tunnel_decrypt(
        &self,
        handle: &CapabilityTunnelHandle,
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        let context = SecurityContext {
            tunnel_id: handle.id.clone(),
            direction: Direction::Inbound,
        };
        <Self as BtspProvider>::decrypt(self, data, &context).await
    }

    async fn tunnel_status(
        &self,
        handle: &CapabilityTunnelHandle,
    ) -> Result<CapabilityTunnelStatus, BearDogError> {
        // Delegate - types are the same
        <Self as BtspProvider>::tunnel_status(self, handle).await
    }

    async fn close_tunnel(&self, handle: &CapabilityTunnelHandle) -> Result<(), BearDogError> {
        // Delegate - types are the same
        <Self as BtspProvider>::close_tunnel(self, handle).await
    }
}
