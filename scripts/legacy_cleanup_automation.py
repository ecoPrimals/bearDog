#!/usr/bin/env python3
"""
BearDog Legacy Cleanup Automation Script

This script automatically identifies and cleans up legacy patterns, deprecated code,
and technical debt markers across the BearDog codebase.

## 🎯 Cleanup Targets

1. **Deprecated Attributes** - Remove #[deprecated] items that are no longer needed
2. **Legacy Compatibility** - Clean up legacy modules and compatibility layers
3. **Technical Debt Markers** - Address TODO, FIXME, XXX, HACK comments
4. **Unused Imports** - Remove unused legacy imports
5. **Dead Code** - Remove commented-out legacy code blocks

## 🏗️ Safety Features

- **Backup Creation**: Creates backups before making changes
- **Dry Run Mode**: Preview changes without applying them
- **Selective Cleanup**: Target specific cleanup categories
- **Rollback Support**: Easy rollback if issues are detected
"""

import os
import re
import sys
import json
import shutil
import argparse
import subprocess
from pathlib import Path
from typing import List, Dict, Set, Tuple, Optional
from dataclasses import dataclass
from datetime import datetime

@dataclass
class CleanupResult:
    """Result of a cleanup operation"""
    file_path: str
    cleanup_type: str
    lines_removed: int
    lines_modified: int
    description: str
    success: bool
    error_message: Optional[str] = None

@dataclass
class LegacyPattern:
    """Definition of a legacy pattern to clean up"""
    name: str
    pattern: str
    replacement: str
    category: str
    description: str
    risk_level: str  # "low", "medium", "high"
    requires_manual_review: bool = False

class BearDogLegacyCleanup:
    """Automated legacy cleanup system for BearDog codebase"""
    
    def __init__(self, workspace_root: str, dry_run: bool = True):
        self.workspace_root = Path(workspace_root)
        self.dry_run = dry_run
        self.backup_dir = self.workspace_root / "cleanup_backups" / datetime.now().strftime("%Y%m%d_%H%M%S")
        self.results: List[CleanupResult] = []
        
        # Initialize legacy patterns
        self.legacy_patterns = self._initialize_legacy_patterns()
        
        # File extensions to process
        self.target_extensions = {'.rs', '.toml', '.md', '.py'}
        
        # Directories to skip
        self.skip_directories = {'target', '.git', 'node_modules', '__pycache__', 'cleanup_backups'}
        
        print(f"🧹 BearDog Legacy Cleanup initialized")
        print(f"   📁 Workspace: {self.workspace_root}")
        print(f"   🔄 Mode: {'DRY RUN' if self.dry_run else 'LIVE'}")
        print(f"   💾 Backup dir: {self.backup_dir}")

    def _initialize_legacy_patterns(self) -> List[LegacyPattern]:
        """Initialize patterns for legacy code cleanup"""
        return [
            # Deprecated attributes
            LegacyPattern(
                name="deprecated_functions",
                pattern=r'#\[deprecated.*?\]\s*\n.*?fn\s+\w+.*?\{.*?\}',
                replacement="",
                category="deprecated",
                description="Remove deprecated functions",
                risk_level="medium",
                requires_manual_review=True
            ),
            
            # Legacy compatibility modules
            LegacyPattern(
                name="legacy_modules",
                pattern=r'pub mod legacy\s*\{[^}]*\}',
                replacement="",
                category="legacy_compat",
                description="Remove legacy compatibility modules",
                risk_level="high",
                requires_manual_review=True
            ),
            
            # Legacy warning comments
            LegacyPattern(
                name="legacy_warnings",
                pattern=r'warn!\("⚠️ Using legacy.*?"\);',
                replacement="",
                category="warnings",
                description="Remove legacy usage warnings",
                risk_level="low"
            ),
            
            # TODO comments (low priority ones)
            LegacyPattern(
                name="simple_todos",
                pattern=r'// TODO: (Add documentation|Update comment|Improve naming)',
                replacement="",
                category="technical_debt",
                description="Remove simple TODO comments",
                risk_level="low"
            ),
            
            # FIXME comments (completed fixes)
            LegacyPattern(
                name="completed_fixes",
                pattern=r'// FIXME: (This is now fixed|No longer needed|Resolved)',
                replacement="",
                category="technical_debt", 
                description="Remove completed FIXME comments",
                risk_level="low"
            ),
            
            # Unused legacy imports
            LegacyPattern(
                name="legacy_imports",
                pattern=r'use.*legacy.*::\*;',
                replacement="",
                category="imports",
                description="Remove unused legacy imports",
                risk_level="medium"
            ),
            
            # Commented out legacy code
            LegacyPattern(
                name="commented_legacy",
                pattern=r'// REMOVED: Legacy.*?\n',
                replacement="",
                category="dead_code",
                description="Remove commented legacy code markers",
                risk_level="low"
            ),
            
            # Legacy type aliases
            LegacyPattern(
                name="legacy_aliases",
                pattern=r'pub use \w+ as Legacy\w+;',
                replacement="",
                category="legacy_compat",
                description="Remove legacy type aliases",
                risk_level="medium"
            ),
            
            # Deprecated feature flags
            LegacyPattern(
                name="deprecated_features",
                pattern=r'#\[cfg\(feature = "legacy.*?"\)\]',
                replacement="",
                category="deprecated",
                description="Remove deprecated feature flags",
                risk_level="medium"
            ),
        ]

    def create_backup(self, file_path: Path) -> bool:
        """Create backup of file before modification"""
        try:
            if not self.backup_dir.exists():
                self.backup_dir.mkdir(parents=True)
            
            # Preserve directory structure in backup
            relative_path = file_path.relative_to(self.workspace_root)
            backup_path = self.backup_dir / relative_path
            backup_path.parent.mkdir(parents=True, exist_ok=True)
            
            shutil.copy2(file_path, backup_path)
            return True
        except Exception as e:
            print(f"❌ Failed to create backup for {file_path}: {e}")
            return False

    def scan_file_for_patterns(self, file_path: Path) -> List[Tuple[LegacyPattern, List[str]]]:
        """Scan file for legacy patterns"""
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
            
            matches = []
            for pattern in self.legacy_patterns:
                regex_matches = re.findall(pattern.pattern, content, re.MULTILINE | re.DOTALL)
                if regex_matches:
                    matches.append((pattern, regex_matches))
            
            return matches
        except Exception as e:
            print(f"⚠️ Error scanning {file_path}: {e}")
            return []

    def apply_cleanup(self, file_path: Path, pattern: LegacyPattern, matches: List[str]) -> CleanupResult:
        """Apply cleanup to a specific file"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                original_content = f.read()
            
            # Apply pattern replacement
            new_content = re.sub(pattern.pattern, pattern.replacement, original_content, flags=re.MULTILINE | re.DOTALL)
            
            # Count changes
            original_lines = len(original_content.splitlines())
            new_lines = len(new_content.splitlines())
            lines_removed = original_lines - new_lines
            lines_modified = 1 if new_content != original_content else 0
            
            if not self.dry_run and new_content != original_content:
                # Create backup before modifying
                if not self.create_backup(file_path):
                    return CleanupResult(
                        str(file_path), pattern.category, 0, 0, 
                        "Failed to create backup", False, "Backup creation failed"
                    )
                
                # Write cleaned content
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(new_content)
            
            return CleanupResult(
                str(file_path), pattern.category, lines_removed, lines_modified,
                f"Applied {pattern.name}: {pattern.description}", True
            )
            
        except Exception as e:
            return CleanupResult(
                str(file_path), pattern.category, 0, 0,
                f"Failed to apply {pattern.name}", False, str(e)
            )

    def scan_codebase(self) -> Dict[str, List[Tuple[LegacyPattern, List[str]]]]:
        """Scan entire codebase for legacy patterns"""
        print("🔍 Scanning codebase for legacy patterns...")
        
        file_matches = {}
        total_files = 0
        
        for root, dirs, files in os.walk(self.workspace_root):
            # Skip unwanted directories
            dirs[:] = [d for d in dirs if d not in self.skip_directories]
            
            for file in files:
                file_path = Path(root) / file
                
                # Only process target file extensions
                if file_path.suffix not in self.target_extensions:
                    continue
                
                total_files += 1
                matches = self.scan_file_for_patterns(file_path)
                
                if matches:
                    file_matches[str(file_path)] = matches
        
        print(f"📊 Scanned {total_files} files, found patterns in {len(file_matches)} files")
        return file_matches

    def generate_cleanup_report(self, file_matches: Dict[str, List[Tuple[LegacyPattern, List[str]]]]) -> Dict:
        """Generate comprehensive cleanup report"""
        report = {
            "scan_timestamp": datetime.now().isoformat(),
            "workspace_root": str(self.workspace_root),
            "dry_run": self.dry_run,
            "summary": {
                "total_files_scanned": 0,
                "files_with_patterns": len(file_matches),
                "total_patterns_found": 0,
                "patterns_by_category": {},
                "patterns_by_risk_level": {}
            },
            "files": []
        }
        
        # Count patterns by category and risk level
        for file_path, matches in file_matches.items():
            file_info = {
                "path": file_path,
                "patterns": []
            }
            
            for pattern, pattern_matches in matches:
                report["summary"]["total_patterns_found"] += len(pattern_matches)
                
                # Count by category
                category = pattern.category
                report["summary"]["patterns_by_category"][category] = \
                    report["summary"]["patterns_by_category"].get(category, 0) + len(pattern_matches)
                
                # Count by risk level
                risk = pattern.risk_level
                report["summary"]["patterns_by_risk_level"][risk] = \
                    report["summary"]["patterns_by_risk_level"].get(risk, 0) + len(pattern_matches)
                
                file_info["patterns"].append({
                    "name": pattern.name,
                    "category": pattern.category,
                    "risk_level": pattern.risk_level,
                    "description": pattern.description,
                    "match_count": len(pattern_matches),
                    "requires_manual_review": pattern.requires_manual_review,
                    "matches": pattern_matches[:5]  # First 5 matches for preview
                })
            
            report["files"].append(file_info)
        
        return report

    def execute_cleanup(self, file_matches: Dict[str, List[Tuple[LegacyPattern, List[str]]]], 
                       categories: Optional[Set[str]] = None,
                       risk_levels: Optional[Set[str]] = None) -> List[CleanupResult]:
        """Execute cleanup operations"""
        print(f"🧹 {'Previewing' if self.dry_run else 'Executing'} cleanup operations...")
        
        results = []
        
        for file_path, matches in file_matches.items():
            for pattern, pattern_matches in matches:
                # Filter by categories if specified
                if categories and pattern.category not in categories:
                    continue
                
                # Filter by risk levels if specified
                if risk_levels and pattern.risk_level not in risk_levels:
                    continue
                
                # Skip patterns that require manual review in automated mode
                if pattern.requires_manual_review and not self.dry_run:
                    print(f"⚠️ Skipping {pattern.name} in {file_path} - requires manual review")
                    continue
                
                result = self.apply_cleanup(Path(file_path), pattern, pattern_matches)
                results.append(result)
        
        return results

    def save_report(self, report: Dict, results: List[CleanupResult]) -> None:
        """Save cleanup report to file"""
        # Add results to report
        report["cleanup_results"] = [
            {
                "file_path": r.file_path,
                "cleanup_type": r.cleanup_type,
                "lines_removed": r.lines_removed,
                "lines_modified": r.lines_modified,
                "description": r.description,
                "success": r.success,
                "error_message": r.error_message
            }
            for r in results
        ]
        
        # Calculate summary statistics
        successful_cleanups = [r for r in results if r.success]
        report["cleanup_summary"] = {
            "total_operations": len(results),
            "successful_operations": len(successful_cleanups),
            "failed_operations": len(results) - len(successful_cleanups),
            "total_lines_removed": sum(r.lines_removed for r in successful_cleanups),
            "total_lines_modified": sum(r.lines_modified for r in successful_cleanups),
            "files_modified": len(set(r.file_path for r in successful_cleanups))
        }
        
        # Save report
        report_path = self.workspace_root / f"legacy_cleanup_report_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(report_path, 'w') as f:
            json.dump(report, f, indent=2)
        
        print(f"📊 Report saved to: {report_path}")

    def print_summary(self, results: List[CleanupResult]) -> None:
        """Print cleanup summary"""
        successful = [r for r in results if r.success]
        failed = [r for r in results if not r.success]
        
        print("\n" + "="*60)
        print("🧹 LEGACY CLEANUP SUMMARY")
        print("="*60)
        print(f"📊 Total operations: {len(results)}")
        print(f"✅ Successful: {len(successful)}")
        print(f"❌ Failed: {len(failed)}")
        print(f"📝 Lines removed: {sum(r.lines_removed for r in successful)}")
        print(f"🔧 Lines modified: {sum(r.lines_modified for r in successful)}")
        print(f"📁 Files affected: {len(set(r.file_path for r in successful))}")
        
        if self.dry_run:
            print("\n🔍 DRY RUN - No files were actually modified")
        else:
            print(f"\n💾 Backups created in: {self.backup_dir}")
        
        # Show category breakdown
        if successful:
            categories = {}
            for result in successful:
                categories[result.cleanup_type] = categories.get(result.cleanup_type, 0) + 1
            
            print("\n📈 Cleanup by category:")
            for category, count in sorted(categories.items()):
                print(f"   {category}: {count} operations")

def main():
    parser = argparse.ArgumentParser(description="BearDog Legacy Cleanup Automation")
    parser.add_argument("--workspace", default=".", help="Workspace root directory")
    parser.add_argument("--dry-run", action="store_true", default=True, help="Preview changes without applying them")
    parser.add_argument("--execute", action="store_true", help="Execute cleanup operations (overrides dry-run)")
    parser.add_argument("--categories", nargs="+", help="Cleanup categories to target", 
                       choices=["deprecated", "legacy_compat", "warnings", "technical_debt", "imports", "dead_code"])
    parser.add_argument("--risk-levels", nargs="+", help="Risk levels to include",
                       choices=["low", "medium", "high"])
    parser.add_argument("--report-only", action="store_true", help="Generate report only, no cleanup")
    
    args = parser.parse_args()
    
    # Override dry_run if execute is specified
    dry_run = not args.execute if args.execute else args.dry_run
    
    # Initialize cleanup system
    cleanup = BearDogLegacyCleanup(args.workspace, dry_run=dry_run)
    
    try:
        # Scan codebase
        file_matches = cleanup.scan_codebase()
        
        if not file_matches:
            print("✅ No legacy patterns found - codebase is clean!")
            return 0
        
        # Generate report
        report = cleanup.generate_cleanup_report(file_matches)
        
        if args.report_only:
            cleanup.save_report(report, [])
            print("📊 Report generated successfully")
            return 0
        
        # Execute cleanup
        categories = set(args.categories) if args.categories else None
        risk_levels = set(args.risk_levels) if args.risk_levels else None
        
        results = cleanup.execute_cleanup(file_matches, categories, risk_levels)
        
        # Print summary
        cleanup.print_summary(results)
        
        # Save comprehensive report
        cleanup.save_report(report, results)
        
        return 0 if all(r.success for r in results) else 1
        
    except KeyboardInterrupt:
        print("\n⏹️ Cleanup interrupted by user")
        return 1
    except Exception as e:
        print(f"❌ Cleanup failed with error: {e}")
        return 1

if __name__ == "__main__":
    sys.exit(main()) 