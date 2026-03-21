# BearDog Comprehensive API Documentation

## Overview

BearDog provides a comprehensive REST API for security operations, genetic spawning, workflow management, and system orchestration. This documentation covers all available endpoints with examples, schemas, and best practices.

**Base URL**: `https://api.beardog.local`  
**API Version**: `v1`  
**Authentication**: Bearer tokens, mTLS, or Ed25519 signatures

---

## Table of Contents

1. [Authentication & Authorization](#authentication--authorization)
2. [Security Operations API](#security-operations-api)
3. [Genetic Operations API](#genetic-operations-api)
4. [Workflow Management API](#workflow-management-api)
5. [System Management API](#system-management-api)
6. [Compliance & Audit API](#compliance--audit-api)
7. [Node Registry API](#node-registry-api)
8. [Monitoring & Health API](#monitoring--health-api)
9. [Error Handling](#error-handling)
10. [Rate Limiting](#rate-limiting)
11. [SDK Examples](#sdk-examples)

---

## Authentication & Authorization

### Overview

BearDog supports multiple authentication methods for different security requirements:

- **Bearer Tokens**: Standard JWT tokens for web applications
- **mTLS**: Mutual TLS authentication for service-to-service communication
- **Ed25519 Signatures**: High-performance digital signatures for low-latency operations
- **Multi-factor**: Combined authentication methods for high-security operations

### Authentication Endpoints

#### POST `/api/v1/auth/authenticate`

Authenticate user credentials and obtain access tokens.

**Request:**
```json
{
  "username": "user@example.com",
  "password": "secure_password",
  "mfa_token": "123456",
  "authentication_method": "password_mfa"
}
```

**Response:**
```json
{
  "success": true,
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "expires_in": 3600,
  "token_type": "Bearer",
  "user_info": {
    "id": "user_123",
    "username": "user@example.com",
    "roles": ["user", "analyst"],
    "permissions": ["read:security", "write:workflows"]
  }
}
```

#### POST `/api/v1/auth/refresh`

Refresh an expired access token using a valid refresh token.

**Request:**
```json
{
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

**Response:**
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "expires_in": 3600,
  "token_type": "Bearer"
}
```

#### POST `/api/v1/auth/revoke`

Revoke access tokens and invalidate sessions.

**Request:**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "revoke_all_sessions": false
}
```

**Response:**
```json
{
  "success": true,
  "message": "Token revoked successfully"
}
```

### Authorization Headers

Include authentication in all API requests:

```http
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
X-BearDog-Signature: signature_here
X-BearDog-Timestamp: 1642123456
```

---

## Security Operations API

### Overview

The Security API provides cryptographic operations, key management, and security policy enforcement.

### Encryption Operations

#### POST `/api/v1/security/encrypt`

Encrypt data using specified encryption algorithms and keys.

**Request:**
```json
{
  "data": "base64_encoded_plaintext_data",
  "encryption_method": "AES-256-GCM",
  "key_id": "key_12345",
  "additional_data": "context_information",
  "encoding": "base64"
}
```

**Response:**
```json
{
  "success": true,
  "encrypted_data": "base64_encoded_ciphertext",
  "encryption_metadata": {
    "algorithm": "AES-256-GCM",
    "key_id": "key_12345",
    "iv": "base64_encoded_iv",
    "auth_tag": "base64_encoded_tag",
    "encrypted_at": "2024-01-19T12:00:00Z"
  }
}
```

#### POST `/api/v1/security/decrypt`

Decrypt previously encrypted data.

**Request:**
```json
{
  "encrypted_data": "base64_encoded_ciphertext",
  "encryption_metadata": {
    "algorithm": "AES-256-GCM",
    "key_id": "key_12345",
    "iv": "base64_encoded_iv",
    "auth_tag": "base64_encoded_tag"
  }
}
```

**Response:**
```json
{
  "success": true,
  "decrypted_data": "base64_encoded_plaintext",
  "verification_status": "authenticated"
}
```

### Digital Signatures

#### POST `/api/v1/security/sign`

Create digital signatures for data integrity and authentication.

**Request:**
```json
{
  "data": "base64_encoded_data_to_sign",
  "signing_key_id": "signing_key_789",
  "signature_algorithm": "Ed25519",
  "hash_algorithm": "SHA256"
}
```

**Response:**
```json
{
  "success": true,
  "signature": "base64_encoded_signature",
  "signature_metadata": {
    "algorithm": "Ed25519",
    "key_id": "signing_key_789",
    "signed_at": "2024-01-19T12:00:00Z",
    "hash_algorithm": "SHA256"
  }
}
```

#### POST `/api/v1/security/verify`

Verify digital signatures.

**Request:**
```json
{
  "data": "base64_encoded_original_data",
  "signature": "base64_encoded_signature",
  "verification_key_id": "verification_key_789",
  "signature_metadata": {
    "algorithm": "Ed25519",
    "hash_algorithm": "SHA256"
  }
}
```

**Response:**
```json
{
  "success": true,
  "signature_valid": true,
  "verification_details": {
    "verified_at": "2024-01-19T12:01:00Z",
    "key_status": "active",
    "trust_level": "high"
  }
}
```

### Key Management

#### GET `/api/v1/security/keys`

List available cryptographic keys.

**Query Parameters:**
- `key_type`: Filter by key type (encryption, signing, verification)
- `status`: Filter by status (active, revoked, expired)
- `purpose`: Filter by purpose (authentication, data_protection, communication)

**Response:**
```json
{
  "keys": [
    {
      "key_id": "key_12345",
      "key_type": "encryption",
      "algorithm": "AES-256",
      "status": "active",
      "created_at": "2024-01-01T00:00:00Z",
      "expires_at": "2024-12-31T23:59:59Z",
      "purpose": "data_protection",
      "metadata": {
        "description": "Primary data encryption key",
        "rotation_schedule": "quarterly"
      }
    }
  ],
  "total_count": 25,
  "page": 1,
  "page_size": 50
}
```

#### POST `/api/v1/security/keys`

Generate new cryptographic keys.

**Request:**
```json
{
  "key_type": "encryption",
  "algorithm": "AES-256",
  "purpose": "data_protection",
  "expiry_duration_days": 365,
  "metadata": {
    "description": "New encryption key for project X",
    "department": "engineering"
  }
}
```

**Response:**
```json
{
  "success": true,
  "key_id": "key_67890",
  "key_info": {
    "algorithm": "AES-256",
    "key_length": 256,
    "created_at": "2024-01-19T12:00:00Z",
    "expires_at": "2025-01-19T12:00:00Z",
    "status": "active"
  }
}
```

#### DELETE `/api/v1/security/keys/{key_id}`

Revoke or delete cryptographic keys.

**Response:**
```json
{
  "success": true,
  "message": "Key key_12345 has been securely revoked",
  "revoked_at": "2024-01-19T12:00:00Z"
}
```

---

## Genetic Operations API

### Overview

The Genetic Operations API enables spawning new nodes, analyzing genetic fitness, and managing evolutionary processes.

### Node Spawning

#### POST `/api/v1/genetics/spawn`

Create new nodes through genetic spawning.

**Request:**
```json
{
  "parent_id": "parent_node_123",
  "co_parents": ["coparent_node_456", "coparent_node_789"],
  "spawn_purpose": "security_response",
  "target_capabilities": [
    "threat_detection",
    "incident_response",
    "forensic_analysis"
  ],
  "resource_limits": {
    "max_memory_mb": 4096,
    "max_cpu_cores": 8,
    "max_storage_gb": 100,
    "network_bandwidth_mbps": 1000
  },
  "security_requirements": [
    "encrypted_storage",
    "secure_boot",
    "tpm_required"
  ],
  "compliance_requirements": [
    "gdpr_compliant",
    "hipaa_compliant"
  ],
  "spawn_restrictions": [
    "no_internet_access",
    "audit_all_operations"
  ],
  "metadata": {
    "project": "security_enhancement",
    "priority": "high",
    "requested_by": "security_team"
  }
}
```

**Response:**
```json
{
  "success": true,
  "spawn_request_id": "spawn_req_abc123",
  "child_node_id": "child_node_xyz789",
  "spawn_details": {
    "generation": 2,
    "genetic_fitness_score": 0.87,
    "inherited_capabilities": [
      "threat_detection",
      "incident_response"
    ],
    "new_mutations": [
      "enhanced_ml_processing",
      "quantum_resistance"
    ],
    "spawn_time_seconds": 45.2,
    "resource_allocation": {
      "memory_mb": 2048,
      "cpu_cores": 4,
      "storage_gb": 50
    }
  },
  "workflow_id": "workflow_spawn_456"
}
```

#### GET `/api/v1/genetics/spawn/{spawn_request_id}/status`

Check the status of a spawning operation.

**Response:**
```json
{
  "spawn_request_id": "spawn_req_abc123",
  "status": "completed",
  "progress": 100,
  "child_node_id": "child_node_xyz789",
  "spawning_stages": [
    {
      "stage": "genetic_analysis",
      "status": "completed",
      "duration_seconds": 12.5
    },
    {
      "stage": "resource_allocation",
      "status": "completed", 
      "duration_seconds": 8.3
    },
    {
      "stage": "capability_inheritance",
      "status": "completed",
      "duration_seconds": 15.7
    },
    {
      "stage": "security_configuration",
      "status": "completed",
      "duration_seconds": 8.7
    }
  ],
  "estimated_completion": null,
  "errors": []
}
```

### Genetic Analysis

#### GET `/api/v1/genetics/nodes/{node_id}/fitness`

Analyze genetic fitness of a node.

**Response:**
```json
{
  "node_id": "node_123",
  "fitness_score": 0.92,
  "fitness_analysis": {
    "performance_score": 0.95,
    "security_score": 0.88,
    "adaptability_score": 0.94,
    "efficiency_score": 0.91
  },
  "genetic_profile": {
    "generation": 3,
    "parent_lineage": ["parent_node_123", "ancestor_node_456"],
    "dominant_traits": [
      "high_throughput_processing",
      "advanced_threat_detection",
      "quantum_resistant_crypto"
    ],
    "recessive_traits": [
      "legacy_compatibility",
      "verbose_logging"
    ],
    "mutations": [
      {
        "mutation_id": "mut_001",
        "type": "performance_enhancement",
        "impact_score": 0.15,
        "stability": "stable"
      }
    ]
  },
  "recommendations": [
    "Consider breeding with high-security nodes to improve security score",
    "Current performance characteristics are optimal for threat detection",
    "Monitor mutation mut_001 for long-term stability"
  ]
}
```

#### POST `/api/v1/genetics/analysis/population`

Analyze genetic diversity and health of node populations.

**Request:**
```json
{
  "population_filter": {
    "node_types": ["security", "compute"],
    "generations": [2, 3, 4],
    "active_only": true
  },
  "analysis_depth": "comprehensive"
}
```

**Response:**
```json
{
  "population_stats": {
    "total_nodes": 150,
    "active_nodes": 142,
    "average_fitness": 0.78,
    "genetic_diversity_index": 0.85,
    "generations_analyzed": [2, 3, 4]
  },
  "diversity_analysis": {
    "trait_distribution": {
      "threat_detection": 0.92,
      "quantum_resistance": 0.34,
      "ml_processing": 0.67
    },
    "mutation_frequency": 0.12,
    "crossover_success_rate": 0.89
  },
  "population_health": {
    "health_score": 0.91,
    "inbreeding_coefficient": 0.05,
    "genetic_bottlenecks": [],
    "recommended_breeding_pairs": [
      {
        "parent1": "node_123",
        "parent2": "node_456",
        "expected_fitness": 0.94,
        "traits_to_enhance": ["security", "efficiency"]
      }
    ]
  }
}
```

---

## Workflow Management API

### Overview

The Workflow API manages multi-party approval processes, automated workflows, and business process orchestration.

### Workflow Operations

#### POST `/api/v1/workflows`

Create new workflow instances.

**Request:**
```json
{
  "workflow_type": "genetic_spawning_approval",
  "initiator": "user_123",
  "target": {
    "type": "node",
    "id": "target_node_456"
  },
  "parameters": {
    "spawn_request_id": "spawn_req_abc123",
    "urgency": "high",
    "business_justification": "Critical security enhancement required"
  },
  "approvers": [
    {
      "user_id": "manager_789",
      "role": "security_manager",
      "required": true
    },
    {
      "user_id": "admin_012", 
      "role": "system_admin",
      "required": true
    }
  ],
  "approval_policy": {
    "minimum_approvals": 2,
    "timeout_hours": 24,
    "escalation_policy": "auto_approve_on_timeout"
  },
  "metadata": {
    "priority": "critical",
    "department": "security",
    "project": "incident_response"
  }
}
```

**Response:**
```json
{
  "success": true,
  "workflow_id": "workflow_def456",
  "workflow_details": {
    "status": "pending_approvals",
    "created_at": "2024-01-19T12:00:00Z",
    "estimated_completion": "2024-01-20T12:00:00Z",
    "approval_progress": {
      "approved": 0,
      "required": 2,
      "pending_approvers": [
        {
          "user_id": "manager_789",
          "role": "security_manager",
          "notified_at": "2024-01-19T12:00:00Z"
        },
        {
          "user_id": "admin_012",
          "role": "system_admin", 
          "notified_at": "2024-01-19T12:00:00Z"
        }
      ]
    }
  }
}
```

#### GET `/api/v1/workflows/{workflow_id}`

Get workflow status and details.

**Response:**
```json
{
  "workflow_id": "workflow_def456",
  "status": "approved",
  "workflow_type": "genetic_spawning_approval",
  "created_at": "2024-01-19T12:00:00Z",
  "completed_at": "2024-01-19T14:30:00Z",
  "duration_minutes": 150,
  "initiator": {
    "user_id": "user_123",
    "username": "security_analyst"
  },
  "approval_history": [
    {
      "approver_id": "manager_789",
      "action": "approved",
      "timestamp": "2024-01-19T13:15:00Z",
      "comments": "Approved for critical security enhancement"
    },
    {
      "approver_id": "admin_012",
      "action": "approved",
      "timestamp": "2024-01-19T14:30:00Z",
      "comments": "Resources allocated, proceeding with spawn"
    }
  ],
  "outcome": {
    "result": "approved",
    "next_actions": [
      "execute_genetic_spawn",
      "notify_stakeholders",
      "begin_monitoring"
    ]
  }
}
```

#### PUT `/api/v1/workflows/{workflow_id}/approve`

Approve or reject workflow requests.

**Request:**
```json
{
  "action": "approve",
  "comments": "Approved after security review",
  "conditions": [
    "monitor_for_24_hours",
    "security_audit_required"
  ]
}
```

**Response:**
```json
{
  "success": true,
  "approval_recorded": true,
  "workflow_status": "partially_approved",
  "remaining_approvals": 1,
  "next_steps": [
    "Waiting for admin_012 approval",
    "Estimated completion: 2024-01-19T18:00:00Z"
  ]
}
```

### Workflow Templates

#### GET `/api/v1/workflows/templates`

List available workflow templates.

**Response:**
```json
{
  "templates": [
    {
      "template_id": "genetic_spawning_template",
      "name": "Genetic Node Spawning Approval",
      "description": "Multi-party approval for genetic spawning operations",
      "required_roles": ["security_manager", "system_admin"],
      "estimated_duration_hours": 2,
      "automation_level": "semi_automated"
    },
    {
      "template_id": "security_incident_template",
      "name": "Security Incident Response",
      "description": "Emergency response workflow for security incidents", 
      "required_roles": ["incident_commander", "security_lead"],
      "estimated_duration_hours": 1,
      "automation_level": "automated"
    }
  ]
}
```

---

## System Management API

### Overview

System management APIs provide health monitoring, configuration management, and operational control.

### Health Monitoring

#### GET `/api/v1/system/health`

Get comprehensive system health status.

**Response:**
```json
{
  "system_status": "healthy",
  "timestamp": "2024-01-19T12:00:00Z",
  "uptime_seconds": 2678400,
  "version": "1.0.0",
  "components": {
    "core": {
      "status": "healthy",
      "response_time_ms": 15,
      "memory_usage_mb": 512,
      "cpu_usage_percent": 25
    },
    "security": {
      "status": "healthy", 
      "active_sessions": 42,
      "failed_auth_attempts_last_hour": 3,
      "key_rotation_status": "current"
    },
    "genetics": {
      "status": "healthy",
      "active_spawn_processes": 2,
      "population_size": 150,
      "average_fitness": 0.78
    },
    "database": {
      "status": "healthy",
      "connection_pool_usage": "45%",
      "query_latency_ms": 12,
      "storage_usage_gb": 45.2
    },
    "network": {
      "status": "healthy",
      "active_connections": 156,
      "throughput_mbps": 234,
      "latency_ms": 8
    }
  },
  "alerts": [
    {
      "severity": "warning",
      "component": "security",
      "message": "Authentication failure rate increased",
      "timestamp": "2024-01-19T11:45:00Z"
    }
  ],
  "metrics_summary": {
    "requests_per_second": 1250,
    "error_rate": 0.02,
    "availability_percent": 99.97
  }
}
```

#### GET `/api/v1/system/metrics`

Get detailed system metrics and performance data.

**Query Parameters:**
- `timeframe`: Time range for metrics (1h, 6h, 24h, 7d)
- `components`: Comma-separated list of components
- `metrics`: Specific metrics to retrieve

**Response:**
```json
{
  "timeframe": "24h",
  "metrics": {
    "performance": {
      "avg_response_time_ms": 45.2,
      "p95_response_time_ms": 120.5,
      "p99_response_time_ms": 250.1,
      "requests_per_second": {
        "current": 1250,
        "peak": 2400,
        "average": 980
      }
    },
    "security": {
      "authentication_success_rate": 0.987,
      "encryption_operations_per_second": 450,
      "signature_operations_per_second": 230,
      "active_security_sessions": 42
    },
    "genetics": {
      "spawn_operations_completed": 15,
      "average_spawn_time_seconds": 45.2,
      "genetic_fitness_trend": 0.025,
      "population_growth_rate": 0.12
    },
    "resources": {
      "memory_usage": {
        "current_mb": 2048,
        "peak_mb": 2856,
        "average_mb": 1920
      },
      "cpu_usage": {
        "current_percent": 25,
        "peak_percent": 78,
        "average_percent": 35
      },
      "disk_usage": {
        "current_gb": 45.2,
        "growth_rate_gb_per_day": 1.2
      }
    }
  }
}
```

### Configuration Management

#### GET `/api/v1/system/config`

Retrieve system configuration.

**Response:**
```json
{
  "configuration": {
    "system": {
      "environment": "production",
      "log_level": "info",
      "max_concurrent_requests": 1000,
      "request_timeout_seconds": 30
    },
    "security": {
      "encryption_algorithm": "AES-256-GCM",
      "signature_algorithm": "Ed25519",
      "session_timeout_minutes": 60,
      "mfa_required": true
    },
    "genetics": {
      "max_population_size": 500,
      "mutation_rate": 0.15,
      "fitness_threshold": 0.75,
      "spawn_timeout_minutes": 10
    },
    "database": {
      "connection_pool_size": 20,
      "query_timeout_seconds": 30,
      "backup_schedule": "daily_2am"
    }
  }
}
```

#### PUT `/api/v1/system/config`

Update system configuration (requires admin privileges).

**Request:**
```json
{
  "configuration_updates": {
    "security": {
      "session_timeout_minutes": 90
    },
    "genetics": {
      "max_population_size": 750
    }
  },
  "restart_required": false
}
```

**Response:**
```json
{
  "success": true,
  "updated_at": "2024-01-19T12:00:00Z",
  "changes_applied": [
    "security.session_timeout_minutes: 60 → 90",
    "genetics.max_population_size: 500 → 750"
  ],
  "restart_required": false
}
```

---

## Compliance & Audit API

### Overview

Compliance and audit APIs provide regulatory compliance checking, audit trail management, and reporting capabilities.

### Compliance Checking

#### POST `/api/v1/compliance/validate`

Validate compliance against specified standards.

**Request:**
```json
{
  "standards": ["gdpr", "hipaa", "sox"],
  "scope": {
    "data_types": ["personal_data", "health_records"],
    "operations": ["data_processing", "storage", "transmission"],
    "time_period": {
      "start": "2024-01-01T00:00:00Z",
      "end": "2024-01-19T23:59:59Z"
    }
  },
  "validation_level": "comprehensive"
}
```

**Response:**
```json
{
  "validation_id": "validation_789",
  "overall_compliance": {
    "status": "compliant",
    "score": 0.94,
    "last_validated": "2024-01-19T12:00:00Z"
  },
  "standard_results": {
    "gdpr": {
      "compliant": true,
      "score": 0.96,
      "requirements_met": 47,
      "requirements_total": 49,
      "violations": [
        {
          "requirement": "data_retention_limits",
          "severity": "medium",
          "description": "Some log files exceed 90-day retention policy",
          "remediation": "Implement automated log rotation"
        }
      ]
    },
    "hipaa": {
      "compliant": true,
      "score": 0.98,
      "requirements_met": 23,
      "requirements_total": 23,
      "violations": []
    },
    "sox": {
      "compliant": true,
      "score": 0.89,
      "requirements_met": 15,
      "requirements_total": 17,
      "violations": [
        {
          "requirement": "financial_data_audit_trail",
          "severity": "high",
          "description": "Incomplete audit trail for financial calculations",
          "remediation": "Enable detailed transaction logging"
        }
      ]
    }
  },
  "recommendations": [
    "Implement automated log retention policies",
    "Enhance financial transaction audit logging",
    "Schedule quarterly compliance reviews"
  ]
}
```

### Audit Trail Management

#### GET `/api/v1/audit/events`

Retrieve audit events and logs.

**Query Parameters:**
- `start_time`: Start of time range (ISO 8601)
- `end_time`: End of time range (ISO 8601)
- `event_types`: Comma-separated list of event types
- `user_id`: Filter by user ID
- `severity`: Filter by severity level
- `limit`: Number of results to return
- `offset`: Pagination offset

**Response:**
```json
{
  "events": [
    {
      "event_id": "audit_001",
      "timestamp": "2024-01-19T12:00:00Z",
      "event_type": "authentication",
      "severity": "info",
      "user_id": "user_123",
      "source_ip": "192.168.1.100",
      "details": {
        "action": "login_success",
        "method": "password_mfa",
        "session_id": "session_abc123"
      },
      "outcome": "success"
    },
    {
      "event_id": "audit_002", 
      "timestamp": "2024-01-19T12:05:00Z",
      "event_type": "genetic_operation",
      "severity": "info",
      "user_id": "user_123",
      "details": {
        "action": "spawn_request",
        "parent_node": "node_456",
        "spawn_purpose": "security_response",
        "workflow_id": "workflow_def456"
      },
      "outcome": "success"
    },
    {
      "event_id": "audit_003",
      "timestamp": "2024-01-19T12:10:00Z", 
      "event_type": "security_operation",
      "severity": "warning",
      "user_id": "user_789",
      "details": {
        "action": "failed_encryption",
        "key_id": "key_invalid",
        "error": "key_not_found"
      },
      "outcome": "failure"
    }
  ],
  "pagination": {
    "total_events": 1547,
    "current_page": 1,
    "page_size": 50,
    "total_pages": 31
  },
  "summary": {
    "success_rate": 0.976,
    "most_common_events": [
      "authentication",
      "data_access", 
      "configuration_change"
    ]
  }
}
```

#### POST `/api/v1/audit/export`

Export audit data for compliance reporting.

**Request:**
```json
{
  "export_format": "json",
  "time_range": {
    "start": "2024-01-01T00:00:00Z",
    "end": "2024-01-31T23:59:59Z"
  },
  "filters": {
    "event_types": ["security_operation", "data_access"],
    "severity_levels": ["warning", "error", "critical"]
  },
  "include_metadata": true,
  "compression": "gzip"
}
```

**Response:**
```json
{
  "export_id": "export_456",
  "status": "processing",
  "estimated_completion": "2024-01-19T12:15:00Z",
  "download_url": null,
  "file_size_estimate_mb": 45.2
}
```

---

## Node Registry API

### Overview

The Node Registry API manages node discovery, registration, and federation across the BearDog network.

### Node Management

#### GET `/api/v1/registry/nodes`

List registered nodes in the network.

**Query Parameters:**
- `status`: Filter by node status (active, inactive, maintenance)
- `node_type`: Filter by node type (security, compute, storage)
- `capabilities`: Filter by capabilities
- `location`: Filter by geographic location

**Response:**
```json
{
  "nodes": [
    {
      "node_id": "node_123",
      "node_type": "security",
      "status": "active",
      "capabilities": [
        "threat_detection",
        "incident_response",
        "forensic_analysis"
      ],
      "location": {
        "region": "us-east-1",
        "availability_zone": "us-east-1a",
        "coordinates": {
          "latitude": 40.7128,
          "longitude": -74.0060
        }
      },
      "resources": {
        "cpu_cores": 16,
        "memory_gb": 64,
        "storage_gb": 1000,
        "network_bandwidth_gbps": 10
      },
      "health": {
        "status": "healthy",
        "cpu_usage": 0.25,
        "memory_usage": 0.40,
        "last_heartbeat": "2024-01-19T11:59:30Z"
      },
      "genetics": {
        "generation": 3,
        "fitness_score": 0.89,
        "parent_nodes": ["node_045", "node_067"]
      },
      "registered_at": "2024-01-15T10:30:00Z",
      "last_updated": "2024-01-19T11:59:30Z"
    }
  ],
  "pagination": {
    "total_nodes": 150,
    "active_nodes": 142,
    "page": 1,
    "page_size": 50
  }
}
```

#### POST `/api/v1/registry/nodes`

Register a new node in the network.

**Request:**
```json
{
  "node_info": {
    "node_type": "compute",
    "capabilities": [
      "data_processing",
      "machine_learning",
      "distributed_computing"
    ],
    "resources": {
      "cpu_cores": 32,
      "memory_gb": 128,
      "storage_gb": 2000,
      "gpu_count": 4
    },
    "location": {
      "region": "us-west-2",
      "availability_zone": "us-west-2b"
    },
    "security_profile": {
      "encryption_capabilities": ["AES-256", "ChaCha20-Poly1305"],
      "signature_algorithms": ["Ed25519", "ECDSA-P256"],
      "secure_boot_enabled": true,
      "tpm_version": "2.0"
    }
  },
  "registration_metadata": {
    "owner": "organization_xyz",
    "purpose": "ml_workloads",
    "compliance_requirements": ["gdpr", "hipaa"]
  }
}
```

**Response:**
```json
{
  "success": true,
  "node_id": "node_789",
  "registration_details": {
    "registered_at": "2024-01-19T12:00:00Z",
    "registration_token": "reg_token_def456",
    "network_address": "node-789.beardog.local",
    "assigned_region": "us-west-2"
  },
  "next_steps": [
    "Configure node with provided registration token",
    "Complete security attestation process",
    "Begin health reporting within 5 minutes"
  ]
}
```

#### GET `/api/v1/registry/nodes/{node_id}`

Get detailed information about a specific node.

**Response:**
```json
{
  "node_id": "node_123",
  "detailed_info": {
    "node_type": "security",
    "status": "active",
    "uptime_seconds": 2678400,
    "version": "1.0.0",
    "capabilities": {
      "threat_detection": {
        "algorithms": ["signature_based", "ml_anomaly", "behavioral"],
        "threat_types": ["malware", "network_intrusion", "data_exfiltration"],
        "performance_rating": 0.92
      },
      "incident_response": {
        "response_types": ["containment", "forensics", "recovery"],
        "average_response_time_seconds": 45,
        "success_rate": 0.96
      }
    },
    "performance_metrics": {
      "requests_processed_last_hour": 5420,
      "average_latency_ms": 15.2,
      "error_rate": 0.002,
      "throughput_ops_per_second": 1200
    },
    "network_topology": {
      "connected_nodes": ["node_456", "node_789", "node_012"],
      "connection_quality": {
        "latency_ms": 8,
        "bandwidth_mbps": 950,
        "packet_loss": 0.001
      }
    },
    "security_status": {
      "attestation_valid": true,
      "last_security_scan": "2024-01-19T06:00:00Z",
      "vulnerabilities_count": 0,
      "security_score": 0.97
    }
  }
}
```

---

## Error Handling

### Standard Error Response

All API errors follow a consistent format:

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Request validation failed",
    "details": {
      "field": "username",
      "reason": "Username must be at least 3 characters long"
    },
    "request_id": "req_123abc",
    "timestamp": "2024-01-19T12:00:00Z"
  }
}
```

### Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `AUTHENTICATION_REQUIRED` | 401 | Authentication credentials missing or invalid |
| `AUTHORIZATION_DENIED` | 403 | User lacks required permissions |
| `RESOURCE_NOT_FOUND` | 404 | Requested resource does not exist |
| `VALIDATION_ERROR` | 400 | Request data validation failed |
| `RATE_LIMIT_EXCEEDED` | 429 | API rate limit exceeded |
| `INTERNAL_ERROR` | 500 | Internal server error occurred |
| `SERVICE_UNAVAILABLE` | 503 | Service temporarily unavailable |
| `GENETIC_SPAWN_FAILED` | 422 | Genetic spawning operation failed |
| `WORKFLOW_ERROR` | 422 | Workflow processing error |
| `COMPLIANCE_VIOLATION` | 422 | Operation violates compliance requirements |

---

## Rate Limiting

### Rate Limit Headers

All responses include rate limiting information:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1642123456
X-RateLimit-Window: 3600
```

### Rate Limit Tiers

| Tier | Requests/Hour | Burst Limit |
|------|---------------|-------------|
| Basic | 1,000 | 50 |
| Premium | 10,000 | 500 |
| Enterprise | 100,000 | 5,000 |
| System | Unlimited | Unlimited |

### Rate Limit Exceeded Response

```json
{
  "success": false,
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "API rate limit exceeded",
    "details": {
      "limit": 1000,
      "window_seconds": 3600,
      "reset_at": "2024-01-19T13:00:00Z",
      "retry_after_seconds": 1800
    }
  }
}
```

---

## SDK Examples

### JavaScript/TypeScript SDK

```typescript
import { BearDogClient, BearDogConfig } from '@beardog/sdk';

// Initialize client
const config: BearDogConfig = {
  baseUrl: 'https://api.beardog.local',
  authentication: {
    method: 'bearer_token',
    token: 'your_access_token_here'
  },
  timeout: 30000,
  retries: 3
};

const client = new BearDogClient(config);

// Authenticate
async function authenticate() {
  try {
    const response = await client.auth.authenticate({
      username: 'user@example.com',
      password: 'secure_password',
      mfa_token: '123456'
    });
    
    console.log('Authentication successful:', response.user_info);
    return response.access_token;
  } catch (error) {
    console.error('Authentication failed:', error);
    throw error;
  }
}

// Encrypt data
async function encryptData(data: string) {
  try {
    const response = await client.security.encrypt({
      data: Buffer.from(data).toString('base64'),
      encryption_method: 'AES-256-GCM',
      key_id: 'default_encryption_key'
    });
    
    return response.encrypted_data;
  } catch (error) {
    console.error('Encryption failed:', error);
    throw error;
  }
}

// Spawn genetic node
async function spawnNode(parentId: string) {
  try {
    const response = await client.genetics.spawn({
      parent_id: parentId,
      spawn_purpose: 'security_response',
      target_capabilities: ['threat_detection', 'incident_response'],
      resource_limits: {
        max_memory_mb: 4096,
        max_cpu_cores: 8
      }
    });
    
    console.log('Node spawned:', response.child_node_id);
    return response;
  } catch (error) {
    console.error('Spawning failed:', error);
    throw error;
  }
}
```

### Python SDK

```python
from beardog_sdk import BearDogClient, BearDogConfig, BearDogError
import asyncio
import base64

# Initialize client
config = BearDogConfig(
    base_url='https://api.beardog.local',
    auth_token='your_access_token_here',
    timeout=30.0,
    max_retries=3
)

client = BearDogClient(config)

async def main():
    try:
        # Authenticate
        auth_response = await client.auth.authenticate(
            username='user@example.com',
            password='secure_password',
            mfa_token='123456'
        )
        print(f"Authenticated as: {auth_response.user_info.username}")
        
        # Update client token
        client.set_auth_token(auth_response.access_token)
        
        # Encrypt sensitive data
        plaintext = "Sensitive information"
        encrypted_response = await client.security.encrypt(
            data=base64.b64encode(plaintext.encode()).decode(),
            encryption_method='AES-256-GCM',
            key_id='default_encryption_key'
        )
        print(f"Data encrypted successfully")
        
        # Create workflow
        workflow_response = await client.workflows.create(
            workflow_type='security_audit',
            initiator='security_team',
            parameters={
                'audit_scope': 'full_system',
                'priority': 'high'
            },
            approvers=[
                {'user_id': 'manager_001', 'role': 'security_manager'}
            ]
        )
        print(f"Workflow created: {workflow_response.workflow_id}")
        
        # Monitor system health
        health_response = await client.system.health()
        print(f"System status: {health_response.system_status}")
        
    except BearDogError as e:
        print(f"API Error: {e.code} - {e.message}")
    except Exception as e:
        print(f"Unexpected error: {e}")

if __name__ == "__main__":
    asyncio.run(main())
```

### Rust SDK

```rust
use beardog_sdk::{BearDogClient, BearDogConfig, BearDogError};
use beardog_sdk::models::{AuthRequest, EncryptRequest, SpawnRequest};

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    // Initialize client
    let config = BearDogConfig::builder()
        .base_url("https://api.beardog.local")
        .auth_token("your_access_token_here")
        .timeout(std::time::Duration::from_secs(30))
        .max_retries(3)
        .build();

    let mut client = BearDogClient::new(config)?;

    // Authenticate
    let auth_request = AuthRequest {
        username: "user@example.com".to_string(),
        password: "secure_password".to_string(),
        mfa_token: Some("123456".to_string()),
        authentication_method: "password_mfa".to_string(),
    };

    let auth_response = client.auth().authenticate(auth_request).await?;
    println!("Authenticated as: {}", auth_response.user_info.username);

    // Update client credentials
    client.set_auth_token(&auth_response.access_token);

    // Perform security operations
    let encrypt_request = EncryptRequest {
        data: base64::encode("Sensitive data"),
        encryption_method: "AES-256-GCM".to_string(),
        key_id: "default_encryption_key".to_string(),
        additional_data: None,
        encoding: Some("base64".to_string()),
    };

    let encrypt_response = client.security().encrypt(encrypt_request).await?;
    println!("Data encrypted successfully");

    // Spawn genetic node
    let spawn_request = SpawnRequest {
        parent_id: "parent_node_123".to_string(),
        spawn_purpose: beardog_sdk::models::SpawnPurpose::SecurityResponse,
        target_capabilities: vec![
            beardog_sdk::models::NodeCapability::ThreatDetection,
            beardog_sdk::models::NodeCapability::IncidentResponse,
        ],
        resource_limits: beardog_sdk::models::ResourceLimits {
            max_memory_mb: 4096,
            max_cpu_cores: 8,
            max_storage_gb: 100,
            network_bandwidth_mbps: 1000,
        },
        security_requirements: vec!["encrypted_storage".to_string()],
        compliance_requirements: vec!["gdpr_compliant".to_string()],
        co_parents: vec![],
        spawn_restrictions: vec![],
        metadata: std::collections::HashMap::new(),
    };

    let spawn_response = client.genetics().spawn(spawn_request).await?;
    println!("Node spawned: {}", spawn_response.child_node_id);

    // Get system health
    let health_response = client.system().health().await?;
    println!("System status: {}", health_response.system_status);

    Ok(())
}
```

---

## Conclusion

This comprehensive API documentation provides complete coverage of BearDog's capabilities. For additional support:

- **Documentation**: [docs.beardog.local](https://docs.beardog.local)
- **API Reference**: [api-docs.beardog.local](https://api-docs.beardog.local)
- **SDK Downloads**: [sdk.beardog.local](https://sdk.beardog.local)
- **Support**: [support@beardog.local](mailto:support@beardog.local)

The API is designed for high performance, security, and reliability. All operations include comprehensive error handling, audit logging, and compliance checking to ensure enterprise-grade operation. 