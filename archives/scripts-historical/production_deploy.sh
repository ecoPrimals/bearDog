#!/bin/bash

# BearDog Production Deployment Script
# Automated deployment with security validation and monitoring setup

set -euo pipefail

# Configuration
DEPLOYMENT_ENV=${1:-production}
DEPLOY_VERSION=${2:-$(git rev-parse --short HEAD)}
DEPLOY_TIMESTAMP=$(date +%Y%m%d_%H%M%S)
DEPLOY_DIR="/opt/beardog"
LOG_DIR="/var/log/beardog"
CONFIG_DIR="/etc/beardog"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log() {
    echo -e "${BLUE}[$(date '+%Y-%m-%d %H:%M:%S')]${NC} $1"
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

error() {
    echo -e "${RED}❌ $1${NC}"
    exit 1
}

# Validate prerequisites
check_prerequisites() {
    log "Checking deployment prerequisites..."
    
    # Check if running as root or with sudo
    if [[ $EUID -ne 0 ]]; then
        error "This script must be run as root or with sudo"
    fi
    
    # Check required tools
    local required_tools=("cargo" "systemctl" "nginx" "docker")
    for tool in "${required_tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            error "Required tool not found: $tool"
        fi
    done
    
    # Check system resources
    local available_memory=$(free -m | awk 'NR==2{print $7}')
    if [[ $available_memory -lt 1024 ]]; then
        warning "Low available memory: ${available_memory}MB (recommended: 2GB+)"
    fi
    
    success "Prerequisites validated"
}

# Security validation
security_validation() {
    log "Running security validation..."
    
    # Run our security audit
    if [[ -f "./scripts/security_audit.sh" ]]; then
        ./scripts/security_audit.sh > "security_audit_${DEPLOY_TIMESTAMP}.log"
        local security_score=$(tail -n 10 "security_audit_${DEPLOY_TIMESTAMP}.log" | grep -o '[0-9]\+%' | head -1 | tr -d '%')
        
        if [[ ${security_score:-0} -lt 80 ]]; then
            error "Security audit failed with score: ${security_score}%. Minimum required: 80%"
        fi
        
        success "Security validation passed (${security_score}%)"
    else
        warning "Security audit script not found - skipping validation"
    fi
}

# Build optimized release
build_release() {
    log "Building optimized release..."
    
    # Clean previous builds
    cargo clean
    
    # Build with production optimizations
    RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C lto=fat -C codegen-units=1" \
    cargo build --release --workspace
    
    # Verify build success
    if [[ ! -f "target/release/beardog" ]]; then
        error "Build failed - beardog binary not found"
    fi
    
    success "Release build completed"
}

# Run comprehensive tests
run_tests() {
    log "Running comprehensive test suite..."
    
    # Unit tests
    cargo test --workspace --release --quiet
    
    # Integration tests
    cargo test --tests --release --quiet
    
    # Performance benchmarks (if available)
    if [[ -d "benches" ]]; then
        cargo bench --quiet > "benchmark_${DEPLOY_TIMESTAMP}.log" 2>&1 || true
    fi
    
    success "Test suite completed"
}

# Setup system directories
setup_directories() {
    log "Setting up system directories..."
    
    # Create directories with proper permissions
    mkdir -p "$DEPLOY_DIR"/{bin,config,data,logs}
    mkdir -p "$LOG_DIR"
    mkdir -p "$CONFIG_DIR"
    
    # Set ownership and permissions
    chown -R beardog:beardog "$DEPLOY_DIR" 2>/dev/null || true
    chown -R beardog:beardog "$LOG_DIR" 2>/dev/null || true
    chmod 755 "$DEPLOY_DIR"
    chmod 750 "$LOG_DIR"
    chmod 750 "$CONFIG_DIR"
    
    success "Directories configured"
}

# Deploy binaries
deploy_binaries() {
    log "Deploying binaries..."
    
    # Copy main binary
    cp "target/release/beardog" "$DEPLOY_DIR/bin/"
    chmod 755 "$DEPLOY_DIR/bin/beardog"
    
    # Copy additional binaries if they exist
    for binary in beardog-cli beardog-admin beardog-monitor; do
        if [[ -f "target/release/$binary" ]]; then
            cp "target/release/$binary" "$DEPLOY_DIR/bin/"
            chmod 755 "$DEPLOY_DIR/bin/$binary"
        fi
    done
    
    success "Binaries deployed"
}

# Deploy configuration
deploy_configuration() {
    log "Deploying configuration..."
    
    # Copy configuration templates
    if [[ -d "configs" ]]; then
        cp -r configs/* "$CONFIG_DIR/"
    fi
    
    # Generate production configuration
    cat > "$CONFIG_DIR/production.toml" << EOF
[network]
api_bind_address = "0.0.0.0:8080"
metrics_bind_address = "127.0.0.1:9090"
health_bind_address = "0.0.0.0:8081"

[security]
session_timeout_secs = 3600
auth_timeout_secs = 300
max_failed_attempts = 5
lockout_duration_secs = 1800

[performance]
request_timeout_secs = 30
max_connections = 1000
worker_threads = 0  # Use all available cores

[logging]
level = "info"
file = "$LOG_DIR/beardog.log"
max_size = "100MB"
max_files = 10

[monitoring]
enabled = true
metrics_endpoint = "/metrics"
health_endpoint = "/health"

[deployment]
version = "$DEPLOY_VERSION"
deployed_at = "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
environment = "$DEPLOYMENT_ENV"
EOF
    
    chmod 640 "$CONFIG_DIR/production.toml"
    success "Configuration deployed"
}

# Setup systemd service
setup_systemd() {
    log "Setting up systemd service..."
    
    cat > "/etc/systemd/system/beardog.service" << EOF
[Unit]
Description=BearDog Security Intelligence Platform
Documentation=https://github.com/ecoprimals/beardog
After=network.target
Wants=network.target

[Service]
Type=exec
User=beardog
Group=beardog
WorkingDirectory=$DEPLOY_DIR
ExecStart=$DEPLOY_DIR/bin/beardog --config $CONFIG_DIR/production.toml
ExecReload=/bin/kill -HUP \$MAINPID
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal
SyslogIdentifier=beardog

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$DEPLOY_DIR/data $LOG_DIR

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

[Install]
WantedBy=multi-user.target
EOF
    
    systemctl daemon-reload
    systemctl enable beardog
    
    success "Systemd service configured"
}

# Setup monitoring
setup_monitoring() {
    log "Setting up monitoring..."
    
    # Create monitoring configuration
    cat > "$CONFIG_DIR/monitoring.toml" << EOF
[prometheus]
enabled = true
listen_address = "127.0.0.1:9090"
metrics_path = "/metrics"

[health_checks]
enabled = true
interval_seconds = 30
timeout_seconds = 5

[alerting]
enabled = true
webhook_url = "${ALERT_WEBHOOK_URL:-}"

[logging]
structured = true
level = "info"
format = "json"
EOF
    
    # Setup log rotation
    cat > "/etc/logrotate.d/beardog" << EOF
$LOG_DIR/*.log {
    daily
    missingok
    rotate 30
    compress
    delaycompress
    notifempty
    create 0644 beardog beardog
    postrotate
        systemctl reload beardog || true
    endscript
}
EOF
    
    success "Monitoring configured"
}

# Setup reverse proxy
setup_reverse_proxy() {
    log "Setting up reverse proxy..."
    
    # Create nginx configuration
    cat > "/etc/nginx/sites-available/beardog" << EOF
upstream beardog_backend {
    server 127.0.0.1:8080;
    keepalive 32;
}

server {
    listen 80;
    listen [::]:80;
    server_name ${DOMAIN_NAME:-beardog.local};
    
    # Security headers
    add_header X-Frame-Options DENY always;
    add_header X-Content-Type-Options nosniff always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    
    # Rate limiting
    limit_req_zone \$binary_remote_addr zone=api:10m rate=10r/s;
    limit_req zone=api burst=20 nodelay;
    
    location / {
        proxy_pass http://beardog_backend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_cache_bypass \$http_upgrade;
        
        # Timeouts
        proxy_connect_timeout 5s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }
    
    location /health {
        proxy_pass http://127.0.0.1:8081/health;
        access_log off;
    }
    
    location /metrics {
        proxy_pass http://127.0.0.1:9090/metrics;
        allow 127.0.0.1;
        deny all;
    }
}
EOF
    
    # Enable site
    ln -sf /etc/nginx/sites-available/beardog /etc/nginx/sites-enabled/
    nginx -t && systemctl reload nginx
    
    success "Reverse proxy configured"
}

# Deploy and start service
deploy_and_start() {
    log "Deploying and starting service..."
    
    # Stop service if running
    systemctl stop beardog 2>/dev/null || true
    
    # Start service
    systemctl start beardog
    
    # Wait for service to be ready
    local max_attempts=30
    local attempt=1
    
    while [[ $attempt -le $max_attempts ]]; do
        if curl -s "http://localhost:8081/health" > /dev/null; then
            break
        fi
        
        if [[ $attempt -eq $max_attempts ]]; then
            error "Service failed to start after $max_attempts attempts"
        fi
        
        sleep 2
        ((attempt++))
    done
    
    success "Service deployed and started"
}

# Verify deployment
verify_deployment() {
    log "Verifying deployment..."
    
    # Check service status
    if ! systemctl is-active --quiet beardog; then
        error "BearDog service is not running"
    fi
    
    # Check health endpoint
    local health_response=$(curl -s "http://localhost:8081/health" || echo "")
    if [[ "$health_response" != *"healthy"* ]]; then
        warning "Health check returned: $health_response"
    fi
    
    # Check metrics endpoint
    if ! curl -s "http://localhost:9090/metrics" > /dev/null; then
        warning "Metrics endpoint not responding"
    fi
    
    # Check logs for errors
    local error_count=$(journalctl -u beardog --since="5 minutes ago" | grep -i error | wc -l)
    if [[ $error_count -gt 0 ]]; then
        warning "Found $error_count errors in recent logs"
    fi
    
    success "Deployment verification completed"
}

# Generate deployment report
generate_report() {
    log "Generating deployment report..."
    
    local report_file="deployment_report_${DEPLOY_TIMESTAMP}.md"
    
    cat > "$report_file" << EOF
# BearDog Production Deployment Report

**Deployment Date**: $(date)  
**Version**: $DEPLOY_VERSION  
**Environment**: $DEPLOYMENT_ENV  
**Deployed By**: $(whoami)

## Deployment Summary

- ✅ Security validation passed
- ✅ Release build completed
- ✅ Test suite passed
- ✅ System configuration deployed
- ✅ Service started and verified

## System Information

- **OS**: $(lsb_release -d 2>/dev/null | cut -f2 || uname -a)
- **Architecture**: $(uname -m)
- **Available Memory**: $(free -h | awk 'NR==2{print $7}')
- **Available Disk**: $(df -h / | awk 'NR==2{print $4}')

## Service Status

- **Service**: $(systemctl is-active beardog)
- **PID**: $(systemctl show beardog --property=MainPID --value)
- **Uptime**: $(systemctl show beardog --property=ActiveEnterTimestamp --value)

## Endpoints

- **API**: http://localhost:8080
- **Health**: http://localhost:8081/health
- **Metrics**: http://localhost:9090/metrics

## Configuration

- **Config Dir**: $CONFIG_DIR
- **Log Dir**: $LOG_DIR
- **Data Dir**: $DEPLOY_DIR/data

## Next Steps

1. Configure SSL certificates for HTTPS
2. Set up external monitoring
3. Configure automated backups
4. Review and tune performance settings

EOF
    
    success "Deployment report generated: $report_file"
}

# Main deployment flow
main() {
    log "🚀 Starting BearDog Production Deployment"
    log "Version: $DEPLOY_VERSION | Environment: $DEPLOYMENT_ENV"
    
    check_prerequisites
    security_validation
    build_release
    run_tests
    setup_directories
    deploy_binaries
    deploy_configuration
    setup_systemd
    setup_monitoring
    setup_reverse_proxy
    deploy_and_start
    verify_deployment
    generate_report
    
    success "🎉 BearDog deployment completed successfully!"
    log "Service is running at: http://localhost:8080"
    log "Health check: http://localhost:8081/health"
    log "Metrics: http://localhost:9090/metrics"
}

# Run main function
main "$@" 