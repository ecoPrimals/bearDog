# 🔒 BTSP Tunnel - Secure Encrypted Connections

**Level**: 0 (Local Primal)  
**Category**: Networking & Security  
**Time**: 15 minutes  
**Dependencies**: None (demonstrates concepts)

---

## 🎯 What This Demo Shows

Understanding **BTSP** (BearDog Secure Tunnel Protocol):
- ✅ End-to-end encryption
- ✅ Perfect Forward Secrecy (PFS)
- ✅ Mutual authentication
- ✅ Key establishment
- ✅ Secure data transfer

BTSP is BearDog's answer to TLS/VPN - built for sovereign, zero-trust networking.

---

## 🚀 Running the Demo

```bash
./run.sh
```

---

## 📊 Expected Output

```
🔒 BearDog - BTSP Tunnel Demo
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Understanding BTSP Protocol
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

BTSP = BearDog Secure Tunnel Protocol

Features:
  ✓ End-to-end encryption (ChaCha20-Poly1305)
  ✓ Perfect Forward Secrecy (ephemeral keys)
  ✓ Mutual authentication (both sides verified)
  ✓ Zero-trust (verify every connection)
  ✓ Sovereign (no CAs required)

vs TLS:
  + No Certificate Authorities needed
  + Ephemeral keys by default
  + Genetic key integration
  + Sovereign identity

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Step 1: Identity Setup
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Alice (Client):
  ✓ Generated identity key (Ed25519)
  ✓ Public key: alice_pub_abc123...
  ✓ Ready to connect

Bob (Server):
  ✓ Generated identity key (Ed25519)
  ✓ Public key: bob_pub_def456...
  ✓ Listening on port 9999

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Step 2: Handshake (Key Exchange)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Alice → Bob: ClientHello
  - Client identity: alice_pub_abc123...
  - Ephemeral key: ephemeral_client_xyz...
  - Supported ciphers: ChaCha20-Poly1305, AES-256-GCM

Bob → Alice: ServerHello
  - Server identity: bob_pub_def456...
  - Ephemeral key: ephemeral_server_uvw...
  - Selected cipher: ChaCha20-Poly1305

Alice verifies: Bob's signature ✓
Bob verifies: Alice's signature ✓

Shared secret established: [hidden]
Session keys derived (HKDF-SHA256):
  - Encryption key (Alice → Bob): [32 bytes]
  - Encryption key (Bob → Alice): [32 bytes]
  - MAC key (Alice → Bob): [32 bytes]
  - MAC key (Bob → Alice): [32 bytes]

✓ Handshake complete - Tunnel established!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Step 3: Secure Data Transfer
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Alice sends: "Hello Bob, this is a secret message!"
  Plaintext: 37 bytes
  Encrypted: 53 bytes (+ auth tag)
  → Bob receives and decrypts ✓

Bob sends: "Hello Alice, message received!"
  Plaintext: 31 bytes
  Encrypted: 47 bytes (+ auth tag)
  → Alice receives and decrypts ✓

Data integrity: ✓ Verified with Poly1305 MAC
Forward secrecy: ✓ Ephemeral keys will be destroyed

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Step 4: Tunnel Closure
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Alice: Closing tunnel...
  ✓ Ephemeral keys destroyed
  ✓ Session keys wiped from memory
  ✓ Connection closed gracefully

Bob: Tunnel closed
  ✓ Ephemeral keys destroyed
  ✓ No record of session keys

Perfect Forward Secrecy maintained!
  → Even if identity keys compromised later,
     this session cannot be decrypted

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Security Properties Verified
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✓ Confidentiality: Messages encrypted
✓ Integrity: MAC verified
✓ Authenticity: Both parties verified
✓ Forward Secrecy: Ephemeral keys destroyed
✓ Sovereignty: No CAs, no middlemen

🔒 BTSP: Sovereign Secure Tunnels!
```

---

## 🧠 Understanding BTSP

### What is BTSP?

**BTSP** = BearDog Secure Tunnel Protocol

A modern, sovereign alternative to TLS/VPNs:
- No Certificate Authorities required
- Built-in Perfect Forward Secrecy
- Integrates with genetic keys
- Zero-trust by default
- Sovereign identity

### vs TLS/SSL

| Feature | TLS/SSL | BTSP |
|---------|---------|------|
| **Identity** | X.509 Certs + CAs | Genetic Keys |
| **Forward Secrecy** | Optional (DHE) | Always |
| **Trust Model** | CA hierarchy | Zero-trust |
| **Sovereignty** | CA-dependent | Fully sovereign |
| **Complexity** | High | Moderate |

### Key Concepts

**1. Ephemeral Keys**: Generated for each session, destroyed after
**2. Perfect Forward Secrecy**: Past sessions safe even if keys compromised later
**3. Mutual Authentication**: Both client and server verify each other
**4. Zero-Trust**: Every connection is verified

---

## 🎓 Protocol Flow

### Handshake Phase
```
1. Alice generates ephemeral key pair
2. Bob generates ephemeral key pair
3. Exchange public keys + identities
4. Verify signatures
5. Compute shared secret (ECDH)
6. Derive session keys (HKDF)
7. Verify handshake integrity
```

### Data Transfer Phase
```
1. Encrypt with ChaCha20-Poly1305
2. Add authentication tag
3. Send encrypted message
4. Verify tag
5. Decrypt message
```

### Closure Phase
```
1. Send close message
2. Destroy ephemeral keys
3. Wipe session keys from memory
4. Close connection
```

---

## 🚀 Next Steps

### Continue Learning
1. **Level 1**: Hardware Integration demos
2. **Level 2**: Ecosystem Integration (BTSP with Songbird!)
3. **02-ecosystem-integration/01-songbird-btsp** - Real cross-primal tunnels

### Real Implementation

This demo shows concepts. For real BTSP:
```rust
// Client
let tunnel = BtspClient::connect("bob.example.com:9999", alice_key).await?;
tunnel.send(b"secret message").await?;
let response = tunnel.receive().await?;

// Server
let server = BtspServer::listen("0.0.0.0:9999", bob_key).await?;
let tunnel = server.accept().await?;
let message = tunnel.receive().await?;
tunnel.send(b"response").await?;
```

---

## 📚 Related Documentation

- **specs/current/integration/SONGBIRD_BEARDOG_VPN_FREE_ARCHITECTURE.md**
- **MULTI_PROTOCOL_GUIDE.md**
- **02-ecosystem-integration/01-songbird-btsp** - Real implementation

---

## 🎯 Level 0 Complete!

**Congratulations!** You've completed all Level 0 demos:

1. ✅ Hello BearDog - First key generation
2. ✅ HSM Discovery - Find hardware
3. ✅ Key Constraints - Self-enforcing keys
4. ✅ Entropy Mixing - Human + machine
5. ✅ Key Lineage - Track ancestry
6. ✅ BTSP Tunnel - Secure connections

**What's Next?**

**Level 1**: Hardware Integration (`../01-hardware-integration/`)
- Real YubiKey usage
- TPM integration
- Mobile HSMs
- HSM comparison

**Level 2**: Ecosystem Integration (`../02-ecosystem-integration/`)
- Songbird BTSP tunnels
- NestGate encrypted storage
- ToadStool encrypted compute
- Squirrel key routing

---

**Demo Status**: ✅ Educational  
**Difficulty**: 🟡 Intermediate  
**Time Required**: 15 minutes

🔒 **BearDog: Sovereign Secure Tunnels!** 🌐

