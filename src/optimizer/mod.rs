mod constant_folder;
mod inline_cache;
mod super_instructions;

pub use constant_folder::ConstantFolder;
pub use inline_cache::InlineCache;
pub use super_instructions::SuperInstructionOptimizer;

use crate::parser::ast::Program;

/// Main optimizer that applies all optimization passes
pub struct Optimizer {
    enable_constant_folding: bool,
    enable_inline_caching: bool,
    enable_super_instructions: bool,
}

impl Optimizer {
    pub fn new() -> Self {
        Optimizer {
            enable_constant_folding: true,
            enable_inline_caching: true,
            enable_super_instructions: true,
        }
    }

    pub fn with_config(constant_folding: bool, inline_caching: bool, super_instructions: bool) -> Self {
        Optimizer {
            enable_constant_folding: constant_folding,
            enable_inline_caching: inline_caching,
            enable_super_instructions: super_instructions,
        }
    }

    /// Run all enabled optimization passes on the AST
    pub fn optimize(&self, mut program: Program) -> Program {
        // Phase 1: Constant Folding (compile-time)
        if self.enable_constant_folding {
            let folder = ConstantFolder::new();
            program = folder.fold(program);
        }

        // Phase 2: Super-Instructions (compile-time pattern detection)
        if self.enable_super_instructions {
            let super_opt = SuperInstructionOptimizer::new();
            program = super_opt.optimize(program);
        }

        // Note: Inline caching happens at runtime, not here
        
        program
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}
