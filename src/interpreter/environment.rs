use std::collections::HashMap;
use crate::types::Value;
use crate::error::FlowError;

#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, (Value, bool, bool)>>, // (value, is_mutable, is_exported)
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }
    
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
    
    pub fn define(&mut self, name: String, value: Value, is_mutable: bool) {
        // Default: not exported unless explicitly marked
        self.define_with_export(name, value, is_mutable, false);
    }
    
    pub fn define_with_export(&mut self, name: String, value: Value, is_mutable: bool, is_exported: bool) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, (value, is_mutable, is_exported));
        }
    }
    
    pub fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some((value, _, _)) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }
    
    pub fn set(&mut self, name: &str, value: Value) -> Result<(), FlowError> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some((old_value, is_mutable, _)) = scope.get_mut(name) {
                if !*is_mutable {
                    return Err(FlowError::runtime(
                        &format!("Cannot reassign sealed essence '{}'! It is bound eternally.", name),
                        0,
                        0,
                    ));
                }
                *old_value = value;
                return Ok(());
            }
        }
        
        Err(FlowError::undefined(
            &format!("You speak the name '{}' but no essence responds!", name),
            0,
            0,
        ))
    }
    
    pub fn get_all_public(&self) -> HashMap<String, Value> {
        let mut public_vars = HashMap::new();
        
        // Only export from the global scope (index 0)
        if let Some(global_scope) = self.scopes.first() {
            for (name, (value, _, is_exported)) in global_scope {
                // Only include exported members
                if *is_exported {
                    public_vars.insert(name.clone(), value.clone());
                }
            }
        }
        
        public_vars
    }
    
    // For backward compatibility: get all members (used for modules without @export)
    pub fn get_all_members(&self) -> HashMap<String, Value> {
        let mut all_vars = HashMap::new();
        
        // Only export from the global scope (index 0)
        if let Some(global_scope) = self.scopes.first() {
            for (name, (value, _, _)) in global_scope {
                all_vars.insert(name.clone(), value.clone());
            }
        }
        
        all_vars
    }
    
    /// Get the current scope depth (0 = only global scope)
    pub fn scope_depth(&self) -> usize {
        self.scopes.len().saturating_sub(1)
    }
    
    /// Get all visible variables from all scopes (for closure capture)
    pub fn get_all_visible(&self) -> HashMap<String, Value> {
        let mut visible = HashMap::new();
        
        // Iterate from outer to inner scope so inner values override outer
        for scope in self.scopes.iter() {
            for (name, (value, _, _)) in scope {
                visible.insert(name.clone(), value.clone());
            }
        }
        
        visible
    }
}
