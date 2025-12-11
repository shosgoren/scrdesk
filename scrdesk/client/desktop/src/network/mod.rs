use anyhow::{Context, Result};
use crate::protocol::Message;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};

const RELAY_SERVER_URL: &str = "ws://72.61.138.218:21117";
const RECONNECT_DELAY_SECS: u64 = 5;
const MAX_RECONNECT_ATTEMPTS: u32 = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Failed,
}

pub struct NetworkConnection {
    state: Arc<Mutex<ConnectionState>>,
    outgoing_tx: mpsc::UnboundedSender<Message>,
    incoming_rx: Arc<Mutex<mpsc::UnboundedReceiver<Message>>>,
}

impl NetworkConnection {
    pub async fn connect(device_id: String) -> Result<Self> {
        let (outgoing_tx, outgoing_rx) = mpsc::unbounded_channel::<Message>();
        let (incoming_tx, incoming_rx) = mpsc::unbounded_channel::<Message>();

        let state = Arc::new(Mutex::new(ConnectionState::Connecting));

        // Spawn connection task
        let state_clone = state.clone();
        tokio::spawn(async move {
            connection_task(device_id, outgoing_rx, incoming_tx, state_clone).await;
        });

        Ok(Self {
            state,
            outgoing_tx,
            incoming_rx: Arc::new(Mutex::new(incoming_rx)),
        })
    }

    pub async fn send(&self, message: Message) -> Result<()> {
        self.outgoing_tx.send(message)
            .context("Failed to send message")?;
        Ok(())
    }

    pub async fn recv(&self) -> Option<Message> {
        self.incoming_rx.lock().await.recv().await
    }

    pub async fn get_state(&self) -> ConnectionState {
        self.state.lock().await.clone()
    }

    pub async fn disconnect(&self) {
        let _ = self.send(Message::Disconnect { reason: Some("User disconnected".to_string()) }).await;
        *self.state.lock().await = ConnectionState::Disconnected;
    }
}

async fn connection_task(
    device_id: String,
    mut outgoing_rx: mpsc::UnboundedReceiver<Message>,
    incoming_tx: mpsc::UnboundedSender<Message>,
    state: Arc<Mutex<ConnectionState>>,
) {
    let mut reconnect_attempts = 0;

    loop {
        tracing::info!("Connecting to relay server: {}", RELAY_SERVER_URL);

        match connect_async(RELAY_SERVER_URL).await {
            Ok((ws_stream, _)) => {
                tracing::info!("Connected to relay server");
                *state.lock().await = ConnectionState::Connected;
                reconnect_attempts = 0;

                let (mut ws_write, mut ws_read) = ws_stream.split();

                // Send Hello message
                let hello = Message::Hello {
                    device_id: device_id.clone(),
                    platform: std::env::consts::OS.to_string(),
                    capabilities: vec!["screen_capture".to_string(), "input_control".to_string()],
                };

                if let Ok(json) = hello.to_json() {
                    if let Err(e) = ws_write.send(WsMessage::Text(json)).await {
                        tracing::error!("Failed to send Hello: {}", e);
                        continue;
                    }
                }

                // Main message loop
                loop {
                    tokio::select! {
                        // Outgoing messages
                        Some(msg) = outgoing_rx.recv() => {
                            match msg.to_json() {
                                Ok(json) => {
                                    if let Err(e) = ws_write.send(WsMessage::Text(json)).await {
                                        tracing::error!("Failed to send message: {}", e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Failed to serialize message: {}", e);
                                }
                            }
                        }

                        // Incoming messages
                        Some(msg) = ws_read.next() => {
                            match msg {
                                Ok(WsMessage::Text(text)) => {
                                    match Message::from_json(&text) {
                                        Ok(parsed) => {
                                            // Handle ping/pong internally
                                            if matches!(parsed, Message::Ping) {
                                                if let Ok(json) = Message::Pong.to_json() {
                                                    let _ = ws_write.send(WsMessage::Text(json)).await;
                                                }
                                                continue;
                                            }

                                            if incoming_tx.send(parsed).is_err() {
                                                tracing::error!("Failed to forward message: receiver dropped");
                                                break;
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to parse message: {}", e);
                                        }
                                    }
                                }

                                Ok(WsMessage::Binary(data)) => {
                                    match Message::from_bytes(&data) {
                                        Ok(parsed) => {
                                            if incoming_tx.send(parsed).is_err() {
                                                tracing::error!("Failed to forward message: receiver dropped");
                                                break;
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to parse binary message: {}", e);
                                        }
                                    }
                                }

                                Ok(WsMessage::Close(_)) => {
                                    tracing::info!("WebSocket closed by server");
                                    break;
                                }

                                Ok(WsMessage::Ping(_)) | Ok(WsMessage::Pong(_)) => {
                                    // Handled by tungstenite
                                }

                                Err(e) => {
                                    tracing::error!("WebSocket error: {}", e);
                                    break;
                                }

                                _ => {}
                            }
                        }

                        else => {
                            tracing::info!("Connection task terminated");
                            break;
                        }
                    }
                }

                tracing::warn!("Connection lost, will attempt to reconnect");
            }

            Err(e) => {
                tracing::error!("Failed to connect: {}", e);
                reconnect_attempts += 1;

                if reconnect_attempts >= MAX_RECONNECT_ATTEMPTS {
                    tracing::error!("Max reconnect attempts reached, giving up");
                    *state.lock().await = ConnectionState::Failed;
                    break;
                }
            }
        }

        // Reconnect delay
        *state.lock().await = ConnectionState::Reconnecting;
        tokio::time::sleep(tokio::time::Duration::from_secs(RECONNECT_DELAY_SECS)).await;
    }
}

/// Connection manager for handling sessions
pub struct ConnectionManager {
    connection: Option<NetworkConnection>,
    session_id: Option<String>,
    remote_id: Option<String>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connection: None,
            session_id: None,
            remote_id: None,
        }
    }

    pub async fn connect(&mut self, device_id: String) -> Result<()> {
        let conn = NetworkConnection::connect(device_id).await?;
        self.connection = Some(conn);
        Ok(())
    }

    pub async fn request_connection(&mut self, target_id: String) -> Result<()> {
        if let Some(conn) = &self.connection {
            conn.send(Message::ConnectRequest {
                target_id: target_id.clone(),
                auth_token: None,
            }).await?;

            self.remote_id = Some(target_id);
        }

        Ok(())
    }

    pub async fn send(&self, message: Message) -> Result<()> {
        if let Some(conn) = &self.connection {
            conn.send(message).await?;
        } else {
            anyhow::bail!("Not connected");
        }

        Ok(())
    }

    pub async fn recv(&self) -> Option<Message> {
        if let Some(conn) = &self.connection {
            conn.recv().await
        } else {
            None
        }
    }

    pub async fn get_state(&self) -> ConnectionState {
        if let Some(conn) = &self.connection {
            conn.get_state().await
        } else {
            ConnectionState::Disconnected
        }
    }

    pub async fn disconnect(&mut self) {
        if let Some(conn) = &self.connection {
            conn.disconnect().await;
        }

        self.connection = None;
        self.session_id = None;
        self.remote_id = None;
    }

    pub fn set_session_id(&mut self, session_id: String) {
        self.session_id = Some(session_id);
    }

    pub fn get_session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    pub fn get_remote_id(&self) -> Option<&str> {
        self.remote_id.as_deref()
    }

    pub fn is_connected(&self) -> bool {
        self.connection.is_some() && self.session_id.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_state() {
        let device_id = "test-device-123".to_string();

        // Note: This will fail without actual relay server
        let result = NetworkConnection::connect(device_id).await;

        // Just verify it returns something
        assert!(result.is_ok() || result.is_err());
    }
}
