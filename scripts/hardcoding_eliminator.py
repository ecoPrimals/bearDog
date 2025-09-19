#!/usr/bin/env python3
"""
BearDog Hardcoding Elimination Script

Systematically eliminates ALL vendor and primal hardcoding to achieve
true primal sovereignty where "each primal only knows itself and discovers
others via the universal adapter."

This achieves O(1) scalability instead of 2^n hardcoding complexity.
"""

import os
import re
import sys
import subprocess
from pathlib import Path
from typing import List, Dict, Tuple, Set
from dataclasses import dataclass

@dataclass
class HardcodingViolation:
    file_path: str
    line_number: int
    line_content: str
    violation_type: str
    primal_or_vendor: str
    suggested_replacement: str
    severity: str

class HardcodingEliminator:
    def __init__(self, workspace_root: str = "."):
        self.workspace_root = Path(workspace_root)
        self.violations: List[HardcodingViolation] = []
        self.fixed_violations: List[HardcodingViolation] = []
        
        # Primal hardcoding patterns (violates sovereignty)
        self.primal_patterns = {
            r'\bsongbird\b': {
                'replacement': 'ServiceCapabilityType::ServiceMesh',
                'explanation': 'Replace songbird with mesh capability discovery',
                'severity': 'CRITICAL'
            },
            r'\btoadstool\b': {
                'replacement': 'ServiceCapabilityType::ComputeIntelligence', 
                'explanation': 'Replace toadstool with compute capability discovery',
                'severity': 'CRITICAL'
            },
            r'\bsquirrel\b': {
                'replacement': 'ServiceCapabilityType::DistributedIntelligence',
                'explanation': 'Replace squirrel with AI capability discovery', 
                'severity': 'CRITICAL'
            },
            r'\bnestgate\b': {
                'replacement': 'ServiceCapabilityType::DataStorage',
                'explanation': 'Replace nestgate with storage capability discovery',
                'severity': 'CRITICAL'
            }
        }
        
        # Vendor hardcoding patterns (creates vendor lock-in)
        self.vendor_patterns = {
            r'\bkubernetes\b|\bk8s\b': {
                'replacement': 'ServiceCapabilityType::ContainerOrchestration',
                'explanation': 'Replace K8s with container orchestration capability',
                'severity': 'HIGH'
            },
            r'\baws\b|\bamazon\b': {
                'replacement': 'ServiceCapabilityType::CloudProvider',
                'explanation': 'Replace AWS with cloud provider capability',
                'severity': 'HIGH'  
            },
            r'\bgcp\b|\bgoogle\b': {
                'replacement': 'ServiceCapabilityType::CloudProvider',
                'explanation': 'Replace GCP with cloud provider capability',
                'severity': 'HIGH'
            },
            r'\bazure\b': {
                'replacement': 'ServiceCapabilityType::CloudProvider', 
                'explanation': 'Replace Azure with cloud provider capability',
                'severity': 'HIGH'
            },
            r'\bconsul\b': {
                'replacement': 'ServiceCapabilityType::ServiceDiscovery',
                'explanation': 'Replace Consul with service discovery capability',
                'severity': 'MEDIUM'
            },
            r'\bdocker\b': {
                'replacement': 'ServiceCapabilityType::ContainerRuntime',
                'explanation': 'Replace Docker with container runtime capability',
                'severity': 'MEDIUM'
            }
        }
        
        # Network hardcoding patterns (mostly acceptable in tests/config)
        self.network_patterns = {
            r'"localhost"': {
                'replacement': 'env::var("SERVICE_HOST").unwrap_or_else(|_| "localhost".to_string())',
                'explanation': 'Replace hardcoded localhost with environment variable',
                'severity': 'LOW'
            },
            r'"127\.0\.0\.1"': {
                'replacement': 'env::var("SERVICE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string())',
                'explanation': 'Replace hardcoded IP with environment variable',
                'severity': 'LOW'
            },
            r':8080\b': {
                'replacement': 'env::var("SERVICE_PORT").unwrap_or_else(|_| "8080".to_string())',
                'explanation': 'Replace hardcoded port with environment variable',
                'severity': 'LOW'
            }
        }
    
    def scan_for_violations(self) -> List[HardcodingViolation]:
        """Scan entire workspace for hardcoding violations"""
        print("🔍 Scanning for hardcoding violations...")
        
        # Find all Rust files (excluding target and archive directories)
        rust_files = []
        for pattern in ["**/*.rs"]:
            for file_path in self.workspace_root.glob(pattern):
                if self._should_scan_file(file_path):
                    rust_files.append(file_path)
        
        print(f"📊 Found {len(rust_files)} Rust files to scan")
        
        # Scan each file
        for file_path in rust_files:
            self._scan_file(file_path)
        
        print(f"🚨 Found {len(self.violations)} hardcoding violations")
        return self.violations
    
    def _should_scan_file(self, file_path: Path) -> bool:
        """Check if file should be scanned"""
        path_str = str(file_path)
        
        # Skip target and archive directories
        if any(skip in path_str for skip in ['/target/', '/archive/', '/.git/']):
            return False
            
        # Skip disabled files
        if path_str.endswith('.disabled'):
            return False
            
        return True
    
    def _scan_file(self, file_path: Path) -> None:
        """Scan individual file for violations"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
                
            for line_num, line in enumerate(lines, 1):
                line_lower = line.lower()
                
                # Check primal patterns (highest severity)
                for pattern, info in self.primal_patterns.items():
                    if re.search(pattern, line_lower, re.IGNORECASE):
                        self.violations.append(HardcodingViolation(
                            file_path=str(file_path),
                            line_number=line_num,
                            line_content=line.strip(),
                            violation_type="PRIMAL_HARDCODING",
                            primal_or_vendor=re.search(pattern, line_lower, re.IGNORECASE).group(),
                            suggested_replacement=info['replacement'],
                            severity=info['severity']
                        ))
                
                # Check vendor patterns
                for pattern, info in self.vendor_patterns.items():
                    if re.search(pattern, line_lower, re.IGNORECASE):
                        self.violations.append(HardcodingViolation(
                            file_path=str(file_path),
                            line_number=line_num, 
                            line_content=line.strip(),
                            violation_type="VENDOR_HARDCODING",
                            primal_or_vendor=re.search(pattern, line_lower, re.IGNORECASE).group(),
                            suggested_replacement=info['replacement'],
                            severity=info['severity']
                        ))
                
                # Check network patterns (in production files only)
                if not any(test_dir in str(file_path) for test_dir in ['/tests/', '/examples/', '/benches/']):
                    for pattern, info in self.network_patterns.items():
                        if re.search(pattern, line):
                            self.violations.append(HardcodingViolation(
                                file_path=str(file_path),
                                line_number=line_num,
                                line_content=line.strip(),
                                violation_type="NETWORK_HARDCODING",
                                primal_or_vendor=re.search(pattern, line).group(),
                                suggested_replacement=info['replacement'],
                                severity=info['severity']
                            ))
                            
        except Exception as e:
            print(f"⚠️ Error scanning {file_path}: {e}")
    
    def generate_report(self) -> str:
        """Generate comprehensive hardcoding elimination report"""
        if not self.violations:
            return "✅ No hardcoding violations found! Perfect primal sovereignty achieved!"
        
        report = []
        report.append("# 🚨 BearDog Hardcoding Elimination Report")
        report.append("")
        report.append("## 📊 Summary")
        
        # Count violations by type and severity
        by_type = {}
        by_severity = {}
        
        for violation in self.violations:
            by_type[violation.violation_type] = by_type.get(violation.violation_type, 0) + 1
            by_severity[violation.severity] = by_severity.get(violation.severity, 0) + 1
        
        report.append(f"**Total Violations**: {len(self.violations)}")
        report.append("")
        
        report.append("### By Type:")
        for vtype, count in sorted(by_type.items()):
            report.append(f"- **{vtype}**: {count} violations")
        
        report.append("")
        report.append("### By Severity:")
        for severity, count in sorted(by_severity.items(), reverse=True):
            emoji = {"CRITICAL": "🔴", "HIGH": "🟡", "MEDIUM": "🟠", "LOW": "🔵"}
            report.append(f"- {emoji.get(severity, '⚪')} **{severity}**: {count} violations")
        
        report.append("")
        report.append("## 🔴 Critical Primal Sovereignty Violations")
        
        critical_violations = [v for v in self.violations if v.severity == "CRITICAL"]
        if critical_violations:
            report.append("These violations directly violate the core principle that 'each primal only knows itself':")
            report.append("")
            
            for violation in critical_violations[:10]:  # Show top 10
                report.append(f"### {violation.file_path}:{violation.line_number}")
                report.append(f"```rust")
                report.append(f"// ❌ VIOLATION:")
                report.append(f"{violation.line_content}")
                report.append(f"")
                report.append(f"// ✅ SHOULD BE:")
                report.append(f"// Use {violation.suggested_replacement} for capability discovery")
                report.append(f"```")
                report.append("")
        
        report.append("## 🏗️ Migration Strategy")
        report.append("")
        report.append("### Phase 1: Critical Primal Violations")
        report.append("Replace all hardcoded primal names with capability-based discovery:")
        report.append("")
        report.append("```rust")
        report.append("// BEFORE (violates sovereignty):")
        report.append('let songbird_client = SongbirdClient::new("http://songbird:8080").await?;')
        report.append("")
        report.append("// AFTER (capability-based):")
        report.append("let mesh_providers = universal_adapter")
        report.append("    .discover_capability(ServiceCapabilityType::ServiceMesh).await?;")
        report.append("```")
        report.append("")
        
        report.append("### Phase 2: Vendor Lock-in Elimination")
        report.append("Replace vendor-specific integrations with universal adapters:")
        report.append("")
        report.append("```rust") 
        report.append("// BEFORE (vendor lock-in):")
        report.append("let k8s_client = KubernetesClient::new().await?;")
        report.append("")
        report.append("// AFTER (vendor-agnostic):")
        report.append("let orchestration = universal_adapter")
        report.append("    .discover_capability(ServiceCapabilityType::ContainerOrchestration).await?;")
        report.append("```")
        
        return "\n".join(report)
    
    def auto_fix_violations(self, dry_run: bool = True) -> List[HardcodingViolation]:
        """Automatically fix hardcoding violations where possible"""
        print(f"🔧 {'Dry run' if dry_run else 'Fixing'} hardcoding violations...")
        
        fixed = []
        
        # Group violations by file for efficient processing
        by_file = {}
        for violation in self.violations:
            if violation.file_path not in by_file:
                by_file[violation.file_path] = []
            by_file[violation.file_path].append(violation)
        
        for file_path, violations in by_file.items():
            if self._fix_file_violations(file_path, violations, dry_run):
                fixed.extend(violations)
        
        print(f"✅ {'Would fix' if dry_run else 'Fixed'} {len(fixed)} violations")
        return fixed
    
    def _fix_file_violations(self, file_path: str, violations: List[HardcodingViolation], dry_run: bool) -> bool:
        """Fix violations in a single file"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Apply fixes (in reverse line order to maintain line numbers)
            violations_sorted = sorted(violations, key=lambda v: v.line_number, reverse=True)
            
            for violation in violations_sorted:
                # Simple pattern replacement for now
                # TODO: Implement more sophisticated AST-based replacement
                if violation.violation_type == "PRIMAL_HARDCODING":
                    content = self._replace_primal_hardcoding(content, violation)
                elif violation.violation_type == "VENDOR_HARDCODING":
                    content = self._replace_vendor_hardcoding(content, violation)
            
            if content != original_content:
                if not dry_run:
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(content)
                    print(f"✅ Fixed violations in {file_path}")
                else:
                    print(f"🔍 Would fix violations in {file_path}")
                return True
                
        except Exception as e:
            print(f"⚠️ Error fixing {file_path}: {e}")
            
        return False
    
    def _replace_primal_hardcoding(self, content: str, violation: HardcodingViolation) -> str:
        """Replace primal hardcoding with capability discovery"""
        # This is a simplified replacement - real implementation would use AST
        primal_name = violation.primal_or_vendor
        
        # Add comment explaining the migration
        replacement = f"""// Migrated from hardcoded '{primal_name}' to capability-based discovery
        // Use: universal_adapter.discover_capability({violation.suggested_replacement}).await?"""
        
        return content  # TODO: Implement actual replacement
    
    def _replace_vendor_hardcoding(self, content: str, violation: HardcodingViolation) -> str:
        """Replace vendor hardcoding with universal adapter"""
        # This is a simplified replacement - real implementation would use AST
        return content  # TODO: Implement actual replacement

def main():
    print("🚀 BearDog Hardcoding Elimination Tool")
    print("🎯 Mission: Achieve true primal sovereignty")
    print("📋 Principle: Each primal only knows itself")
    print("")
    
    eliminator = HardcodingEliminator()
    
    # Scan for violations
    violations = eliminator.scan_for_violations()
    
    if not violations:
        print("🎉 Perfect! No hardcoding violations found!")
        print("👑 True primal sovereignty achieved!")
        return 0
    
    # Generate report
    report = eliminator.generate_report()
    
    # Write report to file
    report_path = "HARDCODING_VIOLATIONS_REPORT.md"
    with open(report_path, 'w') as f:
        f.write(report)
    
    print(f"📊 Hardcoding report written to: {report_path}")
    
    # Show summary
    critical = len([v for v in violations if v.severity == "CRITICAL"])
    high = len([v for v in violations if v.severity == "HIGH"]) 
    
    print(f"🚨 Found {len(violations)} total violations:")
    print(f"   🔴 Critical (primal sovereignty): {critical}")
    print(f"   🟡 High (vendor lock-in): {high}")
    print(f"   🟠 Other: {len(violations) - critical - high}")
    
    if critical > 0:
        print("")
        print("🔴 CRITICAL: Primal sovereignty violations detected!")
        print("👑 Each primal should only know itself - fix these first")
    
    return 1 if violations else 0

if __name__ == "__main__":
    sys.exit(main()) 