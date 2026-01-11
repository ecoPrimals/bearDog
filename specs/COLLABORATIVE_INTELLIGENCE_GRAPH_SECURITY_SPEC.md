# BearDog Collaborative Intelligence - Graph Security API Specification

**Version**: 1.0.0  
**Date**: January 11, 2026  
**Status**: Committed - Implementation Starting Jan 13, 2026  
**Timeline**: 2 weeks (Jan 13-26, 2026)

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [API Specifications](#api-specifications)
4. [Security Model](#security-model)
5. [Implementation Details](#implementation-details)
6. [Testing Strategy](#testing-strategy)
7. [Integration Points](#integration-points)
8. [Performance Requirements](#performance-requirements)

---

## Overview

### Purpose

BearDog provides the security foundation for biomeOS Collaborative Intelligence by validating graph modifications, templates, and origins. This enables secure human-AI collaboration where users can modify execution graphs in real-time while maintaining system security.

### Scope

**In Scope**:
- Graph modification authorization (real-time)
- Template safety validation
- Template origin verification
- Threat detection and prevention
- User permission validation
- Audit trail generation

**Out of Scope**:
- Graph execution (handled by biomeOS)
- UI/visualization (handled by petalTongue)
- AI suggestions (handled by Squirrel)
- Template storage (handled by NestGate)

### Key Requirements

1. **Real-Time Authorization**: < 10ms latency for graph modifications
2. **Comprehensive Security**: 6 threat categories protected
3. **Transparent Reasoning**: All decisions include explanations
4. **High Throughput**: 10,000+ requests/sec
5. **Production Ready**: Battle-tested, fully documented

---

## Architecture

### System Context

```
┌─────────────────────────────────────────────────────────────┐
│                  Collaborative Intelligence                  │
│                      Ecosystem                               │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐    │
│  │ petalTongue  │   │   Squirrel   │   │  NestGate    │    │
│  │ (UI/Editor)  │   │ (AI/Learn)   │   │  (Storage)   │    │
│  └──────┬───────┘   └──────┬───────┘   └──────┬───────┘    │
│         │                  │                  │             │
│         └──────────────────┼──────────────────┘             │
│                            │                                │
│                    ┌───────▼────────┐                       │
│                    │    BearDog     │                       │
│                    │   (Security)   │                       │
│                    └───────┬────────┘                       │
│                            │                                │
│         ┌──────────────────┼──────────────────┐             │
│         │                  │                  │             │
│  ┌──────▼───────┐   ┌──────▼───────┐   ┌──────▼───────┐    │
│  │   Songbird   │   │  ToadStool   │   │   biomeOS    │    │
│  │ (Discovery)  │   │  (Compute)   │   │  (Orchestr)  │    │
│  └──────────────┘   └──────────────┘   └──────────────┘    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Security Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              BearDog Graph Security Architecture             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Layer 1: Authentication (User Identity)                    │
│  ├─ HSM-backed authentication                               │
│  ├─ Genetic lineage verification                            │
│  └─ Trust score calculation                                 │
│                                                              │
│  Layer 2: Authorization (Permissions)                       │
│  ├─ RBAC permission checks                                  │
│  ├─ Graph ownership verification                            │
│  └─ Action-specific authorization                           │
│                                                              │
│  Layer 3: Validation (Safety)                               │
│  ├─ Graph structure validation                              │
│  ├─ Modification safety checks                              │
│  └─ Template integrity verification                         │
│                                                              │
│  Layer 4: Threat Detection (Malicious Patterns)             │
│  ├─ Known malicious pattern detection                       │
│  ├─ Anomaly detection (unusual structures)                  │
│  └─ Code injection prevention                               │
│                                                              │
│  Layer 5: Audit (Provenance)                                │
│  ├─ Template origin verification                            │
│  ├─ Modification chain validation                           │
│  └─ Trust score propagation                                 │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow

```
User Modification Request
    │
    ├──> 1. Authenticate User (Layer 1)
    │       └──> HSM-backed verification
    │
    ├──> 2. Check Permissions (Layer 2)
    │       └──> RBAC + ownership validation
    │
    ├──> 3. Validate Safety (Layer 3)
    │       └──> Structure + safety checks
    │
    ├──> 4. Detect Threats (Layer 4)
    │       └──> Pattern matching + anomalies
    │
    ├──> 5. Audit Trail (Layer 5)
    │       └──> Log decision + reasoning
    │
    └──> Return Authorization Decision
            ├─ Authorized: true + reasoning
            └─ Denied: false + reasoning + recommendations
```

---

## API Specifications

### Transport

**Protocol**: JSON-RPC 2.0 over Unix Domain Sockets  
**Socket Path**: `/tmp/beardog-{family_id}-{node_id}.sock`  
**Format**: UTF-8 encoded JSON

### Common Types

```typescript
// User identity
type UserId = string;

// Graph structure
type GraphId = string;

interface GraphNode {
  id: string;
  type: string;        // "compute" | "storage" | "ai" | etc.
  primal: string;      // "ToadStool" | "NestGate" | "Squirrel" | etc.
  config: object;      // Node-specific configuration
}

interface GraphEdge {
  from: string;        // Source node ID
  to: string;          // Target node ID
  type?: string;       // "dependency" | "data_flow" | etc.
}

interface Graph {
  id: GraphId;
  owner: UserId;
  nodes: GraphNode[];
  edges: GraphEdge[];
  metadata?: object;
}

// Modification
interface GraphModification {
  action: "add_node" | "remove_node" | "modify_node" | "add_edge" | "remove_edge";
  node?: GraphNode;
  node_id?: string;
  edge?: GraphEdge;
  changes?: object;
}

// Template
interface GraphTemplate {
  id: string;
  name: string;
  creator: UserId;
  nodes: GraphNode[];
  edges: GraphEdge[];
  signature?: string;   // Ed25519 signature (base64)
  metadata: {
    version: string;
    created_at: string;
    description?: string;
  };
}

// Results
type RiskLevel = "low" | "medium" | "high" | "critical";
type ThreatCategory = "structure" | "signature" | "code_injection" | "privilege" | "anomaly";

interface ValidationIssue {
  severity: "low" | "medium" | "high" | "critical";
  category: ThreatCategory;
  description: string;
  location?: string;    // Node ID or edge reference
}
```

---

### Method 1: `graph.authorize_modification`

**Purpose**: Authorize a user's request to modify a graph in real-time.

#### Request

```json
{
  "jsonrpc": "2.0",
  "method": "graph.authorize_modification",
  "params": {
    "user_id": "user-alice",
    "graph": {
      "id": "graph-123",
      "owner": "user-alice",
      "nodes": [
        {
          "id": "node-1",
          "type": "compute",
          "primal": "ToadStool",
          "config": { "cpu": 2, "memory": "4GB" }
        }
      ],
      "edges": []
    },
    "modification": {
      "action": "add_node",
      "node": {
        "id": "node-2",
        "type": "storage",
        "primal": "NestGate",
        "config": { "size": "10GB", "type": "ssd" }
      }
    }
  },
  "id": 1
}
```

#### Response (Authorized)

```json
{
  "jsonrpc": "2.0",
  "result": {
    "authorized": true,
    "reasoning": "User alice owns graph-123, node structure is valid, no threats detected",
    "confidence": 0.95,
    "risk_level": "low",
    "checks_performed": [
      "user_authentication",
      "ownership_verification",
      "structure_validation",
      "threat_detection",
      "permission_check"
    ],
    "audit_id": "audit-789"
  },
  "id": 1
}
```

#### Response (Denied)

```json
{
  "jsonrpc": "2.0",
  "result": {
    "authorized": false,
    "reasoning": "Malicious pattern detected: Code injection attempt in node config",
    "confidence": 0.98,
    "risk_level": "high",
    "blocked_reason": "code_injection_detected",
    "threat_details": {
      "category": "code_injection",
      "location": "node-2.config.command",
      "pattern": "eval() call detected"
    },
    "recommendations": [
      "Remove executable code from node config",
      "Use parameterized configuration instead",
      "Review security best practices"
    ],
    "audit_id": "audit-790"
  },
  "id": 1
}
```

#### Error Codes

- `-32602`: Invalid params (missing required fields)
- `-32603`: Internal error (HSM failure, database error)
- `1001`: User authentication failed
- `1002`: Graph not found
- `1003`: Malformed graph structure

#### Performance Requirements

- **Latency**: < 10ms (p95), < 5ms (p50)
- **Throughput**: 10,000+ requests/sec
- **Concurrency**: 1,000 concurrent requests

---

### Method 2: `graph.validate_template`

**Purpose**: Validate a graph template for security issues before storage or deployment.

#### Request

```json
{
  "jsonrpc": "2.0",
  "method": "graph.validate_template",
  "params": {
    "template": {
      "id": "template-123",
      "name": "Web Server Deployment",
      "creator": "user-bob",
      "nodes": [
        {
          "id": "node-1",
          "type": "compute",
          "primal": "ToadStool",
          "config": { "cpu": 4, "memory": "8GB" }
        },
        {
          "id": "node-2",
          "type": "storage",
          "primal": "NestGate",
          "config": { "size": "50GB", "type": "ssd" }
        }
      ],
      "edges": [
        { "from": "node-1", "to": "node-2" }
      ],
      "signature": "base64_ed25519_signature_here",
      "metadata": {
        "version": "1.0.0",
        "created_at": "2026-01-11T12:00:00Z",
        "description": "Standard web server with storage"
      }
    }
  },
  "id": 2
}
```

#### Response (Valid)

```json
{
  "jsonrpc": "2.0",
  "result": {
    "valid": true,
    "risk_level": "low",
    "issues": [],
    "security_score": 0.92,
    "checks_performed": [
      "structure_validation",
      "signature_verification",
      "vulnerability_scan",
      "threat_detection",
      "dependency_analysis"
    ],
    "recommendations": [
      "Consider adding resource limits to node-1",
      "Template is production-ready"
    ],
    "validation_id": "val-456"
  },
  "id": 2
}
```

#### Response (Invalid)

```json
{
  "jsonrpc": "2.0",
  "result": {
    "valid": false,
    "risk_level": "high",
    "issues": [
      {
        "severity": "high",
        "category": "structure",
        "description": "Cyclic dependency detected",
        "location": "edge: node-1 → node-2 → node-1"
      },
      {
        "severity": "medium",
        "category": "signature",
        "description": "Template signature invalid or expired"
      },
      {
        "severity": "high",
        "category": "code_injection",
        "description": "Suspicious command found in node config",
        "location": "node-1.config.startup_script"
      }
    ],
    "security_score": 0.34,
    "recommendations": [
      "Remove cyclic dependency between nodes",
      "Re-sign template with valid Ed25519 signature",
      "Remove executable code from node configurations",
      "Use declarative configuration instead of scripts"
    ],
    "validation_id": "val-457"
  },
  "id": 2
}
```

#### Error Codes

- `-32602`: Invalid params (malformed template)
- `-32603`: Internal error
- `2001`: Template structure invalid
- `2002`: Signature verification failed

#### Performance Requirements

- **Latency**: < 50ms (p95), < 20ms (p50)
- **Throughput**: 5,000+ requests/sec
- **Concurrency**: 500 concurrent requests

---

### Method 3: `graph.audit_origin`

**Purpose**: Verify the origin and provenance of a graph template.

#### Request

```json
{
  "jsonrpc": "2.0",
  "method": "graph.audit_origin",
  "params": {
    "template_id": "template-123"
  },
  "id": 3
}
```

#### Response

```json
{
  "jsonrpc": "2.0",
  "result": {
    "template_id": "template-123",
    "creator": {
      "user_id": "user-bob",
      "identity_verified": true,
      "trust_score": 0.87,
      "reputation": "established_contributor",
      "member_since": "2025-06-15T10:00:00Z",
      "genetic_family": "nat0"
    },
    "lineage": [
      {
        "version": "1.0.0",
        "created_at": "2026-01-10T10:00:00Z",
        "created_by": "user-bob",
        "change_type": "initial_creation",
        "signature": "base64_signature_v1"
      },
      {
        "version": "1.1.0",
        "modified_at": "2026-01-11T12:00:00Z",
        "modified_by": "user-bob",
        "change_type": "optimization",
        "changes": ["reduced_node_count", "improved_efficiency"],
        "signature": "base64_signature_v1.1"
      }
    ],
    "chain_valid": true,
    "risk_level": "low",
    "trust_score": 0.87,
    "community_usage": {
      "deployments": 145,
      "success_rate": 0.94,
      "avg_rating": 4.6,
      "total_ratings": 23
    },
    "security_assessment": {
      "last_scan": "2026-01-11T18:00:00Z",
      "vulnerabilities_found": 0,
      "threat_level": "none"
    },
    "audit_id": "audit-999"
  },
  "id": 3
}
```

#### Response (Untrusted Origin)

```json
{
  "jsonrpc": "2.0",
  "result": {
    "template_id": "template-456",
    "creator": {
      "user_id": "user-unknown",
      "identity_verified": false,
      "trust_score": 0.12,
      "reputation": "new_user",
      "member_since": "2026-01-11T09:00:00Z"
    },
    "lineage": [
      {
        "version": "1.0.0",
        "created_at": "2026-01-11T09:30:00Z",
        "created_by": "user-unknown",
        "change_type": "initial_creation",
        "signature": null
      }
    ],
    "chain_valid": false,
    "risk_level": "high",
    "trust_score": 0.12,
    "community_usage": {
      "deployments": 0,
      "success_rate": null,
      "avg_rating": null,
      "total_ratings": 0
    },
    "security_assessment": {
      "last_scan": "2026-01-11T18:00:00Z",
      "vulnerabilities_found": 3,
      "threat_level": "medium"
    },
    "warnings": [
      "Creator identity not verified",
      "No community usage history",
      "Template not signed",
      "Multiple vulnerabilities detected"
    ],
    "recommendations": [
      "Do not use this template in production",
      "Review template source code carefully",
      "Consider using established community templates instead"
    ],
    "audit_id": "audit-1000"
  },
  "id": 3
}
```

#### Error Codes

- `-32602`: Invalid params (missing template_id)
- `-32603`: Internal error
- `3001`: Template not found
- `3002`: Template metadata unavailable

#### Performance Requirements

- **Latency**: < 100ms (p95), < 50ms (p50)
- **Throughput**: 2,000+ requests/sec
- **Concurrency**: 200 concurrent requests

---

## Security Model

### Threat Model

#### Threats Protected

1. **Unauthorized Modifications**
   - **Threat**: User modifies graphs they don't own
   - **Protection**: Ownership verification + RBAC
   - **Detection**: Layer 2 (Authorization)

2. **Malicious Graph Injection**
   - **Threat**: Attacker injects harmful nodes into graphs
   - **Protection**: Structure validation + pattern matching
   - **Detection**: Layer 3 (Validation) + Layer 4 (Threats)

3. **Template Poisoning**
   - **Threat**: Tampered templates in community library
   - **Protection**: Signature verification + integrity checks
   - **Detection**: Layer 3 (Validation) + Layer 5 (Audit)

4. **Privilege Escalation**
   - **Threat**: User gains unauthorized permissions
   - **Protection**: RBAC + action-specific authorization
   - **Detection**: Layer 2 (Authorization)

5. **Code Injection**
   - **Threat**: Malicious code in node configurations
   - **Protection**: Configuration validation + sandbox checks
   - **Detection**: Layer 4 (Threats)

6. **Supply Chain Attacks**
   - **Threat**: Compromised template origins
   - **Protection**: Provenance tracking + trust scoring
   - **Detection**: Layer 5 (Audit)

### Permission Model

#### Roles

```
Owner (Full Control):
  ✅ Read graph
  ✅ Modify graph (all operations)
  ✅ Delete graph
  ✅ Transfer ownership
  ✅ Manage collaborators

Collaborator (Limited Modification):
  ✅ Read graph
  ✅ Suggest modifications (requires approval)
  ✅ View modification history
  ❌ Delete graph
  ❌ Transfer ownership

Viewer (Read-Only):
  ✅ Read graph
  ✅ View modification history
  ❌ Modify graph
  ❌ Delete graph

Public Template (Community):
  ✅ Anyone can view
  ✅ Anyone can deploy
  ✅ Requires signature from verified creator
  ✅ Subject to community rating
```

#### Permission Matrix

| Action | Owner | Collaborator | Viewer | Public |
|--------|-------|--------------|--------|--------|
| Read | ✅ | ✅ | ✅ | ✅ |
| Add Node | ✅ | 🔶* | ❌ | ❌ |
| Remove Node | ✅ | 🔶* | ❌ | ❌ |
| Modify Node | ✅ | 🔶* | ❌ | ❌ |
| Add Edge | ✅ | 🔶* | ❌ | ❌ |
| Remove Edge | ✅ | 🔶* | ❌ | ❌ |
| Delete Graph | ✅ | ❌ | ❌ | ❌ |
| Transfer Ownership | ✅ | ❌ | ❌ | ❌ |
| Deploy Template | ✅ | ✅ | ✅ | ✅ |

*🔶 = Requires approval from owner

### Signature Verification

**Algorithm**: Ed25519  
**Hash**: BLAKE3  
**Key Management**: HSM-backed storage

**Signature Flow**:
1. Creator generates template
2. Template serialized to canonical JSON
3. BLAKE3 hash computed
4. Hash signed with creator's Ed25519 private key
5. Signature attached to template metadata
6. BearDog verifies signature on validation/audit

**Signature Format**:
```json
{
  "signature": "base64_encoded_ed25519_signature",
  "algorithm": "Ed25519",
  "hash": "BLAKE3",
  "signed_at": "2026-01-11T12:00:00Z",
  "signer_id": "user-bob",
  "signer_public_key": "base64_encoded_public_key"
}
```

---

## Implementation Details

### Code Organization

```
crates/beardog-tunnel/src/
  ├─ graph_security/
  │   ├─ mod.rs                    # Module exports
  │   ├─ authorize.rs              # Method 1 implementation
  │   ├─ validate.rs               # Method 2 implementation
  │   ├─ audit.rs                  # Method 3 implementation
  │   ├─ types.rs                  # Common types
  │   ├─ threats.rs                # Threat detection
  │   ├─ permissions.rs            # RBAC logic
  │   └─ tests/
  │       ├─ authorize_tests.rs    # 25 unit tests
  │       ├─ validate_tests.rs     # 20 unit tests
  │       └─ audit_tests.rs        # 15 unit tests
  │
  └─ unix_socket_ipc.rs            # Add graph.* method handlers

tests/
  └─ graph_security_integration_tests.rs  # 12 integration tests
```

### Existing Capabilities Reused

1. **Authentication** (`crates/beardog-core/src/`)
   - HSM-backed user authentication
   - Genetic lineage verification
   - Trust score calculation

2. **Signature Verification** (`crates/beardog-security/src/`)
   - Ed25519 signature verification
   - BLAKE3 hashing
   - Public key management

3. **Threat Detection** (`crates/beardog-threat/src/`)
   - Pattern matching engine
   - Anomaly detection algorithms
   - Risk scoring

4. **RBAC** (`crates/beardog-auth/src/`)
   - Role-based access control
   - Permission management
   - Action authorization

5. **IPC** (`crates/beardog-tunnel/src/unix_socket_ipc.rs`)
   - Unix socket JSON-RPC server
   - Request/response handling
   - Error handling

6. **Audit** (`crates/beardog-monitoring/src/`)
   - Audit trail generation
   - Provenance tracking
   - Chain of custody

### New Capabilities Required

1. **Graph Structure Validation**
   - Node type validation
   - Edge validation (no cycles, valid connections)
   - Configuration schema validation
   - Dependency analysis

2. **Modification Safety Checks**
   - Safe node addition/removal
   - Safe edge addition/removal
   - Configuration change validation
   - Impact analysis

3. **Template-Specific Threats**
   - Template vulnerability scanner
   - Malicious graph pattern database
   - Community template reputation
   - Supply chain verification

4. **Creator Trust Scoring**
   - Historical success rate
   - Community reputation
   - Verification level
   - Family trust propagation

---

## Testing Strategy

### Unit Tests (60 total)

#### Method 1: `authorize_modification` (25 tests)

**Authentication Tests** (5):
- Valid user, valid graph
- Invalid user, valid graph
- Valid user, non-existent graph
- User authentication timeout
- HSM failure during authentication

**Authorization Tests** (8):
- Owner modifying own graph
- Collaborator requesting modification
- Viewer attempting modification
- Public user attempting modification
- Action-specific permission checks
- Role inheritance
- Permission escalation attempt
- Cross-graph permission isolation

**Validation Tests** (7):
- Valid node addition
- Invalid node structure
- Cyclic dependency detection
- Missing required fields
- Type mismatch in config
- Edge validation
- Graph size limits

**Threat Detection Tests** (5):
- Code injection in config
- Malicious pattern detection
- Privilege escalation attempt
- Resource exhaustion attack
- Suspicious modification patterns

#### Method 2: `validate_template` (20 tests)

**Structure Validation** (7):
- Well-formed template
- Missing required fields
- Invalid node types
- Invalid edge references
- Cyclic dependencies
- Disconnected subgraphs
- Empty template

**Signature Verification** (5):
- Valid signature
- Invalid signature
- Expired signature
- Missing signature
- Wrong signer

**Vulnerability Scanning** (5):
- Code injection detection
- Command injection detection
- Path traversal detection
- Resource abuse detection
- Clean template (no vulnerabilities)

**Threat Detection** (3):
- Known malicious patterns
- Anomalous structures
- Supply chain indicators

#### Method 3: `audit_origin` (15 tests)

**Creator Verification** (5):
- Verified creator
- Unverified creator
- Unknown creator
- Creator trust score calculation
- Family trust propagation

**Lineage Tracking** (5):
- Single version template
- Multi-version template
- Modification chain validation
- Broken chain detection
- Forked template lineage

**Community Metrics** (3):
- High usage, high rating
- Low usage, unknown rating
- New template, no usage

**Security Assessment** (2):
- Clean template
- Template with vulnerabilities

### Integration Tests (12 scenarios)

#### petalTongue Integration (3 tests)
1. User opens graph editor → Modifies node → Real-time authorization → UI updates
2. User adds malicious node → Authorization denies → UI shows error + recommendations
3. User rapid-fire modifications → All authorizations complete < 10ms

#### NestGate Integration (3 tests)
1. User saves template → Validation checks → NestGate stores → Audit trail created
2. User saves malicious template → Validation fails → Storage prevented
3. User retrieves template → Origin audit → Trust score displayed

#### Squirrel Integration (3 tests)
1. AI suggests modification → BearDog validates → Safe suggestion applied
2. AI suggests unsafe modification → BearDog blocks → AI learns
3. AI learns from authorization patterns → Improves future suggestions

#### biomeOS End-to-End (3 tests)
1. Complete workflow: Create → Modify → Validate → Save → Deploy
2. Security checkpoint at each stage → All checks pass
3. Malicious actor blocked at first checkpoint → Audit trail complete

### Performance Tests

#### Throughput Testing
- **Target**: 10,000 req/sec for `authorize_modification`
- **Target**: 5,000 req/sec for `validate_template`
- **Target**: 2,000 req/sec for `audit_origin`
- **Duration**: 1 hour sustained load
- **Measurement**: Requests completed, errors, latency distribution

#### Latency Testing
- **Metric**: p50, p95, p99, max
- **Target (authorize)**: p95 < 10ms, p50 < 5ms
- **Target (validate)**: p95 < 50ms, p50 < 20ms
- **Target (audit)**: p95 < 100ms, p50 < 50ms

#### Concurrency Testing
- **authorize_modification**: 1,000 concurrent requests
- **validate_template**: 500 concurrent requests
- **audit_origin**: 200 concurrent requests
- **Measurement**: Success rate, deadlocks, race conditions

#### Load Testing
- Gradual ramp-up from 0 to 10,000 req/sec
- Sustained load at 5,000 req/sec for 1 hour
- Spike testing (sudden 10x increase)
- Stress testing (2x target load)

### Failure Testing

#### Network Failures
- Unix socket disconnection during request
- Partial message delivery
- Message corruption
- Timeout handling

#### HSM Failures
- HSM unavailable
- HSM timeout
- Key not found
- Signature generation failure

#### Database Failures
- Template not found
- Audit trail storage failure
- Transaction rollback
- Connection pool exhaustion

---

## Integration Points

### With petalTongue (UI)

**Use Case**: Real-time graph editing

**Flow**:
1. User opens graph in editor
2. User modifies node in UI
3. petalTongue calls `graph.authorize_modification`
4. BearDog responds < 10ms
5. petalTongue updates UI based on response
6. If denied, show error + recommendations

**Requirements**:
- < 10ms latency for smooth UX
- Clear error messages for UI display
- Recommendations for fixing denied modifications

### With NestGate (Storage)

**Use Case**: Template storage and retrieval

**Flow**:
1. User saves template in petalTongue
2. NestGate calls `graph.validate_template`
3. BearDog validates and returns result
4. If valid, NestGate stores template
5. On retrieval, NestGate calls `graph.audit_origin`
6. BearDog returns trust score
7. NestGate includes trust score in metadata

**Requirements**:
- Validation before storage (block malicious templates)
- Origin audit on retrieval (inform users of trust level)
- Template signature verification

### With Squirrel (AI)

**Use Case**: AI suggestion validation

**Flow**:
1. Squirrel generates graph improvement suggestion
2. Squirrel calls `graph.authorize_modification` (with AI user context)
3. BearDog validates suggestion safety
4. If authorized, Squirrel provides to user
5. If denied, Squirrel learns from failure
6. Squirrel improves future suggestions

**Requirements**:
- Validate AI suggestions before presenting to users
- Provide learning feedback to Squirrel
- Track AI suggestion success rate

### With Songbird (Discovery)

**Use Case**: Distributed graph validation

**Flow**:
1. Graph spans multiple nodes
2. Songbird coordinates validation
3. BearDog validates on each node
4. Songbird aggregates results
5. Graph deployed if all nodes authorize

**Requirements**:
- Consistent validation across nodes
- Shared threat intelligence
- Coordinated audit trails

### With ToadStool (Compute)

**Use Case**: Resource validation

**Flow**:
1. User creates compute-heavy graph
2. ToadStool estimates resources
3. BearDog validates resource requests are reasonable
4. If suspicious (DoS attempt), BearDog blocks
5. If valid, deployment proceeds

**Requirements**:
- Detect resource abuse attempts
- Validate compute requests
- Coordinate with ToadStool resource planner

### With biomeOS (Orchestration)

**Use Case**: End-to-end security

**Flow**:
1. biomeOS orchestrates deployment
2. Security checkpoints at each stage
3. BearDog validates at each checkpoint
4. Audit trail tracks entire workflow
5. Any failure stops deployment

**Requirements**:
- Integration at all deployment stages
- Complete audit trail
- Failure handling and rollback

---

## Performance Requirements

### Latency

| Method | p50 | p95 | p99 | Max |
|--------|-----|-----|-----|-----|
| `authorize_modification` | < 5ms | < 10ms | < 20ms | < 50ms |
| `validate_template` | < 20ms | < 50ms | < 100ms | < 200ms |
| `audit_origin` | < 50ms | < 100ms | < 200ms | < 500ms |

### Throughput

| Method | Target | Sustained |
|--------|--------|-----------|
| `authorize_modification` | 10,000+ req/sec | 5,000 req/sec (1 hour) |
| `validate_template` | 5,000+ req/sec | 2,500 req/sec (1 hour) |
| `audit_origin` | 2,000+ req/sec | 1,000 req/sec (1 hour) |

### Concurrency

| Method | Concurrent Requests |
|--------|---------------------|
| `authorize_modification` | 1,000 |
| `validate_template` | 500 |
| `audit_origin` | 200 |

### Resource Usage

- **Memory**: < 500MB for graph security module
- **CPU**: < 20% at sustained load
- **Disk I/O**: < 10MB/sec for audit trails
- **Network**: Unix sockets (negligible overhead)

### Availability

- **Uptime**: 99.9% (8.76 hours downtime/year)
- **MTTR**: < 5 minutes
- **Graceful degradation**: Continue with cached trust scores if NestGate unavailable

---

## Appendix

### Environment Variables

```bash
# Graph security configuration
BEARDOG_GRAPH_SECURITY_ENABLED=true

# Threat detection
BEARDOG_GRAPH_THREAT_DB_PATH=/var/lib/beardog/threat_patterns.db
BEARDOG_GRAPH_ANOMALY_THRESHOLD=0.8

# Performance tuning
BEARDOG_GRAPH_MAX_NODES=1000
BEARDOG_GRAPH_MAX_EDGES=5000
BEARDOG_GRAPH_VALIDATION_TIMEOUT_MS=5000

# Audit trail
BEARDOG_GRAPH_AUDIT_ENABLED=true
BEARDOG_GRAPH_AUDIT_PATH=/var/lib/beardog/graph_audit.db

# Signature verification
BEARDOG_GRAPH_REQUIRE_SIGNATURES=true  # For community templates
BEARDOG_GRAPH_SIGNATURE_CACHE_SIZE=1000
```

### Metrics

```rust
// Prometheus metrics exposed
beardog_graph_authorization_total{result="authorized|denied"}
beardog_graph_authorization_duration_seconds{quantile="0.5|0.95|0.99"}
beardog_graph_validation_total{result="valid|invalid"}
beardog_graph_validation_duration_seconds{quantile="0.5|0.95|0.99"}
beardog_graph_audit_total
beardog_graph_audit_duration_seconds{quantile="0.5|0.95|0.99"}
beardog_graph_threats_detected_total{category="code_injection|privilege|..."}
beardog_graph_trust_scores{quantile="0.5|0.95|0.99"}
```

### Error Handling

All errors follow JSON-RPC 2.0 standard with additional BearDog context:

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32603,
    "message": "Internal error",
    "data": {
      "category": "hsm_failure",
      "details": "HSM connection timeout",
      "retry_after": 1000,
      "recommendation": "Retry request after 1 second"
    }
  },
  "id": 1
}
```

### Security Best Practices

1. **Always validate templates before deployment**
2. **Require signatures for community templates**
3. **Check audit results for template trust scores**
4. **Monitor authorization denial rates** (spike = potential attack)
5. **Review audit trails regularly**
6. **Update threat pattern database weekly**
7. **Implement rate limiting on client side**
8. **Use secure Unix socket permissions** (0600)

---

**Document Version**: 1.0.0  
**Last Updated**: January 11, 2026  
**Status**: Committed - Implementation Starting Jan 13, 2026  
**Timeline**: 2 weeks  
**Contact**: BearDog Team (#collaborative-intelligence)

