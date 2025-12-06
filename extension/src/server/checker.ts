// FlowLang Type Checker

import {
    Program, Statement, Expression, FunctionDecl, RitualDecl, LetStatement,
    SealStatement, Parameter, TypeExpr, StanceStatement, PhaseStatement,
    AttemptStatement, AuraStatement, ReturnStatement, SigilDecl, ImportDecl
} from './ast';
import { Range } from './ast';

export interface TypeCheckError {
    message: string;
    range: Range;
    severity: 'error' | 'warning' | 'info';
}

// FlowLang essence types
export type EssenceType =
    | { kind: 'Ember' }
    | { kind: 'Silk' }
    | { kind: 'Pulse' }
    | { kind: 'Flux' }
    | { kind: 'Hollow' }
    | { kind: 'Constellation'; elementType: EssenceType }
    | { kind: 'Relic'; keyType: EssenceType; valueType: EssenceType }
    | { kind: 'Spell'; params: EssenceType[]; returnType: EssenceType }
    | { kind: 'Module'; name: string }
    | { kind: 'Unknown' };

interface SymbolInfo {
    type: EssenceType;
    mutable: boolean;
    range: Range;
}

interface FunctionInfo {
    params: { name: string; type: EssenceType }[];
    returnType: EssenceType;
    range: Range;
}

// Standard library module definitions
const STDLIB_MODULES: Record<string, Record<string, { params: string[]; returnType: string }>> = {
    'std:file': {
        'read': { params: ['Silk'], returnType: 'Silk' },
        'write': { params: ['Silk', 'Silk'], returnType: 'Hollow' },
        'append': { params: ['Silk', 'Silk'], returnType: 'Hollow' },
        'exists': { params: ['Silk'], returnType: 'Pulse' },
        'delete': { params: ['Silk'], returnType: 'Hollow' },
        'list': { params: ['Silk'], returnType: 'Constellation' },
        'mkdir': { params: ['Silk'], returnType: 'Hollow' },
        'copy': { params: ['Silk', 'Silk'], returnType: 'Hollow' },
        'move': { params: ['Silk', 'Silk'], returnType: 'Hollow' },
    },
    'std:json': {
        'parse': { params: ['Silk'], returnType: 'Relic' },
        'stringify': { params: ['Flux'], returnType: 'Silk' },
    },
    'std:net': {
        'get': { params: ['Silk'], returnType: 'Silk' },
        'post': { params: ['Silk', 'Silk'], returnType: 'Silk' },
        'put': { params: ['Silk', 'Silk'], returnType: 'Silk' },
        'delete': { params: ['Silk'], returnType: 'Silk' },
        'patch': { params: ['Silk', 'Silk'], returnType: 'Silk' },
    },
    'std:time': {
        'now': { params: [], returnType: 'Ember' },
        'format': { params: ['Ember', 'Silk'], returnType: 'Silk' },
        'iso': { params: [], returnType: 'Silk' },
        'sleep': { params: ['Ember'], returnType: 'Hollow' },
        'timestamp': { params: [], returnType: 'Ember' },
    },
    'std:timer': {
        'interval': { params: ['Ember', 'Spell'], returnType: 'Flux' },
        'timeout': { params: ['Ember', 'Spell'], returnType: 'Flux' },
        'clear': { params: ['Flux'], returnType: 'Pulse' },
    },
    'std:math': {
        'sin': { params: ['Ember'], returnType: 'Ember' },
        'cos': { params: ['Ember'], returnType: 'Ember' },
        'tan': { params: ['Ember'], returnType: 'Ember' },
        'sqrt': { params: ['Ember'], returnType: 'Ember' },
        'abs': { params: ['Ember'], returnType: 'Ember' },
        'round': { params: ['Ember'], returnType: 'Ember' },
        'floor': { params: ['Ember'], returnType: 'Ember' },
        'ceil': { params: ['Ember'], returnType: 'Ember' },
        'pow': { params: ['Ember', 'Ember'], returnType: 'Ember' },
        'random': { params: [], returnType: 'Ember' },
        'min': { params: ['Ember', 'Ember'], returnType: 'Ember' },
        'max': { params: ['Ember', 'Ember'], returnType: 'Ember' },
        'log': { params: ['Ember'], returnType: 'Ember' },
        'exp': { params: ['Ember'], returnType: 'Ember' },
    },
    'std:string': {
        'len': { params: ['Silk'], returnType: 'Ember' },
        'upper': { params: ['Silk'], returnType: 'Silk' },
        'lower': { params: ['Silk'], returnType: 'Silk' },
        'trim': { params: ['Silk'], returnType: 'Silk' },
        'split': { params: ['Silk', 'Silk'], returnType: 'Constellation' },
        'substring': { params: ['Silk', 'Ember', 'Ember'], returnType: 'Silk' },
        'replace': { params: ['Silk', 'Silk', 'Silk'], returnType: 'Silk' },
        'contains': { params: ['Silk', 'Silk'], returnType: 'Pulse' },
        'starts_with': { params: ['Silk', 'Silk'], returnType: 'Pulse' },
        'ends_with': { params: ['Silk', 'Silk'], returnType: 'Pulse' },
    },
    'std:array': {
        'len': { params: ['Constellation'], returnType: 'Ember' },
        'push': { params: ['Constellation', 'Flux'], returnType: 'Constellation' },
        'pop': { params: ['Constellation'], returnType: 'Flux' },
        'contains': { params: ['Constellation', 'Flux'], returnType: 'Pulse' },
        'reverse': { params: ['Constellation'], returnType: 'Constellation' },
        'sort': { params: ['Constellation'], returnType: 'Constellation' },
    },
    'std:crypto': {
        'sha256': { params: ['Silk'], returnType: 'Silk' },
        'sha512': { params: ['Silk'], returnType: 'Silk' },
        'md5': { params: ['Silk'], returnType: 'Silk' },
        'uuid': { params: [], returnType: 'Silk' },
        'base64_encode': { params: ['Silk'], returnType: 'Silk' },
        'base64_decode': { params: ['Silk'], returnType: 'Silk' },
        'hex_encode': { params: ['Silk'], returnType: 'Silk' },
        'hex_decode': { params: ['Silk'], returnType: 'Silk' },
    },
    'std:cli': {
        'input': { params: ['Silk'], returnType: 'Silk' },
        'args': { params: [], returnType: 'Constellation' },
        'confirm': { params: ['Silk'], returnType: 'Pulse' },
        'select': { params: ['Silk', 'Constellation'], returnType: 'Silk' },
        'clear': { params: [], returnType: 'Hollow' },
        'exit': { params: ['Ember'], returnType: 'Hollow' },
    },
    'std:os': {
        'env': { params: ['Silk'], returnType: 'Silk' },
        'exit': { params: ['Ember'], returnType: 'Hollow' },
        'platform': { params: [], returnType: 'Silk' },
        'cwd': { params: [], returnType: 'Silk' },
        'home': { params: [], returnType: 'Silk' },
    },
    'std:color': {
        'red': { params: ['Silk'], returnType: 'Silk' },
        'green': { params: ['Silk'], returnType: 'Silk' },
        'blue': { params: ['Silk'], returnType: 'Silk' },
        'yellow': { params: ['Silk'], returnType: 'Silk' },
        'magenta': { params: ['Silk'], returnType: 'Silk' },
        'cyan': { params: ['Silk'], returnType: 'Silk' },
        'white': { params: ['Silk'], returnType: 'Silk' },
        'black': { params: ['Silk'], returnType: 'Silk' },
        'bold': { params: ['Silk'], returnType: 'Silk' },
        'italic': { params: ['Silk'], returnType: 'Silk' },
        'underline': { params: ['Silk'], returnType: 'Silk' },
        'reset': { params: ['Silk'], returnType: 'Silk' },
    },
    'std:path': {
        'join': { params: ['Silk', 'Silk'], returnType: 'Silk' },
        'basename': { params: ['Silk'], returnType: 'Silk' },
        'dirname': { params: ['Silk'], returnType: 'Silk' },
        'extname': { params: ['Silk'], returnType: 'Silk' },
        'resolve': { params: ['Silk'], returnType: 'Silk' },
        'is_absolute': { params: ['Silk'], returnType: 'Pulse' },
    },
    'std:url': {
        'parse': { params: ['Silk'], returnType: 'Relic' },
        'encode': { params: ['Silk'], returnType: 'Silk' },
        'decode': { params: ['Silk'], returnType: 'Silk' },
    },
    'std:web': {
        'serve': { params: ['Ember', 'Spell'], returnType: 'Hollow' },
        'route': { params: ['Silk', 'Silk', 'Spell'], returnType: 'Hollow' },
        'json_response': { params: ['Relic'], returnType: 'Relic' },
        'html_response': { params: ['Silk'], returnType: 'Relic' },
        'redirect': { params: ['Silk'], returnType: 'Relic' },
    },
    'std:stream': {
        'create': { params: [], returnType: 'Flux' },
        'write': { params: ['Flux', 'Silk'], returnType: 'Hollow' },
        'read': { params: ['Flux'], returnType: 'Silk' },
        'close': { params: ['Flux'], returnType: 'Hollow' },
    },
};

export class TypeChecker {
    private symbols: Map<string, SymbolInfo> = new Map();
    private functions: Map<string, FunctionInfo> = new Map();
    private sigils: Map<string, { fields: { name: string; type: EssenceType }[] }> = new Map();
    private modules: Map<string, string> = new Map(); // module alias -> module path
    public errors: TypeCheckError[] = [];
    private currentReturnType: EssenceType | null = null;
    private scopeStack: Map<string, SymbolInfo>[] = [];

    check(program: Program): TypeCheckError[] {
        this.errors = [];
        this.symbols = new Map();
        this.functions = new Map();
        this.sigils = new Map();
        this.modules = new Map();
        this.scopeStack = [];

        // Register imported modules from circle statements
        for (const imp of program.imports) {
            this.registerImport(imp);
        }

        // First pass: collect function and sigil declarations
        for (const stmt of program.statements) {
            if (stmt.type === 'FunctionDecl') {
                this.registerFunction(stmt);
            } else if (stmt.type === 'RitualDecl') {
                this.registerRitual(stmt);
            } else if (stmt.type === 'SigilDecl') {
                this.registerSigil(stmt);
            }
        }

        // Second pass: check all statements
        for (const stmt of program.statements) {
            this.checkStatement(stmt);
        }

        return this.errors;
    }

    private registerImport(imp: ImportDecl): void {
        const name = imp.alias || imp.module;
        const modulePath = imp.fromPath || `std:${imp.module}`;

        // Register module as a symbol
        this.symbols.set(name, {
            type: { kind: 'Module', name: modulePath },
            mutable: false,
            range: imp.range,
        });
        this.modules.set(name, modulePath);
    }

    private pushScope(): void {
        this.scopeStack.push(new Map(this.symbols));
    }

    private popScope(): void {
        const prevScope = this.scopeStack.pop();
        if (prevScope) {
            this.symbols = prevScope;
        }
    }

    private registerFunction(decl: FunctionDecl): void {
        const params = decl.params.map(p => ({
            name: p.name,
            type: this.resolveTypeExpr(p.typeAnnotation),
        }));
        const returnType = this.resolveTypeExpr(decl.returnType);

        this.functions.set(decl.name, {
            params,
            returnType,
            range: decl.range,
        });
    }

    private registerRitual(decl: RitualDecl): void {
        const params = decl.params.map(p => ({
            name: p.name,
            type: this.resolveTypeExpr(p.typeAnnotation),
        }));
        const returnType = this.resolveTypeExpr(decl.returnType);

        this.functions.set(decl.name, {
            params,
            returnType,
            range: decl.range,
        });
    }

    private registerSigil(decl: SigilDecl): void {
        const fields = decl.fields.map(f => ({
            name: f.name,
            type: this.resolveTypeExpr(f.typeAnnotation),
        }));
        this.sigils.set(decl.name, { fields });
    }

    private resolveTypeExpr(typeExpr?: TypeExpr): EssenceType {
        if (!typeExpr) return { kind: 'Flux' };

        if (typeExpr.type === 'simple') {
            switch (typeExpr.name) {
                case 'Ember': return { kind: 'Ember' };
                case 'Silk': return { kind: 'Silk' };
                case 'Pulse': return { kind: 'Pulse' };
                case 'Flux': return { kind: 'Flux' };
                case 'Hollow': return { kind: 'Hollow' };
                case 'Spell': return { kind: 'Spell', params: [], returnType: { kind: 'Flux' } };
                default:
                    // Could be a sigil type
                    if (this.sigils.has(typeExpr.name)) {
                        return { kind: 'Relic', keyType: { kind: 'Silk' }, valueType: { kind: 'Flux' } };
                    }
                    return { kind: 'Unknown' };
            }
        }

        if (typeExpr.type === 'generic') {
            if (typeExpr.name === 'Constellation') {
                const elementType = typeExpr.params[0]
                    ? this.resolveTypeExpr(typeExpr.params[0])
                    : { kind: 'Flux' as const };
                return { kind: 'Constellation', elementType };
            }
            if (typeExpr.name === 'Relic') {
                const keyType = typeExpr.params[0]
                    ? this.resolveTypeExpr(typeExpr.params[0])
                    : { kind: 'Silk' as const };
                const valueType = typeExpr.params[1]
                    ? this.resolveTypeExpr(typeExpr.params[1])
                    : { kind: 'Flux' as const };
                return { kind: 'Relic', keyType, valueType };
            }
        }

        return { kind: 'Unknown' };
    }

    private checkStatement(stmt: Statement): void {
        switch (stmt.type) {
            case 'LetStatement':
                this.checkLetStatement(stmt);
                break;
            case 'SealStatement':
                this.checkSealStatement(stmt);
                break;
            case 'FunctionDecl':
                this.checkFunctionDecl(stmt);
                break;
            case 'RitualDecl':
                this.checkRitualDecl(stmt);
                break;
            case 'ReturnStatement':
                this.checkReturnStatement(stmt);
                break;
            case 'StanceStatement':
                this.checkStanceStatement(stmt);
                break;
            case 'AuraStatement':
                this.checkAuraStatement(stmt);
                break;
            case 'PhaseStatement':
                this.checkPhaseStatement(stmt);
                break;
            case 'AttemptStatement':
                this.checkAttemptStatement(stmt);
                break;
            case 'AssignmentStatement':
                this.checkAssignment(stmt);
                break;
            case 'ExpressionStatement':
                this.inferType(stmt.expression);
                break;
        }
    }

    private checkLetStatement(stmt: LetStatement): void {
        const valueType = this.inferType(stmt.value);
        const declaredType = this.resolveTypeExpr(stmt.typeAnnotation);

        if (stmt.typeAnnotation && !this.isAssignable(valueType, declaredType)) {
            this.addError(
                `Type mismatch: expected ${this.typeToString(declaredType)}, got ${this.typeToString(valueType)}`,
                stmt.range,
                'error'
            );
        }

        this.symbols.set(stmt.name, {
            type: stmt.typeAnnotation ? declaredType : valueType,
            mutable: true,
            range: stmt.range,
        });
    }

    private checkSealStatement(stmt: SealStatement): void {
        const valueType = this.inferType(stmt.value);
        const declaredType = this.resolveTypeExpr(stmt.typeAnnotation);

        if (stmt.typeAnnotation && !this.isAssignable(valueType, declaredType)) {
            this.addError(
                `Type mismatch: expected ${this.typeToString(declaredType)}, got ${this.typeToString(valueType)}`,
                stmt.range,
                'error'
            );
        }

        this.symbols.set(stmt.name, {
            type: stmt.typeAnnotation ? declaredType : valueType,
            mutable: false,
            range: stmt.range,
        });
    }

    private checkAssignment(stmt: { name: string; value: Expression; range: Range }): void {
        const symbol = this.symbols.get(stmt.name);

        if (!symbol) {
            this.addError(
                `Undefined variable '${stmt.name}'`,
                stmt.range,
                'error'
            );
            return;
        }

        if (!symbol.mutable) {
            this.addError(
                `Cannot reassign sealed variable '${stmt.name}'`,
                stmt.range,
                'error'
            );
        }

        const valueType = this.inferType(stmt.value);
        if (!this.isAssignable(valueType, symbol.type)) {
            this.addError(
                `Type mismatch: cannot assign ${this.typeToString(valueType)} to ${this.typeToString(symbol.type)}`,
                stmt.range,
                'error'
            );
        }
    }

    private checkFunctionDecl(decl: FunctionDecl): void {
        this.pushScope();

        const funcInfo = this.functions.get(decl.name);
        if (funcInfo) {
            this.currentReturnType = funcInfo.returnType;

            // Add parameters to scope
            for (const param of funcInfo.params) {
                this.symbols.set(param.name, {
                    type: param.type,
                    mutable: true,
                    range: decl.range,
                });
            }
        }

        // Check body
        for (const stmt of decl.body) {
            this.checkStatement(stmt);
        }

        this.currentReturnType = null;
        this.popScope();
    }

    private checkRitualDecl(decl: RitualDecl): void {
        this.pushScope();

        const funcInfo = this.functions.get(decl.name);
        if (funcInfo) {
            this.currentReturnType = funcInfo.returnType;

            for (const param of funcInfo.params) {
                this.symbols.set(param.name, {
                    type: param.type,
                    mutable: true,
                    range: decl.range,
                });
            }
        }

        for (const stmt of decl.body) {
            this.checkStatement(stmt);
        }

        this.currentReturnType = null;
        this.popScope();
    }

    private checkReturnStatement(stmt: ReturnStatement): void {
        if (!this.currentReturnType) return;

        if (stmt.value) {
            const valueType = this.inferType(stmt.value);
            if (!this.isAssignable(valueType, this.currentReturnType)) {
                this.addError(
                    `Return type mismatch: expected ${this.typeToString(this.currentReturnType)}, got ${this.typeToString(valueType)}`,
                    stmt.range,
                    'error'
                );
            }
        } else if (this.currentReturnType.kind !== 'Hollow' && this.currentReturnType.kind !== 'Flux') {
            this.addError(
                `Expected return value of type ${this.typeToString(this.currentReturnType)}`,
                stmt.range,
                'error'
            );
        }
    }

    private checkStanceStatement(stmt: StanceStatement): void {
        const condType = this.inferType(stmt.condition);
        // Condition should be truthy-checkable (any type works in FlowLang)

        this.pushScope();
        for (const s of stmt.thenBranch) {
            this.checkStatement(s);
        }
        this.popScope();

        for (const shift of stmt.shiftBranches) {
            this.inferType(shift.condition);
            this.pushScope();
            for (const s of shift.body) {
                this.checkStatement(s);
            }
            this.popScope();
        }

        if (stmt.abandonBranch) {
            this.pushScope();
            for (const s of stmt.abandonBranch) {
                this.checkStatement(s);
            }
            this.popScope();
        }
    }

    private checkAuraStatement(stmt: AuraStatement): void {
        this.inferType(stmt.value);

        for (const c of stmt.cases) {
            this.inferType(c.pattern);
            this.pushScope();
            for (const s of c.body) {
                this.checkStatement(s);
            }
            this.popScope();
        }

        if (stmt.otherwise) {
            this.pushScope();
            for (const s of stmt.otherwise) {
                this.checkStatement(s);
            }
            this.popScope();
        }
    }

    private checkPhaseStatement(stmt: PhaseStatement): void {
        this.pushScope();

        if (stmt.kind.type === 'count') {
            const fromType = this.inferType(stmt.kind.from);
            const toType = this.inferType(stmt.kind.to);

            if (fromType.kind !== 'Ember' && fromType.kind !== 'Flux') {
                this.addError(
                    `Phase 'from' value must be Ember, got ${this.typeToString(fromType)}`,
                    stmt.range,
                    'error'
                );
            }
            if (toType.kind !== 'Ember' && toType.kind !== 'Flux') {
                this.addError(
                    `Phase 'to' value must be Ember, got ${this.typeToString(toType)}`,
                    stmt.range,
                    'error'
                );
            }

            this.symbols.set(stmt.kind.variable, {
                type: { kind: 'Ember' },
                mutable: false,
                range: stmt.range,
            });
        } else if (stmt.kind.type === 'forEach') {
            const collType = this.inferType(stmt.kind.collection);
            let elementType: EssenceType = { kind: 'Flux' };

            if (collType.kind === 'Constellation') {
                elementType = collType.elementType;
            } else if (collType.kind !== 'Flux') {
                this.addError(
                    `For-each requires a Constellation, got ${this.typeToString(collType)}`,
                    stmt.range,
                    'error'
                );
            }

            this.symbols.set(stmt.kind.variable, {
                type: elementType,
                mutable: false,
                range: stmt.range,
            });
        } else if (stmt.kind.type === 'until') {
            this.inferType(stmt.kind.condition);
        }

        for (const s of stmt.body) {
            this.checkStatement(s);
        }

        this.popScope();
    }

    private checkAttemptStatement(stmt: AttemptStatement): void {
        this.pushScope();
        for (const s of stmt.body) {
            this.checkStatement(s);
        }
        this.popScope();

        for (const rescue of stmt.rescueClauses) {
            this.pushScope();
            if (rescue.binding) {
                this.symbols.set(rescue.binding, {
                    type: { kind: 'Silk' },
                    mutable: false,
                    range: stmt.range,
                });
            }
            for (const s of rescue.body) {
                this.checkStatement(s);
            }
            this.popScope();
        }

        if (stmt.finallyBlock) {
            this.pushScope();
            for (const s of stmt.finallyBlock) {
                this.checkStatement(s);
            }
            this.popScope();
        }
    }

    private inferType(expr: Expression): EssenceType {
        switch (expr.type) {
            case 'NumberLiteral':
                return { kind: 'Ember' };
            case 'StringLiteral':
                return { kind: 'Silk' };
            case 'BooleanLiteral':
                return { kind: 'Pulse' };
            case 'NullLiteral':
                return { kind: 'Hollow' };
            case 'ArrayLiteral':
                if (expr.elements.length === 0) {
                    return { kind: 'Constellation', elementType: { kind: 'Flux' } };
                }
                const firstType = this.inferType(expr.elements[0]);
                return { kind: 'Constellation', elementType: firstType };
            case 'RelicLiteral':
                return { kind: 'Relic', keyType: { kind: 'Silk' }, valueType: { kind: 'Flux' } };
            case 'Identifier':
                const symbol = this.symbols.get(expr.name);
                if (!symbol) {
                    // Check if it's a function
                    if (this.functions.has(expr.name)) {
                        return { kind: 'Spell', params: [], returnType: { kind: 'Flux' } };
                    }
                    this.addError(
                        `Undefined variable '${expr.name}'`,
                        expr.range,
                        'error'
                    );
                    return { kind: 'Unknown' };
                }
                return symbol.type;
            case 'BinaryExpr':
                return this.inferBinaryType(expr);
            case 'UnaryExpr':
                if (expr.operator === '-') {
                    return { kind: 'Ember' };
                }
                return { kind: 'Pulse' }; // negate!
            case 'CallExpr':
                return this.inferCallType(expr);
            case 'MethodCallExpr':
                return this.inferMethodCallType(expr);
            case 'IndexExpr':
                const objType = this.inferType(expr.object);
                if (objType.kind === 'Constellation') {
                    return objType.elementType;
                }
                if (objType.kind === 'Relic') {
                    return objType.valueType;
                }
                return { kind: 'Flux' };
            case 'MemberExpr':
                // Property access like obj.property or module.constant
                const memberObjType = this.inferType(expr.object);
                if (memberObjType.kind === 'Module') {
                    // For modules, property access is treated as Flux (could be constants)
                    return { kind: 'Flux' };
                }
                if (memberObjType.kind === 'Relic') {
                    return memberObjType.valueType;
                }
                return { kind: 'Flux' };
            case 'InlineSpell':
                return { kind: 'Spell', params: [], returnType: { kind: 'Flux' } };
            case 'AwaitExpr':
                return this.inferType(expr.expression);
            default:
                return { kind: 'Flux' };
        }
    }

    private inferBinaryType(expr: { operator: string; left: Expression; right: Expression }): EssenceType {
        const leftType = this.inferType(expr.left);
        const rightType = this.inferType(expr.right);

        switch (expr.operator) {
            case '+':
                // String concatenation or number addition
                if (leftType.kind === 'Silk' || rightType.kind === 'Silk') {
                    return { kind: 'Silk' };
                }
                return { kind: 'Ember' };
            case '-':
            case '*':
            case '/':
            case '%':
                return { kind: 'Ember' };
            case '>>':
            case '<<':
            case '>>=':
            case '<<=':
            case 'is~':
            case 'not~':
            case 'both!':
            case 'either!':
                return { kind: 'Pulse' };
            default:
                return { kind: 'Flux' };
        }
    }

    private inferCallType(expr: { callee: Expression; arguments: Expression[] }): EssenceType {
        if (expr.callee.type === 'Identifier') {
            const funcInfo = this.functions.get(expr.callee.name);
            if (funcInfo) {
                // Check argument count
                if (expr.arguments.length !== funcInfo.params.length) {
                    this.addError(
                        `Function '${expr.callee.name}' expects ${funcInfo.params.length} arguments, got ${expr.arguments.length}`,
                        expr.callee.range,
                        'error'
                    );
                }

                // Check argument types
                for (let i = 0; i < Math.min(expr.arguments.length, funcInfo.params.length); i++) {
                    const argType = this.inferType(expr.arguments[i]);
                    const paramType = funcInfo.params[i].type;
                    if (!this.isAssignable(argType, paramType)) {
                        this.addError(
                            `Argument ${i + 1} type mismatch: expected ${this.typeToString(paramType)}, got ${this.typeToString(argType)}`,
                            expr.arguments[i].range,
                            'error'
                        );
                    }
                }

                return funcInfo.returnType;
            }

            // Built-in functions
            const builtins: Record<string, EssenceType> = {
                'whisper': { kind: 'Hollow' },
                'shout': { kind: 'Hollow' },
                'roar': { kind: 'Hollow' },
                'chant': { kind: 'Hollow' },
            };
            if (builtins[expr.callee.name]) {
                return builtins[expr.callee.name];
            }
        }

        return { kind: 'Flux' };
    }

    private inferMethodCallType(expr: { object: Expression; method: string; arguments: Expression[] }): EssenceType {
        const objType = this.inferType(expr.object);

        // Module method calls (e.g., fs.read(), json.parse())
        if (objType.kind === 'Module') {
            const moduleFuncs = STDLIB_MODULES[objType.name];
            if (moduleFuncs && moduleFuncs[expr.method]) {
                const funcDef = moduleFuncs[expr.method];
                // Return the function's return type
                switch (funcDef.returnType) {
                    case 'Ember': return { kind: 'Ember' };
                    case 'Silk': return { kind: 'Silk' };
                    case 'Pulse': return { kind: 'Pulse' };
                    case 'Hollow': return { kind: 'Hollow' };
                    case 'Relic': return { kind: 'Relic', keyType: { kind: 'Silk' }, valueType: { kind: 'Flux' } };
                    case 'Constellation': return { kind: 'Constellation', elementType: { kind: 'Flux' } };
                    default: return { kind: 'Flux' };
                }
            }
            // Unknown method on module
            return { kind: 'Flux' };
        }

        // Array methods
        if (objType.kind === 'Constellation') {
            switch (expr.method) {
                case 'len':
                    return { kind: 'Ember' };
                case 'push':
                case 'concat':
                case 'filter':
                case 'constellation':
                case 'reverse':
                case 'slice':
                    return objType;
                case 'pop':
                case 'find':
                    return objType.elementType;
                case 'reduce':
                    return { kind: 'Flux' }; // Depends on callback
                case 'join':
                    return { kind: 'Silk' };
                case 'every':
                case 'some':
                    return { kind: 'Pulse' };
            }
        }

        return { kind: 'Flux' };
    }

    private isAssignable(source: EssenceType, target: EssenceType): boolean {
        // Flux accepts anything (dynamic type)
        if (target.kind === 'Flux') return true;
        if (source.kind === 'Flux') return true;
        if (source.kind === 'Unknown') return true;

        // Relic with Flux valueType is dynamic (like json.parse() result)
        // It can be assigned to any type
        if (source.kind === 'Relic' && source.valueType.kind === 'Flux') {
            return true;
        }

        // Any type can be assigned to untyped Relic
        if (target.kind === 'Relic' && target.valueType.kind === 'Flux') {
            return true;
        }

        // Same kind
        if (source.kind === target.kind) {
            if (source.kind === 'Constellation' && target.kind === 'Constellation') {
                return this.isAssignable(source.elementType, target.elementType);
            }
            if (source.kind === 'Relic' && target.kind === 'Relic') {
                return this.isAssignable(source.keyType, target.keyType) &&
                    this.isAssignable(source.valueType, target.valueType);
            }
            return true;
        }

        return false;
    }

    private typeToString(type: EssenceType): any {
        switch (type.kind) {
            case 'Ember':
            case 'Silk':
            case 'Pulse':
            case 'Flux':
            case 'Hollow':
            case 'Unknown':
                return type.kind;
            case 'Constellation':
                return `Constellation<${this.typeToString(type.elementType)}>`;
            case 'Relic':
                return `Relic<${this.typeToString(type.keyType)}, ${this.typeToString(type.valueType)}>`;
            case 'Spell':
                return 'Spell';
        }
    }

    private addError(message: string, range: Range, severity: 'error' | 'warning' | 'info'): void {
        this.errors.push({ message, range, severity });
    }
}
