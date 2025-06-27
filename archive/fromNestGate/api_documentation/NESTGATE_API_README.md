# NestGate API

The NestGate API provides a comprehensive HTTP/REST interface for managing ZFS storage systems with intelligent automation and AI-powered optimization.

## Features

- **Complete ZFS Management**: Pool, dataset, and snapshot operations
- **AI-Powered Optimization**: Intelligent tier recommendations and performance analytics
- **RESTful Design**: Standard HTTP methods and JSON responses
- **Comprehensive Testing**: Unit and integration test suites
- **Development Server**: Ready-to-use development environment
- **CORS Support**: Cross-origin request handling
- **Request Tracing**: Built-in request/response logging

## Quick Start

### Development Server

Run the development server with mock ZFS integration:

```bash
cargo run --example dev_server
```

The server will start on `http://localhost:3000` with full API documentation printed to the console.

### Integration Example

```rust
use std::sync::Arc;
use nestgate_api::{Config, serve_with_zfs};
use nestgate_zfs::{ZfsManager, config::ZfsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize ZFS manager
    let zfs_config = ZfsConfig::default();
    let zfs_manager = Arc::new(ZfsManager::new(zfs_config).await?);
    
    // Configure API server
    let config = Config {
        bind_addr: "0.0.0.0:3000".to_string(),
        enable_zfs_api: true,
        ..Default::default()
    };
    
    // Start server
    serve_with_zfs(config, zfs_manager).await?;
    Ok(())
}
```

## API Endpoints

### Health & Status

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Basic health check |
| GET | `/api/v1/status` | System status |
| GET | `/api/v1/zfs/health` | ZFS health status |
| GET | `/api/v1/zfs/status` | ZFS system status |

### Pool Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/zfs/pools` | List all pools |
| POST | `/api/v1/zfs/pools` | Create new pool |
| GET | `/api/v1/zfs/pools/{name}` | Get pool information |
| DELETE | `/api/v1/zfs/pools/{name}` | Destroy pool |
| GET | `/api/v1/zfs/pools/{name}/status` | Get detailed pool status |
| POST | `/api/v1/zfs/pools/{name}/scrub` | Start pool scrub operation |

### Dataset Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/zfs/datasets` | List all datasets |
| POST | `/api/v1/zfs/datasets` | Create new dataset |
| GET | `/api/v1/zfs/datasets/{name}` | Get dataset information |
| DELETE | `/api/v1/zfs/datasets/{name}` | Destroy dataset |
| GET | `/api/v1/zfs/datasets/{name}/properties` | Get dataset properties |
| PUT | `/api/v1/zfs/datasets/{name}/properties` | Set dataset properties |

### Snapshot Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/zfs/datasets/{name}/snapshots` | List dataset snapshots |
| POST | `/api/v1/zfs/datasets/{name}/snapshots` | Create snapshot |
| DELETE | `/api/v1/zfs/datasets/{name}/snapshots/{snap}` | Delete snapshot |

### AI & Optimization

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/zfs/ai/tier-prediction` | Get AI tier recommendation |
| GET | `/api/v1/zfs/optimization/analytics` | Performance analytics |
| POST | `/api/v1/zfs/optimization/trigger` | Trigger optimization |

## Request/Response Examples

### Create Pool

**Request:**
```bash
curl -X POST http://localhost:3000/api/v1/zfs/pools \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "storage_pool",
    "devices": ["/dev/sda", "/dev/sdb"],
    "config": {
      "raid_level": "mirror",
      "compression": "lz4",
      "dedup": false,
      "encryption": true
    }
  }'
```

**Response:**
```json
{
  "name": "storage_pool",
  "size": 2000000000000,
  "allocated": 0,
  "free": 2000000000000,
  "health": "ONLINE",
  "fragmentation": 0,
  "compression_ratio": 1.0
}
```

### Create Dataset

**Request:**
```bash
curl -X POST http://localhost:3000/api/v1/zfs/datasets \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "documents",
    "parent": "storage_pool",
    "tier": "Warm",
    "properties": {
      "compression": "zstd",
      "recordsize": "128K"
    }
  }'
```

**Response:**
```json
{
  "name": "storage_pool/documents",
  "mountpoint": "/storage_pool/documents",
  "used": 0,
  "available": 2000000000000,
  "tier": "Warm",
  "compression": "zstd",
  "recordsize": "128K"
}
```

### AI Tier Prediction

**Request:**
```bash
curl -X POST http://localhost:3000/api/v1/zfs/ai/tier-prediction \
  -H 'Content-Type: application/json' \
  -d '{
    "file_path": "/storage_pool/documents/report.pdf"
  }'
```

**Response:**
```json
{
  "file_path": "/storage_pool/documents/report.pdf",
  "predicted_tier": "Cold",
  "current_tier": "Warm",
  "confidence": 0.89,
  "reasoning": "Low access frequency, large file size",
  "expected_improvement": 25.5,
  "timestamp": "2024-01-26T10:30:45Z"
}
```

## Data Models

### Storage Tiers

```rust
pub enum StorageTier {
    Hot,    // High-performance, frequently accessed
    Warm,   // Balanced performance and storage
    Cold,   // Long-term storage, infrequently accessed
    Cache,  // Ultra-fast cache layer
}
```

### Pool Configuration

```rust
pub struct PoolConfig {
    pub raid_level: Option<String>,    // mirror, raidz1, raidz2, raidz3
    pub compression: Option<String>,   // lz4, zstd, gzip, off
    pub dedup: Option<bool>,          // Enable deduplication
    pub encryption: Option<bool>,     // Enable encryption
}
```

### Dataset Properties

Common ZFS properties that can be configured:

- `compression`: Compression algorithm (lz4, zstd, gzip, off)
- `recordsize`: Record size (512, 1K, 2K, 4K, 8K, 16K, 32K, 64K, 128K, 256K, 512K, 1M)
- `quota`: Dataset quota (e.g., "100G", "1T")
- `reservation`: Space reservation (e.g., "50G")
- `readonly`: Read-only mode (on/off)
- `atime`: Access time updates (on/off)
- `relatime`: Relative access time (on/off)

## Error Handling

All API endpoints return consistent error responses:

```json
{
  "status": "error",
  "data": null,
  "message": "Detailed error description",
  "timestamp": "2024-01-26T10:30:45Z"
}
```

Common HTTP status codes:
- `200 OK`: Successful operation
- `201 Created`: Resource created successfully
- `204 No Content`: Successful operation with no response body
- `400 Bad Request`: Invalid request parameters
- `404 Not Found`: Resource not found
- `409 Conflict`: Resource already exists
- `500 Internal Server Error`: Server-side error

## Testing

### Run All Tests

```bash
cargo test
```

### Run Integration Tests

```bash
cargo test --test zfs_api_tests
```

### Run Unit Tests

```bash
cargo test --test unit_tests
```

### Test Coverage

Generate test coverage report:

```bash
cargo tarpaulin --out Html
```

## Configuration

### API Server Configuration

```rust
pub struct Config {
    pub bind_addr: String,           // Server bind address
    pub cors: Option<CorsLayer>,     // CORS configuration
    pub enable_zfs_api: bool,        // Enable ZFS endpoints
    pub request_timeout: u64,        // Request timeout (seconds)
    pub max_body_size: usize,        // Max request body size (bytes)
}
```

### Environment Variables

- `RUST_LOG`: Set logging level (e.g., `debug`, `info`, `warn`, `error`)
- `NESTGATE_API_BIND`: Override default bind address
- `NESTGATE_ZFS_MOCK`: Enable ZFS mock mode for testing

## Development

### Prerequisites

- Rust 1.70+
- ZFS utilities (for non-mock mode)
- Development dependencies: `axum-test`, `tokio-test`, `mockall`

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test --all-features
```

### Linting

```bash
cargo clippy --all-targets --all-features
cargo fmt --all --check
```

## Architecture

The API is built using:

- **Axum**: Modern, fast web framework
- **Tower**: Middleware and service abstractions
- **Serde**: JSON serialization/deserialization
- **Tokio**: Async runtime
- **Tracing**: Structured logging

### Key Components

- `handlers/`: HTTP request handlers
- `routes.rs`: Route definitions and configuration
- `models.rs`: Data models and serialization
- `lib.rs`: Main library interface

## Performance

### Benchmarks

- API response time: < 100ms for simple operations
- ZFS operations: Optimized for concurrent access
- Memory usage: Efficient Arc-based state sharing
- Throughput: 1000+ requests/second (typical hardware)

### Optimization Features

- Connection pooling
- Request/response compression
- Async I/O throughout
- Zero-copy JSON parsing where possible

## Security

### Features

- CORS protection
- Request size limits
- Input validation
- Error message sanitization
- Structured logging (no sensitive data)

### Best Practices

- Run behind reverse proxy (nginx, traefik)
- Use TLS/SSL for production
- Implement authentication middleware
- Regular security updates

## Monitoring

### Metrics

The API provides built-in metrics for:
- Request/response times
- Error rates
- ZFS operation statistics
- AI prediction accuracy
- Resource utilization

### Logging

Structured logging with tracing:
```rust
use tracing::{info, warn, error, debug};

// Request tracing
info!("Processing request", method = "POST", path = "/api/v1/zfs/pools");

// Error logging
error!("ZFS operation failed", pool = "storage_pool", error = %e);
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use meaningful variable names
- Add documentation for public APIs
- Include examples in documentation

## License

See the main NestGate repository for license information. 