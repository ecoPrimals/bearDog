// SPDX-License-Identifier: AGPL-3.0-or-later

//! In-memory credential store backend (dev/test, non-persistent).

use beardog_errors::BearDogError;
use beardog_traits::unified::storage::{CredentialStore, SecretMetadata};
use parking_lot::RwLock;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct Entry {
    value: String,
    stored_at: String,
}

/// Volatile in-memory credential store. Secrets are lost on process restart.
///
/// Suitable for development, testing, and as a bootstrap cache before a
/// persistent backend (file vault or platform-native) is available.
#[derive(Debug)]
pub struct InMemoryCredentialStore {
    entries: RwLock<BTreeMap<String, Entry>>,
}

impl InMemoryCredentialStore {
    /// Create an empty in-memory credential store.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: RwLock::new(BTreeMap::new()),
        }
    }
}

impl Default for InMemoryCredentialStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialStore for InMemoryCredentialStore {
    async fn store(&self, name: &str, value: &str) -> Result<(), BearDogError> {
        let entry = Entry {
            value: value.to_string(),
            stored_at: chrono::Utc::now().to_rfc3339(),
        };
        self.entries.write().insert(name.to_string(), entry);
        Ok(())
    }

    async fn retrieve(&self, name: &str) -> Result<(String, SecretMetadata), BearDogError> {
        let entries = self.entries.read();
        let entry = entries
            .get(name)
            .ok_or_else(|| BearDogError::system(format!("Secret '{name}' not found")))?;
        Ok((
            entry.value.clone(),
            SecretMetadata {
                stored_at: entry.stored_at.clone(),
            },
        ))
    }

    async fn list(&self) -> Result<Vec<String>, BearDogError> {
        Ok(self.entries.read().keys().cloned().collect())
    }

    async fn delete(&self, name: &str) -> Result<bool, BearDogError> {
        Ok(self.entries.write().remove(name).is_some())
    }

    fn backend_id(&self) -> &'static str {
        "in-memory"
    }

    fn is_persistent(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn roundtrip_store_retrieve() {
        let store = InMemoryCredentialStore::new();
        store.store("api-key", "sk-test-123").await.unwrap();

        let (val, meta) = store.retrieve("api-key").await.unwrap();
        assert_eq!(val, "sk-test-123");
        assert!(!meta.stored_at.is_empty());
    }

    #[tokio::test]
    async fn retrieve_missing_returns_error() {
        let store = InMemoryCredentialStore::new();
        assert!(store.retrieve("nope").await.is_err());
    }

    #[tokio::test]
    async fn list_and_delete() {
        let store = InMemoryCredentialStore::new();
        store.store("a", "1").await.unwrap();
        store.store("b", "2").await.unwrap();

        let names = store.list().await.unwrap();
        assert_eq!(names, vec!["a", "b"]);

        assert!(store.delete("a").await.unwrap());
        assert!(!store.delete("a").await.unwrap());

        let names = store.list().await.unwrap();
        assert_eq!(names, vec!["b"]);
    }

    #[tokio::test]
    async fn overwrite_replaces_value() {
        let store = InMemoryCredentialStore::new();
        store.store("k", "v1").await.unwrap();
        store.store("k", "v2").await.unwrap();

        let (val, _) = store.retrieve("k").await.unwrap();
        assert_eq!(val, "v2");
    }

    #[test]
    fn backend_identity() {
        let store = InMemoryCredentialStore::new();
        assert_eq!(store.backend_id(), "in-memory");
        assert!(!store.is_persistent());
    }
}
