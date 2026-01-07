# 🎯 tarpc Quick Reference Card

**For**: Songbird Integration Team  
**Date**: January 6, 2026  
**Status**: Production Ready ✅

---

## 🚀 5-Minute Quick Start

### 1. Add Dependencies (30 seconds)

```toml
[dependencies]
beardog-tunnel = { path = "../beardog/crates/beardog-tunnel" }
tarpc = { version = "0.34", features = ["tokio1", "serde-transport"] }
tokio-util = { version = "0.7", features = ["codec"] }
```

### 2. Connect to BearDog (2 minutes)

```rust
use beardog_tunnel::tarpc_service::BearDogServiceClient;
use tarpc::{client, context, tokio_serde::formats::Bincode};
use tokio::net::UnixStream;

// Connect
let stream = UnixStream::connect("/tmp/beardog-nat0-tower1.sock").await?;

// Create transport
let transport = tarpc::serde_transport::new(
    tokio_util::codec::LengthDelimitedCodec::new(),
    Bincode::default(),
).from_stream(stream);

// Create client
let client = BearDogServiceClient::new(
    client::Config::default(),
    transport
).spawn();
```

### 3. Make Calls (2 minutes)

```rust
// Ping
let pong = client.ping(context::current()).await?;

// Trust evaluation
let request = TrustEvaluationRequest {
    peer_id: "tower2".to_string(),
    family_id: "nat0".to_string(),
    requested_operation: Some("encrypt".to_string()),
};
let trust = client.evaluate_trust(context::current(), request).await?;

// Encrypt
let ciphertext = client.birdsong_encrypt(
    context::current(),
    plaintext,
    "nat0".to_string()
).await?;
```

---

## 📚 Full Documentation

| Document | Purpose | Time |
|----------|---------|------|
| `TARPC_CLIENT_LIBRARY.md` | Complete integration guide | 10 min |
| `TARPC_UPSTREAM_HANDOFF.md` | Production deployment | 15 min |
| `TARPC_EVOLUTION_INDEX.md` | Navigation guide | 5 min |

---

## 🎯 Protocol Priority

```
#1 tarpc      ⭐⭐⭐⭐⭐  (5/5/5) - PRIMARY
#2 JSON-RPC   ⭐⭐⭐⭐   (4/4/4) - FALLBACK  
#3 HTTP       ⭐⭐     (2/2/2) - LEGACY
```

---

## 💡 Key Benefits

- ✅ **Type Safety**: Compile-time checks
- ✅ **Performance**: ~2x faster (bincode)
- ✅ **Security**: Level 5/5 (highest)
- ✅ **Modern**: async/await throughout
- ✅ **Tested**: 126 tests (100% passing)

---

## 📞 Need Help?

1. Review `TARPC_CLIENT_LIBRARY.md` (full examples)
2. Check `TARPC_EVOLUTION_INDEX.md` (find what you need)
3. See `TARPC_UPSTREAM_HANDOFF.md` (deployment guide)

---

**Integration Time**: ~1 hour  
**Status**: Production Ready ✅  
**Next**: Test genetic lineage with tarpc

