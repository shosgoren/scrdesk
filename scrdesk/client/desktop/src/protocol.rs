use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    // Connection & Authentication
    Hello {
        device_id: String,
        platform: String,
        capabilities: Vec<String>,
    },
    ConnectRequest {
        target_id: String,
        auth_token: Option<String>,
    },
    ConnectResponse {
        success: bool,
        session_id: Option<String>,
        error: Option<String>,
    },

    // Video Streaming
    VideoFrame {
        data: Vec<u8>,
        width: u32,
        height: u32,
        timestamp: u64,
        is_keyframe: bool,
    },

    // Input Events
    MouseMove {
        x: i32,
        y: i32,
    },
    MouseButton {
        button: MouseButton,
        pressed: bool,
    },
    MouseScroll {
        delta_x: i32,
        delta_y: i32,
    },
    KeyboardEvent {
        key: String,
        pressed: bool,
        modifiers: KeyModifiers,
    },

    // File Transfer
    FileTransferRequest {
        transfer_id: String,
        filename: String,
        filesize: u64,
        direction: TransferDirection,
    },
    FileTransferResponse {
        transfer_id: String,
        accepted: bool,
    },
    FileChunk {
        transfer_id: String,
        chunk_index: u64,
        data: Vec<u8>,
    },
    FileTransferComplete {
        transfer_id: String,
        success: bool,
    },

    // Clipboard
    ClipboardUpdate {
        content: String,
        mime_type: String,
    },

    // Control
    Ping,
    Pong,
    Disconnect {
        reason: Option<String>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransferDirection {
    Upload,   // Local to Remote
    Download, // Remote to Local
}

impl Message {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}
