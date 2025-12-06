//! std:url - URL Parsing Module
//!
//! Provides URL parsing functionality similar to Node.js URL module.

use crate::error::FlowError;
use crate::types::{Value, NativeFn};
use std::collections::HashMap;
use std::sync::Arc;

/// Load the url module
pub fn load_url_module() -> Vec<(&'static str, Value)> {
    vec![
        ("parse", Value::NativeFunction(NativeFn(Arc::new(url_parse)))),
        ("parseQuery", Value::NativeFunction(NativeFn(Arc::new(url_parse_query)))),
        ("format", Value::NativeFunction(NativeFn(Arc::new(url_format)))),
        ("encode", Value::NativeFunction(NativeFn(Arc::new(url_encode)))),
        ("decode", Value::NativeFunction(NativeFn(Arc::new(url_decode)))),
    ]
}

/// url.parse(urlString) -> Relic
/// Parse a URL string into its components
fn url_parse(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("url.parse expects 1 argument (url)", 0, 0));
    }

    let url_str = args[0].to_string();
    
    // Parse the URL
    let mut result = HashMap::new();
    
    // Check for protocol
    let (protocol, rest) = if let Some(idx) = url_str.find("://") {
        (Some(&url_str[..idx]), &url_str[idx + 3..])
    } else {
        (None, url_str.as_str())
    };
    
    if let Some(proto) = protocol {
        result.insert("protocol".to_string(), Value::String(Arc::new(proto.to_string())));
    }
    
    // Split path and query
    let (path_with_host, query_string) = if let Some(idx) = rest.find('?') {
        (&rest[..idx], Some(&rest[idx + 1..]))
    } else {
        (rest, None)
    };
    
    // Split host and path
    let (host, path) = if let Some(idx) = path_with_host.find('/') {
        (&path_with_host[..idx], &path_with_host[idx..])
    } else {
        (path_with_host, "/")
    };
    
    // Parse host and port
    let (hostname, port) = if let Some(idx) = host.find(':') {
        (&host[..idx], Some(&host[idx + 1..]))
    } else {
        (host, None)
    };
    
    if !hostname.is_empty() {
        result.insert("hostname".to_string(), Value::String(Arc::new(hostname.to_string())));
        result.insert("host".to_string(), Value::String(Arc::new(host.to_string())));
    }
    
    if let Some(p) = port {
        if let Ok(port_num) = p.parse::<f64>() {
            result.insert("port".to_string(), Value::Number(port_num));
        }
    }
    
    result.insert("pathname".to_string(), Value::String(Arc::new(path.to_string())));
    
    if let Some(qs) = query_string {
        result.insert("search".to_string(), Value::String(Arc::new(format!("?{}", qs))));
        result.insert("query".to_string(), parse_query_to_relic(qs));
    } else {
        result.insert("query".to_string(), Value::Relic(Arc::new(HashMap::new())));
    }
    
    // Full href
    result.insert("href".to_string(), Value::String(Arc::new(url_str)));
    
    Ok(Value::Relic(Arc::new(result)))
}

/// url.parseQuery(queryString) -> Relic
/// Parse a query string into a Relic (object)
fn url_parse_query(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("url.parseQuery expects 1 argument (query)", 0, 0));
    }

    let query_str = args[0].to_string();
    // Remove leading ? if present
    let query = query_str.trim_start_matches('?');
    
    Ok(parse_query_to_relic(query))
}

/// Helper to parse query string into Value::Relic
fn parse_query_to_relic(query: &str) -> Value {
    let mut map = HashMap::new();
    
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = pair.splitn(2, '=').collect();
        let key = url_decode_string(parts[0]);
        let value = if parts.len() > 1 {
            url_decode_string(parts[1])
        } else {
            String::new()
        };
        
        map.insert(key, Value::String(Arc::new(value)));
    }
    
    Value::Relic(Arc::new(map))
}

/// url.format(urlObject) -> Silk
/// Format a URL object back into a string
fn url_format(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("url.format expects 1 argument (urlObject)", 0, 0));
    }

    let url_obj = match &args[0] {
        Value::Relic(map) => map,
        _ => return Err(FlowError::type_error("url.format expects a Relic", 0, 0)),
    };

    let mut result = String::new();
    
    // Protocol
    if let Some(Value::String(proto)) = url_obj.get("protocol") {
        result.push_str(proto);
        result.push_str("://");
    }
    
    // Host
    if let Some(Value::String(host)) = url_obj.get("host") {
        result.push_str(host);
    } else if let Some(Value::String(hostname)) = url_obj.get("hostname") {
        result.push_str(hostname);
        if let Some(Value::Number(port)) = url_obj.get("port") {
            result.push(':');
            result.push_str(&(*port as u16).to_string());
        }
    }
    
    // Path
    if let Some(Value::String(path)) = url_obj.get("pathname") {
        result.push_str(path);
    }
    
    // Query
    if let Some(Value::String(search)) = url_obj.get("search") {
        result.push_str(search);
    }
    
    Ok(Value::String(Arc::new(result)))
}

/// url.encode(text) -> Silk
/// URL encode a string
fn url_encode(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("url.encode expects 1 argument", 0, 0));
    }

    let text = args[0].to_string();
    let encoded = url_encode_string(&text);
    
    Ok(Value::String(Arc::new(encoded)))
}

/// url.decode(text) -> Silk
/// URL decode a string
fn url_decode(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("url.decode expects 1 argument", 0, 0));
    }

    let text = args[0].to_string();
    let decoded = url_decode_string(&text);
    
    Ok(Value::String(Arc::new(decoded)))
}

/// URL encode helper
fn url_encode_string(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                result.push(c);
            }
            ' ' => result.push_str("%20"),
            _ => {
                for byte in c.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    result
}

/// URL decode helper
fn url_decode_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if hex.len() == 2 {
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    result.push(byte as char);
                } else {
                    result.push('%');
                    result.push_str(&hex);
                }
            } else {
                result.push('%');
                result.push_str(&hex);
            }
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }
    
    result
}
