use std::io::{self, Write};
use colored::*;
use crate::interpreter::Interpreter;
use crate::parser::ast::Statement;

pub async fn run() {
    println!("{}", "🌊 FlowLang REPL".cyan().bold());
    println!("{}", "Type 'exit' to quit.".black().italic());

    let mut interpreter = Interpreter::with_dir(
        std::env::current_dir().unwrap(),
        crate::config::ProjectConfig::default()
    );

    loop {
        print!("{}", "flow> ".green().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input == "exit" {
            break;
        }
        if input.is_empty() {
            continue;
        }

        // Tokenize
        let tokens = match crate::lexer::tokenize(input) {
            Ok(t) => t,
            Err(e) => {
                crate::error::print_error(&e);
                continue;
            }
        };

        // Parse
        let program = match crate::parser::parse(tokens) {
            Ok(p) => p,
            Err(e) => {
                crate::error::print_error(&e);
                continue;
            }
        };

        // Execute
        let mut statements = program.statements;
        
        // Handle imports first
        for import in program.imports {
             if let Err(e) = interpreter.execute_import(&import).await {
                 crate::error::print_error(&e);
             }
        }

        if let Some(last_stmt) = statements.pop() {
            // Execute all preceding statements
            for stmt in statements {
                if let Err(e) = interpreter.execute_statement(&stmt).await {
                    crate::error::print_error(&e);
                }
            }

            // Handle last statement
            match last_stmt {
                Statement::Expression { expr, .. } => {
                    match interpreter.evaluate_expression(&expr).await {
                        Ok(val) => println!("{}", val.to_string().yellow()),
                        Err(e) => crate::error::print_error(&e),
                    }
                }
                _ => {
                    if let Err(e) = interpreter.execute_statement(&last_stmt).await {
                        crate::error::print_error(&e);
                    }
                }
            }
        }
    }
}
