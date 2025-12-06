use crate::error::FlowError;
use crate::types::{NativeFn, Value};
use std::sync::Arc;

pub fn load_net_module() -> Vec<(&'static str, Value)> {
    vec![
        ("get", Value::NativeFunction(NativeFn::new(net_get))),
        ("post", Value::NativeFunction(NativeFn::new(net_post))),
        ("put", Value::NativeFunction(NativeFn::new(net_put))),
        ("patch", Value::NativeFunction(NativeFn::new(net_patch))),
        ("delete", Value::NativeFunction(NativeFn::new(net_delete))),
        ("head", Value::NativeFunction(NativeFn::new(net_head))),
        ("request", Value::NativeFunction(NativeFn::new(net_request))),
    ]
}

// net::get(url: Silk) -> Silk
fn net_get(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "net::get expects 1 argument (url)",
            0,
            0,
        ));
    }

    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "net::get expects a string URL",
                0,
                0,
            ))
        }
    };

    let result = reqwest::blocking::get(&**url)
        .and_then(|resp| resp.error_for_status())
        .and_then(|resp| resp.text());

    match result {
        Ok(body) => Ok(Value::String(Arc::new(body))),
        Err(e) => Err(FlowError::runtime(&format!("HTTP GET failed: {}", e), 0, 0)),
    }
}

// net::post(url: Silk, data: Silk) -> Silk
fn net_post(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 2 {
        return Err(FlowError::runtime(
            "net::post expects 2 arguments (url, data)",
            0,
            0,
        ));
    }

    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "net::post expects a string URL",
                0,
                0,
            ))
        }
    };

    let data = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "net::post expects a string data",
                0,
                0,
            ))
        }
    };

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

// net::put(url: Silk, data: Silk) -> Silk
fn net_put(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 2 {
        return Err(FlowError::runtime(
            "net::put expects 2 arguments (url, data)",
            0,
            0,
        ));
    }

    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "net::put expects a string URL",
                0,
                0,
            ))
        }
    };

    let data = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "net::put expects a string data",
                0,
                0,
            ))
        }
    };

    let client = reqwest::blocking::Client::new();
    let result = client
        .put(&**url)
        .body((*data).clone())
        .send()
        .and_then(|resp| resp.error_for_status())
        .and_then(|resp| resp.text());

    match result {
        Ok(body) => Ok(Value::String(Arc::new(body))),
        Err(e) => Err(FlowError::runtime(&format!("HTTP PUT failed: {}", e), 0, 0)),
    }
}

// net::patch(url: Silk, data: Silk) -> Silk
fn net_patch(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 2 {
        return Err(FlowError::runtime(
            "net::patch expects 2 arguments (url, data)",
            0,
            0,
        ));
    }

    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "net::patch expects a string URL",
                0,
                0,
            ))
        }
    };

    let data = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "net::patch expects a string data",
                0,
                0,
            ))
        }
    };

    let client = reqwest::blocking::Client::new();
    let result = client
        .patch(&**url)
        .body((*data).clone())
        .send()
        .and_then(|resp| resp.error_for_status())
        .and_then(|resp| resp.text());

    match result {
        Ok(body) => Ok(Value::String(Arc::new(body))),
        Err(e) => Err(FlowError::runtime(&format!("HTTP PATCH failed: {}", e), 0, 0)),
    }
}

// net::delete(url: Silk) -> Silk
fn net_delete(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "net::delete expects 1 argument (url)",
            0,
            0,
        ));
    }

    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "net::delete expects a string URL",
                0,
                0,
            ))
        }
    };

    let client = reqwest::blocking::Client::new();
    let result = client
        .delete(&**url)
        .send()
        .and_then(|resp| resp.error_for_status())
        .and_then(|resp| resp.text());

    match result {
        Ok(body) => Ok(Value::String(Arc::new(body))),
        Err(e) => Err(FlowError::runtime(&format!("HTTP DELETE failed: {}", e), 0, 0)),
    }
}

// net::head(url: Silk) -> Silk
fn net_head(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "net::head expects 1 argument (url)",
            0,
            0,
        ));
    }

    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "net::head expects a string URL",
                0,
                0,
            ))
        }
    };

    let client = reqwest::blocking::Client::new();
    let result = client
        .head(&**url)
        .send()
        .and_then(|resp| resp.error_for_status());

    match result {
        Ok(resp) => {
            // HEAD returns no body, so return status code
            Ok(Value::String(Arc::new(format!("Status: {}", resp.status()))))
        }
        Err(e) => Err(FlowError::runtime(&format!("HTTP HEAD failed: {}", e), 0, 0)),
    }
}

// net::request(method: Silk, url: Silk, data: Silk) -> Silk
// Generic request method that accepts any HTTP method
fn net_request(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(FlowError::runtime(
            "net::request expects 2-3 arguments (method, url, [data])",
            0,
            0,
        ));
    }

    let method = match &args[0] {
        Value::String(s) => s.to_uppercase(),
        _ => {
            return Err(FlowError::type_error(
                "net::request expects a string method",
                0,
                0,
            ))
        }
    };

    let url = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "net::request expects a string URL",
                0,
                0,
            ))
        }
    };

    let data = if args.len() == 3 {
        match &args[2] {
            Value::String(s) => Some(s.clone()),
            _ => {
                return Err(FlowError::type_error(
                    "net::request expects a string data",
                    0,
                    0,
                ))
            }
        }
    } else {
        None
    };

    let client = reqwest::blocking::Client::new();
    let mut request = match method.as_str() {
        "GET" => client.get(&**url),
        "POST" => client.post(&**url),
        "PUT" => client.put(&**url),
        "PATCH" => client.patch(&**url),
        "DELETE" => client.delete(&**url),
        "HEAD" => client.head(&**url),
        _ => {
            return Err(FlowError::runtime(
                &format!("Unsupported HTTP method: {}", method),
                0,
                0,
            ))
        }
    };

    if let Some(body) = data {
        request = request.body((*body).clone());
    }

    let result = request
        .send()
        .and_then(|resp| resp.error_for_status())
        .and_then(|resp| resp.text());

    match result {
        Ok(body) => Ok(Value::String(Arc::new(body))),
        Err(e) => Err(FlowError::runtime(&format!("HTTP {} failed: {}", method, e), 0, 0)),
    }
}
