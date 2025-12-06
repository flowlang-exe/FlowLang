// Trace tree rendering functions
use super::StackFrame;

pub struct TraceOptions {
    pub enabled: bool,
    pub max_depth: usize,
    pub raw_mode: bool,
    pub compact: bool,
}

impl Default for TraceOptions {
    fn default() -> Self {
        TraceOptions {
            enabled: false,
            max_depth: 50,
            raw_mode: false,
            compact: false,
        }
    }
}

pub fn render_trace_tree(frames: &[StackFrame], options: &TraceOptions) -> String {
    if frames.is_empty() {
        return String::new();
    }

    if options.raw_mode {
        return render_raw_trace(frames);
    }

    if options.compact {
        return render_compact_trace(frames, options.max_depth);
    }

    render_tree_trace(frames, options.max_depth)
}

fn render_tree_trace(frames: &[StackFrame], max_depth: usize) -> String {
    let mut output = String::new();
    output.push_str("\n📜 TRACE TREE\n");
    output.push_str("root\n");

    let frames_to_show = frames.len().min(max_depth);
    let hidden_count = frames.len().saturating_sub(max_depth);

    for (i, frame) in frames.iter().take(frames_to_show).enumerate() {
        let is_last = i == frames_to_show - 1 && hidden_count == 0;
        let prefix = if is_last { "└─" } else { "├─" };
        let indent = "   ".repeat(i);
        
        let async_marker = if frame.is_async { " await" } else { "" };
        let rescued_marker = if frame.is_rescued { " ✅" } else { "" };
        
        // Extract just the filename from the full path
        let file_name = std::path::Path::new(&frame.file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&frame.file);
        
        output.push_str(&format!(
            "{}{} {}(){} @ {}:{}{}\n",
            indent, prefix, frame.ritual_name, async_marker, file_name, frame.line, rescued_marker
        ));
    }

    if hidden_count > 0 {
        let indent = "   ".repeat(frames_to_show);
        output.push_str(&format!("{}... ({} more calls hidden)\n", indent, hidden_count));
    }

    output
}

fn render_compact_trace(frames: &[StackFrame], max_depth: usize) -> String {
    let mut output = String::new();
    output.push_str("\nTRACE:\n");

    let frames_to_show = frames.len().min(max_depth);
    let hidden_count = frames.len().saturating_sub(max_depth);

    for frame in frames.iter().take(frames_to_show) {
        let async_marker = if frame.is_async { " (async)" } else { "" };
        let rescued_marker = if frame.is_rescued { " ✅" } else { " ❌" };
        output.push_str(&format!(
            " ↳ {}(){}  (line {}){}",
            frame.ritual_name, async_marker, frame.line, rescued_marker
        ));
    }

    if hidden_count > 0 {
        output.push_str(&format!("\n... ({} more calls hidden)\n", hidden_count));
    }

    output
}

fn render_raw_trace(frames: &[StackFrame]) -> String {
    let mut output = String::new();
    output.push_str("\n[RAW STACK TRACE]\n");

    for (i, frame) in frames.iter().enumerate() {
        output.push_str(&format!(
            "#{}: {}() [{}:{}] async={} rescued={}\n",
            i, 
            frame.ritual_name, 
            frame.file, 
            frame.line, 
            frame.is_async,
            frame.is_rescued
        ));
    }

    output
}

// Helper to detect terminal width
pub fn get_terminal_width() -> usize {
    // Try to get from environment variable
    if let Ok(cols) = std::env::var("COLUMNS") {
        if let Ok(width) = cols.parse::<usize>() {
            return width;
        }
    }
    
    // Default fallback
    80
}
