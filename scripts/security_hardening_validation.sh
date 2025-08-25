#!/bin/bash

# 🔒 BearDog Security Hardening Validation Suite
#
# Comprehensive security validation for production deployment

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
VALIDATION_REPORT="security_validation_$(date +%Y%m%d_%H%M%S).md"
BEARDOG_CONFIG_DIR="/opt/beardog/config"
BEARDOG_DATA_DIR="/opt/beardog/data"

echo -e "${BLUE}🔒 BearDog Security Hardening Validation Suite${NC}"
echo "=================================================="

# Tracking variables
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
WARNING_CHECKS=0

print_status() {
    echo -e "${GREEN}✅ $1${NC}"
    ((PASSED_CHECKS++))
    ((TOTAL_CHECKS++))
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
    ((WARNING_CHECKS++))
    ((TOTAL_CHECKS++))
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
    ((FAILED_CHECKS++))
    ((TOTAL_CHECKS++))
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Initialize validation report
init_validation_report() {
    cat > "$VALIDATION_REPORT" << EOF
# 🔒 BearDog Security Hardening Validation Report

**Date**: $(date)
**Validator**: Security Hardening Validation Suite
**Environment**: $(uname -a)

## 🎯 Executive Summary

This report provides comprehensive validation of BearDog's security hardening for production deployment.

---

## 🔍 Security Validation Results

EOF
}

# Function to validate system hardening
validate_system_hardening() {
    print_info "Validating system-level security hardening..."
    
    echo "### 🖥️  System Hardening Validation" >> "$VALIDATION_REPORT"
    echo "" >> "$VALIDATION_REPORT"
    
    # Check if running as non-root user
    if [[ $EUID -eq 0 ]]; then
        print_error "BearDog should not run as root user"
        echo "- ❌ **Root User Check**: BearDog running as root (SECURITY RISK)" >> "$VALIDATION_REPORT"
    else
        print_status "BearDog not running as root user"
        echo "- ✅ **Root User Check**: BearDog not running as root" >> "$VALIDATION_REPORT"
    fi
    
    # Check firewall status
    if command -v ufw &> /dev/null; then
        if ufw status | grep -q "Status: active"; then
            print_status "UFW firewall is active"
            echo "- ✅ **Firewall Status**: UFW firewall active and configured" >> "$VALIDATION_REPORT"
        else
            print_error "UFW firewall is not active"
            echo "- ❌ **Firewall Status**: UFW firewall not active" >> "$VALIDATION_REPORT"
        fi
    else
        print_warning "UFW not installed - using alternative firewall"
        echo "- ⚠️  **Firewall Status**: UFW not found, verify alternative firewall" >> "$VALIDATION_REPORT"
    fi
    
    # Check SELinux/AppArmor
    if command -v getenforce &> /dev/null; then
        selinux_status=$(getenforce 2>/dev/null || echo "Not available")
        if [[ "$selinux_status" == "Enforcing" ]]; then
            print_status "SELinux is enforcing"
            echo "- ✅ **MAC System**: SELinux enforcing mode active" >> "$VALIDATION_REPORT"
        else
            print_warning "SELinux not in enforcing mode: $selinux_status"
            echo "- ⚠️  **MAC System**: SELinux status: $selinux_status" >> "$VALIDATION_REPORT"
        fi
    elif command -v aa-status &> /dev/null; then
        if aa-status --enabled &> /dev/null; then
            print_status "AppArmor is active"
            echo "- ✅ **MAC System**: AppArmor active" >> "$VALIDATION_REPORT"
        else
            print_warning "AppArmor not active"
            echo "- ⚠️  **MAC System**: AppArmor not active" >> "$VALIDATION_REPORT"
        fi
    else
        print_warning "No Mandatory Access Control system detected"
        echo "- ⚠️  **MAC System**: No SELinux or AppArmor detected" >> "$VALIDATION_REPORT"
    fi
    
    echo "" >> "$VALIDATION_REPORT"
}

# Function to validate TLS/encryption configuration
validate_encryption_config() {
    print_info "Validating TLS and encryption configuration..."
    
    echo "### 🔐 Encryption Configuration Validation" >> "$VALIDATION_REPORT"
    echo "" >> "$VALIDATION_REPORT"
    
    # Check TLS certificate files
    if [[ -f "$BEARDOG_CONFIG_DIR/../certs/beardog.crt" ]]; then
        print_status "TLS certificate file found"
        echo "- ✅ **TLS Certificate**: Certificate file present" >> "$VALIDATION_REPORT"
        
        # Validate certificate
        if openssl x509 -in "$BEARDOG_CONFIG_DIR/../certs/beardog.crt" -text -noout &> /dev/null; then
            print_status "TLS certificate is valid"
            echo "- ✅ **Certificate Validity**: Certificate is valid" >> "$VALIDATION_REPORT"
            
            # Check certificate expiration
            cert_expiry=$(openssl x509 -in "$BEARDOG_CONFIG_DIR/../certs/beardog.crt" -enddate -noout | cut -d= -f2)
            cert_expiry_epoch=$(date -d "$cert_expiry" +%s 2>/dev/null || echo "0")
            current_epoch=$(date +%s)
            days_until_expiry=$(( (cert_expiry_epoch - current_epoch) / 86400 ))
            
            if [[ $days_until_expiry -gt 30 ]]; then
                print_status "TLS certificate valid for $days_until_expiry days"
                echo "- ✅ **Certificate Expiry**: Valid for $days_until_expiry days" >> "$VALIDATION_REPORT"
            elif [[ $days_until_expiry -gt 0 ]]; then
                print_warning "TLS certificate expires in $days_until_expiry days"
                echo "- ⚠️  **Certificate Expiry**: Expires in $days_until_expiry days (renewal needed)" >> "$VALIDATION_REPORT"
            else
                print_error "TLS certificate has expired"
                echo "- ❌ **Certificate Expiry**: Certificate has expired" >> "$VALIDATION_REPORT"
            fi
        else
            print_error "TLS certificate is invalid"
            echo "- ❌ **Certificate Validity**: Certificate is invalid or corrupted" >> "$VALIDATION_REPORT"
        fi
    else
        print_error "TLS certificate file not found"
        echo "- ❌ **TLS Certificate**: Certificate file not found" >> "$VALIDATION_REPORT"
    fi
    
    # Check private key file
    if [[ -f "$BEARDOG_CONFIG_DIR/../certs/beardog.key" ]]; then
        print_status "TLS private key file found"
        echo "- ✅ **TLS Private Key**: Private key file present" >> "$VALIDATION_REPORT"
        
        # Check key permissions
        key_perms=$(stat -c "%a" "$BEARDOG_CONFIG_DIR/../certs/beardog.key" 2>/dev/null || echo "000")
        if [[ "$key_perms" == "600" || "$key_perms" == "400" ]]; then
            print_status "TLS private key has secure permissions ($key_perms)"
            echo "- ✅ **Key Permissions**: Secure permissions ($key_perms)" >> "$VALIDATION_REPORT"
        else
            print_error "TLS private key has insecure permissions ($key_perms)"
            echo "- ❌ **Key Permissions**: Insecure permissions ($key_perms) - should be 600" >> "$VALIDATION_REPORT"
        fi
    else
        print_error "TLS private key file not found"
        echo "- ❌ **TLS Private Key**: Private key file not found" >> "$VALIDATION_REPORT"
    fi
    
    echo "" >> "$VALIDATION_REPORT"
}

# Function to validate HSM configuration
validate_hsm_config() {
    print_info "Validating HSM security configuration..."
    
    echo "### 🔧 HSM Configuration Validation" >> "$VALIDATION_REPORT"
    echo "" >> "$VALIDATION_REPORT"
    
    # Check HSM configuration file
    if [[ -f "$BEARDOG_CONFIG_DIR/hsm.conf" ]]; then
        print_status "HSM configuration file found"
        echo "- ✅ **HSM Config File**: Configuration file present" >> "$VALIDATION_REPORT"
        
        # Check HSM config permissions
        hsm_perms=$(stat -c "%a" "$BEARDOG_CONFIG_DIR/hsm.conf" 2>/dev/null || echo "000")
        if [[ "$hsm_perms" == "600" || "$hsm_perms" == "400" ]]; then
            print_status "HSM config has secure permissions ($hsm_perms)"
            echo "- ✅ **HSM Config Permissions**: Secure permissions ($hsm_perms)" >> "$VALIDATION_REPORT"
        else
            print_error "HSM config has insecure permissions ($hsm_perms)"
            echo "- ❌ **HSM Config Permissions**: Insecure permissions ($hsm_perms)" >> "$VALIDATION_REPORT"
        fi
        
        # Validate HSM configuration content
        if grep -q "attestation_required = true" "$BEARDOG_CONFIG_DIR/hsm.conf"; then
            print_status "HSM attestation is required"
            echo "- ✅ **HSM Attestation**: Attestation required in configuration" >> "$VALIDATION_REPORT"
        else
            print_warning "HSM attestation not explicitly required"
            echo "- ⚠️  **HSM Attestation**: Attestation not explicitly required" >> "$VALIDATION_REPORT"
        fi
        
        if grep -q "tamper_resistance_level.*high" "$BEARDOG_CONFIG_DIR/hsm.conf"; then
            print_status "HSM tamper resistance set to high"
            echo "- ✅ **Tamper Resistance**: High tamper resistance configured" >> "$VALIDATION_REPORT"
        else
            print_warning "HSM tamper resistance not set to high"
            echo "- ⚠️  **Tamper Resistance**: Not set to high level" >> "$VALIDATION_REPORT"
        fi
    else
        print_warning "HSM configuration file not found"
        echo "- ⚠️  **HSM Config File**: Configuration file not found (using defaults)" >> "$VALIDATION_REPORT"
    fi
    
    # Check secure key storage directory
    if [[ -d "$BEARDOG_DATA_DIR/keys" ]]; then
        key_dir_perms=$(stat -c "%a" "$BEARDOG_DATA_DIR/keys" 2>/dev/null || echo "000")
        if [[ "$key_dir_perms" == "700" ]]; then
            print_status "Key storage directory has secure permissions ($key_dir_perms)"
            echo "- ✅ **Key Storage Permissions**: Secure directory permissions ($key_dir_perms)" >> "$VALIDATION_REPORT"
        else
            print_error "Key storage directory has insecure permissions ($key_dir_perms)"
            echo "- ❌ **Key Storage Permissions**: Insecure permissions ($key_dir_perms)" >> "$VALIDATION_REPORT"
        fi
    else
        print_warning "Key storage directory not found"
        echo "- ⚠️  **Key Storage Directory**: Directory not found (will be created)" >> "$VALIDATION_REPORT"
    fi
    
    echo "" >> "$VALIDATION_REPORT"
}

# Function to validate access controls
validate_access_controls() {
    print_info "Validating access control configuration..."
    
    echo "### 🚪 Access Control Validation" >> "$VALIDATION_REPORT"
    echo "" >> "$VALIDATION_REPORT"
    
    # Check if beardog user exists
    if id "beardog" &> /dev/null; then
        print_status "BearDog system user exists"
        echo "- ✅ **System User**: BearDog user account configured" >> "$VALIDATION_REPORT"
        
        # Check user shell
        beardog_shell=$(getent passwd beardog | cut -d: -f7)
        if [[ "$beardog_shell" == "/bin/false" || "$beardog_shell" == "/usr/sbin/nologin" ]]; then
            print_status "BearDog user has no shell access"
            echo "- ✅ **User Shell**: No shell access ($beardog_shell)" >> "$VALIDATION_REPORT"
        else
            print_warning "BearDog user has shell access: $beardog_shell"
            echo "- ⚠️  **User Shell**: Shell access enabled ($beardog_shell)" >> "$VALIDATION_REPORT"
        fi
        
        # Check user home directory
        beardog_home=$(getent passwd beardog | cut -d: -f6)
        if [[ "$beardog_home" == "/opt/beardog" ]]; then
            print_status "BearDog user home directory is secure"
            echo "- ✅ **User Home**: Secure home directory ($beardog_home)" >> "$VALIDATION_REPORT"
        else
            print_warning "BearDog user home directory: $beardog_home"
            echo "- ⚠️  **User Home**: Non-standard home directory ($beardog_home)" >> "$VALIDATION_REPORT"
        fi
    else
        print_error "BearDog system user does not exist"
        echo "- ❌ **System User**: BearDog user account not found" >> "$VALIDATION_REPORT"
    fi
    
    # Check service file permissions
    if [[ -f "/etc/systemd/system/beardog.service" ]]; then
        service_perms=$(stat -c "%a" "/etc/systemd/system/beardog.service" 2>/dev/null || echo "000")
        if [[ "$service_perms" == "644" ]]; then
            print_status "Service file has correct permissions ($service_perms)"
            echo "- ✅ **Service File Permissions**: Correct permissions ($service_perms)" >> "$VALIDATION_REPORT"
        else
            print_warning "Service file permissions: $service_perms"
            echo "- ⚠️  **Service File Permissions**: Non-standard permissions ($service_perms)" >> "$VALIDATION_REPORT"
        fi
        
        # Check for security restrictions in service file
        if grep -q "NoNewPrivileges=true" "/etc/systemd/system/beardog.service"; then
            print_status "Service has NoNewPrivileges restriction"
            echo "- ✅ **Service Security**: NoNewPrivileges enabled" >> "$VALIDATION_REPORT"
        else
            print_warning "Service missing NoNewPrivileges restriction"
            echo "- ⚠️  **Service Security**: NoNewPrivileges not set" >> "$VALIDATION_REPORT"
        fi
        
        if grep -q "ProtectSystem=strict" "/etc/systemd/system/beardog.service"; then
            print_status "Service has strict system protection"
            echo "- ✅ **System Protection**: Strict system protection enabled" >> "$VALIDATION_REPORT"
        else
            print_warning "Service missing strict system protection"
            echo "- ⚠️  **System Protection**: Strict protection not enabled" >> "$VALIDATION_REPORT"
        fi
    else
        print_error "BearDog service file not found"
        echo "- ❌ **Service File**: systemd service file not found" >> "$VALIDATION_REPORT"
    fi
    
    echo "" >> "$VALIDATION_REPORT"
}

# Function to validate network security
validate_network_security() {
    print_info "Validating network security configuration..."
    
    echo "### 🌐 Network Security Validation" >> "$VALIDATION_REPORT"
    echo "" >> "$VALIDATION_REPORT"
    
    # Check listening ports
    if command -v ss &> /dev/null; then
        listening_ports=$(ss -tlnp | grep beardog | awk '{print $4}' | cut -d: -f2 | sort -n | tr '\n' ' ')
        if [[ -n "$listening_ports" ]]; then
            print_status "BearDog listening on ports: $listening_ports"
            echo "- ✅ **Listening Ports**: Ports $listening_ports" >> "$VALIDATION_REPORT"
        else
            print_warning "No BearDog processes found listening"
            echo "- ⚠️  **Listening Ports**: No active listeners (service may not be running)" >> "$VALIDATION_REPORT"
        fi
    else
        print_warning "ss command not available for port checking"
        echo "- ⚠️  **Port Analysis**: ss command not available" >> "$VALIDATION_REPORT"
    fi
    
    # Check firewall rules for BearDog ports
    if command -v ufw &> /dev/null && ufw status | grep -q "Status: active"; then
        if ufw status | grep -q "8080"; then
            print_status "Firewall rule exists for port 8080"
            echo "- ✅ **Firewall Rules**: Port 8080 rule configured" >> "$VALIDATION_REPORT"
        else
            print_warning "No firewall rule found for port 8080"
            echo "- ⚠️  **Firewall Rules**: No rule for port 8080" >> "$VALIDATION_REPORT"
        fi
        
        if ufw status | grep -q "8443"; then
            print_status "Firewall rule exists for port 8443"
            echo "- ✅ **Firewall Rules**: Port 8443 rule configured" >> "$VALIDATION_REPORT"
        else
            print_warning "No firewall rule found for port 8443"
            echo "- ⚠️  **Firewall Rules**: No rule for port 8443" >> "$VALIDATION_REPORT"
        fi
    fi
    
    # Check for default deny policy
    if command -v ufw &> /dev/null; then
        if ufw status verbose | grep -q "Default: deny (incoming)"; then
            print_status "Firewall has default deny policy"
            echo "- ✅ **Default Policy**: Default deny for incoming connections" >> "$VALIDATION_REPORT"
        else
            print_error "Firewall does not have default deny policy"
            echo "- ❌ **Default Policy**: Not set to default deny" >> "$VALIDATION_REPORT"
        fi
    fi
    
    echo "" >> "$VALIDATION_REPORT"
}

# Function to validate logging and audit configuration
validate_logging_audit() {
    print_info "Validating logging and audit configuration..."
    
    echo "### 📋 Logging and Audit Validation" >> "$VALIDATION_REPORT"
    echo "" >> "$VALIDATION_REPORT"
    
    # Check log directory permissions
    if [[ -d "/opt/beardog/logs" ]]; then
        log_dir_perms=$(stat -c "%a" "/opt/beardog/logs" 2>/dev/null || echo "000")
        log_dir_owner=$(stat -c "%U" "/opt/beardog/logs" 2>/dev/null || echo "unknown")
        
        if [[ "$log_dir_perms" == "750" || "$log_dir_perms" == "700" ]]; then
            print_status "Log directory has secure permissions ($log_dir_perms)"
            echo "- ✅ **Log Directory Permissions**: Secure permissions ($log_dir_perms)" >> "$VALIDATION_REPORT"
        else
            print_warning "Log directory permissions: $log_dir_perms"
            echo "- ⚠️  **Log Directory Permissions**: Permissions ($log_dir_perms) may be too permissive" >> "$VALIDATION_REPORT"
        fi
        
        if [[ "$log_dir_owner" == "beardog" ]]; then
            print_status "Log directory owned by beardog user"
            echo "- ✅ **Log Directory Owner**: Owned by beardog user" >> "$VALIDATION_REPORT"
        else
            print_error "Log directory not owned by beardog user (owner: $log_dir_owner)"
            echo "- ❌ **Log Directory Owner**: Owned by $log_dir_owner instead of beardog" >> "$VALIDATION_REPORT"
        fi
    else
        print_warning "Log directory not found"
        echo "- ⚠️  **Log Directory**: Directory not found (will be created)" >> "$VALIDATION_REPORT"
    fi
    
    # Check for audit logging configuration
    if [[ -f "$BEARDOG_CONFIG_DIR/monitoring.toml" ]]; then
        if grep -q "audit_log_retention_days" "$BEARDOG_CONFIG_DIR/monitoring.toml"; then
            retention_days=$(grep "audit_log_retention_days" "$BEARDOG_CONFIG_DIR/monitoring.toml" | awk -F'=' '{print $2}' | tr -d ' ')
            print_status "Audit log retention configured: $retention_days days"
            echo "- ✅ **Audit Retention**: Configured for $retention_days days" >> "$VALIDATION_REPORT"
        else
            print_warning "Audit log retention not configured"
            echo "- ⚠️  **Audit Retention**: Not explicitly configured" >> "$VALIDATION_REPORT"
        fi
    else
        print_warning "Monitoring configuration not found"
        echo "- ⚠️  **Monitoring Config**: Configuration file not found" >> "$VALIDATION_REPORT"
    fi
    
    # Check systemd journal configuration
    if systemctl is-active systemd-journald &> /dev/null; then
        print_status "systemd journal is active"
        echo "- ✅ **System Journal**: systemd-journald active" >> "$VALIDATION_REPORT"
    else
        print_error "systemd journal is not active"
        echo "- ❌ **System Journal**: systemd-journald not active" >> "$VALIDATION_REPORT"
    fi
    
    echo "" >> "$VALIDATION_REPORT"
}

# Function to validate environment variables
validate_environment_variables() {
    print_info "Validating environment variable security..."
    
    echo "### 🔐 Environment Variable Security" >> "$VALIDATION_REPORT"
    echo "" >> "$VALIDATION_REPORT"
    
    # Check for required security environment variables
    security_vars=("BEARDOG_ADMIN_PASSWORD" "BEARDOG_DATABASE_URL" "BEARDOG_UNIVERSAL_ADAPTER_ENDPOINT")
    
    for var in "${security_vars[@]}"; do
        if [[ -n "${!var:-}" ]]; then
            print_status "Security variable $var is set"
            echo "- ✅ **$var**: Environment variable configured" >> "$VALIDATION_REPORT"
        else
            print_error "Security variable $var is not set"
            echo "- ❌ **$var**: Environment variable not set" >> "$VALIDATION_REPORT"
        fi
    done
    
    # Check for potentially insecure debug settings
    if [[ "${RUST_LOG:-}" == "debug" || "${RUST_LOG:-}" == "trace" ]]; then
        print_warning "RUST_LOG is set to debug/trace level"
        echo "- ⚠️  **Debug Logging**: RUST_LOG set to ${RUST_LOG} (may expose sensitive info)" >> "$VALIDATION_REPORT"
    else
        print_status "RUST_LOG not set to debug/trace level"
        echo "- ✅ **Debug Logging**: RUST_LOG not set to verbose levels" >> "$VALIDATION_REPORT"
    fi
    
    echo "" >> "$VALIDATION_REPORT"
}

# Function to generate security recommendations
generate_security_recommendations() {
    print_info "Generating security recommendations..."
    
    cat >> "$VALIDATION_REPORT" << EOF

## 🛡️ Security Recommendations

### High Priority Recommendations
EOF

    if [[ $FAILED_CHECKS -gt 0 ]]; then
        cat >> "$VALIDATION_REPORT" << EOF
- **CRITICAL**: Address all failed security checks immediately
- **Review**: All configurations marked with ❌ require immediate attention
- **Verify**: Ensure all security controls are properly implemented
EOF
    fi

    if [[ $WARNING_CHECKS -gt 0 ]]; then
        cat >> "$VALIDATION_REPORT" << EOF
- **Important**: Review all warning items marked with ⚠️
- **Consider**: Implementing recommended security enhancements
- **Monitor**: Set up monitoring for security events and anomalies
EOF
    fi

    cat >> "$VALIDATION_REPORT" << EOF

### General Security Best Practices
- **Regular Updates**: Keep system and dependencies updated
- **Monitoring**: Implement comprehensive security monitoring
- **Backup**: Ensure secure backup of keys and configuration
- **Incident Response**: Have incident response procedures ready
- **Security Audit**: Schedule regular security audits
- **Penetration Testing**: Consider external security testing

### Production Deployment Security Checklist
- [ ] All failed checks (❌) resolved
- [ ] All warning items (⚠️) reviewed and addressed
- [ ] TLS certificates valid and properly configured
- [ ] HSM integration tested and validated
- [ ] Firewall rules configured and tested
- [ ] Access controls implemented and verified
- [ ] Logging and audit trails configured
- [ ] Environment variables secured
- [ ] Service hardening applied
- [ ] Network security validated

---

## 📊 Validation Summary

**Total Checks**: $TOTAL_CHECKS
**Passed**: $PASSED_CHECKS
**Warnings**: $WARNING_CHECKS  
**Failed**: $FAILED_CHECKS

**Security Score**: $(( (PASSED_CHECKS * 100) / TOTAL_CHECKS ))%

EOF

    if [[ $FAILED_CHECKS -eq 0 ]]; then
        cat >> "$VALIDATION_REPORT" << EOF
**Security Status**: ✅ **READY FOR PRODUCTION** (with warning review)

BearDog security hardening validation completed successfully. All critical security controls are in place.
EOF
    else
        cat >> "$VALIDATION_REPORT" << EOF
**Security Status**: ❌ **NOT READY FOR PRODUCTION**

Critical security issues must be resolved before production deployment.
EOF
    fi

    cat >> "$VALIDATION_REPORT" << EOF

---

**Report Generated**: $(date)
**Validation Complete**: $(date +%H:%M:%S)
EOF
}

# Main validation flow
main() {
    print_info "Starting comprehensive security hardening validation..."
    
    init_validation_report
    validate_system_hardening
    validate_encryption_config
    validate_hsm_config
    validate_access_controls
    validate_network_security
    validate_logging_audit
    validate_environment_variables
    generate_security_recommendations
    
    echo "=================================================="
    echo -e "${GREEN}🔒 Security Validation COMPLETE! 🔒${NC}"
    echo -e "${GREEN}Validation Report: $VALIDATION_REPORT${NC}"
    echo -e "${GREEN}Security Score: $(( (PASSED_CHECKS * 100) / TOTAL_CHECKS ))%${NC}"
    
    if [[ $FAILED_CHECKS -eq 0 ]]; then
        echo -e "${GREEN}Status: READY FOR PRODUCTION ✅${NC}"
    else
        echo -e "${RED}Status: SECURITY ISSUES REQUIRE ATTENTION ❌${NC}"
    fi
    echo "=================================================="
}

# Handle script arguments
case "${1:-validate}" in
    "validate")
        main
        ;;
    "quick")
        print_info "Running quick security check..."
        validate_system_hardening
        validate_encryption_config
        validate_access_controls
        ;;
    "network")
        print_info "Running network security validation..."
        validate_network_security
        ;;
    "hsm")
        print_info "Running HSM security validation..."
        validate_hsm_config
        ;;
    *)
        echo "Usage: $0 [validate|quick|network|hsm]"
        echo "  validate - Full security validation suite (default)"
        echo "  quick    - Quick security check"
        echo "  network  - Network security validation only"
        echo "  hsm      - HSM security validation only"
        exit 1
        ;;
esac 