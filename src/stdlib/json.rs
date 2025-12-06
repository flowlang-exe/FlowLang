use crate::error::FlowError;
use crate::types::Value;
use crate::types::NativeFn;
use std::sync::Arc;

pub fn load_json_module() -> Vec<(&'static str, Value)> {
    vec![
        ("parse", Value::NativeFunction(NativeFn::new(json_parse))),
        ("stringify", Value::NativeFunction(NativeFn::new(json_stringify))),
    ]
}

// json::parse(json: Silk) -> Flux
fn json_parse(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "json::parse expects 1 argument (json string)",
            0,
            0,
        ));
    }

    let json_str = match &args[0] {
        Value::String(s) => s.clone(),
        _ => {
            return Err(FlowError::type_error(
                "json::parse expects a string",
                0,
                0,
            ))
        }
    };

    parse_json_value(&json_str)
}

// json::stringify(value: Flux) -> Silk
fn json_stringify(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "json::stringify expects 1 argument",
            0,
            0,
        ));
    }

    let json_string = value_to_json_string(&args[0]);
    Ok(Value::String(Arc::new(json_string)))
}

// Helper: Parse JSON string to FlowLang Value
fn parse_json_value(json_str: &str) -> Result<Value, FlowError> {
    let trimmed = json_str.trim();

    // null
    if trimmed == "null" {
        return Ok(Value::Null);
    }

    // boolean
    if trimmed == "true" {
        return Ok(Value::Boolean(true));
    }
    if trimmed == "false" {
        return Ok(Value::Boolean(false));
    }

    // number
    if let Ok(num) = trimmed.parse::<f64>() {
        return Ok(Value::Number(num));
    }

    // string
    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        let content = &trimmed[1..trimmed.len() - 1];
        let unescaped = content
            .replace("\\\"", "\"")
            .replace("\\\\", "\\")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t");
        return Ok(Value::String(Arc::new(unescaped)));
    }

    // array
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        let content = &trimmed[1..trimmed.len() - 1];
        if content.trim().is_empty() {
            return Ok(Value::Array(Arc::new(vec![])));
        }

        let mut elements = Vec::new();
        let mut depth = 0;
        let mut current = String::new();
        let mut in_string = false;

        for ch in content.chars() {
            if ch == '"' && (current.is_empty() || !current.ends_with('\\')) {
                in_string = !in_string;
            }

            if !in_string {
                if ch == '[' || ch == '{' {
                    depth += 1;
                } else if ch == ']' || ch == '}' {
                    depth -= 1;
                } else if ch == ',' && depth == 0 {
                    elements.push(parse_json_value(&current)?);
                    current.clear();
                    continue;
                }
            }

            current.push(ch);
        }

        if !current.trim().is_empty() {
            elements.push(parse_json_value(&current)?);
        }

        return Ok(Value::Array(Arc::new(elements)));
    }

    // object - parse into Relic (HashMap)
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        use std::collections::HashMap;
        
        let content = &trimmed[1..trimmed.len() - 1];
        if content.trim().is_empty() {
            return Ok(Value::Relic(Arc::new(HashMap::new())));
        }

        let mut map: HashMap<String, Value> = HashMap::new();
        let mut depth = 0;
        let mut current = String::new();
        let mut in_string = false;

        for ch in content.chars() {
            if ch == '"' && (current.is_empty() || !current.ends_with('\\')) {
                in_string = !in_string;
            }
            if !in_string {
                if ch == '{' || ch == '[' {
                    depth += 1;
                } else if ch == '}' || ch == ']' {
                    depth -= 1;
                }
            }
            if ch == ',' && depth == 0 && !in_string {
                parse_json_key_value(&current, &mut map)?;
                current.clear();
            } else {
                current.push(ch);
            }
        }

        if !current.trim().is_empty() {
            parse_json_key_value(&current, &mut map)?;
        }

        return Ok(Value::Relic(Arc::new(map)));
    }

    Err(FlowError::runtime(
        &format!("Invalid JSON: {}", json_str),
        0,
        0,
    ))
}

// Helper: Parse a single "key": value pair
fn parse_json_key_value(pair: &str, map: &mut std::collections::HashMap<String, Value>) -> Result<(), FlowError> {
    let pair = pair.trim();
    
    // Find the colon separating key from value
    let mut in_string = false;
    let mut colon_pos = None;
    
    for (i, ch) in pair.char_indices() {
        if ch == '"' && (i == 0 || pair.chars().nth(i - 1) != Some('\\')) {
            in_string = !in_string;
        }
        if ch == ':' && !in_string {
            colon_pos = Some(i);
            break;
        }
    }
    
    let colon_pos = colon_pos.ok_or_else(|| {
        FlowError::runtime(&format!("Invalid JSON object entry: {}", pair), 0, 0)
    })?;
    
    let key_str = pair[..colon_pos].trim();
    let value_str = pair[colon_pos + 1..].trim();
    
    // Parse key (must be a string)
    if !key_str.starts_with('"') || !key_str.ends_with('"') {
        return Err(FlowError::runtime(&format!("JSON object key must be a string: {}", key_str), 0, 0));
    }
    
    let key = key_str[1..key_str.len() - 1].to_string();
    let value = parse_json_value(value_str)?;
    
    map.insert(key, value);
    Ok(())
}

// Helper: Convert FlowLang Value to JSON string
pub fn value_to_json_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Number(n) => {
            if n.fract() == 0.0 {
                format!("{:.0}", n)
            } else {
                n.to_string()
            }
        }
        Value::String(s) => {
            let escaped = s
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t");
            format!("\"{}\"", escaped)
        }
        Value::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(value_to_json_string).collect();
            format!("[{}]", elements.join(","))
        }
        Value::Function { .. } => "null".to_string(), // Functions can't be serialized
        Value::NativeFunction(_) => "null".to_string(),
        Value::AsyncNativeFunction(_) => "null".to_string(),
        Value::Handle(id) => format!("{}", id), // Handles serialize as their ID number
        Value::Relic(map) => {
            let mut entries: Vec<String> = map.iter()
                .map(|(k, v)| {
                    let escaped_key = k
                        .replace('\\', "\\\\")
                        .replace('"', "\\\"")
                        .replace('\n', "\\n")
                        .replace('\r', "\\r")
                        .replace('\t', "\\t");
                    format!("\"{}\":{}", escaped_key, value_to_json_string(v))
                })
                .collect();
            entries.sort(); // Sort for deterministic output
            format!("{{{}}}", entries.join(","))
        }
    }
}
