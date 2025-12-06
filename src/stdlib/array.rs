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
            Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
            _ => Err(FlowError::type_error("len() expects a Constellation", 0, 0)),
        }
    }))));
    
    module.insert("contains".to_string(), Value::NativeFunction(NativeFn(Arc::new(|args| {
        if args.len() != 2 {
            return Err(FlowError::runtime("contains() expects 2 arguments", 0, 0));
        }
        match &args[0] {
            Value::Array(arr) => {
                let target = &args[1];
                // We need a way to check equality. Value::values_equal is private in Interpreter.
                // We should probably move equality check to Value implementation or make it public.
                // For now, let's implement simple equality here or use to_string comparison as fallback.
                let found = arr.iter().any(|v| v.to_string() == target.to_string());
                Ok(Value::Boolean(found))
            },
            _ => Err(FlowError::type_error("contains() expects a Constellation", 0, 0)),
        }
    }))));
    
    // Note: push/pop mutate the array. But Value::Array contains Rc<Vec<Value>>.
    // We can't mutate Rc contents unless we use RefCell, which we don't.
    // So push/pop must return a NEW array.
    
    module.insert("push".to_string(), Value::NativeFunction(NativeFn(Arc::new(|args| {
        if args.len() != 2 {
            return Err(FlowError::runtime("push() expects 2 arguments", 0, 0));
        }
        match &args[0] {
            Value::Array(arr) => {
                let mut new_arr = (**arr).clone();
                new_arr.push(args[1].clone());
                Ok(Value::Array(Arc::new(new_arr)))
            },
            _ => Err(FlowError::type_error("push() expects a Constellation", 0, 0)),
        }
    }))));
    
    module.insert("pop".to_string(), Value::NativeFunction(NativeFn(Arc::new(|args| {
        if args.len() != 1 {
            return Err(FlowError::runtime("pop() expects 1 argument", 0, 0));
        }
        match &args[0] {
            Value::Array(arr) => {
                let mut new_arr = (**arr).clone();
                new_arr.pop();
                Ok(Value::Array(Arc::new(new_arr)))
            },
            _ => Err(FlowError::type_error("pop() expects a Constellation", 0, 0)),
        }
    }))));
    
    module
}
