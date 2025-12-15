//! std:process - Process/Command execution module
//!
//! Provides functions for running external commands and processes.

use crate::types::{NativeFn, Value};
use crate::error::FlowError;
use std::collections::HashMap;
use std::sync::Arc;
use std::process::{Command, Stdio};

/// Load the process module
pub fn load_process_module() -> Vec<(&'static str, Value)> {
    vec![
        ("exec", Value::NativeFunction(NativeFn::new(process_exec))),
        ("run", Value::NativeFunction(NativeFn::new(process_run))),
        ("output", Value::NativeFunction(NativeFn::new(process_output))),
    ]
}

/// Execute a command string and return the output
/// proc.exec("ls -la") -> Silk (stdout)
fn process_exec(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("exec() requires a command string", 0, 0));
    }

    let cmd_str = args[0].to_string();

    // Determine shell based on OS
    #[cfg(windows)]
    let output = Command::new("cmd")
        .args(["/C", &cmd_str])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    #[cfg(not(windows))]
    let output = Command::new("sh")
        .args(["-c", &cmd_str])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            
            if output.status.success() {
                Ok(Value::String(Arc::new(stdout)))
            } else {
                // Return stderr if command failed
                if !stderr.is_empty() {
                    Err(FlowError::runtime(&format!("Command failed: {}", stderr.trim()), 0, 0))
                } else {
                    Err(FlowError::runtime(&format!("Command failed with exit code: {:?}", output.status.code()), 0, 0))
                }
            }
        }
        Err(e) => Err(FlowError::runtime(&format!("Failed to execute command: {}", e), 0, 0)),
    }
}

/// Run a program with arguments array, return exit code
/// proc.run("git", ["clone", url, dest]) -> Ember (exit code)
fn process_run(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("run() requires a program name", 0, 0));
    }

    let program = args[0].to_string();
    
    // Get arguments array
    let cmd_args: Vec<String> = if args.len() > 1 {
        match &args[1] {
            Value::Array(arr) => {
                arr.iter().map(|v| v.to_string()).collect()
            }
            _ => vec![args[1].to_string()],
        }
    } else {
        vec![]
    };

    let status = Command::new(&program)
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(status) => {
            let code = status.code().unwrap_or(-1) as f64;
            Ok(Value::Number(code))
        }
        Err(e) => Err(FlowError::runtime(&format!("Failed to run '{}': {}", program, e), 0, 0)),
    }
}

/// Run a program and capture output as a Relic {stdout, stderr, code}
/// proc.output("git", ["status"]) -> Relic
fn process_output(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.is_empty() {
        return Err(FlowError::runtime("output() requires a program name", 0, 0));
    }

    let program = args[0].to_string();
    
    // Get arguments array
    let cmd_args: Vec<String> = if args.len() > 1 {
        match &args[1] {
            Value::Array(arr) => {
                arr.iter().map(|v| v.to_string()).collect()
            }
            _ => vec![args[1].to_string()],
        }
    } else {
        vec![]
    };

    let output = Command::new(&program)
        .args(&cmd_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let code = output.status.code().unwrap_or(-1) as f64;

            let mut result = HashMap::new();
            result.insert("stdout".to_string(), Value::String(Arc::new(stdout)));
            result.insert("stderr".to_string(), Value::String(Arc::new(stderr)));
            result.insert("code".to_string(), Value::Number(code));
            result.insert("success".to_string(), Value::Boolean(output.status.success()));

            Ok(Value::Relic(Arc::new(result)))
        }
        Err(e) => Err(FlowError::runtime(&format!("Failed to run '{}': {}", program, e), 0, 0)),
    }
}
