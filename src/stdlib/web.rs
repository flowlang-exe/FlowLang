//! std:web - HTTP Server Module
//!
//! Provides HTTP server functionality using warp.

use crate::error::FlowError;
use crate::types::{Value, AsyncNativeFn, AsyncContext, NativeFn};
use crate::runtime::handle::HandleType;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::oneshot;
use warp::Reply;

/// Load the web module
pub fn load_web_module() -> Vec<(&'static str, Value)> {
    vec![
        ("serve", Value::AsyncNativeFunction(AsyncNativeFn(Arc::new(|args, ctx| {
            Box::pin(web_serve(args, ctx))
        })))),
        // Response helpers
        ("json", Value::NativeFunction(NativeFn(Arc::new(res_json)))),
        ("html", Value::NativeFunction(NativeFn(Arc::new(res_html)))),
        ("text", Value::NativeFunction(NativeFn(Arc::new(res_text)))),
        ("status", Value::NativeFunction(NativeFn(Arc::new(res_status)))),
        ("redirect", Value::NativeFunction(NativeFn(Arc::new(res_redirect)))),
        ("notFound", Value::NativeFunction(NativeFn(Arc::new(res_not_found)))),
        ("badRequest", Value::NativeFunction(NativeFn(Arc::new(res_bad_request)))),
        ("serverError", Value::NativeFunction(NativeFn(Arc::new(res_server_error)))),
    ]
}

/// web.serve(port, handler) -> Handle
/// Creates an HTTP server on the specified port.
/// The handler is called for each request and should return a response object.
async fn web_serve(args: Vec<Value>, ctx: AsyncContext) -> Result<Value, FlowError> {
    if args.len() != 2 {
        return Err(FlowError::runtime(
            "web.serve expects 2 arguments (port, handler)",
            0, 0,
        ));
    }

    let port = match &args[0] {
        Value::Number(n) => *n as u16,
        _ => return Err(FlowError::type_error(
            "web.serve expects a Ember for port",
            0, 0,
        )),
    };

    let handler = match &args[1] {
        Value::Function { .. } => args[1].clone(),
        _ => return Err(FlowError::type_error(
            "web.serve expects a Spell (function) as handler",
            0, 0,
        )),
    };

    // Create shutdown channel
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    // Register the handle
    let handle_id = ctx.runtime.register_handle(HandleType::HttpServer {
        port,
        shutdown_tx: Some(shutdown_tx),
    }).await;

    // Get web callback sender for request handling (with response support)
    let callback_tx = ctx.runtime.web_callback_sender();
    let runtime = ctx.runtime.clone();

    // Spawn the server task
    tokio::spawn(async move {
        use warp::Filter;
        
        // Create a channel for request/response communication
        // Tuple: (method, path, body, headers, client_addr, resp_tx)
        // Response: (status, body, content_type, custom_headers)
        let (req_tx, mut req_rx) = tokio::sync::mpsc::unbounded_channel::<(
            warp::http::Method,
            String,
            String,
            HashMap<String, String>,  // Request Headers
            Option<std::net::SocketAddr>,  // Client IP
            tokio::sync::oneshot::Sender<(u16, String, String, HashMap<String, String>)>,
        )>();

        let req_tx = Arc::new(req_tx);
        let handler_clone = handler.clone();
        let callback_tx_clone = callback_tx.clone();

        // Warp route that handles all requests
        let req_tx_filter = req_tx.clone();
        let routes = warp::any()
            .and(warp::method())
            .and(warp::path::full())
            .and(warp::query::raw().or_else(|_| async { Ok::<_, warp::Rejection>((String::new(),)) }))
            .and(warp::header::headers_cloned())
            .and(warp::addr::remote())
            .and(warp::body::bytes())
            .and_then(move |method: warp::http::Method, 
                           path: warp::path::FullPath, 
                           query: String,
                           headers: warp::http::HeaderMap,
                           addr: Option<std::net::SocketAddr>,
                           body: bytes::Bytes| {
                let req_tx = req_tx_filter.clone();
                async move {
                    let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
                    let body_str = String::from_utf8_lossy(&body).to_string();
                    
                    // Combine path with query string
                    let full_path = if query.is_empty() {
                        path.as_str().to_string()
                    } else {
                        format!("{}?{}", path.as_str(), query)
                    };
                    
                    // Convert headers to HashMap<String, String>
                    let headers_map: HashMap<String, String> = headers
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                        .collect();
                    
                    if req_tx.send((method, full_path, body_str, headers_map, addr, resp_tx)).is_err() {
                        return Ok::<_, warp::Rejection>(
                            warp::reply::with_status(
                                "Server Error",
                                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                            ).into_response()
                        );
                    }
                    
                    match resp_rx.await {
                        Ok((status, body, content_type, custom_headers)) => {
                            let status = warp::http::StatusCode::from_u16(status)
                                .unwrap_or(warp::http::StatusCode::OK);
                            let mut reply = warp::reply::with_status(body, status).into_response();
                            
                            // Set Content-Type
                            reply.headers_mut().insert(
                                "Content-Type",
                                content_type.parse().unwrap_or_else(|_| "text/plain".parse().unwrap())
                            );
                            
                            // Add custom headers
                            for (name, value) in custom_headers {
                                if let (Ok(header_name), Ok(header_value)) = (
                                    warp::http::header::HeaderName::try_from(name.as_str()),
                                    warp::http::header::HeaderValue::try_from(value.as_str())
                                ) {
                                    reply.headers_mut().insert(header_name, header_value);
                                }
                            }
                            
                            Ok(reply)
                        }
                        Err(_) => Ok(
                            warp::reply::with_status(
                                "Handler Error",
                                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                            ).into_response()
                        ),
                    }
                }
            });

        // Spawn request handler task
        let handler_task = tokio::spawn(async move {
            while let Some((method, path, body, headers_map, addr, resp_tx)) = req_rx.recv().await {
                // Parse path and query
                let (pathname, query_string) = if let Some(idx) = path.find('?') {
                    (&path[..idx], Some(&path[idx + 1..]))
                } else {
                    (path.as_str(), None)
                };
                
                // Parse query string into Relic
                let query_map = if let Some(qs) = query_string {
                    let mut map = HashMap::new();
                    for pair in qs.split('&') {
                        if pair.is_empty() {
                            continue;
                        }
                        let parts: Vec<&str> = pair.splitn(2, '=').collect();
                        let key = parts[0].to_string();
                        let value = if parts.len() > 1 {
                            parts[1].to_string()
                        } else {
                            String::new()
                        };
                        map.insert(key, Value::String(Arc::new(value)));
                    }
                    Value::Relic(Arc::new(map))
                } else {
                    Value::Relic(Arc::new(HashMap::new()))
                };
                
                // Convert headers to Value::Relic
                let headers_relic: HashMap<String, Value> = headers_map.iter()
                    .map(|(k, v)| (k.to_lowercase(), Value::String(Arc::new(v.clone()))))
                    .collect();
                
                // Parse cookies from Cookie header
                let cookies_map = if let Some(cookie_header) = headers_map.get("cookie") {
                    let mut map = HashMap::new();
                    for pair in cookie_header.split(';') {
                        let pair = pair.trim();
                        if let Some(idx) = pair.find('=') {
                            let key = pair[..idx].to_string();
                            let value = pair[idx + 1..].to_string();
                            map.insert(key, Value::String(Arc::new(value)));
                        }
                    }
                    Value::Relic(Arc::new(map))
                } else {
                    Value::Relic(Arc::new(HashMap::new()))
                };
                
                // Get host from headers or default
                let host = headers_map.get("host")
                    .cloned()
                    .unwrap_or_else(|| "localhost".to_string());
                
                // Build full URL
                let protocol = "http"; // Could detect from X-Forwarded-Proto
                let url = format!("{}://{}{}", protocol, host, path);
                
                // Get client IP
                let ip = addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string());
                
                // Create enhanced request object
                let mut req_map = HashMap::new();
                req_map.insert("method".to_string(), Value::String(Arc::new(method.to_string())));
                req_map.insert("path".to_string(), Value::String(Arc::new(path.clone())));
                req_map.insert("pathname".to_string(), Value::String(Arc::new(pathname.to_string())));
                req_map.insert("url".to_string(), Value::String(Arc::new(url)));
                req_map.insert("query".to_string(), query_map);
                req_map.insert("headers".to_string(), Value::Relic(Arc::new(headers_relic)));
                req_map.insert("cookies".to_string(), cookies_map);
                req_map.insert("body".to_string(), Value::String(Arc::new(body)));
                req_map.insert("ip".to_string(), Value::String(Arc::new(ip)));
                req_map.insert("host".to_string(), Value::String(Arc::new(host)));
                req_map.insert("protocol".to_string(), Value::String(Arc::new(protocol.to_string())));
                
                let request_value = Value::Relic(Arc::new(req_map));
                
                // Create response object with callable methods
                let mut res_map = HashMap::new();
                res_map.insert("json".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_json))));
                res_map.insert("html".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_html))));
                res_map.insert("text".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_text))));
                res_map.insert("status".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_status))));
                res_map.insert("redirect".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_redirect))));
                res_map.insert("notFound".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_not_found))));
                res_map.insert("badRequest".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_bad_request))));
                res_map.insert("serverError".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_server_error))));
                // Status helpers
                res_map.insert("ok".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_ok))));
                res_map.insert("created".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_created))));
                res_map.insert("noContent".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_no_content))));
                res_map.insert("unauthorized".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_unauthorized))));
                res_map.insert("forbidden".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_forbidden))));
                res_map.insert("send".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_send))));
                // File and header helpers
                res_map.insert("file".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_file))));
                res_map.insert("header".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_header))));
                
                let response_value = Value::Relic(Arc::new(res_map));
                
                // Create response channel to wait for handler result
                let (response_tx, response_rx) = tokio::sync::oneshot::channel();
                
                // Send web callback to main event loop (with response channel)
                // Pass both req and res to handler
                let callback_request = crate::runtime::WebCallbackRequest {
                    callback: handler_clone.clone(),
                    args: vec![request_value, response_value],
                    response_tx,
                };
                
                if callback_tx_clone.send(callback_request).is_err() {
                    let _ = resp_tx.send((500, "Server Error".to_string(), "text/plain".to_string(), HashMap::new()));
                    continue;
                }
                
                // Wait for handler to return response
                match response_rx.await {
                    Ok(result) => {
                        // Extract status, body, content-type, and headers from result
                        let (status, body, content_type, headers) = extract_response(result);
                        let _ = resp_tx.send((status, body, content_type, headers));
                    }
                    Err(_) => {
                        let _ = resp_tx.send((200, "OK".to_string(), "text/plain".to_string(), HashMap::new()));
                    }
                }
            }
        });

        // Run server with graceful shutdown
        let addr = ([0, 0, 0, 0], port);
        let (_, server) = warp::serve(routes)
            .bind_with_graceful_shutdown(addr, async {
                let _ = shutdown_rx.await;
            });

        // println!("🌐 HTTP Server listening on http://0.0.0.0:{}", port);

        server.await;
        handler_task.abort();
        
        // Unregister handle when server stops
        runtime.unregister_handle(handle_id).await;
    });

    Ok(Value::Handle(handle_id))
}

/// Extract status code, body, content-type, and headers from a handler response value
fn extract_response(value: Value) -> (u16, String, String, HashMap<String, String>) {
    match value {
        // Relic with status, body, contentType, and headers fields
        Value::Relic(map) => {
            let status = match map.get("status") {
                Some(Value::Number(n)) => *n as u16,
                _ => 200,
            };
            let body = match map.get("body") {
                Some(v) => v.to_string(),
                None => String::new(),
            };
            // Auto-detect content type from body or use explicit contentType
            let content_type = match map.get("contentType") {
                Some(Value::String(ct)) => (**ct).clone(),
                _ => {
                    // Auto-detect: if body starts with { or [, assume JSON
                    let trimmed = body.trim();
                    if trimmed.starts_with('{') || trimmed.starts_with('[') {
                        String::from("application/json")
                    } else if trimmed.starts_with('<') {
                        String::from("text/html")
                    } else {
                        String::from("text/plain")
                    }
                }
            };
            // Extract custom headers
            let headers: HashMap<String, String> = match map.get("headers") {
                Some(Value::Relic(h)) => {
                    h.iter()
                        .map(|(k, v)| (k.clone(), v.to_string()))
                        .collect()
                }
                _ => HashMap::new(),
            };
            (status, body, content_type, headers)
        }
        // String response (default 200, text/plain)
        Value::String(s) => (200, (*s).clone(), "text/plain".to_string(), HashMap::new()),
        // Number as status code
        Value::Number(n) => (n as u16, String::new(), "text/plain".to_string(), HashMap::new()),
        // Null/Void
        Value::Null => (204, String::new(), "text/plain".to_string(), HashMap::new()),
        // Anything else - convert to string
        _ => (200, value.to_string(), "text/plain".to_string(), HashMap::new()),
    }
}

/// web.json(data) -> Relic
/// Create a JSON response with auto Content-Type
fn res_json(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("web.json expects 1 argument (data)", 0, 0));
    }

    let body = match &args[0] {
        Value::Relic(_) | Value::Array(_) => {
            // Serialize to JSON string
            crate::stdlib::json::value_to_json_string(&args[0])
        }
        Value::String(s) => (**s).clone(),
        _ => args[0].to_string(),
    };

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(200.0));
    map.insert("body".to_string(), Value::String(Arc::new(body)));
    map.insert("contentType".to_string(), Value::String(Arc::new("application/json".to_string())));

    Ok(Value::Relic(Arc::new(map)))
}

/// web.html(content) -> Relic
/// Create an HTML response
fn res_html(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("web.html expects 1 argument (content)", 0, 0));
    }

    let body = args[0].to_string();
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(200.0));
    map.insert("body".to_string(), Value::String(Arc::new(body)));
    map.insert("contentType".to_string(), Value::String(Arc::new("text/html".to_string())));

    Ok(Value::Relic(Arc::new(map)))
}

/// web.text(content) -> Relic
/// Create a plain text response
fn res_text(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("web.text expects 1 argument (content)", 0, 0));
    }

    let body = args[0].to_string();
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(200.0));
    map.insert("body".to_string(), Value::String(Arc::new(body)));
    map.insert("contentType".to_string(), Value::String(Arc::new("text/plain".to_string())));

    Ok(Value::Relic(Arc::new(map)))
}

/// web.status(code, body?) -> Relic
/// Create a response with custom status code
fn res_status(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("web.status expects at least 1 argument (code)", 0, 0));
    }

    let status = match &args[0] {
        Value::Number(n) => *n,
        _ => 200.0,
    };

    let body = if args.len() > 1 {
        args[1].to_string()
    } else {
        String::new()
    };
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(status));
    map.insert("body".to_string(), Value::String(Arc::new(body)));

    Ok(Value::Relic(Arc::new(map)))
}

/// web.redirect(url) -> Relic
/// Create a redirect response (302)
fn res_redirect(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("web.redirect expects 1 argument (url)", 0, 0));
    }

    let url = args[0].to_string();
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(302.0));
    map.insert("body".to_string(), Value::String(Arc::new(String::new())));
    map.insert("headers".to_string(), {
        let mut headers = HashMap::new();
        headers.insert("Location".to_string(), Value::String(Arc::new(url)));
        Value::Relic(Arc::new(headers))
    });

    Ok(Value::Relic(Arc::new(map)))
}

/// web.notFound(message?) -> Relic
/// Create a 404 Not Found response
fn res_not_found(args: Vec<Value>) -> Result<Value, FlowError> {
    let body = if args.is_empty() {
        "Not Found".to_string()
    } else {
        args[0].to_string()
    };
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(404.0));
    map.insert("body".to_string(), Value::String(Arc::new(body)));

    Ok(Value::Relic(Arc::new(map)))
}

/// web.badRequest(message?) -> Relic
/// Create a 400 Bad Request response
fn res_bad_request(args: Vec<Value>) -> Result<Value, FlowError> {
    let body = if args.is_empty() {
        "Bad Request".to_string()
    } else {
        args[0].to_string()
    };
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(400.0));
    map.insert("body".to_string(), Value::String(Arc::new(body)));

    Ok(Value::Relic(Arc::new(map)))
}

/// web.serverError(message?) -> Relic
/// Create a 500 Internal Server Error response
fn res_server_error(args: Vec<Value>) -> Result<Value, FlowError> {
    let body = if args.is_empty() {
        "Internal Server Error".to_string()
    } else {
        args[0].to_string()
    };
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(500.0));
    map.insert("body".to_string(), Value::String(Arc::new(body)));

    Ok(Value::Relic(Arc::new(map)))
}

/// res.ok(message?) -> Relic
/// Create a 200 OK response
fn res_ok(args: Vec<Value>) -> Result<Value, FlowError> {
    let body = if args.is_empty() {
        "OK".to_string()
    } else {
        args[0].to_string()
    };
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(200.0));
    map.insert("body".to_string(), Value::String(Arc::new(body)));

    Ok(Value::Relic(Arc::new(map)))
}

/// res.created(data?) -> Relic
/// Create a 201 Created response
fn res_created(args: Vec<Value>) -> Result<Value, FlowError> {
    let body = if args.is_empty() {
        "Created".to_string()
    } else {
        match &args[0] {
            Value::Relic(_) | Value::Array(_) => {
                crate::stdlib::json::value_to_json_string(&args[0])
            }
            _ => args[0].to_string(),
        }
    };
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(201.0));
    map.insert("body".to_string(), Value::String(Arc::new(body)));

    Ok(Value::Relic(Arc::new(map)))
}

/// res.noContent() -> Relic
/// Create a 204 No Content response
fn res_no_content(_args: Vec<Value>) -> Result<Value, FlowError> {
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(204.0));
    map.insert("body".to_string(), Value::String(Arc::new(String::new())));

    Ok(Value::Relic(Arc::new(map)))
}

/// res.unauthorized(message?) -> Relic
/// Create a 401 Unauthorized response
fn res_unauthorized(args: Vec<Value>) -> Result<Value, FlowError> {
    let body = if args.is_empty() {
        "Unauthorized".to_string()
    } else {
        args[0].to_string()
    };
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(401.0));
    map.insert("body".to_string(), Value::String(Arc::new(body)));

    Ok(Value::Relic(Arc::new(map)))
}

/// res.forbidden(message?) -> Relic
/// Create a 403 Forbidden response
fn res_forbidden(args: Vec<Value>) -> Result<Value, FlowError> {
    let body = if args.is_empty() {
        "Forbidden".to_string()
    } else {
        args[0].to_string()
    };
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(403.0));
    map.insert("body".to_string(), Value::String(Arc::new(body)));

    Ok(Value::Relic(Arc::new(map)))
}

/// res.send(data) -> Relic
/// Auto-detect data type and send appropriate response
fn res_send(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("res.send expects 1 argument (data)", 0, 0));
    }

    let (body, content_type) = match &args[0] {
        Value::Relic(_) | Value::Array(_) => {
            (crate::stdlib::json::value_to_json_string(&args[0]), "application/json")
        }
        Value::String(s) => {
            let s = (**s).clone();
            // Check if it looks like HTML
            if s.trim().starts_with('<') {
                (s, "text/html")
            } else {
                (s, "text/plain")
            }
        }
        _ => (args[0].to_string(), "text/plain"),
    };
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(200.0));
    map.insert("body".to_string(), Value::String(Arc::new(body)));
    map.insert("contentType".to_string(), Value::String(Arc::new(content_type.to_string())));

    Ok(Value::Relic(Arc::new(map)))
}

/// res.file(path) -> Relic
/// Serve a file from disk
fn res_file(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("res.file expects 1 argument (path)", 0, 0));
    }

    let path = args[0].to_string();
    
    // Read file content
    let content = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(e) => {
            return Ok({
                let mut map = HashMap::new();
                map.insert("status".to_string(), Value::Number(404.0));
                map.insert("body".to_string(), Value::String(Arc::new(format!("File not found: {}", e))));
                Value::Relic(Arc::new(map))
            });
        }
    };
    
    // Detect MIME type from extension
    let content_type = match std::path::Path::new(&path).extension().and_then(|e| e.to_str()) {
        Some("html") | Some("htm") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("pdf") => "application/pdf",
        Some("txt") => "text/plain",
        Some("xml") => "application/xml",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        _ => "application/octet-stream",
    };
    
    // Convert to string (for text files) or base64 (for binary)
    let body = if content_type.starts_with("text/") || content_type == "application/json" || content_type == "application/javascript" || content_type == "application/xml" {
        String::from_utf8_lossy(&content).to_string()
    } else {
        // For binary files, we'd need to handle differently
        // For now, try as text
        String::from_utf8_lossy(&content).to_string()
    };
    
    // Extract filename from path
    let filename = std::path::Path::new(&path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file");
    
    // Build headers with Content-Disposition
    let mut headers = HashMap::new();
    headers.insert("Content-Disposition".to_string(), 
        Value::String(Arc::new(format!("inline; filename=\"{}\"", filename))));
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(200.0));
    map.insert("body".to_string(), Value::String(Arc::new(body)));
    map.insert("contentType".to_string(), Value::String(Arc::new(content_type.to_string())));
    map.insert("headers".to_string(), Value::Relic(Arc::new(headers)));

    Ok(Value::Relic(Arc::new(map)))
}

/// res.header(name, value) -> Relic
/// Create a response with custom header
fn res_header(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() < 2 {
        return Err(FlowError::runtime("res.header expects 2 arguments (name, value)", 0, 0));
    }

    let name = args[0].to_string();
    let value = args[1].to_string();
    
    // Return a Relic with headers field
    let mut headers = HashMap::new();
    headers.insert(name, Value::String(Arc::new(value)));
    
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::Number(200.0));
    map.insert("body".to_string(), Value::String(Arc::new(String::new())));
    map.insert("headers".to_string(), Value::Relic(Arc::new(headers)));

    Ok(Value::Relic(Arc::new(map)))
}
