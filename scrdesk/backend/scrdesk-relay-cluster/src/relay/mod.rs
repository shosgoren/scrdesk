mod session;

use scrdesk_shared::config::Config;
use session::{SessionManager, handle_client};
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn start_relay_server(config: Config) -> anyhow::Result<()> {
    let relay_addr = format!("{}:21117", config.server.host);
    tracing::info!("Relay server listening on {} (WebSocket relay)", relay_addr);

    let listener = TcpListener::bind(&relay_addr).await?;
    let manager = Arc::new(SessionManager::new());

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                tracing::info!("New relay connection from {}", addr);
                let manager_clone = manager.clone();

                tokio::spawn(async move {
                    if let Err(e) = handle_client(socket, addr, manager_clone).await {
                        tracing::error!("Error handling client {}: {}", addr, e);
                    }
                });
            }
            Err(e) => {
                tracing::error!("Failed to accept connection: {}", e);
            }
        }
    }
}
