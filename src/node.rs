use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub region: String,
    pub status: NodeStatus,
    pub load: f32,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeStatus {
    Online,
    Offline,
    Maintenance,
}

pub struct NodeManager {
    nodes: Arc<DashMap<String, NodeInfo>>,
}

impl NodeManager {
    pub fn new() -> Self {
        NodeManager {
            nodes: Arc::new(DashMap::new()),
        }
    }

    pub fn add_node(&self, node: NodeInfo) -> anyhow::Result<()> {
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    pub fn remove_node(&self, node_id: &str) -> anyhow::Result<Option<NodeInfo>> {
        Ok(self.nodes.remove(node_id).map(|(_, v)| v))
    }

    pub fn get_node(&self, node_id: &str) -> Option<NodeInfo> {
        self.nodes.get(node_id).map(|entry| entry.clone())
    }

    pub fn list_nodes(&self) -> Vec<NodeInfo> {
        self.nodes
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    pub fn list_online_nodes(&self) -> Vec<NodeInfo> {
        self.nodes
            .iter()
            .filter(|entry| entry.status == NodeStatus::Online)
            .map(|entry| entry.value().clone())
            .collect()
    }

    pub fn update_node_status(&self, node_id: &str, status: NodeStatus) -> anyhow::Result<()> {
        if let Some(mut node) = self.nodes.get_mut(node_id) {
            node.status = status;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Node {} not found", node_id))
        }
    }

    pub fn update_node_load(&self, node_id: &str, load: f32) -> anyhow::Result<()> {
        if let Some(mut node) = self.nodes.get_mut(node_id) {
            node.load = load;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Node {} not found", node_id))
        }
    }

    pub fn select_best_node(&self) -> Option<NodeInfo> {
        self.list_online_nodes()
            .into_iter()
            .min_by(|a, b| a.load.partial_cmp(&b.load).unwrap_or(std::cmp::Ordering::Equal))
    }
}

impl Clone for NodeManager {
    fn clone(&self) -> Self {
        NodeManager {
            nodes: Arc::clone(&self.nodes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_node() {
        let manager = NodeManager::new();
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

        manager.add_node(node.clone()).unwrap();
        let retrieved = manager.get_node("node1").unwrap();
        assert_eq!(retrieved.id, "node1");
    }
}
