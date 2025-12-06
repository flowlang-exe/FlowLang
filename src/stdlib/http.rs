use crate::error::FlowError;
use crate::types::{NativeFn, Value};
use std::sync::Arc;

pub fn load_http_module() -> Vec<(&'static str, Value)> {
    vec![
        ("get", Value::NativeFunction(NativeFn::new(http_get))),
        ("post", Value::NativeFunction(NativeFn::new(http_post))),
    ]
}

// http::get(url: Silk) -> Silk
fn http_get(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "http::get expects 1 argument (url)",
            0,
            0,
        ));
    }

    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "http::get expects a string URL",
                0,
                0,
            ))
        }
    };

    // Use blocking reqwest client
    let result = reqwest::blocking::get(&**url)
        .and_then(|resp| resp.error_for_status())
        .and_then(|resp| resp.text());

    match result {
        Ok(body) => Ok(Value::String(Arc::new(body))),
        Err(e) => Err(FlowError::runtime(&format!("HTTP GET failed: {}", e), 0, 0)),
    }
}

// http::post(url: Silk, data: Silk) -> Silk
fn http_post(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 2 {
        return Err(FlowError::runtime(
            "http::post expects 2 arguments (url, data)",
            0,
            0,
        ));
    }

    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "http::post expects a string URL",
                0,
                0,
            ))
        }
    };

    let data = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "http::post expects a string data",
                0,
                0,
            ))
        }
    };

    // Use blocking reqwest client
    let client = reqwest::blocking::Client::new();
    let result = client
        .post(&**url)
        .body((*data).clone())
        .send()
        .and_then(|resp| resp.error_for_status())
        .and_then(|resp| resp.text());

    match result {
        Ok(body) => Ok(Value::String(Arc::new(body))),
        Err(e) => Err(FlowError::runtime(&format!("HTTP POST failed: {}", e), 0, 0)),
    }
}
