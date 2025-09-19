#!/bin/bash
set -euo pipefail

# BearDog Production Deployment Script
# Automates secure deployment with comprehensive validation

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DEPLOYMENT_ENV="${DEPLOYMENT_ENV:-production}"
NAMESPACE="${NAMESPACE:-beardog-production}"
IMAGE_TAG="${IMAGE_TAG:-latest}"

echo "🚀 Starting BearDog Production Deployment"
echo "   Environment: $DEPLOYMENT_ENV"
echo "   Namespace: $NAMESPACE"
echo "   Image Tag: $IMAGE_TAG"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Pre-deployment validation
validate_prerequisites() {
    log_info "Validating deployment prerequisites..."
    
    # Check required tools
    command -v docker >/dev/null 2>&1 || log_error "Docker is required but not installed"
    command -v kubectl >/dev/null 2>&1 || log_error "kubectl is required but not installed"
    
    # Check environment variables
    [ -n "${BEARDOG_DATABASE_URL:-}" ] || log_error "BEARDOG_DATABASE_URL must be set"
    [ -n "${BEARDOG_SECRET_KEY:-}" ] || log_error "BEARDOG_SECRET_KEY must be set"
    
    # Validate Kubernetes context
    kubectl cluster-info >/dev/null 2>&1 || log_error "Cannot connect to Kubernetes cluster"
    
    log_info "Prerequisites validation passed ✅"
}

# Build production image
build_image() {
    log_info "Building production Docker image..."
    
    cd "$PROJECT_ROOT"
    
    # Build with security optimizations
    docker build \
        -f docker/Dockerfile.production \
        -t "beardog:${IMAGE_TAG}" \
        --build-arg RUST_VERSION=1.75 \
        --build-arg BUILDKIT_INLINE_CACHE=1 \
        --target production \
        .
    
    # Security scan
    if command -v trivy >/dev/null 2>&1; then
        log_info "Running security scan..."
        trivy image --exit-code 0 --severity HIGH,CRITICAL "beardog:${IMAGE_TAG}"
    else
        log_warn "Trivy not found, skipping security scan"
    fi
    
    log_info "Image build completed ✅"
}

# Deploy to Kubernetes
deploy_kubernetes() {
    log_info "Deploying to Kubernetes..."
    
    # Create namespace if it doesn't exist
    kubectl create namespace "$NAMESPACE" --dry-run=client -o yaml | kubectl apply -f -
    
    # Apply production configuration
    envsubst < "$PROJECT_ROOT/k8s/beardog-production.yaml" | kubectl apply -n "$NAMESPACE" -f -
    
    # Wait for deployment
    kubectl rollout status deployment/beardog -n "$NAMESPACE" --timeout=300s
    
    log_info "Kubernetes deployment completed ✅"
}

# Health checks
validate_deployment() {
    log_info "Validating deployment health..."
    
    # Wait for pods to be ready
    kubectl wait --for=condition=ready pod -l app=beardog -n "$NAMESPACE" --timeout=300s
    
    # Get service endpoint
    SERVICE_IP=$(kubectl get service beardog -n "$NAMESPACE" -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
    if [ -z "$SERVICE_IP" ]; then
        SERVICE_IP=$(kubectl get service beardog -n "$NAMESPACE" -o jsonpath='{.spec.clusterIP}')
    fi
    
    # Health check
    if curl -f -s "http://${SERVICE_IP}:8080/health" >/dev/null; then
        log_info "Health check passed ✅"
    else
        log_error "Health check failed ❌"
    fi
    
    # Performance validation
    log_info "Running performance validation..."
    kubectl exec -n "$NAMESPACE" deployment/beardog -- ./bin/beardog --benchmark --duration=30s
    
    log_info "Deployment validation completed ✅"
}

# Monitoring setup
setup_monitoring() {
    log_info "Setting up monitoring and alerting..."
    
    # Deploy monitoring stack if not exists
    if ! kubectl get namespace monitoring >/dev/null 2>&1; then
        log_info "Deploying monitoring infrastructure..."
        kubectl create namespace monitoring
        
        # Apply monitoring manifests (Prometheus, Grafana, etc.)
        kubectl apply -n monitoring -f "$PROJECT_ROOT/k8s/monitoring/"
    fi
    
    # Configure BearDog-specific monitoring
    envsubst < "$PROJECT_ROOT/k8s/beardog-monitoring.yaml" | kubectl apply -n monitoring -f -
    
    log_info "Monitoring setup completed ✅"
}

# Security hardening
apply_security_policies() {
    log_info "Applying security policies..."
    
    # Network policies
    kubectl apply -n "$NAMESPACE" -f "$PROJECT_ROOT/k8s/network-policies.yaml"
    
    # Pod security policies
    kubectl apply -n "$NAMESPACE" -f "$PROJECT_ROOT/k8s/pod-security-policies.yaml"
    
    # RBAC
    kubectl apply -n "$NAMESPACE" -f "$PROJECT_ROOT/k8s/rbac.yaml"
    
    log_info "Security policies applied ✅"
}

# Backup configuration
setup_backup() {
    log_info "Configuring backup systems..."
    
    # Create backup job
    envsubst < "$PROJECT_ROOT/k8s/backup-cronjob.yaml" | kubectl apply -n "$NAMESPACE" -f -
    
    log_info "Backup configuration completed ✅"
}

# Main deployment flow
main() {
    log_info "🐻 BearDog Production Deployment Starting..."
    
    validate_prerequisites
    build_image
    deploy_kubernetes
    apply_security_policies
    setup_monitoring
    setup_backup
    validate_deployment
    
    log_info "🎉 BearDog Production Deployment Completed Successfully!"
    log_info "   Service URL: http://${SERVICE_IP:-localhost}:8080"
    log_info "   Metrics: http://${SERVICE_IP:-localhost}:9090/metrics"
    log_info "   Namespace: $NAMESPACE"
    
    # Display next steps
    echo ""
    echo "📋 Next Steps:"
    echo "   1. Configure DNS pointing to ${SERVICE_IP:-<service-ip>}"
    echo "   2. Set up SSL certificates"
    echo "   3. Configure monitoring dashboards"
    echo "   4. Review security policies"
    echo "   5. Test disaster recovery procedures"
}

# Handle interrupts
trap 'log_error "Deployment interrupted"' INT TERM

# Run main function
main "$@" 