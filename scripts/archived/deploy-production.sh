#!/usr/bin/env bash
# BearDog Production Deployment Script
# Usage: ./scripts/deploy-production.sh [version]

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
VERSION="${1:-$(git describe --tags --always)}"
DEPLOY_DIR="${DEPLOY_DIR:-/opt/beardog}"
BACKUP_DIR="${BACKUP_DIR:-/var/backups/beardog}"
SERVICE_NAME="beardog"
BINARY_NAME="beardog"

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check if running as root or with sudo
    if [[ $EUID -ne 0 ]] && ! sudo -n true 2>/dev/null; then
        log_error "This script must be run with sudo privileges"
        exit 1
    fi
    
    # Check if binary exists
    if [[ ! -f "target/release/$BINARY_NAME" ]]; then
        log_error "Binary not found at target/release/$BINARY_NAME"
        log_info "Run 'cargo build --release' first"
        exit 1
    fi
    
    # Check if deployment directory exists
    if [[ ! -d "$DEPLOY_DIR" ]]; then
        log_warn "Deploy directory doesn't exist, creating: $DEPLOY_DIR"
        sudo mkdir -p "$DEPLOY_DIR"
    fi
    
    log_success "Prerequisites check passed"
}

create_backup() {
    log_info "Creating backup of current deployment..."
    
    if [[ -f "$DEPLOY_DIR/$BINARY_NAME" ]]; then
        sudo mkdir -p "$BACKUP_DIR"
        BACKUP_FILE="$BACKUP_DIR/${BINARY_NAME}-$(date +%Y%m%d-%H%M%S).backup"
        sudo cp "$DEPLOY_DIR/$BINARY_NAME" "$BACKUP_FILE"
        sudo cp "$DEPLOY_DIR/beardog-config.toml" "$BACKUP_DIR/config-$(date +%Y%m%d-%H%M%S).backup" 2>/dev/null || true
        log_success "Backup created: $BACKUP_FILE"
    else
        log_warn "No existing deployment to backup"
    fi
}

stop_service() {
    log_info "Stopping $SERVICE_NAME service..."
    
    if sudo systemctl is-active --quiet "$SERVICE_NAME"; then
        sudo systemctl stop "$SERVICE_NAME"
        log_success "Service stopped"
    else
        log_warn "Service was not running"
    fi
    
    # Wait for service to fully stop
    sleep 2
}

deploy_binary() {
    log_info "Deploying binary (version: $VERSION)..."
    
    # Strip binary (remove debug symbols)
    strip target/release/$BINARY_NAME 2>/dev/null || true
    
    # Copy binary
    sudo cp target/release/$BINARY_NAME "$DEPLOY_DIR/$BINARY_NAME"
    sudo chmod +x "$DEPLOY_DIR/$BINARY_NAME"
    
    # Set ownership
    sudo chown beardog:beardog "$DEPLOY_DIR/$BINARY_NAME" 2>/dev/null || true
    
    # Create version file
    echo "$VERSION" | sudo tee "$DEPLOY_DIR/VERSION" > /dev/null
    
    log_success "Binary deployed"
}

deploy_config() {
    log_info "Deploying configuration..."
    
    if [[ -f "beardog-config.production.toml" ]]; then
        sudo cp beardog-config.production.toml "$DEPLOY_DIR/beardog-config.toml"
        sudo chown beardog:beardog "$DEPLOY_DIR/beardog-config.toml" 2>/dev/null || true
        sudo chmod 600 "$DEPLOY_DIR/beardog-config.toml"
        log_success "Configuration deployed"
    else
        log_warn "No production config found (beardog-config.production.toml)"
    fi
}

start_service() {
    log_info "Starting $SERVICE_NAME service..."
    
    sudo systemctl start "$SERVICE_NAME"
    sleep 3
    
    if sudo systemctl is-active --quiet "$SERVICE_NAME"; then
        log_success "Service started successfully"
    else
        log_error "Service failed to start"
        log_info "Check logs: sudo journalctl -u $SERVICE_NAME -n 50"
        return 1
    fi
}

verify_health() {
    log_info "Verifying service health..."
    
    local max_attempts=30
    local attempt=1
    local health_url="${BEARDOG_HEALTH_URL:-http://localhost:8080/health}"
    
    while [[ $attempt -le $max_attempts ]]; do
        if curl -sf "$health_url" > /dev/null 2>&1; then
            log_success "Health check passed"
            return 0
        fi
        
        log_info "Health check attempt $attempt/$max_attempts..."
        sleep 2
        ((attempt++))
    done
    
    log_error "Health check failed after $max_attempts attempts"
    return 1
}

run_smoke_tests() {
    log_info "Running smoke tests..."
    
    if [[ -f "scripts/smoke-test-production.sh" ]]; then
        if bash scripts/smoke-test-production.sh; then
            log_success "Smoke tests passed"
        else
            log_error "Smoke tests failed"
            return 1
        fi
    else
        log_warn "Smoke test script not found, skipping"
    fi
}

main() {
    echo ""
    echo "╔══════════════════════════════════════════════════════════╗"
    echo "║     🚀 BearDog Production Deployment 🚀                  ║"
    echo "╚══════════════════════════════════════════════════════════╝"
    echo ""
    echo "Version: $VERSION"
    echo "Deploy Directory: $DEPLOY_DIR"
    echo ""
    
    # Confirm deployment
    read -p "Proceed with deployment? (yes/no): " -r
    if [[ ! $REPLY =~ ^[Yy](es)?$ ]]; then
        log_warn "Deployment cancelled"
        exit 0
    fi
    
    # Run deployment steps
    check_prerequisites
    create_backup
    stop_service
    deploy_binary
    deploy_config
    start_service
    
    # Verify deployment
    if verify_health; then
        run_smoke_tests || log_warn "Smoke tests failed but service is running"
        
        echo ""
        echo "╔══════════════════════════════════════════════════════════╗"
        echo "║     ✅ DEPLOYMENT SUCCESSFUL ✅                          ║"
        echo "╚══════════════════════════════════════════════════════════╝"
        echo ""
        log_success "BearDog $VERSION is now running in production!"
        log_info "Monitor logs: sudo journalctl -u $SERVICE_NAME -f"
        log_info "Check status: sudo systemctl status $SERVICE_NAME"
        echo ""
    else
        log_error "Deployment verification failed!"
        log_warn "Consider rolling back with: ./scripts/rollback-deployment.sh"
        exit 1
    fi
}

# Handle script interruption
trap 'log_error "Deployment interrupted! Service may be in inconsistent state."' INT TERM

# Run main function
main "$@"
