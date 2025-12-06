//! std:stream - File Streaming Module
//!
//! Provides file streaming functionality for serving files and handling large data.

use crate::error::FlowError;
use crate::types::{Value, NativeFn};
use std::collections::HashMap;
use std::sync::Arc;
use std::fs;
use std::path::Path;

/// Load the stream module
pub fn load_stream_module() -> Vec<(&'static str, Value)> {
    vec![
        ("readFile", Value::NativeFunction(NativeFn(Arc::new(stream_read_file)))),
        ("readText", Value::NativeFunction(NativeFn(Arc::new(stream_read_text)))),
        ("readBytes", Value::NativeFunction(NativeFn(Arc::new(stream_read_bytes)))),
        ("writeFile", Value::NativeFunction(NativeFn(Arc::new(stream_write_file)))),
        ("exists", Value::NativeFunction(NativeFn(Arc::new(stream_exists)))),
        ("stat", Value::NativeFunction(NativeFn(Arc::new(stream_stat)))),
        ("mimeType", Value::NativeFunction(NativeFn(Arc::new(stream_mime_type)))),
    ]
}

/// stream.readFile(path) -> Relic { content, size, mimeType }
/// Read a file and return its content with metadata
fn stream_read_file(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("stream.readFile expects 1 argument (path)", 0, 0));
    }

    let path_str = args[0].to_string();
    let path = Path::new(&path_str);
    
    if !path.exists() {
        return Err(FlowError::runtime(
            &format!("File not found: {}", path_str),
            0, 0,
        ));
    }
    
    let content = fs::read(&path).map_err(|e| {
        FlowError::runtime(&format!("Failed to read file: {}", e), 0, 0)
    })?;
    
    let metadata = fs::metadata(&path).ok();
    let size = metadata.as_ref().map(|m| m.len() as f64).unwrap_or(0.0);
    
    let mime = get_mime_type(&path_str);
    
    // For text files, return as string; for binary, return as base64
    let content_value = if is_text_mime(&mime) {
        Value::String(Arc::new(String::from_utf8_lossy(&content).to_string()))
    } else {
        // Return as base64 for binary files
        Value::String(Arc::new(base64_encode(&content)))
    };
    
    let mut result = HashMap::new();
    result.insert("content".to_string(), content_value);
    result.insert("size".to_string(), Value::Number(size));
    result.insert("mimeType".to_string(), Value::String(Arc::new(mime)));
    result.insert("path".to_string(), Value::String(Arc::new(path_str)));
    
    Ok(Value::Relic(Arc::new(result)))
}

/// stream.readText(path) -> Silk
/// Read a file as text
fn stream_read_text(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("stream.readText expects 1 argument (path)", 0, 0));
    }

    let path_str = args[0].to_string();
    
    let content = fs::read_to_string(&path_str).map_err(|e| {
        FlowError::runtime(&format!("Failed to read file: {}", e), 0, 0)
    })?;
    
    Ok(Value::String(Arc::new(content)))
}

/// stream.readBytes(path) -> Constellation
/// Read a file as bytes (array of numbers)
fn stream_read_bytes(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("stream.readBytes expects 1 argument (path)", 0, 0));
    }

    let path_str = args[0].to_string();
    
    let content = fs::read(&path_str).map_err(|e| {
        FlowError::runtime(&format!("Failed to read file: {}", e), 0, 0)
    })?;
    
    let bytes: Vec<Value> = content.iter().map(|b| Value::Number(*b as f64)).collect();
    
    Ok(Value::Array(Arc::new(bytes)))
}

/// stream.writeFile(path, content) -> Pulse
/// Write content to a file
fn stream_write_file(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() < 2 {
        return Err(FlowError::runtime("stream.writeFile expects 2 arguments (path, content)", 0, 0));
    }

    let path_str = args[0].to_string();
    let content = args[1].to_string();
    
    fs::write(&path_str, content).map_err(|e| {
        FlowError::runtime(&format!("Failed to write file: {}", e), 0, 0)
    })?;
    
    Ok(Value::Boolean(true))
}

/// stream.exists(path) -> Pulse
/// Check if a file exists
fn stream_exists(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("stream.exists expects 1 argument (path)", 0, 0));
    }

    let path_str = args[0].to_string();
    let exists = Path::new(&path_str).exists();
    
    Ok(Value::Boolean(exists))
}

/// stream.stat(path) -> Relic { size, isFile, isDir, modified }
/// Get file/directory statistics
fn stream_stat(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("stream.stat expects 1 argument (path)", 0, 0));
    }

    let path_str = args[0].to_string();
    let path = Path::new(&path_str);
    
    let metadata = fs::metadata(path).map_err(|e| {
        FlowError::runtime(&format!("Failed to get file stats: {}", e), 0, 0)
    })?;
    
    let mut result = HashMap::new();
    result.insert("size".to_string(), Value::Number(metadata.len() as f64));
    result.insert("isFile".to_string(), Value::Boolean(metadata.is_file()));
    result.insert("isDir".to_string(), Value::Boolean(metadata.is_dir()));
    
    Ok(Value::Relic(Arc::new(result)))
}

/// stream.mimeType(path) -> Silk
/// Get the MIME type for a file based on extension
fn stream_mime_type(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("stream.mimeType expects 1 argument (path)", 0, 0));
    }

    let path_str = args[0].to_string();
    let mime = get_mime_type(&path_str);
    
    Ok(Value::String(Arc::new(mime)))
}

/// Get MIME type from file extension
fn get_mime_type(path: &str) -> String {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    match ext.as_str() {
        // Text
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "txt" => "text/plain",
        "md" => "text/markdown",
        "csv" => "text/csv",
        
        // Images
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "webp" => "image/webp",
        
        // Fonts
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        
        // Media
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        
        // Documents
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        
        _ => "application/octet-stream",
    }.to_string()
}

/// Check if MIME type is text-based
fn is_text_mime(mime: &str) -> bool {
    mime.starts_with("text/") || 
    mime == "application/json" ||
    mime == "application/xml" ||
    mime == "application/javascript"
}

/// Simple base64 encoding
fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut result = String::new();
    
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
        let b2 = chunk.get(2).copied().unwrap_or(0) as usize;
        
        result.push(CHARS[(b0 >> 2)] as char);
        result.push(CHARS[((b0 & 0x03) << 4) | (b1 >> 4)] as char);
        
        if chunk.len() > 1 {
            result.push(CHARS[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(CHARS[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}
