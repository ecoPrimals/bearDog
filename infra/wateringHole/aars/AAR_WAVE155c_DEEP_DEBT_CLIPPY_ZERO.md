# AAR — Wave 155c: Deep Debt Sweep + Clippy Zero

**Date**: July 27, 2026
**Scope**: Deep debt evolution, Clippy zero, CLI wiring, production unwrap hardening

---

## What We Did

### 1. Clippy Zero (31 → 0 warnings)
Fixed warnings across 10 files in 5 crates:
- **beardog-crypto** (8): `# Errors` doc sections on 5 symmetric encrypt/decrypt functions, removed redundant `#[must_use]` on `hmac_sha256` (Result is already must_use), `# Errors` on `hash_password_argon2`
- **libtower** (6): `BearDog` backticks in module docs, `const fn` on `tower_version`, `if let` → `let else` for key slicing, `i32::from(!valid)` for bool conversion
- **beardog-tunnel** (13): `BearDog`/`EllipticCurve`/`SoloKey` backticks, `# Errors` on beacon handlers, collapsible `if`, empty line after doc comment, doc_lazy_continuation, `type_complexity` suppressed on deprecated TLS key derivation code (songBird handoff)
- **beardog-config** (1): `field_reassign_with_default` → struct initializer in `PortDiscoveryConfig::from_env()`
- **beardog-types** (3): `field_reassign_with_default` → struct initializer in `development()`, doc_lazy_continuation fixes

### 2. CLI Dead Code Audit + Wiring
- **Wired**: `handle_revocation_export` and `handle_revocation_import` — complete, tested handlers connected to `ExportRevocations`/`ImportRevocations` CLI subcommands
- **Deleted**: `commands/mod.rs` orphan (declared `pub mod ai` but `ai.rs` never existed, module never included)
- **Moved to `#[cfg(test)]`**: `save_entropy_file`/`load_entropy_file` (test utilities, not production API), `select_hsm` (duplicate of live implementations with inconsistent priority order: Hardware > Software > Mobile vs. Mobile > Hardware > Software in live code)
- **Kept with `#[allow(dead_code)]`**: HOME-based pub API shims (`list_keys`, `delete_key`, `RevocationList::load/save`) — intentional embedder surface

### 3. Production Unwrap Hardening
- **6 sites fixed**: FIDO2 CTAP2 `client_pin.rs` and `hmac_secret.rs` — `expect()` → `BearDogError::hsm()` with `?` propagation for HMAC and COSE encoding
- **9 sites kept with documented `#[expect]`**: Mutex poisoning (unrecoverable), HKDF/HMAC infallible paths, SIGTERM handler
- **0 unfixed production unwrap sites remain**

### 4. beardog-acme Housekeeping
- Added `publish = false` to Cargo.toml (stays in workspace for `tls-gateway` opt-in)
- Already feature-gated in Wave 155b; TLS deps no longer in default build

---

## What We Found

### Dead Code Distribution
| Category | Count | Action |
|----------|-------|--------|
| Unwired CLI handlers | 2 | Wired (export/import revocations) |
| Test-only functions with pub API markers | 3 | Moved to `#[cfg(test)]` |
| Orphan modules | 1 | Deleted |
| Dead_code markers (intentional embedder API) | ~12 | Kept with documented reason |

### File Size Compliance
No production file exceeds 800L. Top 8 files (767L–708L) are in watch zone:
- `bootstrap.rs` (767L) — struct definitions, naturally long
- `solo_v2/provider.rs` (752L) — already refactored in Wave 152
- `ecosystem_membership/types.rs` (750L) — type definitions
- `audit_types.rs` (749L) — type definitions
- Extraction points documented for when files approach threshold

### Production Unwrap Audit
True production `unwrap`/`expect` count: **15** (not ~2,800 as naive grep suggests). The vast majority (~2,800) are in inline `#[cfg(test)]` modules within production files. Relocating test modules to `*_tests.rs` files would clarify this metric.

---

## For Upstream

### bearDog posture after 155c:
- **Clippy**: 0 warnings (pedantic + nursery + all cast lints)
- **Production unwrap**: 0 unfixed sites (9 documented #[expect])
- **Dead code**: Orphans deleted, test-only code properly gated
- **TLS**: Feature-gated, default build is TLS-free
- **All tests passing, 0 failures**

### Next work for bearDog:
- G6: crates.io public flip — metadata done, dry-run passes for Tier 1, publish order validated
- Test module relocation (inline `#[cfg(test)]` → `*_tests.rs`) to clarify production metrics
- G5 Phase 1: Additional libtower exports (AES-GCM, X25519, HKDF)
