use crate::error::FlowError;
use crate::types::{NativeFn, Value};
use std::io::{self, Write};
use std::sync::Arc;

pub fn load_cli_module() -> Vec<(&'static str, Value)> {
    vec![
        ("input", Value::NativeFunction(NativeFn::new(cli_input))),
        ("args", Value::NativeFunction(NativeFn::new(cli_args))),
        ("confirm", Value::NativeFunction(NativeFn::new(cli_confirm))),
        ("select", Value::NativeFunction(NativeFn::new(cli_select))),
        ("clear", Value::NativeFunction(NativeFn::new(cli_clear))),
        ("exit", Value::NativeFunction(NativeFn::new(cli_exit))),
    ]
}

// cli::input(prompt: Silk) -> Silk
fn cli_input(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "cli::input expects 1 argument (prompt)",
            0,
            0,
        ));
    }

    let prompt = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "cli::input expects a Silk prompt",
                0,
                0,
            ))
        }
    };

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(Value::String(Arc::new(input.trim().to_string()))),
        Err(e) => Err(FlowError::runtime(
            &format!("Failed to read input: {}", e),
            0,
            0,
        )),
    }
}

// cli::args() -> Constellation<Silk>
fn cli_args(_args: Vec<Value>) -> Result<Value, FlowError> {
    // Read from environment variable set by main.rs
    let args: Vec<Value> = if let Ok(args_str) = std::env::var("FLOWLANG_SCRIPT_ARGS") {
        if args_str.is_empty() {
            Vec::new()
        } else {
            args_str
                .split('\x1F') // Unit separator
                .map(|arg| Value::String(Arc::new(arg.to_string())))
                .collect()
        }
    } else {
        Vec::new()
    };

    Ok(Value::Array(Arc::new(args)))
}

// cli::confirm(prompt: Silk) -> Pulse
fn cli_confirm(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "cli::confirm expects 1 argument (prompt)",
            0,
            0,
        ));
    }

    let prompt = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "cli::confirm expects a Silk prompt",
                0,
                0,
            ))
        }
    };

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let response = input.trim().to_lowercase();
            let is_yes = response == "y" || response == "yes";
            Ok(Value::Boolean(is_yes))
        }
        Err(e) => Err(FlowError::runtime(
            &format!("Failed to read input: {}", e),
            0,
            0,
        )),
    }
}

// cli::select(prompt: Silk, options: Constellation<Silk>) -> Silk
fn cli_select(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 2 {
        return Err(FlowError::runtime(
            "cli::select expects 2 arguments (prompt, options)",
            0,
            0,
        ));
    }

    let prompt = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "cli::select expects a Silk prompt",
                0,
                0,
            ))
        }
    };

    let options = match &args[1] {
        Value::Array(arr) => arr.clone(),
        _ => {
            return Err(FlowError::type_error(
                "cli::select expects a Constellation of options",
                0,
                0,
            ))
        }
    };

    if options.is_empty() {
        return Err(FlowError::runtime(
            "cli::select requires at least one option",
            0,
            0,
        ));
    }

    // Display prompt and options
    println!("{}", prompt);
    for (i, option) in options.iter().enumerate() {
        if let Value::String(s) = option {
            println!("  {}. {}", i + 1, s);
        }
    }

    // Get user selection
    loop {
        print!("Enter choice (1-{}): ", options.len());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        if let Ok(choice) = input.trim().parse::<usize>() {
            if choice > 0 && choice <= options.len() {
                if let Value::String(s) = &options[choice - 1] {
                    return Ok(Value::String(s.clone()));
                }
            }
        }
        println!("Invalid choice. Please try again.");
    }
}

// cli::clear() -> Hollow
fn cli_clear(_args: Vec<Value>) -> Result<Value, FlowError> {
    // Clear screen using ANSI escape codes (works on most terminals)
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
    Ok(Value::Null)
}

// cli::exit(code: Ember) -> Hollow
fn cli_exit(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "cli::exit expects 1 argument (exit code)",
            0,
            0,
        ));
    }

    let code = match &args[0] {
        Value::Number(n) => *n as i32,
        _ => {
            return Err(FlowError::type_error(
                "cli::exit expects an Ember exit code",
                0,
                0,
            ))
        }
    };

    std::process::exit(code);
}
