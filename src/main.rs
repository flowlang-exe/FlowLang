mod lexer;
mod parser;
mod interpreter;
mod error;
mod types;
mod stdlib;
mod repl;
mod config;
mod cache;
mod optimizer;
mod runtime;
mod package_manager;

use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "flowlang")]
#[command(about = "FlowLang - A mystical anime-themed scripting language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Enable verbose output for debugging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a FlowLang script file
    Run {
        /// Path to the .flow file (optional if config.flowlang.json exists)
        file: Option<PathBuf>,
        
        /// Enable stack trace display on errors
        #[arg(long)]
        trace: bool,
        
        /// Maximum depth for stack trace (default: 50)
        #[arg(long, default_value_t = 50)]
        trace_depth: usize,
        
        /// Show raw stack trace output
        #[arg(long)]
        trace_raw: bool,
        
        /// Arguments to pass to the script
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Run the FlowLang REPL
    Repl,
    /// Developer commands for debugging
    #[command(subcommand)]
    Dev(DevCommands),
    /// Initialize a new FlowLang project
    Init {
        /// Name of the project (defaults to current directory name)
        #[arg(default_value = ".")]
        name: String,
    },
}

#[derive(Subcommand)]
enum DevCommands {
    /// Show lexer tokens for a file
    Lex {
        /// Path to the .flow file
        file: PathBuf,
    },
    /// Show parser AST for a file
    Parse {
        /// Path to the .flow file
        file: PathBuf,
    },
    /// Show detailed AST structure
    Ast {
        /// Path to the .flow file
        file: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let verbose = cli.verbose;
    
    match cli.command {
        Some(Commands::Run { file, trace, trace_depth, trace_raw, args }) => {
            let (file_path, project_config) = match file {
                Some(path) => {
                    // Try to load config if it exists in current dir, otherwise default
                    let config_path = PathBuf::from("config.flowlang.json");
                    let config = if config_path.exists() {
                        config::ProjectConfig::load(&config_path).unwrap_or_default()
                    } else {
                        config::ProjectConfig::default()
                    };
                    (path, config)
                },
                None => {
                    // Look for config file
                    let config_path = PathBuf::from("config.flowlang.json");
                    if config_path.exists() {
                        match config::ProjectConfig::load(&config_path) {
                            Ok(config) => (PathBuf::from(config.entry.clone()), config),
                            Err(e) => {
                                error::print_error(&e);
                                return;
                            }
                        }
                    } else {
                        eprintln!("{}", "❌ No file specified and no config.flowlang.json found.".red().bold());
                        eprintln!("   Usage: flowlang run <file>");
                        eprintln!("   Or run inside a project initialized with 'flowlang init'");
                        return;
                    }
                }
            };
            
            // Set script arguments in environment for cli.args() to access
            std::env::set_var("FLOWLANG_SCRIPT_ARGS", args.join("\x1F")); // Use unit separator
            
            run_file(file_path, project_config, verbose, trace, trace_depth, trace_raw).await;
        }
        Some(Commands::Repl) => {
            repl::run().await;
        }
        Some(Commands::Dev(dev_cmd)) => {
            match dev_cmd {
                DevCommands::Lex { file } => {
                    dev_lex(file).await;
                }
                DevCommands::Parse { file } => {
                    dev_parse(file).await;
                }
                DevCommands::Ast { file } => {
                    dev_ast(file).await;
                }
            }
        }
        Some(Commands::Init { name }) => {
            run_init(name).await;
        }
        None => {
            print_banner();
            println!("{}", "Use --help to see available commands".yellow());
        }
    }
}

async fn run_init(name: String) {
    use std::path::Path;
    
    let (project_name, project_path) = if name == "." {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let dir_name = current_dir.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "my-flow-project".to_string());
        (dir_name, current_dir)
    } else {
        (name.clone(), PathBuf::from(&name))
    };
    
    println!("{}", "✨ Initializing new FlowLang project...".bright_cyan().bold());
    
    // Create directory if it doesn't exist
    if !project_path.exists() {
        if let Err(e) = fs::create_dir_all(&project_path) {
            eprintln!("{} {}", "❌ Failed to create directory:".red().bold(), e);
            return;
        }
        println!("{} {}", "📂 Created directory:".green(), project_path.display());
    }
    
    // Create src directory
    let src_path = project_path.join("src");
    if !src_path.exists() {
        if let Err(e) = fs::create_dir(&src_path) {
            eprintln!("{} {}", "❌ Failed to create src directory:".red().bold(), e);
            return;
        }
        println!("{} {}", "📂 Created directory:".green(), src_path.display());
    }
    
    // Create config file
    let config = config::ProjectConfig::new(&project_name);
    let config_path = project_path.join("config.flowlang.json");
    
    if !config_path.exists() {
        if let Err(e) = config.save(&config_path) {
            error::print_error(&e);
            return;
        }
        println!("{} {}", "📝 Created config:".green(), config_path.display());
    } else {
        println!("{} {}", "⚠️  Config file already exists:".yellow(), config_path.display());
    }
    
    // Create main.flow
    let main_flow_path = src_path.join("main.flow");
    if !main_flow_path.exists() {
        let main_content = r#"-- Welcome to FlowLang!
-- This is your entry point.

shout("✨ The Flow has begun!")

cast Spell greet(name) {
    return "Hello, " + name + "!"
}

shout(greet("World"))
"#;
        if let Err(e) = fs::write(&main_flow_path, main_content) {
            eprintln!("{} {}", "❌ Failed to create main.flow:".red().bold(), e);
            return;
        }
        println!("{} {}", "📜 Created file:".green(), main_flow_path.display());
    }
    
    // Create .gitignore
    let gitignore_path = project_path.join(".gitignore");
    if !gitignore_path.exists() {
        let gitignore_content = "target/\n.gemini/\n";
        if let Err(e) = fs::write(&gitignore_path, gitignore_content) {
            eprintln!("{} {}", "⚠️  Failed to create .gitignore:".yellow(), e);
        } else {
            println!("{} {}", "🙈 Created .gitignore".green(), gitignore_path.display());
        }
    }
    
    println!();
    println!("{}", "🎉 Project initialized successfully!".bright_green().bold());
    println!("   cd {}", project_path.display());
    println!("   flowlang run src/main.flow");
}

fn print_banner() {
    println!("{}", "╔═══════════════════════════════════════╗".bright_magenta());
    println!("{}", "║     🌌 FLOWLANG VM v1.0 🌌          ║".bright_magenta());
    println!("{}", "║  A Mystical Anime Scripting Language ║".bright_magenta());
    println!("{}", "╚═══════════════════════════════════════╝".bright_magenta());
    println!();
}

async fn run_file(path: PathBuf, config: config::ProjectConfig, verbose: bool, trace: bool, trace_depth: usize, trace_raw: bool) {
    use std::time::Instant;
    
    let start_time = Instant::now();
    
    // Create trace options
    let trace_options = error::TraceOptions {
        enabled: trace,
        max_depth: trace_depth,
        raw_mode: trace_raw,
        compact: error::get_terminal_width() < 60,
    };
    
    if verbose {
        println!("{}", "═══ VERBOSE MODE ═══".bright_yellow().bold());
        println!("{} {}", "📂 Reading file:".bright_cyan(), path.display());
        if config.type_required {
            println!("{}", "🔒 Strict Mode: ENABLED".bright_magenta().bold());
        }
    }
    
    // Read the source file
    let source = match fs::read_to_string(&path) {
        Ok(content) => {
            // Strip BOM if present
            let content = content.replace("\u{feff}", "");
            
            if verbose {
                println!("{} {} bytes", "✓ File read:".green(), content.len());
            }
            content
        }
        Err(e) => {
            eprintln!("{} {}", "❌ Failed to read file:".red().bold(), e);
            return;
        }
    };
    
    // Try to load from cache
    let cache_manager = cache::CacheManager::new();
    let mut ast = None;
    
    if let Some(cached_ast) = cache_manager.load(&path, &source) {
        if verbose {
            println!("{}", "⚡ AST loaded from cache!".bright_green());
        }
        ast = Some(cached_ast);
    }
    
    if ast.is_none() {
        if verbose {
            println!("\n{}", "🔤 Lexical Analysis...".bright_cyan());
        }
        
        let lex_start = Instant::now();
        
        // Lexical analysis
        let tokens = match lexer::tokenize(&source) {
            Ok(tokens) => {
                if verbose {
                    let lex_time = lex_start.elapsed();
                    println!("{} {} tokens generated ({:.2}ms)", 
                        "✓ Tokenization complete:".green(), 
                        tokens.len(),
                        lex_time.as_secs_f64() * 1000.0
                    );
                }
                tokens
            }
            Err(e) => {
                error::print_error_with_episode(&e, trace, &trace_options, path.file_name().and_then(|n| n.to_str()));
                return;
            }
        };
        
        if verbose {
            println!("\n{}", "🌳 Parsing...".bright_cyan());
        }
        
        let parse_start = Instant::now();
        
        // Parsing
        match parser::parse(tokens) {
            Ok(parsed_ast) => {
                if verbose {
                    let parse_time = parse_start.elapsed();
                    println!("{} {} imports, {} statements ({:.2}ms)", 
                        "✓ Parsing complete:".green(), 
                        parsed_ast.imports.len(), 
                        parsed_ast.statements.len(),
                        parse_time.as_secs_f64() * 1000.0
                    );
                }
                
                // Save to cache
                if let Err(e) = cache_manager.save(&path, &source, &parsed_ast) {
                    if verbose {
                        eprintln!("{} {}", "⚠️ Failed to save AST cache:".yellow(), e);
                    }
                } else if verbose {
                    println!("{}", "💾 AST saved to cache".bright_green());
                }
                
                ast = Some(parsed_ast);
            }
            Err(e) => {
                error::print_error_with_episode(&e, trace, &trace_options, path.file_name().and_then(|n| n.to_str()));
                return;
            }
        }
    }
    
    let mut ast = ast.unwrap(); // Safe because we handled errors above
    
    // Phase 2: Optimization
    if verbose {
        println!("\n{}", "🔧 Optimizing...".bright_cyan());
    }
    
    let opt_start = Instant::now();
    let optimizer = optimizer::Optimizer::new();
    ast = optimizer.optimize(ast);
    
    if verbose {
        let opt_time = opt_start.elapsed();
        println!("{} ({:.2}ms)", 
            "✓ Optimization complete".green(),
            opt_time.as_secs_f64() * 1000.0
        );
    }
    
    if verbose {
        println!("\n{}", "⚡ Executing...".bright_cyan());
        println!("{}", "─".repeat(50).dimmed());
    }
    
    let exec_start = Instant::now();
    
    // Interpretation
    let script_dir = path.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();
    let mut interpreter = interpreter::Interpreter::with_dir(script_dir, config);
    
    if let Err(e) = interpreter.execute(ast).await {
        let filename = path.file_name().and_then(|n| n.to_str());
        error::print_error_with_episode(&e, trace, &trace_options, filename);
        return;
    }
    
    let exec_time = exec_start.elapsed();

    // ═══════════════════════════════════════════════════════════════
// 🚀 SIMPLE PIPELINED APPROACH - No Spawn Needed
// 
// Instead of spawning tasks, we process requests in a pipelined fashion:
// - Fetch multiple requests at once
// - Process them sequentially but quickly
// - This reduces idle time between requests
// ═══════════════════════════════════════════════════════════════

// Event loop: keep running while there are active handles
let runtime = interpreter.runtime();
let handle_count = runtime.active_handle_count().await;

if handle_count > 0 {
    if verbose {
        println!("{}", "─".repeat(50).dimmed());
        println!("{} {} active handle(s)", 
            "🔄 Event loop starting:".bright_cyan().bold(),
            handle_count
        );
    }
    
    // Set up Ctrl+C handler
    let shutdown_signal = runtime.shutdown_signal();
    tokio::spawn(async move {
        if let Ok(()) = tokio::signal::ctrl_c().await {
            shutdown_signal.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    });
    
    // Batch processing for better throughput
    let batch_size = 10; // Process up to 10 requests per tick
    let mut total_requests = 0u64;
    
    if verbose {
        println!("{} Batch processing (up to {} per tick)", 
            "🔧 Mode:".bright_cyan().bold(),
            batch_size
        );
    }
    
    // Main event loop
    loop {
        // Check for shutdown signal
        if runtime.is_shutdown_signaled() {
            if verbose {
                println!("{}", "\n⚡ Shutdown signal received".yellow());
            }
            break;
        }
        
        // Check handle count
        let count = runtime.active_handle_count().await;
        if count == 0 {
            if verbose {
                println!("{}", "✨ All handles closed".bright_green());
            }
            break;
        }
        
        // Process pending timer callbacks (fire-and-forget)
        while let Some(request) = runtime.run_event_loop_tick().await {
            if let Err(e) = interpreter.execute_function(request.callback, request.args).await {
                eprintln!("{} {}", "⚠️ Callback error:".yellow(), e);
            }
        }
        
        // Process web callbacks concurrently
        let semaphore = runtime.web_handler_semaphore();
        
        // Loop until there are no more requests OR we hit a reasonable batch size per tick
        // But since we are spawning tasks, we can process as many as the channel gives us
        // limited by the semaphore if we want to throttle spawning (or the tasks throttle themselves)
        let mut loop_batch = 0;
        
        while loop_batch < batch_size {
            // Check if we have permit for new handler
            // If semaphore is closed/zero, we skip this tick to handle other events
            if semaphore.available_permits() > 0 {
                match runtime.get_web_callback().await {
                    Some(web_request) => {
                        // Clone interpreter for this task
                        // This uses our new Arc<Mutex> shared state for modules
                        let mut task_interpreter = interpreter.clone();
                        let permit = semaphore.clone().acquire_owned().await.unwrap();
                        
                        tokio::spawn(async move {
                            // The permit is held for the duration of this block
                            let _permit = permit;
                            
                            // Execute the handler
                            let result = match task_interpreter.execute_function(
                                web_request.callback, 
                                web_request.args
                            ).await {
                                Ok(value) => value,
                                Err(e) => {
                                    eprintln!("{} {}", "⚠️ Web handler error:".yellow(), e);
                                    crate::types::Value::String(std::sync::Arc::new(format!("Error: {}", e)))
                                }
                            };
                            
                            // Send response back
                            let _ = web_request.response_tx.send(result);
                        });
                        
                        total_requests += 1;
                        loop_batch += 1;
                    }
                    None => {
                        break;
                    }
                }
            } else {
                // Max concurrent handlers reached, wait for next tick
                break;
            }
        }
        
        // Brief sleep only if we didn't process a full batch
        if loop_batch == 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        } else {
            // Yield to allow other tasks to run
            tokio::task::yield_now().await;
        }
    }
    
    if verbose {
        println!("{}", "🏁 Event loop ended".bright_cyan());
        if total_requests > 0 {
            println!("{} {} web requests processed", 
                "📊 Total:".bright_green(),
                total_requests
            );
        }
    }
}

let total_time = start_time.elapsed();

if verbose {
    println!("{}", "─".repeat(50).dimmed());
    println!("{}", "✅ Execution completed successfully".bright_green().bold());
    println!("\n{}", "⏱️  Timing:".bright_yellow());
    println!("   Execution: {:.2}ms", exec_time.as_secs_f64() * 1000.0);
    println!("   Total:     {:.2}ms", total_time.as_secs_f64() * 1000.0);
}
}

async fn dev_lex(path: PathBuf) {
    println!("{}", "🔤 LEXER OUTPUT".bright_yellow().bold());
    println!("{}", "═".repeat(60).yellow());
    
    let source = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}", "❌ Failed to read file:".red().bold(), e);
            return;
        }
    };
    
    match lexer::tokenize(&source) {
        Ok(tokens) => {
            println!("{} {} tokens\n", "Total:".bright_cyan(), tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                println!("{:4} | {}:{:<3} | {:?}", 
                    i, 
                    token.line, 
                    token.column,
                    token.kind
                );
            }
        }
        Err(e) => {
            error::print_error(&e);
        }
    }
}

async fn dev_parse(path: PathBuf) {
    println!("{}", "🌳 PARSER OUTPUT".bright_yellow().bold());
    println!("{}", "═".repeat(60).yellow());
    
    let source = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}", "❌ Failed to read file:".red().bold(), e);
            return;
        }
    };
    
    let tokens = match lexer::tokenize(&source) {
        Ok(tokens) => tokens,
        Err(e) => {
            error::print_error(&e);
            return;
        }
    };
    
    match parser::parse(tokens) {
        Ok(ast) => {
            println!("{} {} imports", "Imports:".bright_cyan(), ast.imports.len());
            for import in &ast.imports {
                println!("  - {} {:?}", "circle".bright_magenta(), import.module);
            }
            
            println!("\n{} {} statements", "Statements:".bright_cyan(), ast.statements.len());
            for (i, stmt) in ast.statements.iter().enumerate() {
                println!("  {:2}. {:?}", i + 1, stmt);
            }
        }
        Err(e) => {
            error::print_error(&e);
        }
    }
}

async fn dev_ast(path: PathBuf) {
    println!("{}", "🌲 DETAILED AST".bright_yellow().bold());
    println!("{}", "═".repeat(60).yellow());
    
    let source = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}", "❌ Failed to read file:".red().bold(), e);
            return;
        }
    };
    
    let tokens = match lexer::tokenize(&source) {
        Ok(tokens) => tokens,
        Err(e) => {
            error::print_error(&e);
            return;
        }
    };
    
    match parser::parse(tokens) {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(e) => {
            error::print_error(&e);
        }
    }
}
