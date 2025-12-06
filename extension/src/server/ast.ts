// FlowLang AST Node Types

export interface Position {
    line: number;
    column: number;
    offset: number;
}

export interface Range {
    start: Position;
    end: Position;
}

// Base node
export interface ASTNode {
    type: string;
    range: Range;
}

// Program root
export interface Program extends ASTNode {
    type: 'Program';
    imports: ImportDecl[];
    statements: Statement[];
}

// Import declaration
export interface ImportDecl extends ASTNode {
    type: 'ImportDecl';
    module: string;
    alias?: string;
    fromPath?: string;
    selective?: SelectiveImport[];
}

export interface SelectiveImport {
    name: string;
    alias?: string;
}

// Statements
export type Statement =
    | LetStatement
    | SealStatement
    | FunctionDecl
    | RitualDecl
    | SigilDecl
    | ReturnStatement
    | StanceStatement
    | AuraStatement
    | PhaseStatement
    | AttemptStatement
    | ExpressionStatement
    | WaitStatement
    | AssignmentStatement;

export interface LetStatement extends ASTNode {
    type: 'LetStatement';
    name: string;
    typeAnnotation?: TypeExpr;
    value: Expression;
    isExported: boolean;
}

export interface SealStatement extends ASTNode {
    type: 'SealStatement';
    name: string;
    typeAnnotation?: TypeExpr;
    value: Expression;
    isExported: boolean;
}

export interface AssignmentStatement extends ASTNode {
    type: 'AssignmentStatement';
    name: string;
    value: Expression;
}

export interface FunctionDecl extends ASTNode {
    type: 'FunctionDecl';
    name: string;
    params: Parameter[];
    returnType?: TypeExpr;
    body: Statement[];
    sigils: string[];
    isExported: boolean;
}

export interface RitualDecl extends ASTNode {
    type: 'RitualDecl';
    name: string;
    params: Parameter[];
    returnType?: TypeExpr;
    body: Statement[];
    isExported: boolean;
}

export interface SigilDecl extends ASTNode {
    type: 'SigilDecl';
    name: string;
    fields: SigilField[];
}

export interface SigilField {
    name: string;
    typeAnnotation: TypeExpr;
}

export interface Parameter {
    name: string;
    typeAnnotation?: TypeExpr;
}

export interface ReturnStatement extends ASTNode {
    type: 'ReturnStatement';
    value?: Expression;
}

export interface StanceStatement extends ASTNode {
    type: 'StanceStatement';
    condition: Expression;
    thenBranch: Statement[];
    shiftBranches: { condition: Expression; body: Statement[] }[];
    abandonBranch?: Statement[];
}

export interface AuraStatement extends ASTNode {
    type: 'AuraStatement';
    value: Expression;
    cases: { pattern: Expression; body: Statement[] }[];
    otherwise?: Statement[];
}

export interface PhaseStatement extends ASTNode {
    type: 'PhaseStatement';
    kind: PhaseKind;
    body: Statement[];
}

export type PhaseKind =
    | { type: 'count'; variable: string; from: Expression; to: Expression }
    | { type: 'forEach'; variable: string; collection: Expression }
    | { type: 'until'; condition: Expression }
    | { type: 'forever' };

export interface AttemptStatement extends ASTNode {
    type: 'AttemptStatement';
    body: Statement[];
    rescueClauses: RescueClause[];
    finallyBlock?: Statement[];
}

export interface RescueClause {
    errorType?: string;
    binding?: string;
    body: Statement[];
}

export interface ExpressionStatement extends ASTNode {
    type: 'ExpressionStatement';
    expression: Expression;
}

export interface WaitStatement extends ASTNode {
    type: 'WaitStatement';
    duration: Expression;
    unit: string; // 'ms', 's', 'm'
}

// Type expressions
export type TypeExpr =
    | { type: 'simple'; name: string }
    | { type: 'generic'; name: string; params: TypeExpr[] };

// Expressions
export type Expression =
    | Identifier
    | NumberLiteral
    | StringLiteral
    | BooleanLiteral
    | NullLiteral
    | ArrayLiteral
    | RelicLiteral
    | BinaryExpr
    | UnaryExpr
    | CallExpr
    | MethodCallExpr
    | IndexExpr
    | MemberExpr
    | InlineSpell
    | AwaitExpr;

export interface Identifier extends ASTNode {
    type: 'Identifier';
    name: string;
}

export interface NumberLiteral extends ASTNode {
    type: 'NumberLiteral';
    value: number;
}

export interface StringLiteral extends ASTNode {
    type: 'StringLiteral';
    value: string;
    isTemplate: boolean;
}

export interface BooleanLiteral extends ASTNode {
    type: 'BooleanLiteral';
    value: boolean;
}

export interface NullLiteral extends ASTNode {
    type: 'NullLiteral';
}

export interface ArrayLiteral extends ASTNode {
    type: 'ArrayLiteral';
    elements: Expression[];
}

export interface RelicLiteral extends ASTNode {
    type: 'RelicLiteral';
    entries: { key: string; value: Expression }[];
}

export interface BinaryExpr extends ASTNode {
    type: 'BinaryExpr';
    operator: BinaryOp;
    left: Expression;
    right: Expression;
}

export type BinaryOp =
    | '+' | '-' | '*' | '/' | '%'
    | '>>' | '<<' | '>>=' | '<<='
    | 'is~' | 'not~'
    | 'both!' | 'either!';

export interface UnaryExpr extends ASTNode {
    type: 'UnaryExpr';
    operator: 'negate!' | '-';
    operand: Expression;
}

export interface CallExpr extends ASTNode {
    type: 'CallExpr';
    callee: Expression;
    arguments: Expression[];
}

export interface MethodCallExpr extends ASTNode {
    type: 'MethodCallExpr';
    object: Expression;
    method: string;
    arguments: Expression[];
}

export interface IndexExpr extends ASTNode {
    type: 'IndexExpr';
    object: Expression;
    index: Expression;
}

export interface MemberExpr extends ASTNode {
    type: 'MemberExpr';
    object: Expression;
    property: string;
}

export interface InlineSpell extends ASTNode {
    type: 'InlineSpell';
    params: string[];
    paramTypes: (TypeExpr | undefined)[];
    returnType?: TypeExpr;
    body: Expression | Statement[];
    isExpression: boolean;
}

export interface AwaitExpr extends ASTNode {
    type: 'AwaitExpr';
    expression: Expression;
}
