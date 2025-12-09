use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio_tungstenite::{accept_async, tungstenite::Message as WsMessage, WebSocketStream};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
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
    Ping,
    Pong,
    Disconnect {
        reason: Option<String>,
    },
    // All other messages are relayed as-is
    Relay {
        data: Vec<u8>,
    },
}

pub struct Client {
    pub device_id: String,
    pub platform: String,
    pub tx: mpsc::UnboundedSender<WsMessage>,
}

pub struct Session {
    pub id: String,
    pub client_a: String,
    pub client_b: String,
    pub created_at: std::time::Instant,
}

pub struct SessionManager {
    clients: Arc<RwLock<HashMap<String, Client>>>,
    sessions: Arc<RwLock<HashMap<String, Session>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_client(&self, device_id: String, platform: String, tx: mpsc::UnboundedSender<WsMessage>) {
        let client = Client {
            device_id: device_id.clone(),
            platform,
            tx,
        };

        let mut clients = self.clients.write().await;
        clients.insert(device_id.clone(), client);
        tracing::info!("Client registered: {}", device_id);
    }

    pub async fn unregister_client(&self, device_id: &str) {
        let mut clients = self.clients.write().await;
        clients.remove(device_id);
        tracing::info!("Client unregistered: {}", device_id);

        // Remove any sessions involving this client
        let mut sessions = self.sessions.write().await;
        sessions.retain(|_, session| {
            session.client_a != device_id && session.client_b != device_id
        });
    }

    pub async fn create_session(&self, client_a: String, client_b: String) -> Result<String> {
        let session_id = uuid::Uuid::new_v4().to_string();

        let session = Session {
            id: session_id.clone(),
            client_a: client_a.clone(),
            client_b: client_b.clone(),
            created_at: std::time::Instant::now(),
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session);

        tracing::info!("Session created: {} ({} <-> {})", session_id, client_a, client_b);

        Ok(session_id)
    }

    pub async fn relay_message(&self, from: &str, to: &str, message: WsMessage) -> Result<()> {
        let clients = self.clients.read().await;

        if let Some(client) = clients.get(to) {
            client.tx.send(message)
                .map_err(|_| anyhow::anyhow!("Failed to send message to {}", to))?;
            tracing::debug!("Relayed message from {} to {}", from, to);
        } else {
            tracing::warn!("Target client not found: {}", to);
        }

        Ok(())
    }

    pub async fn get_peer(&self, device_id: &str) -> Option<String> {
        let sessions = self.sessions.read().await;

        for session in sessions.values() {
            if session.client_a == device_id {
                return Some(session.client_b.clone());
            } else if session.client_b == device_id {
                return Some(session.client_a.clone());
            }
        }

        None
    }

    pub async fn client_exists(&self, device_id: &str) -> bool {
        let clients = self.clients.read().await;
        clients.contains_key(device_id)
    }
}

pub async fn handle_client(
    socket: TcpStream,
    addr: std::net::SocketAddr,
    manager: Arc<SessionManager>,
) -> Result<()> {
    let ws_stream = accept_async(socket).await
        .map_err(|e| anyhow::anyhow!("WebSocket handshake failed: {}", e))?;

    let (mut ws_write, mut ws_read) = ws_stream.split();

    // Create channel for outgoing messages
    let (tx, mut rx) = mpsc::unbounded_channel::<WsMessage>();

    let mut device_id: Option<String> = None;
    let mut authenticated = false;

    // Spawn task to send outgoing messages
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = ws_write.send(msg).await {
                tracing::error!("Failed to send message: {}", e);
                break;
            }
        }
    });

    // Handle incoming messages
    while let Some(msg) = ws_read.next().await {
        match msg {
            Ok(WsMessage::Text(text)) => {
                match serde_json::from_str::<Message>(&text) {
                    Ok(Message::Hello { device_id: id, platform, capabilities }) => {
                        device_id = Some(id.clone());
                        manager.register_client(id.clone(), platform, tx.clone()).await;
                        authenticated = true;

                        tracing::info!("Client authenticated: {} from {}", id, addr);

                        // Send acknowledgment
                        let ack = serde_json::to_string(&Message::ConnectResponse {
                            success: true,
                            session_id: Some(id.clone()),
                            error: None,
                        }).unwrap();
                        let _ = tx.send(WsMessage::Text(ack));
                    }

                    Ok(Message::ConnectRequest { target_id, auth_token }) => {
                        if !authenticated || device_id.is_none() {
                            let error = serde_json::to_string(&Message::ConnectResponse {
                                success: false,
                                session_id: None,
                                error: Some("Not authenticated".to_string()),
                            }).unwrap();
                            let _ = tx.send(WsMessage::Text(error));
                            continue;
                        }

                        let from_id = device_id.as_ref().unwrap();

                        // Check if target exists
                        if !manager.client_exists(&target_id).await {
                            let error = serde_json::to_string(&Message::ConnectResponse {
                                success: false,
                                session_id: None,
                                error: Some("Target device not found".to_string()),
                            }).unwrap();
                            let _ = tx.send(WsMessage::Text(error));
                            continue;
                        }

                        // Create session
                        match manager.create_session(from_id.clone(), target_id.clone()).await {
                            Ok(session_id) => {
                                let response = serde_json::to_string(&Message::ConnectResponse {
                                    success: true,
                                    session_id: Some(session_id.clone()),
                                    error: None,
                                }).unwrap();
                                let _ = tx.send(WsMessage::Text(response.clone()));

                                // Notify target
                                let _ = manager.relay_message(
                                    from_id,
                                    &target_id,
                                    WsMessage::Text(response),
                                ).await;
                            }
                            Err(e) => {
                                let error = serde_json::to_string(&Message::ConnectResponse {
                                    success: false,
                                    session_id: None,
                                    error: Some(format!("Failed to create session: {}", e)),
                                }).unwrap();
                                let _ = tx.send(WsMessage::Text(error));
                            }
                        }
                    }

                    Ok(Message::Ping) => {
                        let pong = serde_json::to_string(&Message::Pong).unwrap();
                        let _ = tx.send(WsMessage::Text(pong));
                    }

                    Ok(Message::Disconnect { .. }) => {
                        tracing::info!("Client requested disconnect: {:?}", device_id);
                        break;
                    }

                    Ok(_) | Err(_) => {
                        // Relay all other messages to peer
                        if let Some(ref dev_id) = device_id {
                            if let Some(peer_id) = manager.get_peer(dev_id).await {
                                let _ = manager.relay_message(dev_id, &peer_id, WsMessage::Text(text)).await;
                            }
                        }
                    }
                }
            }

            Ok(WsMessage::Binary(data)) => {
                // Relay binary messages (video frames, etc.) to peer
                if let Some(ref dev_id) = device_id {
                    if let Some(peer_id) = manager.get_peer(dev_id).await {
                        let _ = manager.relay_message(dev_id, &peer_id, WsMessage::Binary(data)).await;
                    }
                }
            }

            Ok(WsMessage::Close(_)) => {
                tracing::info!("Client closed connection: {:?}", device_id);
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

    // Cleanup
    if let Some(dev_id) = device_id {
        manager.unregister_client(&dev_id).await;
    }

    send_task.abort();

    Ok(())
}
