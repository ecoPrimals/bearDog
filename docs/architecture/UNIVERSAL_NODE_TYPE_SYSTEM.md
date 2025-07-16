# Universal Node Type System

## Overview

The BearDog node registry now features a **universal and agnostic node type system** that can support any node type - both current and future. This system replaces the previous hardcoded `NodeRole` enum with a flexible, extensible architecture that enables through node types and any others in the future.

## Key Features

### 🌟 Universal & Agnostic
- **String-based identification**: Node types are identified by flexible string names
- **Capability-based registration**: Node types are defined by their capabilities, not hardcoded enums
- **Fully extensible**: New node types can be added without code changes
- **Backward compatible**: All existing node types continue to work

### 🔧 Core Components

#### 1. NodeType Struct
```rust
pub struct NodeType {
    pub type_name: String,              // e.g., "security", "phonebook", "compute"
    pub display_name: String,           // Human-readable name
    pub default_capabilities: Vec<String>, // Default capabilities for this type
    pub metadata: HashMap<String, String>, // Additional metadata
}
```

#### 2. NodeTypeRegistry
```rust
pub struct NodeTypeRegistry {
    types: HashMap<String, NodeType>,
    capability_index: HashMap<String, Vec<String>>,
}
```

#### 3. Backward Compatibility Constants
```rust
pub mod node_types {
    pub const SECURITY: &str = "security";
    pub const PHONEBOOK: &str = "phonebook";
    pub const FEDERATION: &str = "federation";
    pub const COMPUTE: &str = "compute";
    pub const STORAGE: &str = "storage";
    pub const RELAY: &str = "relay";
    pub const BACKUP: &str = "backup";
    
    pub fn security() -> NodeType { /* ... */ }
    pub fn compute() -> NodeType { /* ... */ }
    // ... etc
}
```

## Usage Examples

### Basic Node Type Creation
```rust
use beardog::node_registry::{NodeType, NodeTypeRegistry, node_types};

// Using backward-compatible constants
let config = RegistryConfig {
    node_type: node_types::SECURITY.to_string(),
    // ... other config
};

// Creating custom node types
let custom_node = NodeType::with_capabilities(
    "ai_processor".to_string(),
    "AI Processing Node".to_string(),
    vec![
        "machine-learning".to_string(),
        "model-inference".to_string(),
        "data-analysis".to_string(),
    ],
);
```

### Registry Configuration
```rust
let mut registry = NodeTypeRegistry::new();

// Register a new node type
registry.register_type(custom_node)?;

// Find node types by capability
let ml_nodes = registry.find_types_by_capability("machine-learning");

// Check if a type exists
if registry.type_exists("ai_processor") {
    println!("AI processor node type is registered");
}
```

### Advanced Example: Future Node Types
```rust
// Example: Register a completely new node type for future use
let quantum_node = NodeType::with_capabilities(
    "quantum_compute".to_string(),
    "Quantum Computing Node".to_string(),
    vec![
        "quantum-processing".to_string(),
        "quantum-entanglement".to_string(),
        "quantum-cryptography".to_string(),
    ],
);

// Add metadata
let mut quantum_node = quantum_node;
quantum_node.add_metadata("quantum_backend".to_string(), "qiskit".to_string());
quantum_node.add_metadata("qubit_count".to_string(), "128".to_string());

// Register it
registry.register_type(quantum_node)?;
```

## Standard Node Types

The system comes with pre-registered standard node types:

| Type | String ID | Capabilities |
|------|-----------|-------------|
| **Security** | `"security"` | authentication, authorization, threat-detection, crypto, audit |
| **Phonebook** | `"phonebook"` | discovery, registration, heartbeat, service-advertisement |
| **Federation** | `"federation"` | federation, cross-registry, trust-propagation, discovery |
| **Compute** | `"compute"` | compute, task-execution, resource-sharing, genetic-spawning |
| **Storage** | `"storage"` | storage, data-persistence, backup, replication |
| **Relay** | `"relay"` | relay, routing, traffic-forwarding, network-bridge |
| **Backup** | `"backup"` | backup, archival, long-term-storage, disaster-recovery |

## Migration Guide

### From Old System
```rust
// OLD: Hardcoded enum
pub enum NodeRole {
    SecurityNode,
    PhonebookNode,
    // ...
}

let config = RegistryConfig {
    node_role: NodeRole::SecurityNode,
    // ...
};
```

### To New System
```rust
// NEW: Universal string-based system
use beardog::node_registry::node_types;

let config = RegistryConfig {
    node_type: node_types::SECURITY.to_string(),
    node_type_registry: NodeTypeRegistry::default(),
    // ...
};
```

## Extending the System

### Adding New Node Types at Runtime
```rust
// Example: Add a new node type for IoT devices
let iot_node = NodeType::with_capabilities(
    "iot_gateway".to_string(),
    "IoT Gateway Node".to_string(),
    vec![
        "device-management".to_string(),
        "sensor-data".to_string(),
        "edge-computing".to_string(),
        "protocol-translation".to_string(),
    ],
);

// Register it
registry.register_type(iot_node)?;

// Now you can use it
let config = RegistryConfig {
    node_type: "iot_gateway".to_string(),
    // ...
};
```

### Creating Domain-Specific Node Types
```rust
// Example: Blockchain-specific node types
let blockchain_validator = NodeType::with_capabilities(
    "blockchain_validator".to_string(),
    "Blockchain Validator Node".to_string(),
    vec![
        "block-validation".to_string(),
        "consensus-participation".to_string(),
        "transaction-verification".to_string(),
        "smart-contract-execution".to_string(),
    ],
);

let blockchain_storage = NodeType::with_capabilities(
    "blockchain_storage".to_string(),
    "Blockchain Storage Node".to_string(),
    vec![
        "distributed-ledger".to_string(),
        "immutable-storage".to_string(),
        "chain-synchronization".to_string(),
        "historical-data".to_string(),
    ],
);
```

## API Reference

### NodeType Methods
```rust
impl NodeType {
    pub fn new(type_name: String, display_name: String) -> Self
    pub fn with_capabilities(type_name: String, display_name: String, capabilities: Vec<String>) -> Self
    pub fn add_capability(&mut self, capability: String)
    pub fn add_metadata(&mut self, key: String, value: String)
    pub fn has_capability(&self, capability: &str) -> bool
}
```

### NodeTypeRegistry Methods
```rust
impl NodeTypeRegistry {
    pub fn new() -> Self
    pub fn register_type(&mut self, node_type: NodeType) -> BearDogResult<()>
    pub fn get_type(&self, type_name: &str) -> Option<&NodeType>
    pub fn list_types(&self) -> Vec<&NodeType>
    pub fn find_types_by_capability(&self, capability: &str) -> Vec<&NodeType>
    pub fn type_exists(&self, type_name: &str) -> bool
}
```

## Benefits

### 🎯 Flexibility
- **No code changes needed** for new node types
- **Runtime registration** of node types
- **Capability-based discovery** for smart matching

### 🔧 Extensibility
- **Future-proof** - supports any node type
- **Metadata support** for rich node descriptions
- **Capability indexing** for fast lookups

### 🔄 Compatibility
- **Backward compatible** with existing code
- **Smooth migration** path from old system
- **Existing APIs unchanged**

### 🌐 Universality
- **Any protocol** can define node types
- **Any ecosystem** can extend the system
- **Any future technology** can be integrated

## Conclusion

The universal node type system makes BearDog truly **universal and agnostic**, enabling support for any node types now and in the future. The system is designed to be:

- **Extensible**: New node types can be added without code changes
- **Flexible**: String-based identification supports any naming scheme
- **Capable**: Capability-based registration enables smart discovery
- **Compatible**: Existing code continues to work unchanged
- **Future-proof**: Ready for any future node type requirements

This architecture ensures that everyone can have their own BearDog with their own registry that can share and connect with any other BearDog instance, regardless of the node types they support. 