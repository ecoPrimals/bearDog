# 🗂️ Archive Cleanup Plan - January 24, 2026

**Task**: Review and archive outdated documentation, keep code as fossil record  
**Date**: January 24, 2026

---

## 📊 Current State

### Root Documentation Files (30 markdown files)
Many of these are from today's evolution session and should be archived for the fossil record.

### Archives Structure
Good archive organization exists:
- `archives/evolution_jan_24_2026/` - Already contains 14 docs from earlier today
- `archives/test_stabilization_jan_24_2026/` - 13 test-related docs
- `archives/smart_file_refactoring_jan_24_2026/` - 16 refactoring docs
- Plus many older session archives

### TODOs in Code
**87 TODOs across 57 files** - These are mostly legitimate work items, not outdated

---

## 🎯 Files to Archive

### Session Documentation (10 files → archive)
These are from today's comprehensive evolution session and should be preserved:

1. **COMPREHENSIVE_CODE_REVIEW_JAN_24_2026.md** (19K)
   - Initial comprehensive audit
   - Move to: `archives/evolution_jan_24_2026/`

2. **COMPREHENSIVE_EVOLUTION_COMPLETE_JAN_24_2026.md** (13K)
   - Evolution completion report
   - Move to: `archives/evolution_jan_24_2026/`

3. **EVOLUTION_PROGRESS_JAN_24_2026.md** (8.3K)
   - Initial progress log
   - Move to: `archives/evolution_jan_24_2026/`

4. **EVOLUTION_PROGRESS_JAN_24_2026_CONTINUED.md** (8.5K)
   - Continued progress log
   - Move to: `archives/evolution_jan_24_2026/`

5. **EVOLUTION_SESSION_SUMMARY_JAN_24_2026.md** (8.4K)
   - Mid-session summary
   - Move to: `archives/evolution_jan_24_2026/`

6. **FINAL_EVOLUTION_SUMMARY_JAN_24_2026.md** (15K)
   - Complete evolution metrics
   - Move to: `archives/evolution_jan_24_2026/`

7. **SESSION_FINAL_SUMMARY_JAN_24_2026.md** (11K)
   - Executive session summary
   - Move to: `archives/evolution_jan_24_2026/`

8. **DOCUMENTATION_IMPROVEMENT_JAN_24_2026.md** (2.6K)
   - Documentation fixes log
   - Move to: `archives/evolution_jan_24_2026/`

9. **HARDCODING_ELIMINATION_STATUS_JAN_24_2026.md** (11K)
   - **KEEP AT ROOT** - This is a strategic plan, not a session doc

10. **DOCUMENTATION_WARNINGS_ANALYSIS_JAN_24_2026.md** (8.9K)
    - **KEEP AT ROOT** - This is a strategic plan, not a session doc

### Update Summary Documentation (3 files → archive)
These document the documentation update itself:

11. **ROOT_DOCS_UPDATE_SUMMARY.md**
    - Summary of root docs update
    - Move to: `archives/evolution_jan_24_2026/`

### Total to Archive: **8 session documents** (~106K)

---

## ✅ Files to KEEP at Root

### Current Status & Navigation (6 files)
These are living documents that should stay at root:
- ✅ **CURRENT_STATUS.md** - Single source of truth for current state
- ✅ **README.md** - Project overview
- ✅ **EVOLUTION_SESSION_INDEX.md** - Index to archived session docs
- ✅ **ROOT_DOCUMENTATION_GUIDE.md** - Master navigation
- ✅ **START_HERE_DEVELOPERS.md** - Developer quick start
- ✅ **ROOT_DOCS_INDEX.md** - Documentation index

### Strategic Plans (2 files)
Active plans for ongoing work:
- ✅ **HARDCODING_ELIMINATION_STATUS_JAN_24_2026.md** - 3-week hardcoding plan
- ✅ **DOCUMENTATION_WARNINGS_ANALYSIS_JAN_24_2026.md** - Documentation roadmap

### Essential Documentation (10+ files)
Core docs that should always be at root:
- ✅ **ARCHITECTURE.md**
- ✅ **CHANGELOG.md**
- ✅ **SECURITY.md**
- ✅ **DOCS_INDEX.md**
- ✅ **QUICK_START.md**
- ✅ **QUICK_START_ZERO_HARDCODING.md**
- ✅ **QUICK_START_SOFTWARE_HSM.md**
- ✅ **QUICK_REFERENCE_TARPC.md**
- ✅ **UNIBIN_ECOBIN_EXPLAINED.md**
- ✅ **UNIVERSAL_ADAPTER_QUICK_REF.md**
- ✅ **ENVIRONMENT_VARIABLES.md**
- ✅ **ENTROPY_HIERARCHY_PRINCIPLE.md**
- ✅ **HOT_PLUG_HSM_DEMO.md**
- ✅ **JWT_SECRET_QUICK_REF.md**
- ✅ **MOCK_ISOLATION_POLICY.md**
- ✅ **PHYSICAL_GENESIS_BOOTSTRAP_PLAN.md**
- ✅ **RUN_ENTROPY_TEST.md**

---

## 📋 Archive Structure Update

### Create/Update Archive README
Update `archives/evolution_jan_24_2026/README.md` to include:
- All 14 existing docs (already there)
- 8 new session docs (to be moved)
- Total: 22 documents

### Archive Organization
```
archives/evolution_jan_24_2026/
├── README.md (updated index)
│
├── Initial Planning (existing)
│   ├── ANDROID_MOCK_EVOLUTION_PLAN_JAN_24_2026.md
│   ├── BEARDOG_HARDENING_PLAN_JAN_24_2026.md
│   ├── BEARDOG_HARDENING_RESPONSE_TO_SONGBIRD_JAN_24_2026.md
│   └── COMPREHENSIVE_EXECUTION_PLAN_JAN_24_2026.md
│
├── Documentation Sprint (existing)
│   ├── DOCUMENTATION_PROGRESS.md
│   ├── DOCUMENTATION_PROGRESS_SESSION_FINAL.md
│   ├── DOCUMENTATION_SPRINT_1_COMPLETE.md
│   └── DOCUMENTATION_SPRINT_FINAL_STATUS.md
│
├── Evolution Completion (existing)
│   ├── EVOLUTION_COMPLETE_JAN_24_2026.md
│   ├── EVOLUTION_READY_FOR_NEXT.md
│   ├── EVOLUTION_SESSION_SUMMARY_JAN_24_2026.md
│   ├── EVOLUTION_SUMMARY_FINAL_JAN_24_2026.md
│   ├── README_EVOLUTION.md
│   └── SESSION_SUMMARY_JAN_24_2026.md
│
└── Comprehensive Session (NEW - to be moved)
    ├── COMPREHENSIVE_CODE_REVIEW_JAN_24_2026.md
    ├── COMPREHENSIVE_EVOLUTION_COMPLETE_JAN_24_2026.md
    ├── EVOLUTION_PROGRESS_JAN_24_2026.md
    ├── EVOLUTION_PROGRESS_JAN_24_2026_CONTINUED.md
    ├── EVOLUTION_SESSION_SUMMARY_JAN_24_2026.md
    ├── FINAL_EVOLUTION_SUMMARY_JAN_24_2026.md
    ├── SESSION_FINAL_SUMMARY_JAN_24_2026.md
    ├── DOCUMENTATION_IMPROVEMENT_JAN_24_2026.md
    └── ROOT_DOCS_UPDATE_SUMMARY.md
```

---

## 🔍 TODOs in Code - Analysis

### Summary: 87 TODOs across 57 files

Most of these are **legitimate work items**, not outdated markers. Examples:
- Graph security validation (5 TODOs)
- Production config management (6 TODOs)
- Network/endpoint discovery (8 TODOs)
- AI/ML integration (3 TODOs)
- HSM provider improvements (4 TODOs)

### Recommendation: **KEEP ALL CODE TODOs**
These mark:
- Future enhancements
- Known limitations
- Areas for improvement
- Technical debt to address

They are part of the "fossil record" and guide future work.

### Action: Create TODO Audit Document
Document all TODOs for tracking, but leave them in code.

---

## 🎯 Execution Plan

### Step 1: Create Archive README (5 min)
Update `archives/evolution_jan_24_2026/README.md` with complete index.

### Step 2: Move Session Documents (5 min)
Move 8 session documents to `archives/evolution_jan_24_2026/`.

### Step 3: Update Navigation (10 min)
Update `EVOLUTION_SESSION_INDEX.md` with new archive location.

### Step 4: Create TODO Audit (10 min)
Document all 87 TODOs in a tracking file (keep in code).

### Step 5: Verify and Test (5 min)
- Verify all links in root docs still work
- Ensure navigation paths are correct
- Test that archive is accessible

### Step 6: Git Operations (5 min)
- `git add` moved files
- `git commit` with clear message
- `git push` via SSH

**Total Time**: ~40 minutes

---

## ✅ Success Criteria

1. ✅ Root directory has only active/living documents
2. ✅ Session documents preserved in archive
3. ✅ Navigation updated to point to archive
4. ✅ All TODOs documented but kept in code
5. ✅ Git history preserved (move, not delete)
6. ✅ All links working correctly

---

## 📝 Files Summary

### To Archive (8 files)
- COMPREHENSIVE_CODE_REVIEW_JAN_24_2026.md
- COMPREHENSIVE_EVOLUTION_COMPLETE_JAN_24_2026.md
- EVOLUTION_PROGRESS_JAN_24_2026.md
- EVOLUTION_PROGRESS_JAN_24_2026_CONTINUED.md
- EVOLUTION_SESSION_SUMMARY_JAN_24_2026.md
- FINAL_EVOLUTION_SUMMARY_JAN_24_2026.md
- SESSION_FINAL_SUMMARY_JAN_24_2026.md
- DOCUMENTATION_IMPROVEMENT_JAN_24_2026.md
- ROOT_DOCS_UPDATE_SUMMARY.md

### To Keep at Root (20+ files)
- Current status and navigation (6)
- Strategic plans (2)
- Essential documentation (12+)

### Code TODOs (87 in 57 files)
- **Action**: Document in audit file
- **Action**: Keep in code as work items

---

**Ready to proceed with archiving?**

---

🐻🐕 **Clean archives preserve history while keeping the root focused.** ✨

