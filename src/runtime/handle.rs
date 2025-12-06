//! Handle types and registry for the FlowLang event loop
//!
//! Handles represent active resources that keep the process alive:
//! - Timers (interval, timeout)
//! - Servers (HTTP, WebSocket, TCP)
//! - Connections
//! - File watchers

use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::oneshot;

/// Unique identifier for a handle
pub type HandleId = u64;

/// Types of handles that can be registered
#[derive(Debug)]
pub enum HandleType {
    /// Interval timer that fires repeatedly
    Interval {
        interval_ms: u64,
        /// Channel to signal cancellation
        cancel_tx: Option<oneshot::Sender<()>>,
    },
    
    /// One-shot timeout timer
    Timeout {
        delay_ms: u64,
        /// Channel to signal cancellation
        cancel_tx: Option<oneshot::Sender<()>>,
    },
    
    /// HTTP server listening on a port
    HttpServer {
        port: u16,
        /// Channel to signal shutdown
        shutdown_tx: Option<oneshot::Sender<()>>,
    },
    
    /// TCP server listening on a port
    TcpServer {
        port: u16,
        shutdown_tx: Option<oneshot::Sender<()>>,
    },
    
    /// WebSocket server
    WebSocketServer {
        port: u16,
        shutdown_tx: Option<oneshot::Sender<()>>,
    },
    
    /// Generic handle for future extensions
    Generic {
        name: String,
    },
}

impl HandleType {
    /// Get a human-readable name for the handle type
    pub fn type_name(&self) -> &'static str {
        match self {
            HandleType::Interval { .. } => "Interval",
            HandleType::Timeout { .. } => "Timeout",
            HandleType::HttpServer { .. } => "HttpServer",
            HandleType::TcpServer { .. } => "TcpServer",
            HandleType::WebSocketServer { .. } => "WebSocketServer",
            HandleType::Generic { .. } => "Generic",
        }
    }
}

/// A registered handle with metadata
#[derive(Debug)]
pub struct Handle {
    /// Unique ID for this handle
    pub id: HandleId,
    /// The type and data of this handle
    pub handle_type: HandleType,
    /// When this handle was created
    pub created_at: Instant,
}

impl Handle {
    /// Create a new handle
    pub fn new(id: HandleId, handle_type: HandleType) -> Self {
        Handle {
            id,
            handle_type,
            created_at: Instant::now(),
        }
    }
    
    /// Get how long this handle has been alive
    pub fn age_ms(&self) -> u128 {
        self.created_at.elapsed().as_millis()
    }
}

/// Registry that tracks all active handles
#[derive(Debug)]
pub struct HandleRegistry {
    /// Map of handle ID to handle
    handles: HashMap<HandleId, Handle>,
    /// Counter for generating unique IDs
    next_id: HandleId,
}

impl HandleRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        HandleRegistry {
            handles: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// Add a new handle and return its ID
    pub fn add(&mut self, handle_type: HandleType) -> HandleId {
        let id = self.next_id;
        self.next_id += 1;
        
        let handle = Handle::new(id, handle_type);
        self.handles.insert(id, handle);
        
        id
    }
    
    /// Remove a handle by ID, returns true if it existed
    pub fn remove(&mut self, id: HandleId) -> bool {
        self.handles.remove(&id).is_some()
    }
    
    /// Get a handle by ID
    pub fn get(&self, id: HandleId) -> Option<&Handle> {
        self.handles.get(&id)
    }
    
    /// Get a mutable handle by ID
    pub fn get_mut(&mut self, id: HandleId) -> Option<&mut Handle> {
        self.handles.get_mut(&id)
    }
    
    /// Get the number of active handles
    pub fn count(&self) -> usize {
        self.handles.len()
    }
    
    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.handles.is_empty()
    }
    
    /// Get all handle IDs
    pub fn ids(&self) -> Vec<HandleId> {
        self.handles.keys().cloned().collect()
    }
    
    /// Get summary of all handles for debugging
    pub fn summary(&self) -> String {
        if self.is_empty() {
            return "No active handles".to_string();
        }
        
        let mut parts = Vec::new();
        for handle in self.handles.values() {
            parts.push(format!("{}(#{})", handle.handle_type.type_name(), handle.id));
        }
        parts.join(", ")
    }
}

impl Default for HandleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_registry_add_remove() {
        let mut registry = HandleRegistry::new();
        
        let id1 = registry.add(HandleType::Generic { name: "test1".into() });
        let id2 = registry.add(HandleType::Generic { name: "test2".into() });
        
        assert_eq!(registry.count(), 2);
        assert!(registry.get(id1).is_some());
        assert!(registry.get(id2).is_some());
        
        assert!(registry.remove(id1));
        assert_eq!(registry.count(), 1);
        assert!(registry.get(id1).is_none());
        
        assert!(!registry.remove(id1)); // Already removed
    }
}
