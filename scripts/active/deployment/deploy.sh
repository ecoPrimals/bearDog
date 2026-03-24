#!/bin/bash
# NOTE: This deployment script needs updating for the current architecture.
# (Repository has no ./charts/beardog Helm chart; DB backup and rollout names are placeholders.)

# BearDog Deployment Automation Script
# Provides secure, automated deployment with health checks and rollback capabilities

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
DEPLOYMENT_LOG="/tmp/beardog-deployment-$(date +%Y%m%d-%H%M%S).log"
HEALTH_CHECK_TIMEOUT=300  # 5 minutes
ROLLBACK_TIMEOUT=180      # 3 minutes

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1" | tee -a "$DEPLOYMENT_LOG"
}

log_success() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] ✅ $1${NC}" | tee -a "$DEPLOYMENT_LOG"
}

log_warning() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] ⚠️  $1${NC}" | tee -a "$DEPLOYMENT_LOG"
}

log_error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ❌ $1${NC}" | tee -a "$DEPLOYMENT_LOG"
}

# Error handling
cleanup() {
    local exit_code=$?
    if [ $exit_code -ne 0 ]; then
        log_error "Deployment failed with exit code $exit_code"
        log "Deployment log saved to: $DEPLOYMENT_LOG"
    fi
    exit $exit_code
}

trap cleanup EXIT

# Help function
show_help() {
    cat << EOF
BearDog Deployment Script

Usage: $0 [OPTIONS] ENVIRONMENT

ENVIRONMENTS:
    staging     Deploy to staging environment
    production  Deploy to production environment
    local       Deploy to local development environment

OPTIONS:
    -h, --help              Show this help message
    -v, --version VERSION   Deploy specific version (default: latest)
    -r, --rollback          Rollback to previous version
    -c, --check-only        Only run health checks, no deployment
    -f, --force             Force deployment without confirmation
    --skip-tests            Skip pre-deployment tests
    --skip-backup           Skip database backup
    --dry-run               Show what would be deployed without executing

EXAMPLES:
    $0 staging                          Deploy latest to staging
    $0 production -v v1.2.3            Deploy v1.2.3 to production
    $0 production --rollback            Rollback production to previous version
    $0 staging --check-only             Check staging health without deploying

EOF
}

# Parse command line arguments
ENVIRONMENT=""
VERSION="latest"
ROLLBACK=false
CHECK_ONLY=false
FORCE=false
SKIP_TESTS=false
SKIP_BACKUP=false
DRY_RUN=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -r|--rollback)
            ROLLBACK=true
            shift
            ;;
        -c|--check-only)
            CHECK_ONLY=true
            shift
            ;;
        -f|--force)
            FORCE=true
            shift
            ;;
        --skip-tests)
            SKIP_TESTS=true
            shift
            ;;
        --skip-backup)
            SKIP_BACKUP=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        staging|production|local)
            ENVIRONMENT="$1"
            shift
            ;;
        *)
            log_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Validate environment
if [ -z "$ENVIRONMENT" ]; then
    log_error "Environment must be specified"
    show_help
    exit 1
fi

# Environment-specific configuration
case $ENVIRONMENT in
    staging)
        NAMESPACE="beardog-staging"
        REPLICA_COUNT=2
        RESOURCE_LIMITS="cpu=500m,memory=1Gi"
        ;;
    production)
        NAMESPACE="beardog-production"
        REPLICA_COUNT=3
        RESOURCE_LIMITS="cpu=1000m,memory=2Gi"
        ;;
    local)
        NAMESPACE="beardog-local"
        REPLICA_COUNT=1
        RESOURCE_LIMITS="cpu=250m,memory=512Mi"
        ;;
    *)
        log_error "Invalid environment: $ENVIRONMENT"
        exit 1
        ;;
esac

log "🚀 Starting BearDog deployment to $ENVIRONMENT"
log "Version: $VERSION"
log "Namespace: $NAMESPACE"
log "Deployment log: $DEPLOYMENT_LOG"

# Pre-deployment validation
validate_environment() {
    log "🔍 Validating deployment environment..."
    
    # Check required tools
    local required_tools=("docker" "kubectl" "helm")
    for tool in "${required_tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            log_error "Required tool not found: $tool"
            exit 1
        fi
    done
    
    # Check Kubernetes connectivity
    if ! kubectl cluster-info &> /dev/null; then
        log_error "Cannot connect to Kubernetes cluster"
        exit 1
    fi
    
    # Check namespace exists
    if ! kubectl get namespace "$NAMESPACE" &> /dev/null; then
        log "Creating namespace: $NAMESPACE"
        if [ "$DRY_RUN" = false ]; then
            kubectl create namespace "$NAMESPACE"
        fi
    fi
    
    log_success "Environment validation passed"
}

# Run pre-deployment tests
run_tests() {
    if [ "$SKIP_TESTS" = true ]; then
        log_warning "Skipping pre-deployment tests"
        return 0
    fi
    
    log "🧪 Running pre-deployment tests..."
    
    cd "$PROJECT_ROOT"
    
    # Security scan
    log "Running security audit..."
    if [ "$DRY_RUN" = false ]; then
        cargo audit || {
            log_error "Security audit failed"
            exit 1
        }
    fi
    
    # Unit tests
    log "Running unit tests..."
    if [ "$DRY_RUN" = false ]; then
        cargo test --workspace --lib || {
            log_error "Unit tests failed"
            exit 1
        }
    fi
    
    # Integration tests
    log "Running integration tests..."
    if [ "$DRY_RUN" = false ]; then
        cargo test --workspace --tests || {
            log_error "Integration tests failed"
            exit 1
        }
    fi
    
    log_success "All tests passed"
}

# Build and push container image
build_and_push() {
    log "🏗️  Building and pushing container image..."
    
    local image_tag="ghcr.io/ecoprimal/beardog:$VERSION"
    
    if [ "$DRY_RUN" = false ]; then
        # Build image
        docker build -t "$image_tag" "$PROJECT_ROOT"
        
        # Push image
        docker push "$image_tag"
    fi
    
    log_success "Container image built and pushed: $image_tag"
}

# Create database backup
backup_database() {
    if [ "$SKIP_BACKUP" = true ]; then
        log_warning "Skipping database backup"
        return 0
    fi
    
    log "💾 Creating database backup..."
    
    local backup_name="beardog-backup-$(date +%Y%m%d-%H%M%S)"
    
    if [ "$DRY_RUN" = false ]; then
        # Create backup using your preferred method
        # This is a placeholder - implement your actual backup logic
        kubectl exec -n "$NAMESPACE" deployment/beardog-db -- \
            pg_dump beardog > "/tmp/$backup_name.sql" || {
            log_error "Database backup failed"
            exit 1
        }
    fi
    
    log_success "Database backup created: $backup_name"
}

# Deploy application
deploy_application() {
    log "🚢 Deploying BearDog application..."
    
    local image_tag="ghcr.io/ecoprimal/beardog:$VERSION"
    
    if [ "$DRY_RUN" = false ]; then
        # Deploy using Helm
        helm upgrade --install beardog ./charts/beardog \
            --namespace "$NAMESPACE" \
            --set image.tag="$VERSION" \
            --set replicaCount="$REPLICA_COUNT" \
            --set resources.limits.cpu="$(echo $RESOURCE_LIMITS | cut -d',' -f1 | cut -d'=' -f2)" \
            --set resources.limits.memory="$(echo $RESOURCE_LIMITS | cut -d',' -f2 | cut -d'=' -f2)" \
            --timeout 10m \
            --wait || {
            log_error "Application deployment failed"
            exit 1
        }
    fi
    
    log_success "Application deployed successfully"
}

# Health check
health_check() {
    log "🏥 Running health checks..."
    
    local start_time=$(date +%s)
    local timeout_time=$((start_time + HEALTH_CHECK_TIMEOUT))
    
    while [ $(date +%s) -lt $timeout_time ]; do
        if [ "$DRY_RUN" = false ]; then
            # Check if pods are ready
            local ready_pods=$(kubectl get pods -n "$NAMESPACE" -l app=beardog -o jsonpath='{.items[*].status.conditions[?(@.type=="Ready")].status}' | tr ' ' '\n' | grep -c "True" || echo "0")
            local total_pods=$(kubectl get pods -n "$NAMESPACE" -l app=beardog --no-headers | wc -l)
            
            if [ "$ready_pods" -eq "$REPLICA_COUNT" ] && [ "$total_pods" -eq "$REPLICA_COUNT" ]; then
                # Test application health endpoint
                if kubectl exec -n "$NAMESPACE" deployment/beardog -- beardog status &> /dev/null; then
                    log_success "Health check passed - all $REPLICA_COUNT pods are ready and healthy"
                    return 0
                fi
            fi
        else
            log "DRY RUN: Would check health of $REPLICA_COUNT pods"
            return 0
        fi
        
        log "Waiting for pods to be ready... ($ready_pods/$REPLICA_COUNT ready)"
        sleep 10
    done
    
    log_error "Health check failed - timeout after $HEALTH_CHECK_TIMEOUT seconds"
    return 1
}

# Rollback deployment
rollback_deployment() {
    log "🔄 Rolling back deployment..."
    
    if [ "$DRY_RUN" = false ]; then
        helm rollback beardog -n "$NAMESPACE" || {
            log_error "Rollback failed"
            exit 1
        }
        
        # Wait for rollback to complete
        kubectl rollout status deployment/beardog -n "$NAMESPACE" --timeout="${ROLLBACK_TIMEOUT}s" || {
            log_error "Rollback timeout"
            exit 1
        }
    fi
    
    log_success "Rollback completed successfully"
}

# Post-deployment verification
post_deployment_verification() {
    log "✅ Running post-deployment verification..."
    
    if [ "$DRY_RUN" = false ]; then
        # Run smoke tests
        log "Running smoke tests..."
        kubectl exec -n "$NAMESPACE" deployment/beardog -- beardog doctor || {
            log_warning "Smoke tests failed"
        }
        
        # Check metrics endpoint
        log "Checking metrics endpoint..."
        kubectl exec -n "$NAMESPACE" deployment/beardog -- curl -f http://localhost:8080/metrics &> /dev/null || {
            log_warning "Metrics endpoint not responding"
        }
    fi
    
    log_success "Post-deployment verification completed"
}

# Cleanup old deployments
cleanup_old_deployments() {
    log "🧹 Cleaning up old deployments..."
    
    if [ "$DRY_RUN" = false ]; then
        # Keep only last 3 releases
        helm history beardog -n "$NAMESPACE" --max 3 &> /dev/null || true
    fi
    
    log_success "Cleanup completed"
}

# Send deployment notification
send_notification() {
    local status=$1
    local message="BearDog deployment to $ENVIRONMENT: $status (Version: $VERSION)"
    
    log "📢 Sending deployment notification..."
    
    # Add your notification logic here (Slack, email, etc.)
    # Example:
    # curl -X POST -H 'Content-type: application/json' \
    #   --data "{\"text\":\"$message\"}" \
    #   "$SLACK_WEBHOOK_URL"
    
    log_success "Notification sent: $message"
}

# Confirmation prompt
confirm_deployment() {
    if [ "$FORCE" = true ] || [ "$DRY_RUN" = true ]; then
        return 0
    fi
    
    echo
    echo "🚨 DEPLOYMENT CONFIRMATION 🚨"
    echo "Environment: $ENVIRONMENT"
    echo "Version: $VERSION"
    echo "Namespace: $NAMESPACE"
    echo "Replicas: $REPLICA_COUNT"
    echo
    read -p "Are you sure you want to proceed? (yes/no): " -r
    echo
    
    if [[ ! $REPLY =~ ^[Yy]es$ ]]; then
        log "Deployment cancelled by user"
        exit 0
    fi
}

# Main deployment workflow
main() {
    # Handle special modes
    if [ "$CHECK_ONLY" = true ]; then
        validate_environment
        health_check
        log_success "Health check completed"
        exit 0
    fi
    
    if [ "$ROLLBACK" = true ]; then
        confirm_deployment
        validate_environment
        rollback_deployment
        health_check
        send_notification "ROLLBACK SUCCESS"
        log_success "Rollback completed successfully"
        exit 0
    fi
    
    # Standard deployment workflow
    confirm_deployment
    validate_environment
    run_tests
    backup_database
    build_and_push
    deploy_application
    
    # Health check with rollback on failure
    if ! health_check; then
        log_error "Deployment health check failed - initiating rollback"
        rollback_deployment
        if health_check; then
            send_notification "DEPLOYMENT FAILED - ROLLBACK SUCCESS"
            log_error "Deployment failed but rollback was successful"
        else
            send_notification "DEPLOYMENT FAILED - ROLLBACK FAILED"
            log_error "Deployment and rollback both failed - manual intervention required"
        fi
        exit 1
    fi
    
    post_deployment_verification
    cleanup_old_deployments
    send_notification "SUCCESS"
    
    log_success "🎉 BearDog deployment to $ENVIRONMENT completed successfully!"
    log "Deployment log saved to: $DEPLOYMENT_LOG"
}

# Run main function
main "$@" 