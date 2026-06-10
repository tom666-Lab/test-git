# VPN Airport - Rust Implementation

A high-performance VPN airport server written in Rust with support for multiple nodes, data forwarding, and load balancing.

## Features

- 🚀 **Async/Await** - Built on Tokio for high concurrency
- 🌍 **Multi-Node Support** - Manage multiple VPN nodes across regions
- 📊 **Load Balancing** - Automatic node selection based on load
- 🔄 **Data Forwarding** - Efficient TCP connection forwarding
- 🔐 **Security Ready** - Built-in encryption and compression support
- 📈 **Monitoring** - Real-time node status and load tracking

## Quick Start

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

## Configuration

Create a `config.json` file:

```json
{
  "bind_addr": "0.0.0.0",
  "bind_port": 8888,
  "max_connections": 1000,
  "buffer_size": 65536,
  "timeout_secs": 300,
  "enable_compression": true,
  "enable_encryption": true
}
```

## Project Structure

```
src/
├── main.rs          # Server entry point
├── config.rs        # Configuration management
├── node.rs          # Node management system
└── forwarder.rs     # Data forwarding engine
```

## Modules

### Node Manager (`node.rs`)
- Add/remove VPN nodes
- Track node status (Online, Offline, Maintenance)
- Monitor node load
- Select best node for routing

### Data Forwarder (`forwarder.rs`)
- Handle incoming connections
- Forward data to target nodes
- Manage response routing
- Support compression and encryption

### Configuration (`config.rs`)
- Load configuration from JSON files
- Default configuration settings
- Runtime configuration updates

## API Examples

### Add a Node

```rust
let node = NodeInfo {
    id: "node1".to_string(),
    name: "US Server 1".to_string(),
    address: "192.168.1.1".to_string(),
    port: 443,
    region: "US".to_string(),
    status: NodeStatus::Online,
    load: 0.5,
    created_at: 0,
};

manager.add_node(node)?;
```

### Select Best Node

```rust
if let Some(node) = manager.select_best_node() {
    println!("Best node: {}", node.name);
}
```

## Testing

Run tests with:

```bash
cargo test
```

## Performance

- Supports 1000+ concurrent connections
- Configurable buffer size (default: 64KB)
- Non-blocking I/O with Tokio
- Efficient connection pooling

## Dependencies

- **tokio** - Async runtime
- **bytes** - Efficient byte buffer handling
- **dashmap** - Concurrent hashmap
- **serde** - Serialization/deserialization
- **tracing** - Structured logging

## Future Enhancements

- [ ] WebSocket support
- [ ] gRPC control plane
- [ ] Redis integration for distributed state
- [ ] Metrics export (Prometheus)
- [ ] Container support (Docker)
- [ ] Kubernetes deployment

## License

MIT

## Contributing

Contributions welcome! Please submit PRs with tests.
