mod node;
mod forwarder;
mod config;

use anyhow::Result;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting VPN Airport Server...");

    // Load configuration
    let config = config::Config::default();
    info!("Configuration loaded: {:?}", config);

    // Initialize node manager
    let node_manager = node::NodeManager::new();
    info!("Node manager initialized");

    // Initialize forwarder
    let forwarder = forwarder::DataForwarder::new(config.clone());
    info!("Data forwarder initialized");

    // Start listening for connections
    let listener_addr = format!("{}:{}", config.bind_addr, config.bind_port);
    let listener = tokio::net::TcpListener::bind(&listener_addr).await?;
    info!("Server listening on {}", listener_addr);

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                info!("New connection from {}", addr);
                let forwarder_clone = forwarder.clone();
                
                tokio::spawn(async move {
                    if let Err(e) = forwarder_clone.handle_connection(socket, addr).await {
                        warn!("Error handling connection from {}: {}", addr, e);
                    }
                });
            }
            Err(e) => {
                warn!("Error accepting connection: {}", e);
            }
        }
    }
}
