// SPDX-License-Identifier: AGPL-3.0-or-later

//! Software HSM key storage

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Software key store
#[derive(Clone)]
pub struct SoftwareKeyStore {
    keys: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl SoftwareKeyStore {
    /// Create new key store
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// # Errors
    ///
    /// Returns an error if key deletion fails in the underlying HSM provider.
    /// Store a key
    pub async fn store_key(&self, key_id: String, key_data: Vec<u8>) -> Result<(), BearDogError> {
        let mut keys = self.keys.write().await;
        keys.insert(key_id, key_data);
        Ok(())
    }

    /// # Errors
    ///
    /// Returns an error if key deletion fails in the underlying HSM provider.
    /// Retrieve a key
    pub async fn get_key(&self, key_id: &str) -> Result<Option<Vec<u8>>, BearDogError> {
        let keys = self.keys.read().await;
        Ok(keys.get(key_id).cloned())
    }

    /// # Errors
    ///
    /// Returns an error if key deletion fails in the underlying HSM provider.
    /// Delete a key
    pub async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        let mut keys = self.keys.write().await;
        keys.remove(key_id);
        Ok(())
    }

    /// # Errors
    ///
    /// Returns an error if key deletion fails in the underlying HSM provider.
    /// List all key IDs
    pub async fn list_keys(&self) -> Result<Vec<String>, BearDogError> {
        let keys = self.keys.read().await;
        Ok(keys.keys().cloned().collect())
    }
}

impl Default for SoftwareKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_keystore_operations() {
        let store = SoftwareKeyStore::new();
        
        // Store a key
        store.store_key("test-key".to_string(), vec![1, 2, 3, 4]).await?;
        
        // Retrieve the key
        let key = store.get_key("test-key").await?;
        assert_eq!(key, Some(vec![1, 2, 3, 4]));
        
        // List keys
        let keys = store.list_keys().await?;
        assert_eq!(keys.len(), 1);
        
        // Delete the key
        store.delete_key("test-key").await?;
        let key = store.get_key("test-key").await?;
        assert_eq!(key, None);
    }
}
