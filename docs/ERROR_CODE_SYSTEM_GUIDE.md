# BearDog Error Code System Guide

**Status**: ✅ **IMPLEMENTED** - Production-Ready

---

## 📖 **Overview**

BearDog's structured error code system provides machine-readable error identifiers for:
- Consistent error tracking across the ecosystem
- Monitoring and alerting systems
- Internationalization (codes are language-independent)
- Error analytics and reporting

---

## 🏗️ **Code Structure**

### Format: `CATEGORY_NNNN_DESCRIPTION`

**Categories** (by 1000s range):
| Range | Category | Examples |
|-------|----------|----------|
| 1000-1999 | Security | Unauthorized, Forbidden, Token Expired |
| 2000-2999 | Network | Connection Timeout, DNS Failed |
| 3000-3999 | HSM/Hardware | Device Not Found, FIDO2 Timeout |
| 4000-4999 | Storage | File Not Found, Disk Full |
| 5000-5999 | Configuration | Parse Error, Missing Field |
| 6000-6999 | Service Discovery | Service Not Found, Health Check Failed |
| 7000-7999 | AI/ML | Model Not Found, Inference Failed |
| 8000-8999 | System | Out of Memory, Deadlock |
| 9000-9999 | Application | Invalid Request, Validation Failed |

---

## 💡 **Usage Examples**

### Basic Usage

```rust
use beardog_errors::{BearDogError, BearDogErrorCode};

// Security error with code
let error = BearDogError::security("Invalid credentials", None)
    .with_code(BearDogErrorCode::SEC_1003_AUTHENTICATION_FAILED);

// Check error code
if error.code() == Some(&BearDogErrorCode::SEC_1003_AUTHENTICATION_FAILED) {
    // Handle authentication failure
    eprintln!("Authentication failed!");
}
```

### Network Error with Code

```rust
use beardog_errors::{BearDogError, BearDogErrorCode};

fn connect_to_service(url: &str) -> Result<(), BearDogError> {
    // Simulate connection failure
    Err(BearDogError::network("Connection timeout", None)
        .with_code(BearDogErrorCode::NET_2002_CONNECTION_TIMEOUT))
}

match connect_to_service("https://api.example.com") {
    Ok(_) => println!("Connected!"),
    Err(e) => {
        if e.code() == Some(&BearDogErrorCode::NET_2002_CONNECTION_TIMEOUT) {
            // Retry logic
            eprintln!("Connection timed out, retrying...");
        }
    }
}
```

### HSM Error with Code

```rust
use beardog_errors::{BearDogError, BearDogErrorCode};

fn detect_fido2_device() -> Result<String, BearDogError> {
    // Simulate device not found
    Err(BearDogError::hsm("FIDO2 device not detected")
        .with_code(BearDogErrorCode::HSM_3020_FIDO2_DEVICE_NOT_FOUND))
}

match detect_fido2_device() {
    Ok(device) => println!("Found device: {}", device),
    Err(e) => {
        match e.code() {
            Some(&BearDogErrorCode::HSM_3020_FIDO2_DEVICE_NOT_FOUND) => {
                eprintln!("Please insert your security key");
            },
            Some(&BearDogErrorCode::HSM_3022_FIDO2_TIMEOUT) => {
                eprintln!("User interaction required");
            },
            _ => eprintln!("HSM error: {}", e),
        }
    }
}
```

---

## 📊 **Code Catalog**

### Security Errors (1000-1999)

| Code | Enum | Description |
|------|------|-------------|
| 1001 | `SEC_1001_UNAUTHORIZED` | Missing or invalid credentials |
| 1002 | `SEC_1002_FORBIDDEN` | Insufficient permissions |
| 1003 | `SEC_1003_AUTHENTICATION_FAILED` | Invalid username/password |
| 1004 | `SEC_1004_TOKEN_EXPIRED` | JWT/session token expired |
| 1005 | `SEC_1005_TOKEN_INVALID` | Malformed or tampered token |
| 1010 | `SEC_1010_ENCRYPTION_FAILED` | Cipher operation error |
| 1011 | `SEC_1011_DECRYPTION_FAILED` | Decryption error or wrong key |
| 1012 | `SEC_1012_KEY_GENERATION_FAILED` | RNG or key derivation error |
| 1020 | `SEC_1020_CERT_VALIDATION_FAILED` | Invalid cert chain |
| 1021 | `SEC_1021_CERT_EXPIRED` | Certificate expired |
| 1022 | `SEC_1022_TLS_HANDSHAKE_FAILED` | TLS handshake error |
| 1030 | `SEC_1030_RATE_LIMIT_EXCEEDED` | Too many requests |
| 1040 | `SEC_1040_MALICIOUS_INPUT_DETECTED` | Potential injection attack |

### Network Errors (2000-2999)

| Code | Enum | Description |
|------|------|-------------|
| 2001 | `NET_2001_CONNECTION_REFUSED` | Target unreachable or port closed |
| 2002 | `NET_2002_CONNECTION_TIMEOUT` | Target not responding |
| 2003 | `NET_2003_CONNECTION_RESET` | Connection dropped |
| 2010 | `NET_2010_REQUEST_TIMEOUT` | No response within timeout |
| 2020 | `NET_2020_DNS_RESOLUTION_FAILED` | Hostname lookup error |
| 2400 | `NET_2400_BAD_REQUEST` | HTTP 400 |
| 2404 | `NET_2404_NOT_FOUND` | HTTP 404 |
| 2500 | `NET_2500_INTERNAL_SERVER_ERROR` | HTTP 500 |
| 2502 | `NET_2502_BAD_GATEWAY` | HTTP 502 |
| 2503 | `NET_2503_SERVICE_UNAVAILABLE` | HTTP 503 |
| 2504 | `NET_2504_GATEWAY_TIMEOUT` | HTTP 504 |
| 2600 | `NET_2600_CIRCUIT_BREAKER_OPEN` | Too many failures |
| 2610 | `NET_2610_NO_HEALTHY_BACKENDS` | Load balancer error |

### HSM/Hardware Errors (3000-3999)

| Code | Enum | Description |
|------|------|-------------|
| 3001 | `HSM_3001_DEVICE_NOT_FOUND` | No hardware detected |
| 3002 | `HSM_3002_OPERATION_FAILED` | Generic hardware error |
| 3003 | `HSM_3003_AUTHENTICATION_REQUIRED` | PIN/passphrase needed |
| 3004 | `HSM_3004_AUTHENTICATION_FAILED` | Wrong PIN/passphrase |
| 3005 | `HSM_3005_PIN_BLOCKED` | Too many failed attempts |
| 3010 | `HSM_3010_PKCS11_INIT_FAILED` | PKCS#11 init error |
| 3011 | `HSM_3011_PKCS11_SESSION_ERROR` | PKCS#11 session error |
| 3012 | `HSM_3012_PKCS11_TOKEN_NOT_PRESENT` | Token not present |
| 3020 | `HSM_3020_FIDO2_DEVICE_NOT_FOUND` | FIDO2 device not detected |
| 3021 | `HSM_3021_FIDO2_OPERATION_CANCELED` | User declined |
| 3022 | `HSM_3022_FIDO2_TIMEOUT` | No user interaction |
| 3030 | `HSM_3030_TPM_NOT_AVAILABLE` | TPM 2.0 not available |
| 3031 | `HSM_3031_TPM_OPERATION_FAILED` | TPM operation failed |
| 3040 | `HSM_3040_STRONGBOX_NOT_AVAILABLE` | Android StrongBox unavailable |
| 3041 | `HSM_3041_STRONGBOX_OPERATION_FAILED` | StrongBox operation failed |

### Storage Errors (4000-4999)

| Code | Enum | Description |
|------|------|-------------|
| 4001 | `STORAGE_4001_FILE_NOT_FOUND` | File not found |
| 4002 | `STORAGE_4002_PERMISSION_DENIED` | Cannot read/write file |
| 4003 | `STORAGE_4003_DISK_FULL` | No space left |
| 4004 | `STORAGE_4004_IO_ERROR` | Read/write failed |
| 4010 | `STORAGE_4010_DB_CONNECTION_FAILED` | Database connection error |
| 4011 | `STORAGE_4011_DB_QUERY_FAILED` | Query failed |
| 4012 | `STORAGE_4012_DB_TRANSACTION_FAILED` | Transaction failed |
| 4020 | `STORAGE_4020_CACHE_MISS` | Key not in cache |
| 4021 | `STORAGE_4021_CACHE_EVICTION_FAILED` | Cache eviction error |
| 4030 | `STORAGE_4030_SERIALIZATION_FAILED` | Cannot encode data |
| 4031 | `STORAGE_4031_DESERIALIZATION_FAILED` | Cannot decode data |

### Configuration Errors (5000-5999)

| Code | Enum | Description |
|------|------|-------------|
| 5001 | `CONFIG_5001_FILE_NOT_FOUND` | Config file not found |
| 5002 | `CONFIG_5002_PARSE_ERROR` | Invalid format |
| 5003 | `CONFIG_5003_VALIDATION_FAILED` | Invalid values |
| 5004 | `CONFIG_5004_MISSING_REQUIRED_FIELD` | Required field missing |
| 5005 | `CONFIG_5005_INVALID_VALUE` | Invalid value |
| 5010 | `CONFIG_5010_ENV_VAR_NOT_SET` | Environment variable not set |
| 5011 | `CONFIG_5011_ENV_VAR_INVALID` | Environment variable invalid |

### Service Discovery Errors (6000-6999)

| Code | Enum | Description |
|------|------|-------------|
| 6001 | `DISCOVERY_6001_SERVICE_NOT_FOUND` | No instances discovered |
| 6002 | `DISCOVERY_6002_REGISTRATION_FAILED` | Registration failed |
| 6003 | `DISCOVERY_6003_DEREGISTRATION_FAILED` | Deregistration failed |
| 6010 | `DISCOVERY_6010_HEALTH_CHECK_FAILED` | Service unhealthy |
| 6020 | `DISCOVERY_6020_TIMEOUT` | Discovery timeout |
| 6030 | `DISCOVERY_6030_NO_HEALTHY_INSTANCES` | No healthy instances |

### AI/ML Errors (7000-7999)

| Code | Enum | Description |
|------|------|-------------|
| 7001 | `AI_7001_MODEL_NOT_FOUND` | Model not found |
| 7002 | `AI_7002_MODEL_LOAD_FAILED` | Model loading failed |
| 7010 | `AI_7010_INFERENCE_FAILED` | Prediction error |
| 7011 | `AI_7011_INFERENCE_TIMEOUT` | Inference timeout |
| 7020 | `AI_7020_TRAINING_FAILED` | Training failed |
| 7030 | `AI_7030_INVALID_INPUT_SHAPE` | Tensor dimension mismatch |
| 7040 | `AI_7040_GPU_NOT_AVAILABLE` | CUDA error |

### System Errors (8000-8999)

| Code | Enum | Description |
|------|------|-------------|
| 8001 | `SYS_8001_OUT_OF_MEMORY` | Out of memory |
| 8002 | `SYS_8002_THREAD_POOL_EXHAUSTED` | Thread pool full |
| 8003 | `SYS_8003_RESOURCE_LIMIT_EXCEEDED` | Resource limit hit |
| 8010 | `SYS_8010_DEADLOCK_DETECTED` | Deadlock detected |
| 8020 | `SYS_8020_PANIC` | Unrecoverable error |
| 8030 | `SYS_8030_INVALID_STATE` | Invalid state transition |
| 8040 | `SYS_8040_INITIALIZATION_FAILED` | Init failed |
| 8041 | `SYS_8041_SHUTDOWN_TIMEOUT` | Shutdown timeout |

### Application Errors (9000-9999)

| Code | Enum | Description |
|------|------|-------------|
| 9001 | `APP_9001_INVALID_REQUEST` | Malformed input |
| 9002 | `APP_9002_RESOURCE_NOT_FOUND` | Business entity not found |
| 9003 | `APP_9003_RESOURCE_ALREADY_EXISTS` | Duplicate creation |
| 9004 | `APP_9004_OPERATION_NOT_PERMITTED` | Business rule violation |
| 9005 | `APP_9005_VALIDATION_FAILED` | Business validation failed |
| 9010 | `APP_9010_WORKFLOW_TIMEOUT` | Workflow timeout |
| 9011 | `APP_9011_WORKFLOW_CANCELED` | Workflow canceled |

---

## 🔍 **Monitoring and Alerting**

### Prometheus Metrics Example

```rust
use prometheus::{IntCounterVec, register_int_counter_vec};
use beardog_errors::{BearDogError, BearDogErrorCode};

lazy_static! {
    static ref ERROR_COUNTER: IntCounterVec = register_int_counter_vec!(
        "beardog_errors_total",
        "Total number of errors by code",
        &["code", "category"]
    ).unwrap();
}

fn track_error(error: &BearDogError) {
    if let Some(code) = error.code() {
        ERROR_COUNTER
            .with_label_values(&[
                &code.code().to_string(),
                code.category(),
            ])
            .inc();
    }
}
```

### Log Aggregation Example

```rust
use beardog_errors::{BearDogError, BearDogErrorCode};
use tracing::{error, warn};

fn log_error(err: &BearDogError) {
    match err.code() {
        Some(code) => {
            error!(
                error_code = code.code(),
                category = code.category(),
                message = %err,
                "Error occurred"
            );
        },
        None => {
            warn!(message = %err, "Uncoded error occurred");
        }
    }
}
```

---

## 🌍 **Internationalization**

Error codes enable language-independent error handling:

```rust
use beardog_errors::{BearDogError, BearDogErrorCode};

fn translate_error(error: &BearDogError, lang: &str) -> String {
    match (error.code(), lang) {
        (Some(&BearDogErrorCode::SEC_1001_UNAUTHORIZED), "es") => {
            "Acceso no autorizado".to_string()
        },
        (Some(&BearDogErrorCode::SEC_1001_UNAUTHORIZED), "fr") => {
            "Accès non autorisé".to_string()
        },
        (Some(code), _) => {
            // Fallback to English description
            code.description().to_string()
        },
        _ => error.to_string(),
    }
}
```

---

## 🏆 **Best Practices**

### ✅ DO:
- Use error codes for all production errors
- Log error codes for monitoring
- Use codes for user-facing error messages
- Track error code frequency
- Document new error codes

### ❌ DON'T:
- Reuse error codes for different errors
- Skip error codes for critical errors
- Change error code meanings
- Use error codes for debug logging

---

## 📈 **Benefits**

1. **Consistent Tracking**: Same error always has same code
2. **Machine-Readable**: Easy to parse and aggregate
3. **Language-Independent**: Codes work across locales
4. **Analytics-Friendly**: Easy to track error trends
5. **Debug-Friendly**: Codes pinpoint exact error type

---

## 🚀 **Future Enhancements**

### Planned Features:
- [ ] Error code analytics dashboard
- [ ] Automatic error code documentation
- [ ] Error code deprecation warnings
- [ ] Error code migration tools

---

**Implemented**: November 9, 2025  
**Status**: ✅ Production-Ready  
**Coverage**: 60+ error codes across 9 categories

