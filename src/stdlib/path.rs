//! std:path - Path manipulation module (Node.js-style)

use crate::error::FlowError;
use crate::types::{NativeFn, Value};
use std::collections::HashMap;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::sync::Arc;

pub fn load_path_module() -> Vec<(&'static str, Value)> {
    vec![
        ("join", Value::NativeFunction(NativeFn::new(path_join))),
        ("dirname", Value::NativeFunction(NativeFn::new(path_dirname))),
        ("basename", Value::NativeFunction(NativeFn::new(path_basename))),
        ("extname", Value::NativeFunction(NativeFn::new(path_extname))),
        ("parse", Value::NativeFunction(NativeFn::new(path_parse))),
        ("format", Value::NativeFunction(NativeFn::new(path_format))),
        ("resolve", Value::NativeFunction(NativeFn::new(path_resolve))),
        ("normalize", Value::NativeFunction(NativeFn::new(path_normalize))),
        ("isAbsolute", Value::NativeFunction(NativeFn::new(path_is_absolute))),
        ("relative", Value::NativeFunction(NativeFn::new(path_relative))),
        ("sep", Value::String(Arc::new(MAIN_SEPARATOR.to_string()))),
    ]
}

/// path.join(...parts) -> Silk
/// Join path segments with platform separator
fn path_join(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Ok(Value::String(Arc::new(String::new())));
    }

    let mut path = PathBuf::new();
    for arg in args {
        path.push(arg.to_string());
    }

    Ok(Value::String(Arc::new(path.to_string_lossy().to_string())))
}

/// path.dirname(path) -> Silk
/// Get directory name from path
fn path_dirname(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("path.dirname expects 1 argument (path)", 0, 0));
    }

    let path_str = args[0].to_string();
    let path = Path::new(&path_str);
    let dirname = path.parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string());

    Ok(Value::String(Arc::new(dirname)))
}

/// path.basename(path, ext?) -> Silk
/// Get base name from path, optionally removing extension
fn path_basename(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("path.basename expects 1-2 arguments (path, ext?)", 0, 0));
    }

    let path_str = args[0].to_string();
    let path = Path::new(&path_str);
    let basename = path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    // If extension provided, remove it
    if args.len() > 1 {
        let ext = args[1].to_string();
        if basename.ends_with(&ext) {
            return Ok(Value::String(Arc::new(basename[..basename.len() - ext.len()].to_string())));
        }
    }

    Ok(Value::String(Arc::new(basename)))
}

/// path.extname(path) -> Silk
/// Get file extension including the dot
fn path_extname(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("path.extname expects 1 argument (path)", 0, 0));
    }

    let path_str = args[0].to_string();
    let path = Path::new(&path_str);
    let ext = path.extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();

    Ok(Value::String(Arc::new(ext)))
}

/// path.parse(path) -> Relic
/// Parse path into components: root, dir, base, ext, name
fn path_parse(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("path.parse expects 1 argument (path)", 0, 0));
    }

    let path_str = args[0].to_string();
    let path = Path::new(&path_str);

    let mut map = HashMap::new();

    // root (e.g., "/" on Unix, "C:\" on Windows)
    let root = if path.is_absolute() {
        #[cfg(windows)]
        {
            path.components().next()
                .map(|c| c.as_os_str().to_string_lossy().to_string())
                .unwrap_or_default()
        }
        #[cfg(not(windows))]
        {
            "/".to_string()
        }
    } else {
        String::new()
    };

    // dir (directory portion)
    let dir = path.parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    // base (filename with extension)
    let base = path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    // ext (extension with dot)
    let ext = path.extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();

    // name (filename without extension)
    let name = path.file_stem()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    map.insert("root".to_string(), Value::String(Arc::new(root)));
    map.insert("dir".to_string(), Value::String(Arc::new(dir)));
    map.insert("base".to_string(), Value::String(Arc::new(base)));
    map.insert("ext".to_string(), Value::String(Arc::new(ext)));
    map.insert("name".to_string(), Value::String(Arc::new(name)));

    Ok(Value::Relic(Arc::new(map)))
}

/// path.format(pathObject) -> Silk
/// Format path object back to string
fn path_format(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("path.format expects 1 argument (pathObject)", 0, 0));
    }

    let map = match &args[0] {
        Value::Relic(m) => m.clone(),
        _ => return Err(FlowError::type_error("path.format expects a Relic", 0, 0)),
    };

    // If dir and base are provided, use them
    let dir = map.get("dir").map(|v| v.to_string()).unwrap_or_default();
    let base = map.get("base").map(|v| v.to_string()).unwrap_or_default();

    if !base.is_empty() {
        if dir.is_empty() {
            return Ok(Value::String(Arc::new(base)));
        }
        let mut path = PathBuf::from(&dir);
        path.push(&base);
        return Ok(Value::String(Arc::new(path.to_string_lossy().to_string())));
    }

    // Otherwise use name + ext
    let name = map.get("name").map(|v| v.to_string()).unwrap_or_default();
    let ext = map.get("ext").map(|v| v.to_string()).unwrap_or_default();

    let filename = format!("{}{}", name, ext);
    if dir.is_empty() {
        return Ok(Value::String(Arc::new(filename)));
    }

    let mut path = PathBuf::from(&dir);
    path.push(&filename);
    Ok(Value::String(Arc::new(path.to_string_lossy().to_string())))
}

/// path.resolve(...paths) -> Silk
/// Resolve path segments to absolute path
fn path_resolve(args: Vec<Value>) -> Result<Value, FlowError> {
    let mut path = std::env::current_dir().unwrap_or_default();

    for arg in args {
        let segment = arg.to_string();
        let segment_path = Path::new(&segment);
        
        if segment_path.is_absolute() {
            path = PathBuf::from(segment);
        } else {
            path.push(segment);
        }
    }

    // Normalize the path
    Ok(Value::String(Arc::new(path.to_string_lossy().to_string())))
}

/// path.normalize(path) -> Silk
/// Normalize a path, resolving '..' and '.'
fn path_normalize(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("path.normalize expects 1 argument (path)", 0, 0));
    }

    let path_str = args[0].to_string();
    let path = PathBuf::from(&path_str);

    // Simple normalization using canonicalize if exists, otherwise just clean up
    let normalized = if path.exists() {
        path.canonicalize()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or(path_str)
    } else {
        // Manual normalization for non-existent paths
        let mut components: Vec<&str> = Vec::new();
        for part in path_str.split(['/', '\\']) {
            match part {
                "" | "." => continue,
                ".." => { components.pop(); }
                p => components.push(p),
            }
        }
        components.join(&MAIN_SEPARATOR.to_string())
    };

    Ok(Value::String(Arc::new(normalized)))
}

/// path.isAbsolute(path) -> Pulse
/// Check if path is absolute
fn path_is_absolute(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("path.isAbsolute expects 1 argument (path)", 0, 0));
    }

    let path_str = args[0].to_string();
    let path = Path::new(&path_str);
    Ok(Value::Boolean(path.is_absolute()))
}

/// path.relative(from, to) -> Silk
/// Get relative path from one path to another
fn path_relative(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() < 2 {
        return Err(FlowError::runtime("path.relative expects 2 arguments (from, to)", 0, 0));
    }

    let from = PathBuf::from(args[0].to_string());
    let to = PathBuf::from(args[1].to_string());

    // Simple relative path calculation
    let from_abs = if from.is_absolute() {
        from
    } else {
        std::env::current_dir().unwrap_or_default().join(&from)
    };

    let to_abs = if to.is_absolute() {
        to
    } else {
        std::env::current_dir().unwrap_or_default().join(&to)
    };

    // Try to use pathdiff crate logic manually
    let relative = pathdiff_relative(&from_abs, &to_abs);
    Ok(Value::String(Arc::new(relative)))
}

/// Simple relative path calculation
fn pathdiff_relative(from: &Path, to: &Path) -> String {
    let from_components: Vec<_> = from.components().collect();
    let to_components: Vec<_> = to.components().collect();

    // Find common prefix
    let mut common_len = 0;
    for (a, b) in from_components.iter().zip(to_components.iter()) {
        if a == b {
            common_len += 1;
        } else {
            break;
        }
    }

    // Build relative path
    let mut result = PathBuf::new();
    
    // Add ".." for each remaining component in "from"
    for _ in common_len..from_components.len() {
        result.push("..");
    }
    
    // Add remaining components from "to"
    for component in &to_components[common_len..] {
        result.push(component);
    }

    if result.as_os_str().is_empty() {
        ".".to_string()
    } else {
        result.to_string_lossy().to_string()
    }
}
