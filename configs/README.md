# BearDog Unified Configuration System

This directory contains the unified configuration system for BearDog deployment, eliminating configuration duplication through environment-driven configuration.

## 🎯 Unified Configuration Architecture

### Core Configuration
- **`beardog-config-template.toml`** - Single unified configuration template with environment variable substitution
- **`environments/development.env`** - Development environment variables
- **`environments/production.env`** - Production environment variables

### Legacy Configurations (Removed)
Legacy files (`development-config.toml`, `production-config.toml`, `distributed_beardog_config.toml`)
have been removed. Use `environments/development.env` and `environments/production.env` instead.

## Usage

### Development
```bash
# Source development environment
source configs/environments/development.env

# Use unified template
export BEARDOG_CONFIG=configs/beardog-config-template.toml
cargo run
```

### Production
```bash
# Source production environment (customize first!)
source configs/environments/production.env

# Set any additional environment-specific variables
export BEARDOG_HSM_PIN="your-secure-pin"
export BEARDOG_SMTP_SERVER="your-smtp-server"

# Use unified template
export BEARDOG_CONFIG=configs/beardog-config-template.toml
beardog server
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
export BEARDOG_DATABASE_URL="sqlite:///var/lib/beardog/state.db"
export BEARDOG_SECURITY_HSM_ENABLED=true
```

## Validation

Configuration files are validated when the process loads them (for example when starting the server with `BEARDOG_CONFIG` set). There is no separate `--validate` flag on the `beardog` binary; run your usual entrypoint (for example `beardog server` with `BEARDOG_CONFIG=your-config.toml`) and confirm startup succeeds.

## Security Notes

- **Never commit production secrets** to version control
- Use environment variables or external secret management for sensitive values
- Production configs should have restrictive file permissions (600)
- Regular security audits of configuration settings are recommended

---

For detailed configuration options, see `beardog-config-template.toml` and the environment files. 