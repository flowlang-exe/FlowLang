pub mod environment;

use environment::Environment;
use crate::error::FlowError;
use crate::parser::ast::*;
use crate::types::{Value, AsyncContext};
use crate::stdlib;
use crate::runtime::Runtime;

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::sync::Arc;

use crate::config::ProjectConfig;

pub struct Interpreter {
    env: Environment,
    module_cache: HashMap<String, Environment>,
    current_dir: PathBuf,
    current_file: String,  // Track current file for error reporting
    loading_stack: Vec<String>,  // Track module loading chain for circular dependency detection
    config: ProjectConfig,
    /// Runtime for event loop and handle management
    runtime: Arc<Runtime>,
}

impl Interpreter {
    pub fn new(config: ProjectConfig) -> Self {
        Interpreter {
            env: Environment::new(),
            module_cache: HashMap::new(),
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            current_file: "main.flow".to_string(),
            loading_stack: Vec::new(),
            config,
            runtime: Arc::new(Runtime::new()),
        }
    }
    
    pub fn with_dir(dir: PathBuf, config: ProjectConfig) -> Self {
        Interpreter {
            env: Environment::new(),
            module_cache: HashMap::new(),
            current_dir: dir,
            current_file: "module.flow".to_string(),
            loading_stack: Vec::new(),
            config,
            runtime: Arc::new(Runtime::new()),
        }
    }
    
    /// Get access to the runtime for event loop management
    pub fn runtime(&self) -> Arc<Runtime> {
        self.runtime.clone()
    }
    
    /// Execute a FlowLang function with given arguments
    /// Useful for calling FlowLang handlers from native code (e.g., web server)
    pub async fn execute_function(&mut self, func: Value, args: Vec<Value>) -> Result<Value, FlowError> {
        match func {
            Value::Function { params, body, .. } => {
                // Push new scope for function
                self.env.push_scope();
                
                // Bind parameters
                for (param, arg) in params.iter().zip(args.iter()) {
                    self.env.define(param.clone(), arg.clone(), true);
                }
                
                // Execute function body
                let mut result = Value::Null;
                for stmt in body.iter() {
                    if let Some(ret_val) = self.execute_statement(stmt).await? {
                        result = ret_val;
                        break; // Early return
                    }
                }
                
                // Pop scope
                self.env.pop_scope();
                
                Ok(result)
            }
            Value::NativeFunction(f) => {
                // Native functions are synchronous
                f.0(args)
            }
            _ => Err(FlowError::type_error("Not a function", 0, 0))
        }
    }
    
    fn check_type_compatibility(&self, value: &Value, expected: &crate::types::EssenceType) -> bool {
        use crate::types::{EssenceType, Value};
        match (value, expected) {
            (Value::Number(_), EssenceType::Ember) => true,
            (Value::String(_), EssenceType::Silk) => true,
            (Value::Boolean(_), EssenceType::Pulse) => true,
            (_, EssenceType::Flux) => true, // Flux accepts anything
            (Value::Null, EssenceType::Hollow) => true,
            (Value::Array(arr), EssenceType::Constellation(inner_type)) => {
                for item in arr.iter() {
                    if !self.check_type_compatibility(item, inner_type) {
                        return false;
                    }
                }
                true
            }
            (Value::Relic(map), EssenceType::Relic(key_type, val_type)) => {
                // Keys are always strings in JSON/Maps, so we check if key_type is Silk
                if !matches!(**key_type, EssenceType::Silk) {
                    return false; 
                }
                for val in map.values() {
                    if !self.check_type_compatibility(val, val_type) {
                        return false;
                    }
                }
                true
            }
            (Value::Function { .. } | Value::NativeFunction(_), EssenceType::Spell) => true,
            _ => false,
        }
    }

    pub fn execute<'a>(&'a mut self, program: Program) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), FlowError>> + 'a>> {
        Box::pin(async move {
            // Process imports first
            for import in program.imports {
                self.execute_import(&import).await?;
            }
            
            // Execute all statements
            for statement in program.statements {
                self.execute_statement(&statement).await?;
            }
            
            Ok(())
        })
    }
    
    pub async fn execute_import(&mut self, import: &Import) -> Result<(), FlowError> {
        // Check for std: import
        if let Some(path) = &import.from_path {
            if path.starts_with("std:") {
                let lib_name = &path[4..];
                if let Some(module_map) = stdlib::load_module(lib_name) {
                    let alias = import.alias.clone().unwrap_or(import.module.clone());
                    let relic = Value::Relic(Arc::new(module_map));
                    self.env.define(alias, relic, false);
                    return Ok(());
                } else {
                    return Err(FlowError::runtime(
                        &format!("Unknown standard library module '{}'", lib_name),
                        0, 0
                    ));
                }
            }
        }

        // 1. Resolve path
        let mut module_path = self.current_dir.clone();
        
        if let Some(path) = &import.from_path {
            module_path.push(path);
        } else {
            module_path.push(&import.module);
        }
        
        // Add .flow extension if missing
        if module_path.extension().is_none() {
            module_path.set_extension("flow");
        }
        
        let canonical_path = match fs::canonicalize(&module_path) {
            Ok(p) => p,
            Err(_) => {
                // Try relative to current dir if canonicalize fails (e.g. file doesn't exist yet)
                // But for execution, file must exist
                return Err(FlowError::runtime(
                    &format!("Cannot find circle '{}' at {}", import.module, module_path.display()),
                    0, 0
                ));
            }
        };
        
        let module_key = canonical_path.to_string_lossy().to_string();
        
        // Check for circular dependency BEFORE loading
        if self.loading_stack.contains(&module_key) {
            // Build clean import chain with just filenames
            let clean_chain: Vec<String> = self.loading_stack.iter()
                .map(|path| {
                    std::path::Path::new(path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(path)
                        .to_string()
                })
                .collect();
            
            let current_file = canonical_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&module_key);
            
            let chain_display = clean_chain.join(" → ");
            
            return Err(FlowError::runtime(
                &format!("Circular dependency detected!\nThe snake eats its own tail!\n\nImport chain: {} → {}\n\nThis circular import creates an infinite loop.", 
                    chain_display, current_file),
                0, 0
            ));
        }
        
        // 2. Check cache or load
        if !self.module_cache.contains_key(&module_key) {
            // Push to loading stack before loading
            self.loading_stack.push(module_key.clone());
            
            // Load module
            let source = fs::read_to_string(&canonical_path).map_err(|e| {
                FlowError::runtime(
                    &format!("Failed to read circle '{}': {}", import.module, e),
                    0, 0
                )
            })?;
            
            // Parse
            let tokens = crate::lexer::tokenize(&source)?;
            let ast = crate::parser::parse(tokens)?;
            
            // Execute in new interpreter
            let module_dir = canonical_path.parent().unwrap().to_path_buf();
            let mut module_interpreter = Interpreter::with_dir(module_dir, self.config.clone());
            
            // Set the current file for error reporting
            module_interpreter.current_file = canonical_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("module.flow")
                .to_string();
            
            // Pass the loading stack to the child interpreter
            module_interpreter.loading_stack = self.loading_stack.clone();
            
            // Recursively execute imports and statements
            module_interpreter.execute(ast).await?;
            
            // Cache environment
            self.module_cache.insert(module_key.clone(), module_interpreter.env);
            
            // Pop from loading stack after successful load
            self.loading_stack.pop();
        }
        
        // 3. Import symbols
        let module_env = self.module_cache.get(&module_key).unwrap();
        
        // Handle selective imports or full module import
        if let Some(selective_imports) = &import.selective {
            // Selective import mode: circle {member1, member2 as alias} from "..."
            for sel_import in selective_imports {
                // Get the member from module environment
                let member_value = module_env.get(&sel_import.name)
                    .ok_or_else(|| FlowError::runtime(
                        &format!("Module '{}' does not export '{}'", 
                            import.module, sel_import.name),
                        0, 0
                    ))?;
                
                // Use alias if provided, otherwise use original name
                let import_name = sel_import.alias.as_ref()
                    .unwrap_or(&sel_import.name);
                
                self.env.define(import_name.clone(), member_value, false);
            }
        } else {
            // Full module import: circle module from "..." or circle module as alias
            let public_vars = module_env.get_all_public();
            
            // Determine alias: use explicit alias, or module name if no alias provided
            let alias = import.alias.clone().unwrap_or(import.module.clone());
        
            // Import as object/map (Relic)
            let mut module_map = HashMap::new();
            for (name, value) in public_vars {
                module_map.insert(name, value);
            }
            
            let relic = Value::Relic(Arc::new(module_map));
            self.env.define(alias, relic, false); // Modules are sealed (immutable)
        }
        
        Ok(())
    }
    
    pub fn execute_statement<'a>(&'a mut self, stmt: &'a Statement) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Option<Value>, FlowError>> + 'a>> {
        Box::pin(async move {
        match stmt {
            Statement::Let { name, type_annotation, value, is_exported, line } => {
                let val = self.evaluate_expression(value).await?;
                
                // DEBUG
                // println!("DEBUG: Let {}, type_required: {}, has_annotation: {}", name, self.config.type_required, type_annotation.is_some());

                if let Some(expected_type) = type_annotation {
                    if !self.check_type_compatibility(&val, expected_type) {
                        return Err(FlowError::type_error(
                            &format!("Expected essence {}, but found {}!", expected_type, val.type_name()),
                            *line, 0
                        ));
                    }
                } else if self.config.type_required {
                    return Err(FlowError::type_error(
                        &format!("Type annotation required for variable '{}' in strict mode!", name),
                        *line, 0
                    ));
                }
                
                self.env.define_with_export(name.clone(), val, true, *is_exported);
                Ok(None)
            }
            
            Statement::Seal { name, type_annotation, value, is_exported, line } => {
                let val = self.evaluate_expression(value).await?;
                
                if let Some(expected_type) = type_annotation {
                    if !self.check_type_compatibility(&val, expected_type) {
                        return Err(FlowError::type_error(
                            &format!("Expected essence {}, but found {}!", expected_type, val.type_name()),
                            *line, 0
                        ));
                    }
                } else if self.config.type_required {
                    return Err(FlowError::type_error(
                        &format!("Type annotation required for sealed variable '{}' in strict mode!", name),
                        *line, 0
                    ));
                }
                
                self.env.define_with_export(name.clone(), val, false, *is_exported);
                Ok(None)
            }
            
            Statement::Assignment { name, value, line } => {
                let val = self.evaluate_expression(value).await?;
                
                // Try to update the variable
                match self.env.set(name, val.clone()) {
                    Ok(_) => Ok(None),
                    Err(_) => {
                        // Variable doesn't exist or is sealed
                        Err(FlowError::runtime(
                            &format!("Cannot assign to '{}'. Variable may not exist or is sealed (use 'seal' for immutable, 'let' for mutable).", name),
                            *line,
                            0,
                        ))
                    }
                }
            }
            
            Statement::FunctionDecl { name, params, return_type, body, sigils: _, is_exported, line } => {
                // Check strict mode for params and return type
                if self.config.type_required {
                    if return_type.is_none() {
                        return Err(FlowError::type_error(
                            &format!("Return type required for Spell '{}' in strict mode!", name),
                            *line, 0
                        ));
                    }
                    for param in params {
                        if param.type_annotation.is_none() {
                            return Err(FlowError::type_error(
                                &format!("Type annotation required for parameter '{}' in Spell '{}'!", param.name, name),
                                *line, 0
                            ));
                        }
                    }
                }

                let param_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();
                let param_types: Vec<Option<crate::types::EssenceType>> = params.iter().map(|p| p.type_annotation.clone()).collect();
                
                let func = Value::Function {
                    params: param_names,
                    param_types,
                    return_type: return_type.clone(),
                    body: Arc::new(body.clone()),
                    is_async: false,
                };
                self.env.define_with_export(name.clone(), func, false, *is_exported);
                Ok(None)
            }
            
            Statement::Ritual { name, params, return_type: _, body, is_exported, line: _ } => {
                let param_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();
                // Rituals currently don't support type checking in this implementation, 
                // but we could add it similarly. For now, filling with None.
                let param_types = vec![None; params.len()];
                
                let func = Value::Function {
                    params: param_names,
                    param_types,
                    return_type: None,
                    body: Arc::new(body.clone()),
                    is_async: true,
                };
                self.env.define_with_export(name.clone(), func, false, *is_exported);
                Ok(None)
            }
            
            Statement::Return { value, .. } => {
                let val = if let Some(expr) = value {
                    self.evaluate_expression(expr).await?
                } else {
                    Value::Null
                };
                Ok(Some(val))
            }
            
            Statement::Stance {
                condition,
                then_branch,
                shift_branches,
                abandon_branch,
                line: _,
            } => {
                let cond_value = self.evaluate_expression(condition).await?;
                
                if cond_value.is_truthy() {
                    self.env.push_scope();
                    for stmt in then_branch {
                        if let Some(ret) = self.execute_statement(stmt).await? {
                            self.env.pop_scope();
                            return Ok(Some(ret));
                        }
                    }
                    self.env.pop_scope();
                } else {
                    // Try shift branches
                    let mut executed = false;
                    for (shift_cond, shift_body) in shift_branches {
                        let shift_val = self.evaluate_expression(shift_cond).await?;
                        if shift_val.is_truthy() {
                            self.env.push_scope();
                            for stmt in shift_body {
                                if let Some(ret) = self.execute_statement(stmt).await? {
                                    self.env.pop_scope();
                                    return Ok(Some(ret));
                                }
                            }
                            self.env.pop_scope();
                            executed = true;
                            break;
                        }
                    }
                    
                    // Execute abandon branch if no shift matched
                    if !executed {
                        if let Some(abandon_body) = abandon_branch {
                            self.env.push_scope();
                            for stmt in abandon_body {
                                if let Some(ret) = self.execute_statement(stmt).await? {
                                    self.env.pop_scope();
                                    return Ok(Some(ret));
                                }
                            }
                            self.env.pop_scope();
                        }
                    }
                }
                
                Ok(None)
            }
            
            Statement::Aura { value, cases, otherwise, line: _ } => {
                let aura_value = self.evaluate_expression(value).await?;
                
                for (case_expr, case_body) in cases {
                    let case_value = self.evaluate_expression(case_expr).await?;
                    
                    // Simple equality check
                    if self.values_equal(&aura_value, &case_value) {
                        self.env.push_scope();
                        for stmt in case_body {
                            if let Some(ret) = self.execute_statement(stmt).await? {
                                self.env.pop_scope();
                                return Ok(Some(ret));
                            }
                        }
                        self.env.pop_scope();
                        return Ok(None);
                    }
                }
                
                // Execute otherwise if no case matched
                if let Some(otherwise_body) = otherwise {
                    self.env.push_scope();
                    for stmt in otherwise_body {
                        if let Some(ret) = self.execute_statement(stmt).await? {
                            self.env.pop_scope();
                            return Ok(Some(ret));
                        }
                    }
                    self.env.pop_scope();
                }
                
                Ok(None)
            }
            
            Statement::Phase { kind, body, line } => {
                match kind {
                    PhaseKind::Count { variable, from, to } => {
                        let from_val = self.evaluate_expression(from).await?;
                        let to_val = self.evaluate_expression(to).await?;
                        
                        let start = match from_val {
                            Value::Number(n) => n as i64,
                            _ => return Err(FlowError::type_error(
                                "Phase 'from' must be an Ember!",
                                *line,
                                0,
                            )),
                        };
                        
                        let end = match to_val {
                            Value::Number(n) => n as i64,
                            _ => return Err(FlowError::type_error(
                                "Phase 'to' must be an Ember!",
                                *line,
                                0,
                            )),
                        };
                        
                        for i in start..=end {
                            self.env.push_scope();
                            self.env.define(variable.clone(), Value::Number(i as f64), false);
                            
                            let mut break_loop = false;
                            
                            for stmt in body {
                                match self.execute_statement(stmt).await {
                                    Ok(Some(ret)) => {
                                        self.env.pop_scope();
                                        return Ok(Some(ret));
                                    }
                                    Ok(None) => {}
                                    Err(FlowError::Break { .. }) => {
                                        break_loop = true;
                                        break;
                                    }
                                    Err(FlowError::Continue { .. }) => {
                                        break; // Break inner statement loop, continue outer phase loop
                                    }
                                    Err(e) => {
                                        self.env.pop_scope();
                                        return Err(e);
                                    }
                                }
                            }
                            
                            self.env.pop_scope();
                            
                            if break_loop {
                                break;
                            }
                        }
                    }
                    
                    PhaseKind::ForEach { variable, collection } => {
                        let collection_val = self.evaluate_expression(collection).await?;
                        
                        match collection_val {
                            Value::Array(arr) => {
                                for item in arr.iter() {
                                    self.env.push_scope();
                                    self.env.define(variable.clone(), item.clone(), false);
                                    
                                    let mut break_loop = false;
                                    
                                    for stmt in body {
                                        match self.execute_statement(stmt).await {
                                            Ok(Some(ret)) => {
                                                self.env.pop_scope();
                                                return Ok(Some(ret));
                                            }
                                            Ok(None) => {}
                                            Err(FlowError::Break { .. }) => {
                                                break_loop = true;
                                                break;
                                            }
                                            Err(FlowError::Continue { .. }) => {
                                                break; // Break inner statement loop, continue outer phase loop
                                            }
                                            Err(e) => {
                                                self.env.pop_scope();
                                                return Err(e);
                                            }
                                        }
                                    }
                                    
                                    self.env.pop_scope();
                                    
                                    if break_loop {
                                        break;
                                    }
                                }
                            }
                            _ => return Err(FlowError::type_error(
                                "For-each loop requires a Constellation (array)!",
                                *line,
                                0,
                            )),
                        }
                    }
                    
                    PhaseKind::Until { condition } => {
                        loop {
                            let cond_val = self.evaluate_expression(condition).await?;
                            if cond_val.is_truthy() {
                                break;
                            }
                            
                            self.env.push_scope();
                            let mut break_loop = false;
                            
                            for stmt in body {
                                match self.execute_statement(stmt).await {
                                    Ok(Some(ret)) => {
                                        self.env.pop_scope();
                                        return Ok(Some(ret));
                                    }
                                    Ok(None) => {}
                                    Err(FlowError::Break { .. }) => {
                                        break_loop = true;
                                        break;
                                    }
                                    Err(FlowError::Continue { .. }) => {
                                        break; // Break inner statement loop, continue outer phase loop
                                    }
                                    Err(e) => {
                                        self.env.pop_scope();
                                        return Err(e);
                                    }
                                }
                            }
                            
                            self.env.pop_scope();
                            
                            if break_loop {
                                break;
                            }
                        }
                    }
                    
                    PhaseKind::Forever => {
                        loop {
                            self.env.push_scope();
                            let mut break_loop = false;
                            
                            for stmt in body {
                                match self.execute_statement(stmt).await {
                                    Ok(Some(ret)) => {
                                        self.env.pop_scope();
                                        return Ok(Some(ret));
                                    }
                                    Ok(None) => {}
                                    Err(FlowError::Break { .. }) => {
                                        break_loop = true;
                                        break;
                                    }
                                    Err(FlowError::Continue { .. }) => {
                                        break; // Break inner statement loop, continue outer phase loop
                                    }
                                    Err(e) => {
                                        self.env.pop_scope();
                                        return Err(e);
                                    }
                                }
                            }
                            
                            self.env.pop_scope();
                            
                            if break_loop {
                                break;
                            }
                        }
                    }
                }
                
                Ok(None)
            }
            
            Statement::Expression { expr, .. } => {
                self.evaluate_expression(expr).await?;
                Ok(None)
            }
            
            Statement::Wait { duration, unit, line } => {
                let dur_val = self.evaluate_expression(duration).await?;
                
                let ms = match dur_val {
                    Value::Number(n) => {
                        match unit.as_str() {
                            "ms" => n as u64,
                            "s" => (n * 1000.0) as u64,
                            "m" => (n * 60000.0) as u64,
                            _ => return Err(FlowError::runtime(
                                &format!("Unknown time unit '{}'", unit),
                                *line,
                                0,
                            )),
                        }
                    }
                    _ => return Err(FlowError::type_error(
                        "Wait duration must be an Ember!",
                        *line,
                        0,
                    )),
                };
                
                // Process callbacks while waiting
                let start = std::time::Instant::now();
                let wait_duration = std::time::Duration::from_millis(ms);
                
                while start.elapsed() < wait_duration {
                    // Process any pending callbacks
                    while let Some(request) = self.runtime.run_event_loop_tick().await {
                        if let Err(e) = self.execute_function(request.callback, request.args).await {
                            eprintln!("Callback error: {}", e);
                        }
                    }
                    
                    // Sleep for a short tick interval
                    let remaining = wait_duration.saturating_sub(start.elapsed());
                    let tick = std::cmp::min(remaining, std::time::Duration::from_millis(10));
                    if tick > std::time::Duration::ZERO {
                        tokio::time::sleep(tick).await;
                    }
                }
                
                Ok(None)
            }
            
            Statement::Perform { rituals, line: _ } => {
                // For parallel execution, we need to spawn tasks
                // This is a simplified implementation that awaits them concurrently
                
                for ritual_expr in rituals {
                    // We need to clone the environment for each task
                    // But Environment is not Clone/Send safe easily
                    // So for now, we'll just evaluate them sequentially but mark as TODO
                    // In a full implementation, we'd need Arc<Mutex<Environment>>
                    
                    // NOTE: True parallel execution requires significant architectural changes
                    // to make Environment thread-safe. For this version, we'll execute
                    // them sequentially but treating them as a group.
                    self.evaluate_expression(ritual_expr).await?;
                }
                
                Ok(None)
            }
            
            // ⚔️ ERROR ARC - Attempt/Rescue Implementation
            Statement::Attempt { body, rescue_clauses, finally_block, line } => {
                let mut result = Ok(None);
                let mut error_caught = false;
                
                // Execute the attempt block
                for stmt in body {
                    match self.execute_statement(stmt).await {
                        Ok(val) => result = Ok(val),
                        Err(err) => {
                            // Error occurred - try to match rescue clauses
                            error_caught = true;
                            let error_type = err.error_type_name();
                            let error_msg = err.to_string();
                            
                            let mut handled = false;
                            
                            for rescue in rescue_clauses {
                                // Check if this rescue clause matches the error type
                                let type_matches = rescue.error_type.as_ref()
                                    .map(|t| t == error_type)
                                    .unwrap_or(true); // No type specified = catch all
                                
                                if type_matches {
                                    handled = true;
                                    
                                    // Bind error to variable if specified
                                    if let Some(binding) = &rescue.binding {
                                        self.env.define(
                                            binding.clone(),
                                            Value::String(Arc::new(error_msg.clone())),
                                            true
                                        );
                                    }
                                    
                                    // Handle retry logic
                                    if let Some(retry_count) = rescue.retry_count {
                                        let mut attempts = 0;
                                        loop {
                                            attempts += 1;
                                            
                                            // Execute rescue body
                                            for rescue_stmt in &rescue.body {
                                                self.execute_statement(rescue_stmt).await?;
                                            }
                                            
                                            if attempts >= retry_count {
                                                break;
                                            }
                                            
                                            // Retry the attempt block
                                            let mut retry_success = true;
                                            for stmt in body {
                                                if let Err(_) = self.execute_statement(stmt).await {
                                                    retry_success = false;
                                                    break;
                                                }
                                            }
                                            
                                            if retry_success {
                                                break;
                                            }
                                        }
                                    } else {
                                        // No retry - just execute rescue body
                                        for rescue_stmt in &rescue.body {
                                            self.execute_statement(rescue_stmt).await?;
                                        }
                                    }
                                    
                                    result = Ok(None);
                                    break;
                                }
                            }
                            
                            if !handled {
                                // No rescue clause matched - propagate error
                                result = Err(err);
                            }
                            
                            break;
                        }
                    }
                }
                
                // Execute finally block if present
                if let Some(finally) = finally_block {
                    for stmt in finally {
                        self.execute_statement(stmt).await?;
                    }
                }
                
                result
            }
            
            Statement::Panic { message, line } => {
                let msg = self.evaluate_expression(message).await?;
                return Err(FlowError::panic(&msg.to_string(), *line, 0));
            }
            
            Statement::Rebound { error, line } => {
                // Rethrow logic
                if let Some(err_name) = error {
                    // Rethrow specific error variable
                    if let Some(val) = self.env.get(err_name) {
                        if let Value::String(msg) = val {
                            // Create a new Spirit error with the message
                            // Ideally we'd preserve the original type, but for now we wrap it
                            return Err(FlowError::spirit(&msg, *line, 0));
                        }
                    }
                    return Err(FlowError::runtime(&format!("Cannot rebound undefined error: {}", err_name), *line, 0));
                } else {
                    // Rethrow current error (not supported without context, so throw generic)
                    return Err(FlowError::spirit("Rebound!", *line, 0));
                }
            }
            
            Statement::Ward { body, line: _ } => {
                // Ward: Error containment
                // Execute body, if error occurs, suppress it and continue
                for stmt in body {
                    match self.execute_statement(stmt).await {
                        Ok(val) => {
                            if val.is_some() {
                                return Ok(val);
                            }
                        }
                        Err(err) => {
                            // Ward absorbs the error
                            eprintln!("🛡️ WARD ABSORBED ERROR: {}", err);
                            break; // Stop executing ward body, but don't propagate error
                        }
                    }
                }
                Ok(None)
            }
            
            Statement::BreakSeal { line } => {
                return Err(FlowError::break_seal(*line, 0));
            }
            
            Statement::FractureSeal { line } => {
                return Err(FlowError::fracture_seal(*line, 0));
            }
            
            Statement::ShatterGrandSeal { value, line: _ } => {
                // This is essentially a return statement
                if let Some(expr) = value {
                    let val = self.evaluate_expression(expr).await?;
                    return Ok(Some(val));
                }
                Ok(Some(Value::Null))
            }
            
            Statement::Wound { message, line: _ } => {
                let msg = self.evaluate_expression(message).await?;
                // Wound is a soft error, just print it
                let err = FlowError::wound(&msg.to_string(), 0, 0);
                crate::error::print_error(&err);
                Ok(None)
            }
            
            Statement::Rupture { error_type, message, line } => {
                let msg_val = self.evaluate_expression(message).await?;
                let msg = msg_val.to_string();
                
                match error_type.as_str() {
                    "Rift" => Err(FlowError::Rift { message: msg, line: *line, column: 0 }),
                    "Glitch" => Err(FlowError::Glitch { message: msg, line: *line, column: 0 }),
                    "VoidTear" => Err(FlowError::VoidTear { message: msg, line: *line, column: 0 }),
                    "Spirit" => Err(FlowError::Spirit { message: msg, line: *line, column: 0 }),
                    _ => Err(FlowError::runtime(&format!("Unknown error type: {}", error_type), *line, 0)),
                }
            }
            
            // Sigil type definitions (stored for type checking but don't execute)
            Statement::SigilDecl { name: _, fields: _, is_exported: _, line: _ } => {
                // Sigil definitions are stored at parse time for type checking
                // No runtime effect - they just define a type structure
                Ok(None)
            }
        }
        })
    }
    
    pub fn evaluate_expression<'a>(&'a mut self, expr: &'a Expression) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, FlowError>> + 'a>> {
        Box::pin(async move {
        match expr {
            Expression::Number(n) => Ok(Value::Number(*n)),
            Expression::String(s) => Ok(Value::String(Arc::new(s.clone()))),
            Expression::InterpolatedString(parts) => {
                let mut result = String::new();
                for part in parts {
                    let val = self.evaluate_expression(part).await?;
                    result.push_str(&val.to_string());
                }
                Ok(Value::String(Arc::new(result)))
            }
            Expression::Boolean(b) => Ok(Value::Boolean(*b)),
            
            Expression::Identifier(name) => {
                self.env.get(name).ok_or_else(|| {
                    FlowError::undefined(
                        &format!("You speak the name '{}' but no essence responds!", name),
                        0,
                        0,
                    )
                })
            }
            
            Expression::Binary { left, operator, right } => {
                let left_val = self.evaluate_expression(left).await?;
                let right_val = self.evaluate_expression(right).await?;
                
                self.apply_binary_op(&left_val, *operator, &right_val)
            }
            
            Expression::Unary { operator, operand } => {
                let val = self.evaluate_expression(operand).await?;
                
                match operator {
                    UnaryOp::Negate => match val {
                        Value::Boolean(b) => Ok(Value::Boolean(!b)),
                        _ => Err(FlowError::type_error(
                            "negate! can only be applied to Pulse essence!",
                            0,
                            0,
                        )),
                    },
                    UnaryOp::Minus => match val {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(FlowError::type_error(
                            "Minus can only be applied to Ember essence!",
                            0,
                            0,
                        )),
                    },
                }
            }
            
            Expression::Call { callee, arguments } => {
                // Evaluate arguments first
                let mut arg_values = Vec::new();
                for arg in arguments {
                    arg_values.push(self.evaluate_expression(arg).await?);
                }
                
                // Check if it's a simple identifier call (for built-ins)
                if let Expression::Identifier(name) = callee.as_ref() {
                    if stdlib::is_builtin(name) {
                        return stdlib::call_builtin(name, arg_values);
                    }
                }
                
                // Evaluate callee to get the function value
                let func_val = self.evaluate_expression(callee).await?;
                
                match func_val {
                    Value::Function { params, param_types, return_type, body, is_async: _ } => {
                        if params.len() != arg_values.len() {
                            return Err(FlowError::runtime(
                                &format!(
                                    "Spell expects {} essences, but {} were provided!",
                                    params.len(),
                                    arg_values.len()
                                ),
                                0,
                                0,
                            ));
                        }
                        
                        // Check argument types
                        for (i, (arg_val, param_type)) in arg_values.iter().zip(param_types.iter()).enumerate() {
                            if let Some(expected) = param_type {
                                if !self.check_type_compatibility(arg_val, expected) {
                                    return Err(FlowError::type_error(
                                        &format!(
                                            "Argument {} expected essence {}, but found {}!",
                                            i + 1,
                                            expected,
                                            arg_val.type_name()
                                        ),
                                        0,
                                        0,
                                    ));
                                }
                            }
                        }
                        
                        // Create new scope for function
                        self.env.push_scope();
                        
                        // Bind parameters
                        for (param, arg) in params.iter().zip(arg_values.iter()) {
                            self.env.define(param.clone(), arg.clone(), true);
                        }
                        
                        // Execute function body
                        let mut result = Value::Null;
                        for stmt in body.iter() {
                            if let Some(ret_val) = self.execute_statement(stmt).await? {
                                result = ret_val;
                                break;
                            }
                        }
                        
                        self.env.pop_scope();
                        
                        // Check return type
                        if let Some(expected_ret) = return_type {
                            if !self.check_type_compatibility(&result, &expected_ret) {
                                return Err(FlowError::type_error(
                                    &format!(
                                        "Spell expected to return essence {}, but returned {}!",
                                        expected_ret,
                                        result.type_name()
                                    ),
                                    0,
                                    0,
                                ));
                            }
                        }
                        
                        Ok(result)
                    }
                    Value::NativeFunction(func) => {
                        (func.0)(arg_values)
                    }
                    Value::AsyncNativeFunction(func) => {
                        // Create async context with runtime access
                        let ctx = AsyncContext {
                            runtime: self.runtime.clone(),
                        };
                        // Call the async native function
                        (func.0)(arg_values, ctx).await
                    }
                    _ => Err(FlowError::type_error(
                        "Can only call Spells!",
                        0,
                        0,
                    )),
                }
            }
            
            Expression::Array { elements } => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.evaluate_expression(elem).await?);
                }
                Ok(Value::Array(Arc::new(values)))
            }

            Expression::Relic { entries } => {
                let mut map = HashMap::new();
                for (key, value_expr) in entries {
                    let val = self.evaluate_expression(value_expr).await?;
                    map.insert(key.clone(), val);
                }
                Ok(Value::Relic(Arc::new(map)))
            }
            
            Expression::Index { object, index } => {
                let obj_val = self.evaluate_expression(object).await?;
                let idx_val = self.evaluate_expression(index).await?;
                
                match (obj_val, idx_val) {
                    (Value::Array(arr), Value::Number(n)) => {
                        let idx = n as usize;
                        if idx < arr.len() {
                            Ok(arr[idx].clone())
                        } else {
                            Err(FlowError::out_of_range(
                                &format!("Index {} is beyond the Constellation's bounds!", idx),
                                0,
                                0,
                            ))
                        }
                    }
                    (Value::Relic(map), Value::String(key)) => {
                        map.get(key.as_str()).cloned().ok_or_else(|| {
                            FlowError::undefined(
                                &format!("The Relic holds no secret named '{}'!", key),
                                0,
                                0,
                            )
                        })
                    }
                    _ => Err(FlowError::type_error(
                        "Invalid indexing operation!",
                        0,
                        0,
                    )),
                }
            }
            
            Expression::MethodCall { object, method, arguments } => {
                let obj_value = self.evaluate_expression(object).await?;
                
                // Evaluate arguments
                let mut arg_values = Vec::new();
                for arg in arguments {
                    arg_values.push(self.evaluate_expression(arg).await?);
                }
                
                // Dispatch based on object type
                match &obj_value {
                    Value::Array(arr) => {
                        match method.as_str() {
                            "len" => {
                                if !arg_values.is_empty() {
                                    return Err(FlowError::runtime(
                                        "Array.len() takes no arguments",
                                        0,
                                        0,
                                    ));
                                }
                                Ok(Value::Number(arr.len() as f64))
                            }
                            "push" => {
                                if arg_values.len() != 1 {
                                    return Err(FlowError::runtime(
                                        "Array.push() takes exactly 1 argument",
                                        0,
                                        0,
                                    ));
                                }
                                let mut new_arr = arr.as_ref().clone();
                                new_arr.push(arg_values[0].clone());
                                Ok(Value::Array(Arc::new(new_arr)))
                            }
                            "pop" => {
                                if !arg_values.is_empty() {
                                    return Err(FlowError::runtime(
                                        "Array.pop() takes no arguments",
                                        0,
                                        0,
                                    ));
                                }
                                if arr.is_empty() {
                                    return Err(FlowError::runtime(
                                        "Cannot pop from empty array",
                                        0,
                                        0,
                                    ));
                                }
                                Ok(arr.last().unwrap().clone())
                            }
                            "slice" => {
                                if arg_values.len() != 2 {
                                    return Err(FlowError::runtime(
                                        "Array.slice() takes exactly 2 arguments (start, end)",
                                        0,
                                        0,
                                    ));
                                }
                                let start = match &arg_values[0] {
                                    Value::Number(n) => *n as usize,
                                    _ => return Err(FlowError::type_error(
                                        "Array.slice() start index must be a number",
                                        0,
                                        0,
                                    )),
                                };
                                let end = match &arg_values[1] {
                                    Value::Number(n) => *n as usize,
                                    _ => return Err(FlowError::type_error(
                                        "Array.slice() end index must be a number",
                                        0,
                                        0,
                                    )),
                                };
                                
                                if start > arr.len() || end > arr.len() || start > end {
                                    return Err(FlowError::runtime(
                                        "Array.slice() indices out of bounds",
                                        0,
                                        0,
                                    ));
                                }
                                
                                let sliced = arr[start..end].to_vec();
                                Ok(Value::Array(Arc::new(sliced)))
                            }
                            "concat" => {
                                // concat(otherArray) - merge two Constellations
                                if arg_values.len() != 1 {
                                    return Err(FlowError::runtime(
                                        "Constellation.concat() takes exactly 1 argument",
                                        0,
                                        0,
                                    ));
                                }
                                match &arg_values[0] {
                                    Value::Array(other_arr) => {
                                        let mut new_arr = arr.as_ref().clone();
                                        new_arr.extend(other_arr.iter().cloned());
                                        Ok(Value::Array(Arc::new(new_arr)))
                                    }
                                    _ => Err(FlowError::type_error(
                                        "Constellation.concat() requires a Constellation argument",
                                        0,
                                        0,
                                    )),
                                }
                            }
                            "constellation" => {
                                // constellation(spell) - transform each element using the spell (like map)
                                if arg_values.len() != 1 {
                                    return Err(FlowError::runtime(
                                        "Constellation.constellation() takes exactly 1 argument (a Spell)",
                                        0,
                                        0,
                                    ));
                                }
                                
                                let callback = arg_values[0].clone();
                                let mut result = Vec::new();
                                
                                for item in arr.iter() {
                                    let mapped_value = match &callback {
                                        Value::Function { params, param_types: _, return_type: _, body, is_async: _ } => {
                                            if params.is_empty() {
                                                return Err(FlowError::runtime(
                                                    "Constellation.constellation() Spell must accept at least 1 parameter",
                                                    0,
                                                    0,
                                                ));
                                            }
                                            
                                            self.env.push_scope();
                                            // Bind the element to the first parameter
                                            self.env.define(params[0].clone(), item.clone(), true);
                                            
                                            let mut ret_val = Value::Null;
                                            for stmt in body.iter() {
                                                if let Some(val) = self.execute_statement(stmt).await? {
                                                    ret_val = val;
                                                    break;
                                                }
                                            }
                                            self.env.pop_scope();
                                            ret_val
                                        }
                                        Value::NativeFunction(nf) => {
                                            (nf.0)(vec![item.clone()])?
                                        }
                                        _ => {
                                            return Err(FlowError::type_error(
                                                "Constellation.constellation() requires a Spell as argument",
                                                0,
                                                0,
                                            ));
                                        }
                                    };
                                    result.push(mapped_value);
                                }
                                
                                Ok(Value::Array(Arc::new(result)))
                            }
                            "filter" => {
                                // filter(spell) - keep elements where spell returns truthy
                                if arg_values.len() != 1 {
                                    return Err(FlowError::runtime(
                                        "Constellation.filter() takes exactly 1 argument (a Spell)",
                                        0,
                                        0,
                                    ));
                                }
                                
                                let callback = arg_values[0].clone();
                                let mut result = Vec::new();
                                
                                for item in arr.iter() {
                                    let should_keep = match &callback {
                                        Value::Function { params, param_types: _, return_type: _, body, is_async: _ } => {
                                            if params.is_empty() {
                                                return Err(FlowError::runtime(
                                                    "Constellation.filter() Spell must accept at least 1 parameter",
                                                    0,
                                                    0,
                                                ));
                                            }
                                            
                                            self.env.push_scope();
                                            self.env.define(params[0].clone(), item.clone(), true);
                                            
                                            let mut ret_val = Value::Null;
                                            for stmt in body.iter() {
                                                if let Some(val) = self.execute_statement(stmt).await? {
                                                    ret_val = val;
                                                    break;
                                                }
                                            }
                                            self.env.pop_scope();
                                            ret_val.is_truthy()
                                        }
                                        Value::NativeFunction(nf) => {
                                            let result = (nf.0)(vec![item.clone()])?;
                                            result.is_truthy()
                                        }
                                        _ => {
                                            return Err(FlowError::type_error(
                                                "Constellation.filter() requires a Spell as argument",
                                                0,
                                                0,
                                            ));
                                        }
                                    };
                                    
                                    if should_keep {
                                        result.push(item.clone());
                                    }
                                }
                                
                                Ok(Value::Array(Arc::new(result)))
                            }
                            "reduce" => {
                                // reduce(spell, initialValue) - reduce to single value
                                if arg_values.len() != 2 {
                                    return Err(FlowError::runtime(
                                        "Constellation.reduce() takes exactly 2 arguments (Spell, initialValue)",
                                        0,
                                        0,
                                    ));
                                }
                                
                                let callback = arg_values[0].clone();
                                let mut accumulator = arg_values[1].clone();
                                
                                for item in arr.iter() {
                                    accumulator = match &callback {
                                        Value::Function { params, param_types: _, return_type: _, body, is_async: _ } => {
                                            if params.len() < 2 {
                                                return Err(FlowError::runtime(
                                                    "Constellation.reduce() Spell must accept 2 parameters (accumulator, element)",
                                                    0,
                                                    0,
                                                ));
                                            }
                                            
                                            self.env.push_scope();
                                            self.env.define(params[0].clone(), accumulator.clone(), true);
                                            self.env.define(params[1].clone(), item.clone(), true);
                                            
                                            let mut ret_val = Value::Null;
                                            for stmt in body.iter() {
                                                if let Some(val) = self.execute_statement(stmt).await? {
                                                    ret_val = val;
                                                    break;
                                                }
                                            }
                                            self.env.pop_scope();
                                            ret_val
                                        }
                                        Value::NativeFunction(nf) => {
                                            (nf.0)(vec![accumulator.clone(), item.clone()])?
                                        }
                                        _ => {
                                            return Err(FlowError::type_error(
                                                "Constellation.reduce() requires a Spell as first argument",
                                                0,
                                                0,
                                            ));
                                        }
                                    };
                                }
                                
                                Ok(accumulator)
                            }
                            "find" => {
                                // find(spell) - return first element where spell returns truthy
                                if arg_values.len() != 1 {
                                    return Err(FlowError::runtime(
                                        "Constellation.find() takes exactly 1 argument (a Spell)",
                                        0,
                                        0,
                                    ));
                                }
                                
                                let callback = arg_values[0].clone();
                                
                                for item in arr.iter() {
                                    let matches = match &callback {
                                        Value::Function { params, param_types: _, return_type: _, body, is_async: _ } => {
                                            if params.is_empty() {
                                                return Err(FlowError::runtime(
                                                    "Constellation.find() Spell must accept at least 1 parameter",
                                                    0,
                                                    0,
                                                ));
                                            }
                                            
                                            self.env.push_scope();
                                            self.env.define(params[0].clone(), item.clone(), true);
                                            
                                            let mut ret_val = Value::Null;
                                            for stmt in body.iter() {
                                                if let Some(val) = self.execute_statement(stmt).await? {
                                                    ret_val = val;
                                                    break;
                                                }
                                            }
                                            self.env.pop_scope();
                                            ret_val.is_truthy()
                                        }
                                        Value::NativeFunction(nf) => {
                                            let result = (nf.0)(vec![item.clone()])?;
                                            result.is_truthy()
                                        }
                                        _ => {
                                            return Err(FlowError::type_error(
                                                "Constellation.find() requires a Spell as argument",
                                                0,
                                                0,
                                            ));
                                        }
                                    };
                                    
                                    if matches {
                                        return Ok(item.clone());
                                    }
                                }
                                
                                Ok(Value::Null)
                            }
                            "every" => {
                                // every(spell) - return true if spell returns truthy for all elements
                                if arg_values.len() != 1 {
                                    return Err(FlowError::runtime(
                                        "Constellation.every() takes exactly 1 argument (a Spell)",
                                        0,
                                        0,
                                    ));
                                }
                                
                                let callback = arg_values[0].clone();
                                
                                for item in arr.iter() {
                                    let passes = match &callback {
                                        Value::Function { params, param_types: _, return_type: _, body, is_async: _ } => {
                                            if params.is_empty() {
                                                return Err(FlowError::runtime(
                                                    "Constellation.every() Spell must accept at least 1 parameter",
                                                    0,
                                                    0,
                                                ));
                                            }
                                            
                                            self.env.push_scope();
                                            self.env.define(params[0].clone(), item.clone(), true);
                                            
                                            let mut ret_val = Value::Null;
                                            for stmt in body.iter() {
                                                if let Some(val) = self.execute_statement(stmt).await? {
                                                    ret_val = val;
                                                    break;
                                                }
                                            }
                                            self.env.pop_scope();
                                            ret_val.is_truthy()
                                        }
                                        Value::NativeFunction(nf) => {
                                            let result = (nf.0)(vec![item.clone()])?;
                                            result.is_truthy()
                                        }
                                        _ => {
                                            return Err(FlowError::type_error(
                                                "Constellation.every() requires a Spell as argument",
                                                0,
                                                0,
                                            ));
                                        }
                                    };
                                    
                                    if !passes {
                                        return Ok(Value::Boolean(false));
                                    }
                                }
                                
                                Ok(Value::Boolean(true))
                            }
                            "some" => {
                                // some(spell) - return true if spell returns truthy for any element
                                if arg_values.len() != 1 {
                                    return Err(FlowError::runtime(
                                        "Constellation.some() takes exactly 1 argument (a Spell)",
                                        0,
                                        0,
                                    ));
                                }
                                
                                let callback = arg_values[0].clone();
                                
                                for item in arr.iter() {
                                    let passes = match &callback {
                                        Value::Function { params, param_types: _, return_type: _, body, is_async: _ } => {
                                            if params.is_empty() {
                                                return Err(FlowError::runtime(
                                                    "Constellation.some() Spell must accept at least 1 parameter",
                                                    0,
                                                    0,
                                                ));
                                            }
                                            
                                            self.env.push_scope();
                                            self.env.define(params[0].clone(), item.clone(), true);
                                            
                                            let mut ret_val = Value::Null;
                                            for stmt in body.iter() {
                                                if let Some(val) = self.execute_statement(stmt).await? {
                                                    ret_val = val;
                                                    break;
                                                }
                                            }
                                            self.env.pop_scope();
                                            ret_val.is_truthy()
                                        }
                                        Value::NativeFunction(nf) => {
                                            let result = (nf.0)(vec![item.clone()])?;
                                            result.is_truthy()
                                        }
                                        _ => {
                                            return Err(FlowError::type_error(
                                                "Constellation.some() requires a Spell as argument",
                                                0,
                                                0,
                                            ));
                                        }
                                    };
                                    
                                    if passes {
                                        return Ok(Value::Boolean(true));
                                    }
                                }
                                
                                Ok(Value::Boolean(false))
                            }
                            "reverse" => {
                                // reverse() - return a new reversed array
                                if !arg_values.is_empty() {
                                    return Err(FlowError::runtime(
                                        "Constellation.reverse() takes no arguments",
                                        0,
                                        0,
                                    ));
                                }
                                let mut reversed = arr.as_ref().clone();
                                reversed.reverse();
                                Ok(Value::Array(Arc::new(reversed)))
                            }
                            "join" => {
                                // join(separator) - join elements into a string
                                if arg_values.len() != 1 {
                                    return Err(FlowError::runtime(
                                        "Constellation.join() takes exactly 1 argument (separator)",
                                        0,
                                        0,
                                    ));
                                }
                                let separator = match &arg_values[0] {
                                    Value::String(s) => s.as_str().to_string(),
                                    _ => return Err(FlowError::type_error(
                                        "Constellation.join() separator must be a Silk (string)",
                                        0,
                                        0,
                                    )),
                                };
                                let joined: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                                Ok(Value::String(Arc::new(joined.join(&separator))))
                            }
                            _ => Err(FlowError::runtime(
                                &format!("Unknown method '{}' on Constellation", method),
                                0,
                                0,
                            )),
                        }
                    }
                    Value::Relic(map) => {
                        // Handle module function calls like color.cyan()
                        let func = map.get(method.as_str()).ok_or_else(|| {
                            FlowError::undefined(
                                &format!("Module has no function '{}'", method),
                                0,
                                0,
                            )
                        })?;
                        
                        // Call the function
                        match func {
                            Value::NativeFunction(nf) => {
                                (nf.0)(arg_values)
                            }
                            Value::AsyncNativeFunction(af) => {
                                // Create async context and call the async function
                                let ctx = AsyncContext {
                                    runtime: self.runtime.clone(),
                                };
                                (af.0)(arg_values, ctx).await
                            }
                            Value::Function { params, param_types: _, return_type, body, is_async: _ } => {
                                if params.len() != arg_values.len() {
                                    return Err(FlowError::runtime(
                                        &format!("Function expects {} arguments, got {}", params.len(), arg_values.len()),
                                        0,
                                        0,
                                    ));
                                }
                                
                                self.env.push_scope();
                                for (param, arg) in params.iter().zip(arg_values.iter()) {
                                    self.env.define(param.clone(), arg.clone(), true);
                                }
                                
                                let mut result = Value::Null;
                                for stmt in body.iter() {
                                    if let Some(ret_val) = self.execute_statement(stmt).await? {
                                        result = ret_val;
                                        break;
                                    }
                                }
                                self.env.pop_scope();
                                
                                if let Some(expected_ret) = return_type {
                                    if !self.check_type_compatibility(&result, &expected_ret) {
                                        return Err(FlowError::type_error(
                                            &format!("Function expected to return {}, but returned {}",
                                                expected_ret, result.type_name()),
                                            0,
                                            0,
                                        ));
                                    }
                                }
                                
                                Ok(result)
                            }
                            _ => Err(FlowError::type_error(
                                &format!("'{}' is not a function", method),
                                0,
                                0,
                            )),
                        }
                    }
                    _ => Err(FlowError::type_error(
                        &format!("Type {} has no methods", obj_value.type_name()),
                        0,
                        0,
                    )),
                }
            }
            
            Expression::Await { expr } => {
                // For now, just evaluate the expression
                // In a full implementation, this would handle async
                self.evaluate_expression(expr).await
            }
            
            Expression::ComboChain { initial, operations } => {
                let mut value = self.evaluate_expression(initial).await?;
                
                for op in operations {
                    match op {
                        ChainOperation::Call(name, args) => {
                            // Apply function to current value
                            let mut arg_values = vec![value.clone()];
                            for arg in args {
                                arg_values.push(self.evaluate_expression(arg).await?);
                            }
                            
                            if stdlib::is_builtin(name) {
                                value = stdlib::call_builtin(name, arg_values)?;
                            } else {
                                // User-defined function
                                return Err(FlowError::runtime(
                                    "Combo chains with user functions not yet implemented!",
                                    0,
                                    0,
                                ));
                            }
                        }
                        ChainOperation::Method(_name) => {
                            // Apply method to current value
                            return Err(FlowError::runtime(
                                "Combo chain methods not yet implemented!",
                                0,
                                0,
                            ));
                        }
                    }
                }
                
                Ok(value)
            }
            Expression::InlineSpell { params, body, param_types, return_type, .. } => {
                // Create a Value::Function from the inline Spell
                let body_statements = match body {
                    InlineSpellBody::Expression(expr) => {
                        // Convert expression to return statement
                        vec![Statement::Return {
                            value: Some((**expr).clone()),
                            line: 0,
                        }]
                    }
                    InlineSpellBody::Block(stmts) => stmts.clone(),
                };
                
                Ok(Value::Function {
                    params: params.clone(),
                    param_types: param_types.clone(),
                    return_type: return_type.clone(),
                    body: Arc::new(body_statements),
                    is_async: false,
                })
            }
        }
        })
    }
    
    fn apply_binary_op(&self, left: &Value, op: BinaryOp, right: &Value) -> Result<Value, FlowError> {
        match (left, op, right) {
            // Arithmetic
            (Value::Number(a), BinaryOp::Add, Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Number(a), BinaryOp::Subtract, Value::Number(b)) => Ok(Value::Number(a - b)),
            (Value::Number(a), BinaryOp::Multiply, Value::Number(b)) => Ok(Value::Number(a * b)),
            (Value::Number(a), BinaryOp::Divide, Value::Number(b)) => {
                if *b == 0.0 {
                    Err(FlowError::division_by_zero(0, 0))
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            (Value::Number(a), BinaryOp::Modulo, Value::Number(b)) => Ok(Value::Number(a % b)),
            
            // String concatenation
            (Value::String(a), BinaryOp::Add, Value::String(b)) => {
                Ok(Value::String(Arc::new(format!("{}{}", a, b))))
            }
            (Value::String(a), BinaryOp::Add, b) => {
                Ok(Value::String(Arc::new(format!("{}{}", a, b.to_string()))))
            }
            (a, BinaryOp::Add, Value::String(b)) => {
                Ok(Value::String(Arc::new(format!("{}{}", a.to_string(), b))))
            }
            
            // Comparison
            (Value::Number(a), BinaryOp::Greater, Value::Number(b)) => Ok(Value::Boolean(a > b)),
            (Value::Number(a), BinaryOp::Less, Value::Number(b)) => Ok(Value::Boolean(a < b)),
            (Value::Number(a), BinaryOp::GreaterEq, Value::Number(b)) => Ok(Value::Boolean(a >= b)),
            (Value::Number(a), BinaryOp::LessEq, Value::Number(b)) => Ok(Value::Boolean(a <= b)),
            
            // Equality
            (a, BinaryOp::IsEqual, b) => Ok(Value::Boolean(self.values_equal(a, b))),
            (a, BinaryOp::NotEqual, b) => Ok(Value::Boolean(!self.values_equal(a, b))),
            
            // Logical
            (a, BinaryOp::Both, b) => Ok(Value::Boolean(a.is_truthy() && b.is_truthy())),
            (a, BinaryOp::Either, b) => Ok(Value::Boolean(a.is_truthy() || b.is_truthy())),
            
            _ => Err(FlowError::type_error(
                &format!(
                    "Cannot apply {:?} to {} and {}",
                    op,
                    left.type_name(),
                    right.type_name()
                ),
                0,
                0,
            )),
        }
    }
    
    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::Boolean(x), Value::Boolean(y)) => x == y,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}
