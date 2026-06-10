use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bind_addr: String,
    pub bind_port: u16,
    pub max_connections: usize,
    pub buffer_size: usize,
    pub timeout_secs: u64,
    pub enable_compression: bool,
    pub enable_encryption: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bind_addr: "0.0.0.0".to_string(),
            bind_port: 8888,
            max_connections: 1000,
            buffer_size: 65536,
            timeout_secs: 300,
            enable_compression: true,
            enable_encryption: true,
        }
    }
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn to_file(&self, path: &str) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
