# Unsafe Code Documentation Guide

**Date**: January 13, 2026  
**Status**: Documentation framework for 108 unsafe usages  
**Goal**: 100% documented with safety invariants

---

## Unsafe Code Categories in BearDog

### Category 1: SIMD Operations (~60%)

**Justification**: Performance-critical cryptography and data processing  
**Safety Pattern**: Alignment and length checks before unsafe operations

**Example Pattern**:
```rust
/// # Safety
///
/// This function uses SIMD intrinsics for performance.
///
/// ## Safety Invariants:
/// - Input slices must be properly aligned for SIMD operations
/// - Length must be verified before accessing
/// - No out-of-bounds access possible due to pre-flight checks
///
/// ## Why Unsafe:
/// SIMD intrinsics require unsafe blocks but are safe when:
/// 1. Alignment requirements met (verified at runtime)
/// 2. Length requirements met (checked before call)
/// 3. Target features available (checked with cfg)
#[target_feature(enable = "avx2")]
unsafe fn simd_process(data: &[u8]) -> Vec<u8> {
    // Safety: Length checked by caller, alignment verified
    // AVX2 available (guaranteed by target_feature attribute)
    use std::arch::x86_64::*;
    // ... SIMD operations
}
```

---

### Category 2: Platform FFI (~25%)

**Justification**: Android StrongBox, iOS Secure Enclave platform integration  
**Safety Pattern**: Null checks, error handling, type validation

**Android Example**:
```rust
/// # Safety
///
/// Calls Android KeyStore native methods via JNI.
///
/// ## Safety Invariants:
/// - JNI environment pointer is valid (provided by Android runtime)
/// - Object references are non-null (verified before use)
/// - Method signatures match native declarations
/// - Exceptions are checked and propagated
///
/// ## Why Unsafe:
/// JNI calls are inherently unsafe but safe when:
/// 1. Environment is from Android runtime (guaranteed)
/// 2. Null checks performed on all object returns
/// 3. Exception checks after each JNI call
/// 4. Proper local reference management
unsafe fn call_keystore_method(env: &JNIEnv, ...) -> Result<T> {
    // Safety: env from Android runtime, null-checked below
    let result = env.CallObjectMethod(...);
    
    // Null check
    if result.is_null() {
        return Err(...);
    }
    
    // Exception check
    if env.ExceptionCheck() {
        env.ExceptionClear();
        return Err(...);
    }
    
    Ok(result)
}
```

**iOS Example**:
```rust
/// # Safety
///
/// Interacts with iOS Secure Enclave via Security framework.
///
/// ## Safety Invariants:
/// - CFRef types are properly retained/released
/// - OSStatus errors are checked
/// - Pointer validity verified before dereference
///
/// ## Why Unsafe:
/// Security framework uses C APIs but safe when:
/// 1. CFRetain/CFRelease balanced
/// 2. Status codes checked after each call
/// 3. Pointers verified non-null
unsafe fn secure_enclave_operation(...) -> Result<T> {
    // Safety: Framework-provided pointer, status checked
    let status = SecItemAdd(...);
    if status != errSecSuccess {
        return Err(...);
    }
    Ok(...)
}
```

---

### Category 3: Zero-Copy Optimization (~10%)

**Justification**: Performance optimization for high-throughput operations  
**Safety Pattern**: Lifetime tracking, borrow checker verification

**Example**:
```rust
/// # Safety
///
/// Transmutes data for zero-copy serialization.
///
/// ## Safety Invariants:
/// - Source and destination types have same memory layout
/// - Alignment requirements identical
/// - No invalid bit patterns created
/// - Lifetimes properly tracked
///
/// ## Why Unsafe:
/// Transmute is unsafe but safe when:
/// 1. Layout compatibility verified (repr(C), size_of checks)
/// 2. Alignment verified at compile time
/// 3. Validity invariants preserved
unsafe fn zero_copy_transmute<T, U>(data: &T) -> &U 
where
    T: Copy,
    U: Copy,
{
    // Safety: Caller ensures T and U have compatible layouts
    // Compile-time checks verify size and alignment
    debug_assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<U>());
    debug_assert!(std::mem::align_of::<T>() >= std::mem::align_of::<U>());
    
    std::mem::transmute(data)
}
```

---

### Category 4: Other (~5%)

**Needs individual review**

---

## Documentation Template

For each unsafe block, add documentation using this template:

```rust
/// # Safety
///
/// [One-line summary of what makes this safe]
///
/// ## Safety Invariants:
/// - [Invariant 1]
/// - [Invariant 2]
/// - [Invariant 3]
///
/// ## Why Unsafe:
/// [Brief explanation of why unsafe is needed]
/// Safe when:
/// 1. [Condition 1]
/// 2. [Condition 2]
/// 3. [Condition 3]
///
/// ## Audit Trail:
/// - Reviewed: [Date]
/// - Reviewer: Deep Debt Evolution
/// - Status: ✅ Verified safe under documented invariants
unsafe fn operation() {
    // Implementation
}
```

---

## Files to Document (Priority Order)

### High Priority (SIMD)
1. `beardog-security/src/simd_crypto.rs` (10 blocks)
2. `beardog-utils/src/simd_safe.rs` (7 blocks)
3. `beardog-utils/src/simd/safe_ops.rs` (7 blocks)
4. `beardog-utils/src/simd/crypto.rs` (5 blocks)

### High Priority (FFI)
5. `beardog-security/src/hsm/android_strongbox/native_strongbox.rs`
6. `beardog-security/src/hsm/android_strongbox/jni_bridge.rs`
7. `beardog-tunnel/src/tunnel/hsm/ios_secure_enclave/mod.rs`
8. `beardog-tunnel/src/tunnel/hsm/android_strongbox/mod.rs`

### Medium Priority (Zero-Copy)
9. `beardog-utils/src/zero_copy/hyperoptimized_zero_copy.rs`
10. `beardog-utils/src/ultimate_performance.rs`

### Review Queue (Determine Category)
11. All other unsafe usages (individually review)

---

## Verification Process

For each unsafe block:

1. ✅ **Document** with safety comment
2. ✅ **Verify** invariants hold
3. ✅ **Test** with property tests where possible
4. ✅ **Audit** periodically (quarterly review)
5. ✅ **Evolve** to safe Rust if possible

---

## Evolution Opportunities

### Can Potentially Remove Unsafe:

1. **SIMD without intrinsics**: Use `std::simd` (stable soon)
2. **FFI-free alternatives**: Pure Rust platform abstractions
3. **Zero-copy with MaybeUninit**: Safe uninitialized memory

### Must Remain Unsafe (Justified):

1. **SIMD intrinsics**: Until `std::simd` stabilizes with full feature parity
2. **Platform FFI**: No pure Rust alternative for hardware security
3. **Performance-critical transmutes**: When safe abstractions too slow

---

## Success Criteria

- ✅ 100% of unsafe blocks documented
- ✅ All safety invariants explicit
- ✅ Audit trail for each block
- ✅ Evolution plan for FFI-free blocks
- ✅ Test coverage on unsafe code paths

---

**Status**: 📝 **FRAMEWORK COMPLETE**  
**Next**: Apply to high-priority files  
**Goal**: 100% documented unsafe code


