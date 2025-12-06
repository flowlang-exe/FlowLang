use crate::error::FlowError;
use crate::types::{NativeFn, Value};
use std::sync::Arc;
use colored::Colorize;

pub fn load_color_module() -> Vec<(&'static str, Value)> {
    vec![
        // Basic colors
        ("red", Value::NativeFunction(NativeFn::new(color_red))),
        ("green", Value::NativeFunction(NativeFn::new(color_green))),
        ("blue", Value::NativeFunction(NativeFn::new(color_blue))),
        ("yellow", Value::NativeFunction(NativeFn::new(color_yellow))),
        ("magenta", Value::NativeFunction(NativeFn::new(color_magenta))),
        ("cyan", Value::NativeFunction(NativeFn::new(color_cyan))),
        ("white", Value::NativeFunction(NativeFn::new(color_white))),
        ("black", Value::NativeFunction(NativeFn::new(color_black))),
        
        // Bright colors
        ("bright_red", Value::NativeFunction(NativeFn::new(color_bright_red))),
        ("bright_green", Value::NativeFunction(NativeFn::new(color_bright_green))),
        ("bright_blue", Value::NativeFunction(NativeFn::new(color_bright_blue))),
        ("bright_yellow", Value::NativeFunction(NativeFn::new(color_bright_yellow))),
        ("bright_magenta", Value::NativeFunction(NativeFn::new(color_bright_magenta))),
        ("bright_cyan", Value::NativeFunction(NativeFn::new(color_bright_cyan))),
        
        // Styles
        ("bold", Value::NativeFunction(NativeFn::new(style_bold))),
        ("italic", Value::NativeFunction(NativeFn::new(style_italic))),
        ("underline", Value::NativeFunction(NativeFn::new(style_underline))),
        ("dimmed", Value::NativeFunction(NativeFn::new(style_dimmed))),
        ("strikethrough", Value::NativeFunction(NativeFn::new(style_strikethrough))),
    ]
}

// Helper function to get string from args
fn get_string_arg(args: &[Value], fn_name: &str) -> Result<Arc<String>, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            &format!("color::{} expects 1 argument (text)", fn_name),
            0,
            0,
        ));
    }

    match &args[0] {
        Value::String(s) => Ok(s.clone()),
        _ => Err(FlowError::type_error(
            &format!("color::{} expects a Silk (string)", fn_name),
            0,
            0,
        )),
    }
}

// Basic colors
fn color_red(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "red")?;
    Ok(Value::String(Arc::new(text.red().to_string())))
}

fn color_green(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "green")?;
    Ok(Value::String(Arc::new(text.green().to_string())))
}

fn color_blue(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "blue")?;
    Ok(Value::String(Arc::new(text.blue().to_string())))
}

fn color_yellow(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "yellow")?;
    Ok(Value::String(Arc::new(text.yellow().to_string())))
}

fn color_magenta(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "magenta")?;
    Ok(Value::String(Arc::new(text.magenta().to_string())))
}

fn color_cyan(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "cyan")?;
    Ok(Value::String(Arc::new(text.cyan().to_string())))
}

fn color_white(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "white")?;
    Ok(Value::String(Arc::new(text.white().to_string())))
}

fn color_black(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "black")?;
    Ok(Value::String(Arc::new(text.black().to_string())))
}

// Bright colors
fn color_bright_red(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "bright_red")?;
    Ok(Value::String(Arc::new(text.bright_red().to_string())))
}

fn color_bright_green(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "bright_green")?;
    Ok(Value::String(Arc::new(text.bright_green().to_string())))
}

fn color_bright_blue(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "bright_blue")?;
    Ok(Value::String(Arc::new(text.bright_blue().to_string())))
}

fn color_bright_yellow(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "bright_yellow")?;
    Ok(Value::String(Arc::new(text.bright_yellow().to_string())))
}

fn color_bright_magenta(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "bright_magenta")?;
    Ok(Value::String(Arc::new(text.bright_magenta().to_string())))
}

fn color_bright_cyan(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "bright_cyan")?;
    Ok(Value::String(Arc::new(text.bright_cyan().to_string())))
}

// Styles
fn style_bold(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "bold")?;
    Ok(Value::String(Arc::new(text.bold().to_string())))
}

fn style_italic(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "italic")?;
    Ok(Value::String(Arc::new(text.italic().to_string())))
}

fn style_underline(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "underline")?;
    Ok(Value::String(Arc::new(text.underline().to_string())))
}

fn style_dimmed(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "dimmed")?;
    Ok(Value::String(Arc::new(text.dimmed().to_string())))
}

fn style_strikethrough(args: Vec<Value>) -> Result<Value, FlowError> {
    let text = get_string_arg(&args, "strikethrough")?;
    Ok(Value::String(Arc::new(text.strikethrough().to_string())))
}
