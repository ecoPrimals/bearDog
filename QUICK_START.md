# 🚀 QUICK START - BearDog Server v0.15.0

**Status**: ✅ READY TO RUN  
**Location**: `/home/eastgate/Development/ecoPrimals/phase1/beardog`

---

## 🎯 ONE COMMAND TO START

```bash
./start-beardog-server.sh
```

**Default**: Starts on `http://127.0.0.1:9000`

---

## ✅ VERIFY IT'S RUNNING

```bash
curl http://127.0.0.1:9000/health
```

Expected: `{"status":"healthy","version":"0.15.0",...}`

---

## 🎵 TEST V2 API (SONGBIRD NEEDS THIS)

```bash
# Encrypt
curl -X POST http://127.0.0.1:9000/api/v2/birdsong/encrypt \
  -H "Content-Type: application/json" \
  -d '{"plaintext":"dGVzdA=="}'

# Decrypt
curl -X POST http://127.0.0.1:9000/api/v2/birdsong/decrypt \
  -H "Content-Type: application/json" \
  -d '{"ciphertext":"<from_above>"}'
```

---

## 🐦 START SONGBIRD

```bash
cd /path/to/songbird
SONGBIRD_BEARDOG_ENDPOINT="http://127.0.0.1:9000" cargo run
```

---

## 🎊 WHAT TO LOOK FOR

### In BearDog Logs:
```
✅ HSM Manager initialized
✅ Genetic Engine initialized
🚀 BearDog Server Starting on 127.0.0.1:9000
🎵 BirdSong v2 encrypt for family: ...
✅ BirdSong v2 encrypted successfully
```

### In Songbird Logs:
```
✅ BearDog BirdSong provider initialized
📡 Broadcasting encrypted discovery with genetic lineage
🎵 Received encrypted discovery packet
✅ Decrypted successfully - same family detected!
🤝 Auto-trust: Limited trust granted based on lineage
```

---

## 🔧 CONFIGURATION

### Custom Port:
```bash
BEARDOG_BIND_ADDR="127.0.0.1:8080" ./start-beardog-server.sh
```

### Debug Logs:
```bash
RUST_LOG="debug" ./start-beardog-server.sh
```

---

## 📚 FULL DOCUMENTATION

- **`BEARDOG_SERVER_V2_API_READY.md`** - Complete guide
- **`SOLUTION_COMPLETE_V2_API_READY.md`** - How we solved it
- **`BEARDOG_V2_API_DEPLOYMENT_SUCCESS.md`** - Success details

---

## ✨ FILES

- **Binary**: `primalBins/beardog-server-v0.15.0-with-v2-api` (6.1MB)
- **Startup**: `start-beardog-server.sh`
- **Source**: `beardog-server.rs`

---

**🎉 Ready for the first genetic federation! 🐻🐕🎵🐦**

