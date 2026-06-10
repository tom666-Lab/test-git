use crate::config::Config;
use anyhow::Result;
use bytes::{BytesMut, buf::BufMut};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{info, error};

#[derive(Clone)]
pub struct DataForwarder {
    config: Config,
}

impl DataForwarder {
    pub fn new(config: Config) -> Self {
        DataForwarder { config }
    }

    pub async fn handle_connection(&self, mut client: TcpStream, client_addr: SocketAddr) -> Result<()> {
        info!("Handling connection from {}", client_addr);

        // Read initial data from client
        let mut buffer = BytesMut::with_capacity(self.config.buffer_size);
        let n = client.read_buf(&mut buffer).await?;

        if n == 0 {
            info!("Connection from {} closed (no data)", client_addr);
            return Ok(());
        }

        info!("Received {} bytes from {}", n, client_addr);

        // Parse request (simplified example - you would implement proper protocol parsing)
        let request = String::from_utf8_lossy(&buffer[..n]);
        info!("Request: {}", request);

        // Forward data to target node (example - hardcoded for now)
        let target_addr = "127.0.0.1:9999";
        match TcpStream::connect(target_addr).await {
            Ok(mut server) => {
                info!("Connected to target node at {}", target_addr);
                
                // Send data to server
                server.write_all(&buffer[..n]).await?;
                
                // Read response from server
                let mut response_buffer = BytesMut::with_capacity(self.config.buffer_size);
                let resp_n = server.read_buf(&mut response_buffer).await?;
                
                if resp_n > 0 {
                    // Send response back to client
                    client.write_all(&response_buffer[..resp_n]).await?;
                    client.flush().await?;
                    info!("Forwarded {} bytes response to {}", resp_n, client_addr);
                }
            }
            Err(e) => {
                error!("Failed to connect to target node: {}", e);
                let error_msg = "HTTP/1.1 503 Service Unavailable\r\n\r\n";
                client.write_all(error_msg.as_bytes()).await?;
                client.flush().await?;
            }
        }

        Ok(())
    }

    pub async fn forward_to_node(&self, data: &[u8], node_addr: &str) -> Result<Vec<u8>> {
        let mut server = TcpStream::connect(node_addr).await?;
        server.write_all(data).await?;

        let mut response = BytesMut::with_capacity(self.config.buffer_size);
        server.read_buf(&mut response).await?;

        Ok(response.to_vec())
    }

    pub fn should_compress(&self) -> bool {
        self.config.enable_compression
    }

    pub fn should_encrypt(&self) -> bool {
        self.config.enable_encryption
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forwarder_creation() {
        let config = Config::default();
        let forwarder = DataForwarder::new(config);
        assert!(forwarder.should_compress());
        assert!(forwarder.should_encrypt());
    }
}
