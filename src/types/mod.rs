use std::collections::HashMap;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use crate::error::FlowError;

pub struct NativeFn(pub Arc<dyn Fn(Vec<Value>) -> Result<Value, FlowError> + Send + Sync>);

impl NativeFn {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(Vec<Value>) -> Result<Value, FlowError> + Send + Sync + 'static,
    {
        NativeFn(Arc::new(f))
    }
}

impl std::fmt::Debug for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}

impl PartialEq for NativeFn {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Clone for NativeFn {
    fn clone(&self) -> Self {
        NativeFn(self.0.clone())
    }
}

/// Async native function type for functions that need async runtime access
/// (e.g., timers, servers, async I/O)
pub struct AsyncNativeFn(
    pub Arc<dyn Fn(Vec<Value>, AsyncContext) -> Pin<Box<dyn Future<Output = Result<Value, FlowError>> + Send>> + Send + Sync>
);

/// Context passed to async native functions for runtime access
#[derive(Clone)]
pub struct AsyncContext {
    pub runtime: Arc<crate::runtime::Runtime>,
}

impl AsyncNativeFn {
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: Fn(Vec<Value>, AsyncContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Value, FlowError>> + Send + 'static,
    {
        AsyncNativeFn(Arc::new(move |args, ctx| Box::pin(f(args, ctx))))
    }
}

impl std::fmt::Debug for AsyncNativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<async native fn>")
    }
}

impl PartialEq for AsyncNativeFn {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Clone for AsyncNativeFn {
    fn clone(&self) -> Self {
        AsyncNativeFn(self.0.clone())
    }
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EssenceType {
    Ember,              // number
    Silk,               // string
    Pulse,              // boolean
    Flux,               // any
    Hollow,             // void
    Constellation(Box<EssenceType>), // array
    Relic(Box<EssenceType>, Box<EssenceType>), // map
    Spell,              // function
}

impl std::fmt::Display for EssenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EssenceType::Ember => write!(f, "Ember"),
            EssenceType::Silk => write!(f, "Silk"),
            EssenceType::Pulse => write!(f, "Pulse"),
            EssenceType::Flux => write!(f, "Flux"),
            EssenceType::Hollow => write!(f, "Hollow"),
            EssenceType::Constellation(inner) => write!(f, "Constellation<{}>", inner),
            EssenceType::Relic(k, v) => write!(f, "Relic<{}, {}>", k, v),
            EssenceType::Spell => write!(f, "Spell"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(Arc<String>),
    Boolean(bool),
    Array(Arc<Vec<Value>>),
    Relic(Arc<HashMap<String, Value>>),
    Null,
    Function {
        params: Vec<String>,
        param_types: Vec<Option<EssenceType>>,
        return_type: Option<EssenceType>,
        body: Arc<Vec<crate::parser::ast::Statement>>,
        is_async: bool,
    },
    NativeFunction(NativeFn),
    /// Async native function that has access to the runtime
    AsyncNativeFunction(AsyncNativeFn),
    /// Handle ID returned by timer.interval, server.http, etc.
    Handle(u64),
}

impl Value {
    pub fn type_name(&self) -> &str {
        match self {
            Value::Number(_) => "Ember",
            Value::String(_) => "Silk",
            Value::Boolean(_) => "Pulse",
            Value::Array(_) => "Constellation",
            Value::Relic(_) => "Relic",
            Value::Null => "Hollow",
            Value::Function { .. } | Value::NativeFunction(_) | Value::AsyncNativeFunction(_) => "Spell",
            Value::Handle(_) => "Handle",
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Relic(m) => !m.is_empty(),
            Value::Function { .. } | Value::NativeFunction(_) | Value::AsyncNativeFunction(_) => true,
            Value::Handle(id) => *id > 0,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => (**s).clone(),
            Value::Boolean(b) => b.to_string(),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Value::Relic(map) => {
                let mut entries: Vec<String> = map.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                entries.sort(); // Sort for deterministic output
                format!("{{ {} }}", entries.join(", "))
            }
            Value::Null => "null".to_string(),
            Value::Function { params, .. } => {
                format!("Spell({})", params.join(", "))
            }
            Value::NativeFunction(_) | Value::AsyncNativeFunction(_) => "Spell(native)".to_string(),
            Value::Handle(id) => format!("Handle(#{})", id),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
