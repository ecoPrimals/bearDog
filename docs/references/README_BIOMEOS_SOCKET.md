# 🔌 BearDog Socket Configuration

**For**: biomeOS Integration  
**Date**: January 30, 2026  
**Status**: ✅ **IMPLEMENTED**

---

## Quick Reference

### Default Socket Location (biomeOS Standard)

```
/run/user/$UID/biomeos/beardog.sock
```

### Environment Variables (Priority Order)

| Variable | Priority | Example | Notes |
|----------|----------|---------|-------|
| `BEARDOG_SOCKET` | **1 (highest)** | `/custom/beardog.sock` | Primal-specific override |
| `BIOMEOS_SOCKET_PATH` | **2** | `/run/biomeos/beardog.sock` | Full socket path |
| `BIOMEOS_SOCKET_DIR` | **2** | `/run/biomeos/` | Socket directory (adds `beardog.sock`) |
| *(XDG default)* | **4** | `/run/user/1000/biomeos/beardog.sock` | Auto-discovered |

---

## Usage Examples

### Default (biomeOS Standard)

```bash
# No configuration needed - uses XDG standard
./beardog server

# Socket created at: /run/user/1000/biomeos/beardog.sock
```

### Custom Path

```bash
# Override with specific socket path
BEARDOG_SOCKET=/custom/beardog.sock ./beardog server
```

### Shared biomeOS Directory

```bash
# Use custom biomeOS directory
BIOMEOS_SOCKET_DIR=/var/run/biomeos ./beardog server

# Socket created at: /var/run/biomeos/beardog.sock
```

### Full Path Override

```bash
# Specify complete socket path
BIOMEOS_SOCKET_PATH=/opt/sockets/beardog.sock ./beardog server
```

---

## biomeOS Integration

### Discovery

biomeOS can discover BearDog at:

1. `$BEARDOG_SOCKET` (if set)
2. `/run/user/$UID/biomeos/beardog.sock` (XDG standard)
3. `/tmp/beardog-*.sock` (fallback scan)

### Socket Directory Structure

```
/run/user/1000/biomeos/
├── beardog.sock      ← BearDog crypto service
├── songbird.sock     ← Songbird TLS service
├── nestgate.sock     ← NestGate networking
├── toadstool.sock    ← ToadStool workflows
└── squirrel.sock     ← Squirrel routing
```

---

## Verification

### Check Socket Exists

```bash
ls -la /run/user/$(id -u)/biomeos/beardog.sock
# Expected: srwxrwxrwx ... beardog.sock
```

### Test Connection

```bash
echo '{"jsonrpc":"2.0","method":"health","id":1}' | \
  nc -U /run/user/$(id -u)/biomeos/beardog.sock

# Expected: {"jsonrpc":"2.0","result":{"status":"healthy"},"id":1}
```

---

## See Also

- **[ENVIRONMENT_VARIABLES.md](ENVIRONMENT_VARIABLES.md)** - Complete environment variable reference (including socket path resolution)
- **[README.md](../../README.md)** - BearDog project overview
- **[START_HERE.md](../../START_HERE.md)** - Quick start guide

---

**Status**: ✅ Implemented and tested  
**Compatible**: biomeOS NUCLEUS integration ready

🦀 **TRUE PRIMAL socket discovery** 🦀
