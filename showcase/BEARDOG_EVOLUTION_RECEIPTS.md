# BearDog Evolution: Receipt System

## Discovery (Dec 19, 2025)

**Problem Found**: Showcase demos revealed that BearDog CLI has NO formal receipt generation!
- Key operations just print success messages to stdout
- No structured JSON receipts created
- No verification/validation capability
- Showcase demo script was manually creating fake "receipts"

**However**: BearDog codebase has RICH audit infrastructure!
- `beardog-monitoring` has comprehensive `AuditEvent` with integrity validation
- `beardog-compliance` has `AuditEntry` and compliance tracking
- `beardog-tunnel` HSM has `AuditLogEntry` 
- **But CLI doesn't use any of it!**

## Proposed Solution: Universal Receipt System

### 1. Create Receipt Type (beardog-types)

```rust
// crates/beardog-types/src/receipt.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Universal operation receipt for all BearDog operations
/// Provides verifiable proof of operation execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationReceipt {
    /// Unique receipt identifier
    pub receipt_id: String,
    
    /// Operation type (key-generate, key-derive, key-mix, etc.)
    pub operation: String,
    
    /// ISO 8601 timestamp
    pub timestamp: String,
    
    /// Operation result
    pub result: OperationResult,
    
    /// Key details (if applicable)
    pub key_info: Option<KeyInfo>,
    
    /// HSM details (if applicable)
    pub hsm_info: Option<HsmInfo>,
    
    /// Optional metadata
    pub metadata: HashMap<String, serde_json::Value>,
    
    /// Cryptographic signature for verification
    pub signature: Option<String>,
    
    /// Chain reference to previous receipt (for lineage)
    pub previous_receipt_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationResult {
    Success,
    Failure { error: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    pub key_id: String,
    pub algorithm: String,
    pub generation: u32,
    pub parent_key_id: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmInfo {
    pub name: String,
    pub vendor: Option<String>,
    pub model: Option<String>,
}

impl OperationReceipt {
    /// Create a new receipt
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            receipt_id: Uuid::new_v4().to_string(),
            operation: operation.into(),
            timestamp: Utc::now().to_rfc3339(),
            result: OperationResult::Success,
            key_info: None,
            hsm_info: None,
            metadata: HashMap::new(),
            signature: None,
            previous_receipt_id: None,
        }
    }
    
    /// Builder pattern for easy construction
    pub fn with_key_info(mut self, key_info: KeyInfo) -> Self {
        self.key_info = Some(key_info);
        self
    }
    
    pub fn with_hsm_info(mut self, hsm_info: HsmInfo) -> Self {
        self.hsm_info = Some(hsm_info);
        self
    }
    
    pub fn with_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }
    
    pub fn with_parent_receipt(mut self, parent_id: impl Into<String>) -> Self {
        self.previous_receipt_id = Some(parent_id.into());
        self
    }
    
    /// Save receipt to file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    /// Validate receipt structure
    pub fn validate(&self) -> Result<(), String> {
        if self.receipt_id.is_empty() {
            return Err("Missing receipt_id".to_string());
        }
        if self.operation.is_empty() {
            return Err("Missing operation".to_string());
        }
        if self.timestamp.is_empty() {
            return Err("Missing timestamp".to_string());
        }
        Ok(())
    }
}
```

### 2. Integrate into CLI Handlers

**Pattern to add to ALL handlers**:

```rust
// In each handler function (key_generate, key_derive, key_mix, etc.)

// Create receipt
let receipt = OperationReceipt::new("key-generate")
    .with_key_info(KeyInfo {
        key_id: key_id.to_string(),
        algorithm: algorithm.to_string(),
        generation: 0,
        parent_key_id: None,
        expires_at: None,
    })
    .with_hsm_info(HsmInfo {
        name: selected_hsm.name.clone(),
        vendor: Some(selected_hsm.vendor.clone()),
        model: selected_hsm.model.clone(),
    })
    .with_metadata("entropy_source", json!("human"));

// Save receipt
let receipt_path = std::path::Path::new("receipts")
    .join(format!("receipt-{}-{}.json", operation, Utc::now().format("%Y%m%d-%H%M%S")));
std::fs::create_dir_all("receipts")?;
receipt.save_to_file(&receipt_path)?;

println!("📜 Receipt saved: {}", receipt_path.display());
```

### 3. Add CLI Receipt Management Commands

```bash
# List receipts
beardog receipt list [--operation <type>] [--date <date>]

# Show receipt details
beardog receipt show <receipt-id>

# Validate receipt
beardog receipt validate <receipt-file>

# Export receipt chain (full lineage)
beardog receipt chain <key-id> --output lineage.json

# Verify receipt signature
beardog receipt verify <receipt-file>
```

### 4. Benefits

✅ **Verifiable Operations**: Every operation has cryptographic proof
✅ **Audit Trail**: Complete history of all operations
✅ **Compliance**: Meets regulatory requirements for audit logs
✅ **Debugging**: Clear trace of what happened when
✅ **Lineage**: Track key derivation chains
✅ **Sovereign**: Receipts are portable, human-readable JSON
✅ **Integration**: Works with existing BearDog audit infrastructure

### 5. Implementation Plan

1. **Phase 1**: Create receipt type in `beardog-types`
2. **Phase 2**: Add receipt generation to all CLI handlers
3. **Phase 3**: Add receipt management commands
4. **Phase 4**: Add signature verification
5. **Phase 5**: Integrate with existing audit infrastructure

### 6. Testing Strategy

- Unit tests for receipt creation/validation
- Integration tests for receipt persistence
- E2E tests for full operation+receipt flow
- Chaos tests for receipt corruption detection

## Status

- [x] Problem identified
- [x] Design documented
- [ ] Implementation (next step)
- [ ] Testing
- [ ] Documentation

## Files to Create/Modify

### Create:
- `crates/beardog-types/src/receipt.rs`
- `crates/beardog-cli/src/handlers/receipt.rs`

### Modify:
- `crates/beardog-types/src/lib.rs` (add receipt module)
- `crates/beardog-cli/src/handlers/key.rs` (add receipt generation)
- `crates/beardog-cli/src/handlers/key_derive.rs` (add receipt generation)
- `crates/beardog-cli/src/handlers/key_mix.rs` (add receipt generation)
- `crates/beardog-cli/src/handlers/key_delegate.rs` (add receipt generation)
- `crates/beardog-cli/src/main_new.rs` (add receipt commands)

---

**This is a MAJOR evolution that makes BearDog enterprise-grade!**

