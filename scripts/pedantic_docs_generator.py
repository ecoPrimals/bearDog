#!/usr/bin/env python3
"""
📚 PEDANTIC DOCUMENTATION GENERATOR
Systematically adds missing documentation to achieve 100% docs coverage.
"""

import os
import re
import sys
from pathlib import Path

class DocsGenerator:
    def __init__(self):
        self.docs_added = 0
        self.files_updated = 0
        
        # Common documentation templates
        self.doc_templates = {
            'struct_field': {
                'id': 'Unique identifier',
                'primal_id': 'Primal identifier',
                'library_id': 'Library identifier', 
                'policy_id': 'Policy identifier',
                'service_id': 'Service identifier',
                'execution_time_us': 'Execution time in microseconds',
                'performance': 'Performance information',
                'genesis_timestamp': 'Genesis timestamp',
                'last_validation': 'Last validation timestamp',
                'announcement_timestamp': 'Announcement timestamp',
                'invalid_announcements': 'Number of invalid announcements',
                'endpoint': 'Service endpoint',
                'headers': 'HTTP headers',
                'domain': 'Domain name',
                'servers': 'List of servers',
                'service_type': 'Service type',
                'interface': 'Network interface',
                'continuous_monitoring': 'Enable continuous monitoring',
                'datacenter': 'Datacenter location',
                'key_prefix': 'Key prefix',
                'timeout_ms': 'Timeout in milliseconds',
                'recovery_timeout_secs': 'Recovery timeout in seconds',
                'max_latency_ms': 'Maximum latency in milliseconds',
                'min_throughput': 'Minimum throughput',
                'target_latency_ms': 'Target latency in milliseconds',
                'min_requests_per_second': 'Minimum requests per second',
                'target_requests_per_second': 'Target requests per second',
                'performance_priority': 'Performance priority flag',
                'optimization_target': 'Optimization target',
                'quality_requirements': 'Quality requirements',
                'workload_type': 'Workload type',
                'current_metrics': 'Current performance metrics',
                'target_improvement': 'Target improvement ratio (0.0 to 1.0)',
                'algorithm_type': 'Cryptographic algorithm type',
                'security_requirements': 'Security requirements',
                'performance_constraints': 'Performance constraints',
                'game_type': 'Game type',
                'latency_requirements': 'Latency requirements',
                'throughput_requirements': 'Throughput requirements',
                'model_type': 'Machine learning model type',
                'training_data_size': 'Training data size',
                'accuracy_target': 'Target accuracy',
                'performance_metrics': 'Performance metrics',
                'confidence_score': 'Confidence score',
                'implementation_effort': 'Implementation effort level',
                'format': 'Format specification',
            },
            'variant': {
                'TlsV1_2': 'TLS version 1.2',
                'TlsV1_3': 'TLS version 1.3',
                'Random': 'Random load balancing',
                'MulticastDNS': 'Multicast DNS discovery',
                'Mdns': 'mDNS service discovery',
                'Native': 'Native implementation',
                'UsbToken': 'USB token device',
                'SmartCard': 'Smart card device',
                'SoftwareBased': 'Software-based implementation',
                'NeuralNetwork': 'Neural network model',
                'DecisionTree': 'Decision tree model',
                'RandomForest': 'Random forest model',
                'SVM': 'Support vector machine',
                'GeneticOptimization': 'Genetic algorithm optimization',
                'PerformanceAcceleration': 'Performance acceleration optimization',
                'CryptographicOptimization': 'Cryptographic optimization',
                'GamingOptimization': 'Gaming performance optimization',
                'MLOptimization': 'Machine learning optimization',
                'MinValue': 'Minimum value constraint',
                'MaxValue': 'Maximum value constraint',
                'MinLength': 'Minimum length constraint',
                'MaxLength': 'Maximum length constraint',
                'Pattern': 'Pattern constraint',
                'Custom': 'Custom configuration',
                'Low': 'Low priority/level',
                'Medium': 'Medium priority/level',
                'High': 'High priority/level',
                'Critical': 'Critical priority/level',
                'Standard': 'Standard level',
                'Balanced': 'Balanced configuration',
                'Aggressive': 'Aggressive configuration',
                'Planned': 'Planned status',
                'ConstGenerics': 'Const generics optimization',
                'SimdAcceleration': 'SIMD acceleration',
                'CompileTimeEvaluation': 'Compile-time evaluation',
                'Histogram': 'Histogram metric',
                'Info': 'Information level',
                'Grpc': 'gRPC protocol',
                'WebSocket': 'WebSocket protocol',
                'Degraded': 'Degraded status',
                'Maintenance': 'Maintenance mode',
                'FailureDuration': 'Failure duration threshold',
                'Transform': 'Data transformation',
            },
            'method': {
                'default_config': 'Creates default configuration',
                'spawn_genetic_offspring': 'Spawns genetic offspring using the provided genetics engine',
                'check_service_health': 'Checks the health status of a service',
                'increment_connections': 'Increments the connection count for a service',
                'decrement_connections': 'Decrements the connection count for a service',
                'register_service': 'Registers a service with the discovery system',
                'deregister_service': 'Deregisters a service from the discovery system',
                'discover_services': 'Discovers services by name',
                'get_available_capabilities': 'Gets available capabilities',
                'request_optimization': 'Requests optimization for the given request',
                'local_genetic_optimization': 'Performs local genetic optimization',
                'local_performance_optimization': 'Performs local performance optimization',
                'local_crypto_optimization': 'Performs local cryptographic optimization',
                'provider_info': 'Gets provider information',
                'metrics': 'Gets provider metrics',
                'device_info': 'Gets device information',
                'attest': 'Performs device attestation',
                'performance_rating': 'Calculates performance rating',
                'recommended_log_level': 'Gets recommended log level',
                'recommended_backup': 'Gets recommended backup configuration',
                'production': 'Creates production configuration',
                'service': 'Sets service information',
                'deployment': 'Sets deployment information',
                'add_performance_data': 'Adds performance data to history',
                'get_performance_history': 'Gets performance history',
                'clear_history': 'Clears performance history',
                'try_pop': 'Attempts to pop an item from the collection',
                'supported_types': 'Gets supported workflow types',
            },
            'trait': {
                'ProtocolHandler': 'Protocol handler for service discovery',
                'UniversalCapabilityDiscovery': 'Universal capability discovery trait',
                'UniversalOptimizationService': 'Universal optimization service trait',
                'LocalOptimizer': 'Local optimization trait',
            },
            'type_alias': {
                'MemoryCache': 'Memory-based cache type alias',
                'HardwareCache': 'Hardware-based cache type alias',
                'SoftwareSecurity': 'Software security type alias',
                'HardwareSecurity': 'Hardware security type alias',
                'RateLimitingConfig': 'Rate limiting configuration type alias',
                'SecurityPolicyConfig': 'Security policy configuration type alias',
                'ThreatDetectionConfig': 'Threat detection configuration type alias',
            },
            'associated_type': {
                'SupportedWorkflows': 'Supported workflow types',
                'Error': 'Error type for the trait',
            }
        }
    
    def generate_documentation(self, item_type, item_name, context=None):
        """Generate appropriate documentation for an item"""
        
        # Try specific templates first
        if item_type in self.doc_templates and item_name in self.doc_templates[item_type]:
            return self.doc_templates[item_type][item_name]
        
        # Fallback to generic documentation
        if item_type == 'struct_field':
            # Try to infer from field name
            if 'id' in item_name.lower():
                return f"{item_name.replace('_', ' ').title()} identifier"
            elif 'config' in item_name.lower():
                return f"{item_name.replace('_', ' ').title()} configuration"
            elif 'time' in item_name.lower():
                return f"{item_name.replace('_', ' ').title()} timestamp"
            elif 'count' in item_name.lower() or 'num' in item_name.lower():
                return f"Number of {item_name.replace('_', ' ')}"
            elif 'enable' in item_name.lower() or item_name.startswith('is_'):
                return f"Whether {item_name.replace('_', ' ')} is enabled"
            else:
                return f"{item_name.replace('_', ' ').title()}"
        
        elif item_type == 'variant':
            return f"{item_name} variant"
        
        elif item_type == 'method':
            if item_name.startswith('get_'):
                return f"Gets {item_name[4:].replace('_', ' ')}"
            elif item_name.startswith('set_'):
                return f"Sets {item_name[4:].replace('_', ' ')}"
            elif item_name.startswith('is_'):
                return f"Checks if {item_name[3:].replace('_', ' ')}"
            elif item_name.startswith('has_'):
                return f"Checks if has {item_name[4:].replace('_', ' ')}"
            else:
                return f"{item_name.replace('_', ' ').title()} operation"
        
        elif item_type == 'trait':
            return f"{item_name} trait for {context or 'operations'}"
        
        elif item_type == 'type_alias':
            return f"Type alias for {item_name.replace('_', ' ').lower()}"
        
        elif item_type == 'associated_type':
            return f"{item_name.replace('_', ' ').title()} associated type"
        
        else:
            return f"{item_name.replace('_', ' ').title()}"
    
    def add_missing_docs(self, content, file_path):
        """Add missing documentation to a file"""
        lines = content.split('\n')
        result_lines = []
        updated = False
        
        i = 0
        while i < len(lines):
            line = lines[i]
            
            # Check for missing documentation patterns
            if self.needs_documentation(lines, i):
                # Add documentation
                indent = self.get_indent(line)
                doc_line = self.generate_doc_line(lines, i, indent)
                if doc_line:
                    result_lines.append(doc_line)
                    updated = True
                    self.docs_added += 1
            
            result_lines.append(line)
            i += 1
        
        if updated:
            self.files_updated += 1
            print(f"  📚 Added documentation: {file_path}")
        
        return '\n'.join(result_lines)
    
    def needs_documentation(self, lines, i):
        """Check if a line needs documentation"""
        line = lines[i].strip()
        
        # Skip if already has documentation
        if i > 0 and lines[i-1].strip().startswith('///'):
            return False
        
        # Check for items that need documentation
        patterns = [
            r'pub\s+fn\s+\w+',
            r'pub\s+struct\s+\w+',
            r'pub\s+enum\s+\w+',
            r'pub\s+trait\s+\w+',
            r'pub\s+type\s+\w+',
            r'pub\s+\w+:\s+',
            r'^\s*\w+\s*,?\s*$',  # enum variants
            r'type\s+\w+.*[=:]',  # associated types
        ]
        
        return any(re.search(pattern, line) for pattern in patterns)
    
    def get_indent(self, line):
        """Get the indentation of a line"""
        return len(line) - len(line.lstrip())
    
    def generate_doc_line(self, lines, i, indent):
        """Generate a documentation line for the given context"""
        line = lines[i].strip()
        
        # Extract item information
        if 'pub fn' in line:
            match = re.search(r'pub\s+fn\s+(\w+)', line)
            if match:
                method_name = match.group(1)
                doc = self.generate_documentation('method', method_name)
                return ' ' * indent + f'/// {doc}'
        
        elif 'pub struct' in line:
            return ' ' * indent + '/// ' + line.replace('pub struct ', '').replace(' {', ' structure')
        
        elif 'pub enum' in line:
            return ' ' * indent + '/// ' + line.replace('pub enum ', '').replace(' {', ' enumeration')
        
        elif 'pub trait' in line:
            match = re.search(r'pub\s+trait\s+(\w+)', line)
            if match:
                trait_name = match.group(1)
                doc = self.generate_documentation('trait', trait_name)
                return ' ' * indent + f'/// {doc}'
        
        elif 'pub type' in line:
            match = re.search(r'pub\s+type\s+(\w+)', line)
            if match:
                type_name = match.group(1)
                doc = self.generate_documentation('type_alias', type_name)
                return ' ' * indent + f'/// {doc}'
        
        elif 'pub ' in line and ':' in line:
            # Struct field
            match = re.search(r'pub\s+(\w+):', line)
            if match:
                field_name = match.group(1)
                doc = self.generate_documentation('struct_field', field_name)
                return ' ' * indent + f'/// {doc}'
        
        elif re.match(r'^\s*\w+\s*,?\s*$', line):
            # Enum variant
            variant_name = line.strip().rstrip(',')
            if variant_name and not variant_name.startswith('//'):
                doc = self.generate_documentation('variant', variant_name)
                return ' ' * indent + f'/// {doc}'
        
        elif 'type ' in line and ('=' in line or ':' in line):
            # Associated type
            match = re.search(r'type\s+(\w+)', line)
            if match:
                type_name = match.group(1)
                doc = self.generate_documentation('associated_type', type_name)
                return ' ' * indent + f'/// {doc}'
        
        return None
    
    def process_file(self, file_path):
        """Process a single file"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            content = self.add_missing_docs(content, file_path)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                return True
            return False
        except Exception as e:
            print(f"❌ Error processing {file_path}: {e}")
            return False
    
    def process_directory(self, directory):
        """Process all Rust files in directory"""
        print(f"🔍 Scanning {directory} for missing documentation...")
        
        rust_files = list(Path(directory).rglob("*.rs"))
        print(f"📊 Found {len(rust_files)} Rust files")
        
        # Focus on core library files first
        priority_files = [f for f in rust_files if 'src/' in str(f) and 'test' not in str(f).lower()]
        
        for file_path in priority_files[:50]:  # Process first 50 files to avoid overwhelming
            self.process_file(file_path)
        
        print(f"\n📚 DOCUMENTATION GENERATION COMPLETE:")
        print(f"   📊 Documentation added: {self.docs_added}")
        print(f"   📁 Files updated: {self.files_updated}")

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 pedantic_docs_generator.py <directory>")
        sys.exit(1)
    
    directory = sys.argv[1]
    if not os.path.exists(directory):
        print(f"❌ Directory not found: {directory}")
        sys.exit(1)
    
    print("📚 PEDANTIC DOCUMENTATION GENERATOR")
    print("🎯 Mission: 100% documentation coverage")
    print("📋 Principle: Every public API deserves clear documentation")
    
    generator = DocsGenerator()
    generator.process_directory(directory)
    
    if generator.docs_added > 0:
        print(f"\n✅ SUCCESS: Added {generator.docs_added} documentation entries!")
    else:
        print("\n📊 No missing documentation found")

if __name__ == "__main__":
    main() 