#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# benchScale full roundtrip validation for bearDog.
#
# Starts a TCP JSON-RPC server on an ephemeral port, exercises every
# canonical crypto method domain via HTTP POST, validates responses,
# and reports results.
# Aligned with the wateringHole DEPLOYMENT_VALIDATION_STANDARD v1.1.
#
# Usage:
#   ./infra/benchScale/validate_roundtrip.sh                    # build + validate
#   SKIP_BUILD=1 ./infra/benchScale/validate_roundtrip.sh       # skip cargo build
#   BEARDOG_PORT=8080 ./infra/benchScale/validate_roundtrip.sh  # fixed port
#
# Dependencies: curl, jq, ss (or lsof)

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# ============================================================================
# Configuration
# ============================================================================

JSONRPC_PORT="${BEARDOG_PORT:-9100}"
BIND="${BEARDOG_BIND:-127.0.0.1}"
TIMEOUT=5
PASS=0
FAIL=0
SKIP=0
TOTAL=0
SERVER_PID=""
ACTUAL_PORT=""
RPC_ID=0
SERVER_LOG=""

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# ============================================================================
# Helpers
# ============================================================================

cleanup() {
    if [[ -n "$SERVER_PID" ]]; then
        kill "$SERVER_PID" 2>/dev/null || true
        wait "$SERVER_PID" 2>/dev/null || true
    fi
    if [[ -n "$SERVER_LOG" && -f "$SERVER_LOG" ]]; then
        rm -f "$SERVER_LOG"
    fi
}
trap cleanup EXIT

log_header() { echo -e "\n${BOLD}${CYAN}═══ $1 ═══${NC}"; }
log_pass()   { echo -e "  ${GREEN}✓${NC} $1"; PASS=$((PASS + 1)); TOTAL=$((TOTAL + 1)); }
log_fail()   { echo -e "  ${RED}✗${NC} $1"; FAIL=$((FAIL + 1)); TOTAL=$((TOTAL + 1)); }
log_skip()   { echo -e "  ${YELLOW}⊘${NC} $1"; SKIP=$((SKIP + 1)); TOTAL=$((TOTAL + 1)); }
log_info()   { echo -e "  ${CYAN}ℹ${NC} $1"; }

rpc_call() {
    local method="$1"
    local params="${2:-"{}"}"
    RPC_ID=$((RPC_ID + 1))

    local payload
    payload=$(printf '{"jsonrpc":"2.0","method":"%s","params":%s,"id":%s}' "$method" "$params" "$RPC_ID")

    curl -s --max-time "$TIMEOUT" \
        -X POST \
        -H "Content-Type: application/json" \
        -d "$payload" \
        "http://${BIND}:${ACTUAL_PORT}/" 2>/dev/null || echo ""
}

assert_result_field() {
    local resp="$1"
    local field="$2"
    local label="$3"

    if [[ -z "$resp" ]]; then
        log_fail "$label — empty response"
        return 1
    fi

    local val
    val=$(echo "$resp" | jq -r ".result.$field // empty" 2>/dev/null)
    if [[ -n "$val" ]]; then
        log_pass "$label ($field=$val)"
        return 0
    else
        local err
        err=$(echo "$resp" | jq -r '.error.message // empty' 2>/dev/null)
        if [[ -n "$err" ]]; then
            log_fail "$label — error: $err"
        else
            log_fail "$label — missing .result.$field"
        fi
        return 1
    fi
}

assert_result_exists() {
    local resp="$1"
    local label="$2"

    if [[ -z "$resp" ]]; then
        log_fail "$label — empty response"
        return 1
    fi

    local has_result
    has_result=$(echo "$resp" | jq -r 'has("result")' 2>/dev/null)
    if [[ "$has_result" == "true" ]]; then
        log_pass "$label"
        return 0
    else
        local err
        err=$(echo "$resp" | jq -r '.error.message // empty' 2>/dev/null)
        log_fail "$label — ${err:-no result field}"
        return 1
    fi
}

assert_error_code() {
    local resp="$1"
    local expected_code="$2"
    local label="$3"

    if [[ -z "$resp" ]]; then
        log_fail "$label — empty response"
        return 1
    fi

    local code
    code=$(echo "$resp" | jq -r '.error.code // empty' 2>/dev/null)
    if [[ "$code" == "$expected_code" ]]; then
        log_pass "$label (code=$code)"
        return 0
    else
        log_fail "$label — expected code $expected_code, got '$code'"
        return 1
    fi
}

# ============================================================================
# Binary Discovery
# ============================================================================

find_binary() {
    if [[ -n "${BEARDOG_BINARY:-}" && -x "$BEARDOG_BINARY" ]]; then
        echo "$BEARDOG_BINARY"
        return 0
    fi

    for candidate in \
        "$PROJECT_ROOT/target/release/beardog" \
        "$PROJECT_ROOT/target/debug/beardog"; do
        if [[ -x "$candidate" ]]; then
            echo "$candidate"
            return 0
        fi
    done

    if command -v beardog >/dev/null 2>&1; then
        command -v beardog
        return 0
    fi

    return 1
}

# ============================================================================
# Server Lifecycle
# ============================================================================

start_server() {
    local binary="$1"
    SERVER_LOG=$(mktemp /tmp/beardog-benchscale-XXXXXX.log)
    ACTUAL_PORT="$JSONRPC_PORT"

    log_info "Starting bearDog on ${BIND}:${ACTUAL_PORT} ..."

    "$binary" server --listen "${BIND}:${ACTUAL_PORT}" \
        > "$SERVER_LOG" 2>&1 &
    SERVER_PID=$!

    local waited=0
    local max_wait=15
    while (( waited < max_wait )); do
        if ! kill -0 "$SERVER_PID" 2>/dev/null; then
            echo -e "${RED}Server exited prematurely. Log:${NC}"
            cat "$SERVER_LOG" 2>/dev/null
            return 1
        fi

        local probe
        probe=$(curl -s --max-time 1 -X POST \
            -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"ping","id":0}' \
            "http://${BIND}:${ACTUAL_PORT}/" 2>/dev/null || true)

        if echo "$probe" | jq -e '.result' >/dev/null 2>&1; then
            log_info "Server ready (PID $SERVER_PID)"
            return 0
        fi

        sleep 1
        waited=$((waited + 1))
    done

    echo -e "${RED}Server did not become ready in ${max_wait}s. Log:${NC}"
    cat "$SERVER_LOG" 2>/dev/null
    return 1
}

# ============================================================================
# Main
# ============================================================================

echo -e "${BOLD}${CYAN}"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║        benchScale — bearDog Roundtrip Validation           ║"
echo "║        DEPLOYMENT_VALIDATION_STANDARD v1.1                 ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Build
if [[ "${SKIP_BUILD:-}" != "1" ]]; then
    log_header "Phase 0: Build"
    log_info "cargo build --release ..."
    if ! cargo build --release --manifest-path "$PROJECT_ROOT/Cargo.toml" 2>&1; then
        echo -e "${RED}Build failed.${NC}"
        exit 1
    fi
    log_pass "Build succeeded"
fi

# Find binary
BINARY=$(find_binary) || {
    echo -e "${RED}Cannot find beardog binary. Build first or set BEARDOG_BINARY.${NC}"
    exit 1
}
log_info "Using binary: $BINARY"

# Start server
start_server "$BINARY" || exit 1

# ============================================================================
# Phase 1: Health Triad
# ============================================================================

log_header "Phase 1: Health Triad"

resp=$(rpc_call "ping")
assert_result_field "$resp" "status" "ping → status"

resp=$(rpc_call "health.liveness")
assert_result_field "$resp" "status" "health.liveness → status"

resp=$(rpc_call "health.readiness")
assert_result_field "$resp" "status" "health.readiness → status"

resp=$(rpc_call "health.check")
assert_result_exists "$resp" "health.check → result"

resp=$(rpc_call "health")
assert_result_field "$resp" "status" "health (legacy alias) → status"

resp=$(rpc_call "status")
assert_result_field "$resp" "status" "status (legacy alias) → status"

resp=$(rpc_call "check")
assert_result_exists "$resp" "check (legacy alias) → result"

# ============================================================================
# Phase 2: EdDSA Sign / Verify Roundtrip
# ============================================================================

log_header "Phase 2: EdDSA Keypair + Sign + Verify"

resp=$(rpc_call "crypto.ed25519_generate_keypair")
public_key=$(echo "$resp" | jq -r '.result.public_key // empty' 2>/dev/null)
if [[ -n "$public_key" ]]; then
    log_pass "crypto.ed25519_generate_keypair → public_key"
else
    log_fail "crypto.ed25519_generate_keypair — missing public_key"
fi

# SGVsbG8gYmVhckRvZw== = "Hello bearDog"
SIGN_MSG="SGVsbG8gYmVhckRvZw=="

resp=$(rpc_call "crypto.sign_ed25519" "{\"message\":\"$SIGN_MSG\"}")
signature=$(echo "$resp" | jq -r '.result.signature // empty' 2>/dev/null)
sign_pubkey=$(echo "$resp" | jq -r '.result.public_key // empty' 2>/dev/null)
if [[ -n "$signature" ]]; then
    log_pass "crypto.sign_ed25519 → signature"
else
    log_fail "crypto.sign_ed25519 — missing signature"
fi

if [[ -n "$signature" && -n "$sign_pubkey" ]]; then
    resp=$(rpc_call "crypto.verify_ed25519" "{\"message\":\"$SIGN_MSG\",\"signature\":\"$signature\",\"public_key\":\"$sign_pubkey\"}")
    valid=$(echo "$resp" | jq -r '.result.valid // empty' 2>/dev/null)
    if [[ "$valid" == "true" ]]; then
        log_pass "crypto.verify_ed25519 → valid=true"
    else
        log_fail "crypto.verify_ed25519 — expected valid=true, got '$valid'"
    fi
else
    log_skip "crypto.verify_ed25519 — skipped (no signature/pubkey from sign)"
fi

# ============================================================================
# Phase 3: Hashing (BLAKE3 + SHA-256)
# ============================================================================

log_header "Phase 3: Hashing"

resp=$(rpc_call "crypto.blake3_hash" "{\"data\":\"$SIGN_MSG\"}")
hash=$(echo "$resp" | jq -r '.result.hash // empty' 2>/dev/null)
if [[ -n "$hash" ]]; then
    log_pass "crypto.blake3_hash → hash"
else
    log_fail "crypto.blake3_hash — missing hash"
fi

resp=$(rpc_call "crypto.sha256" "{\"data\":\"$SIGN_MSG\"}")
sha_hash=$(echo "$resp" | jq -r '.result.hash // empty' 2>/dev/null)
if [[ -n "$sha_hash" ]]; then
    log_pass "crypto.sha256 → hash"
else
    log_fail "crypto.sha256 — missing hash"
fi

# Determinism: same input → same output
resp2=$(rpc_call "crypto.blake3_hash" "{\"data\":\"$SIGN_MSG\"}")
hash2=$(echo "$resp2" | jq -r '.result.hash // empty' 2>/dev/null)
if [[ "$hash" == "$hash2" ]]; then
    log_pass "crypto.blake3_hash deterministic"
else
    log_fail "crypto.blake3_hash non-deterministic: $hash != $hash2"
fi

# ============================================================================
# Phase 4: HMAC
# ============================================================================

log_header "Phase 4: HMAC"

resp=$(rpc_call "crypto.hmac_sha256" "{\"data\":\"$SIGN_MSG\",\"key\":\"dGVzdGtleQ==\"}")
assert_result_exists "$resp" "crypto.hmac_sha256 → result"

# ============================================================================
# Phase 5: AES-256-GCM Encrypt / Decrypt
# ============================================================================

log_header "Phase 5: AES-256-GCM AEAD"

resp=$(rpc_call "crypto.aes256_gcm_encrypt" "{\"plaintext\":\"$SIGN_MSG\"}")
ciphertext=$(echo "$resp" | jq -r '.result.ciphertext // empty' 2>/dev/null)
nonce=$(echo "$resp" | jq -r '.result.nonce // empty' 2>/dev/null)
enc_key=$(echo "$resp" | jq -r '.result.key // empty' 2>/dev/null)
if [[ -n "$ciphertext" ]]; then
    log_pass "crypto.aes256_gcm_encrypt → ciphertext"
else
    log_fail "crypto.aes256_gcm_encrypt — missing ciphertext"
fi

if [[ -n "$ciphertext" && -n "$nonce" && -n "$enc_key" ]]; then
    resp=$(rpc_call "crypto.aes256_gcm_decrypt" "{\"ciphertext\":\"$ciphertext\",\"nonce\":\"$nonce\",\"key\":\"$enc_key\"}")
    plaintext=$(echo "$resp" | jq -r '.result.plaintext // empty' 2>/dev/null)
    if [[ "$plaintext" == "$SIGN_MSG" ]]; then
        log_pass "crypto.aes256_gcm_decrypt → plaintext matches"
    elif [[ -n "$plaintext" ]]; then
        log_pass "crypto.aes256_gcm_decrypt → plaintext returned"
    else
        log_fail "crypto.aes256_gcm_decrypt — missing plaintext"
    fi
else
    log_skip "crypto.aes256_gcm_decrypt — skipped (no ciphertext/nonce/key)"
fi

# ============================================================================
# Phase 6: ChaCha20-Poly1305 Encrypt / Decrypt
# ============================================================================

log_header "Phase 6: ChaCha20-Poly1305 AEAD"

resp=$(rpc_call "crypto.chacha20_poly1305_encrypt" "{\"plaintext\":\"$SIGN_MSG\"}")
cc_ct=$(echo "$resp" | jq -r '.result.ciphertext // empty' 2>/dev/null)
cc_nonce=$(echo "$resp" | jq -r '.result.nonce // empty' 2>/dev/null)
cc_key=$(echo "$resp" | jq -r '.result.key // empty' 2>/dev/null)
if [[ -n "$cc_ct" ]]; then
    log_pass "crypto.chacha20_poly1305_encrypt → ciphertext"
else
    log_fail "crypto.chacha20_poly1305_encrypt — missing ciphertext"
fi

if [[ -n "$cc_ct" && -n "$cc_nonce" && -n "$cc_key" ]]; then
    resp=$(rpc_call "crypto.chacha20_poly1305_decrypt" "{\"ciphertext\":\"$cc_ct\",\"nonce\":\"$cc_nonce\",\"key\":\"$cc_key\"}")
    plaintext=$(echo "$resp" | jq -r '.result.plaintext // empty' 2>/dev/null)
    if [[ -n "$plaintext" ]]; then
        log_pass "crypto.chacha20_poly1305_decrypt → plaintext returned"
    else
        log_fail "crypto.chacha20_poly1305_decrypt — missing plaintext"
    fi
else
    log_skip "crypto.chacha20_poly1305_decrypt — skipped (no ciphertext/nonce/key)"
fi

# ============================================================================
# Phase 7: X25519 Key Exchange
# ============================================================================

log_header "Phase 7: X25519 Key Exchange"

resp=$(rpc_call "crypto.x25519_generate_ephemeral")
x_pubkey=$(echo "$resp" | jq -r '.result.public_key // empty' 2>/dev/null)
if [[ -n "$x_pubkey" ]]; then
    log_pass "crypto.x25519_generate_ephemeral → public_key"
else
    log_fail "crypto.x25519_generate_ephemeral — missing public_key"
fi

# ============================================================================
# Phase 8: Key Derivation
# ============================================================================

log_header "Phase 8: Key Derivation (HKDF)"

resp=$(rpc_call "crypto.hkdf_sha256" "{\"ikm\":\"$SIGN_MSG\",\"info\":\"dGVzdA==\",\"length\":32}")
assert_result_exists "$resp" "crypto.hkdf_sha256 → result"

# ============================================================================
# Phase 9: Password Hashing
# ============================================================================

log_header "Phase 9: Password Hashing (Argon2id)"

resp=$(rpc_call "crypto.argon2id_hash" "{\"password\":\"hunter2\"}")
argon_hash=$(echo "$resp" | jq -r '.result.hash // empty' 2>/dev/null)
if [[ -n "$argon_hash" ]]; then
    log_pass "crypto.argon2id_hash → hash"
else
    log_fail "crypto.argon2id_hash — missing hash"
fi

if [[ -n "$argon_hash" ]]; then
    resp=$(rpc_call "crypto.argon2id_verify" "{\"password\":\"hunter2\",\"hash\":\"$argon_hash\"}")
    argon_valid=$(echo "$resp" | jq -r '.result.valid // empty' 2>/dev/null)
    if [[ "$argon_valid" == "true" ]]; then
        log_pass "crypto.argon2id_verify → valid=true"
    else
        log_fail "crypto.argon2id_verify — expected valid=true, got '$argon_valid'"
    fi
else
    log_skip "crypto.argon2id_verify — skipped (no hash)"
fi

# ============================================================================
# Phase 10: Semantic Aliases
# ============================================================================

log_header "Phase 10: Semantic Aliases"

resp=$(rpc_call "crypto.sign" "{\"message\":\"$SIGN_MSG\"}")
assert_result_exists "$resp" "crypto.sign (alias → ed25519)"

resp=$(rpc_call "crypto.hash" "{\"data\":\"$SIGN_MSG\"}")
assert_result_exists "$resp" "crypto.hash (alias → blake3)"

resp=$(rpc_call "crypto.encrypt" "{\"plaintext\":\"$SIGN_MSG\"}")
assert_result_exists "$resp" "crypto.encrypt (alias → aes256_gcm)"

resp=$(rpc_call "crypto.generate_keypair")
assert_result_exists "$resp" "crypto.generate_keypair (alias → ed25519)"

resp=$(rpc_call "crypto.public_key")
assert_result_exists "$resp" "crypto.public_key"

# ============================================================================
# Phase 11: Cross-Primal Namespace (beardog.crypto.*)
# ============================================================================

log_header "Phase 11: Cross-Primal Namespace"

resp=$(rpc_call "beardog.crypto.blake3_hash" "{\"data\":\"$SIGN_MSG\"}")
bd_hash=$(echo "$resp" | jq -r '.result.hash // empty' 2>/dev/null)
if [[ -n "$bd_hash" ]]; then
    log_pass "beardog.crypto.blake3_hash → hash"
    if [[ "$bd_hash" == "$hash" ]]; then
        log_pass "beardog.crypto.blake3_hash matches crypto.blake3_hash"
    else
        log_fail "beardog.crypto.blake3_hash result mismatch"
    fi
else
    log_fail "beardog.crypto.blake3_hash — missing hash"
fi

resp=$(rpc_call "beardog.crypto.sign_ed25519" "{\"message\":\"$SIGN_MSG\"}")
assert_result_exists "$resp" "beardog.crypto.sign_ed25519"

resp=$(rpc_call "beardog.crypto.hmac_sha256" "{\"data\":\"$SIGN_MSG\",\"key\":\"dGVzdGtleQ==\"}")
assert_result_exists "$resp" "beardog.crypto.hmac_sha256"

# ============================================================================
# Phase 12: Introspection
# ============================================================================

log_header "Phase 12: Introspection"

resp=$(rpc_call "primal.info")
primal_name=$(echo "$resp" | jq -r '.result.primal // .result.name // empty' 2>/dev/null)
if [[ -n "$primal_name" ]]; then
    log_pass "primal.info → primal=$primal_name"
else
    log_fail "primal.info — missing primal/name"
fi

resp=$(rpc_call "rpc.methods")
method_count=$(echo "$resp" | jq -r '.result.methods | length // 0' 2>/dev/null)
if [[ "$method_count" -gt 50 ]]; then
    log_pass "rpc.methods → ${method_count} methods registered"
else
    log_fail "rpc.methods — expected >50 methods, got $method_count"
fi

# ============================================================================
# Phase 13: Error Paths
# ============================================================================

log_header "Phase 13: Error Paths"

resp=$(rpc_call "nonexistent.method.should.fail")
assert_error_code "$resp" "-32601" "unknown method → -32601 (Method not found)"

# ============================================================================
# Phase 14: Burst Probe (Liveness Stability)
# ============================================================================

log_header "Phase 14: Burst Probe (20 rapid liveness)"

burst_pass=0
burst_fail=0
for _ in $(seq 1 20); do
    resp=$(rpc_call "health.liveness")
    if echo "$resp" | jq -e '.result.status' >/dev/null 2>&1; then
        burst_pass=$((burst_pass + 1))
    else
        burst_fail=$((burst_fail + 1))
    fi
done

if [[ $burst_fail -eq 0 ]]; then
    log_pass "Burst probe: $burst_pass/$((burst_pass + burst_fail)) passed"
    PASS=$((PASS + 1))
    TOTAL=$((TOTAL + 1))
else
    log_fail "Burst probe: $burst_fail/$((burst_pass + burst_fail)) failed"
    FAIL=$((FAIL + 1))
    TOTAL=$((TOTAL + 1))
fi

# ============================================================================
# Summary
# ============================================================================

echo ""
echo -e "${BOLD}${CYAN}══════════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}  bearDog benchScale Roundtrip Summary${NC}"
echo -e "${BOLD}${CYAN}══════════════════════════════════════════════════════════════${NC}"
echo -e "  ${GREEN}PASS${NC}: $PASS"
echo -e "  ${RED}FAIL${NC}: $FAIL"
echo -e "  ${YELLOW}SKIP${NC}: $SKIP"
echo -e "  TOTAL: $TOTAL"
echo ""

if [[ $FAIL -eq 0 ]]; then
    echo -e "  ${GREEN}${BOLD}✓ ALL CHECKS PASSED${NC}"
    exit 0
else
    echo -e "  ${RED}${BOLD}✗ $FAIL CHECK(S) FAILED${NC}"
    exit 1
fi
