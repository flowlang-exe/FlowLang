//! FlowLang Runtime - Event Loop and Handle Management
//!
//! This module provides a Node.js-style event loop that keeps the process
//! running while there are active handles (servers, timers, etc.)

pub mod handle;

use handle::{HandleId, HandleRegistry, HandleType};
use crate::types::Value;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc, oneshot};
use colored::Colorize;

/// Callback request sent from async tasks to the main event loop
#[derive(Clone)]
pub struct CallbackRequest {
    pub callback: Value,
    pub args: Vec<Value>,
}

/// Web callback request with response channel for synchronous handler execution
pub struct WebCallbackRequest {
    pub callback: Value,
    pub args: Vec<Value>,
    pub response_tx: oneshot::Sender<Value>,
}

/// The FlowLang Runtime manages the event loop and active handles
pub struct Runtime {
    /// Registry of all active handles
    handles: Arc<Mutex<HandleRegistry>>,
    /// Signal to trigger graceful shutdown
    shutdown: Arc<AtomicBool>,
    /// Channel sender for callback requests (fire-and-forget, like timers)
    callback_tx: mpsc::UnboundedSender<CallbackRequest>,
    /// Channel receiver for callback requests
    callback_rx: Arc<Mutex<mpsc::UnboundedReceiver<CallbackRequest>>>,
    /// Channel sender for web callback requests (wait for response)
    web_callback_tx: mpsc::UnboundedSender<WebCallbackRequest>,
    /// Channel receiver for web callback requests
    web_callback_rx: Arc<Mutex<mpsc::UnboundedReceiver<WebCallbackRequest>>>,
}

impl Runtime {
    /// Create a new Runtime instance
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let (web_tx, web_rx) = mpsc::unbounded_channel();
        Runtime {
            handles: Arc::new(Mutex::new(HandleRegistry::new())),
            shutdown: Arc::new(AtomicBool::new(false)),
            callback_tx: tx,
            callback_rx: Arc::new(Mutex::new(rx)),
            web_callback_tx: web_tx,
            web_callback_rx: Arc::new(Mutex::new(web_rx)),
        }
    }
    
    /// Get a clone of the handles Arc for sharing
    pub fn handles(&self) -> Arc<Mutex<HandleRegistry>> {
        self.handles.clone()
    }
    
    /// Get a clone of the callback sender for async tasks (fire-and-forget)
    pub fn callback_sender(&self) -> mpsc::UnboundedSender<CallbackRequest> {
        self.callback_tx.clone()
    }
    
    /// Get a clone of the web callback sender for web handlers (waits for response)
    pub fn web_callback_sender(&self) -> mpsc::UnboundedSender<WebCallbackRequest> {
        self.web_callback_tx.clone()
    }
    
    /// Get a clone of the shutdown signal Arc
    pub fn shutdown_signal(&self) -> Arc<AtomicBool> {
        self.shutdown.clone()
    }
    
    /// Process web callbacks (returns callback with its response channel)
    pub async fn get_web_callback(&self) -> Option<WebCallbackRequest> {
        let mut rx = self.web_callback_rx.lock().await;
        rx.try_recv().ok()
    }
    
    /// Register a new handle and return its ID
    pub async fn register_handle(&self, handle_type: HandleType) -> HandleId {
        let mut registry = self.handles.lock().await;
        registry.add(handle_type)
    }
    
    /// Unregister a handle by ID
    pub async fn unregister_handle(&self, id: HandleId) -> bool {
        let mut registry = self.handles.lock().await;
        registry.remove(id)
    }
    
    /// Get the count of active handles
    pub async fn active_handle_count(&self) -> usize {
        let registry = self.handles.lock().await;
        registry.count()
    }
    
    /// Check if a specific handle exists
    pub async fn has_handle(&self, id: HandleId) -> bool {
        let registry = self.handles.lock().await;
        registry.get(id).is_some()
    }
    
    /// Signal the runtime to shutdown
    pub fn signal_shutdown(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }
    
    /// Check if shutdown was signaled
    pub fn is_shutdown_signaled(&self) -> bool {
        self.shutdown.load(Ordering::SeqCst)
    }
    
    /// Run the event loop until all handles are closed or shutdown is signaled
    /// Returns pending callbacks that need to be executed by the interpreter
    pub async fn run_event_loop_tick(&self) -> Option<CallbackRequest> {
        // Try to receive a callback request (non-blocking)
        let mut rx = self.callback_rx.lock().await;
        match rx.try_recv() {
            Ok(request) => Some(request),
            Err(_) => None,
        }
    }
    
    /// Run the event loop until all handles are closed or shutdown is signaled
    /// This is the main event loop that keeps the process alive while there
    /// are active handles (servers, timers, etc.)
    pub async fn run_until_complete(&self, verbose: bool) {
        // Set up Ctrl+C handler
        let shutdown_signal = self.shutdown.clone();
        let ctrlc_result = tokio::spawn(async move {
            if let Ok(()) = tokio::signal::ctrl_c().await {
                shutdown_signal.store(true, Ordering::SeqCst);
            }
        });
        
        if verbose {
            println!("{}", "🔄 Event loop started...".bright_cyan());
        }
        
        // Main event loop - check for handles or shutdown every 100ms
        loop {
            // Check for shutdown signal
            if self.is_shutdown_signaled() {
                if verbose {
                    println!("{}", "\n⚡ Shutdown signal received".yellow());
                }
                break;
            }
            
            // Check handle count
            let count = self.active_handle_count().await;
            if count == 0 {
                if verbose {
                    println!("{}", "✨ All handles closed, exiting event loop".bright_green());
                }
                break;
            }
            
            // Sleep briefly to avoid busy-waiting
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        // Cleanup: abort the ctrlc handler if still running
        ctrlc_result.abort();
        
        if verbose {
            println!("{}", "🏁 Event loop ended".bright_cyan());
        }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Runtime {
    fn clone(&self) -> Self {
        Runtime {
            handles: self.handles.clone(),
            shutdown: self.shutdown.clone(),
            callback_tx: self.callback_tx.clone(),
            callback_rx: self.callback_rx.clone(),
            web_callback_tx: self.web_callback_tx.clone(),
            web_callback_rx: self.web_callback_rx.clone(),
        }
    }
}

