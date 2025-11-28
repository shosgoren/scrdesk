use scrdesk_shared::config::Config;
use tokio::net::TcpListener;

pub async fn start_relay_server(config: Config) -> anyhow::Result<()> {
    let relay_addr = format!("{}:21117", config.server.host);
    tracing::info!("Relay server listening on {} (RustDesk protocol)", relay_addr);

    let listener = TcpListener::bind(&relay_addr).await?;

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                tracing::info!("New relay connection from {}", addr);
                tokio::spawn(async move {
                    // Handle RustDesk relay protocol
                    // TODO: Implement full RustDesk relay protocol
                    tracing::info!("Handling connection from {}", addr);
                });
            }
            Err(e) => {
                tracing::error!("Failed to accept connection: {}", e);
            }
        }
    }
}
