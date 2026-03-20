#!/bin/bash
# Test Runner by Domain
# Created: October 26, 2025

set -e

DOMAIN="${1:-all}"

echo "🧪 Running BearDog Tests by Domain: $DOMAIN"
echo "============================================="

case "$DOMAIN" in
    "core")
        echo "Running Core Domain Tests..."
        cargo test -p beardog-core
        cargo test --tests -- core
        ;;
    
    "security")
        echo "Running Security Domain Tests..."
        cargo test -p beardog-security
        cargo test -p beardog-crypto
        cargo test --tests -- security
        ;;
    
    "hsm")
        echo "Running HSM Domain Tests..."
        cargo test -p beardog-tunnel -- hsm
        cargo test --tests -- hsm
        ;;
    
    "networking")
        echo "Running Networking Domain Tests..."
        cargo test -p beardog-networking
        cargo test --tests -- network
        ;;
    
    "genetics")
        echo "Running Genetics Domain Tests..."
        cargo test -p beardog-genetics
        cargo test --tests -- genetic
        ;;
    
    "monitoring")
        echo "Running Monitoring Domain Tests..."
        cargo test -p beardog-monitoring
        cargo test --tests -- monitoring
        ;;
    
    "workflows")
        echo "Running Workflows Domain Tests..."
        cargo test -p beardog-workflows
        cargo test --tests -- workflow
        ;;
    
    "adapters")
        echo "Running Adapters Domain Tests..."
        cargo test -p beardog-adapters
        cargo test --tests -- adapter
        ;;
    
    "auth")
        echo "Running Auth Domain Tests..."
        cargo test -p beardog-auth
        cargo test --tests -- auth
        ;;
    
    "types")
        echo "Running Types Domain Tests..."
        cargo test -p beardog-types
        cargo test --tests -- types
        ;;
    
    "all")
        echo "Running All Domain Tests..."
        cargo test --workspace
        ;;
    
    *)
        echo "Unknown domain: $DOMAIN"
        echo ""
        echo "Available domains:"
        echo "  core        - Core orchestration"
        echo "  security    - Security subsystem"
        echo "  hsm         - HSM operations"
        echo "  networking  - Network operations"
        echo "  genetics    - Genetic evolution"
        echo "  monitoring  - Monitoring/observability"
        echo "  workflows   - Workflow orchestration"
        echo "  adapters    - Adapter system"
        echo "  auth        - Authentication/authorization"
        echo "  types       - Type system"
        echo "  all         - All domains (default)"
        exit 1
        ;;
esac

echo ""
echo "✅ Tests completed for domain: $DOMAIN"

