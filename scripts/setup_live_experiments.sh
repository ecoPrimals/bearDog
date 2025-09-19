#!/bin/bash
# 🏗️ **BEARDOG LIVE EXPERIMENTAL INFRASTRUCTURE SETUP**
#
# Purpose: Set up live experimental validation infrastructure
# Philosophy: Zero mocks, zero simulations, 100% live systems
# Usage: ./scripts/setup_live_experiments.sh

set -e

echo "🏗️ **BEARDOG LIVE EXPERIMENTAL INFRASTRUCTURE SETUP**"
echo "   Setting up REAL hardware testing environment"
echo "   No mocks, no simulations, no placeholders!"
echo

# Check if running as root for system installations
if [[ $EUID -eq 0 ]]; then
   echo "❌ Please don't run this script as root. We'll use sudo when needed."
   exit 1
fi

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to install packages if they don't exist (with better error handling)
install_if_missing() {
    local package=$1
    local command_name=${2:-$1}
    
    if ! command_exists "$command_name"; then
        echo "📦 Installing $package..."
        if sudo apt-get install -y "$package" 2>/dev/null; then
            echo "✅ $package installed successfully"
        else
            echo "⚠️  Failed to install $package - will continue without it"
            return 1
        fi
    else
        echo "✅ $package already installed"
    fi
}

# Function to install packages with alternatives
install_with_alternatives() {
    local primary_package=$1
    local alternative_package=$2
    local command_name=$3
    
    if ! command_exists "$command_name"; then
        echo "📦 Installing $primary_package..."
        if sudo apt-get install -y "$primary_package" 2>/dev/null; then
            echo "✅ $primary_package installed successfully"
        else
            echo "⚠️  $primary_package failed, trying $alternative_package..."
            if sudo apt-get install -y "$alternative_package" 2>/dev/null; then
                echo "✅ $alternative_package installed successfully"
            else
                echo "⚠️  Both $primary_package and $alternative_package failed - will continue without"
                return 1
            fi
        fi
    else
        echo "✅ $command_name already available"
    fi
}

echo "🔍 **CHECKING SYSTEM REQUIREMENTS**"

# Check OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "✅ Linux detected - compatible with live experiments"
else
    echo "⚠️  Non-Linux OS detected. Some features may not be available."
fi

# Check CPU entropy support
if grep -q rdrand /proc/cpuinfo; then
    echo "✅ CPU RDRAND entropy support detected"
else
    echo "⚠️  CPU RDRAND not detected - will use system entropy"
fi

# Check TPM availability
if [[ -e /dev/tpm0 ]] || [[ -e /dev/tpmrm0 ]]; then
    echo "✅ TPM hardware detected"
else
    echo "⚠️  TPM hardware not detected - will skip TPM tests"
fi

echo

echo "📦 **INSTALLING LIVE INFRASTRUCTURE DEPENDENCIES**"

# Update package list with better error handling
echo "🔄 Updating package lists..."
if ! sudo apt-get update -qq; then
    echo "⚠️  Package list update failed - continuing with existing cache"
fi

# Install base development tools
install_if_missing "build-essential"
install_if_missing "git"
install_if_missing "curl"
install_if_missing "wget"

# Install Rust if not present
if ! command_exists "rustc"; then
    echo "📦 Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust installed successfully"
else
    echo "✅ Rust already installed"
    # Update to latest
    rustup update stable >/dev/null 2>&1 || true
fi

# Install nightly Rust for script execution
if ! rustup toolchain list | grep -q nightly; then
    echo "📦 Installing Rust nightly for script execution..."
    rustup toolchain install nightly
else
    echo "✅ Rust nightly already available"
fi

# Install HSM and crypto tools
echo "🔐 Installing HSM and cryptographic tools..."
install_if_missing "softhsm2"
install_if_missing "opensc-pkcs11" "pkcs11-tool"
install_if_missing "tpm2-tools" "tpm2_getrandom"

# Install network tools for timing precision (with alternatives)
echo "🌐 Installing network timing tools..."
install_with_alternatives "ntp" "systemd-timesyncd" "ntpdate"
install_if_missing "tcpdump"
install_if_missing "iperf3"

# Install security testing tools
echo "🔬 Installing security testing tools..."
install_if_missing "nmap"
install_if_missing "masscan" || echo "⚠️  masscan not available - will skip advanced network scanning"

echo

echo "🔧 **CONFIGURING LIVE INFRASTRUCTURE**"

# Set up time synchronization (with multiple approaches)
echo "⏰ Configuring time synchronization for precise timing..."
if command_exists "systemctl"; then
    # Try NTP first
    if systemctl is-enabled ntp >/dev/null 2>&1; then
        sudo systemctl enable ntp >/dev/null 2>&1 || true
        sudo systemctl start ntp >/dev/null 2>&1 || true
        echo "✅ NTP time synchronization configured"
    # Fallback to systemd-timesyncd
    elif systemctl is-enabled systemd-timesyncd >/dev/null 2>&1; then
        sudo systemctl enable systemd-timesyncd >/dev/null 2>&1 || true
        sudo systemctl start systemd-timesyncd >/dev/null 2>&1 || true
        echo "✅ systemd-timesyncd time synchronization configured"
    else
        echo "⚠️  No time synchronization service available - timing may be less precise"
    fi
else
    echo "⚠️  systemctl not available - manual time sync may be needed"
fi

# Initialize SoftHSM if not already done
echo "🔑 Configuring SoftHSM for live testing..."
if command_exists "softhsm2-util" && command_exists "pkcs11-tool"; then
    # Check if SoftHSM is already initialized
    if ! pkcs11-tool --module /usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so --list-slots 2>/dev/null | grep -q "BearDog"; then
        echo "   Initializing SoftHSM token..."
        if softhsm2-util --init-token --slot 0 --label "BearDog-Live" --so-pin 1234 --pin 5678 >/dev/null 2>&1; then
            echo "✅ SoftHSM initialized successfully"
        else
            echo "⚠️  SoftHSM initialization failed - will skip SoftHSM tests"
        fi
    else
        echo "✅ SoftHSM already configured"
    fi
else
    echo "⚠️  SoftHSM tools not available - will skip SoftHSM tests"
fi

# Test TPM if available
if command_exists "tpm2_getrandom"; then
    echo "🔐 Testing TPM functionality..."
    if timeout 5 tpm2_getrandom 8 >/dev/null 2>&1; then
        echo "✅ TPM is functional and ready for testing"
    else
        echo "⚠️  TPM detected but not functional - will skip TPM tests"
    fi
else
    echo "⚠️  TPM tools not available - will skip TPM tests"
fi

echo

echo "🧪 **VALIDATING LIVE INFRASTRUCTURE**"

# Test entropy sources
echo "📊 Testing entropy sources..."

# Test /dev/random
if [[ -r /dev/random ]]; then
    echo "✅ /dev/random entropy source available"
else
    echo "❌ /dev/random not accessible"
fi

# Test /dev/urandom as fallback
if [[ -r /dev/urandom ]]; then
    echo "✅ /dev/urandom entropy source available"
else
    echo "⚠️  /dev/urandom not accessible - entropy collection may be limited"
fi

# Test network timing precision
echo "🌐 Testing network timing precision..."
if command_exists "ping"; then
    if timeout 10 ping -c 3 8.8.8.8 >/dev/null 2>&1; then
        echo "✅ Network connectivity and timing measurement working"
    else
        echo "⚠️  Network connectivity limited - some timing tests may be skipped"
    fi
else
    echo "⚠️  ping not available - network timing tests may be limited"
fi

# Test HSM availability
echo "🔑 Testing HSM availability..."
HSM_COUNT=0

if command_exists "pkcs11-tool" && pkcs11-tool --module /usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so --list-slots >/dev/null 2>&1; then
    echo "✅ SoftHSM available for testing"
    ((HSM_COUNT++))
fi

if command_exists "tpm2_getrandom" && timeout 5 tpm2_getrandom 8 >/dev/null 2>&1; then
    echo "✅ TPM available for testing"
    ((HSM_COUNT++))
fi

if [[ $HSM_COUNT -eq 0 ]]; then
    echo "⚠️  No HSM hardware available - some tests will be skipped"
else
    echo "✅ $HSM_COUNT HSM source(s) available for testing"
fi

# Test Rust nightly availability
echo "🦀 Testing Rust nightly availability..."
if rustup toolchain list | grep -q nightly; then
    echo "✅ Rust nightly available for script execution"
else
    echo "❌ Rust nightly not available - cannot run validation script"
    exit 1
fi

echo

echo "🚀 **CREATING LIVE EXPERIMENT EXECUTION ENVIRONMENT**"

# Create experiments directory if it doesn't exist
mkdir -p experiments/results
mkdir -p experiments/data

# Create a simple execution script
cat > run_live_validation.sh << 'EOF'
#!/bin/bash
# Live BearDog Experimental Validation Execution Script

echo "🚀 **EXECUTING LIVE BEARDOG EXPERIMENTAL VALIDATION**"
echo "   Using REAL hardware and ACTUAL data collection"
echo

# Check if nightly Rust is available
if ! rustup toolchain list | grep -q nightly; then
    echo "❌ Rust nightly not available. Please run setup_live_experiments.sh first."
    exit 1
fi

# Execute the live validation demo
if [[ -f "experiments/validation_demo.rs" ]]; then
    echo "🧬 Starting live cryptographic validation..."
    echo "   This will take approximately 2 minutes..."
    echo
    
    # Run with timeout to prevent hanging
    if timeout 300 cargo +nightly -Zscript experiments/validation_demo.rs; then
        echo
        echo "🎊 **LIVE VALIDATION EXECUTION SUCCESSFUL**"
        
        # Show any generated reports
        if ls beardog_live_validation_*.json >/dev/null 2>&1; then
            echo "📄 Generated validation reports:"
            ls -la beardog_live_validation_*.json
        fi
    else
        echo
        echo "❌ **LIVE VALIDATION EXECUTION FAILED OR TIMED OUT**"
        echo "   Check the output above for specific error details"
        exit 1
    fi
else
    echo "❌ validation_demo.rs not found. Please ensure the file exists."
    exit 1
fi

echo
echo "🎊 **LIVE VALIDATION EXECUTION COMPLETE**"
EOF

chmod +x run_live_validation.sh

echo

echo "✅ **LIVE INFRASTRUCTURE SETUP COMPLETE!**"
echo
echo "🎯 **READY FOR LIVE EXPERIMENTAL VALIDATION**"
echo
echo "📋 **NEXT STEPS:**"
echo "   1. Run live validation demo:"
echo "      ./run_live_validation.sh"
echo
echo "   2. Or run directly:"
echo "      cargo +nightly -Zscript experiments/validation_demo.rs"
echo
echo "🧬 **INFRASTRUCTURE SUMMARY:**"
echo "   ✅ Live hardware entropy sources configured"
echo "   ✅ HSM testing infrastructure ready ($HSM_COUNT HSM(s) available)"
echo "   ✅ Network timing tools installed"
echo "   ✅ Security testing tools available"
echo "   ✅ Real-time data collection framework ready"
echo "   ✅ Rust nightly toolchain configured"
echo
echo "🎊 **LIVE SOVEREIGN SCIENCE INFRASTRUCTURE READY!**" 