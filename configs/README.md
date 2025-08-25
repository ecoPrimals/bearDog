# BearDog Configuration Files

This directory contains configuration templates and examples for BearDog deployment.

## Configuration Files

### Production Configurations
- **`production-config.toml`** - Production deployment configuration with security hardening
- **`distributed_beardog_config.toml`** - Multi-node distributed deployment configuration

### Development Configurations  
- **`development-config.toml`** - Development environment with debug settings
- **`example-config.toml`** - Comprehensive example showing all available options

### Base Configurations
- **`beardog-config.toml`** - Base configuration template
- **`network-defaults.toml`** - Default network settings and security policies

## Usage

### Development
```bash
# Copy development config
cp configs/development-config.toml beardog-config.toml

# Or use environment variable
export BEARDOG_CONFIG=configs/development-config.toml
cargo run
```

### Production
```bash
# Copy and customize production config
cp configs/production-config.toml /etc/beardog/config.toml
# Edit /etc/beardog/config.toml for your environment

# Run with production config
beardog-server --config /etc/beardog/config.toml
```

### Docker
```bash
# Mount config directory
docker run -v $(pwd)/configs:/etc/beardog/configs beardog:latest
```

## Configuration Sections

### Core Settings
- **`[server]`** - HTTP server configuration
- **`[security]`** - Security policies and HSM settings  
- **`[database]`** - Database connection settings
- **`[logging]`** - Logging configuration

### Features
- **`[hsm]`** - Hardware Security Module configuration
- **`[threat_detection]`** - ML-powered threat analysis settings
- **`[compliance]`** - Compliance monitoring configuration
- **`[workflows]`** - Workflow engine settings

### Integration
- **`[adapters]`** - External system adapter configuration
- **`[monitoring]`** - Metrics and monitoring settings
- **`[federation]`** - Multi-node federation settings

## Environment Variables

All configuration values can be overridden with environment variables using the format:
```
BEARDOG_<SECTION>_<KEY>=value
```

Examples:
```bash
export BEARDOG_SERVER_PORT=8080
export BEARDOG_DATABASE_URL="postgresql://..."
export BEARDOG_SECURITY_HSM_ENABLED=true
```

## Validation

Configuration files are validated on startup. Use the validation command to check your config:

```bash
beardog-server --config your-config.toml --validate
```

## Security Notes

- **Never commit production secrets** to version control
- Use environment variables or external secret management for sensitive values
- Production configs should have restrictive file permissions (600)
- Regular security audits of configuration settings are recommended

---

For detailed configuration options, see the [Configuration Reference](../CONFIGURATION.md). 