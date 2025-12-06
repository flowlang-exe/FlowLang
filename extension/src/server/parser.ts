// FlowLang Parser - Recursive Descent Parser

import { Lexer, Token, TokenType, LexerError } from './lexer';
import {
    Program, Statement, Expression, ImportDecl, LetStatement, SealStatement,
    FunctionDecl, RitualDecl, Parameter, ReturnStatement, StanceStatement,
    AuraStatement, PhaseStatement, AttemptStatement, ExpressionStatement,
    WaitStatement, AssignmentStatement, TypeExpr, RescueClause, SigilDecl,
    Identifier, NumberLiteral, StringLiteral, BooleanLiteral, NullLiteral,
    ArrayLiteral, RelicLiteral, BinaryExpr, UnaryExpr, CallExpr, MethodCallExpr,
    IndexExpr, InlineSpell, AwaitExpr, Range, BinaryOp
} from './ast';

export interface ParseError {
    message: string;
    range: Range;
}

export class Parser {
    private tokens: Token[] = [];
    private current: number = 0;
    public errors: ParseError[] = [];
    public lexerErrors: LexerError[] = [];

    parse(source: string): Program {
        const lexer = new Lexer(source);
        this.tokens = lexer.tokenize();
        this.lexerErrors = lexer.errors;
        this.current = 0;
        this.errors = [];

        const imports: ImportDecl[] = [];
        const statements: Statement[] = [];

        while (!this.isAtEnd()) {
            try {
                // Parse imports first
                if (this.check(TokenType.Circle)) {
                    imports.push(this.parseImport());
                } else if (this.check(TokenType.At)) {
                    // Sigil annotation - parse and attach to next declaration
                    const sigils = this.parseSigils();
                    const stmt = this.parseDeclaration(sigils);
                    if (stmt) statements.push(stmt);
                } else {
                    const stmt = this.parseStatement();
                    if (stmt) statements.push(stmt);
                }
            } catch (e) {
                this.synchronize();
            }
        }

        return {
            type: 'Program',
            imports,
            statements,
            range: {
                start: { line: 1, column: 1, offset: 0 },
                end: this.previous().range.end,
            },
        };
    }

    private parseImport(): ImportDecl {
        const start = this.advance(); // consume 'circle'
        const moduleName = this.consume(TokenType.Identifier, "Expected module name after 'circle'");

        let alias: string | undefined;
        let fromPath: string | undefined;

        if (this.match(TokenType.As)) {
            alias = this.consume(TokenType.Identifier, "Expected alias after 'as'").value;
        }

        if (this.match(TokenType.From)) {
            fromPath = this.consume(TokenType.String, "Expected path after 'from'").value;
        }

        return {
            type: 'ImportDecl',
            module: moduleName.value,
            alias,
            fromPath,
            range: { start: start.range.start, end: this.previous().range.end },
        };
    }

    private parseSigils(): string[] {
        const sigils: string[] = [];
        while (this.check(TokenType.At)) {
            this.advance(); // consume @
            const name = this.consume(TokenType.Identifier, "Expected sigil name after '@'");
            sigils.push(name.value);
        }
        return sigils;
    }

    private parseDeclaration(sigils: string[] = []): Statement | null {
        // Check if @export sigil is present
        const isExported = sigils.includes('export');

        if (this.check(TokenType.Cast)) {
            return this.parseFunctionDecl(sigils, isExported);
        }
        if (this.check(TokenType.Ritual)) {
            return this.parseRitualDecl(isExported);
        }
        if (this.check(TokenType.Seal)) {
            return this.parseSealStatement(isExported);
        }
        if (this.check(TokenType.Let)) {
            return this.parseLetStatement(isExported);
        }
        if (this.check(TokenType.Sigil)) {
            return this.parseSigilDecl();
        }
        return this.parseStatement();
    }

    private parseStatement(): Statement | null {
        // Check for export
        const isExported = this.match(TokenType.Export);

        if (this.check(TokenType.Let)) {
            return this.parseLetStatement(isExported);
        }
        if (this.check(TokenType.Seal)) {
            return this.parseSealStatement(isExported);
        }
        if (this.check(TokenType.Cast)) {
            return this.parseFunctionDecl([], isExported);
        }
        if (this.check(TokenType.Ritual)) {
            return this.parseRitualDecl(isExported);
        }
        if (this.check(TokenType.Sigil)) {
            return this.parseSigilDecl();
        }
        if (this.check(TokenType.Return)) {
            return this.parseReturnStatement();
        }
        if (this.check(TokenType.In)) {
            return this.parseStanceStatement();
        }
        if (this.check(TokenType.Invoke)) {
            return this.parseAuraStatement();
        }
        if (this.check(TokenType.Enter)) {
            return this.parsePhaseStatement();
        }
        if (this.check(TokenType.Attempt)) {
            return this.parseAttemptStatement();
        }
        if (this.check(TokenType.Wait)) {
            return this.parseWaitStatement();
        }
        // Error handling statements
        if (this.check(TokenType.Rupture)) {
            return this.parseRuptureStatement();
        }
        if (this.check(TokenType.Panic)) {
            return this.parsePanicStatement();
        }
        if (this.check(TokenType.Wound)) {
            return this.parseWoundStatement();
        }
        if (this.check(TokenType.Ward)) {
            return this.parseWardStatement();
        }
        if (this.check(TokenType.Rebound)) {
            return this.parseReboundStatement();
        }
        // Loop control statements
        if (this.check(TokenType.Break)) {
            return this.parseBreakStatement();
        }
        if (this.check(TokenType.Fracture)) {
            return this.parseFractureStatement();
        }
        if (this.check(TokenType.Shatter)) {
            return this.parseShatterStatement();
        }

        // Check for assignment or expression
        if (this.check(TokenType.Identifier)) {
            const expr = this.parseExpression();

            // Check if this is an assignment
            if (expr.type === 'Identifier' && this.check(TokenType.Equals)) {
                this.advance(); // consume '='
                const value = this.parseExpression();
                return {
                    type: 'AssignmentStatement',
                    name: (expr as Identifier).name,
                    value,
                    range: { start: expr.range.start, end: value.range.end },
                };
            }

            return {
                type: 'ExpressionStatement',
                expression: expr,
                range: expr.range,
            };
        }

        // Default: expression statement
        const expr = this.parseExpression();
        return {
            type: 'ExpressionStatement',
            expression: expr,
            range: expr.range,
        };
    }

    private parseLetStatement(isExported: boolean = false): LetStatement {
        const start = this.advance(); // consume 'let'
        const name = this.consume(TokenType.Identifier, "Expected variable name after 'let'");

        let typeAnnotation: TypeExpr | undefined;
        if (this.match(TokenType.Colon)) {
            typeAnnotation = this.parseTypeExpr();
        }

        this.consume(TokenType.Equals, "Expected '=' after variable name");
        const value = this.parseExpression();

        return {
            type: 'LetStatement',
            name: name.value,
            typeAnnotation,
            value,
            isExported,
            range: { start: start.range.start, end: value.range.end },
        };
    }

    private parseSealStatement(isExported: boolean = false): SealStatement {
        const start = this.advance(); // consume 'seal'
        const name = this.consume(TokenType.Identifier, "Expected variable name after 'seal'");

        let typeAnnotation: TypeExpr | undefined;
        if (this.match(TokenType.Colon)) {
            typeAnnotation = this.parseTypeExpr();
        }

        this.consume(TokenType.Equals, "Expected '=' after variable name");
        const value = this.parseExpression();

        return {
            type: 'SealStatement',
            name: name.value,
            typeAnnotation,
            value,
            isExported,
            range: { start: start.range.start, end: value.range.end },
        };
    }

    private parseFunctionDecl(sigils: string[] = [], isExported: boolean = false): FunctionDecl {
        const start = this.advance(); // consume 'cast'
        this.consume(TokenType.Spell, "Expected 'Spell' after 'cast'");
        const name = this.consume(TokenType.Identifier, "Expected function name");

        this.consume(TokenType.LParen, "Expected '(' after function name");
        const params = this.parseParameters();
        this.consume(TokenType.RParen, "Expected ')' after parameters");

        let returnType: TypeExpr | undefined;
        if (this.match(TokenType.Arrow)) {
            returnType = this.parseTypeExpr();
        }

        this.consume(TokenType.LBrace, "Expected '{' before function body");
        const body = this.parseBlock();
        this.consume(TokenType.RBrace, "Expected '}' after function body");

        return {
            type: 'FunctionDecl',
            name: name.value,
            params,
            returnType,
            body,
            sigils,
            isExported,
            range: { start: start.range.start, end: this.previous().range.end },
        };
    }

    private parseRitualDecl(isExported: boolean = false): RitualDecl {
        const start = this.advance(); // consume 'ritual'
        const name = this.consume(TokenType.Identifier, "Expected ritual name");

        let params: Parameter[] = [];
        if (this.check(TokenType.LParen)) {
            this.advance();
            params = this.parseParameters();
            this.consume(TokenType.RParen, "Expected ')' after parameters");
        }

        // Optional return type
        let returnType: TypeExpr | undefined;
        if (this.match(TokenType.Arrow)) {
            returnType = this.parseTypeExpr();
        }

        this.consume(TokenType.DoubleColon, "Expected '::' after ritual declaration");

        const body: Statement[] = [];
        while (!this.check(TokenType.End)) {
            if (this.isAtEnd()) {
                this.error("Expected 'end' to close ritual");
                break;
            }
            const stmt = this.parseStatement();
            if (stmt) body.push(stmt);
        }

        // Consume 'end' keyword
        if (this.check(TokenType.End)) {
            this.advance();
        }

        return {
            type: 'RitualDecl',
            name: name.value,
            params,
            returnType,
            body,
            isExported,
            range: { start: start.range.start, end: this.previous().range.end },
        };
    }

    private parseSigilDecl(): SigilDecl {
        const start = this.advance(); // consume 'sigil'
        const name = this.consume(TokenType.Identifier, "Expected sigil type name");

        this.consume(TokenType.LBrace, "Expected '{' after sigil name");

        const fields: { name: string; typeAnnotation: TypeExpr }[] = [];
        while (!this.check(TokenType.RBrace) && !this.isAtEnd()) {
            const fieldName = this.consume(TokenType.Identifier, "Expected field name");
            this.consume(TokenType.Colon, "Expected ':' after field name");
            const fieldType = this.parseTypeExpr();
            fields.push({ name: fieldName.value, typeAnnotation: fieldType });
            this.match(TokenType.Comma); // optional comma
        }

        this.consume(TokenType.RBrace, "Expected '}' after sigil fields");

        return {
            type: 'SigilDecl',
            name: name.value,
            fields,
            range: { start: start.range.start, end: this.previous().range.end },
        };
    }

    private parseParameters(): Parameter[] {
        const params: Parameter[] = [];

        if (!this.check(TokenType.RParen)) {
            do {
                // Check for typed parameter: Type name or name: Type
                let paramName: string;
                let typeAnnotation: TypeExpr | undefined;

                const first = this.advance();

                if (this.isTypeToken(first.type)) {
                    // Type-first syntax: Ember x
                    typeAnnotation = { type: 'simple', name: first.value };
                    paramName = this.consume(TokenType.Identifier, "Expected parameter name after type").value;
                } else if (first.type === TokenType.Identifier) {
                    paramName = first.value;
                    if (this.match(TokenType.Colon)) {
                        typeAnnotation = this.parseTypeExpr();
                    }
                } else {
                    this.error("Expected parameter name or type");
                    paramName = 'unknown';
                }

                params.push({ name: paramName, typeAnnotation });
            } while (this.match(TokenType.Comma));
        }

        return params;
    }

    private parseTypeExpr(): TypeExpr {
        const typeName = this.advance();

        if (this.match(TokenType.Less)) {
            // Generic type: Constellation<T> or Relic<K, V>
            const params: TypeExpr[] = [];
            do {
                params.push(this.parseTypeExpr());
            } while (this.match(TokenType.Comma));

            // Consume closing >
            if (this.check(TokenType.Greater)) {
                this.advance();
            }

            return { type: 'generic', name: typeName.value, params };
        }

        return { type: 'simple', name: typeName.value };
    }

    private parseReturnStatement(): ReturnStatement {
        const start = this.advance(); // consume 'return'

        let value: Expression | undefined;
        if (!this.check(TokenType.RBrace) && !this.isAtEnd()) {
            // Check if next token is on same line (simple heuristic)
            if (this.peek().range.start.line === start.range.start.line) {
                value = this.parseExpression();
            }
        }

        return {
            type: 'ReturnStatement',
            value,
            range: { start: start.range.start, end: value?.range.end ?? start.range.end },
        };
    }

    private parseStanceStatement(): StanceStatement {
        const start = this.advance(); // consume 'in'
        this.consume(TokenType.Stance, "Expected 'Stance' after 'in'");

        this.consume(TokenType.LParen, "Expected '(' after 'Stance'");
        const condition = this.parseExpression();
        this.consume(TokenType.RParen, "Expected ')' after condition");

        this.consume(TokenType.LBrace, "Expected '{' before body");
        const thenBranch = this.parseBlock();
        this.consume(TokenType.RBrace, "Expected '}' after body");

        const shiftBranches: { condition: Expression; body: Statement[] }[] = [];
        let abandonBranch: Statement[] | undefined;

        while (this.match(TokenType.Shift)) {
            this.consume(TokenType.Stance, "Expected 'Stance' after 'shift'");
            this.consume(TokenType.LParen, "Expected '(' after 'Stance'");
            const shiftCond = this.parseExpression();
            this.consume(TokenType.RParen, "Expected ')' after condition");
            this.consume(TokenType.LBrace, "Expected '{' before body");
            const shiftBody = this.parseBlock();
            this.consume(TokenType.RBrace, "Expected '}' after body");
            shiftBranches.push({ condition: shiftCond, body: shiftBody });
        }

        if (this.match(TokenType.Abandon)) {
            this.consume(TokenType.Stance, "Expected 'Stance' after 'abandon'");
            this.consume(TokenType.LBrace, "Expected '{' before body");
            abandonBranch = this.parseBlock();
            this.consume(TokenType.RBrace, "Expected '}' after body");
        } else if (this.match(TokenType.Otherwise)) {
            this.consume(TokenType.LBrace, "Expected '{' before body");
            abandonBranch = this.parseBlock();
            this.consume(TokenType.RBrace, "Expected '}' after body");
        }

        return {
            type: 'StanceStatement',
            condition,
            thenBranch,
            shiftBranches,
            abandonBranch,
            range: { start: start.range.start, end: this.previous().range.end },
        };
    }

    private parseAuraStatement(): AuraStatement {
        const start = this.advance(); // consume 'invoke'
        this.consume(TokenType.Aura, "Expected 'Aura' after 'invoke'");
        const value = this.parseExpression();
        this.consume(TokenType.LBrace, "Expected '{' after Aura value");

        const cases: { pattern: Expression; body: Statement[] }[] = [];
        let otherwise: Statement[] | undefined;

        while (!this.check(TokenType.RBrace) && !this.isAtEnd()) {
            if (this.match(TokenType.When)) {
                const pattern = this.parseExpression();
                this.consume(TokenType.Arrow, "Expected '->' after pattern");

                let body: Statement[];
                if (this.check(TokenType.LBrace)) {
                    this.advance();
                    body = this.parseBlock();
                    this.consume(TokenType.RBrace, "Expected '}' after case body");
                } else {
                    const expr = this.parseExpression();
                    body = [{ type: 'ExpressionStatement', expression: expr, range: expr.range }];
                }
                cases.push({ pattern, body });
            } else if (this.match(TokenType.Otherwise)) {
                this.consume(TokenType.Arrow, "Expected '->' after 'otherwise'");
                if (this.check(TokenType.LBrace)) {
                    this.advance();
                    otherwise = this.parseBlock();
                    this.consume(TokenType.RBrace, "Expected '}' after otherwise body");
                } else {
                    const expr = this.parseExpression();
                    otherwise = [{ type: 'ExpressionStatement', expression: expr, range: expr.range }];
                }
            } else {
                break;
            }
        }

        this.consume(TokenType.RBrace, "Expected '}' after Aura cases");

        return {
            type: 'AuraStatement',
            value,
            cases,
            otherwise,
            range: { start: start.range.start, end: this.previous().range.end },
        };
    }

    private parsePhaseStatement(): PhaseStatement {
        const start = this.advance(); // consume 'enter'
        this.consume(TokenType.Phase, "Expected 'Phase' after 'enter'");

        let kind: PhaseStatement['kind'];

        if (this.match(TokenType.Forever)) {
            kind = { type: 'forever' };
        } else if (this.match(TokenType.Until)) {
            this.consume(TokenType.LParen, "Expected '(' after 'until'");
            const condition = this.parseExpression();
            this.consume(TokenType.RParen, "Expected ')' after condition");
            kind = { type: 'until', condition };
        } else {
            // Count or forEach
            const variable = this.consume(TokenType.Identifier, "Expected loop variable");

            if (this.match(TokenType.From)) {
                const from = this.parseExpression();
                this.consume(TokenType.To, "Expected 'to' after 'from' value");
                const to = this.parseExpression();
                kind = { type: 'count', variable: variable.value, from, to };
            } else if (this.match(TokenType.In)) {
                const collection = this.parseExpression();
                kind = { type: 'forEach', variable: variable.value, collection };
            } else {
                this.error("Expected 'from' or 'in' after loop variable");
                kind = { type: 'forever' };
            }
        }

        this.consume(TokenType.LBrace, "Expected '{' before loop body");
        const body = this.parseBlock();
        this.consume(TokenType.RBrace, "Expected '}' after loop body");

        return {
            type: 'PhaseStatement',
            kind,
            body,
            range: { start: start.range.start, end: this.previous().range.end },
        };
    }

    private parseAttemptStatement(): AttemptStatement {
        const start = this.advance(); // consume 'attempt'
        this.consume(TokenType.LBrace, "Expected '{' after 'attempt'");
        const body = this.parseBlock();
        this.consume(TokenType.RBrace, "Expected '}' after attempt body");

        const rescueClauses: RescueClause[] = [];
        let finallyBlock: Statement[] | undefined;

        while (this.match(TokenType.Rescue)) {
            let errorType: string | undefined;
            let binding: string | undefined;

            if (this.isErrorTypeToken(this.peek().type)) {
                errorType = this.advance().value;
            }

            if (this.match(TokenType.As)) {
                binding = this.consume(TokenType.Identifier, "Expected binding name after 'as'").value;
            }

            this.consume(TokenType.LBrace, "Expected '{' after rescue clause");
            const rescueBody = this.parseBlock();
            this.consume(TokenType.RBrace, "Expected '}' after rescue body");

            rescueClauses.push({ errorType, binding, body: rescueBody });
        }

        if (this.match(TokenType.Finally)) {
            this.consume(TokenType.LBrace, "Expected '{' after 'finally'");
            finallyBlock = this.parseBlock();
            this.consume(TokenType.RBrace, "Expected '}' after finally body");
        }

        return {
            type: 'AttemptStatement',
            body,
            rescueClauses,
            finallyBlock,
            range: { start: start.range.start, end: this.previous().range.end },
        };
    }

    private parseWaitStatement(): WaitStatement {
        const start = this.advance(); // consume 'wait'
        const duration = this.parseExpression();

        // Parse unit (ms, s, m)
        let unit = 'ms';
        if (this.check(TokenType.Identifier)) {
            const unitToken = this.peek();
            if (['ms', 's', 'm'].includes(unitToken.value)) {
                unit = this.advance().value;
            }
        }

        return {
            type: 'WaitStatement',
            duration,
            unit,
            range: { start: start.range.start, end: this.previous().range.end },
        };
    }

    private parseBlock(): Statement[] {
        const statements: Statement[] = [];
        while (!this.check(TokenType.RBrace) && !this.isAtEnd()) {
            const stmt = this.parseStatement();
            if (stmt) statements.push(stmt);
        }
        return statements;
    }

    // Expression parsing with precedence
    private parseExpression(): Expression {
        return this.parseOr();
    }

    private parseOr(): Expression {
        let left = this.parseAnd();

        while (this.match(TokenType.Either)) {
            const right = this.parseAnd();
            left = {
                type: 'BinaryExpr',
                operator: 'either!' as BinaryOp,
                left,
                right,
                range: { start: left.range.start, end: right.range.end },
            };
        }

        return left;
    }

    private parseAnd(): Expression {
        let left = this.parseEquality();

        while (this.match(TokenType.Both)) {
            const right = this.parseEquality();
            left = {
                type: 'BinaryExpr',
                operator: 'both!' as BinaryOp,
                left,
                right,
                range: { start: left.range.start, end: right.range.end },
            };
        }

        return left;
    }

    private parseEquality(): Expression {
        let left = this.parseComparison();

        while (this.match(TokenType.IsEqual) || this.match(TokenType.NotEqual)) {
            const op = this.previous().value as BinaryOp;
            const right = this.parseComparison();
            left = {
                type: 'BinaryExpr',
                operator: op,
                left,
                right,
                range: { start: left.range.start, end: right.range.end },
            };
        }

        return left;
    }

    private parseComparison(): Expression {
        let left = this.parseTerm();

        while (this.match(TokenType.Greater) || this.match(TokenType.Less) ||
            this.match(TokenType.GreaterEq) || this.match(TokenType.LessEq)) {
            const op = this.previous().value as BinaryOp;
            const right = this.parseTerm();
            left = {
                type: 'BinaryExpr',
                operator: op,
                left,
                right,
                range: { start: left.range.start, end: right.range.end },
            };
        }

        return left;
    }

    private parseTerm(): Expression {
        let left = this.parseFactor();

        while (this.match(TokenType.Plus) || this.match(TokenType.Minus)) {
            const op = this.previous().value as BinaryOp;
            const right = this.parseFactor();
            left = {
                type: 'BinaryExpr',
                operator: op,
                left,
                right,
                range: { start: left.range.start, end: right.range.end },
            };
        }

        return left;
    }

    private parseFactor(): Expression {
        let left = this.parseUnary();

        while (this.match(TokenType.Star) || this.match(TokenType.Slash) || this.match(TokenType.Percent)) {
            const op = this.previous().value as BinaryOp;
            const right = this.parseUnary();
            left = {
                type: 'BinaryExpr',
                operator: op,
                left,
                right,
                range: { start: left.range.start, end: right.range.end },
            };
        }

        return left;
    }

    private parseUnary(): Expression {
        if (this.match(TokenType.Negate) || this.match(TokenType.Minus)) {
            const op = this.previous();
            const operand = this.parseUnary();
            return {
                type: 'UnaryExpr',
                operator: op.value === 'negate!' ? 'negate!' : '-',
                operand,
                range: { start: op.range.start, end: operand.range.end },
            };
        }

        return this.parseCall();
    }

    private parseCall(): Expression {
        let expr = this.parsePrimary();

        while (true) {
            if (this.match(TokenType.LParen)) {
                const args = this.parseArguments();
                this.consume(TokenType.RParen, "Expected ')' after arguments");
                expr = {
                    type: 'CallExpr',
                    callee: expr,
                    arguments: args,
                    range: { start: expr.range.start, end: this.previous().range.end },
                };
            } else if (this.match(TokenType.Dot)) {
                const name = this.consume(TokenType.Identifier, "Expected method or property name after '.'");

                if (this.match(TokenType.LParen)) {
                    const args = this.parseArguments();
                    this.consume(TokenType.RParen, "Expected ')' after arguments");
                    expr = {
                        type: 'MethodCallExpr',
                        object: expr,
                        method: name.value,
                        arguments: args,
                        range: { start: expr.range.start, end: this.previous().range.end },
                    };
                } else {
                    expr = {
                        type: 'MemberExpr',
                        object: expr,
                        property: name.value,
                        range: { start: expr.range.start, end: name.range.end },
                    };
                }
            } else if (this.match(TokenType.LBracket)) {
                const index = this.parseExpression();
                this.consume(TokenType.RBracket, "Expected ']' after index");
                expr = {
                    type: 'IndexExpr',
                    object: expr,
                    index,
                    range: { start: expr.range.start, end: this.previous().range.end },
                };
            } else {
                break;
            }
        }

        return expr;
    }

    private parseArguments(): Expression[] {
        const args: Expression[] = [];
        if (!this.check(TokenType.RParen)) {
            do {
                args.push(this.parseExpression());
            } while (this.match(TokenType.Comma));
        }
        return args;
    }

    private parsePrimary(): Expression {
        // Await expression
        if (this.match(TokenType.Await)) {
            const start = this.previous();
            const expr = this.parseExpression();
            return {
                type: 'AwaitExpr',
                expression: expr,
                range: { start: start.range.start, end: expr.range.end },
            };
        }

        // Inline spell: cast Spell x -> x * 2 or cast Spell (a, b) { ... }
        if (this.check(TokenType.Cast)) {
            return this.parseInlineSpell();
        }

        // Literals
        if (this.match(TokenType.Number)) {
            const token = this.previous();
            return {
                type: 'NumberLiteral',
                value: parseFloat(token.value),
                range: token.range,
            };
        }

        if (this.match(TokenType.String)) {
            const token = this.previous();
            return {
                type: 'StringLiteral',
                value: token.value,
                isTemplate: false,
                range: token.range,
            };
        }

        if (this.match(TokenType.Boolean) || this.match(TokenType.Both) || this.match(TokenType.Either)) {
            const token = this.previous();
            return {
                type: 'BooleanLiteral',
                value: token.value === 'true' || token.type === TokenType.Both,
                range: token.range,
            };
        }

        if (this.match(TokenType.Null)) {
            const token = this.previous();
            return {
                type: 'NullLiteral',
                range: token.range,
            };
        }

        // Array literal
        if (this.match(TokenType.LBracket)) {
            const start = this.previous();
            const elements: Expression[] = [];
            if (!this.check(TokenType.RBracket)) {
                do {
                    elements.push(this.parseExpression());
                } while (this.match(TokenType.Comma));
            }
            this.consume(TokenType.RBracket, "Expected ']' after array elements");
            return {
                type: 'ArrayLiteral',
                elements,
                range: { start: start.range.start, end: this.previous().range.end },
            };
        }

        // Relic (object) literal
        if (this.match(TokenType.LBrace)) {
            const start = this.previous();
            const entries: { key: string; value: Expression }[] = [];

            if (!this.check(TokenType.RBrace)) {
                do {
                    const key = this.consume(TokenType.String, "Expected string key in Relic").value;
                    this.consume(TokenType.Colon, "Expected ':' after key");
                    const value = this.parseExpression();
                    entries.push({ key, value });
                } while (this.match(TokenType.Comma));
            }

            this.consume(TokenType.RBrace, "Expected '}' after Relic entries");
            return {
                type: 'RelicLiteral',
                entries,
                range: { start: start.range.start, end: this.previous().range.end },
            };
        }

        // Grouped expression
        if (this.match(TokenType.LParen)) {
            const expr = this.parseExpression();
            this.consume(TokenType.RParen, "Expected ')' after expression");
            return expr;
        }

        // Identifier
        if (this.match(TokenType.Identifier)) {
            const token = this.previous();
            return {
                type: 'Identifier',
                name: token.value,
                range: token.range,
            };
        }

        // Error fallback
        const token = this.peek();
        this.error(`Unexpected token '${token.value}'`);
        this.advance();
        return {
            type: 'NullLiteral',
            range: token.range,
        };
    }

    private parseInlineSpell(): InlineSpell {
        const start = this.advance(); // consume 'cast'
        this.consume(TokenType.Spell, "Expected 'Spell' after 'cast'");

        let params: string[] = [];
        let paramTypes: (TypeExpr | undefined)[] = [];

        // Parse parameters
        if (this.check(TokenType.LParen)) {
            this.advance();
            if (!this.check(TokenType.RParen)) {
                do {
                    const param = this.consume(TokenType.Identifier, "Expected parameter name");
                    params.push(param.value);
                    paramTypes.push(undefined);
                } while (this.match(TokenType.Comma));
            }
            this.consume(TokenType.RParen, "Expected ')' after parameters");
        } else if (this.check(TokenType.Identifier)) {
            // Single parameter without parens: cast Spell x -> ...
            const param = this.advance();
            params.push(param.value);
            paramTypes.push(undefined);
        }

        // Expression body with arrow or block body
        if (this.match(TokenType.Arrow)) {
            const body = this.parseExpression();
            return {
                type: 'InlineSpell',
                params,
                paramTypes,
                body,
                isExpression: true,
                range: { start: start.range.start, end: body.range.end },
            };
        } else if (this.check(TokenType.LBrace)) {
            this.advance();
            const body = this.parseBlock();
            this.consume(TokenType.RBrace, "Expected '}' after inline spell body");
            return {
                type: 'InlineSpell',
                params,
                paramTypes,
                body,
                isExpression: false,
                range: { start: start.range.start, end: this.previous().range.end },
            };
        }

        this.error("Expected '->' or '{' after inline spell parameters");
        return {
            type: 'InlineSpell',
            params,
            paramTypes,
            body: { type: 'NullLiteral', range: start.range },
            isExpression: true,
            range: start.range,
        };
    }

    // Helper methods
    // Error handling statement parsers
    private parseRuptureStatement(): Statement {
        const start = this.advance(); // consume 'rupture'
        // Get error type (Rift, Glitch, VoidTear, Spirit)
        let errorType: string | undefined;
        if (this.isErrorTypeToken(this.peek().type)) {
            errorType = this.advance().value;
        }
        const message = this.parseExpression();
        return {
            type: 'ExpressionStatement',
            expression: {
                type: 'CallExpr',
                callee: { type: 'Identifier', name: 'rupture', range: start.range },
                arguments: [message],
                range: { start: start.range.start, end: message.range.end },
            },
            range: { start: start.range.start, end: message.range.end },
        };
    }

    private parsePanicStatement(): Statement {
        const start = this.advance(); // consume 'panic'
        const message = this.parseExpression();
        return {
            type: 'ExpressionStatement',
            expression: {
                type: 'CallExpr',
                callee: { type: 'Identifier', name: 'panic', range: start.range },
                arguments: [message],
                range: { start: start.range.start, end: message.range.end },
            },
            range: { start: start.range.start, end: message.range.end },
        };
    }

    private parseWoundStatement(): Statement {
        const start = this.advance(); // consume 'wound'
        const message = this.parseExpression();
        return {
            type: 'ExpressionStatement',
            expression: {
                type: 'CallExpr',
                callee: { type: 'Identifier', name: 'wound', range: start.range },
                arguments: [message],
                range: { start: start.range.start, end: message.range.end },
            },
            range: { start: start.range.start, end: message.range.end },
        };
    }

    private parseWardStatement(): Statement {
        const start = this.advance(); // consume 'ward'
        this.consume(TokenType.LBrace, "Expected '{' after 'ward'");
        const body = this.parseBlock();
        this.consume(TokenType.RBrace, "Expected '}' after ward body");
        return {
            type: 'ExpressionStatement',
            expression: {
                type: 'CallExpr',
                callee: { type: 'Identifier', name: 'ward', range: start.range },
                arguments: [],
                range: { start: start.range.start, end: this.previous().range.end },
            },
            range: { start: start.range.start, end: this.previous().range.end },
        };
    }

    private parseReboundStatement(): Statement {
        const start = this.advance(); // consume 'rebound'
        let value: Expression | undefined;
        // Check if there's a value to rethrow
        if (!this.check(TokenType.RBrace) && !this.isAtEnd() &&
            this.peek().range.start.line === start.range.start.line) {
            value = this.parseExpression();
        }
        return {
            type: 'ExpressionStatement',
            expression: {
                type: 'CallExpr',
                callee: { type: 'Identifier', name: 'rebound', range: start.range },
                arguments: value ? [value] : [],
                range: { start: start.range.start, end: value?.range.end ?? start.range.end },
            },
            range: { start: start.range.start, end: value?.range.end ?? start.range.end },
        };
    }

    private parseBreakStatement(): Statement {
        const start = this.advance(); // consume 'break'
        // Expect 'seal' identifier after break
        if (this.check(TokenType.Identifier) && this.peek().value === 'seal') {
            this.advance();
        }
        return {
            type: 'ExpressionStatement',
            expression: {
                type: 'Identifier',
                name: 'break',
                range: { start: start.range.start, end: this.previous().range.end },
            },
            range: { start: start.range.start, end: this.previous().range.end },
        };
    }

    private parseFractureStatement(): Statement {
        const start = this.advance(); // consume 'fracture'
        // Expect 'seal' identifier after fracture
        if (this.check(TokenType.Identifier) && this.peek().value === 'seal') {
            this.advance();
        }
        return {
            type: 'ExpressionStatement',
            expression: {
                type: 'Identifier',
                name: 'continue',
                range: { start: start.range.start, end: this.previous().range.end },
            },
            range: { start: start.range.start, end: this.previous().range.end },
        };
    }

    private parseShatterStatement(): Statement {
        const start = this.advance(); // consume 'shatter'
        // Expect 'grand_seal' after shatter
        if (this.check(TokenType.GrandSeal) || (this.check(TokenType.Identifier) && this.peek().value === 'grand_seal')) {
            this.advance();
        }
        let value: Expression | undefined;
        if (!this.check(TokenType.RBrace) && !this.isAtEnd()) {
            value = this.parseExpression();
        }
        return {
            type: 'ReturnStatement',
            value,
            range: { start: start.range.start, end: value?.range.end ?? this.previous().range.end },
        };
    }

    private isTypeToken(type: TokenType): boolean {
        return [
            TokenType.Ember, TokenType.Silk, TokenType.Pulse,
            TokenType.Flux, TokenType.Hollow, TokenType.Constellation,
            TokenType.Relic, TokenType.Spell
        ].includes(type);
    }

    private isErrorTypeToken(type: TokenType): boolean {
        return [
            TokenType.Rift, TokenType.Glitch, TokenType.Spirit, TokenType.VoidTear
        ].includes(type);
    }

    private check(type: TokenType): boolean {
        if (this.isAtEnd()) return false;
        return this.peek().type === type;
    }

    private match(...types: TokenType[]): boolean {
        for (const type of types) {
            if (this.check(type)) {
                this.advance();
                return true;
            }
        }
        return false;
    }

    private advance(): Token {
        if (!this.isAtEnd()) this.current++;
        return this.previous();
    }

    private consume(type: TokenType, message: string): Token {
        if (this.check(type)) return this.advance();

        this.error(message);
        return this.peek();
    }

    private peek(): Token {
        return this.tokens[this.current];
    }

    private previous(): Token {
        return this.tokens[this.current - 1];
    }

    private isAtEnd(): boolean {
        return this.peek().type === TokenType.EOF;
    }

    private error(message: string): void {
        const token = this.peek();
        this.errors.push({
            message,
            range: token.range,
        });
    }

    private synchronize(): void {
        this.advance();

        while (!this.isAtEnd()) {
            // Skip to next statement boundary
            switch (this.peek().type) {
                case TokenType.Let:
                case TokenType.Seal:
                case TokenType.Cast:
                case TokenType.Ritual:
                case TokenType.Circle:
                case TokenType.In:
                case TokenType.Enter:
                case TokenType.Invoke:
                case TokenType.Attempt:
                case TokenType.Return:
                    return;
            }
            this.advance();
        }
    }
}
