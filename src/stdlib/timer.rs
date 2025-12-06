//! Timer standard library for FlowLang
//!
//! Provides timer functions that keep the process alive:
//! - `timer.interval(ms, callback)` - Repeating timer
//! - `timer.timeout(ms, callback)` - One-shot timer
//! - `timer.clear(handle)` - Cancel a timer

use crate::error::FlowError;
use crate::types::{AsyncNativeFn, Value, AsyncContext};
use crate::runtime::handle::HandleType;
use std::sync::Arc;
use tokio::sync::oneshot;

pub fn load_timer_module() -> Vec<(&'static str, Value)> {
    vec![
        ("interval", Value::AsyncNativeFunction(AsyncNativeFn::new(timer_interval))),
        ("timeout", Value::AsyncNativeFunction(AsyncNativeFn::new(timer_timeout))),
        ("clear", Value::AsyncNativeFunction(AsyncNativeFn::new(timer_clear))),
    ]
}

/// timer.interval(ms, callback) -> Handle
/// Creates a repeating timer that calls the callback every `ms` milliseconds.
/// Returns a handle that can be used to cancel the timer.
async fn timer_interval(args: Vec<Value>, ctx: AsyncContext) -> Result<Value, FlowError> {
    if args.len() != 2 {
        return Err(FlowError::runtime(
            "timer.interval expects 2 arguments (ms, callback)",
            0, 0,
        ));
    }

    let ms = match &args[0] {
        Value::Number(n) => *n as u64,
        _ => return Err(FlowError::type_error(
            "timer.interval expects a number for ms",
            0, 0,
        )),
    };

    let callback = match &args[1] {
        Value::Function { .. } | Value::NativeFunction(_) => args[1].clone(),
        _ => return Err(FlowError::type_error(
            "timer.interval expects a Spell (function) as callback",
            0, 0,
        )),
    };

    // Create cancellation channel
    let (cancel_tx, mut cancel_rx) = oneshot::channel::<()>();

    // Register the handle
    let handle_id = ctx.runtime.register_handle(HandleType::Interval {
        interval_ms: ms,
        cancel_tx: Some(cancel_tx),
    }).await;

    // Get callback sender for sending callback requests to main event loop
    let callback_tx = ctx.runtime.callback_sender();
    
    // Clone runtime for the spawned task
    let runtime = ctx.runtime.clone();

    // Spawn the interval task
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(ms));
        
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // Send callback request to main event loop
                    let request = crate::runtime::CallbackRequest {
                        callback: callback.clone(),
                        args: vec![],
                    };
                    let _ = callback_tx.send(request);
                }
                _ = &mut cancel_rx => {
                    // Timer cancelled
                    break;
                }
            }
        }
        
        // Unregister handle when done
        runtime.unregister_handle(handle_id).await;
    });

    Ok(Value::Handle(handle_id))
}

/// timer.timeout(ms, callback) -> Handle
/// Creates a one-shot timer that calls the callback after `ms` milliseconds.
/// Returns a handle that can be used to cancel the timer.
async fn timer_timeout(args: Vec<Value>, ctx: AsyncContext) -> Result<Value, FlowError> {
    if args.len() != 2 {
        return Err(FlowError::runtime(
            "timer.timeout expects 2 arguments (ms, callback)",
            0, 0,
        ));
    }

    let ms = match &args[0] {
        Value::Number(n) => *n as u64,
        _ => return Err(FlowError::type_error(
            "timer.timeout expects a number for ms",
            0, 0,
        )),
    };

    let callback = match &args[1] {
        Value::Function { .. } | Value::NativeFunction(_) => args[1].clone(),
        _ => return Err(FlowError::type_error(
            "timer.timeout expects a Spell (function) as callback",
            0, 0,
        )),
    };

    // Create cancellation channel
    let (cancel_tx, cancel_rx) = oneshot::channel::<()>();

    // Register the handle
    let handle_id = ctx.runtime.register_handle(HandleType::Timeout {
        delay_ms: ms,
        cancel_tx: Some(cancel_tx),
    }).await;

    // Get callback sender
    let callback_tx = ctx.runtime.callback_sender();
    
    // Clone runtime for the spawned task
    let runtime = ctx.runtime.clone();

    // Spawn the timeout task
    tokio::spawn(async move {
        let sleep = tokio::time::sleep(tokio::time::Duration::from_millis(ms));
        
        tokio::select! {
            _ = sleep => {
                // Send callback request to main event loop
                let request = crate::runtime::CallbackRequest {
                    callback: callback.clone(),
                    args: vec![],
                };
                let _ = callback_tx.send(request);
            }
            _ = cancel_rx => {
                // Timer cancelled, do nothing
            }
        }
        
        // Unregister handle when done (either completed or cancelled)
        runtime.unregister_handle(handle_id).await;
    });

    Ok(Value::Handle(handle_id))
}

/// timer.clear(handle) -> Pulse
/// Cancels a timer by its handle. Returns true if the timer was found and cancelled.
async fn timer_clear(args: Vec<Value>, ctx: AsyncContext) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "timer.clear expects 1 argument (handle)",
            0, 0,
        ));
    }

    let handle_id = match &args[0] {
        Value::Handle(id) => *id,
        Value::Number(n) => *n as u64,
        _ => return Err(FlowError::type_error(
            "timer.clear expects a Handle",
            0, 0,
        )),
    };

    // Get the handle registry Arc first, then lock it
    let handles = ctx.runtime.handles();
    let mut registry = handles.lock().await;
    
    if let Some(handle) = registry.get_mut(handle_id) {
        // Send cancel signal based on handle type
        match &mut handle.handle_type {
            HandleType::Interval { cancel_tx, .. } => {
                if let Some(tx) = cancel_tx.take() {
                    let _ = tx.send(());
                }
            }
            HandleType::Timeout { cancel_tx, .. } => {
                if let Some(tx) = cancel_tx.take() {
                    let _ = tx.send(());
                }
            }
            _ => {}
        }
        
        // Remove the handle from registry
        registry.remove(handle_id);
        
        Ok(Value::Boolean(true))
    } else {
        Ok(Value::Boolean(false))
    }
}
