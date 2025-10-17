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

    /// Store a key
    pub async fn store_key(&self, key_id: String, key_data: Vec<u8>) -> Result<(), BearDogError> {
        let mut keys = self.keys.write().await;
        keys.insert(key_id, key_data);
        Ok(())
    }

    /// Retrieve a key
    pub async fn get_key(&self, key_id: &str) -> Result<Option<Vec<u8>>, BearDogError> {
        let keys = self.keys.read().await;
        Ok(keys.get(key_id).cloned())
    }

    /// Delete a key
    pub async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        let mut keys = self.keys.write().await;
        keys.remove(key_id);
        Ok(())
    }

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
        store.store_key("test-key".to_string(), vec![1, 2, 3, 4]).await.unwrap();
        
        // Retrieve the key
        let key = store.get_key("test-key").await.unwrap();
        assert_eq!(key, Some(vec![1, 2, 3, 4]));
        
        // List keys
        let keys = store.list_keys().await.unwrap();
        assert_eq!(keys.len(), 1);
        
        // Delete the key
        store.delete_key("test-key").await.unwrap();
        let key = store.get_key("test-key").await.unwrap();
        assert_eq!(key, None);
    }
}
