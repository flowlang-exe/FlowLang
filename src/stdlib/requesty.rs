use crate::error::FlowError;
use crate::types::{NativeFn, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

pub fn load_requesty_module() -> Vec<(&'static str, Value)> {
    vec![
        ("get", Value::NativeFunction(NativeFn::new(req_get))),
        ("post", Value::NativeFunction(NativeFn::new(req_post))),
        ("put", Value::NativeFunction(NativeFn::new(req_put))),
        ("delete", Value::NativeFunction(NativeFn::new(req_delete))),
        ("patch", Value::NativeFunction(NativeFn::new(req_patch))),
        ("head", Value::NativeFunction(NativeFn::new(req_head))),
        ("options", Value::NativeFunction(NativeFn::new(req_options))),
        ("request", Value::NativeFunction(NativeFn::new(req_wrapper))),
    ]
}

/// Helper to parse options object
struct RequestOptions {
    method: String,
    headers: HashMap<String, String>,
    body: Option<String>,
    timeout: Option<Duration>,
}

fn parse_options(args: &[Value], default_method: &str) -> Result<(String, RequestOptions), FlowError> {
    // Arg 0 is always URL
    let url = match args.get(0) {
        Some(Value::String(s)) => s.to_string(),
        _ => return Err(FlowError::runtime("Expected URL as first argument", 0, 0)),
    };

    let mut options = RequestOptions {
        method: default_method.to_string(),
        headers: HashMap::new(),
        body: None,
        timeout: None,
    };

    // Arg 1 is optional options object or body
    if let Some(arg) = args.get(1) {
        if let Value::Relic(map) = arg {
            if let Some(Value::String(m)) = map.get("method") {
                options.method = m.to_uppercase();
            }
            if let Some(Value::Relic(h)) = map.get("headers") {
                for (k, v) in h.iter() {
                    options.headers.insert(k.clone(), v.to_string());
                }
            }
            if let Some(Value::Number(ms)) = map.get("timeout") {
                 options.timeout = Some(Duration::from_millis(*ms as u64));
            }
            if let Some(Value::String(b)) = map.get("body") {
                options.body = Some(b.to_string());
            } else if let Some(json_val) = map.get("json") {
                // Manually serialize JSON
                let s = value_to_json_string(json_val)?;
                options.body = Some(s);
                options.headers.insert("Content-Type".to_string(), "application/json".to_string());
            }
        } 
        else if let Value::String(s) = arg {
             if matches!(default_method, "POST" | "PUT" | "PATCH") {
                 options.body = Some(s.to_string());
             }
        }
    }
    
    // Check for 3rd arg
    if let Some(Value::Relic(map)) = args.get(2) {
        if let Some(Value::Relic(h)) = map.get("headers") {
            for (k, v) in h.iter() {
                options.headers.insert(k.clone(), v.to_string());
            }
        }
        if let Some(Value::Number(ms)) = map.get("timeout") {
                options.timeout = Some(Duration::from_millis(*ms as u64));
        }
    }

    Ok((url, options))
}

fn execute_request(url: String, opts: RequestOptions) -> Result<Value, FlowError> {
    let client_builder = reqwest::blocking::Client::builder();
    
    let client = if let Some(t) = opts.timeout {
        client_builder.timeout(t)
    } else {
        client_builder
    }.build().map_err(|e| FlowError::runtime(&format!("Failed to build client: {}", e), 0, 0))?;

    let mut req_builder = match opts.method.as_str() {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        "HEAD" => client.head(&url),
        "PATCH" => client.patch(&url),
        "OPTIONS" => client.request(reqwest::Method::OPTIONS, &url),
        m => return Err(FlowError::runtime(&format!("Unsupported method: {}", m), 0, 0)),
    };

    for (k, v) in opts.headers {
        req_builder = req_builder.header(k, v);
    }

    if let Some(body) = opts.body {
        req_builder = req_builder.body(body);
    }

    let result = req_builder.send();

    match result {
        Ok(resp) => {
            let status = resp.status().as_u16() as f64;
            let status_text = resp.status().canonical_reason().unwrap_or("").to_string();
            let headers_map: HashMap<String, Value> = resp.headers()
                .iter()
                .map(|(k, v)| (k.to_string(), Value::String(Arc::new(v.to_str().unwrap_or("").to_string()))))
                .collect();
            
            let text = resp.text().unwrap_or_default();
            
            let mut response_map = HashMap::new();
            response_map.insert("status".to_string(), Value::Number(status));
            response_map.insert("statusText".to_string(), Value::String(Arc::new(status_text)));
            response_map.insert("headers".to_string(), Value::Relic(Arc::new(headers_map)));
            response_map.insert("text".to_string(), Value::String(Arc::new(text.clone())));
            
            if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&text) {
                 response_map.insert("json".to_string(), json_to_value(json_val));
            } else {
                 response_map.insert("json".to_string(), Value::Null);
            }

            Ok(Value::Relic(Arc::new(response_map)))
        }
        Err(e) => Err(FlowError::runtime(&format!("Request failed: {}", e), 0, 0)),
    }
}

fn json_to_value(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Boolean(b),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                Value::Number(f)
            } else {
                Value::Number(0.0)
            }
        },
        serde_json::Value::String(s) => Value::String(Arc::new(s)),
        serde_json::Value::Array(a) => {
            Value::Array(Arc::new(a.into_iter().map(json_to_value).collect()))
        },
        serde_json::Value::Object(o) => {
            let map: HashMap<String, Value> = o.into_iter().map(|(k, v)| (k, json_to_value(v))).collect();
            Value::Relic(Arc::new(map))
        }
    }
}

// Convert Value to JSON string manually
fn value_to_json_string(v: &Value) -> Result<String, FlowError> {
    match v {
        Value::Null => Ok("null".to_string()),
        Value::Boolean(b) => Ok(b.to_string()),
        Value::Number(n) => Ok(n.to_string()),
        Value::String(s) => Ok(serde_json::to_string(&**s).unwrap()), // Use serde for string escaping
        Value::Array(arr) => {
            let elems: Result<Vec<String>, _> = arr.iter().map(|e| value_to_json_string(e)).collect();
            Ok(format!("[{}]", elems?.join(",")))
        },
        Value::Relic(map) => {
            let mut entries = Vec::new();
            for (k, v) in map.iter() {
                let key = serde_json::to_string(k).unwrap();
                let val = value_to_json_string(v)?;
                entries.push(format!("{}:{}", key, val));
            }
            Ok(format!("{{{}}}", entries.join(",")))
        },
        _ => Err(FlowError::runtime("Cannot serialize this type to JSON", 0, 0)),
    }
}

fn req_get(args: Vec<Value>) -> Result<Value, FlowError> {
    let (url, opts) = parse_options(&args, "GET")?;
    execute_request(url, opts)
}

fn req_post(args: Vec<Value>) -> Result<Value, FlowError> {
    let (url, opts) = parse_options(&args, "POST")?;
    execute_request(url, opts)
}

fn req_put(args: Vec<Value>) -> Result<Value, FlowError> {
    let (url, opts) = parse_options(&args, "PUT")?;
    execute_request(url, opts)
}

fn req_delete(args: Vec<Value>) -> Result<Value, FlowError> {
    let (url, opts) = parse_options(&args, "DELETE")?;
    execute_request(url, opts)
}

fn req_patch(args: Vec<Value>) -> Result<Value, FlowError> {
    let (url, opts) = parse_options(&args, "PATCH")?;
    execute_request(url, opts)
}

fn req_head(args: Vec<Value>) -> Result<Value, FlowError> {
    let (url, opts) = parse_options(&args, "HEAD")?;
    execute_request(url, opts)
}

fn req_options(args: Vec<Value>) -> Result<Value, FlowError> {
    let (url, opts) = parse_options(&args, "OPTIONS")?;
    execute_request(url, opts)
}

fn req_wrapper(args: Vec<Value>) -> Result<Value, FlowError> {
    let (url, opts) = parse_options(&args, "GET")?;
    execute_request(url, opts)
}
