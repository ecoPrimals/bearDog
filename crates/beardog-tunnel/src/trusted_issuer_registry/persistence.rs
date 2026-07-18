// SPDX-License-Identifier: AGPL-3.0-or-later

use base64::Engine;
use ed25519_dalek::VerifyingKey;
use std::path::{Path, PathBuf};

use super::did::did_matches_key;
use super::registry::TrustedIssuerRegistry;
use super::types::{IssuerInfo, PersistedIssuer, PersistedRegistry, TrustMethod};

impl TrustedIssuerRegistry {
    /// Default on-disk path for the trusted issuer registry.
    ///
    /// Uses `$XDG_DATA_HOME/beardog/trusted_issuers.json` via [`directories::ProjectDirs`],
    /// falling back to `$HOME/.local/share/beardog/trusted_issuers.json`.
    #[must_use]
    pub fn save_path() -> PathBuf {
        directories::ProjectDirs::from("org", "beardog", "beardog").map_or_else(
            || {
                directories::BaseDirs::new().map_or_else(
                    || PathBuf::from(".local/share/beardog/trusted_issuers.json"),
                    |d| {
                        d.home_dir()
                            .join(".local/share/beardog/trusted_issuers.json")
                    },
                )
            },
            |dirs| dirs.data_dir().join("trusted_issuers.json"),
        )
    }

    /// Serialize the registry to JSON at `path`.
    ///
    /// Parent directories are created if they do not exist.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if the file cannot be written.
    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let inner = self
            .inner
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        let issuers: Vec<PersistedIssuer> = inner
            .issuers
            .values()
            .map(|(vk, info)| PersistedIssuer {
                did: info.did.clone(),
                public_key_b64: base64::engine::general_purpose::STANDARD.encode(vk.as_bytes()),
                gate_id: info.gate_id.clone(),
                family_id: info.family_id.clone(),
                trust_method: info.trust_method.as_str().to_owned(),
                registered_at: info.registered_at,
            })
            .collect();

        let persisted = PersistedRegistry {
            version: 1,
            issuers,
        };

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(&persisted)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }

    /// Load a registry from JSON at `path`.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if the file cannot be read or contains invalid data.
    pub fn load_from_file(path: &Path) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(path)?;
        let persisted: PersistedRegistry = serde_json::from_str(&contents)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let registry = Self::new();
        {
            let mut inner = registry
                .inner
                .write()
                .unwrap_or_else(std::sync::PoisonError::into_inner);

            for entry in persisted.issuers {
                let key_bytes = base64::engine::general_purpose::STANDARD
                    .decode(&entry.public_key_b64)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                let key_array: [u8; 32] = key_bytes.try_into().map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "public_key must be 32 bytes",
                    )
                })?;
                let vk = VerifyingKey::from_bytes(&key_array)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

                if !did_matches_key(&entry.did, &vk) {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("DID does not match public key for {}", entry.did),
                    ));
                }

                let trust_method = TrustMethod::parse(&entry.trust_method).ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("unknown trust_method: {}", entry.trust_method),
                    )
                })?;

                let info = IssuerInfo {
                    did: entry.did.clone(),
                    gate_id: entry.gate_id,
                    family_id: entry.family_id,
                    registered_at: entry.registered_at,
                    trust_method,
                };
                inner.issuers.insert(entry.did, (vk, info));
            }
        }

        Ok(registry)
    }
}
