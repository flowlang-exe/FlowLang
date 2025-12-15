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

    // Create Response Prototype (Singleton)
    // Contains efficient static references to helper functions to avoid
    // rebuilding this HashMap for every single request (allocating ~16 strings/Arcs per req).
    let response_prototype = {
        let mut map = HashMap::new();
        map.insert("json".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_json))));
        map.insert("html".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_html))));
        map.insert("text".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_text))));
        map.insert("status".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_status))));
        map.insert("redirect".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_redirect))));
        map.insert("notFound".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_not_found))));
        map.insert("badRequest".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_bad_request))));
        map.insert("serverError".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_server_error))));
        map.insert("ok".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_ok))));
        map.insert("created".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_created))));
        map.insert("noContent".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_no_content))));
        map.insert("unauthorized".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_unauthorized))));
        map.insert("forbidden".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_forbidden))));
        map.insert("send".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_send))));
        map.insert("file".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_file))));
        map.insert("header".to_string(), Value::NativeFunction(NativeFn(Arc::new(res_header))));
        Value::Relic(Arc::new(map))
    };

    // Spawn the server task
    tokio::spawn(async move {
        use warp::Filter;
        
        let handler_clone = handler.clone();
        let callback_tx_clone = callback_tx.clone();
        let response_prototype = response_prototype.clone(); // Clone the prototype Value (cheap Arc clone)

        // Warp route that handles all requests
        // Note: Logic moved INSIDE the filter to run concurrently on Tokio thread pool
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
                
                // Clone shared resources for this specific request task
                let handler = handler_clone.clone();
                let callback_tx = callback_tx_clone.clone();
                let response_proto = response_prototype.clone();
                
                async move {
                    // --- PRE-PROCESSING (Concurrent) ---
                    // This runs on a worker thread, unrelated to the interpreter lock
                    
                    let path_str = path.as_str().to_string();
                    let body_str = String::from_utf8_lossy(&body).to_string();
                    
                    // Combine path with query string for 'url' field
                    let full_path = if query.is_empty() {
                        path_str.clone()
                    } else {
                        format!("{}?{}", path_str, query)
                    };
                    
                    // Single-Pass Header Processing
                    // Extracts 'host' and builds the Relic map in one go
                    let mut headers_relic = HashMap::new();
                    let mut host = "localhost".to_string();
                    
                    for (k, v) in headers.iter() {
                        let k_str = k.as_str();
                        let v_str = v.to_str().unwrap_or("");
                        
                        if k_str == "host" {
                            host = v_str.to_string();
                        }
                        
                        headers_relic.insert(
                            k_str.to_string(), 
                            Value::String(Arc::new(v_str.to_string()))
                        );
                    }

                    // Parse path and query (cheap string operations)
                    let pathname = path_str.clone();
                    
                    // REMOVED: Eager Cookie Parsing (Expensive & often unused)
                    // Users can parse req.headers["cookie"] if needed
                    let cookies_map = Value::Relic(Arc::new(HashMap::new()));
                    
                    // REMOVED: Eager Query Parsing (Expensive & often unused)
                    // Users can parse req.url or req.query_string if needed
                    let query_map = Value::Relic(Arc::new(HashMap::new()));
                    
                    // Build URL
                    let protocol = "http"; 
                    let url = format!("{}://{}{}", protocol, host, full_path);
                    let ip = addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string());
                    
                    // Create Request Object
                    // Minimized allocations where possible
                    let mut req_map = HashMap::new();
                    req_map.insert("method".to_string(), Value::String(Arc::new(method.to_string())));
                    req_map.insert("url".to_string(), Value::String(Arc::new(url)));
                    req_map.insert("path".to_string(), Value::String(Arc::new(full_path))); // Full path with query
                    req_map.insert("pathname".to_string(), Value::String(Arc::new(pathname))); // Just path
                    req_map.insert("query".to_string(), query_map); // Empty (Lazy)
                    req_map.insert("headers".to_string(), Value::Relic(Arc::new(headers_relic)));
                    req_map.insert("cookies".to_string(), cookies_map); // Empty (Lazy)
                    req_map.insert("body".to_string(), Value::String(Arc::new(body_str)));
                    req_map.insert("ip".to_string(), Value::String(Arc::new(ip)));
                    req_map.insert("host".to_string(), Value::String(Arc::new(host)));
                    req_map.insert("protocol".to_string(), Value::String(Arc::new(protocol.to_string())));
                    
                    let request_value = Value::Relic(Arc::new(req_map));
                    
                    // Use cached Response Prototype (Ref count bump only, no allocation)
                    let response_value = response_proto;

                    // --- DISPATCH TO INTERPRETER ---
                    let (response_tx, response_rx) = tokio::sync::oneshot::channel();
                    
                    let callback_request = crate::runtime::WebCallbackRequest {
                        callback: handler,
                        args: vec![request_value, response_value],
                        response_tx,
                    };

                    if callback_tx.send(callback_request).is_err() {
                        return Ok::<_, warp::Rejection>(
                            warp::reply::with_status(
                                "Server Busy",
                                warp::http::StatusCode::SERVICE_UNAVAILABLE,
                            ).into_response()
                        );
                    }

                    // Wait for result from Interpreter
                    match response_rx.await {
                        Ok(result) => {
                            let (status, body, content_type, custom_headers) = extract_response(result);
                            
                            let status_code = warp::http::StatusCode::from_u16(status)
                                .unwrap_or(warp::http::StatusCode::OK);
                                
                            let mut reply = warp::reply::with_status(body, status_code).into_response();
                            
                            reply.headers_mut().insert(
                                "Content-Type",
                                content_type.parse().unwrap_or_else(|_| "text/plain".parse().unwrap())
                            );
                            
                            for (name, value) in custom_headers {
                                if let (Ok(n), Ok(v)) = (
                                    warp::http::header::HeaderName::try_from(name.as_str()),
                                    warp::http::header::HeaderValue::try_from(value.as_str())
                                ) {
                                    reply.headers_mut().insert(n, v);
                                }
                            }
                            
                            Ok(reply)
                        }
                        Err(_) => Ok(
                            warp::reply::with_status(
                                "Evaluation Error",
                                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                            ).into_response()
                        ),
                    }
                }
            });

        // Run server with graceful shutdown
        let addr = ([0, 0, 0, 0], port);
        let (_, server) = warp::serve(routes)
            .bind_with_graceful_shutdown(addr, async {
                let _ = shutdown_rx.await;
            });

        server.await;
        
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

    let body = crate::stdlib::json::value_to_json_string(&args[0]);

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
