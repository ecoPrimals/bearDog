// SPDX-License-Identifier: AGPL-3.0-only

//! Generic `SecureTunnelProvider` Implementation
//!
//! This is the primary interface for BearDog's secure tunnel capability.
//! Contains the real implementation of all tunnel operations using genetic
//! cryptography and TOFU trust management.

use async_trait::async_trait;

use super::tunnel::Tunnel;
use super::types::TrustLevel;
use super::BeardogBtspProvider;
use beardog_capabilities::traits::{
    PeerEndpoint, SecureTunnelProvider, TunnelHandle as CapabilityTunnelHandle,
    TunnelStatus as CapabilityTunnelStatus,
};
use beardog_errors::BearDogError;
use tracing::{debug, info, warn};

#[async_trait]
impl SecureTunnelProvider for BeardogBtspProvider {
    async fn establish_tunnel(
        &self,
        peer: PeerEndpoint,
    ) -> Result<CapabilityTunnelHandle, BearDogError> {
        info!("🌉 Establishing secure tunnel with peer: {}", peer.id);

        // 1. Check peer trust status
        let trust_level = match self.get_peer_trust(&peer.id).await {
            Some(level) => {
                debug!("📋 Peer {} has existing trust: {:?}", peer.id, level);
                self.update_peer_trust(&peer.id).await?;
                level
            }
            None => {
                // TOFU: Trust On First Use
                if let Some(ref public_key) = peer.public_key {
                    self.pin_peer_key(&peer.id, public_key).await?
                } else {
                    warn!("⚠️  Peer {} has no public key - tentative trust", peer.id);
                    TrustLevel::Tentative
                }
            }
        };

        // 2. Generate session key using genetic cryptography
        let session_key = self.generate_session_key(&peer.id).await?;

        // 3. Establish mTLS connection
        self.establish_mtls(&peer, &session_key).await?;

        // 4. Create tunnel
        let tunnel_id = format!("btsp_{}", uuid::Uuid::new_v4().simple());
        let tunnel = Tunnel::new(
            tunnel_id.clone(),
            peer.id.clone(),
            peer.endpoint.clone(),
            session_key,
            trust_level,
        );

        let established_at = tunnel.established_at;

        // 5. Store tunnel
        self.tunnels.write().insert(tunnel_id.clone(), tunnel);

        info!("✅ Secure tunnel established: {} -> {}", peer.id, tunnel_id);

        Ok(CapabilityTunnelHandle {
            id: tunnel_id,
            peer_id: peer.id.clone(),
            established_at: established_at.to_rfc3339(),
        })
    }

    async fn tunnel_encrypt(
        &self,
        handle: &CapabilityTunnelHandle,
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        // Clone session key without holding lock across await
        let session_key = {
            let tunnels = self.tunnels.read();
            let tunnel = tunnels
                .get(&handle.id)
                .ok_or_else(|| BearDogError::invalid_input("Tunnel not found"))?;
            tunnel.session_key.clone()
        };

        // Encrypt with genetic key lineage
        let ciphertext = self.encrypt_with_lineage(data, &session_key).await?;

        // Update statistics
        {
            let tunnels = self.tunnels.read();
            if let Some(tunnel) = tunnels.get(&handle.id) {
                tunnel.add_bytes_sent(ciphertext.len() as u64);
            }
        }

        debug!("🔒 Encrypted {} bytes for tunnel {}", data.len(), handle.id);

        Ok(ciphertext)
    }

    async fn tunnel_decrypt(
        &self,
        handle: &CapabilityTunnelHandle,
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        // Clone session key without holding lock across await
        let session_key = {
            let tunnels = self.tunnels.read();
            let tunnel = tunnels
                .get(&handle.id)
                .ok_or_else(|| BearDogError::invalid_input("Tunnel not found"))?;
            tunnel.session_key.clone()
        };

        // Decrypt with genetic key lineage verification
        let plaintext = self.decrypt_with_lineage(data, &session_key).await?;

        // Update statistics
        {
            let tunnels = self.tunnels.read();
            if let Some(tunnel) = tunnels.get(&handle.id) {
                tunnel.add_bytes_received(data.len() as u64);
            }
        }

        debug!(
            "🔓 Decrypted {} bytes for tunnel {}",
            plaintext.len(),
            handle.id
        );

        Ok(plaintext)
    }

    async fn tunnel_status(
        &self,
        handle: &CapabilityTunnelHandle,
    ) -> Result<CapabilityTunnelStatus, BearDogError> {
        let tunnels = self.tunnels.read();
        let tunnel = tunnels
            .get(&handle.id)
            .ok_or_else(|| BearDogError::invalid_input("Tunnel not found"))?;

        Ok(CapabilityTunnelStatus {
            tunnel_id: handle.id.clone(),
            active: tunnel.is_active(),
            bytes_sent: tunnel.bytes_sent(),
            bytes_received: tunnel.bytes_received(),
            last_activity: tunnel.last_activity().to_rfc3339(),
        })
    }

    async fn close_tunnel(&self, handle: &CapabilityTunnelHandle) -> Result<(), BearDogError> {
        info!("🔒 Closing secure tunnel: {}", handle.id);

        // Get tunnel info before removing
        let (peer_id, bytes_sent, bytes_received) = {
            let tunnels = self.tunnels.read();
            if let Some(tunnel) = tunnels.get(&handle.id) {
                (
                    tunnel.peer_id.clone(),
                    tunnel.bytes_sent(),
                    tunnel.bytes_received(),
                )
            } else {
                return Err(BearDogError::not_found(format!(
                    "Tunnel {} not found",
                    handle.id
                )));
            }
        };

        // Clean up session key from HSM first
        self.cleanup_session_key(&peer_id).await?;

        // Remove tunnel (Drop impl will zeroize session key in memory)
        let mut tunnels = self.tunnels.write();
        tunnels.remove(&handle.id);

        info!(
            "✅ Secure tunnel closed: {} (sent: {} bytes, received: {} bytes)",
            handle.id, bytes_sent, bytes_received
        );

        Ok(())
    }
}
