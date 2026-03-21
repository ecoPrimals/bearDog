# 🌍 BearDog Isomorphic IPC Evolution Plan

**Date**: January 31, 2026 (Evening)  
**Priority**: HIGH (Next session - unblocks TOWER atomic + STUN handshake)  
**Effort**: 4-6 hours (direct evolution from songbird pattern)  
**Status**: Ready for Implementation

═══════════════════════════════════════════════════════════════════

## 🎯 EXECUTIVE SUMMARY

**Objective**: Evolve BearDog's IPC to isomorphic pattern (Try→Detect→Adapt→Succeed)

**Current State**: ✅ **EXCELLENT FOUNDATION**
- Universal platform traits complete (Jan 31 AM)
- `PlatformListener`, `PlatformStream` working
- Unix + Android implemented
- Handler refactoring complete
- 1381 tests passing

**Gap**: Missing TCP fallback for Android SELinux constraints

**Target**: Same pattern as songbird (proven on Pixel 8a)

═══════════════════════════════════════════════════════════════════

## 📊 CURRENT ARCHITECTURE ANALYSIS

### ✅ Strengths (Ready for Isomorphic Evolution)

**1. Universal Trait Foundation** (Completed Today!)
```rust
// We already have this! (Jan 31, 2026)
pub trait PlatformListener: Send + Sync {
    async fn accept(&mut self) -> std::io::Result<Box<dyn PlatformStream>>;
    fn local_addr(&self) -> std::io::Result<String>;
}

pub trait PlatformStream: AsyncRead + AsyncWrite + Send + Sync + Unpin {}
```

**2. Platform Abstractions** (Phases 1 & 2 Complete)
```rust
// Already implemented:
- Unix: UnixPlatformListener + UnixPlatformStream
- Android: AndroidPlatformListener + AndroidPlatformStream
- Windows: Ready (traits defined, impl pending)
```

**3. Handler Universality** (Completed Today)
```rust
// Already uses universal streams:
async fn handle_connection(&self, stream: Box<dyn PlatformStream>) -> Result<()>
```

**4. Platform-Agnostic Binding** (Already in place)
```rust
// Location: crates/beardog-tunnel/src/platform/mod.rs:209
pub fn get_socket_endpoint() -> std::io::Result<SocketEndpoint> {
    // 1. BEARDOG_ABSTRACT_SOCKET env var
    // 2. BEARDOG_SOCKET env var  
    // 3. Platform default
    Socket::create_endpoint("beardog")
}
```

### 🎯 Gap: TCP Fallback

**What's Missing**: Try→Detect→Adapt pattern

**Current Behavior**:
```rust
// Location: crates/beardog-tunnel/src/unix_socket_ipc/server.rs:206
let mut listener = Socket::bind(&endpoint).context(format!(
    "Failed to bind socket on {}: {}",
    platform_type,
    endpoint.display()
))?;  // ❌ FAILS on SELinux Android - no fallback!
```

**Needed Behavior** (songbird pattern):
```rust
// Try optimal path first
match try_unix_socket().await {
    Ok(listener) => run_unix_server(listener),
    // Detect platform constraint
    Err(e) if is_platform_constraint(&e) => {
        // Adapt automatically!
        start_tcp_fallback().await
    }
    Err(e) => Err(e)  // Real error
}
```

═══════════════════════════════════════════════════════════════════

## 🏗️ IMPLEMENTATION PHASES

### Phase 1: Platform Constraint Detection (30 minutes)

**Location**: `crates/beardog-tunnel/src/unix_socket_ipc/server.rs`

**Add Methods**:
```rust
impl UnixSocketIpcServer {
    /// Detect if error is a platform constraint (not a real error)
    fn is_platform_constraint(&self, error: &anyhow::Error) -> bool {
        if let Some(io_err) = error.downcast_ref::<std::io::Error>() {
            match io_err.kind() {
                // Permission denied often means SELinux blocking
                ErrorKind::PermissionDenied => {
                    self.is_selinux_enforcing()
                }
                // Address family not supported
                ErrorKind::Unsupported => true,
                _ => false
            }
        } else {
            false
        }
    }

    /// Check if SELinux is enforcing (Android)
    fn is_selinux_enforcing(&self) -> bool {
        std::fs::read_to_string("/sys/fs/selinux/enforce")
            .ok()
            .and_then(|s| s.trim().parse::<u8>().ok())
            .map(|v| v == 1)
            .unwrap_or(false)
    }
}
```

**Source**: Direct copy from songbird's `server.rs` (lines 420-444)

---

### Phase 2: Try Unix Socket Method (30 minutes)

**Location**: `crates/beardog-tunnel/src/unix_socket_ipc/server.rs`

**Refactor `start()` to `try_unix_server()`**:
```rust
impl UnixSocketIpcServer {
    /// Try to start Unix socket server (optimal path)
    async fn try_unix_server(self: Arc<Self>) -> Result<()> {
        {
            let mut is_running = self.is_running.write().await;
            if *is_running {
                warn!("⚠️  Unix socket IPC server already running");
                return Ok(());
            }
            *is_running = true;
        }

        info!("   Trying Unix socket IPC (optimal)...");

        // Platform-agnostic socket binding
        let endpoint = crate::platform::get_socket_endpoint()
            .map_err(|e| anyhow::anyhow!("Failed to create socket endpoint: {}", e))?;

        // Bind with platform-specific logic
        let mut listener = Socket::bind(&endpoint).context(format!(
            "Failed to bind socket: {}",
            endpoint.display()
        ))?;

        // Mark ready
        self.is_ready.store(true, std::sync::atomic::Ordering::Release);

        info!("✅ Unix socket IPC listening: {}", endpoint.display());

        // Accept loop (unchanged)
        self.accept_loop(listener).await
    }

    /// Accept connections loop (universal)
    async fn accept_loop(
        self: Arc<Self>,
        mut listener: Box<dyn PlatformListener>,
    ) -> Result<()> {
        loop {
            match listener.accept().await {
                Ok(stream) => {
                    let server = Arc::clone(&self);
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_connection(stream).await {
                            error!("❌ Connection handler error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("❌ Failed to accept connection: {}", e);
                }
            }
        }
    }
}
```

**Changes**:
- Rename `start()` → `try_unix_server()`
- Extract accept loop to `accept_loop()` (reusable!)
- Keep all existing logic

---

### Phase 3: TCP Fallback Implementation (2-3 hours)

**Location**: `crates/beardog-tunnel/src/unix_socket_ipc/server.rs`

**Add TCP Fallback Server**:
```rust
use tokio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::io::Write;

impl UnixSocketIpcServer {
    /// Start TCP fallback server (isomorphic mode)
    async fn start_tcp_fallback(self: Arc<Self>) -> Result<()> {
        info!("🌐 Starting TCP IPC fallback (isomorphic mode)");
        info!("   Protocol: JSON-RPC 2.0 (same as Unix socket)");

        // Bind to localhost only (security: same as Unix socket)
        let listener = TcpListener::bind("127.0.0.1:0").await
            .context("Failed to bind TCP socket")?;

        let local_addr = listener.local_addr()?;
        info!("✅ TCP IPC listening on {}", local_addr);

        // Write discovery file for clients
        self.write_tcp_discovery_file(&local_addr)?;

        // Mark ready
        self.is_ready.store(true, std::sync::atomic::Ordering::Release);

        info!("   Status: READY ✅ (isomorphic TCP fallback active)");

        // Accept connections (same protocol as Unix!)
        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    let server = Arc::clone(&self);
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_tcp_connection(stream).await {
                            error!("❌ TCP connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("❌ Failed to accept TCP connection: {}", e);
                }
            }
        }
    }

    /// Handle TCP connection (uses same protocol as Unix)
    async fn handle_tcp_connection(&self, stream: TcpStream) -> Result<()> {
        debug!("📥 New TCP IPC connection (isomorphic fallback)");

        // Wrap TcpStream as Box<dyn PlatformStream>
        let platform_stream: Box<dyn PlatformStream> = Box::new(stream);

        // Use existing universal handler!
        self.handle_connection(platform_stream).await
    }

    /// Write TCP discovery file for clients
    fn write_tcp_discovery_file(&self, addr: &SocketAddr) -> Result<()> {
        // XDG-compliant discovery paths
        let discovery_dirs = [
            std::env::var("XDG_RUNTIME_DIR").ok(),
            std::env::var("HOME").map(|h| format!("{}/.local/share", h)),
            Some("/tmp".to_string()),
        ];

        for dir in discovery_dirs.iter().filter_map(|d| d.as_ref()) {
            let discovery_file = format!("{}/beardog-ipc-port", dir);

            if let Ok(mut f) = std::fs::File::create(&discovery_file) {
                // Write in format: tcp:127.0.0.1:PORT
                writeln!(f, "tcp:{}", addr)?;
                info!("📁 TCP discovery file: {}", discovery_file);
                return Ok(());
            }
        }

        Ok(())
    }
}

// Implement PlatformStream for TcpStream (enables reuse!)
impl PlatformStream for TcpStream {}
```

**Key Features**:
- Same JSON-RPC protocol as Unix
- Localhost only (security)
- Discovery file for clients
- Reuses existing `handle_connection()` via `PlatformStream` trait!

---

### Phase 4: Isomorphic Entry Point (30 minutes)

**Location**: `crates/beardog-tunnel/src/unix_socket_ipc/server.rs`

**New Public `start()` Method**:
```rust
impl UnixSocketIpcServer {
    /// Start IPC server (isomorphic mode)
    ///
    /// Automatically adapts to platform constraints:
    /// - Linux/macOS: Unix sockets (optimal)
    /// - Android (SELinux): TCP fallback (automatic)
    ///
    /// This is biological adaptation - learns environment and evolves!
    pub async fn start(self: Arc<Self>) -> Result<()> {
        info!("🔌 Starting IPC server (isomorphic mode)...");

        // 1. TRY Unix socket first (optimal)
        info!("   Trying Unix socket IPC (optimal)...");

        match self.clone().try_unix_server().await {
            Ok(()) => Ok(()),

            // 2. DETECT platform constraints
            Err(e) if self.is_platform_constraint(&e) => {
                warn!("⚠️  Unix sockets unavailable: {}", e);
                warn!("   Detected platform constraint, adapting...");

                // 3. ADAPT to TCP fallback
                self.start_tcp_fallback().await
            }

            // 4. Real error
            Err(e) => {
                error!("❌ Failed to start IPC server: {}", e);
                Err(e)
            }
        }
    }
}
```

**Result**: **TRUE ISOMORPHISM!** ✅

---

### Phase 5: Client Discovery (2-3 hours)

**Location**: `crates/beardog-ipc/src/lib.rs`

**Add IPC Endpoint Types**:
```rust
/// IPC endpoint types (Unix or TCP)
#[derive(Debug, Clone)]
pub enum IpcEndpoint {
    UnixSocket(std::path::PathBuf),
    TcpLocal(std::net::SocketAddr),
}

impl IpcEndpoint {
    pub fn display(&self) -> String {
        match self {
            IpcEndpoint::UnixSocket(path) => format!("unix:{}", path.display()),
            IpcEndpoint::TcpLocal(addr) => format!("tcp:{}", addr),
        }
    }
}
```

**Add Discovery Function**:
```rust
/// Discover BearDog IPC endpoint (Unix or TCP)
///
/// Tries Unix socket first, falls back to TCP discovery file.
/// This enables isomorphic operation - same client code works everywhere!
pub fn discover_beardog_endpoint() -> Result<IpcEndpoint> {
    // 1. Try Unix socket paths first (optimal)
    let socket_paths = get_unix_socket_paths();
    for path in socket_paths {
        if path.exists() {
            return Ok(IpcEndpoint::UnixSocket(path));
        }
    }

    // 2. Try TCP discovery file (fallback)
    if let Ok(endpoint) = discover_tcp_endpoint() {
        return Ok(endpoint);
    }

    Err(anyhow::anyhow!("Could not discover BearDog IPC endpoint"))
}

fn get_unix_socket_paths() -> Vec<std::path::PathBuf> {
    use std::path::PathBuf;

    let mut paths = Vec::new();

    // 1. BEARDOG_SOCKET env var
    if let Ok(path) = std::env::var("BEARDOG_SOCKET") {
        paths.push(PathBuf::from(path));
    }

    // 2. XDG runtime dir
    if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        paths.push(PathBuf::from(format!("{}/biomeos/beardog.sock", runtime_dir)));
    }

    // 3. /tmp fallback
    paths.push(PathBuf::from("/tmp/beardog.sock"));

    paths
}

fn discover_tcp_endpoint() -> Result<IpcEndpoint> {
    let discovery_files = [
        std::env::var("XDG_RUNTIME_DIR").ok().map(|d| format!("{}/beardog-ipc-port", d)),
        std::env::var("HOME").ok().map(|h| format!("{}/.local/share/beardog-ipc-port", h)),
        Some("/tmp/beardog-ipc-port".to_string()),
    ];

    for file in discovery_files.iter().filter_map(|f| f.as_ref()) {
        if let Ok(contents) = std::fs::read_to_string(file) {
            // Parse format: tcp:127.0.0.1:PORT
            if let Some(addr_str) = contents.trim().strip_prefix("tcp:") {
                if let Ok(addr) = addr_str.parse::<std::net::SocketAddr>() {
                    tracing::info!("📡 Found TCP discovery file: {} -> {}", file, addr);
                    return Ok(IpcEndpoint::TcpLocal(addr));
                }
            }
        }
    }

    Err(anyhow::anyhow!("No TCP discovery file found"))
}
```

**Connection Helper**:
```rust
use tokio::net::TcpStream as TokioTcpStream;
use tokio::net::UnixStream;

/// Connect to BearDog IPC endpoint (polymorphic)
pub async fn connect_beardog() -> Result<Box<dyn AsyncStream>> {
    let endpoint = discover_beardog_endpoint()?;

    match endpoint {
        IpcEndpoint::UnixSocket(path) => {
            let stream = UnixStream::connect(&path).await?;
            Ok(Box::new(stream) as Box<dyn AsyncStream>)
        }
        IpcEndpoint::TcpLocal(addr) => {
            let stream = TokioTcpStream::connect(addr).await?;
            Ok(Box::new(stream) as Box<dyn AsyncStream>)
        }
    }
}

/// Polymorphic stream trait for IPC
pub trait AsyncStream: tokio::io::AsyncRead + tokio::io::AsyncWrite + Send + Unpin {}
impl AsyncStream for UnixStream {}
impl AsyncStream for TokioTcpStream {}
```

═══════════════════════════════════════════════════════════════════

## ✅ VALIDATION CHECKLIST

### Server-Side ✅

- [ ] `is_platform_constraint()` method added
- [ ] `is_selinux_enforcing()` checks `/sys/fs/selinux/enforce`
- [ ] `try_unix_server()` attempts optimal path
- [ ] `start_tcp_fallback()` binds to `127.0.0.1:0`
- [ ] TCP server uses same JSON-RPC protocol
- [ ] Discovery file written to XDG paths
- [ ] `start()` implements Try→Detect→Adapt pattern
- [ ] Logs show adaptation on Android
- [ ] `PlatformStream` implemented for `TcpStream`

### Client-Side ✅

- [ ] `IpcEndpoint` enum defined
- [ ] `discover_beardog_endpoint()` tries Unix then TCP
- [ ] TCP discovery file parsed correctly
- [ ] `AsyncStream` trait for polymorphic streams
- [ ] `connect_beardog()` handles both types

### Testing ✅

- [ ] Build succeeds for x86_64 and aarch64
- [ ] Linux: Uses Unix sockets (logs confirm)
- [ ] Android: Uses TCP fallback (logs confirm SELinux)
- [ ] No environment variables required
- [ ] Inter-primal communication works
- [ ] Deep Debt principles maintained

═══════════════════════════════════════════════════════════════════

## 📊 ESTIMATED EFFORT

| Phase | Effort | Complexity |
|-------|--------|------------|
| Phase 1: Constraint Detection | 30 min | Low (copy from songbird) |
| Phase 2: Try Unix Method | 30 min | Low (refactor existing) |
| Phase 3: TCP Fallback | 2-3 hours | Medium (new code) |
| Phase 4: Isomorphic Entry | 30 min | Low (pattern application) |
| Phase 5: Client Discovery | 2-3 hours | Medium (new module) |
| **Total** | **4-6 hours** | **Medium** |

**Advantage**: We already have universal traits (today's work)! 🎉

═══════════════════════════════════════════════════════════════════

## 🎯 PRIORITY & IMPACT

**Priority**: **HIGH** (Next session)

**Why High Priority**:
1. Unblocks TOWER atomic testing (beardog + songbird)
2. Enables STUN handshake validation
3. Proven pattern (songbird on Pixel 8a)
4. Foundation ready (Phases 1 & 2 complete)

**Impact**:
- ✅ Android deployment ready
- ✅ TOWER atomic validated
- ✅ Deep debt A++ maintained
- ✅ Pattern for other primals

**Blockers Removed**:
- Android SELinux constraints
- Unix socket availability
- Platform-specific configuration

═══════════════════════════════════════════════════════════════════

## 📚 REFERENCE MATERIALS

**Primary Reference**: songbird v3.33.0
- `crates/songbird-orchestrator/src/ipc/pure_rust_server/server.rs` (lines 250-446)
- `crates/songbird-http-client/src/crypto/socket_discovery.rs`

**Today's Foundation**:
- `docs/sessions/2026-01-30/PLATFORM_UNIVERSALITY_PHASE1_JAN_31_2026.md`
- `docs/sessions/2026-01-30/PLATFORM_UNIVERSALITY_PHASE2_JAN_31_2026.md`
- `crates/beardog-tunnel/src/platform/mod.rs` (universal traits)

**Validation Proof**:
- Pixel 8a logs showing automatic TCP fallback
- songbird ISOMORPHIC_IPC_VALIDATION_COMPLETE.md

═══════════════════════════════════════════════════════════════════

## 🎊 SUCCESS CRITERIA

**Implementation Complete When**:

1. ✅ Builds for x86_64 and aarch64
2. ✅ Linux: Uses Unix sockets (optimal)
3. ✅ Android: Auto-falls back to TCP
4. ✅ Zero configuration needed
5. ✅ Logs show: "⚠️ Unix sockets unavailable, using TCP fallback"
6. ✅ Logs show: "✅ TCP IPC listening on 127.0.0.1:XXXXX"
7. ✅ Discovery file created
8. ✅ Clients connect successfully on both
9. ✅ Deep Debt principles maintained (A++ grade)

**Expected Logs on Android**:
```
[INFO] 🔌 Starting IPC server (isomorphic mode)...
[INFO]    Trying Unix socket IPC (optimal)...
[WARN] ⚠️  Unix sockets unavailable: Permission denied
[WARN]    Detected platform constraint, adapting...
[INFO] 🌐 Starting TCP IPC fallback (isomorphic mode)
[INFO]    Protocol: JSON-RPC 2.0 (same as Unix socket)
[INFO] ✅ TCP IPC listening on 127.0.0.1:45763
[INFO]    Status: READY ✅ (isomorphic TCP fallback active)
```

**This proves TRUE isomorphism!** 🌍

═══════════════════════════════════════════════════════════════════

## 💡 KEY INSIGHTS

### 1. We're 50% Done Already!

Today's universal trait work (Phases 1 & 2) provides:
- `PlatformListener` trait ✅
- `PlatformStream` trait ✅
- Handler universality ✅
- Unix + Android impl ✅

**Remaining**: TCP fallback + client discovery

### 2. Direct Copy from songbird

Most code can be **directly copied** from songbird:
- Constraint detection (100% reusable)
- SELinux checking (100% reusable)
- Discovery file format (100% reusable)

### 3. Pattern Is Universal

Try→Detect→Adapt works for:
- IPC (Unix → TCP) ← **We're doing this!**
- Storage (mmap → file → memory)
- Crypto (hardware → software HSM)
- Display (Wayland → X11 → framebuffer)

**Apply everywhere platform constraints exist!**

═══════════════════════════════════════════════════════════════════

## 🏁 NEXT STEPS

**Immediate (Next Session)**:
1. Read songbird's `server.rs` (lines 250-446)
2. Implement Phase 1-4 (server-side isomorphism)
3. Test on Linux (should use Unix)
4. Deploy to Android (should use TCP)
5. Capture logs proving adaptation

**Short-Term (Same session)**:
6. Implement Phase 5 (client discovery)
7. Test end-to-end communication
8. Validate TOWER atomic with songbird

**Documentation**:
9. Capture Android logs showing TCP fallback
10. Document isomorphic IPC complete
11. Create handoff for toadstool/nestgate/squirrel

═══════════════════════════════════════════════════════════════════

**Status**: Ready for Implementation  
**Foundation**: Complete (Phases 1 & 2 done today!)  
**Reference**: songbird v3.33.0 (proven on Pixel 8a)  
**Priority**: HIGH (unblocks TOWER atomic)

🌍🧬🦀 **Binary = DNA: Universal, Deterministic, Adaptive** 🦀🧬🌍

**Let's evolve BearDog to TRUE isomorphism!** 🚀
