//! ZAB-Sync - Distributed synchronization protocol

use crate::QwartzError;

/// ZAB-Sync Node information
#[derive(Debug, Clone)]
pub struct SyncNode {
    /// Node ID
    pub id: String,
    /// Node address
    pub address: String,
    /// Port
    pub port: u16,
    /// Last sync timestamp
    pub last_sync: u64,
    /// Connection status
    pub connected: bool,
}

impl SyncNode {
    /// Create new sync node
    pub fn new(id: &str, address: &str, port: u16) -> Self {
        Self {
            id: id.to_string(),
            address: address.to_string(),
            port,
            last_sync: 0,
            connected: false,
        }
    }
}

/// ZAB-Sync Protocol
pub struct ZABSync {
    /// Local node ID
    local_id: String,
    /// Known peers
    peers: Vec<SyncNode>,
    /// Sync counter
    sync_count: u64,
}

impl ZABSync {
    /// Create new sync manager
    pub fn new(local_id: &str) -> Self {
        Self {
            local_id: local_id.to_string(),
            peers: Vec::new(),
            sync_count: 0,
        }
    }

    /// Add peer node
    pub fn add_peer(&mut self, node: SyncNode) {
        self.peers.push(node);
    }

    /// Remove peer
    pub fn remove_peer(&mut self, id: &str) {
        self.peers.retain(|p| p.id != id);
    }

    /// Get connected peers
    pub fn connected_peers(&self) -> Vec<&SyncNode> {
        self.peers.iter().filter(|p| p.connected).collect()
    }

    /// Sync with peer
    pub async fn sync_with(&mut self, peer_id: &str) -> Result<Vec<u8>, QwartzError> {
        // Find peer
        let peer = self.peers.iter_mut().find(|p| p.id == peer_id);
        
        if let Some(peer) = peer {
            peer.last_sync = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            peer.connected = true;
            self.sync_count += 1;
            
            // Return sync data
            Ok(format!("SYNC:{}:{}", self.local_id, self.sync_count).into_bytes())
        } else {
            Err(QwartzError::SyncError("Peer not found".into()))
        }
    }

    /// Broadcast sync to all peers
    pub async fn broadcast(&mut self, data: &[u8]) -> Result<usize, QwartzError> {
        let mut synced = 0;
        
        for peer in &mut self.peers {
            peer.last_sync = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            peer.connected = true;
            synced += 1;
        }
        
        self.sync_count += 1;
        
        // In real implementation, would send data to each peer
        Ok(synced)
    }

    /// Check peer health
    pub fn check_health(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Mark peers as disconnected if no sync in 60 seconds
        for peer in &mut self.peers {
            if now - peer.last_sync > 60 {
                peer.connected = false;
            }
        }
    }

    /// Get sync statistics
    pub fn stats(&self) -> SyncStats {
        SyncStats {
            local_id: self.local_id.clone(),
            peer_count: self.peers.len(),
            connected_count: self.peers.iter().filter(|p| p.connected).count(),
            sync_count: self.sync_count,
        }
    }
}

/// Sync statistics
#[derive(Debug, Clone)]
pub struct SyncStats {
    pub local_id: String,
    pub peer_count: usize,
    pub connected_count: usize,
    pub sync_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sync() {
        let mut sync = ZABSync::new("node1");
        sync.add_peer(SyncNode::new("node2", "192.168.1.1", 9000));
        
        let result = sync.sync_with("node2").await;
        assert!(result.is_ok());
        
        let stats = sync.stats();
        assert_eq!(stats.peer_count, 1);
        assert_eq!(stats.connected_count, 1);
    }
}
