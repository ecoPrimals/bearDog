# BearDog API Documentation

## Overview

The BearDog API provides a comprehensive RESTful interface for all system operations, including authentication, genetic spawning, security analysis, compliance management, and ecosystem integration.

**Base URL**: `https://api.beardog.local`  
**API Version**: `v1`  
**Authentication**: Bearer Token (JWT)

## Quick Start

### 1. Authentication

```bash
# Get an authentication token
curl -X POST https://api.beardog.local/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "your_username", "password": "your_password"}'

# Response
{
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_at": "2024-12-31T23:59:59Z",
    "user_info": {
      "username": "your_username",
      "permissions": ["read", "write", "spawn"]
    }
  },
  "metadata": {
    "request_id": "req_123456",
    "processing_time_ms": 45
  }
}
```

### 2. Using the API

```bash
# Use the token in subsequent requests
curl -X GET https://api.beardog.local/api/auth/session \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

## Authentication API

### POST /api/auth/login

Authenticate a user and receive a JWT token.

**Request Body:**
```json
{
  "username": "string",
  "password": "string",
  "mfa_code": "string (optional)",
  "remember_device": "boolean (optional)"
}
```

**Response:**
```json
{
  "data": {
    "token": "string",
    "refresh_token": "string",
    "expires_at": "datetime",
    "user_info": {
      "username": "string",
      "permissions": ["string"],
      "roles": ["string"]
    }
  }
}
```

### POST /api/auth/refresh

Refresh an expired JWT token.

**Request Body:**
```json
{
  "refresh_token": "string"
}
```

### POST /api/auth/logout

Invalidate the current session.

**Headers:** `Authorization: Bearer <token>`

### GET /api/auth/session

Get current session information.

**Headers:** `Authorization: Bearer <token>`

**Response:**
```json
{
  "data": {
    "username": "string",
    "permissions": ["string"],
    "expires_at": "datetime",
    "session_id": "string"
  }
}
```

## Genetic Spawning API

### POST /api/genetics/spawn

Create a new genetic spawn instance.

**Headers:** `Authorization: Bearer <token>`

**Request Body:**
```json
{
  "purpose": "string",
  "parent_genetics": ["string"],
  "resource_limits": {
    "max_memory_mb": "number",
    "max_cpu_percent": "number",
    "max_disk_mb": "number",
    "max_network_mbps": "number",
    "max_concurrent_connections": "number"
  },
  "target_capabilities": ["string"],
  "security_clearance": "Basic|Medium|High|Maximum"
}
```

**Response:**
```json
{
  "data": {
    "spawn_id": "string",
    "genetics": {
      "id": "string",
      "generation": "number",
      "fitness_score": "number",
      "capabilities": ["string"],
      "specializations": ["string"]
    },
    "success": "boolean",
    "messages": ["string"]
  }
}
```

### GET /api/genetics/spawn/{spawn_id}/status

Get the status of a genetic spawn.

**Headers:** `Authorization: Bearer <token>`

**Response:**
```json
{
  "data": {
    "spawn_id": "string",
    "status": "Initializing|Active|Paused|Terminated|Failed",
    "health_percentage": "number",
    "uptime_seconds": "number",
    "last_activity": "datetime",
    "resource_usage": {
      "memory_mb": "number",
      "cpu_percent": "number",
      "disk_mb": "number"
    }
  }
}
```

### POST /api/genetics/recombine

Perform genetic recombination between two parent genetics.

**Headers:** `Authorization: Bearer <token>`

**Request Body:**
```json
{
  "parent1_id": "string",
  "parent2_id": "string",
  "mutation_rate": "number (0.0-1.0)",
  "crossover_rate": "number (0.0-1.0)"
}
```

## Security Analysis API

### POST /api/security/analyze

Analyze a security event for threats.

**Headers:** `Authorization: Bearer <token>`

**Request Body:**
```json
{
  "event_type": "string",
  "source_ip": "string",
  "destination_ip": "string",
  "user_id": "string",
  "data_size": "number (optional)",
  "user_agent": "string (optional)",
  "location": "string (optional)",
  "additional_data": {
    "key": "value"
  }
}
```

**Response:**
```json
{
  "data": {
    "event_id": "string",
    "threats_detected": "number",
    "risk_level": "LOW|MEDIUM|HIGH|CRITICAL",
    "detected_threats": [
      {
        "threat_id": "string",
        "threat_type": "string",
        "severity": "string",
        "description": "string",
        "evidence": ["string"]
      }
    ],
    "ml_predictions": [
      {
        "model_id": "string",
        "prediction_type": "string",
        "confidence_score": "number",
        "recommendations": ["string"]
      }
    ],
    "incident_created": "string (optional)"
  }
}
```

### POST /api/security/analyze/batch

Analyze multiple security events in batch.

**Headers:** `Authorization: Bearer <token>`

**Request Body:**
```json
{
  "events": [
    {
      "event_type": "string",
      "source_ip": "string",
      "destination_ip": "string",
      "user_id": "string"
    }
  ]
}
```

### POST /api/security/ml/predict

Get ML-powered threat predictions for an event.

**Headers:** `Authorization: Bearer <token>`

### GET /api/security/statistics

Get security statistics and metrics.

**Headers:** `Authorization: Bearer <token>`

**Response:**
```json
{
  "data": {
    "total_events_analyzed": "number",
    "threats_detected_today": "number",
    "active_incidents": "number",
    "ml_models_active": "number",
    "detection_rules_active": "number",
    "threat_distribution": {
      "SUSPICIOUS_LOGIN": "number",
      "DATA_EXFILTRATION": "number",
      "MALWARE_DETECTION": "number"
    },
    "top_threat_sources": ["string"]
  }
}
```

## Compliance API

### POST /api/compliance/audit/start

Start a compliance audit.

**Headers:** `Authorization: Bearer <token>`

**Request Body:**
```json
{
  "audit_type": "GDPR|HIPAA|SOX|PCI_DSS",
  "scope": ["string"],
  "automated_remediation": "boolean"
}
```

**Response:**
```json
{
  "data": {
    "audit_id": "string",
    "audit_type": "string",
    "status": "STARTED",
    "created_at": "datetime",
    "estimated_completion": "datetime"
  }
}
```

### GET /api/compliance/audit/{audit_id}/status

Get audit status and results.

**Headers:** `Authorization: Bearer <token>`

**Response:**
```json
{
  "data": {
    "audit_id": "string",
    "status": "IN_PROGRESS|COMPLETED|FAILED",
    "progress_percentage": "number",
    "compliance_score": "number",
    "violations_found": "number",
    "recommendations": ["string"],
    "report_url": "string (when completed)"
  }
}
```

### GET /api/compliance/status

Get overall compliance status.

**Headers:** `Authorization: Bearer <token>`

**Response:**
```json
{
  "data": {
    "overall_score": "number",
    "gdpr_compliance": "number",
    "hipaa_compliance": "number",
    "sox_compliance": "number",
    "pci_dss_compliance": "number",
    "last_audit": "datetime",
    "next_audit": "datetime",
    "active_violations": "number"
  }
}
```

## Sovereignty API

### POST /api/sovereignty/consent/request

Request consent from another party.

**Headers:** `Authorization: Bearer <token>`

**Request Body:**
```json
{
  "requester_id": "string",
  "target_party_id": "string",
  "requested_permissions": ["string"],
  "purpose": "string",
  "urgency_level": "Low|Normal|High|Critical",
  "response_deadline_hours": "number"
}
```

### GET /api/sovereignty/consent/requests

Get pending consent requests.

**Headers:** `Authorization: Bearer <token>`

### POST /api/sovereignty/identity/attest

Create an identity attestation.

**Headers:** `Authorization: Bearer <token>`

## Monitoring API

### GET /api/monitoring/health

Get system health status.

**Response:**
```json
{
  "status": "healthy|degraded|unhealthy",
  "timestamp": "datetime",
  "services": [
    {
      "name": "string",
      "status": "healthy|degraded|unhealthy",
      "response_time_ms": "number",
      "details": "string"
    }
  ],
  "system_info": {
    "uptime_seconds": "number",
    "version": "string",
    "environment": "string"
  }
}
```

### GET /api/monitoring/metrics

Get system metrics in Prometheus format.

### GET /api/monitoring/dashboard

Get dashboard data for monitoring interfaces.

**Headers:** `Authorization: Bearer <token>`

## Rate Limiting

All API endpoints are subject to rate limiting:

- **Default Limit**: 60 requests per minute per IP address
- **Authenticated Limit**: 600 requests per minute per user
- **Burst Capacity**: 10 requests
- **Rate Limit Headers**:
  - `X-RateLimit-Limit`: Maximum requests per window
  - `X-RateLimit-Remaining`: Remaining requests in current window
  - `X-RateLimit-Reset`: When the current window resets

When rate limited, the API returns HTTP 429 with:
```json
{
  "error": "RATE_LIMIT_EXCEEDED",
  "message": "Rate limit exceeded. Try again in 30 seconds.",
  "retry_after": 30
}
```

## Error Responses

The API uses standard HTTP status codes and returns errors in a consistent format:

```json
{
  "error_code": "string",
  "message": "string",
  "details": {
    "field": "validation error details"
  },
  "request_id": "string",
  "timestamp": "datetime"
}
```

### Common Error Codes

- **400 Bad Request**: Invalid request format or parameters
- **401 Unauthorized**: Missing or invalid authentication token
- **403 Forbidden**: Insufficient permissions for the operation
- **404 Not Found**: Resource not found
- **409 Conflict**: Resource already exists or conflict
- **422 Unprocessable Entity**: Valid format but invalid data
- **429 Too Many Requests**: Rate limit exceeded
- **500 Internal Server Error**: Server error
- **503 Service Unavailable**: Service temporarily unavailable

## Response Format

All API responses follow a consistent format:

### Success Response
```json
{
  "data": {
    // Response payload
  },
  "metadata": {
    "request_id": "string",
    "processing_time_ms": "number",
    "timestamp": "datetime",
    "api_version": "string",
    "cached": "boolean"
  }
}
```

### Error Response
```json
{
  "error_code": "string",
  "message": "string",
  "details": {},
  "request_id": "string",
  "timestamp": "datetime"
}
```

## SDKs and Client Libraries

### Curl Examples

```bash
# Health Check
curl -X GET https://api.beardog.local/api/monitoring/health

# Authentication
curl -X POST https://api.beardog.local/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "secure_password"}'

# Genetic Spawning
curl -X POST https://api.beardog.local/api/genetics/spawn \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "purpose": "load_balancing",
    "parent_genetics": [],
    "resource_limits": {
      "max_memory_mb": 1024,
      "max_cpu_percent": 50,
      "max_disk_mb": 5120
    },
    "target_capabilities": ["storage", "compute"]
  }'

# Security Analysis
curl -X POST https://api.beardog.local/api/security/analyze \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "event_type": "login_attempt",
    "source_ip": "203.0.113.1",
    "destination_ip": "192.168.1.100",
    "user_id": "user123"
  }'
```

### Python SDK Example

```python
import requests

class BearDogClient:
    def __init__(self, base_url, username, password):
        self.base_url = base_url
        self.session = requests.Session()
        self.authenticate(username, password)
    
    def authenticate(self, username, password):
        response = self.session.post(
            f"{self.base_url}/api/auth/login",
            json={"username": username, "password": password}
        )
        token = response.json()["data"]["token"]
        self.session.headers.update({"Authorization": f"Bearer {token}"})
    
    def spawn_genetics(self, spawn_request):
        return self.session.post(
            f"{self.base_url}/api/genetics/spawn",
            json=spawn_request
        ).json()
    
    def analyze_security_event(self, event):
        return self.session.post(
            f"{self.base_url}/api/security/analyze",
            json=event
        ).json()

# Usage
client = BearDogClient("https://api.beardog.local", "admin", "password")
result = client.spawn_genetics({
    "purpose": "testing",
    "resource_limits": {"max_memory_mb": 512}
})
```

## Webhook Support

BearDog can send webhook notifications for various events:

### Webhook Events

- `genetic.spawn.created` - New genetic spawn created
- `genetic.spawn.completed` - Genetic spawn completed
- `security.threat.detected` - Security threat detected
- `compliance.audit.completed` - Compliance audit completed
- `system.health.degraded` - System health degraded

### Webhook Format

```json
{
  "event_type": "genetic.spawn.created",
  "timestamp": "datetime",
  "data": {
    // Event-specific data
  },
  "webhook_id": "string",
  "signature": "string"
}
```

## API Versioning

The BearDog API uses semantic versioning:

- **Current Version**: `v1`
- **Header**: `X-BearDog-API-Version: 1.0.0`
- **URL Versioning**: `/api/v1/...` (optional, defaults to latest)

## Support and Resources

- **Documentation**: https://docs.beardog.eco/api
- **Status Page**: https://status.beardog.eco
- **Support**: api-support@beardog.eco
- **Rate Limit Increases**: enterprise@beardog.eco

## Changelog

### v1.0.0 (Current)
- Initial API release
- Full authentication system
- Genetic spawning operations
- Security analysis endpoints
- Compliance management
- Real-time monitoring 