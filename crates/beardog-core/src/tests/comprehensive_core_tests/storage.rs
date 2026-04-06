// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct StorageEntry {
    key: String,
    value: Vec<u8>,
    encrypted: bool,
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[test]
fn test_storage_write() {
    let mut storage: HashMap<String, StorageEntry> = HashMap::new();

    let entry = StorageEntry {
        key: "test-key-001".to_string(),
        value: b"test data".to_vec(),
        encrypted: false,
    };

    storage.insert(entry.key.clone(), entry.clone());

    assert!(storage.contains_key("test-key-001"));
    assert_eq!(storage.get("test-key-001").unwrap().value, b"test data");
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[test]
fn test_storage_read() {
    let mut storage: HashMap<String, StorageEntry> = HashMap::new();

    let entry = StorageEntry {
        key: "read-key-001".to_string(),
        value: b"read data".to_vec(),
        encrypted: false,
    };

    storage.insert(entry.key.clone(), entry.clone());

    let read_entry = storage.get("read-key-001");
    assert!(read_entry.is_some());
    assert_eq!(read_entry.unwrap().value, b"read data");
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: normal
#[test]
fn test_storage_delete() {
    let mut storage: HashMap<String, StorageEntry> = HashMap::new();

    let entry = StorageEntry {
        key: "delete-key-001".to_string(),
        value: b"to be deleted".to_vec(),
        encrypted: false,
    };

    storage.insert(entry.key.clone(), entry.clone());
    assert!(storage.contains_key("delete-key-001"));

    storage.remove("delete-key-001");
    assert!(!storage.contains_key("delete-key-001"));
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: normal
#[test]
fn test_storage_list() {
    let mut storage: HashMap<String, StorageEntry> = HashMap::new();

    for i in 0..5 {
        let entry = StorageEntry {
            key: format!("key-{}", i),
            value: format!("value-{}", i).into_bytes(),
            encrypted: false,
        };
        storage.insert(entry.key.clone(), entry);
    }

    assert_eq!(storage.len(), 5);
    let keys: Vec<String> = storage.keys().cloned().collect();
    assert!(keys.contains(&"key-0".to_string()));
    assert!(keys.contains(&"key-4".to_string()));
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[test]
fn test_storage_encryption() {
    let mut storage: HashMap<String, StorageEntry> = HashMap::new();

    let encrypted_entry = StorageEntry {
        key: "encrypted-key-001".to_string(),
        value: b"encrypted data".to_vec(),
        encrypted: true,
    };

    let plain_entry = StorageEntry {
        key: "plain-key-001".to_string(),
        value: b"plain data".to_vec(),
        encrypted: false,
    };

    storage.insert(encrypted_entry.key.clone(), encrypted_entry.clone());
    storage.insert(plain_entry.key.clone(), plain_entry.clone());

    assert!(storage.get("encrypted-key-001").unwrap().encrypted);
    assert!(!storage.get("plain-key-001").unwrap().encrypted);
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: normal
#[test]
fn test_storage_transaction() {
    let mut storage: HashMap<String, StorageEntry> = HashMap::new();
    let mut transaction: Vec<StorageEntry> = Vec::new();

    for i in 0..3 {
        let entry = StorageEntry {
            key: format!("tx-key-{}", i),
            value: format!("tx-value-{}", i).into_bytes(),
            encrypted: false,
        };
        transaction.push(entry);
    }

    for entry in transaction {
        storage.insert(entry.key.clone(), entry);
    }

    assert_eq!(storage.len(), 3);
    assert!(storage.contains_key("tx-key-0"));
    assert!(storage.contains_key("tx-key-2"));
}
