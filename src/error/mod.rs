use colored::*;
use std::fmt;

// Episode system for anime-themed errors
pub mod episodes;
pub use episodes::{EpisodeInfo, StackFrame, get_episode_for_error};

// Trace tree visualization
pub mod trace;
pub use trace::{TraceOptions, render_trace_tree, get_terminal_width};

// Enhanced error display with episodes
pub mod display;
pub use display::print_error_with_episode;


#[derive(Debug, Clone)]
pub enum FlowError {
    Syntax { message: String, line: usize, column: usize },
    Type { message: String, line: usize, column: usize },
    Runtime { message: String, line: usize, column: usize },
    Undefined { message: String, line: usize, column: usize },
    OutOfRange { message: String, line: usize, column: usize },
    DivisionByZero { message: String, line: usize, column: usize },
    // ⚔️ ERROR ARC - Anime-Style Errors
    Rift { message: String, line: usize, column: usize },      // Network/IO errors
    Glitch { message: String, line: usize, column: usize },    // Parse/Format errors
    VoidTear { message: String, line: usize, column: usize },  // Null/Empty access
    Spirit { message: String, line: usize, column: usize },    // Generic catchable error
    Panic { message: String, line: usize, column: usize },     // Catastrophic failure
    Wound { message: String, line: usize, column: usize },     // Soft error (non-fatal)
    
    // Control Flow "Errors" (Internal use only)
    Break { line: usize, column: usize },
    Continue { line: usize, column: usize },
}

impl FlowError {
    pub fn syntax(message: &str, line: usize, column: usize) -> Self {
        FlowError::Syntax {
            message: message.to_string(),
            line,
            column,
        }
    }
    
    pub fn type_error(message: &str, line: usize, column: usize) -> Self {
        FlowError::Type {
            message: message.to_string(),
            line,
            column,
        }
    }
    
    pub fn runtime(message: &str, line: usize, column: usize) -> Self {
        FlowError::Runtime {
            message: message.to_string(),
            line,
            column,
        }
    }
    
    pub fn undefined(message: &str, line: usize, column: usize) -> Self {
        FlowError::Undefined {
            message: message.to_string(),
            line,
            column,
        }
    }
    
    pub fn out_of_range(message: &str, line: usize, column: usize) -> Self {
        FlowError::OutOfRange {
            message: message.to_string(),
            line,
            column,
        }
    }
    
    pub fn division_by_zero(line: usize, column: usize) -> Self {
        FlowError::DivisionByZero {
            message: "STOP! To divide by the Hollow is to tear reality apart!".to_string(),
            line,
            column,
        }
    }
    
    // ⚔️ ERROR ARC - Helper methods
    pub fn rift(message: &str, line: usize, column: usize) -> Self {
        FlowError::Rift {
            message: message.to_string(),
            line,
            column,
        }
    }
    
    pub fn glitch(message: &str, line: usize, column: usize) -> Self {
        FlowError::Glitch {
            message: message.to_string(),
            line,
            column,
        }
    }
    
    pub fn void_tear(message: &str, line: usize, column: usize) -> Self {
        FlowError::VoidTear {
            message: message.to_string(),
            line,
            column,
        }
    }
    
    pub fn spirit(message: &str, line: usize, column: usize) -> Self {
        FlowError::Spirit {
            message: message.to_string(),
            line,
            column,
        }
    }
    
    pub fn panic(message: &str, line: usize, column: usize) -> Self {
        FlowError::Panic {
            message: message.to_string(),
            line,
            column,
        }
    }
    
    pub fn wound(message: &str, line: usize, column: usize) -> Self {
        FlowError::Wound {
            message: message.to_string(),
            line,
            column,
        }
    }
    
    pub fn break_seal(line: usize, column: usize) -> Self {
        FlowError::Break { line, column }
    }
    
    pub fn fracture_seal(line: usize, column: usize) -> Self {
        FlowError::Continue { line, column }
    }
    
    pub fn error_type_name(&self) -> &str {
        match self {
            FlowError::Syntax { .. } => "Syntax",
            FlowError::Type { .. } => "Type",
            FlowError::Runtime { .. } => "Runtime",
            FlowError::Undefined { .. } => "Undefined",
            FlowError::OutOfRange { .. } => "OutOfRange",
            FlowError::DivisionByZero { .. } => "DivisionByZero",
            FlowError::Rift { .. } => "Rift",
            FlowError::Glitch { .. } => "Glitch",
            FlowError::VoidTear { .. } => "VoidTear",
            FlowError::Spirit { .. } => "Spirit",
            FlowError::Panic { .. } => "Panic",
            FlowError::Wound { .. } => "Wound",
            FlowError::Break { .. } => "Break",
            FlowError::Continue { .. } => "Continue",
        }
    }
}

impl fmt::Display for FlowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FlowError::Syntax { message, line, column } => {
                write!(f, "Syntax Error at {}:{} - {}", line, column, message)
            }
            FlowError::Type { message, line, column } => {
                write!(f, "Type Error at {}:{} - {}", line, column, message)
            }
            FlowError::Runtime { message, line, column } => {
                write!(f, "Runtime Error at {}:{} - {}", line, column, message)
            }
            FlowError::Undefined { message, line, column } => {
                write!(f, "Undefined Error at {}:{} - {}", line, column, message)
            }
            FlowError::OutOfRange { message, line, column } => {
                write!(f, "Out of Range Error at {}:{} - {}", line, column, message)
            }
            FlowError::DivisionByZero { message, line, column } => {
                write!(f, "Division by Zero at {}:{} - {}", line, column, message)
            }
            FlowError::Rift { message, line, column } => {
                write!(f, "🌐 RIFT at {}:{} - {}", line, column, message)
            }
            FlowError::Glitch { message, line, column } => {
                write!(f, "⚡ GLITCH at {}:{} - {}", line, column, message)
            }
            FlowError::VoidTear { message, line, column } => {
                write!(f, "🕳️  VOID TEAR at {}:{} - {}", line, column, message)
            }
            FlowError::Spirit { message, line, column } => {
                write!(f, "👻 SPIRIT at {}:{} - {}", line, column, message)
            }
            FlowError::Panic { message, line, column } => {
                write!(f, "💀 PANIC at {}:{} - {}", line, column, message)
            }
            FlowError::Wound { message, line, column } => {
                write!(f, "🩹 WOUND at {}:{} - {}", line, column, message)
            }
            FlowError::Break { line, column } => {
                write!(f, "Break at {}:{}", line, column)
            }
            FlowError::Continue { line, column } => {
                write!(f, "Continue at {}:{}", line, column)
            }
        }
    }
}

impl std::error::Error for FlowError {}

pub fn print_error(error: &FlowError) {
    // Default trace options for simple print_error
    let trace_options = TraceOptions {
        enabled: false,
        max_depth: 50,
        raw_mode: false,
        compact: get_terminal_width() < 60,
    };
    
    print_error_with_episode(error, false, &trace_options, None);
}
