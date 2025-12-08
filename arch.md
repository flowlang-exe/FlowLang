# 🌌 FlowLang Architecture

This document describes the internal architecture of the FlowLang runtime and development tools.

## 1. System Overview

FlowLang consists of two main components:
1.  **The Rust Runtime (`flowlang` CLI)**: The core execution engine that runs your code. It handles parsing, optimizing, and executing scripts with an async event loop.
2.  **The VS Code Extension (LSP)**: A TypeScript-based Language Server that provides editor features like autocomplete, type checking, and error highlighting.

```text
+---------------------------------------------------------------+
|                    RUST RUNTIME (flowlang)                    |
+---------------------------------------------------------------+
|                                                               |
|  [Source Code] -> .flow file                                  |
|       |                                                       |
|       v                                                       |
|  [Lexer]  -> Token Stream                                     |
|       |                                                       |
|       v                                                       |
|  [Parser] -> Abstract Syntax Tree (AST)                       |
|       |                                                       |
|       v                                                       |
|  [Optimizer] -> Optimized AST (Const Folding, etc.)           |
|       |                                                       |
|       v                                                       |
|  [Interpreter] -> Executes Nodes                              |
|       |                                                       |
|       v                                                       |
|  [Event Loop] -> Async Tasks, Timers, HTTP                    |
|                                                               |
+---------------------------------------------------------------+

                          ^
                          |  (Shared Concepts / AST Design)
                          v

+---------------------------------------------------------------+
|                    VS CODE EXTENSION (LSP)                    |
+---------------------------------------------------------------+
|                                                               |
|  [VS Code Editor] <--> [LSP Server]                           |
|                             |                                 |
|                             v                                 |
|                       [TS Parser]                             |
|                             |                                 |
|                             v                                 |
|                        [TS AST]                               |
|                             |                                 |
|                             v                                 |
|                     [Type Checker]                            |
|                     * Sigil Validation                        |
|                     * Type Inference                          |
|                                                               |
+---------------------------------------------------------------+
```

## 2. The Rust Runtime (`src/`)

The core runtime is built in Rust using `tokio` for asynchronous execution.

### 2.1. Execution Pipeline (`main.rs`)
When you run `flowlang run script.flow`, the following pipeline executes:

1.  **Lexical Analysis (`src/lexer/mod.rs`)**:
    *   Converts source text into a stream of `Token`s.
    *   Handles string interpolation (`TokenKind::StringPart`), numbers, and keywords.
    *   Produces `TokenKind::Both` / `TokenKind::Either` for boolean literals.

2.  **Parsing (`src/parser/mod.rs`)**:
    *   Uses a recursive descent parser.
    *   Constructs the **AST** (`src/parser/ast.rs`).
    *   Handles specific constructs:
        *   `SigilInstance`: `Name { field: value }` logic.
        *   `Stance` (Switch): Complex pattern matching structure.
        *   `Ritual`/`Spell`: Function declarations.

3.  **Optimization (`src/optimizer/`)**:
    *   **Constant Folding** (`constant_folder.rs`): Pre-calculates constant expressions (e.g., `2 + 3` → `5`).
    *   **Super Instructions** (`super_instructions.rs`): Merges common patterns into single optimized nodes (e.g., `Set` + `Get` optimization).
    *   *Note: Both optimizers were recently updated to handle `SigilInstance` recursively.*

4.  **Interpretation (`src/interpreter/mod.rs`)**:
    *   Walks the AST and executes nodes significantly.
    *   **Environment**: Manages scopes, variables (`Let`), and constants (`Seal`).
    *   **Sigil Registry**: Stores `SigilDecl` schemas for runtime validation.
    *   **Values (`src/types.rs`)**:
        *   `Ember` (f64), `Silk` (String), `Pulse` (bool)
        *   `Relic` (HashMap) - used for generic objects and Sigil instances.
        *   `Flux` (Any)

5.  **Event Loop (`src/main.rs`)**:
    *   Built on `tokio`.
    *   Maintains a count of active handles (timers, servers).
    *   Processes **Callbacks**:
        *   `std:timer`: `timeout` / `interval`.
        *   `std:net`: Web server request handlers.
    *   Keeps the process alive until all async tasks complete.

## 3. The Extension (LSP) (`extension/src/server/`)

The VS Code extension implements the Language Server Protocol (LSP) in TypeScript. It provides a "static" view of the code, separate from the Rust runtime.

*   **Parser (`parser.ts`)**: A parallel implementation of the Rust parser in TypeScript. It builds a slightly lightweight AST for analysis.
    *   *Recently updated to support `SigilInstance` syntax.*
*   **Type Checker (`checker.ts`)**: 
    *   Performs static analysis.
    *   **Sigil Validation**: Checks that fields in `SigilInstance` match the `SigilDecl`.
    *   **Inference**: Infers types for variables (`let x = 10` → `Ember`).
    *   **Compatibility**: Checks assignments via `isAssignable`.
*   **Editor Features (`server.ts`)**:
    *   **Hover**: Shows variable types (including `*(inferred)*`), function signatures, and Sigil definitions.
    *   **Completion**: Suggests keywords, variables, and snippets.
    *   **Definitions**: Jump to definition for variables and functions.

## 4. Key Data Structures

### AST (`Statement` Enum)
```rust
pub enum Statement {
    Let { name, value, .. },
    SigilDecl { name, fields, .. }, // Custom Type Definition
    Expression(Expression),
    // ...
}
```

### Values (`Value` Enum)
The runtime representation of data:
```rust
pub enum Value {
    Ember(f64),
    Silk(String),
    Pulse(bool),
    Relic(Arc<HashMap<String, Value>>), // Used for Sigils & Maps
    Constellation(Arc<Vec<Value>>),     // Arrays
    Spell(FunctionDefinition),          // Functions
    Null,
}
```

## 5. Recent Improvements

*   **Working Sigils**: Full pipeline support from Parser → AST → Interpreter → Checker.
*   **Type Inference**: The LSP now intelligently infers types for hover and validation.
*   **Parser Fixes**: Boolean literals `both!` and `either!` are correctly prioritized in expressions.
