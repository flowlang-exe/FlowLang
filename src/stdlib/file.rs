use crate::error::FlowError;
use crate::types::{NativeFn, Value};
use std::fs;
use std::path::Path;
use std::sync::Arc;

pub fn load_file_module() -> Vec<(&'static str, Value)> {
    vec![
        ("read", Value::NativeFunction(NativeFn::new(file_read))),
        ("write", Value::NativeFunction(NativeFn::new(file_write))),
        ("exists", Value::NativeFunction(NativeFn::new(file_exists))),
        ("delete", Value::NativeFunction(NativeFn::new(file_delete))),
        ("list", Value::NativeFunction(NativeFn::new(file_list))),
        ("create_dir", Value::NativeFunction(NativeFn::new(file_create_dir))),
    ]
}

// file::read(path: Silk) -> Silk
fn file_read(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "file::read expects 1 argument (path)",
            0,
            0,
        ));
    }

    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "file::read expects a string path",
                0,
                0,
            ))
        }
    };

    match fs::read_to_string(&*path) {
        Ok(content) => Ok(Value::String(Arc::new(content))),
        Err(e) => Err(FlowError::runtime(
            &format!("Failed to read file '{}': {}", path, e),
            0,
            0,
        )),
    }
}

// file::write(path: Silk, content: Silk) -> Pulse
fn file_write(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 2 {
        return Err(FlowError::runtime(
            "file::write expects 2 arguments (path, content)",
            0,
            0,
        ));
    }

    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "file::write expects a string path",
                0,
                0,
            ))
        }
    };

    let content = match &args[1] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "file::write expects a string content",
                0,
                0,
            ))
        }
    };

    match fs::write(&*path, &*content) {
        Ok(_) => Ok(Value::Boolean(true)),
        Err(e) => Err(FlowError::runtime(
            &format!("Failed to write file '{}': {}", path, e),
            0,
            0,
        )),
    }
}

// file::exists(path: Silk) -> Pulse
fn file_exists(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "file::exists expects 1 argument (path)",
            0,
            0,
        ));
    }

    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "file::exists expects a string path",
                0,
                0,
            ))
        }
    };

    Ok(Value::Boolean(Path::new(&*path).exists()))
}

// file::delete(path: Silk) -> Pulse
fn file_delete(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "file::delete expects 1 argument (path)",
            0,
            0,
        ));
    }

    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "file::delete expects a string path",
                0,
                0,
            ))
        }
    };

    let path_obj = Path::new(&*path);
    
    if path_obj.is_file() {
        match fs::remove_file(path_obj) {
            Ok(_) => Ok(Value::Boolean(true)),
            Err(e) => Err(FlowError::runtime(
                &format!("Failed to delete file '{}': {}", path, e),
                0,
                0,
            )),
        }
    } else if path_obj.is_dir() {
        match fs::remove_dir_all(path_obj) {
            Ok(_) => Ok(Value::Boolean(true)),
            Err(e) => Err(FlowError::runtime(
                &format!("Failed to delete directory '{}': {}", path, e),
                0,
                0,
            )),
        }
    } else {
        Err(FlowError::runtime(
            &format!("Path '{}' does not exist", path),
            0,
            0,
        ))
    }
}

// file::list(path: Silk) -> Constellation<Silk>
fn file_list(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "file::list expects 1 argument (path)",
            0,
            0,
        ));
    }

    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "file::list expects a string path",
                0,
                0,
            ))
        }
    };

    match fs::read_dir(&*path) {
        Ok(entries) => {
            let mut files = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        files.push(Value::String(Arc::new(name.to_string())));
                    }
                }
            }
            Ok(Value::Array(Arc::new(files)))
        }
        Err(e) => Err(FlowError::runtime(
            &format!("Failed to list directory '{}': {}", path, e),
            0,
            0,
        )),
    }
}

// file::create_dir(path: Silk) -> Pulse
fn file_create_dir(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "file::create_dir expects 1 argument (path)",
            0,
            0,
        ));
    }

    let path = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "file::create_dir expects a string path",
                0,
                0,
            ))
        }
    };

    match fs::create_dir_all(&*path) {
        Ok(_) => Ok(Value::Boolean(true)),
        Err(e) => Err(FlowError::runtime(
            &format!("Failed to create directory '{}': {}", path, e),
            0,
            0,
        )),
    }
}
