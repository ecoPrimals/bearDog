# BearDog JWT Secret Generation - Quick Reference

**Date**: January 16, 2026  
**Status**: ✅ Production Ready

---

## 📡 **JSON-RPC Request**

```json
{
  "jsonrpc": "2.0",
  "method": "beardog.generate_jwt_secret",
  "params": {
    "purpose": "nestgate_authentication",
    "strength": "high"
  },
  "id": 1
}
```

## 📥 **JSON-RPC Response**

```json
{
  "jsonrpc": "2.0",
  "result": {
    "secret": "A7k9x... (88-character base64 string)",
    "purpose": "nestgate_authentication",
    "strength": "high",
    "byte_length": 64,
    "encoded_length": 88,
    "algorithm": "CSPRNG",
    "provider": "beardog",
    "generated_at": "2026-01-16T12:34:56.789Z"
  },
  "id": 1
}
```

---

## 🎚️ **Strength Levels**

| Strength | Bytes | Base64 Length | Bits | Recommended Use |
|----------|-------|---------------|------|-----------------|
| `high`   | 64    | 88+           | 512  | **Production** ✅ |
| `medium` | 48    | 64+           | 384  | Standard apps |
| `low`    | 32    | 44+           | 256  | Dev/testing only |

**Default**: `high` (recommended for all production use)

---

## 🔌 **Connection**

**Unix Socket**: `/tmp/beardog-{family_id}-{node_id}.sock`

Example: `/tmp/beardog-default-default.sock`

---

## 🧪 **Quick Test**

```bash
# Test JWT secret generation
echo '{"jsonrpc":"2.0","method":"beardog.generate_jwt_secret","params":{"purpose":"test","strength":"high"},"id":1}' \
  | socat - UNIX-CONNECT:/tmp/beardog-default-default.sock
```

Expected output: JSON response with 88-character base64 `secret` field

---

## 🎯 **Method Aliases**

All these work identically:
- `beardog.generate_jwt_secret` (recommended)
- `security.generate_jwt_secret`
- `beardog.jwt_secret`
- `security.jwt_secret`

---

## 🔍 **Capability Discovery**

```json
{
  "jsonrpc": "2.0",
  "method": "capabilities.list",
  "id": 1
}
```

Look for:
```json
{
  "type": "jwt_secrets",
  "version": "1.0",
  "methods": ["generate_jwt_secret"],
  "description": "JWT secret generation for authentication systems"
}
```

---

## ✅ **Success Checklist for bioemOS Team**

1. ✅ BearDog server running  
2. ✅ Unix socket exists at `/tmp/beardog-*.sock`  
3. ✅ Neural API sends JSON-RPC request  
4. ✅ BearDog returns secret (not "Method not found")  
5. ✅ NestGate receives and uses secret  
6. ✅ NestGate starts successfully

---

## 🚨 **Troubleshooting**

| Issue | Solution |
|-------|----------|
| "Method not found" | Update to BearDog v0.9.0+ |
| Socket not found | Check BearDog is running |
| Secret too short | Use `"strength": "high"` |
| Connection refused | Verify socket path is correct |

---

## 📚 **Full Documentation**

See `JWT_SECRET_GENERATION_COMPLETE.md` for:
- Implementation details
- Security analysis
- Test coverage
- Integration examples
- Future enhancements

---

**Status**: 🟢 **READY FOR PRODUCTION**  
**Tests**: ✅ 6/6 passing  
**Integration**: ✅ Neural API compatible

