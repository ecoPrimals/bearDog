# GitHub Issues for Production TODOs
**Created**: December 17, 2025  
**Purpose**: Track 7 legitimate TODOs as Phase 2/5 features

---

## Issue #1: Integrate mDNS Discovery for Runtime Primal Discovery

**Title**: Implement mDNS/Bonjour integration for primal discovery

**Labels**: `enhancement`, `phase-2`, `networking`, `discovery`

**Priority**: Medium

**Estimated Effort**: 2-3 hours

**Description**:
Complete the mDNS integration for runtime primal discovery to enable zero-configuration network discovery.

**Current State**:
- Placeholder exists in `crates/beardog-adapters/src/universal/primal_runtime_discovery.rs:96`
- Returns `None` to fall through to other discovery methods
- Framework is ready, needs actual implementation

**Implementation Plan**:
1. Integrate `mdns-sd` crate or similar
2. Implement service announcement and discovery
3. Handle network changes and service updates
4. Add comprehensive tests for discovery scenarios
5. Document mDNS usage patterns

**Acceptance Criteria**:
- [ ] Primals can discover each other on local network without configuration
- [ ] Service announces itself on network
- [ ] Discovery works across different network topologies
- [ ] Graceful fallback to other discovery methods if mDNS unavailable
- [ ] Tests cover discovery, announcement, and failure scenarios

**Related Files**:
- `crates/beardog-adapters/src/universal/primal_runtime_discovery.rs`
- `crates/beardog-config/src/runtime_network_discovery.rs`

**Benefits**:
- Zero-configuration deployment
- Automatic peer discovery
- Completes hardcoding elimination Phase 2

---

## Issue #2: Implement SHA Hardware Acceleration Detection

**Title**: Add runtime detection for CPU SHA extensions (SHA-NI)

**Labels**: `enhancement`, `performance`, `cryptography`, `low-priority`

**Priority**: Low

**Estimated Effort**: 1-2 hours

**Description**:
Add runtime detection of CPU SHA extensions (SHA-NI) to enable hardware-accelerated hashing when available.

**Current State**:
- `crates/beardog-core/src/crypto_service/algorithms/discovery.rs:158`
- Currently set to `hardware_accelerated: false`
- System works fine without it, this is pure optimization

**Implementation Plan**:
1. Use `cpuid` crate or similar for CPU feature detection
2. Detect SHA-NI support at runtime
3. Flag SHA algorithms as hardware-accelerated when supported
4. Consider auto-selecting hardware path when available
5. Benchmark performance improvement

**Acceptance Criteria**:
- [ ] Runtime detection of SHA-NI support
- [ ] Capability correctly reflects hardware acceleration status
- [ ] Performance tests show acceleration when available
- [ ] No performance regression when unavailable
- [ ] Cross-platform support (x86_64, ARM)

**Performance Impact**:
- Expected: 2-4x faster SHA operations on supported CPUs
- Most modern x86_64 CPUs support SHA-NI

**Related Files**:
- `crates/beardog-core/src/crypto_service/algorithms/discovery.rs`

---

## Issue #3: Implement RSA-PSS Signature Algorithm

**Title**: Add RSA-PSS signing and verification support

**Labels**: `enhancement`, `phase-2`, `cryptography`, `optional`

**Priority**: Low (only if needed for specific use cases)

**Estimated Effort**: 3-4 hours

**Description**:
Implement RSA-PSS (Probabilistic Signature Scheme) signing and verification if required for specific integrations.

**Current State**:
- `crates/beardog-core/src/crypto_service/implementation.rs:314`
- Returns explicit error: "RSA-PSS not yet implemented"
- Safe placeholder, no security issues

**Implementation Plan**:
1. Assess if RSA-PSS is actually needed for any use cases
2. If yes, integrate with `rsa` crate
3. Implement signing with proper padding
4. Implement verification
5. Add comprehensive tests including edge cases
6. Document RSA-PSS usage and key sizes

**Decision Point**:
- **Consider removing** if no use case requires RSA-PSS
- Ed25519 and ECDSA P-256 cover most modern needs
- RSA-PSS is legacy compatibility feature

**Acceptance Criteria** (if implemented):
- [ ] RSA-PSS 2048-bit and 4096-bit support
- [ ] Sign and verify operations working
- [ ] Tests covering various key sizes and inputs
- [ ] Documentation on when to use vs Ed25519/ECDSA
- [ ] Performance benchmarks

**Alternative**:
- Remove the `SignatureAlgorithm::RsaPss` variant entirely if not needed

**Related Files**:
- `crates/beardog-core/src/crypto_service/implementation.rs`

---

## Issue #4: Implement Multi-Signature Verification for Genetic Keys

**Title**: Add multi-signature verification for genetic key constraints

**Labels**: `enhancement`, `phase-2`, `genetics`, `advanced-crypto`

**Priority**: Medium

**Estimated Effort**: 1 week

**Description**:
Implement multi-signature verification as part of genetic key constraint enforcement, enabling collaborative key operations with multiple parties.

**Current State**:
- `crates/beardog-genetics/src/constraints/enforcement.rs:240`
- Placeholder returns `Ok(())` - needs implementation
- Part of genetic key constraints system

**Implementation Plan**:
1. Design multi-sig protocol (threshold signatures)
2. Implement signature aggregation
3. Add verification logic for N-of-M signatures
4. Handle key coordination across network
5. Implement timeout and failure handling
6. Add comprehensive tests for various scenarios

**Use Cases**:
- Research collaboration keys requiring multiple approvals
- Joint custody of sensitive data
- Distributed governance of shared resources
- Multi-party consent for operations

**Technical Approach**:
- Consider using threshold cryptography libraries
- BLS signatures for aggregation
- or Schnorr multi-sig for simpler approach

**Acceptance Criteria**:
- [ ] N-of-M signature threshold support
- [ ] Signature aggregation working
- [ ] Network coordination for distributed signing
- [ ] Timeout and failure recovery
- [ ] Tests cover 2-of-3, 3-of-5, and edge cases
- [ ] Documentation with examples

**Related Files**:
- `crates/beardog-genetics/src/constraints/enforcement.rs`
- `crates/beardog-types/src/genetics_constraints.rs`

---

## Issue #5: Implement Behavioral Constraints for Genetic Keys

**Title**: Add behavioral constraint enforcement (biometric, MFA, rate limiting)

**Labels**: `enhancement`, `phase-2`, `genetics`, `security`, `complex`

**Priority**: Medium

**Estimated Effort**: 1-2 weeks

**Description**:
Implement behavioral constraint checking for genetic keys, including biometric verification, multi-factor authentication, and rate limiting.

**Current State**:
- `crates/beardog-genetics/src/constraints/enforcement.rs:249`
- Placeholder comment, no implementation yet
- Part of advanced genetic key security features

**Implementation Plan**:

**1. Biometric Integration**:
- Interface with system biometric APIs (FaceID, TouchID, etc.)
- Support for FIDO2/WebAuthn
- Secure biometric data handling
- Privacy-preserving verification

**2. MFA Support**:
- TOTP (Time-based One-Time Password)
- Hardware security keys (YubiKey, etc.)
- SMS/Email backup codes
- Recovery mechanisms

**3. Rate Limiting**:
- Operation counting per time window
- Progressive backoff on failures
- Anomaly detection for unusual patterns
- Alert mechanisms

**4. Usage Pattern Analysis**:
- Normal usage baseline
- Deviation detection
- Contextual access control (time, location, etc.)

**Acceptance Criteria**:
- [ ] Biometric verification working on supported platforms
- [ ] MFA integration with common methods
- [ ] Rate limiting preventing abuse
- [ ] Usage pattern analysis detecting anomalies
- [ ] Privacy-preserving implementation
- [ ] Comprehensive tests for all constraint types
- [ ] Documentation with configuration examples

**Privacy Considerations**:
- No biometric data storage (use system APIs)
- Minimal telemetry, user-controlled
- Transparent to users what's being tracked

**Related Files**:
- `crates/beardog-genetics/src/constraints/enforcement.rs`
- `crates/beardog-types/src/genetics_constraints.rs`

---

## Issue #6: Complete Behavioral Verification Implementation

**Title**: Implement behavioral verification for genetic keys

**Labels**: `enhancement`, `phase-2`, `genetics`, `security`

**Priority**: Medium

**Estimated Effort**: 1-2 weeks (consolidate with Issue #5)

**Description**:
Complete the behavioral verification system for genetic keys, including biometric checks, usage pattern analysis, and network constraints.

**Current State**:
- `crates/beardog-types/src/genetics_constraints.rs:588`
- Placeholder with TODO comment
- Related to Issue #5 (can be consolidated)

**Note**: This issue overlaps significantly with Issue #5. Consider **consolidating** into a single "Behavioral Constraints & Verification" epic.

**Implementation Approach**:
See Issue #5 for detailed implementation plan. The two TODOs represent different parts of the same system:
- Issue #5: Constraint enforcement engine
- This issue: Verification logic and checks

**Recommendation**: 
Merge with Issue #5 as a single comprehensive implementation.

**Related Files**:
- `crates/beardog-types/src/genetics_constraints.rs`
- `crates/beardog-genetics/src/constraints/enforcement.rs`

---

## Issue #7: Implement Usage Metering and License Checking

**Title**: Add usage metering and license verification system (Phase 5)

**Labels**: `enhancement`, `phase-5`, `licensing`, `enterprise`, `low-priority`

**Priority**: Low (Phase 5 feature)

**Estimated Effort**: 2-3 weeks

**Description**:
Implement comprehensive usage metering and license checking system for enterprise deployments and commercial use tracking.

**Current State**:
- `crates/beardog-core/src/certificates/issuer.rs:194`
- Returns `Ok(false)` - safe default
- Explicitly documented as Phase 5 feature
- Not required for current functionality

**Implementation Plan**:

**Phase 5.1: Basic Metering**
1. Operation counting and tracking
2. Resource usage monitoring
3. Time-based metrics
4. Export to monitoring systems

**Phase 5.2: License System**
1. License key generation and validation
2. Feature flag control
3. Expiration handling
4. Renewal mechanisms

**Phase 5.3: Commercial Detection**
1. Enterprise usage pattern detection
2. Automatic tier classification
3. Usage-based pricing support
4. Transparent reporting

**Use Cases**:
- Open source personal use (free, unlimited)
- Small team deployments (free tier)
- Enterprise commercial use (paid licensing)
- Cloud provider integrations (metered billing)

**Sovereignty Principles**:
- User can always see what's being metered
- Telemetry is opt-out with full disclosure
- No hidden tracking or surveillance
- Open source core remains free

**Acceptance Criteria**:
- [ ] Operation metering working
- [ ] License validation system
- [ ] Transparent reporting to users
- [ ] Enterprise detection accurate
- [ ] Free tier for personal/research use
- [ ] Documentation on licensing model
- [ ] Privacy-preserving implementation

**Timeline**: Not required until Phase 5 (Q2 2026 or later)

**Related Files**:
- `crates/beardog-core/src/certificates/issuer.rs`
- Future: `crates/beardog-licensing/` (new crate)

---

## Summary Table

| Issue | Priority | Effort | Phase | Status |
|-------|----------|--------|-------|--------|
| #1: mDNS Integration | Medium | 2-3h | Phase 2 | Ready to implement |
| #2: SHA Acceleration | Low | 1-2h | Phase 2 | Optional optimization |
| #3: RSA-PSS | Low | 3-4h | Phase 2 | Consider removing |
| #4: Multi-Signature | Medium | 1 week | Phase 2 | Complex feature |
| #5: Behavioral Constraints | Medium | 1-2 weeks | Phase 2 | Complex feature |
| #6: Behavioral Verification | Medium | - | Phase 2 | Merge with #5 |
| #7: License System | Low | 2-3 weeks | Phase 5 | Future work |

**Total Effort**: ~4-5 weeks for all Phase 2 features

**Recommended Priority**:
1. #1 (mDNS) - Completes hardcoding elimination
2. #4 & #5 (Genetics features) - Core genetic key functionality
3. #2 (SHA acceleration) - Easy performance win
4. #3 (RSA-PSS) - Only if actually needed
5. #7 (Licensing) - Phase 5, not urgent

---

## Next Steps

1. Create these as actual GitHub issues
2. Add appropriate labels and milestones
3. Assign to appropriate team members or phases
4. Link related issues (#5 and #6 should be linked)
5. Add to project board for Phase 2 planning

**Note**: All 7 TODOs are legitimate future features, not technical debt. The code is production-ready without them.

---

**Created**: December 17, 2025  
**Reviewed**: Development Team  
**Status**: Ready for issue creation

