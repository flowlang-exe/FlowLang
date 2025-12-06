use crate::error::FlowError;
use crate::types::{NativeFn, Value};
use chrono::{DateTime, Local, Utc};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn load_time_module() -> Vec<(&'static str, Value)> {
    vec![
        ("now", Value::NativeFunction(NativeFn::new(time_now))),
        ("format", Value::NativeFunction(NativeFn::new(time_format))),
        ("sleep", Value::NativeFunction(NativeFn::new(time_sleep))),
        ("timestamp", Value::NativeFunction(NativeFn::new(time_timestamp))),
    ]
}

// time::now() -> Silk
fn time_now(_args: Vec<Value>) -> Result<Value, FlowError> {
    let now = Local::now();
    Ok(Value::String(Arc::new(now.to_rfc3339())))
}

// time::format(format_string: Silk) -> Silk
fn time_format(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime(
            "time::format expects 1 argument (format string)",
            0,
            0,
        ));
    }

    let format_str = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "time::format expects a string format",
                0,
                0,
            ))
        }
    };

    let now = Local::now();
    let formatted = now.format(&format_str).to_string();
    Ok(Value::String(Arc::new(formatted)))
}

// time::sleep(seconds: Ember) -> Hollow
fn time_sleep(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime(
            "time::sleep expects 1 argument (seconds)",
            0,
            0,
        ));
    }

    let seconds = match &args[0] {
        Value::Number(n) => *n,
        _ => {
            return Err(FlowError::type_error(
                "time::sleep expects a number",
                0,
                0,
            ))
        }
    };

    if seconds < 0.0 {
        return Err(FlowError::runtime(
            "time::sleep expects a non-negative number",
            0,
            0,
        ));
    }

    let duration = Duration::from_secs_f64(seconds);
    thread::sleep(duration);
    
    Ok(Value::Null)
}

// time::timestamp() -> Ember
fn time_timestamp(_args: Vec<Value>) -> Result<Value, FlowError> {
    let now = Utc::now();
    let timestamp = now.timestamp() as f64;
    Ok(Value::Number(timestamp))
}
