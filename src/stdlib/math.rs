use crate::types::{Value, NativeFn};
use crate::error::FlowError;
use std::collections::HashMap;
use std::sync::Arc;

pub fn get_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();
    
    // Constants
    module.insert("PI".to_string(), Value::Number(std::f64::consts::PI));
    module.insert("E".to_string(), Value::Number(std::f64::consts::E));
    
    // Functions
    module.insert("sin".to_string(), create_math_fn("sin", |n| n.sin()));
    module.insert("cos".to_string(), create_math_fn("cos", |n| n.cos()));
    module.insert("tan".to_string(), create_math_fn("tan", |n| n.tan()));
    module.insert("sqrt".to_string(), create_math_fn("sqrt", |n| n.sqrt()));
    module.insert("abs".to_string(), create_math_fn("abs", |n| n.abs()));
    module.insert("round".to_string(), create_math_fn("round", |n| n.round()));
    module.insert("floor".to_string(), create_math_fn("floor", |n| n.floor()));
    module.insert("ceil".to_string(), create_math_fn("ceil", |n| n.ceil()));
    
    module.insert("min".to_string(), Value::NativeFunction(NativeFn(Arc::new(|args| {
        if args.len() != 2 {
            return Err(FlowError::runtime("min() expects 2 arguments", 0, 0));
        }
        match (&args[0], &args[1]) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.min(*b))),
            _ => Err(FlowError::type_error("min() expects Embers", 0, 0)),
        }
    }))));
    
    module.insert("max".to_string(), Value::NativeFunction(NativeFn(Arc::new(|args| {
        if args.len() != 2 {
            return Err(FlowError::runtime("max() expects 2 arguments", 0, 0));
        }
        match (&args[0], &args[1]) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.max(*b))),
            _ => Err(FlowError::type_error("max() expects Embers", 0, 0)),
        }
    }))));
    
    module.insert("pow".to_string(), Value::NativeFunction(NativeFn(Arc::new(|args| {
        if args.len() != 2 {
            return Err(FlowError::runtime("pow() expects 2 arguments", 0, 0));
        }
        match (&args[0], &args[1]) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(*b))),
            _ => Err(FlowError::type_error("pow() expects Embers", 0, 0)),
        }
    }))));
    
    module
}

fn create_math_fn(name: &str, op: fn(f64) -> f64) -> Value {
    let name = name.to_string();
    Value::NativeFunction(NativeFn(Arc::new(move |args| {
        if args.len() != 1 {
            return Err(FlowError::runtime(&format!("{}() expects 1 argument", name), 0, 0));
        }
        match &args[0] {
            Value::Number(n) => Ok(Value::Number(op(*n))),
            _ => Err(FlowError::type_error(&format!("{}() expects an Ember", name), 0, 0)),
        }
    })))
}
