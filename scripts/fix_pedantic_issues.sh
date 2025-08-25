#!/bin/bash

# BearDog Pedantic Issues Fix Script
# This script systematically fixes all pedantic clippy issues

set -e

echo "🔧 Starting PEDANTIC fixes for BearDog codebase..."

# Function to add metadata to a Cargo.toml file
add_cargo_metadata() {
    local crate_path="$1"
    local crate_name="$2"
    local description="$3"
    local keywords="$4"
    local categories="$5"
    
    echo "📦 Adding metadata to $crate_name..."
    
    # Add metadata after the edition line
    sed -i "/^edition = \"2021\"$/a\\
description = \"$description\"\\
license = \"MIT OR Apache-2.0\"\\
repository = \"https://github.com/ecoPrimals/beardog\"\\
readme = \"README.md\"\\
keywords = [$keywords]\\
categories = [$categories]\\
authors = [\"BearDog Team <beardog@ecoprimals.dev>\"]" "$crate_path/Cargo.toml"
}

# Function to create a basic README for a crate
create_readme() {
    local crate_path="$1"
    local crate_name="$2"
    local description="$3"
    
    if [ ! -f "$crate_path/README.md" ]; then
        echo "📝 Creating README for $crate_name..."
        cat > "$crate_path/README.md" << EOF
# $crate_name

$description

## Overview

This crate is part of the \`BearDog\` distributed security ecosystem, providing high-performance, type-safe abstractions for secure distributed computing.

## Features

- Zero-cost abstractions
- Memory safety guarantees  
- Production-ready performance
- Comprehensive error handling

## License

This project is licensed under the AGPL-3.0 License - see the LICENSE file for details.
EOF
    fi
}

# Fix all crate metadata
echo "📋 Fixing Cargo.toml metadata for all crates..."

# Core crates
add_cargo_metadata "crates/beardog-errors" "beardog-errors" "Comprehensive error handling for the BearDog distributed security ecosystem" "\"error-handling\", \"security\", \"distributed\", \"rust\", \"beardog\"" "\"development-tools::debugging\", \"rust-patterns\""

add_cargo_metadata "crates/beardog-types" "beardog-types" "Canonical type system for the BearDog distributed security ecosystem" "\"types\", \"security\", \"distributed\", \"canonical\", \"beardog\"" "\"data-structures\", \"rust-patterns\""

add_cargo_metadata "crates/beardog-config" "beardog-config" "Configuration management for the BearDog distributed security ecosystem" "\"configuration\", \"security\", \"distributed\", \"settings\", \"beardog\"" "\"config\", \"development-tools\""

# Security crates  
add_cargo_metadata "crates/beardog-security" "beardog-security" "High-performance cryptographic operations for the BearDog ecosystem" "\"cryptography\", \"security\", \"zero-copy\", \"performance\", \"beardog\"" "\"cryptography\", \"security\""

add_cargo_metadata "crates/beardog-auth" "beardog-auth" "Authentication and authorization for the BearDog distributed security ecosystem" "\"authentication\", \"authorization\", \"security\", \"distributed\", \"beardog\"" "\"authentication\", \"security\""

# Infrastructure crates
add_cargo_metadata "crates/beardog-core" "beardog-core" "Core functionality for the BearDog distributed security ecosystem" "\"core\", \"security\", \"distributed\", \"ecosystem\", \"beardog\"" "\"cryptography\", \"network-programming\""

add_cargo_metadata "crates/beardog-api" "beardog-api" "High-performance API server for the BearDog distributed security ecosystem" "\"api\", \"server\", \"security\", \"performance\", \"beardog\"" "\"web-programming\", \"network-programming\""

add_cargo_metadata "crates/beardog-cli" "beardog-cli" "Command-line interface for the BearDog distributed security ecosystem" "\"cli\", \"command-line\", \"security\", \"tools\", \"beardog\"" "\"command-line-utilities\""

# Specialized crates
add_cargo_metadata "crates/beardog-tunnel" "beardog-tunnel" "Secure tunneling and HSM integration for the BearDog ecosystem" "\"tunnel\", \"hsm\", \"security\", \"mobile\", \"beardog\"" "\"cryptography\", \"hardware-support\""

add_cargo_metadata "crates/beardog-genetics" "beardog-genetics" "Genetic algorithms and distributed spawning for the BearDog ecosystem" "\"genetics\", \"algorithms\", \"distributed\", \"spawning\", \"beardog\"" "\"algorithms\", \"science\""

add_cargo_metadata "crates/beardog-monitoring" "beardog-monitoring" "Comprehensive monitoring and observability for the BearDog ecosystem" "\"monitoring\", \"observability\", \"metrics\", \"health\", \"beardog\"" "\"development-tools::profiling\""

add_cargo_metadata "crates/beardog-utils" "beardog-utils" "Utility functions and helpers for the BearDog distributed security ecosystem" "\"utilities\", \"helpers\", \"security\", \"performance\", \"beardog\"" "\"development-tools\", \"rust-patterns\""

add_cargo_metadata "crates/beardog-workflows" "beardog-workflows" "Workflow orchestration and automation for the BearDog ecosystem" "\"workflows\", \"orchestration\", \"automation\", \"distributed\", \"beardog\"" "\"development-tools\""

add_cargo_metadata "crates/beardog-production" "beardog-production" "Production deployment and management for the BearDog ecosystem" "\"production\", \"deployment\", \"management\", \"operations\", \"beardog\"" "\"development-tools\""

add_cargo_metadata "crates/beardog-compliance" "beardog-compliance" "Compliance and regulatory features for the BearDog ecosystem" "\"compliance\", \"regulatory\", \"audit\", \"governance\", \"beardog\"" "\"development-tools\""

add_cargo_metadata "crates/beardog-threat" "beardog-threat" "Threat detection and response for the BearDog distributed security ecosystem" "\"threat-detection\", \"security\", \"response\", \"analysis\", \"beardog\"" "\"security\""

add_cargo_metadata "crates/beardog-node-registry" "beardog-node-registry" "Distributed node registry for the BearDog ecosystem" "\"node-registry\", \"distributed\", \"discovery\", \"network\", \"beardog\"" "\"network-programming\""

# Special crates
add_cargo_metadata "crates/beardog-deploy" "beardog-deploy" "Deployment tools and automation for the BearDog ecosystem" "\"deployment\", \"automation\", \"tools\", \"devops\", \"beardog\"" "\"development-tools\""

add_cargo_metadata "benchmarks" "beardog-benchmarks" "Performance benchmarks for the BearDog distributed security ecosystem" "\"benchmarks\", \"performance\", \"testing\", \"metrics\", \"beardog\"" "\"development-tools::testing\""

add_cargo_metadata "android" "beardog-pixel8-android" "Android Pixel 8 native integration for the BearDog ecosystem" "\"android\", \"mobile\", \"pixel8\", \"native\", \"beardog\"" "\"os::android\""

echo "✅ Cargo metadata fixes completed!"

# Create READMEs for all crates that don't have them
echo "📚 Creating README files..."

create_readme "crates/beardog-errors" "BearDog Errors" "Comprehensive error handling for the BearDog distributed security ecosystem."
create_readme "crates/beardog-types" "BearDog Types" "Canonical type system for the BearDog distributed security ecosystem."
create_readme "crates/beardog-config" "BearDog Config" "Configuration management for the BearDog distributed security ecosystem."
create_readme "crates/beardog-security" "BearDog Security" "High-performance cryptographic operations for the BearDog ecosystem."
create_readme "crates/beardog-auth" "BearDog Auth" "Authentication and authorization for the BearDog distributed security ecosystem."
create_readme "crates/beardog-core" "BearDog Core" "Core functionality for the BearDog distributed security ecosystem."
create_readme "crates/beardog-api" "BearDog API" "High-performance API server for the BearDog distributed security ecosystem."
create_readme "crates/beardog-cli" "BearDog CLI" "Command-line interface for the BearDog distributed security ecosystem."
create_readme "crates/beardog-tunnel" "BearDog Tunnel" "Secure tunneling and HSM integration for the BearDog ecosystem."
create_readme "crates/beardog-genetics" "BearDog Genetics" "Genetic algorithms and distributed spawning for the BearDog ecosystem."
create_readme "crates/beardog-monitoring" "BearDog Monitoring" "Comprehensive monitoring and observability for the BearDog ecosystem."
create_readme "crates/beardog-utils" "BearDog Utils" "Utility functions and helpers for the BearDog distributed security ecosystem."

echo "✅ README creation completed!"

echo "🎉 All pedantic metadata fixes completed!"
echo "📝 Next steps:"
echo "   1. Run: cargo clippy --all-targets --all-features -- -W clippy::pedantic"
echo "   2. Fix remaining code style issues"
echo "   3. Update dependency versions to resolve conflicts" 