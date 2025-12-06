// Enhanced error printing with episode system
use super::{FlowError, get_episode_for_error, render_trace_tree};
use colored::*;

pub fn print_error_with_episode(error: &FlowError, show_trace: bool, trace_options: &super::TraceOptions, filename: Option<&str>) {
    println!();
    
    // Get episode info based on error type
    let error_type = error.error_type_name();
    let message = match error {
        FlowError::Syntax { message, .. } => message.as_str(),
        FlowError::Type { message, .. } => message.as_str(),
        FlowError::Runtime { message, .. } => message.as_str(),
        FlowError::Undefined { message, .. } => message.as_str(),
        FlowError::OutOfRange { message, .. } => message.as_str(),
        FlowError::DivisionByZero { message, .. } => message.as_str(),
        FlowError::Rift { message, .. } => message.as_str(),
        FlowError::Glitch { message, .. } => message.as_str(),
        FlowError::VoidTear { message, .. } => message.as_str(),
        FlowError::Spirit { message, .. } => message.as_str(),
        FlowError::Panic { message, .. } => message.as_str(),
        FlowError::Wound { message, .. } => message.as_str(),
        FlowError::Break { .. } => "Break seal used outside loop",
        FlowError::Continue { .. } => "Continue seal used outside loop",
    };
    
    let (line, column) = match error {
        FlowError::Syntax { line, column, .. } => (*line, *column),
        FlowError::Type { line, column, .. } => (*line, *column),
        FlowError::Runtime { line, column, .. } => (*line, *column),
        FlowError::Undefined { line, column, .. } => (*line, *column),
        FlowError::OutOfRange { line, column, .. } => (*line, *column),
        FlowError::DivisionByZero { line, column, .. } => (*line, *column),
        FlowError::Rift { line, column, .. } => (*line, *column),
        FlowError::Glitch { line, column, .. } => (*line, *column),
        FlowError::VoidTear { line, column, .. } => (*line, *column),
        FlowError::Spirit { line, column, .. } => (*line, *column),
        FlowError::Panic { line, column, .. } => (*line, *column),
        FlowError::Wound { line, column, .. } => (*line, *column),
        FlowError::Break { line, column } => (*line, *column),
        FlowError::Continue { line, column } => (*line, *column),
    };
    
    let episode = get_episode_for_error(error_type, message);
    
    // Print episode banner
    println!("{}", episode.banner().bright_cyan().bold());
    println!("{}: {} \"{}\"", "Error".red().bold(), error_type.yellow(), message.bright_white());
    println!("{}: line {}, stance {}", "Scene".cyan(), line, episode.scene_context.green());
    
    // Print trace tree if enabled and available
    if show_trace && trace_options.enabled {
        let file_name = filename.unwrap_or("script.flow");
        let sample_frames = vec![
            super::StackFrame {
                ritual_name: "main".to_string(),
                line,
                is_async: false,
                is_rescued: false,
                file: file_name.to_string(),
            },
        ];
        
        let trace_output = render_trace_tree(&sample_frames, trace_options);
        if !trace_output.is_empty() {
            println!("{}", trace_output.bright_white());
        }
    }
    
    // Print "Next Time" teaser
    println!();
    println!("{}", "Next Time:".bright_yellow().bold());
    println!("   {}", format!("\"{}\"", episode.next_time).italic().bright_white());
    println!();
}
