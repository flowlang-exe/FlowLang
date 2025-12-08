use crate::types::EssenceType;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub imports: Vec<Import>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub module: String,
    pub alias: Option<String>,
    pub from_path: Option<String>,
    pub selective: Option<Vec<SelectiveImport>>,  // NEW: For selective imports
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectiveImport {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Let {
        name: String,
        type_annotation: Option<EssenceType>,
        value: Expression,
        is_exported: bool,  // NEW: Track if exported
        line: usize,
    },
    Seal {
        name: String,
        type_annotation: Option<EssenceType>,
        value: Expression,
        is_exported: bool,  // NEW: Track if exported
        line: usize,
    },
    Assignment {
        name: String,
        value: Expression,
        line: usize,
    },
    FunctionDecl {
        name: String,
        params: Vec<Parameter>,
        return_type: Option<EssenceType>,
        body: Vec<Statement>,
        sigils: Vec<String>,
        is_exported: bool,  // NEW: Track if exported
        line: usize,
    },
    Ritual {
        name: String,
        params: Vec<Parameter>,
        return_type: Option<EssenceType>,
        body: Vec<Statement>,
        is_exported: bool,  // NEW: Track if exported
        line: usize,
    },
    Return {
        value: Option<Expression>,
        line: usize,
    },
    Stance {
        condition: Expression,
        then_branch: Vec<Statement>,
        shift_branches: Vec<(Expression, Vec<Statement>)>,
        abandon_branch: Option<Vec<Statement>>,
        line: usize,
    },
    Aura {
        value: Expression,
        cases: Vec<(Expression, Vec<Statement>)>,
        otherwise: Option<Vec<Statement>>,
        line: usize,
    },
    Phase {
        kind: PhaseKind,
        body: Vec<Statement>,
        line: usize,
    },
    Expression {
        expr: Expression,
        line: usize,
    },
    Wait {
        duration: Expression,
        unit: String,
        line: usize,
    },
    Perform {
        rituals: Vec<Expression>,
        line: usize,
    },
    // ⚔️ ERROR ARC - Error Handling Statements
    Attempt {
        body: Vec<Statement>,
        rescue_clauses: Vec<RescueClause>,
        finally_block: Option<Vec<Statement>>,
        line: usize,
    },
    Panic {
        message: Expression,
        line: usize,
    },
    Rebound {
        error: Option<String>, // Variable name holding the error
        line: usize,
    },
    Ward {
        body: Vec<Statement>,
        line: usize,
    },
    BreakSeal {
        line: usize,
    },
    FractureSeal {
        line: usize,
    },
    ShatterGrandSeal {
        value: Option<Expression>,
        line: usize,
    },
    Wound {
        message: Expression,
        line: usize,
    },
    Rupture {
        error_type: String,  // "Rift", "Glitch", "VoidTear", "Spirit"
        message: Expression,
        line: usize,
    },
    SigilDecl {
        name: String,
        fields: Vec<SigilField>,
        is_exported: bool,
        line: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RescueClause {
    pub error_type: Option<String>, // e.g., "Rift", "Glitch", None for catch-all
    pub binding: Option<String>,     // Variable to bind error to (e.g., "e")
    pub retry_count: Option<usize>,  // For "rescue retry 3"
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhaseKind {
    Count {
        variable: String,
        from: Expression,
        to: Expression,
    },
    ForEach {
        variable: String,
        collection: Expression,
    },
    Until {
        condition: Expression,
    },
    Forever,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<EssenceType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigilField {
    pub name: String,
    pub field_type: EssenceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Number(f64),
    String(String),
    InterpolatedString(Vec<Expression>),
    Boolean(bool),
    Identifier(String),
    
    Binary {
        left: Box<Expression>,
        operator: BinaryOp,
        right: Box<Expression>,
    },
    
    Unary {
        operator: UnaryOp,
        operand: Box<Expression>,
    },
    
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    
    MethodCall {
        object: Box<Expression>,
        method: String,
        arguments: Vec<Expression>,
    },
    
    Await {
        expr: Box<Expression>,
    },
    
    Array {
        elements: Vec<Expression>,
    },
    
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    
    Relic {
        entries: Vec<(String, Expression)>,
    },
    
    ComboChain {
        initial: Box<Expression>,
        operations: Vec<ChainOperation>,
    },
    
    // NEW: Inline Spell functions
    InlineSpell {
        params: Vec<String>,
        param_types: Vec<Option<EssenceType>>,
        return_type: Option<EssenceType>,
        body: InlineSpellBody,
        line: usize,
    },
    
    // NEW: Sigil Instantiation
    SigilInstance {
        sigil_name: String,
        fields: Vec<(String, Expression)>,
        line: usize,
    },
}

// NEW: Body type for inline Spells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InlineSpellBody {
    Expression(Box<Expression>),  // Spell x -> x * 2
    Block(Vec<Statement>),         // Spell (x) { return x * 2 }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainOperation {
    Call(String, Vec<Expression>),
    Method(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    
    IsEqual,
    NotEqual,
    Greater,
    Less,
    GreaterEq,
    LessEq,
    
    Both,
    Either,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UnaryOp {
    Negate,
    Minus,
}
