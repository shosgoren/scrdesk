# ScrDesk Remote Desktop - Full Implementation Plan

## Overview
Implement full remote desktop functionality with:
- Real-time screen capture and streaming
- Bidirectional mouse & keyboard control
- File transfer (both directions)
- Clipboard synchronization
- Connection ID-based pairing

## Current State Analysis

### ✅ What We Have
1. **UI Layer**: Modern interface with guest mode and login
2. **Backend Services**: Auth, Device Manager, Core Server running
3. **Basic Structure**: ConnectionManager skeleton, relay server placeholder
4. **Dependencies**: RustDesk hbb_common already added
5. **Platform APIs**: Windows/macOS/Linux dependencies configured

### ❌ What's Missing (Core Functionality)
1. Screen capture implementation
2. Video encoding/decoding
3. Network protocol for data transmission
4. Mouse/keyboard event simulation
5. File transfer protocol
6. Clipboard sync mechanism

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Desktop Client (Rust)                     │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌────────────┐  ┌─────────────┐  ┌──────────────┐          │
│  │ UI Layer   │  │  Capture    │  │   Control    │          │
│  │  (egui)    │  │   Engine    │  │   Engine     │          │
│  └─────┬──────┘  └──────┬──────┘  └──────┬───────┘          │
│        │                │                 │                   │
│        └────────────────┼─────────────────┘                   │
│                         │                                     │
│                  ┌──────▼──────┐                             │
│                  │  Connection │                             │
│                  │   Manager   │                             │
│                  └──────┬──────┘                             │
│                         │                                     │
└─────────────────────────┼─────────────────────────────────────┘
                          │
                    WebSocket/TCP
                          │
┌─────────────────────────▼─────────────────────────────────────┐
│                   Relay Server (Rust)                         │
├───────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌────────────┐  ┌──────────────┐          │
│  │   Session    │  │   P2P NAT  │  │    Message   │          │
│  │   Manager    │  │  Traversal │  │    Router    │          │
│  └──────────────┘  └────────────┘  └──────────────┘          │
└───────────────────────────────────────────────────────────────┘
```

## Implementation Phases

### Phase 1: Screen Capture (Priority: HIGH)
**Goal**: Capture screen and send raw frames

#### 1.1 Platform-Specific Screen Capture
**File**: `scrdesk/client/desktop/src/capture/mod.rs`

```rust
pub trait ScreenCapture {
    fn start(&mut self) -> Result<()>;
    fn capture_frame(&mut self) -> Result<Frame>;
    fn stop(&mut self);
}

#[cfg(target_os = "windows")]
mod windows;  // Use DXGI Desktop Duplication API

#[cfg(target_os = "macos")]
mod macos;    // Use CGDisplayStream

#[cfg(target_os = "linux")]
mod linux;    // Use X11/Wayland
```

**Dependencies Needed**:
- Windows: `windows-capture` crate or raw `winapi`
- macOS: `core-graphics` crate
- Linux: `xcb` or `wayland-client`

**Estimated Time**: 1-2 days

#### 1.2 Video Encoding
**File**: `scrdesk/client/desktop/src/encoding/mod.rs`

Use `ffmpeg` or `openh264` for H.264 encoding:
```rust
pub struct VideoEncoder {
    encoder: Encoder,
    bitrate: u32,
    fps: u32,
}

impl VideoEncoder {
    pub fn encode_frame(&mut self, frame: &Frame) -> Result<Vec<u8>>;
}
```

**Dependencies**:
- `openh264` (Mozilla's H.264 codec - license-free)
- Or `ffmpeg-next` (more features but requires system FFmpeg)

**Estimated Time**: 1 day

### Phase 2: Network Protocol (Priority: HIGH)
**Goal**: Establish reliable connection between clients

#### 2.1 Protocol Design
**Message Types**:
```rust
pub enum Message {
    // Connection
    Hello { device_id: String, capabilities: Vec<String> },
    ConnectRequest { target_id: String, auth_token: String },
    ConnectResponse { success: bool, session_id: String },

    // Streaming
    VideoFrame { data: Vec<u8>, timestamp: u64, keyframe: bool },
    AudioFrame { data: Vec<u8>, timestamp: u64 },

    // Input Events
    MouseEvent { x: i32, y: i32, button: MouseButton, action: MouseAction },
    KeyboardEvent { key: KeyCode, modifiers: Modifiers, action: KeyAction },

    // File Transfer
    FileTransferRequest { filename: String, size: u64 },
    FileChunk { transfer_id: String, chunk: Vec<u8>, offset: u64 },

    // Clipboard
    ClipboardSync { content: String, mime_type: String },

    // Control
    Ping,
    Pong,
    Disconnect,
}
```

#### 2.2 Relay Server Implementation
**File**: `scrdesk/backend/scrdesk-relay-cluster/src/relay/session.rs`

```rust
pub struct Session {
    id: String,
    client_a: Arc<Mutex<WebSocket>>,
    client_b: Arc<Mutex<WebSocket>>,
    bandwidth_limiter: BandwidthLimiter,
}

impl Session {
    pub async fn relay_data(&self, from: Client, data: Vec<u8>);
    pub async fn handle_message(&self, msg: Message) -> Result<()>;
}
```

**Features**:
- WebSocket or TCP with custom protocol
- Message routing between connected clients
- Bandwidth management
- Session timeout handling

**Estimated Time**: 2 days

### Phase 3: Mouse & Keyboard Control (Priority: HIGH)
**Goal**: Send and receive input events

#### 3.1 Input Capture (Sender Side)
**File**: `scrdesk/client/desktop/src/input/capture.rs`

Integrate with egui to capture local input:
```rust
pub struct InputCapture {
    mouse_pos: (i32, i32),
    pressed_keys: HashSet<KeyCode>,
}

impl InputCapture {
    pub fn handle_egui_event(&mut self, event: &egui::Event) -> Option<Message>;
}
```

#### 3.2 Input Simulation (Receiver Side)
**File**: `scrdesk/client/desktop/src/input/simulator.rs`

Platform-specific input injection:

**Windows**:
```rust
use winapi::um::winuser::{SendInput, INPUT};

pub fn simulate_mouse(x: i32, y: i32, button: MouseButton);
pub fn simulate_keyboard(key: KeyCode, action: KeyAction);
```

**macOS**:
```rust
use core_graphics::event::{CGEvent, CGEventType};

pub fn simulate_mouse(x: i32, y: i32, button: MouseButton);
pub fn simulate_keyboard(key: KeyCode, action: KeyAction);
```

**Linux**:
```rust
use x11::xtest::*;

pub fn simulate_mouse(x: i32, y: i32, button: MouseButton);
pub fn simulate_keyboard(key: KeyCode, action: KeyAction);
```

**Estimated Time**: 2 days

### Phase 4: File Transfer (Priority: MEDIUM)
**Goal**: Send/receive files between connected machines

#### 4.1 File Transfer Protocol
**File**: `scrdesk/client/desktop/src/transfer/mod.rs`

```rust
pub struct FileTransfer {
    transfers: HashMap<String, TransferState>,
}

pub struct TransferState {
    filename: String,
    total_size: u64,
    received_bytes: u64,
    file_handle: File,
}

impl FileTransfer {
    pub async fn send_file(&mut self, path: PathBuf, conn: &Connection) -> Result<()>;
    pub async fn receive_chunk(&mut self, transfer_id: String, data: Vec<u8>) -> Result<()>;
}
```

**Features**:
- Chunked transfer (1MB chunks)
- Progress tracking
- Resume capability
- Checksum verification

**Estimated Time**: 1-2 days

### Phase 5: Clipboard Sync (Priority: LOW)
**Goal**: Sync clipboard between machines

#### 5.1 Clipboard Monitor
**File**: `scrdesk/client/desktop/src/clipboard/mod.rs`

```rust
use arboard::Clipboard;  // Cross-platform clipboard

pub struct ClipboardSync {
    clipboard: Clipboard,
    last_content: String,
}

impl ClipboardSync {
    pub fn check_changes(&mut self) -> Option<String>;
    pub fn set_content(&mut self, content: String) -> Result<()>;
}
```

**Estimated Time**: 1 day

## Integration Plan

### Step 1: Update Desktop Client Main Loop
**File**: `scrdesk/client/desktop/src/main.rs`

Add streaming and control threads:
```rust
struct ScrDeskApp {
    // ... existing fields
    capture_engine: Option<CaptureEngine>,
    encoder: Option<VideoEncoder>,
    input_simulator: InputSimulator,
    file_transfer: FileTransfer,
    clipboard_sync: ClipboardSync,
}

impl ScrDeskApp {
    fn start_streaming(&mut self) {
        // Spawn screen capture thread
        // Spawn encoding thread
        // Spawn network send thread
    }

    fn handle_incoming_message(&mut self, msg: Message) {
        match msg {
            Message::VideoFrame { .. } => { /* render */ },
            Message::MouseEvent { .. } => { /* simulate */ },
            Message::KeyboardEvent { .. } => { /* simulate */ },
            Message::FileChunk { .. } => { /* save */ },
            Message::ClipboardSync { .. } => { /* update */ },
            _ => {}
        }
    }
}
```

### Step 2: Implement Relay Server
**File**: `scrdesk/backend/scrdesk-relay-cluster/src/relay/mod.rs`

Complete the relay server:
```rust
pub async fn start_relay_server(config: Config) -> Result<()> {
    let sessions: Arc<Mutex<HashMap<String, Session>>> = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("0.0.0.0:21117").await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        let sessions = sessions.clone();

        tokio::spawn(async move {
            handle_client(socket, addr, sessions).await;
        });
    }
}

async fn handle_client(socket: TcpStream, addr: SocketAddr, sessions: Arc<Mutex<HashMap<String, Session>>>) {
    // 1. Receive Hello message
    // 2. Authenticate
    // 3. Wait for ConnectRequest or create new session
    // 4. Start relaying data
}
```

### Step 3: UI Integration
Update the guest mode screen to show:
- Local connection ID
- Remote ID input field
- Connection status
- Video stream display
- File transfer UI
- Clipboard status

## Dependencies to Add

### Cargo.toml Updates

```toml
[dependencies]
# Video encoding
openh264 = "0.4"  # or ffmpeg-next = "6.0"

# Screen capture
[target.'cfg(target_os = "windows")'.dependencies]
windows-capture = "1.0"

[target.'cfg(target_os = "macos")'.dependencies]
core-graphics = "0.23"
core-foundation = "0.9"

[target.'cfg(target_os = "linux")'.dependencies]
xcb = "1.2"

# Clipboard
arboard = "3.3"

# Networking
tokio-tungstenite = "0.21"  # WebSocket
futures = "0.3"

# Utilities
sha2 = "0.10"  # Checksums
```

## Testing Strategy

### Unit Tests
- Frame encoding/decoding
- Message serialization
- Input event conversion

### Integration Tests
- Local loopback connection
- File transfer completion
- Clipboard sync

### Manual Testing
- Test on same network (LAN)
- Test through relay (different networks)
- Test all platforms (Windows ↔ Mac ↔ Linux)

## Performance Targets

- **Latency**: < 100ms for input events
- **Frame Rate**: 30 FPS minimum, 60 FPS target
- **Bandwidth**: Adaptive 1-10 Mbps based on network
- **File Transfer**: > 10 MB/s on LAN

## Security Considerations

1. **Encryption**: All data over TLS/SSL
2. **Authentication**: JWT tokens for device auth
3. **Authorization**: Connection approval required
4. **Session Timeout**: 1 hour for guest mode
5. **Input Validation**: Sanitize all received data

## Timeline Estimation

### Minimal Viable Product (3-5 days)
- ✅ Screen capture (Windows/Mac)
- ✅ H.264 encoding
- ✅ Basic relay server
- ✅ Mouse control (one-way)
- ✅ Keyboard control (one-way)

### Full Feature Set (7-10 days)
- ✅ All platforms
- ✅ Bidirectional control
- ✅ File transfer
- ✅ Clipboard sync
- ✅ Optimizations (adaptive bitrate, etc.)

## Risk Mitigation

### Technical Risks
1. **Platform-specific APIs**: Test on each platform early
2. **Network issues**: Implement reconnection logic
3. **Performance**: Profile and optimize encoding pipeline

### Compatibility Risks
1. **Firewall/NAT**: Use STUN/TURN as fallback
2. **Codec support**: Test H.264 on all platforms
3. **Screen resolution**: Support scaling

## Next Steps

1. **User Approval**: Confirm this plan meets requirements
2. **Start Phase 1**: Begin with screen capture implementation
3. **Iterative Development**: Build, test, improve each phase
4. **Integration**: Connect all components
5. **Testing**: Comprehensive testing on all platforms
6. **Deployment**: Update GitHub Actions for all platforms

## Questions for User

1. **Priority**: MVP (3-5 days) or Full Feature (7-10 days)?
2. **Platforms**: Focus on Windows + Mac first, or all three?
3. **Quality vs Speed**: Higher quality (H.265) or faster implementation (H.264)?
4. **File Transfer Limits**: Any size restrictions?
5. **Security Level**: Basic TLS or additional encryption layer?
