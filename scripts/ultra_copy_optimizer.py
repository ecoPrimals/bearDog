#!/usr/bin/env python3
"""
🚀 ULTRA COPY OPTIMIZER
Adds Copy trait to ALL eligible types for theoretical maximum performance.
"""

import os
import re
import sys
from pathlib import Path

class UltraCopyOptimizer:
    def __init__(self):
        self.types_optimized = 0
        self.files_updated = 0
        
        # All eligible types that should get Copy trait
        self.copy_eligible_types = [
            'ConnectionStatus', 'ComputePriority', 'OptimizationType', 'ResourceUsageStats',
            'ComputeMetrics', 'CacheMetrics', 'StorageType', 'StorageOperation', 'StorageStatus',
            'ReplicationHealth', 'LibraryStatus', 'SafetyLevel', 'CpuIntensity', 'SecurityClearance',
            'CacheConfig', 'SecurityConfig', 'ProtocolStatistics', 'HealthStatistics',
            'LoadBalancingAlgorithm', 'CircuitBreakerConfig', 'TlsVersion', 'CacheConfig',
            'AuthenticationMethod', 'NetworkUtils', 'ServiceRegistryConfig', 'EffortLevel',
            'RecommendationPriority', 'CryptoAlgorithmType', 'SecurityRequirements',
            'CryptoPerformanceConstraints', 'GameType', 'LatencyRequirements',
            'ThroughputRequirements', 'MLModelType', 'GeneticQualityRequirements',
            'PerformanceMetrics', 'GeneticTarget', 'WorkloadType', 'LatencyMetrics',
            'BandwidthMetrics', 'DiscoveryProtocol', 'CapabilityRegistryConfig',
            'CapabilityRegistryMetrics', 'EcosystemListenerMetrics'
        ]
    
    def add_copy_to_type(self, content, file_path):
        """Add Copy trait to eligible types"""
        lines = content.split('\n')
        result_lines = []
        updated = False
        
        i = 0
        while i < len(lines):
            line = lines[i]
            
            # Look for derive macros on enums and structs
            if line.strip().startswith('#[derive(') and 'Copy' not in line:
                # Check if next lines contain an eligible type
                j = i + 1
                while j < len(lines) and (lines[j].strip().startswith('///') or lines[j].strip() == ''):
                    j += 1
                
                if j < len(lines):
                    type_line = lines[j].strip()
                    # Extract type name
                    type_name = None
                    if 'pub enum ' in type_line:
                        type_name = type_line.split('pub enum ')[1].split()[0].split('{')[0].strip()
                    elif 'pub struct ' in type_line:
                        type_name = type_line.split('pub struct ')[1].split()[0].split('{')[0].strip()
                    
                    if type_name and (type_name in self.copy_eligible_types or self._is_simple_type(lines, j)):
                        # Add Copy to the derive macro
                        if 'Clone' in line and 'Copy' not in line:
                            new_line = line.replace('Clone', 'Clone, Copy')
                            result_lines.append(new_line)
                            updated = True
                            self.types_optimized += 1
                            print(f"  ✅ Added Copy to {type_name}")
                        else:
                            result_lines.append(line)
                    else:
                        result_lines.append(line)
                else:
                    result_lines.append(line)
            else:
                result_lines.append(line)
            
            i += 1
        
        if updated:
            self.files_updated += 1
        
        return '\n'.join(result_lines)
    
    def _is_simple_type(self, lines, type_line_index):
        """Check if a type is simple enough to implement Copy"""
        # Look ahead to see the type definition
        i = type_line_index + 1
        brace_count = 0
        has_complex_fields = False
        
        while i < len(lines):
            line = lines[i].strip()
            if '{' in line:
                brace_count += line.count('{')
            if '}' in line:
                brace_count -= line.count('}')
                if brace_count == 0:
                    break
            
            # Check for complex field types
            if brace_count > 0 and ':' in line:
                # Extract field type
                field_type = line.split(':')[1].strip().rstrip(',')
                if any(complex_type in field_type for complex_type in ['String', 'Vec', 'HashMap', 'Arc', 'Rc']):
                    has_complex_fields = True
                    break
            
            i += 1
        
        return not has_complex_fields
    
    def optimize_file(self, file_path):
        """Optimize a single file"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            content = self.add_copy_to_type(content, file_path)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                return True
            return False
        except Exception as e:
            print(f"❌ Error processing {file_path}: {e}")
            return False
    
    def optimize_directory(self, directory):
        """Optimize all Rust files in directory"""
        print(f"🔍 Ultra-scanning {directory} for Copy optimization opportunities...")
        
        rust_files = list(Path(directory).rglob("*.rs"))
        print(f"📊 Found {len(rust_files)} Rust files")
        
        for file_path in rust_files:
            # Skip test files
            if 'test' in str(file_path).lower():
                continue
            
            self.optimize_file(file_path)
        
        print(f"\n🚀 ULTRA COPY OPTIMIZATION COMPLETE:")
        print(f"   📊 Types optimized: {self.types_optimized}")
        print(f"   📁 Files updated: {self.files_updated}")

def main():
    print("🚀 ULTRA COPY OPTIMIZER")
    print("🎯 Mission: Add Copy trait to ALL eligible types")
    print("📋 Principle: Theoretical maximum performance through optimal copying")
    
    optimizer = UltraCopyOptimizer()
    optimizer.optimize_directory("crates/beardog-core/src/")
    
    if optimizer.types_optimized > 0:
        print(f"\n✅ SUCCESS: Optimized {optimizer.types_optimized} types for maximum performance!")
    else:
        print("\n📊 All eligible types already optimized")

if __name__ == "__main__":
    main() 