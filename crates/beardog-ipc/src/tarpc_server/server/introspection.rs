// SPDX-License-Identifier: AGPL-3.0-or-later

//! Primal introspection: identity, capabilities, health.

use beardog_types::constants::domains::network::ipc_discovery::BEARDOG_CAPABILITY_DOMAIN;

use crate::tarpc_types::{Capability, HealthStatus, MethodInfo, PrimalInfo};

use super::BearDogCryptoServer;

pub(super) async fn primal_info(server: &BearDogCryptoServer) -> PrimalInfo {
    PrimalInfo {
        name: server.primal_name.clone(),
        version: server.version.clone(),
        family: server.primal_family.clone(),
        capabilities: vec![
            BEARDOG_CAPABILITY_DOMAIN.to_string(),
            "signatures".to_string(),
            "encryption".to_string(),
            "hashing".to_string(),
            "key-exchange".to_string(),
            "tls".to_string(),
            "genetic".to_string(),
        ],
    }
}

pub(super) async fn rpc_methods() -> Vec<MethodInfo> {
    vec![
        MethodInfo {
            name: "generate_ed25519".to_string(),
            description: "Generate Ed25519 keypair".to_string(),
            params: vec![],
        },
        MethodInfo {
            name: "sign_ed25519".to_string(),
            description: "Sign data with Ed25519".to_string(),
            params: vec!["data".to_string(), "private_key".to_string()],
        },
        MethodInfo {
            name: "chacha20_poly1305_encrypt".to_string(),
            description: "Encrypt with ChaCha20-Poly1305".to_string(),
            params: vec!["plaintext".to_string(), "key".to_string()],
        },
        MethodInfo {
            name: "blake3_hash".to_string(),
            description: "Hash with BLAKE3".to_string(),
            params: vec!["data".to_string()],
        },
        // ... more methods would be listed here
    ]
}

pub(super) async fn primal_capabilities() -> Vec<Capability> {
    vec![
        Capability {
            name: "crypto.signatures.ed25519".to_string(),
            version: "1.0".to_string(),
        },
        Capability {
            name: "crypto.signatures.ecdsa".to_string(),
            version: "1.0".to_string(),
        },
        Capability {
            name: "crypto.encryption.chacha20-poly1305".to_string(),
            version: "1.0".to_string(),
        },
        Capability {
            name: "crypto.encryption.aes-256-gcm".to_string(),
            version: "1.0".to_string(),
        },
        Capability {
            name: "crypto.hashing.blake3".to_string(),
            version: "1.0".to_string(),
        },
        Capability {
            name: "crypto.key-exchange.x25519".to_string(),
            version: "1.0".to_string(),
        },
        Capability {
            name: "crypto.tls.1.3".to_string(),
            version: "1.0".to_string(),
        },
        Capability {
            name: "genetic.lineage".to_string(),
            version: "1.0".to_string(),
        },
    ]
}

pub(super) async fn health(server: &BearDogCryptoServer) -> HealthStatus {
    HealthStatus {
        status: "healthy".to_string(),
        version: server.version.clone(),
        uptime_seconds: server.start_time.elapsed().as_secs(),
    }
}
