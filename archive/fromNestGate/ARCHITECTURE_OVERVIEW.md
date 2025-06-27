---
title: NestGate v2 Architecture Overview - Orchestrator-Centric Sovereign Design
description: System architecture documentation with v2 orchestrator-centric patterns
version: 2.0.0
date: 2025-01-26
---

# NestGate v2 Architecture Overview - Orchestrator-Centric Sovereign Design

This document provides an overview of the NestGate v2 orchestrator-centric sovereign NAS management system architecture.

## **v2 Architectural Philosophy**

NestGate v2 is built on **orchestrator-centric connectivity** with **sovereign operation** as the primary design principle:

1. **Orchestrator-First Design**: ALL connectivity flows through nestgate-orchestrator
2. **Sovereign Operation**: Fully autonomous capability with no external dependencies
3. **Optional Federation**: MCP integration when available, graceful degradation when not
4. **Centralized Connectivity Hub**: Single point of control for all service communication
5. **Service Registry Management**: Orchestrator manages all service discovery and registration

## System Components

The NestGate v2 system consists of these key components:

1. **NestGate Orchestrator** - Central connectivity hub and service coordinator
2. **Service Registry** - Orchestrator-managed service discovery and registration  
3. **Connection Proxy** - Routes all external connections through orchestrator
4. **Health Monitor** - Orchestrator-based service health monitoring
5. **MCP Federation** - Optional connectivity to MCP clusters
6. **Core Services** - Storage, network, and management services
7. **Storage Layer** - ZFS-based tiered storage management

## v2 Architecture Diagram

```mermaid
---
title: NestGate v2 Orchestrator-Centric Architecture
---
flowchart TB
    subgraph "External Connectivity"
        EXT[External Clients]
        MCP[MCP Federation<br/>*Optional*]
        UI[Management UI]
        API[External APIs]
    end

    subgraph "NestGate Orchestrator Hub"
        ORCH[nestgate-orchestrator<br/>:8080]
        REG[Service Registry]
        PROXY[Connection Proxy]
        HEALTH[Health Monitor]
        FED[MCP Federation Handler]
    end

    subgraph "Core Services"
        CORE[nestgate-core<br/>Storage Management]
        NET[nestgate-network<br/>Protocol Services]
        ZFS[nestgate-zfs<br/>ZFS Integration]
        META[nestgate-meta<br/>Metadata Store]
    end

    subgraph "Storage Layer"
        HOT[Hot Tier<br/>NVMe Cache]
        WARM[Warm Tier<br/>Primary ZFS]
        COLD[Cold Tier<br/>Archive Storage]
    end

    %% External to Orchestrator
    EXT --> ORCH
    UI --> ORCH
    API --> ORCH
    MCP -.->|Optional| ORCH

    %% Orchestrator Internal
    ORCH --> REG
    ORCH --> PROXY
    ORCH --> HEALTH
    ORCH --> FED

    %% Orchestrator to Services
    PROXY --> CORE
    PROXY --> NET
    PROXY --> ZFS
    PROXY --> META

    %% Service to Storage
    CORE --> HOT
    CORE --> WARM
    CORE --> COLD
    ZFS --> WARM
    ZFS --> COLD

    %% Health Monitoring
    HEALTH -.-> CORE
    HEALTH -.-> NET
    HEALTH -.-> ZFS
    HEALTH -.-> META

    %% Service Registry
    REG -.-> CORE
    REG -.-> NET
    REG -.-> ZFS
    REG -.-> META

    %% Styling
    classDef orchestrator fill:#e1f5fe,stroke:#01579b,stroke-width:3px
    classDef service fill:#f3e5f5,stroke:#4a148c,stroke-width:2px
    classDef storage fill:#e8f5e8,stroke:#1b5e20,stroke-width:2px
    classDef external fill:#fff3e0,stroke:#e65100,stroke-width:2px
    classDef optional fill:#f0f0f0,stroke:#757575,stroke-width:1px,stroke-dasharray: 5 5

    class ORCH,REG,PROXY,HEALTH,FED orchestrator
    class CORE,NET,ZFS,META service
    class HOT,WARM,COLD storage
    class EXT,UI,API external
    class MCP optional
```

## Sovereign Operation Modes

### 1. Standalone Mode (Default)
```yaml
mode: standalone
dependencies: NONE
connectivity: Internal orchestrator only
description: Fully autonomous operation
```

```mermaid
---
title: Standalone Mode - Fully Sovereign
---
flowchart LR
    UI[Management UI] --> ORCH[NestGate Orchestrator]
    API[Local APIs] --> ORCH
    ORCH --> SERVICES[Core Services]
    SERVICES --> STORAGE[ZFS Storage]
    
    classDef standalone fill:#e8f5e8,stroke:#1b5e20,stroke-width:3px
    class UI,API,ORCH,SERVICES,STORAGE standalone
```

### 2. Federated Mode (Optional)
```yaml
mode: federated
dependencies: MCP cluster available
connectivity: Orchestrator + MCP federation
description: Connected to MCP cluster with fallback to standalone
```

```mermaid
---
title: Federated Mode - Optional MCP Integration
---
flowchart LR
    MCP[MCP Cluster] -.->|Optional| ORCH[NestGate Orchestrator]
    UI[Management UI] --> ORCH
    ORCH --> SERVICES[Core Services]
    SERVICES --> STORAGE[ZFS Storage]
    
    classDef federated fill:#e1f5fe,stroke:#01579b,stroke-width:3px
    classDef optional fill:#f0f0f0,stroke:#757575,stroke-width:1px,stroke-dasharray: 5 5
    class UI,ORCH,SERVICES,STORAGE federated
    class MCP optional
```

## Component Interactions

### Orchestrator-Centric Connectivity Flow

```mermaid
---
title: v2 Orchestrator Connectivity Flow
---
sequenceDiagram
    participant C as Client
    participant O as NestGate Orchestrator
    participant R as Service Registry
    participant P as Connection Proxy
    participant S as Core Service

    C->>O: Request (e.g., GET /api/health)
    O->>R: Lookup service endpoint
    R->>O: Return service details
    O->>P: Route request to service
    P->>S: Forward request
    S->>P: Generate response
    P->>O: Return response
    O->>C: Send response to client

    Note over O: All connectivity flows<br/>through orchestrator
```

### Service Registration Flow

```mermaid
---
title: v2 Service Registration Flow
---
sequenceDiagram
    participant S as Service
    participant O as Orchestrator
    participant R as Service Registry
    participant H as Health Monitor

    S->>O: Register service
    O->>R: Store service info
    R->>O: Confirm registration
    O->>H: Start health monitoring
    H->>S: Initial health check
    S->>H: Health status
    H->>O: Report service healthy
    O->>S: Registration complete

    Note over O: Orchestrator manages<br/>all service lifecycle
```

### MCP Federation Flow (Optional)

```mermaid
---
title: v2 MCP Federation Flow (Optional)
---
sequenceDiagram
    participant M as MCP Cluster
    participant F as Federation Handler
    participant O as Orchestrator
    participant S as Services

    Note over F: Auto-detect MCP availability
    F->>M: Attempt connection
    alt MCP Available
        M->>F: Connection established
        F->>O: Enable federation mode
        O->>S: Update service configuration
        loop Periodic Heartbeat
            F->>M: Send heartbeat
            M->>F: Cluster updates
        end
    else MCP Unavailable
        F->>O: Fallback to standalone mode
        O->>S: Operate independently
        Note over O,S: Fully sovereign operation
    end
```

## Data Flow Architecture

```mermaid
---
title: v2 Data Flow - Orchestrator Hub
---
flowchart TD
    subgraph "External Sources"
        EXT[External Requests]
        UI[Management UI]
        MCP[MCP Federation]
    end

    subgraph "Orchestrator Hub"
        ORCH[Central Orchestrator]
        REG[Service Registry]
        PROXY[Connection Proxy]
        HEALTH[Health Monitor]
    end

    subgraph "Service Layer"
        API[REST APIs]
        WS[WebSocket]
        ZFS[ZFS Management]
        NET[Network Protocols]
    end

    subgraph "Storage Layer"
        POOL[ZFS Pools]
        DATA[Datasets]
        SNAP[Snapshots]
    end

    %% External to Orchestrator
    EXT --> ORCH
    UI --> ORCH
    MCP -.->|Optional| ORCH

    %% Orchestrator Internal Flow
    ORCH --> REG
    ORCH --> PROXY
    ORCH --> HEALTH

    %% Orchestrator to Services
    PROXY --> API
    PROXY --> WS
    PROXY --> ZFS
    PROXY --> NET

    %% Services to Storage
    API --> POOL
    ZFS --> POOL
    ZFS --> DATA
    ZFS --> SNAP

    %% Health Monitoring
    HEALTH -.-> API
    HEALTH -.-> WS
    HEALTH -.-> ZFS
    HEALTH -.-> NET

    classDef orchestrator fill:#e1f5fe,stroke:#01579b,stroke-width:3px
    classDef service fill:#f3e5f5,stroke:#4a148c,stroke-width:2px
    classDef storage fill:#e8f5e8,stroke:#1b5e20,stroke-width:2px
    classDef external fill:#fff3e0,stroke:#e65100,stroke-width:2px

    class ORCH,REG,PROXY,HEALTH orchestrator
    class API,WS,ZFS,NET service
    class POOL,DATA,SNAP storage
    class EXT,UI,MCP external
```

## Component Details

### NestGate Orchestrator (Core Hub)
```yaml
crate: nestgate-orchestrator
port: 8080 (fixed)
responsibility: Central connectivity hub
features:
  - Service registry management
  - Connection proxy and routing
  - Health monitoring coordination
  - MCP federation handling
  - Graceful degradation support
```

**Key Capabilities:**
- **Service Discovery**: Central registry for all system services
- **Connection Routing**: Proxy all external connections to appropriate services
- **Health Monitoring**: Continuous health checks with automatic recovery
- **Federation Support**: Optional MCP integration with standalone fallback

### Service Registry
```yaml
component: ServiceRegistry
managed_by: nestgate-orchestrator
responsibility: Service discovery and registration
```

**Service Types Managed:**
- **nestgate-core**: Storage management and tier coordination
- **nestgate-network**: Protocol services (NFS, SMB, HTTP)
- **nestgate-zfs**: ZFS integration and management
- **nestgate-meta**: Metadata and configuration storage

### Connection Proxy
```yaml
component: ConnectionProxy
managed_by: nestgate-orchestrator
responsibility: Route all external connectivity
```

**Routing Patterns:**
- External API calls → Appropriate service endpoints
- Management UI requests → Core services
- Protocol requests → Network services
- Health checks → Service health endpoints

### Health Monitor
```yaml
component: HealthMonitor
managed_by: nestgate-orchestrator
interval: 30 seconds (configurable)
responsibility: Service health coordination
```

**Health Check Types:**
- **HTTP Health Endpoints**: Service-specific health APIs
- **Process Monitoring**: Service process status
- **Resource Monitoring**: CPU, memory, disk usage
- **Custom Checks**: Service-specific health validation

### MCP Federation (Optional)
```yaml
component: McpFederation
managed_by: nestgate-orchestrator
mode: auto-detect (configurable)
responsibility: Optional cluster connectivity
```

**Federation Modes:**
- **Standalone**: No MCP dependency (default)
- **Auto-Detect**: Attempt MCP connection, fallback to standalone
- **Federated**: Active MCP cluster participation

## Storage Integration

### ZFS Management via Orchestrator
```yaml
storage_architecture: orchestrator_managed
tiers:
  hot: NVMe cache (future)
  warm: Primary ZFS pools
  cold: Archive storage
integration: All ZFS operations via orchestrator
```

**Storage Operations Flow:**
1. Client requests storage operation
2. Orchestrator routes to nestgate-core
3. Core service coordinates with nestgate-zfs
4. ZFS operations executed on storage layer
5. Results routed back through orchestrator

### Tiered Storage Management
```mermaid
---
title: Storage Tier Management via Orchestrator
---
flowchart LR
    ORCH[Orchestrator] --> CORE[nestgate-core]
    CORE --> ZFS[nestgate-zfs]
    ZFS --> HOT[Hot Tier<br/>*Future*]
    ZFS --> WARM[Warm Tier<br/>Active]
    ZFS --> COLD[Cold Tier<br/>Archive]
    
    classDef current fill:#e8f5e8,stroke:#1b5e20,stroke-width:3px
    classDef future fill:#f0f0f0,stroke:#757575,stroke-width:1px,stroke-dasharray: 5 5
    
    class ORCH,CORE,ZFS,WARM,COLD current
    class HOT future
```

## Security and Access Control

### Orchestrator-Managed Security
```yaml
security_model: orchestrator_centric
authentication: Centralized through orchestrator
authorization: Role-based access control (RBAC)
encryption: TLS for all external connections
```

**Security Flow:**
1. All external connections terminate at orchestrator
2. Authentication and authorization at orchestrator level
3. Authenticated requests routed to appropriate services
4. Services trust orchestrator authentication decisions

## Deployment Architecture

### Minimal Sovereign Deployment
```yaml
requirements:
  cpu: 4 cores minimum
  ram: 32GB ECC recommended
  storage: Single ZFS pool supported
  network: 1G minimum, 10G preferred
  dependencies: NONE (fully autonomous)
```

### Deployment Flow
```mermaid
---
title: v2 Deployment Flow
---
flowchart TD
    START[Start Deployment] --> ORCH[Start Orchestrator]
    ORCH --> REG[Initialize Service Registry]
    REG --> SERVICES[Register Core Services]
    SERVICES --> HEALTH[Start Health Monitoring]
    HEALTH --> CHECK[Health Check All Services]
    CHECK --> READY{All Services Healthy?}
    READY -->|Yes| COMPLETE[Deployment Complete]
    READY -->|No| RETRY[Retry Service Startup]
    RETRY --> CHECK
    
    classDef success fill:#e8f5e8,stroke:#1b5e20,stroke-width:3px
    classDef process fill:#e1f5fe,stroke:#01579b,stroke-width:2px
    
    class COMPLETE success
    class ORCH,REG,SERVICES,HEALTH,CHECK process
```

## Performance Characteristics

### Orchestrator Overhead
```yaml
connection_latency: <5ms additional overhead
throughput_impact: <2% for data operations
cpu_overhead: <5% under normal load
memory_overhead: ~50MB for orchestrator
```

### Scaling Characteristics
- **Single Node**: Fully supported (current implementation)
- **Multi-Node**: Planned for future orchestrator coordination
- **Federation**: Optional MCP cluster participation
- **Storage**: Horizontal scaling via ZFS pool expansion

## Future Enhancements

### Phase 2: Enhanced Storage Tiers (2025 Q2)
- Hot tier (NVMe) integration via orchestrator
- Automated tier migration policies
- AI workload optimization

### Phase 3: Advanced Federation (2025 Q2-Q3)
- Multi-node orchestrator coordination
- Distributed storage management
- Cross-cluster replication

### Phase 4: AI Integration (2025 Q3-Q4)
- Model hosting infrastructure
- GPU integration via orchestrator
- AI-specific storage patterns

## Summary

NestGate v2 represents a **successful architectural evolution** from complex port management to **orchestrator-centric sovereign design**:

### Key Architectural Principles
1. **Orchestrator-Centric**: All connectivity flows through single hub
2. **Sovereign Operation**: Fully autonomous with no external dependencies
3. **Optional Federation**: MCP integration when available, standalone when not
4. **Simplified Design**: Single orchestrator vs complex port manager
5. **Production Ready**: Robust error handling and graceful degradation

### Implementation Success
- ✅ **Zero compilation errors** across all crates
- ✅ **Successful deployment** with orchestrator coordination
- ✅ **Standalone operation** verified and functional
- ✅ **Optional federation** ready for MCP integration
- ✅ **Simplified management** via central orchestrator

The v2 architecture successfully delivers on the **sovereign NAS vision** while maintaining flexibility for future enhancements and optional cluster participation. 