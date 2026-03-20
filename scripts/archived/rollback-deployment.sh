#!/usr/bin/env bash
# BearDog Production Rollback Script
# Usage: ./scripts/rollback-deployment.sh

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
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

find_latest_backup() {
    log_info "Finding latest backup..."
    
    LATEST_BACKUP=$(sudo find "$BACKUP_DIR" -name "${BINARY_NAME}-*.backup" -type f 2>/dev/null | sort -r | head -n 1)
    
    if [[ -z "$LATEST_BACKUP" ]]; then
        log_error "No backup found in $BACKUP_DIR"
        exit 1
    fi
    
    log_info "Latest backup: $LATEST_BACKUP"
    
    # Find corresponding config backup
    BACKUP_TIMESTAMP=$(basename "$LATEST_BACKUP" | sed -n 's/.*-\([0-9]\{8\}-[0-9]\{6\}\)\.backup/\1/p')
    CONFIG_BACKUP="$BACKUP_DIR/config-${BACKUP_TIMESTAMP}.backup"
    
    if [[ -f "$CONFIG_BACKUP" ]]; then
        log_info "Config backup: $CONFIG_BACKUP"
    else
        log_warn "No config backup found for timestamp $BACKUP_TIMESTAMP"
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
    
    sleep 2
}

restore_binary() {
    log_info "Restoring binary from backup..."
    
    sudo cp "$LATEST_BACKUP" "$DEPLOY_DIR/$BINARY_NAME"
    sudo chmod +x "$DEPLOY_DIR/$BINARY_NAME"
    sudo chown beardog:beardog "$DEPLOY_DIR/$BINARY_NAME" 2>/dev/null || true
    
    log_success "Binary restored"
}

restore_config() {
    if [[ -f "$CONFIG_BACKUP" ]]; then
        log_info "Restoring configuration from backup..."
        
        sudo cp "$CONFIG_BACKUP" "$DEPLOY_DIR/beardog-config.toml"
        sudo chmod 600 "$DEPLOY_DIR/beardog-config.toml"
        sudo chown beardog:beardog "$DEPLOY_DIR/beardog-config.toml" 2>/dev/null || true
        
        log_success "Configuration restored"
    else
        log_warn "No config backup to restore"
    fi
}

start_service() {
    log_info "Starting $SERVICE_NAME service..."
    
    sudo systemctl start "$SERVICE_NAME"
    sleep 3
    
    if sudo systemctl is-active --quiet "$SERVICE_NAME"; then
        log_success "Service started successfully"
    else
        log_error "Service failed to start after rollback"
        log_info "Check logs: sudo journalctl -u $SERVICE_NAME -n 50"
        return 1
    fi
}

verify_health() {
    log_info "Verifying service health after rollback..."
    
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

main() {
    echo ""
    echo "╔══════════════════════════════════════════════════════════╗"
    echo "║     🔄 BearDog Production Rollback 🔄                    ║"
    echo "╚══════════════════════════════════════════════════════════╝"
    echo ""
    echo "This will rollback to the previous deployment"
    echo ""
    
    # Check if running as root or with sudo
    if [[ $EUID -ne 0 ]] && ! sudo -n true 2>/dev/null; then
        log_error "This script must be run with sudo privileges"
        exit 1
    fi
    
    # Confirm rollback
    log_warn "Rolling back will restore the previous version"
    read -p "Proceed with rollback? (yes/no): " -r
    if [[ ! $REPLY =~ ^[Yy](es)?$ ]]; then
        log_warn "Rollback cancelled"
        exit 0
    fi
    
    # Run rollback steps
    find_latest_backup
    stop_service
    restore_binary
    restore_config
    start_service
    
    # Verify rollback
    if verify_health; then
        echo ""
        echo "╔══════════════════════════════════════════════════════════╗"
        echo "║     ✅ ROLLBACK SUCCESSFUL ✅                            ║"
        echo "╚══════════════════════════════════════════════════════════╝"
        echo ""
        log_success "BearDog has been rolled back to previous version"
        log_info "Monitor logs: sudo journalctl -u $SERVICE_NAME -f"
        echo ""
        log_warn "Don't forget to:"
        log_warn "  1. Document the reason for rollback"
        log_warn "  2. Create an incident report"
        log_warn "  3. Investigate the failed deployment"
        echo ""
    else
        log_error "Rollback verification failed!"
        log_error "Service may be in an inconsistent state"
        log_info "Contact the on-call engineer immediately"
        exit 1
    fi
}

# Handle script interruption
trap 'log_error "Rollback interrupted!"' INT TERM

# Run main function
main "$@"

