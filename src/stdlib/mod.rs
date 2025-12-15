pub mod io;
pub mod math;
pub mod string;
pub mod array;
pub mod file;
pub mod json;
pub mod net;
pub mod time;
pub mod cli;
pub mod color;
pub mod crypto;
pub mod os;
pub mod timer;
pub mod web;
pub mod url;
pub mod stream;
pub mod path;
pub mod process;
pub mod git;

use std::collections::HashMap;

use crate::types::Value;
use crate::error::FlowError;

pub fn load_module(name: &str) -> Option<HashMap<String, Value>> {
    match name {
        "math" => Some(math::get_module()),
        "string" => Some(string::get_module()),
        "array" => Some(array::get_module()),
        "file" => {
            let mut map = HashMap::new();
            for (key, value) in file::load_file_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "json" => {
            let mut map = HashMap::new();
            for (key, value) in json::load_json_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "net" => {
            let mut map = HashMap::new();
            for (key, value) in net::load_net_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "time" => {
            let mut map = HashMap::new();
            for (key, value) in time::load_time_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "cli" => {
            let mut map = HashMap::new();
            for (key, value) in cli::load_cli_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "color" => {
            let mut map = HashMap::new();
            for (key, value) in color::load_color_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "crypto" => {
            let mut map = HashMap::new();
            for (key, value) in crypto::load_crypto_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "os" => {
            let mut map = HashMap::new();
            for (key, value) in os::load_os_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "timer" => {
            let mut map = HashMap::new();
            for (key, value) in timer::load_timer_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "web" => {
            let mut map = HashMap::new();
            for (key, value) in web::load_web_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "url" => {
            let mut map = HashMap::new();
            for (key, value) in url::load_url_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "stream" => {
            let mut map = HashMap::new();
            for (key, value) in stream::load_stream_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "path" => {
            let mut map = HashMap::new();
            for (key, value) in path::load_path_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "process" => {
            let mut map = HashMap::new();
            for (key, value) in process::load_process_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        "git" => {
            let mut map = HashMap::new();
            for (key, value) in git::load_git_module() {
                map.insert(key.to_string(), value);
            }
            Some(map)
        }
        _ => None,
    }
}

pub fn call_builtin(name: &str, args: Vec<Value>) -> Result<Value, FlowError> {
    match name {
        "whisper" => {
            if args.len() != 1 {
                return Err(FlowError::runtime(
                    "whisper() expects 1 argument",
                    0,
                    0,
                ));
            }
            io::whisper(&args[0].to_string());
            Ok(Value::Null)
        }
        "shout" => {
            if args.len() != 1 {
                return Err(FlowError::runtime(
                    "shout() expects 1 argument",
                    0,
                    0,
                ));
            }
            io::shout(&args[0].to_string());
            Ok(Value::Null)
        }
        "roar" => {
            if args.len() != 1 {
                return Err(FlowError::runtime(
                    "roar() expects 1 argument",
                    0,
                    0,
                ));
            }
            io::roar(&args[0].to_string());
            Ok(Value::Null)
        }
        "chant" => {
            if args.len() != 1 {
                return Err(FlowError::runtime(
                    "chant() expects 1 argument",
                    0,
                    0,
                ));
            }
            io::chant(&args[0].to_string());
            Ok(Value::Null)
        }
        _ => Err(FlowError::undefined(
            &format!("Unknown built-in function: {}", name),
            0,
            0,
        )),
    }
}

pub fn is_builtin(name: &str) -> bool {
    matches!(name, "whisper" | "shout" | "roar" | "chant" | "drift" | "strike")
}
