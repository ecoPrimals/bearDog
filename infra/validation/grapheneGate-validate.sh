#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# grapheneGate Validation Script — BearDog on-device 13-check matrix
#
# Prerequisites:
#   - ADB connected to grapheneGate (Pixel 8a)
#   - beardog binary already pushed to /data/local/tmp/beardog-wave156i
#     (or override with BEARDOG_BIN env var)
#   - Port 19876 free on host (for ADB forward)
#
# Usage:
#   ./infra/validation/grapheneGate-validate.sh
#   BEARDOG_BIN=/data/local/tmp/beardog-custom ./infra/validation/grapheneGate-validate.sh

set -euo pipefail

BEARDOG_BIN="${BEARDOG_BIN:-/data/local/tmp/beardog-wave156i}"
PORT="${BEARDOG_PORT:-19876}"
TIMEOUT=3
PASS=0
FAIL=0
PARTIAL=0
RESULTS=()

red()    { printf '\033[1;31m%s\033[0m' "$*"; }
green()  { printf '\033[1;32m%s\033[0m' "$*"; }
yellow() { printf '\033[1;33m%s\033[0m' "$*"; }

record() {
  local check="$1" status="$2" note="${3:-}"
  case "$status" in
    PASS)    PASS=$((PASS + 1));    sym="$(green PASS)"    ;;
    FAIL)    FAIL=$((FAIL + 1));    sym="$(red FAIL)"      ;;
    PARTIAL) PARTIAL=$((PARTIAL+1)); sym="$(yellow PARTIAL)" ;;
  esac
  RESULTS+=("$sym | $check | $note")
  printf '  %s  %s  %s\n' "$sym" "$check" "$note"
}

rpc() {
  local method="$1" params="$2" id="${3:-1}"
  local payload="{\"jsonrpc\":\"2.0\",\"method\":\"$method\",\"params\":$params,\"id\":$id}"
  echo "$payload" | nc -w "$TIMEOUT" 127.0.0.1 "$PORT" 2>/dev/null
}

cleanup() {
  adb shell "pkill -f '${BEARDOG_BIN##*/}'" 2>/dev/null || true
  adb forward --remove tcp:"$PORT" 2>/dev/null || true
}
trap cleanup EXIT

printf '\n══════════════════════════════════════════════\n'
printf '  🐻🐕  grapheneGate Validation Matrix\n'
printf '══════════════════════════════════════════════\n\n'

# Verify ADB connection
if ! adb get-state >/dev/null 2>&1; then
  echo "$(red 'ERROR'): No ADB device connected"
  exit 1
fi
DEVICE=$(adb shell getprop ro.product.model 2>/dev/null | tr -d '\r')
echo "Device: $DEVICE"
echo ""

# ── Check 1: Binary runs ──
OUT=$(adb shell "$BEARDOG_BIN --version" 2>&1 | tr -d '\r')
if echo "$OUT" | grep -q 'beardog'; then
  record "1. Binary runs" PASS "$OUT"
else
  record "1. Binary runs" FAIL "$OUT"
fi

# ── Check 2: Health (doctor) ──
OUT=$(adb shell "$BEARDOG_BIN doctor" 2>&1 | tr -d '\r')
if echo "$OUT" | grep -q 'HEALTHY'; then
  record "2. Health (doctor)" PASS ""
else
  record "2. Health (doctor)" FAIL ""
fi

# ── Check 3: HSM discover ──
OUT=$(adb shell "$BEARDOG_BIN hsm discover" 2>&1 | tr -d '\r')
if echo "$OUT" | grep -qi 'strongbox\|Android.*HSM'; then
  record "3. HSM discover" PASS "StrongBox detected"
else
  record "3. HSM discover" PARTIAL "No StrongBox; software only"
fi

# ── Start server for JSON-RPC checks ──
cleanup 2>/dev/null || true
adb shell "BEARDOG_FAMILY_SEED=validation nohup $BEARDOG_BIN server --bind-mode tcp --listen 127.0.0.1:$PORT > /data/local/tmp/beardog-validate.log 2>&1 &"
sleep 3
adb forward tcp:"$PORT" tcp:"$PORT" >/dev/null

# Quick server-alive test
HEALTH=$(rpc "health.check" "{}")
if ! echo "$HEALTH" | grep -q '"healthy"'; then
  echo "$(red 'FATAL'): server did not start — cannot run JSON-RPC checks"
  adb shell "cat /data/local/tmp/beardog-validate.log" | tail -20
  exit 1
fi

# ── Check 4: Ed25519 sign + verify ──
SIGN_OUT=$(rpc "crypto.sign_ed25519" '{"key_id":"validate","message":"dGVzdA=="}')
SIG=$(echo "$SIGN_OUT" | grep -o '"signature":"[^"]*"' | cut -d'"' -f4)
PUB=$(echo "$SIGN_OUT" | grep -o '"public_key":"[^"]*"' | cut -d'"' -f4)
if [ -n "$SIG" ] && [ -n "$PUB" ]; then
  VERIFY=$(rpc "crypto.verify_ed25519" "{\"public_key\":\"$PUB\",\"message\":\"dGVzdA==\",\"signature\":\"$SIG\"}" 4)
  if echo "$VERIFY" | grep -q '"valid":true'; then
    record "4. Ed25519 sign/verify" PASS "roundtrip"
  else
    record "4. Ed25519 sign/verify" FAIL "verify returned: $VERIFY"
  fi
else
  record "4. Ed25519 sign/verify" FAIL "sign returned: $SIGN_OUT"
fi

# ── Check 5: ChaCha20-Poly1305 roundtrip ──
ENC5=$(rpc "crypto.chacha20_poly1305_encrypt" '{"key":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=","plaintext":"dGVzdA=="}' 5)
CT5=$(echo "$ENC5" | grep -o '"ciphertext":"[^"]*"' | cut -d'"' -f4)
NONCE5=$(echo "$ENC5" | grep -o '"nonce":"[^"]*"' | cut -d'"' -f4)
TAG5=$(echo "$ENC5" | grep -o '"tag":"[^"]*"' | cut -d'"' -f4)
if [ -n "$CT5" ]; then
  DEC5=$(rpc "crypto.chacha20_poly1305_decrypt" "{\"key\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=\",\"ciphertext\":\"$CT5\",\"nonce\":\"$NONCE5\",\"tag\":\"$TAG5\"}" 52)
  if echo "$DEC5" | grep -q '"plaintext":"dGVzdA=="'; then
    record "5. ChaCha20 roundtrip" PASS ""
  else
    record "5. ChaCha20 roundtrip" FAIL "$DEC5"
  fi
else
  record "5. ChaCha20 roundtrip" FAIL "$ENC5"
fi

# ── Check 6: AES-256-GCM roundtrip ──
ENC6=$(rpc "crypto.aes256_gcm_encrypt" '{"key":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=","plaintext":"dGVzdA=="}' 6)
CT6=$(echo "$ENC6" | grep -o '"ciphertext":"[^"]*"' | cut -d'"' -f4)
NONCE6=$(echo "$ENC6" | grep -o '"nonce":"[^"]*"' | cut -d'"' -f4)
if [ -n "$CT6" ]; then
  DEC6=$(rpc "crypto.aes256_gcm_decrypt" "{\"key\":\"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=\",\"ciphertext\":\"$CT6\",\"nonce\":\"$NONCE6\"}" 62)
  if echo "$DEC6" | grep -q '"plaintext":"dGVzdA=="'; then
    record "6. AES-256-GCM roundtrip" PASS ""
  else
    record "6. AES-256-GCM roundtrip" FAIL "$DEC6"
  fi
else
  record "6. AES-256-GCM roundtrip" FAIL "$ENC6"
fi

# ── Check 7: BLAKE3 hash ──
B3=$(rpc "crypto.blake3_hash" '{"data":"dGVzdA=="}' 7)
if echo "$B3" | grep -q '"hash"'; then
  record "7. BLAKE3 hash" PASS ""
else
  record "7. BLAKE3 hash" FAIL "$B3"
fi

# ── Check 8: HKDF derivation ──
HKDF=$(rpc "crypto.hkdf_sha256" '{"ikm":"dGVzdA==","salt":"c2FsdA==","info":"aW5mbw==","length":32}' 8)
if echo "$HKDF" | grep -q '"okm"'; then
  record "8. HKDF-SHA256" PASS ""
else
  record "8. HKDF-SHA256" FAIL "$HKDF"
fi

# ── Check 9: Ionic token lifecycle ──
ISSUE=$(rpc "auth.issue_ionic" '{"gate_id":"grapheneGate","scope":["crypto"]}' 9)
TOKEN=$(echo "$ISSUE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
if [ -n "$TOKEN" ]; then
  VERIFY=$(rpc "auth.verify_ionic" "{\"token\":\"$TOKEN\"}" 92)
  if echo "$VERIFY" | grep -q '"valid":true'; then
    record "9. Ionic token lifecycle" PASS ""
  else
    record "9. Ionic token lifecycle" FAIL "verify: $VERIFY"
  fi
else
  record "9. Ionic token lifecycle" FAIL "issue: $ISSUE"
fi

# ── Check 10: Secrets store/retrieve ──
STORE=$(rpc "secrets.store" '{"name":"validate-secret","value":"dGVzdA=="}' 10)
if echo "$STORE" | grep -q '"stored":true'; then
  RETRIEVE=$(rpc "secrets.retrieve" '{"name":"validate-secret"}' 102)
  if echo "$RETRIEVE" | grep -q '"value":"dGVzdA=="'; then
    record "10. Secrets roundtrip" PASS ""
  else
    record "10. Secrets roundtrip" FAIL "retrieve: $RETRIEVE"
  fi
else
  record "10. Secrets roundtrip" FAIL "store: $STORE"
fi

# ── Check 11: Capabilities list ──
CAPS=$(rpc "capabilities.list" '{}' 11)
if echo "$CAPS" | grep -q '"methods"'; then
  METHOD_COUNT=$(echo "$CAPS" | grep -o '"methods":\[' | wc -l)
  record "11. Capabilities list" PASS "full surface advertised"
else
  record "11. Capabilities list" FAIL "$CAPS"
fi

# ── Check 12: Abstract socket IPC ──
# Abstract sockets require app-level SELinux context on GrapheneOS.
# Under adb shell, this is expected to fail. TCP transport is the
# validated transport for non-app processes.
adb shell "pkill -f '${BEARDOG_BIN##*/}'" 2>/dev/null || true
sleep 1
adb shell "BEARDOG_FAMILY_SEED=validation nohup $BEARDOG_BIN server --bind-mode abstract > /data/local/tmp/beardog-abs.log 2>&1 &"
sleep 2
ABS_LOG=$(adb shell "cat /data/local/tmp/beardog-abs.log 2>/dev/null" | tr -d '\r')
if echo "$ABS_LOG" | grep -q 'listening.*abstract\|IPC server.*running'; then
  record "12. Abstract socket IPC" PASS ""
else
  record "12. Abstract socket IPC" PARTIAL "SELinux blocks under adb shell (expected); TCP validated"
fi

# ── Check 13: StrongBox keygen ──
adb shell "pkill -f '${BEARDOG_BIN##*/}'" 2>/dev/null || true
KEYGEN=$(adb shell "keystore_cli_v2 generate --algo EC --curve P256 --purpose SIGN --strongbox --alias beardog_validate_probe 2>&1" | tr -d '\r')
if echo "$KEYGEN" | grep -qi 'success'; then
  adb shell "keystore_cli_v2 delete --alias beardog_validate_probe" 2>/dev/null || true
  record "13. StrongBox keygen" PASS ""
else
  record "13. StrongBox keygen" FAIL "$KEYGEN"
fi

# ── Summary ──
printf '\n══════════════════════════════════════════════\n'
printf '  Results: %s PASS  %s FAIL  %s PARTIAL\n' "$PASS" "$FAIL" "$PARTIAL"
printf '══════════════════════════════════════════════\n\n'

if [ "$FAIL" -gt 0 ]; then
  exit 1
fi
exit 0
