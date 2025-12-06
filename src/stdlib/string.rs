use crate::types::{Value, NativeFn};
use crate::error::FlowError;
use std::collections::HashMap;
use std::sync::Arc;

pub fn get_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();
    
    module.insert("len".to_string(), Value::NativeFunction(NativeFn(Arc::new(|args| {
        if args.len() != 1 {
            return Err(FlowError::runtime("len() expects 1 argument", 0, 0));
        }
        match &args[0] {
            Value::String(s) => Ok(Value::Number(s.len() as f64)),
            _ => Err(FlowError::type_error("len() expects a Silk", 0, 0)),
        }
    }))));
    
    module.insert("upper".to_string(), Value::NativeFunction(NativeFn(Arc::new(|args| {
        if args.len() != 1 {
            return Err(FlowError::runtime("upper() expects 1 argument", 0, 0));
        }
        match &args[0] {
            Value::String(s) => Ok(Value::String(Arc::new(s.to_uppercase()))),
            _ => Err(FlowError::type_error("upper() expects a Silk", 0, 0)),
        }
    }))));
    
    module.insert("lower".to_string(), Value::NativeFunction(NativeFn(Arc::new(|args| {
        if args.len() != 1 {
            return Err(FlowError::runtime("lower() expects 1 argument", 0, 0));
        }
        match &args[0] {
            Value::String(s) => Ok(Value::String(Arc::new(s.to_lowercase()))),
            _ => Err(FlowError::type_error("lower() expects a Silk", 0, 0)),
        }
    }))));
    
    module.insert("trim".to_string(), Value::NativeFunction(NativeFn(Arc::new(|args| {
        if args.len() != 1 {
            return Err(FlowError::runtime("trim() expects 1 argument", 0, 0));
        }
        match &args[0] {
            Value::String(s) => Ok(Value::String(Arc::new(s.trim().to_string()))),
            _ => Err(FlowError::type_error("trim() expects a Silk", 0, 0)),
        }
    }))));
    
    module.insert("contains".to_string(), Value::NativeFunction(NativeFn(Arc::new(|args| {
        if args.len() != 2 {
            return Err(FlowError::runtime("contains() expects 2 arguments", 0, 0));
        }
        match (&args[0], &args[1]) {
            (Value::String(s), Value::String(sub)) => Ok(Value::Boolean(s.contains(sub.as_str()))),
            _ => Err(FlowError::type_error("contains() expects Silks", 0, 0)),
        }
    }))));
    
    module
}
