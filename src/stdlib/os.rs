use crate::error::FlowError;
use crate::types::{NativeFn, Value};
use std::sync::Arc;
use std::env;

pub fn load_os_module() -> Vec<(&'static str, Value)> {
    vec![
        // OS Information
        ("name", Value::NativeFunction(NativeFn::new(os_name))),
        ("arch", Value::NativeFunction(NativeFn::new(os_arch))),
        ("family", Value::NativeFunction(NativeFn::new(os_family))),
        ("version", Value::NativeFunction(NativeFn::new(os_version))),
        
        // Environment
        ("env", Value::NativeFunction(NativeFn::new(os_env))),
        ("set_env", Value::NativeFunction(NativeFn::new(os_set_env))),
        ("cwd", Value::NativeFunction(NativeFn::new(os_cwd))),
        ("home_dir", Value::NativeFunction(NativeFn::new(os_home_dir))),
        
        // Process
        ("pid", Value::NativeFunction(NativeFn::new(os_pid))),
    ]
}

// Get OS name
fn os_name(_args: Vec<Value>) -> Result<Value, FlowError> {
    Ok(Value::String(Arc::new(env::consts::OS.to_string())))
}

// Get architecture
fn os_arch(_args: Vec<Value>) -> Result<Value, FlowError> {
    Ok(Value::String(Arc::new(env::consts::ARCH.to_string())))
}

// Get OS family
fn os_family(_args: Vec<Value>) -> Result<Value, FlowError> {
    Ok(Value::String(Arc::new(env::consts::FAMILY.to_string())))
}

// Get OS version (best effort)
fn os_version(_args: Vec<Value>) -> Result<Value, FlowError> {
    // This is a simplified version - for full OS version detection,
    // you'd need platform-specific crates
    #[cfg(target_os = "windows")]
    {
        Ok(Value::String(Arc::new("Windows".to_string())))
    }
    #[cfg(target_os = "linux")]
    {
        Ok(Value::String(Arc::new("Linux".to_string())))
    }
    #[cfg(target_os = "macos")]
    {
        Ok(Value::String(Arc::new("macOS".to_string())))
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        Ok(Value::String(Arc::new("Unknown".to_string())))
    }
}

// Get environment variable
fn os_env(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "os::env expects 1 argument (variable name)",
            0,
            0,
        ));
    }

    let var_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(FlowError::type_error("os::env expects a Silk (string)", 0, 0)),
    };

    match env::var(var_name.as_str()) {
        Ok(value) => Ok(Value::String(Arc::new(value))),
        Err(_) => Ok(Value::Null),
    }
}

// Set environment variable
fn os_set_env(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 2 {
        return Err(FlowError::runtime(
            "os::set_env expects 2 arguments (name, value)",
            0,
            0,
        ));
    }

    let name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(FlowError::type_error("os::set_env expects Silk for name", 0, 0)),
    };

    let value = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(FlowError::type_error("os::set_env expects Silk for value", 0, 0)),
    };

    env::set_var(name.as_str(), value.as_str());
    Ok(Value::Null)
}

// Get current working directory
fn os_cwd(_args: Vec<Value>) -> Result<Value, FlowError> {
    match env::current_dir() {
        Ok(path) => Ok(Value::String(Arc::new(path.to_string_lossy().to_string()))),
        Err(e) => Err(FlowError::runtime(&format!("Failed to get cwd: {}", e), 0, 0)),
    }
}

// Get home directory
fn os_home_dir(_args: Vec<Value>) -> Result<Value, FlowError> {
    match env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
        Ok(home) => Ok(Value::String(Arc::new(home))),
        Err(_) => Ok(Value::Null),
    }
}

// Get process ID
fn os_pid(_args: Vec<Value>) -> Result<Value, FlowError> {
    Ok(Value::Number(std::process::id() as f64))
}
