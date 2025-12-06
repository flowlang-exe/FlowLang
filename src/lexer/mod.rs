pub mod token;

use token::{Token, TokenKind};
use crate::error::FlowError;

pub struct Lexer {
    source: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
    interpolation_stack: Vec<usize>, // Tracks brace depth where interpolation started
    brace_depth: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
            interpolation_stack: Vec::new(),
            brace_depth: 0,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, FlowError> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace();
            
            if self.is_at_end() {
                break;
            }
            
            // Skip comments
            if self.peek() == '-' && self.peek_next() == '-' {
                self.skip_line_comment();
                continue;
            }
            
            if self.peek() == '/' && self.peek_next() == '*' {
                self.skip_block_comment()?;
                continue;
            }
            
            self.scan_token(&mut tokens)?;
        }
        
        tokens.push(Token::new(TokenKind::Eof, String::new(), self.line, self.column));
        Ok(tokens)
    }
    
    fn scan_token(&mut self, tokens: &mut Vec<Token>) -> Result<(), FlowError> {
        let start_line = self.line;
        let start_column = self.column;
        let c = self.advance();
        
        match c {
            // Single character tokens
            '(' => tokens.push(Token::new(TokenKind::LeftParen, c.to_string(), start_line, start_column)),
            ')' => tokens.push(Token::new(TokenKind::RightParen, c.to_string(), start_line, start_column)),
            '{' => {
                self.brace_depth += 1;
                tokens.push(Token::new(TokenKind::LeftBrace, c.to_string(), start_line, start_column));
            }
            '}' => {
                if self.brace_depth > 0 {
                    self.brace_depth -= 1;
                }
                tokens.push(Token::new(TokenKind::RightBrace, c.to_string(), start_line, start_column));
                
                // Check if we are resuming template literal interpolation
                if let Some(&depth) = self.interpolation_stack.last() {
                    if self.brace_depth == depth {
                        self.interpolation_stack.pop();
                        self.scan_template_literal(tokens, self.line, self.column)?;
                    }
                }
            }
            '[' => tokens.push(Token::new(TokenKind::LeftBracket, c.to_string(), start_line, start_column)),
            ']' => tokens.push(Token::new(TokenKind::RightBracket, c.to_string(), start_line, start_column)),
            ',' => tokens.push(Token::new(TokenKind::Comma, c.to_string(), start_line, start_column)),
            '.' => tokens.push(Token::new(TokenKind::Dot, c.to_string(), start_line, start_column)),
            '+' => tokens.push(Token::new(TokenKind::Plus, c.to_string(), start_line, start_column)),
            '-' => {
                if self.peek() == '>' {
                    self.advance();
                    tokens.push(Token::new(TokenKind::Arrow, "->".to_string(), start_line, start_column));
                } else {
                    tokens.push(Token::new(TokenKind::Minus, c.to_string(), start_line, start_column));
                }
            }
            '*' => tokens.push(Token::new(TokenKind::Star, c.to_string(), start_line, start_column)),
            '/' => tokens.push(Token::new(TokenKind::Slash, c.to_string(), start_line, start_column)),
            '%' => tokens.push(Token::new(TokenKind::Percent, c.to_string(), start_line, start_column)),
            
            // Multi-character operators
            '>' => {
                if self.peek() == '>' {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        tokens.push(Token::new(TokenKind::GreaterEq, ">>=".to_string(), start_line, start_column));
                    } else {
                        tokens.push(Token::new(TokenKind::Greater, ">>".to_string(), start_line, start_column));
                    }
                } else if self.peek() == '=' {
                    self.advance();
                    tokens.push(Token::new(TokenKind::GreaterEq, ">=".to_string(), start_line, start_column));
                } else {
                    tokens.push(Token::new(TokenKind::Greater, ">".to_string(), start_line, start_column));
                }
            }
            
            '<' => {
                if self.peek() == '<' {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        tokens.push(Token::new(TokenKind::LessEq, "<<=".to_string(), start_line, start_column));
                    } else {
                        tokens.push(Token::new(TokenKind::Less, "<<".to_string(), start_line, start_column));
                    }
                } else if self.peek() == '=' {
                    self.advance();
                    tokens.push(Token::new(TokenKind::LessEq, "<=".to_string(), start_line, start_column));
                } else {
                    tokens.push(Token::new(TokenKind::Less, "<".to_string(), start_line, start_column));
                }
            }
            
            '!' => {
                if self.peek() == '!' {
                    self.advance();
                    tokens.push(Token::new(TokenKind::ChainEnd, "!!".to_string(), start_line, start_column));
                } else {
                    return Err(FlowError::syntax(
                        "Lone '!' detected! Use 'both!', 'either!', 'negate!' or '!!' for chain end.",
                        start_line,
                        start_column,
                    ));
                }
            }
            
            ':' => {
                if self.peek() == ':' {
                    self.advance();
                    tokens.push(Token::new(TokenKind::DoubleColon, "::".to_string(), start_line, start_column));
                } else {
                    tokens.push(Token::new(TokenKind::Colon, c.to_string(), start_line, start_column));
                }
            }
            
            '=' => {
                if self.peek() == '>' {
                    self.advance();
                    tokens.push(Token::new(TokenKind::FatArrow, "=>".to_string(), start_line, start_column));
                } else {
                    tokens.push(Token::new(TokenKind::Equals, c.to_string(), start_line, start_column));
                }
            }
            
            // Strings
            '"' => self.scan_double_quote_string(tokens, start_line, start_column)?,
            '\'' => self.scan_simple_string(tokens, start_line, start_column)?,
            '`' => self.scan_template_literal(tokens, start_line, start_column)?,
            
            // Sigils
            '@' => self.scan_sigil(tokens, start_line, start_column)?,
            
            // Numbers
            c if c.is_ascii_digit() => self.scan_number(tokens, c, start_line, start_column)?,
            
            // Identifiers and keywords
            c if c.is_alphabetic() || c == '_' => {
                self.scan_identifier_or_keyword(tokens, c, start_line, start_column)?
            }
            
            _ => {
                return Err(FlowError::syntax(
                    &format!("Unknown character '{}' encountered in the flow!", c),
                    start_line,
                    start_column,
                ));
            }
        }
        
        Ok(())
    }
    
    fn scan_double_quote_string(&mut self, tokens: &mut Vec<Token>, start_line: usize, start_column: usize) -> Result<(), FlowError> {
        let mut value = String::new();
        
        while !self.is_at_end() {
            if self.peek() == '"' {
                self.advance(); // consume "
                
                // End of string
                tokens.push(Token::new(
                    TokenKind::String(value.clone()),
                    format!("\"{}\"", value),
                    start_line,
                    start_column,
                ));
                return Ok(());
            }
            
            if self.peek() == '\\' {
                // Handle escape sequences
                self.advance(); // consume backslash
                
                if self.is_at_end() {
                    return Err(FlowError::syntax(
                        "Unterminated escape sequence in string",
                        start_line,
                        start_column,
                    ));
                }
                
                let escaped_char = match self.peek() {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    '\'' => '\'',
                    '0' => '\0',
                    _ => {
                        // Unknown escape sequence - just include the character as-is
                        self.peek()
                    }
                };
                
                value.push(escaped_char);
                self.advance();
            } else {
                if self.peek() == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                value.push(self.advance());
            }
        }
        
        Err(FlowError::syntax(
            "Unterminated string! The Silk essence must be closed with \".",
            start_line,
            start_column,
        ))
    }
    
    fn scan_simple_string(&mut self, tokens: &mut Vec<Token>, start_line: usize, start_column: usize) -> Result<(), FlowError> {
        let mut value = String::new();
        
        while !self.is_at_end() {
            if self.peek() == '\'' {
                self.advance(); // consume '
                
                // End of string
                tokens.push(Token::new(
                    TokenKind::String(value.clone()),
                    format!("'{}'", value),
                    start_line,
                    start_column,
                ));
                return Ok(());
            }
            
            if self.peek() == '\\' {
                // Handle escape sequences
                self.advance(); // consume backslash
                
                if self.is_at_end() {
                    return Err(FlowError::syntax(
                        "Unterminated escape sequence in string",
                        start_line,
                        start_column,
                    ));
                }
                
                let escaped_char = match self.peek() {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    '\'' => '\'',
                    '0' => '\0',
                    _ => {
                        // Unknown escape sequence - just include the character as-is
                        self.peek()
                    }
                };
                
                value.push(escaped_char);
                self.advance();
            } else {
                if self.peek() == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                value.push(self.advance());
            }
        }
        
        Err(FlowError::syntax(
            "Unterminated string! The Silk essence must be closed with '.",
            start_line,
            start_column,
        ))
    }
    
    fn scan_template_literal(&mut self, tokens: &mut Vec<Token>, start_line: usize, start_column: usize) -> Result<(), FlowError> {
        let mut value = String::new();
        
        while !self.is_at_end() {
            if self.peek() == '`' {
                self.advance(); // consume `
                
                // End of template literal segment
                tokens.push(Token::new(
                    TokenKind::String(value.clone()),
                    format!("`{}`", value),
                    start_line,
                    start_column,
                ));
                return Ok(());
            }
            
            if self.peek() == '$' && self.peek_next() == '{' {
                self.advance(); // $
                self.advance(); // {
                
                // Emit the string part before the interpolation
                tokens.push(Token::new(
                    TokenKind::StringPart(value.clone()),
                    value,
                    start_line,
                    start_column,
                ));
                
                // Emit interpolation start
                tokens.push(Token::new(
                    TokenKind::InterpolationStart,
                    "${".to_string(),
                    self.line,
                    self.column,
                ));
                
                // Push current brace depth to stack
                self.interpolation_stack.push(self.brace_depth);
                return Ok(());
            }
            
            if self.peek() == '\\' {
                // Handle escape sequences
                self.advance(); // consume backslash
                
                if self.is_at_end() {
                    return Err(FlowError::syntax(
                        "Unterminated escape sequence in template literal",
                        start_line,
                        start_column,
                    ));
                }
                
                let escaped_char = match self.peek() {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '`' => '`',
                    '"' => '"',
                    '\'' => '\'',
                    '0' => '\0',
                    _ => {
                        // Unknown escape sequence - just include the character as-is
                        self.peek()
                    }
                };
                
                value.push(escaped_char);
                self.advance();
            } else {
                if self.peek() == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                value.push(self.advance());
            }
        }
        
        Err(FlowError::syntax(
            "Unterminated template literal! The Silk essence must be closed with `.",
            start_line,
            start_column,
        ))
    }
    
    fn scan_number(&mut self, tokens: &mut Vec<Token>, first: char, start_line: usize, start_column: usize) -> Result<(), FlowError> {
        let mut num_str = String::from(first);
        
        while !self.is_at_end() && (self.peek().is_ascii_digit() || self.peek() == '.') {
            num_str.push(self.advance());
        }
        
        let value: f64 = num_str.parse().map_err(|_| {
            FlowError::syntax(
                &format!("Invalid Ember essence: '{}'", num_str),
                start_line,
                start_column,
            )
        })?;
        
        tokens.push(Token::new(
            TokenKind::Number(value),
            num_str,
            start_line,
            start_column,
        ));
        Ok(())
    }
    
    fn scan_sigil(&mut self, tokens: &mut Vec<Token>, start_line: usize, start_column: usize) -> Result<(), FlowError> {
        let mut name = String::new();
        
        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            name.push(self.advance());
        }
        
        if name.is_empty() {
            return Err(FlowError::syntax(
                "Empty sigil detected! Sigils must have a name after @.",
                start_line,
                start_column,
            ));
        }
        
        tokens.push(Token::new(
            TokenKind::Sigil(name.clone()),
            format!("@{}", name),
            start_line,
            start_column,
        ));
        Ok(())
    }
    
    fn scan_identifier_or_keyword(&mut self, tokens: &mut Vec<Token>, first: char, start_line: usize, start_column: usize) -> Result<(), FlowError> {
        let mut ident = String::from(first);
        
        // Scan alphanumeric and underscores
        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            ident.push(self.advance());
        }
        
        // Check for special operator suffixes (~ or !)
        if !self.is_at_end() && (self.peek() == '~' || self.peek() == '!') {
            ident.push(self.advance());
        }
        
        // Check for multi-word keywords
        let kind = match ident.as_str() {
            "in" => {
                self.skip_whitespace();
                if self.match_word("Stance") {
                    TokenKind::InStance
                } else {
                    // Standalone 'in' for for-each loops
                    TokenKind::In
                }
            }
            "shift" => {
                self.skip_whitespace();
                if self.match_word("Stance") {
                    TokenKind::ShiftStance
                } else {
                    return Err(FlowError::syntax(
                        "Expected 'Stance' after 'shift'!",
                        start_line,
                        start_column,
                    ));
                }
            }
            "abandon" => {
                self.skip_whitespace();
                if self.match_word("Stance") {
                    TokenKind::AbandonStance
                } else {
                    return Err(FlowError::syntax(
                        "Expected 'Stance' after 'abandon'!",
                        start_line,
                        start_column,
                    ));
                }
            }
            "invoke" => {
                self.skip_whitespace();
                if self.match_word("Aura") {
                    TokenKind::InvokeAura
                } else {
                    return Err(FlowError::syntax(
                        "Expected 'Aura' after 'invoke'!",
                        start_line,
                        start_column,
                    ));
                }
            }
            "enter" => {
                self.skip_whitespace();
                if self.match_word("Phase") {
                    TokenKind::EnterPhase
                } else {
                    return Err(FlowError::syntax(
                        "Expected 'Phase' after 'enter'!",
                        start_line,
                        start_column,
                    ));
                }
            }
            "cast" => {
                self.skip_whitespace();
                if self.match_word("Spell") {
                    TokenKind::CastSpell
                } else {
                    return Err(FlowError::syntax(
                        "Expected 'Spell' after 'cast'!",
                        start_line,
                        start_column,
                    ));
                }
            }
            
            // Special operators
            "is~" => TokenKind::IsEqual,
            "not~" => TokenKind::NotEqual,
            "both!" => TokenKind::Both,
            "either!" => TokenKind::Either,
            "negate!" => TokenKind::Negate,
            
            // Single word keywords
            "when" => TokenKind::When,
            "otherwise" => TokenKind::Otherwise,
            "from" => TokenKind::From,
            "to" => TokenKind::To,
            "until" => TokenKind::Until,
            "forever" => TokenKind::Forever,
            "ritual" => TokenKind::Ritual,
            "await" => TokenKind::Await,
            "perform" => TokenKind::Perform,
            "wait" => TokenKind::Wait,
            "let" => TokenKind::Let,
            "seal" => TokenKind::Seal,
            "return" => TokenKind::Return,
            "circle" => TokenKind::Circle,
            "as" => TokenKind::As,
            "end" => TokenKind::End,
            
            // Types
            "Ember" => TokenKind::Ember,
            "Silk" => TokenKind::Silk,
            "Pulse" => TokenKind::Pulse,
            "Flux" => TokenKind::Flux,
            "Hollow" => TokenKind::Hollow,
            "Constellation" => TokenKind::Constellation,
            "Relic" => TokenKind::Relic,
            "Spell" => TokenKind::Spell,
            "sigil" => TokenKind::SigilDef,
            
            // Boolean literals
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            
            // ⚔️ ERROR ARC - Error Handling Keywords
            "panic" => TokenKind::Panic,
            "wound" => TokenKind::Wound,
            "attempt" => TokenKind::Attempt,
            "rescue" => TokenKind::Rescue,
            "rebound" => TokenKind::Rebound,
            "ward" => TokenKind::Ward,
            "break" => TokenKind::Break,
            "fracture" => TokenKind::Fracture,
            "shatter" => TokenKind::Shatter,
            "finally" => TokenKind::Finally,
            "retry" => TokenKind::Retry,
            "grand_seal" => TokenKind::GrandSeal,
            "rupture" => TokenKind::Rupture,
            
            // Otherwise it's an identifier
            _ => TokenKind::Identifier(ident.clone()),
        };
        
        tokens.push(Token::new(kind, ident, start_line, start_column));
        Ok(())
    }
    
    fn match_word(&mut self, word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();
        
        for (i, &ch) in chars.iter().enumerate() {
            if self.current + i >= self.source.len() || self.source[self.current + i] != ch {
                return false;
            }
        }
        
        // Advance past the word
        for _ in 0..chars.len() {
            self.advance();
        }
        
        true
    }
    
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
                _ => break,
            }
        }
    }
    
    fn skip_line_comment(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
    }
    
    fn skip_block_comment(&mut self) -> Result<(), FlowError> {
        let start_line = self.line;
        let start_column = self.column;
        
        self.advance(); // /
        self.advance(); // *
        
        while !self.is_at_end() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance();
                self.advance();
                return Ok(());
            }
            
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            
            self.advance();
        }
        
        Err(FlowError::syntax(
            "Unterminated block comment! The mystical notes must be closed with */.",
            start_line,
            start_column,
        ))
    }
    
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }
    
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }
    
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        self.column += 1;
        c
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, FlowError> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}
