// FlowLang Lexer - Tokenizer

import { Position, Range } from './ast';

export enum TokenType {
    // Literals
    Number = 'Number',
    String = 'String',
    Boolean = 'Boolean',
    Null = 'Null',

    // Identifiers
    Identifier = 'Identifier',

    // Keywords
    Let = 'Let',
    Seal = 'Seal',
    Cast = 'Cast',
    Spell = 'Spell',
    Ritual = 'Ritual',
    Circle = 'Circle',
    From = 'From',
    As = 'As',
    In = 'In',
    Stance = 'Stance',
    Shift = 'Shift',
    Abandon = 'Abandon',
    Otherwise = 'Otherwise',
    Invoke = 'Invoke',
    Aura = 'Aura',
    When = 'When',
    Enter = 'Enter',
    Phase = 'Phase',
    To = 'To',
    Until = 'Until',
    Forever = 'Forever',
    Return = 'Return',
    Attempt = 'Attempt',
    Rescue = 'Rescue',
    Rupture = 'Rupture',
    Ward = 'Ward',
    Rebound = 'Rebound',
    Finally = 'Finally',
    Await = 'Await',
    Wait = 'Wait',
    Perform = 'Perform',
    Sigil = 'Sigil',
    Export = 'Export',
    End = 'End',
    // New error control keywords
    Panic = 'Panic',
    Wound = 'Wound',
    Break = 'Break',
    Fracture = 'Fracture',
    Shatter = 'Shatter',
    GrandSeal = 'GrandSeal',
    Retry = 'Retry',

    // Types (Essences)
    Ember = 'Ember',
    Silk = 'Silk',
    Pulse = 'Pulse',
    Flux = 'Flux',
    Hollow = 'Hollow',
    Constellation = 'Constellation',
    Relic = 'Relic',

    // Error types
    Rift = 'Rift',
    Glitch = 'Glitch',
    Spirit = 'Spirit',
    VoidTear = 'VoidTear',

    // Operators
    Plus = 'Plus',
    Minus = 'Minus',
    Star = 'Star',
    Slash = 'Slash',
    Percent = 'Percent',
    Greater = 'Greater',        // >>
    Less = 'Less',              // <<
    GreaterEq = 'GreaterEq',    // >>=
    LessEq = 'LessEq',          // <<=
    IsEqual = 'IsEqual',        // is~
    NotEqual = 'NotEqual',      // not~
    Both = 'Both',              // both!
    Either = 'Either',          // either!
    Negate = 'Negate',          // negate!
    Arrow = 'Arrow',            // ->
    FatArrow = 'FatArrow',      // =>
    DoubleColon = 'DoubleColon', // ::

    // Punctuation
    LParen = 'LParen',
    RParen = 'RParen',
    LBrace = 'LBrace',
    RBrace = 'RBrace',
    LBracket = 'LBracket',
    RBracket = 'RBracket',
    Comma = 'Comma',
    Colon = 'Colon',
    Dot = 'Dot',
    At = 'At',
    Equals = 'Equals',

    // Special
    EOF = 'EOF',
    Error = 'Error',
}

export interface Token {
    type: TokenType;
    value: string;
    range: Range;
}

const KEYWORDS: Record<string, TokenType> = {
    'let': TokenType.Let,
    'seal': TokenType.Seal,
    'cast': TokenType.Cast,
    'Spell': TokenType.Spell,
    'ritual': TokenType.Ritual,
    'circle': TokenType.Circle,
    'from': TokenType.From,
    'as': TokenType.As,
    'in': TokenType.In,
    'Stance': TokenType.Stance,
    'shift': TokenType.Shift,
    'abandon': TokenType.Abandon,
    'otherwise': TokenType.Otherwise,
    'invoke': TokenType.Invoke,
    'Aura': TokenType.Aura,
    'when': TokenType.When,
    'enter': TokenType.Enter,
    'Phase': TokenType.Phase,
    'to': TokenType.To,
    'until': TokenType.Until,
    'forever': TokenType.Forever,
    'return': TokenType.Return,
    'attempt': TokenType.Attempt,
    'rescue': TokenType.Rescue,
    'rupture': TokenType.Rupture,
    'ward': TokenType.Ward,
    'rebound': TokenType.Rebound,
    'finally': TokenType.Finally,
    'await': TokenType.Await,
    'wait': TokenType.Wait,
    'perform': TokenType.Perform,
    'sigil': TokenType.Sigil,
    'void': TokenType.Null,
    'end': TokenType.End,
    // New keywords
    'panic': TokenType.Panic,
    'wound': TokenType.Wound,
    'break': TokenType.Break,
    'fracture': TokenType.Fracture,
    'shatter': TokenType.Shatter,
    'grand_seal': TokenType.GrandSeal,
    'retry': TokenType.Retry,
    // Types
    'Ember': TokenType.Ember,
    'Silk': TokenType.Silk,
    'Pulse': TokenType.Pulse,
    'Flux': TokenType.Flux,
    'Hollow': TokenType.Hollow,
    'Constellation': TokenType.Constellation,
    'Relic': TokenType.Relic,
    // Error types
    'Rift': TokenType.Rift,
    'Glitch': TokenType.Glitch,
    'Spirit': TokenType.Spirit,
    'VoidTear': TokenType.VoidTear,
};

export interface LexerError {
    message: string;
    range: Range;
}

export class Lexer {
    private source: string;
    private pos: number = 0;
    private line: number = 1;
    private column: number = 1;
    private tokens: Token[] = [];
    public errors: LexerError[] = [];

    constructor(source: string) {
        this.source = source;
    }

    tokenize(): Token[] {
        while (!this.isAtEnd()) {
            this.scanToken();
        }

        this.tokens.push({
            type: TokenType.EOF,
            value: '',
            range: this.makeRange(this.pos, this.pos),
        });

        return this.tokens;
    }

    private scanToken(): void {
        this.skipWhitespace();
        if (this.isAtEnd()) return;

        const start = this.pos;
        const startLine = this.line;
        const startColumn = this.column;
        const char = this.advance();

        // Comments
        if (char === '-' && this.peek() === '-') {
            this.skipLineComment();
            return;
        }
        if (char === '/' && this.peek() === '*') {
            this.skipBlockComment();
            return;
        }

        // Numbers
        if (this.isDigit(char) || (char === '-' && this.isDigit(this.peek()))) {
            this.pos--; this.column--;
            this.scanNumber();
            return;
        }

        // Strings
        if (char === '"' || char === "'" || char === '`') {
            this.scanString(char);
            return;
        }

        // Identifiers and keywords
        if (this.isAlpha(char)) {
            this.pos--; this.column--;
            this.scanIdentifier();
            return;
        }

        // Sigils (decorators)
        if (char === '@') {
            this.addToken(TokenType.At, '@', start, startLine, startColumn);
            return;
        }

        // Operators and punctuation
        switch (char) {
            case '+':
                this.addToken(TokenType.Plus, '+', start, startLine, startColumn);
                break;
            case '-':
                if (this.peek() === '>') {
                    this.advance();
                    this.addToken(TokenType.Arrow, '->', start, startLine, startColumn);
                } else {
                    this.addToken(TokenType.Minus, '-', start, startLine, startColumn);
                }
                break;
            case '*':
                this.addToken(TokenType.Star, '*', start, startLine, startColumn);
                break;
            case '/':
                this.addToken(TokenType.Slash, '/', start, startLine, startColumn);
                break;
            case '%':
                this.addToken(TokenType.Percent, '%', start, startLine, startColumn);
                break;
            case '>':
                if (this.peek() === '>') {
                    this.advance();
                    if (this.peek() === '=') {
                        this.advance();
                        this.addToken(TokenType.GreaterEq, '>>=', start, startLine, startColumn);
                    } else {
                        this.addToken(TokenType.Greater, '>>', start, startLine, startColumn);
                    }
                } else {
                    // Allow single > for generics (closing angle bracket)
                    this.addToken(TokenType.Greater, '>', start, startLine, startColumn);
                }
                break;
            case '<':
                if (this.peek() === '<') {
                    this.advance();
                    if (this.peek() === '=') {
                        this.advance();
                        this.addToken(TokenType.LessEq, '<<=', start, startLine, startColumn);
                    } else {
                        this.addToken(TokenType.Less, '<<', start, startLine, startColumn);
                    }
                } else {
                    // Allow single < for generics
                    this.addToken(TokenType.Less, '<', start, startLine, startColumn);
                }
                break;
            case '=':
                if (this.peek() === '>') {
                    this.advance();
                    this.addToken(TokenType.FatArrow, '=>', start, startLine, startColumn);
                } else {
                    this.addToken(TokenType.Equals, '=', start, startLine, startColumn);
                }
                break;
            case '(':
                this.addToken(TokenType.LParen, '(', start, startLine, startColumn);
                break;
            case ')':
                this.addToken(TokenType.RParen, ')', start, startLine, startColumn);
                break;
            case '{':
                this.addToken(TokenType.LBrace, '{', start, startLine, startColumn);
                break;
            case '}':
                this.addToken(TokenType.RBrace, '}', start, startLine, startColumn);
                break;
            case '[':
                this.addToken(TokenType.LBracket, '[', start, startLine, startColumn);
                break;
            case ']':
                this.addToken(TokenType.RBracket, ']', start, startLine, startColumn);
                break;
            case ',':
                this.addToken(TokenType.Comma, ',', start, startLine, startColumn);
                break;
            case ':':
                if (this.peek() === ':') {
                    this.advance();
                    this.addToken(TokenType.DoubleColon, '::', start, startLine, startColumn);
                } else {
                    this.addToken(TokenType.Colon, ':', start, startLine, startColumn);
                }
                break;
            case '.':
                this.addToken(TokenType.Dot, '.', start, startLine, startColumn);
                break;
            default:
                this.addError(`Unexpected character '${char}'`, start, startLine, startColumn);
        }
    }

    private scanNumber(): void {
        const start = this.pos;
        const startLine = this.line;
        const startColumn = this.column;

        if (this.peek() === '-') {
            this.advance();
        }

        while (this.isDigit(this.peek())) {
            this.advance();
        }

        if (this.peek() === '.' && this.isDigit(this.peekNext())) {
            this.advance(); // consume '.'
            while (this.isDigit(this.peek())) {
                this.advance();
            }
        }

        const value = this.source.substring(start, this.pos);
        this.addToken(TokenType.Number, value, start, startLine, startColumn);
    }

    private scanString(quote: string): void {
        const start = this.pos - 1; // include opening quote
        const startLine = this.line;
        const startColumn = this.column - 1;
        let value = '';

        while (!this.isAtEnd() && this.peek() !== quote) {
            if (this.peek() === '\n') {
                this.line++;
                this.column = 1;
            }
            if (this.peek() === '\\' && this.peekNext()) {
                this.advance(); // consume backslash
                const escaped = this.advance();
                switch (escaped) {
                    case 'n': value += '\n'; break;
                    case 't': value += '\t'; break;
                    case 'r': value += '\r'; break;
                    case '\\': value += '\\'; break;
                    case '"': value += '"'; break;
                    case "'": value += "'"; break;
                    case '`': value += '`'; break;
                    default: value += escaped;
                }
            } else {
                value += this.advance();
            }
        }

        if (this.isAtEnd()) {
            this.addError('Unterminated string', start, startLine, startColumn);
            return;
        }

        this.advance(); // consume closing quote
        this.addToken(TokenType.String, value, start, startLine, startColumn);
    }

    private scanIdentifier(): void {
        const start = this.pos;
        const startLine = this.line;
        const startColumn = this.column;

        while (this.isAlphaNumeric(this.peek())) {
            this.advance();
        }

        // Check for special operators
        if (this.peek() === '~' || this.peek() === '!') {
            const word = this.source.substring(start, this.pos);
            if (word === 'is' && this.peek() === '~') {
                this.advance();
                this.addToken(TokenType.IsEqual, 'is~', start, startLine, startColumn);
                return;
            }
            if (word === 'not' && this.peek() === '~') {
                this.advance();
                this.addToken(TokenType.NotEqual, 'not~', start, startLine, startColumn);
                return;
            }
            if (word === 'both' && this.peek() === '!') {
                this.advance();
                this.addToken(TokenType.Both, 'both!', start, startLine, startColumn);
                return;
            }
            if (word === 'either' && this.peek() === '!') {
                this.advance();
                this.addToken(TokenType.Either, 'either!', start, startLine, startColumn);
                return;
            }
            if (word === 'negate' && this.peek() === '!') {
                this.advance();
                this.addToken(TokenType.Negate, 'negate!', start, startLine, startColumn);
                return;
            }
        }

        const word = this.source.substring(start, this.pos);
        const tokenType = KEYWORDS[word] || TokenType.Identifier;
        this.addToken(tokenType, word, start, startLine, startColumn);
    }

    private skipWhitespace(): void {
        while (!this.isAtEnd()) {
            const char = this.peek();
            if (char === ' ' || char === '\r' || char === '\t') {
                this.advance();
            } else if (char === '\n') {
                this.line++;
                this.column = 1;
                this.pos++;
            } else {
                break;
            }
        }
    }

    private skipLineComment(): void {
        while (!this.isAtEnd() && this.peek() !== '\n') {
            this.advance();
        }
    }

    private skipBlockComment(): void {
        this.advance(); // consume *
        while (!this.isAtEnd()) {
            if (this.peek() === '*' && this.peekNext() === '/') {
                this.advance();
                this.advance();
                return;
            }
            if (this.peek() === '\n') {
                this.line++;
                this.column = 1;
            }
            this.advance();
        }
        this.addError('Unterminated block comment', this.pos, this.line, this.column);
    }

    private isAtEnd(): boolean {
        return this.pos >= this.source.length;
    }

    private peek(): string {
        if (this.isAtEnd()) return '\0';
        return this.source[this.pos];
    }

    private peekNext(): string {
        if (this.pos + 1 >= this.source.length) return '\0';
        return this.source[this.pos + 1];
    }

    private advance(): string {
        const char = this.source[this.pos];
        this.pos++;
        this.column++;
        return char;
    }

    private isDigit(char: string): boolean {
        return char >= '0' && char <= '9';
    }

    private isAlpha(char: string): boolean {
        return (char >= 'a' && char <= 'z') ||
            (char >= 'A' && char <= 'Z') ||
            char === '_';
    }

    private isAlphaNumeric(char: string): boolean {
        return this.isAlpha(char) || this.isDigit(char);
    }

    private makeRange(start: number, end: number): Range {
        const startPos = this.offsetToPosition(start);
        const endPos = this.offsetToPosition(end);
        return { start: startPos, end: endPos };
    }

    private offsetToPosition(offset: number): Position {
        let line = 1;
        let column = 1;
        for (let i = 0; i < offset && i < this.source.length; i++) {
            if (this.source[i] === '\n') {
                line++;
                column = 1;
            } else {
                column++;
            }
        }
        return { line, column, offset };
    }

    private addToken(type: TokenType, value: string, start: number, startLine: number, startColumn: number): void {
        this.tokens.push({
            type,
            value,
            range: {
                start: { line: startLine, column: startColumn, offset: start },
                end: { line: this.line, column: this.column, offset: this.pos },
            },
        });
    }

    private addError(message: string, start: number, startLine: number, startColumn: number): void {
        this.errors.push({
            message,
            range: {
                start: { line: startLine, column: startColumn, offset: start },
                end: { line: this.line, column: this.column, offset: this.pos },
            },
        });
    }
}
