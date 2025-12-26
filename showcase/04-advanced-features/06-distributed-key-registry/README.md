# 📡 Distributed Key Registry Demo

**Phase**: 4 (Advanced Features)  
**Demo**: 6/10  
**Priority**: 🔥 MEDIUM-HIGH  
**Status**: ✅ COMPLETE

---

## 🎯 Overview

This demo demonstrates a **distributed key registry** for managing cryptographic keys across multiple nodes in a decentralized manner. It validates consensus-based key registration, Byzantine fault-tolerant replication, key discovery across nodes, version control for key updates, and conflict resolution.

### **What You'll Learn**
- Distributed key storage and retrieval
- Consensus mechanisms for key registration
- Byzantine fault tolerance (BFT)
- Key versioning and updates
- Conflict resolution strategies
- Multi-node synchronization
- Eventual consistency models

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│         DISTRIBUTED KEY REGISTRY ARCHITECTURE               │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Node A               Node B               Node C            │
│  ┌──────────┐        ┌──────────┐        ┌──────────┐      │
│  │ Registry │◄──────►│ Registry │◄──────►│ Registry │      │
│  │ Keys: 10 │  Sync  │ Keys: 10 │  Sync  │ Keys: 10 │      │
│  └──────────┘        └──────────┘        └──────────┘      │
│       │                   │                   │              │
│       └───────────────────┴───────────────────┘              │
│                    Consensus Layer                           │
│                                                               │
│  Key Registration Flow:                                      │
│  1. Node A: Propose key registration                         │
│  2. Broadcast to all nodes                                   │
│  3. Nodes vote (2f+1 required for BFT)                       │
│  4. Consensus reached → Commit                               │
│  5. All nodes update local registry                          │
│                                                               │
│  Key Discovery:                                              │
│  - Query any node                                            │
│  - Eventual consistency guarantee                            │
│  - Conflict resolution via version vectors                   │
│                                                               │
│  Byzantine Fault Tolerance:                                  │
│  - Tolerates f faulty nodes in 3f+1 system                   │
│  - Requires 2f+1 votes for consensus                         │
│  - Example: 4 nodes tolerate 1 faulty                        │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔐 Use Cases

### 1. **Multi-Node Key Management**
**Scenario**: Distribute keys across data centers  
**Benefit**: High availability, no single point of failure

### 2. **Key Rotation Coordination**
**Scenario**: Coordinate key updates across all nodes  
**Benefit**: Consistent key versions, smooth transitions

### 3. **Disaster Recovery**
**Scenario**: Recover keys from surviving nodes  
**Benefit**: Resilience against node failures

### 4. **Cross-Organization Key Sharing**
**Scenario**: Share keys securely between organizations  
**Benefit**: Decentralized trust, no central authority

---

## 🚀 Quick Start

```bash
cd showcase/04-advanced-features/06-distributed-key-registry
./run-demo.sh
```

---

## 📊 What Gets Validated

### Key Registration
- ✅ Propose key to registry
- ✅ Broadcast to all nodes
- ✅ Achieve consensus (2f+1 votes)
- ✅ Commit to all nodes
- ✅ Verify consistency

### Key Discovery
- ✅ Query key from any node
- ✅ Retrieve latest version
- ✅ Handle node failures
- ✅ Eventual consistency

### Consensus & BFT
- ✅ Require 2f+1 votes for 3f+1 system
- ✅ Tolerate f faulty nodes
- ✅ Reject invalid proposals
- ✅ Handle network partitions

### Version Control
- ✅ Track key versions
- ✅ Update keys with new versions
- ✅ Resolve conflicts
- ✅ Maintain version history

---

## 🎯 Expected Results

### Performance Targets
- **Key Registration**: < 100ms (3-node consensus)
- **Key Discovery**: < 10ms (local lookup)
- **Synchronization**: < 50ms (node-to-node)
- **Conflict Resolution**: < 20ms

### Fault Tolerance
- **3-node system**: Tolerates 0 faults (majority required)
- **4-node system**: Tolerates 1 fault
- **7-node system**: Tolerates 2 faults
- **Byzantine tolerance**: 2f+1 in 3f+1 system

---

## 🔬 Technical Details

### Consensus Algorithm

**Simplified PBFT (Practical Byzantine Fault Tolerance)**:
1. **Pre-Prepare**: Leader proposes operation
2. **Prepare**: Nodes validate and broadcast prepare
3. **Commit**: If 2f+1 prepares, broadcast commit
4. **Execute**: If 2f+1 commits, execute operation

**Quorum Requirements**:
- **Read Quorum**: f+1 nodes (tolerate f failures)
- **Write Quorum**: 2f+1 nodes (BFT guarantee)

### Version Vectors

Track causality and detect conflicts:
- **Format**: {node_id: version}
- **Example**: {A:3, B:2, C:1}
- **Conflict**: Concurrent updates from different nodes
- **Resolution**: Last-write-wins or semantic merge

### Data Structure

```rust
struct KeyRecord {
    key_id: String,
    public_key: Vec<u8>,
    version: u64,
    version_vector: HashMap<String, u64>,
    metadata: KeyMetadata,
    signatures: Vec<NodeSignature>,
}

struct NodeSignature {
    node_id: String,
    signature: Vec<u8>,
    timestamp: i64,
}
```

---

## 📈 Scalability

### Horizontal Scaling
- **Small**: 3-4 nodes (single data center)
- **Medium**: 7-10 nodes (multi-region)
- **Large**: 10+ nodes (global distribution)

### Trade-offs
- **More nodes**: Higher availability, slower consensus
- **Fewer nodes**: Faster consensus, lower fault tolerance
- **Sweet spot**: 4-7 nodes for most use cases

---

## 🎯 Spec Claims Validated

1. ✅ **Distributed Key Storage**: Multi-node registry
2. ✅ **Consensus Mechanisms**: PBFT-like consensus
3. ✅ **Byzantine Fault Tolerance**: 2f+1 in 3f+1
4. ✅ **Key Versioning**: Version vectors
5. ✅ **Conflict Resolution**: Automatic resolution
6. ✅ **High Availability**: No single point of failure
7. ✅ **Eventual Consistency**: Guaranteed convergence
8. ✅ **Sub-100ms Registration**: Fast consensus

---

## 📚 References

- [Practical Byzantine Fault Tolerance](http://pmg.csail.mit.edu/papers/osdi99.pdf)
- [Dynamo: Amazon's Highly Available Key-value Store](https://www.allthingsdistributed.com/files/amazon-dynamo-sosp2007.pdf)
- [Conflict-free Replicated Data Types (CRDTs)](https://crdt.tech/)

---

**Demo Complete**: Validates distributed key management with consensus, BFT, and version control across multiple nodes.

🐻 **BearDog: Decentralized, Distributed, Resilient!** 📡

