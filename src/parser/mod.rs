pub mod ast;

use ast::*;
use crate::error::FlowError;
use crate::lexer::token::{Token, TokenKind};
use crate::types::EssenceType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Program, FlowError> {
        let mut imports = Vec::new();
        let mut statements = Vec::new();
        
        // Parse imports first
        while self.match_token(&TokenKind::Circle) {
            imports.push(self.parse_import()?);
        }
        
        // Parse statements
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        Ok(Program { imports, statements })
    }
    
    fn parse_import(&mut self) -> Result<Import, FlowError> {
        let line = self.previous().line;
        
        // Check for selective imports: circle {member1, member2} from "..."
        let selective = if self.check(&TokenKind::LeftBrace) {
            self.advance(); // consume {
            let mut imports = Vec::new();
            
            loop {
                // Parse member name
                let member_name = if let TokenKind::Identifier(name) = &self.peek().kind {
                    name.clone()
                } else {
                    return Err(FlowError::syntax(
                        "Expected member name in selective import",
                        self.peek().line,
                        self.peek().column,
                    ));
                };
                self.advance();
                
                // Check for alias: member as alias
                let alias = if self.match_token(&TokenKind::As) {
                    if let TokenKind::Identifier(alias_name) = &self.peek().kind {
                        let a = alias_name.clone();
                        self.advance();
                        Some(a)
                    } else {
                        return Err(FlowError::syntax(
                            "Expected alias name after 'as'",
                            self.peek().line,
                            self.peek().column,
                        ));
                    }
                } else {
                    None
                };
                
                imports.push(SelectiveImport {
                    name: member_name,
                    alias,
                });
                
                // Check for comma (more imports) or closing brace
                if self.match_token(&TokenKind::Comma) {
                    continue;
                } else if self.check(&TokenKind::RightBrace) {
                    self.advance(); // consume }
                    break;
                } else {
                    return Err(FlowError::syntax(
                        "Expected ',' or '}' in selective import",
                        self.peek().line,
                        self.peek().column,
                    ));
                }
            }
            
            Some(imports)
        } else {
            None
        };
        
        // Parse module name or continue with 'from' for selective imports
        let (module_name, from_path, alias) = if selective.is_some() {
            // For selective imports, expect 'from "path"'
            if !self.match_token(&TokenKind::From) {
                return Err(FlowError::syntax(
                    "Expected 'from' after selective import list",
                    self.peek().line,
                    self.peek().column,
                ));
            }
            
            if let TokenKind::String(path) = &self.peek().kind {
                let path_str = path.clone();
                self.advance();
                // For selective imports, module name is derived from path
                // e.g., "std:color" -> module name is "color" (but we don't use it for selective)
                (path_str.clone(), Some(path_str), None)
            } else {
                return Err(FlowError::syntax(
                    "Expected string path after 'from'",
                    self.peek().line,
                    self.peek().column,
                ));
            }
        } else {
            // Normal import: circle module from "path" or circle module as alias
            let module_name = if let TokenKind::Identifier(name) = &self.peek().kind {
                name.clone()
            } else {
                return Err(FlowError::syntax(
                    "Expected module name after 'circle'!",
                    line,
                    self.peek().column,
                ));
            };
            self.advance();
            
            let mut from_path_opt = None;
            let mut module_alias = None;
            
            // Check for 'from "path"'
            if self.match_token(&TokenKind::From) {
                if let TokenKind::String(path) = &self.peek().kind {
                    from_path_opt = Some(path.clone());
                    self.advance();
                } else {
                    return Err(FlowError::syntax(
                        "Expected string path after 'from'!",
                        self.peek().line,
                        self.peek().column,
                    ));
                }
            }
            
            // Check for 'as alias'
            if self.match_token(&TokenKind::As) {
                if let TokenKind::Identifier(name) = &self.peek().kind {
                    module_alias = Some(name.clone());
                    self.advance();
                } else {
                    return Err(FlowError::syntax(
                        "Expected identifier after 'as'!",
                        self.peek().line,
                        self.peek().column,
                    ));
                }
            }
            
            (module_name, from_path_opt, module_alias)
        };
        
        Ok(Import {
            module: module_name,
            alias,
            from_path,
            selective,
        })
    }
    
    fn parse_statement(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        
        // Skip sigils and store them
        let mut sigils = Vec::new();
        while let TokenKind::Sigil(name) = &self.peek().kind {
            sigils.push(name.clone());
            self.advance();
        }
        
        match &self.peek().kind {
            TokenKind::Let => self.parse_let(sigils.clone()),
            TokenKind::Seal => self.parse_seal(sigils.clone()),
            TokenKind::CastSpell => self.parse_function(sigils),
            TokenKind::Ritual => self.parse_ritual(sigils.clone()),
            TokenKind::Return => self.parse_return(),
            TokenKind::InStance => self.parse_stance(),
            TokenKind::InvokeAura => self.parse_aura(),
            TokenKind::EnterPhase => self.parse_phase(),
            TokenKind::Wait => self.parse_wait(),
            TokenKind::Perform => self.parse_perform(),
            // ⚔️ ERROR ARC
            TokenKind::Panic => self.parse_panic(),
            TokenKind::Wound => self.parse_wound(),
            TokenKind::Rupture => self.parse_rupture(),
            TokenKind::Attempt => self.parse_attempt(),
            TokenKind::Rebound => self.parse_rebound(),
            TokenKind::Ward => self.parse_ward(),
            TokenKind::Break => self.parse_break_seal(),
            TokenKind::Fracture => self.parse_fracture_seal(),
            TokenKind::Shatter => self.parse_shatter_grand_seal(),
            TokenKind::SigilDef => self.parse_sigil_def(sigils.clone()),
            _ => {
                // Check if this is an assignment (identifier = expression)
                if let TokenKind::Identifier(name) = &self.peek().kind {
                    let var_name = name.clone();
                    let start_pos = self.current;
                    self.advance(); // consume identifier
                    
                    if self.match_token(&TokenKind::Equals) {
                        // This is an assignment
                        let value = self.parse_expression()?;
                        return Ok(Statement::Assignment {
                            name: var_name,
                            value,
                            line,
                        });
                    } else {
                        // Not an assignment, backtrack and parse as expression
                        self.current = start_pos;
                    }
                }
                
                let expr = self.parse_expression()?;
                Ok(Statement::Expression { expr, line })
            }
        }
    }
    
    fn parse_let(&mut self, sigils: Vec<String>) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'let'
        
        let name = self.expect_identifier("Expected variable name after 'let'")?;
        
        let type_annotation = if self.match_token(&TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.expect(&TokenKind::Equals, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;
        
        // Check if @export sigil is present
        let is_exported = sigils.contains(&"export".to_string());
        
        Ok(Statement::Let {
            name,
            type_annotation,
            value,
            is_exported,
            line,
        })
    }
    
    fn parse_seal(&mut self, sigils: Vec<String>) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'seal'
        
        let name = self.expect_identifier("Expected variable name after 'seal'")?;
        
        let type_annotation = if self.match_token(&TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.expect(&TokenKind::Equals, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;
        
        // Check if @export sigil is present
        let is_exported = sigils.contains(&"export".to_string());
        
        Ok(Statement::Seal {
            name,
            type_annotation,
            value,
            is_exported,
            line,
        })
    }
    
    fn parse_function(&mut self, sigils: Vec<String>) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'cast Spell'
        
        let name = self.expect_identifier("Expected function name after 'cast Spell'")?;
        
        self.expect(&TokenKind::LeftParen, "Expected '(' after function name")?;
        
        let params = self.parse_parameters()?;
        
        self.expect(&TokenKind::RightParen, "Expected ')' after parameters")?;
        
        let return_type = if self.match_token(&TokenKind::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.expect(&TokenKind::LeftBrace, "Expected '{' before function body")?;
        
        let body = self.parse_block()?;
        
        self.expect(&TokenKind::RightBrace, "Expected '}' after function body")?;
        
        // Check if @export sigil is present
        let is_exported = sigils.contains(&"export".to_string());
        
        Ok(Statement::FunctionDecl {
            name,
            params,
            return_type,
            body,
            sigils,
            is_exported,
            line,
        })
    }
    
    fn parse_ritual(&mut self, sigils: Vec<String>) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'ritual'
        
        let name = self.expect_identifier("Expected ritual name")?;
        
        // Check for optional parameters
        let params = if self.match_token(&TokenKind::LeftParen) {
            let p = self.parse_parameters()?;
            self.expect(&TokenKind::RightParen, "Expected ')' after parameters")?;
            p
        } else {
            Vec::new()
        };
        
        self.expect(&TokenKind::DoubleColon, "Expected '::' after ritual declaration")?;
        
        let mut body = Vec::new();
        while !self.check(&TokenKind::End) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        
        self.expect(&TokenKind::End, "Expected 'end' after ritual body")?;
        
        // Check if @export sigil is present
        let is_exported = sigils.contains(&"export".to_string());
        
        Ok(Statement::Ritual {
            name,
            params,
            return_type: None,
            body,
            is_exported,
            line,
        })
    }
    
    fn parse_sigil_def(&mut self, sigils: Vec<String>) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'sigil'
        
        // Get sigil name
        let name = if let TokenKind::Identifier(n) = &self.peek().kind {
            let name = n.clone();
            self.advance();
            name
        } else {
            return Err(FlowError::syntax(
                "Expected sigil name after 'sigil' keyword",
                self.peek().line,
                self.peek().column,
            ));
        };
        
        // Expect opening brace
        if !self.match_token(&TokenKind::LeftBrace) {
            return Err(FlowError::syntax(
                "Expected '{' after sigil name",
                self.peek().line,
                self.peek().column,
            ));
        }
        
        // Parse fields
        let mut fields = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Get field name
            let field_name = if let TokenKind::Identifier(n) = &self.peek().kind {
                let name = n.clone();
                self.advance();
                name
            } else {
                return Err(FlowError::syntax(
                    "Expected field name in sigil definition",
                    self.peek().line,
                    self.peek().column,
                ));
            };
            
            // Expect colon
            if !self.match_token(&TokenKind::Colon) {
                return Err(FlowError::syntax(
                    "Expected ':' after field name",
                    self.peek().line,
                    self.peek().column,
                ));
            }
            
            // Parse field type
            let field_type = self.parse_type()?;
            
            fields.push(ast::SigilField {
                name: field_name,
                field_type,
            });
            
            // Optional comma (or newline)
            self.match_token(&TokenKind::Comma);
        }
        
        // Expect closing brace
        if !self.match_token(&TokenKind::RightBrace) {
            return Err(FlowError::syntax(
                "Expected '}' to close sigil definition",
                self.peek().line,
                self.peek().column,
            ));
        }
        
        // Check for @export sigil
        let is_exported = sigils.contains(&"export".to_string());
        
        Ok(Statement::SigilDecl {
            name,
            fields,
            is_exported,
            line,
        })
    }
    
    fn parse_wait(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'wait'
        
        let duration = self.parse_expression()?;
        
        // Check for unit (s, ms, m)
        // Since we don't have specific tokens for units, we check for identifiers
        let unit = if let TokenKind::Identifier(u) = &self.peek().kind {
            let unit_str = u.clone();
            self.advance();
            unit_str
        } else {
            "ms".to_string() // Default to ms if no unit provided
        };
        
        Ok(Statement::Wait {
            duration,
            unit,
            line,
        })
    }
    
    fn parse_perform(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'perform'
        
        let mut rituals = Vec::new();
        
        loop {
            rituals.push(self.parse_expression()?);
            
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
        
        Ok(Statement::Perform {
            rituals,
            line,
        })
    }
    
    fn parse_return(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'return'
        
        let value = if self.is_at_end() || self.check(&TokenKind::RightBrace) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        Ok(Statement::Return { value, line })
    }
    
    fn parse_stance(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'in Stance'
        
        self.expect(&TokenKind::LeftParen, "Expected '(' after 'in Stance'")?;
        let condition = self.parse_expression()?;
        self.expect(&TokenKind::RightParen, "Expected ')' after condition")?;
        
        self.expect(&TokenKind::LeftBrace, "Expected '{' after condition")?;
        let then_branch = self.parse_block()?;
        self.expect(&TokenKind::RightBrace, "Expected '}' after then branch")?;
        
        let mut shift_branches = Vec::new();
        while self.match_token(&TokenKind::ShiftStance) {
            self.expect(&TokenKind::LeftParen, "Expected '(' after 'shift Stance'")?;
            let shift_condition = self.parse_expression()?;
            self.expect(&TokenKind::RightParen, "Expected ')' after condition")?;
            
            self.expect(&TokenKind::LeftBrace, "Expected '{' after condition")?;
            let shift_body = self.parse_block()?;
            self.expect(&TokenKind::RightBrace, "Expected '}' after shift branch")?;
            
            shift_branches.push((shift_condition, shift_body));
        }
        
        let abandon_branch = if self.match_token(&TokenKind::AbandonStance) {
            self.expect(&TokenKind::LeftBrace, "Expected '{' after 'abandon Stance'")?;
            let abandon_body = self.parse_block()?;
            self.expect(&TokenKind::RightBrace, "Expected '}' after abandon branch")?;
            Some(abandon_body)
        } else if self.check(&TokenKind::Otherwise) {
            // User mistakenly used 'otherwise' instead of 'abandon Stance'
            return Err(FlowError::syntax(
                "'otherwise' cannot be used here!\n\n\
                 💡 TIP: Use 'abandon Stance' for else in if statements:\n\
                    } abandon Stance {\n\
                 \n\
                 'otherwise' is only for switch (invoke Aura):\n\
                    invoke Aura value {\n\
                        when 1 -> shout(\"one\")\n\
                        otherwise -> shout(\"default\")\n\
                    }",
                self.peek().line,
                self.peek().column,
            ));
        } else {
            None
        };
        
        Ok(Statement::Stance {
            condition,
            then_branch,
            shift_branches,
            abandon_branch,
            line,
        })
    }
    
    fn parse_aura(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'invoke Aura'
        
        let value = self.parse_expression()?;
        
        self.expect(&TokenKind::LeftBrace, "Expected '{' after aura value")?;
        
        let mut cases = Vec::new();
        let mut otherwise = None;
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::When) {
                let case_value = self.parse_expression()?;
                self.expect(&TokenKind::Arrow, "Expected '->' after when value")?;
                
                let case_body = if self.check(&TokenKind::LeftBrace) {
                    self.advance();
                    let body = self.parse_block()?;
                    self.expect(&TokenKind::RightBrace, "Expected '}' after case body")?;
                    body
                } else {
                    vec![Statement::Expression {
                        expr: self.parse_expression()?,
                        line: self.peek().line,
                    }]
                };
                
                cases.push((case_value, case_body));
            } else if self.match_token(&TokenKind::Otherwise) {
                self.expect(&TokenKind::Arrow, "Expected '->' after otherwise")?;
                
                otherwise = if self.check(&TokenKind::LeftBrace) {
                    self.advance();
                    let body = self.parse_block()?;
                    self.expect(&TokenKind::RightBrace, "Expected '}' after otherwise body")?;
                    Some(body)
                } else {
                    Some(vec![Statement::Expression {
                        expr: self.parse_expression()?,
                        line: self.peek().line,
                    }])
                };
                break;
            } else {
                break;
            }
        }
        
        self.expect(&TokenKind::RightBrace, "Expected '}' after aura cases")?;
        
        Ok(Statement::Aura {
            value,
            cases,
            otherwise,
            line,
        })
    }
    
    fn parse_phase(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'enter Phase'
        
        let kind = if self.match_token(&TokenKind::Forever) {
            PhaseKind::Forever
        } else if self.match_token(&TokenKind::Until) {
            self.expect(&TokenKind::LeftParen, "Expected '(' after 'until'")?;
            let condition = self.parse_expression()?;
            self.expect(&TokenKind::RightParen, "Expected ')' after condition")?;
            PhaseKind::Until { condition }
        } else {
            // Could be: enter Phase i from 0 to 5  OR  enter Phase item in collection
            let variable = self.expect_identifier("Expected loop variable")?;
            
            if self.match_token(&TokenKind::In) {
                // For-each loop: enter Phase item in collection
                let collection = self.parse_expression()?;
                PhaseKind::ForEach { variable, collection }
            } else {
                // Count loop: enter Phase i from 0 to 5
                self.expect(&TokenKind::From, "Expected 'from' or 'in' in phase loop")?;
                let from = self.parse_expression()?;
                self.expect(&TokenKind::To, "Expected 'to' in phase loop")?;
                let to = self.parse_expression()?;
                PhaseKind::Count { variable, from, to }
            }
        };
        
        self.expect(&TokenKind::LeftBrace, "Expected '{' before phase body")?;
        let body = self.parse_block()?;
        self.expect(&TokenKind::RightBrace, "Expected '}' after phase body")?;
        
        Ok(Statement::Phase { kind, body, line })
    }
    
    fn parse_block(&mut self) -> Result<Vec<Statement>, FlowError> {
        let mut statements = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        Ok(statements)
    }
    
    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, FlowError> {
        let mut params = Vec::new();
        
        if self.check(&TokenKind::RightParen) {
            return Ok(params);
        }
        
        loop {
            let type_annotation = if self.check_type() {
                Some(self.parse_type()?)
            } else {
                None
            };
            
            let name = self.expect_identifier("Expected parameter name")?;
            
            params.push(Parameter {
                name,
                type_annotation,
            });
            
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
        
        Ok(params)
    }
    
    fn parse_type(&mut self) -> Result<EssenceType, FlowError> {
        match &self.peek().kind {
            TokenKind::Ember => {
                self.advance();
                Ok(EssenceType::Ember)
            }
            TokenKind::Silk => {
                self.advance();
                Ok(EssenceType::Silk)
            }
            TokenKind::Pulse => {
                self.advance();
                Ok(EssenceType::Pulse)
            }
            TokenKind::Flux => {
                self.advance();
                Ok(EssenceType::Flux)
            }
            TokenKind::Hollow => {
                self.advance();
                Ok(EssenceType::Hollow)
            }
            TokenKind::Constellation => {
                self.advance();
                self.expect(&TokenKind::Less, "Expected '<' after Constellation")?;
                let inner = self.parse_type()?;
                self.expect(&TokenKind::Greater, "Expected '>' after type")?;
                Ok(EssenceType::Constellation(Box::new(inner)))
            }
            TokenKind::Relic => {
                self.advance();
                self.expect(&TokenKind::Less, "Expected '<' after Relic")?;
                let key_type = self.parse_type()?;
                self.expect(&TokenKind::Comma, "Expected ',' after key type")?;
                let value_type = self.parse_type()?;
                self.expect(&TokenKind::Greater, "Expected '>' after value type")?;
                Ok(EssenceType::Relic(Box::new(key_type), Box::new(value_type)))
            }
            TokenKind::Spell => {
                self.advance();
                Ok(EssenceType::Spell)
            }
            _ => Err(FlowError::syntax(
                "Expected type name!",
                self.peek().line,
                self.peek().column,
            )),
        }
    }
    
    fn check_type(&self) -> bool {
        matches!(
            &self.peek().kind,
            TokenKind::Ember
                | TokenKind::Silk
                | TokenKind::Pulse
                | TokenKind::Flux
                | TokenKind::Hollow
                | TokenKind::Constellation
                | TokenKind::Spell
        )
    }
    
    fn parse_expression(&mut self) -> Result<Expression, FlowError> {
        self.parse_combo_chain()
    }
    
    fn parse_combo_chain(&mut self) -> Result<Expression, FlowError> {
        let mut expr = self.parse_logical_or()?;
        
        // Check for combo chain operator >>
        if self.check(&TokenKind::Greater) {
            let mut operations = Vec::new();
            
            while self.match_token(&TokenKind::Greater) {
                // Parse chain operation (function call or method)
                if let TokenKind::Identifier(name) = &self.peek().kind {
                    let op_name = name.clone();
                    self.advance();
                    
                    // Check if it's a function call with arguments
                    if self.match_token(&TokenKind::LeftParen) {
                        let args = self.parse_arguments()?;
                        self.expect(&TokenKind::RightParen, "Expected ')' after arguments")?;
                        operations.push(ChainOperation::Call(op_name, args));
                    } else {
                        operations.push(ChainOperation::Method(op_name));
                    }
                } else {
                    break;
                }
                
                // Check for chain end !!
                if self.match_token(&TokenKind::ChainEnd) {
                    break;
                }
            }
            
            if !operations.is_empty() {
                expr = Expression::ComboChain {
                    initial: Box::new(expr),
                    operations,
                };
            }
        }
        
        Ok(expr)
    }
    
    fn parse_logical_or(&mut self) -> Result<Expression, FlowError> {
        let mut expr = self.parse_logical_and()?;
        
        while self.match_token(&TokenKind::Either) {
            let right = self.parse_logical_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Either,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_logical_and(&mut self) -> Result<Expression, FlowError> {
        let mut expr = self.parse_equality()?;
        
        while self.match_token(&TokenKind::Both) {
            let right = self.parse_equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Both,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_equality(&mut self) -> Result<Expression, FlowError> {
        let mut expr = self.parse_comparison()?;
        
        while let Some(op) = self.match_tokens(&[TokenKind::IsEqual, TokenKind::NotEqual]) {
            let operator = match op {
                TokenKind::IsEqual => BinaryOp::IsEqual,
                TokenKind::NotEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            
            let right = self.parse_comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_comparison(&mut self) -> Result<Expression, FlowError> {
        let mut expr = self.parse_term()?;
        
        while let Some(op) = self.match_tokens(&[
            TokenKind::Greater,
            TokenKind::Less,
            TokenKind::GreaterEq,
            TokenKind::LessEq,
        ]) {
            let operator = match op {
                TokenKind::Greater => BinaryOp::Greater,
                TokenKind::Less => BinaryOp::Less,
                TokenKind::GreaterEq => BinaryOp::GreaterEq,
                TokenKind::LessEq => BinaryOp::LessEq,
                _ => unreachable!(),
            };
            
            let right = self.parse_term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_term(&mut self) -> Result<Expression, FlowError> {
        let mut expr = self.parse_factor()?;
        
        while let Some(op) = self.match_tokens(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = match op {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            
            let right = self.parse_factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_factor(&mut self) -> Result<Expression, FlowError> {
        let mut expr = self.parse_unary()?;
        
        while let Some(op) = self.match_tokens(&[TokenKind::Star, TokenKind::Slash, TokenKind::Percent]) {
            let operator = match op {
                TokenKind::Star => BinaryOp::Multiply,
                TokenKind::Slash => BinaryOp::Divide,
                TokenKind::Percent => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            
            let right = self.parse_unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_unary(&mut self) -> Result<Expression, FlowError> {
        if let Some(op) = self.match_tokens(&[TokenKind::Negate, TokenKind::Minus]) {
            let operator = match op {
                TokenKind::Negate => UnaryOp::Negate,
                TokenKind::Minus => UnaryOp::Minus,
                _ => unreachable!(),
            };
            
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary {
                operator,
                operand: Box::new(operand),
            });
        }
        
        if self.match_token(&TokenKind::Await) {
            let expr = self.parse_unary()?;
            return Ok(Expression::Await {
                expr: Box::new(expr),
            });
        }
        
        self.parse_postfix()
    }
    
    fn parse_postfix(&mut self) -> Result<Expression, FlowError> {
        let mut expr = self.parse_primary()?;
        
        loop {
            if self.match_token(&TokenKind::LeftParen) {
                let arguments = self.parse_arguments()?;
                self.expect(&TokenKind::RightParen, "Expected ')' after arguments")?;
                expr = Expression::Call {
                    callee: Box::new(expr),
                    arguments,
                };
            } else if self.match_token(&TokenKind::LeftBracket) {
                let index = self.parse_expression()?;
                self.expect(&TokenKind::RightBracket, "Expected ']' after index")?;
                expr = Expression::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_token(&TokenKind::Dot) {
                let name = self.expect_identifier("Expected property name after '.'")?;
                
                // Check if this is a method call (followed by '(')
                if self.check(&TokenKind::LeftParen) {
                    self.advance(); // consume '('
                    let arguments = self.parse_arguments()?;
                    self.expect(&TokenKind::RightParen, "Expected ')' after method arguments")?;
                    expr = Expression::MethodCall {
                        object: Box::new(expr),
                        method: name,
                        arguments,
                    };
                } else {
                    // Property access
                    expr = Expression::Index {
                        object: Box::new(expr),
                        index: Box::new(Expression::String(name)),
                    };
                }
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn parse_primary(&mut self) -> Result<Expression, FlowError> {
        match &self.peek().kind.clone() {
            TokenKind::Number(n) => {
                let value = *n;
                self.advance();
                Ok(Expression::Number(value))
            }
            TokenKind::String(s) => {
                let value = s.clone();
                self.advance();
                Ok(Expression::String(value))
            }
            TokenKind::StringPart(s) => {
                let mut parts = Vec::new();
                parts.push(Expression::String(s.clone()));
                self.advance();
                
                loop {
                    self.expect(&TokenKind::InterpolationStart, "Expected '${' in interpolated string")?;
                    parts.push(self.parse_expression()?);
                    self.expect(&TokenKind::RightBrace, "Expected '}' after interpolation")?;
                    
                    match &self.peek().kind {
                        TokenKind::StringPart(s) => {
                            parts.push(Expression::String(s.clone()));
                            self.advance();
                        }
                        TokenKind::String(s) => {
                            parts.push(Expression::String(s.clone()));
                            self.advance();
                            break;
                        }
                        _ => return Err(FlowError::syntax(
                            "Expected string continuation after interpolation",
                            self.peek().line,
                            self.peek().column,
                        )),
                    }
                }
                Ok(Expression::InterpolatedString(parts))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expression::Boolean(true))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expression::Boolean(false))
            }
            // Hande both! and either! as boolean literals in primary expression context
            TokenKind::Both => {
                self.advance();
                Ok(Expression::Boolean(true))
            }
            TokenKind::Either => {
                self.advance();
                Ok(Expression::Boolean(false))
            }
            TokenKind::Identifier(name) => {
                let sigil_name = name.clone();
                let line = self.peek().line;
                self.advance();
                
                // Check for Sigil Instantiation: Identifier { key: value, ... }
                // Only treat as SigilInstance if:
                // 1. The '{' is on the SAME line as the identifier (to avoid consuming block braces)
                // 2. AND after '{', we see 'identifier :' pattern (not 'identifier =')
                if self.check(&TokenKind::LeftBrace) && self.peek().line == line {
                    // Look ahead to confirm this is actually a Sigil instantiation
                    // by checking if we have 'identifier :' pattern after '{'
                    let brace_pos = self.current;
                    self.advance(); // consume '{'
                    
                    let is_sigil_instance = if self.check(&TokenKind::RightBrace) {
                        // Empty braces {} - treat as empty SigilInstance
                        true
                    } else if let TokenKind::Identifier(_) = &self.peek().kind {
                        // Check if next token after identifier is ':'
                        let id_pos = self.current;
                        self.advance(); // consume identifier
                        let has_colon = self.check(&TokenKind::Colon);
                        self.current = id_pos; // restore position
                        has_colon
                    } else {
                        false
                    };
                    
                    if is_sigil_instance {
                        // Parse as Sigil Instantiation
                        let mut fields = Vec::new();
                        
                        if !self.check(&TokenKind::RightBrace) {
                            loop {
                                let key = if let TokenKind::Identifier(s) = &self.peek().kind {
                                    s.clone()
                                } else {
                                    return Err(FlowError::syntax(
                                        "Expected identifier key in Sigil instantiation!",
                                        self.peek().line,
                                        self.peek().column,
                                    ));
                                };
                                self.advance();
                                
                                self.expect(&TokenKind::Colon, "Expected ':' after field name")?;
                                let val = self.parse_expression()?;
                                
                                fields.push((key, val));
                                
                                if !self.match_token(&TokenKind::Comma) {
                                    break;
                                }
                            }
                        }
                        
                        self.expect(&TokenKind::RightBrace, "Expected '}' after Sigil fields")?;
                        Ok(Expression::SigilInstance { sigil_name, fields, line })
                    } else {
                        // Not a Sigil instantiation, backtrack and return just the identifier
                        self.current = brace_pos;
                        Ok(Expression::Identifier(sigil_name))
                    }
                } else {
                    Ok(Expression::Identifier(sigil_name))
                }
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(&TokenKind::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            TokenKind::LeftBracket => {
                self.advance();
                let elements = self.parse_arguments()?;
                self.expect(&TokenKind::RightBracket, "Expected ']' after array elements")?;
                Ok(Expression::Array { elements })
            }
            TokenKind::LeftBrace => {
                self.advance();
                let mut entries = Vec::new();
                
                if !self.check(&TokenKind::RightBrace) {
                    loop {
                        let key = if let TokenKind::String(s) = &self.peek().kind {
                            s.clone()
                        } else if let TokenKind::Identifier(s) = &self.peek().kind {
                            s.clone()
                        } else {
                            return Err(FlowError::syntax(
                                "Expected string or identifier key in Relic!",
                                self.peek().line,
                                self.peek().column,
                            ));
                        };
                        self.advance();
                        
                        self.expect(&TokenKind::Colon, "Expected ':' after key")?;
                        let value = self.parse_expression()?;
                        
                        entries.push((key, value));
                        
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                }
                
                self.expect(&TokenKind::RightBrace, "Expected '}' after Relic entries")?;
                Ok(Expression::Relic { entries })
            }
            TokenKind::CastSpell => {
                // Inline Spell: Spell (params) -> expr  OR  Spell (params) { block }
                self.advance(); // consume 'Spell'
                self.parse_inline_spell()
            }
            _ => Err(FlowError::syntax(
                &format!("Unexpected token: {}", self.peek().kind),
                self.peek().line,
                self.peek().column,
            )),
        }
    }
    
    fn parse_arguments(&mut self) -> Result<Vec<Expression>, FlowError> {
        let mut args = Vec::new();
        
        if self.check(&TokenKind::RightParen) || self.check(&TokenKind::RightBracket) {
            return Ok(args);
        }
        
        loop {
            args.push(self.parse_expression()?);
            
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
        
        Ok(args)
    }
    
    // Helper methods
    
    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn match_tokens(&mut self, kinds: &[TokenKind]) -> Option<TokenKind> {
        for kind in kinds {
            if self.check(kind) {
                let matched = self.peek().kind.clone();
                self.advance();
                return Some(matched);
            }
        }
        None
    }
    
    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn expect(&mut self, kind: &TokenKind, message: &str) -> Result<(), FlowError> {
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            Err(FlowError::syntax(
                message,
                self.peek().line,
                self.peek().column,
            ))
        }
    }
    
    fn expect_identifier(&mut self, message: &str) -> Result<String, FlowError> {
        match &self.peek().kind {
            TokenKind::Identifier(name) => {
                let value = name.clone();
                self.advance();
                Ok(value)
            }
            _ => Err(FlowError::syntax(
                message,
                self.peek().line,
                self.peek().column,
            )),
        }
    }
    
    // ⚔️ ERROR ARC - Parsing Methods
    fn parse_panic(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'panic'
        
        let message = self.parse_expression()?;
        
        Ok(Statement::Panic { message, line })
    }
    
    fn parse_wound(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'wound'
        
        let message = self.parse_expression()?;
        
        Ok(Statement::Wound { message, line })
    }
    
    fn parse_rupture(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'rupture'
        
        // Expect error type: Rift, Glitch, VoidTear, or Spirit
        let error_type = if let TokenKind::Identifier(name) = &self.peek().kind {
            if matches!(name.as_str(), "Rift" | "Glitch" | "VoidTear" | "Spirit") {
                let err_type = name.clone();
                self.advance();
                err_type
            } else {
                return Err(FlowError::syntax(
                    "Expected error type (Rift, Glitch, VoidTear, or Spirit) after 'rupture'",
                    self.peek().line,
                    self.peek().column,
                ));
            }
        } else {
            return Err(FlowError::syntax(
                "Expected error type after 'rupture'",
                self.peek().line,
                self.peek().column,
            ));
        };
        
        let message = self.parse_expression()?;
        
        Ok(Statement::Rupture {
            error_type,
            message,
            line,
        })
    }
    
    fn parse_attempt(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'attempt'
        
        self.expect(&TokenKind::LeftBrace, "Expected '{' after 'attempt'")?;
        
        let body = self.parse_block()?;
        self.expect(&TokenKind::RightBrace, "Expected '}' after attempt block")?;
        
        let mut rescue_clauses = Vec::new();
        let mut finally_block = None;
        
        // Parse rescue clauses
        while self.match_token(&TokenKind::Rescue) {
            let rescue = self.parse_rescue_clause()?;
            rescue_clauses.push(rescue);
        }
        
        // Parse optional finally block
        if self.match_token(&TokenKind::Finally) {
            self.expect(&TokenKind::LeftBrace, "Expected '{' after 'finally'")?;
            finally_block = Some(self.parse_block()?);
            self.expect(&TokenKind::RightBrace, "Expected '}' after finally block")?;
        }
        
        Ok(Statement::Attempt {
            body,
            rescue_clauses,
            finally_block,
            line,
        })
    }
    
    fn parse_rescue_clause(&mut self) -> Result<ast::RescueClause, FlowError> {
        // rescue [ErrorType] [as binding] [retry N] { body }
        let mut error_type = None;
        let mut binding = None;
        let mut retry_count = None;
        
        // Check for error type (e.g., "Rift", "Glitch")
        if let TokenKind::Identifier(name) = &self.peek().kind {
            if matches!(name.as_str(), "Rift" | "Glitch" | "VoidTear" | "Spirit") {
                error_type = Some(name.clone());
                self.advance();
            }
        }
        
        // Check for "as binding"
        if self.match_token(&TokenKind::As) {
            binding = Some(self.expect_identifier("Expected variable name after 'as'")?);
        }
        
        // Check for "retry N"
        if self.match_token(&TokenKind::Retry) {
            if let TokenKind::Number(n) = &self.peek().kind {
                retry_count = Some(*n as usize);
                self.advance();
            } else {
                return Err(FlowError::syntax(
                    "Expected number after 'retry'",
                    self.peek().line,
                    self.peek().column,
                ));
            }
        }
        
        self.expect(&TokenKind::LeftBrace, "Expected '{' after rescue clause")?;
        let body = self.parse_block()?;
        self.expect(&TokenKind::RightBrace, "Expected '}' after rescue block")?;
        
        Ok(ast::RescueClause {
            error_type,
            binding,
            retry_count,
            body,
        })
    }
    
    fn parse_rebound(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'rebound'
        
        // Optional: rebound <error_variable>
        let error = if let TokenKind::Identifier(name) = &self.peek().kind {
            let err_name = name.clone();
            self.advance();
            Some(err_name)
        } else {
            None
        };
        
        Ok(Statement::Rebound { error, line })
    }
    
    fn parse_ward(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'ward'
        
        self.expect(&TokenKind::LeftBrace, "Expected '{' after 'ward'")?;
        let body = self.parse_block()?;
        self.expect(&TokenKind::RightBrace, "Expected '}' after ward block")?;
        
        Ok(Statement::Ward { body, line })
    }
    
    
    fn parse_break_seal(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'break'
        
        // Expect 'seal' keyword
        self.expect(&TokenKind::Seal, "Expected 'seal' after 'break'")?;
        
        Ok(Statement::BreakSeal { line })
    }
    
    fn parse_fracture_seal(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'fracture'
        
        // Expect 'seal' keyword
        self.expect(&TokenKind::Seal, "Expected 'seal' after 'fracture'")?;
        
        Ok(Statement::FractureSeal { line })
    }
    
    fn parse_shatter_grand_seal(&mut self) -> Result<Statement, FlowError> {
        let line = self.peek().line;
        self.advance(); // consume 'shatter'
        
        // Expect 'grand_seal'
        if let TokenKind::GrandSeal = &self.peek().kind {
            self.advance();
        } else {
            return Err(FlowError::syntax(
                "Expected 'grand_seal' after 'shatter'",
                self.peek().line,
                self.peek().column,
            ));
        }
        
        // Optional return value
        let value = if !matches!(&self.peek().kind, TokenKind::RightBrace | TokenKind::Eof) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        Ok(Statement::ShatterGrandSeal { value, line })
    }
    
    fn expect_identifier_value(&mut self, expected: &str, message: &str) -> Result<(), FlowError> {
        match &self.peek().kind {
            TokenKind::Identifier(name) if name == expected => {
                self.advance();
                Ok(())
            }
            _ => Err(FlowError::syntax(
                message,
                self.peek().line,
                self.peek().column,
            )),
        }
    }
    
    // NEW: Parse inline Spell functions
    // Syntax: Spell (params) -> expr  OR  Spell (params) { block }
    // Also: Spell x -> expr (single param, no parens)
    fn parse_inline_spell(&mut self) -> Result<Expression, FlowError> {
        let line = self.previous().line; // Spell token
        
        let mut params = Vec::new();
        let param_types = Vec::new(); // Types not supported yet
        let return_type = None; // Not supported yet
        
        // Parse parameters
        if self.check(&TokenKind::LeftParen) {
            // Multi-param or no-param: Spell () or Spell (x, y)
            self.advance(); // consume '('
            
            if !self.check(&TokenKind::RightParen) {
                loop {
                    let param = self.expect_identifier("Expected parameter name")?;
                    params.push(param);
                    
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
            }
            
            self.expect(&TokenKind::RightParen, "Expected ')' after parameters")?;
        } else if let TokenKind::Identifier(name) = &self.peek().kind {
            // Single param shortcut: Spell x -> ...
            params.push(name.clone());
            self.advance();
        } else {
            return Err(FlowError::syntax(
                "Expected '(' or parameter name after 'Spell'",
                self.peek().line,
                self.peek().column,
            ));
        }
        
        // Parse body: -> expr  OR  { block }
        let body = if self.match_token(&TokenKind::Arrow) {
            // Expression body: Spell x -> x * 2
            let expr = self.parse_expression()?;
            InlineSpellBody::Expression(Box::new(expr))
        } else if self.match_token(&TokenKind::LeftBrace) {
            // Block body: Spell (x) { return x * 2 }
            let statements = self.parse_block()?;
            self.expect(&TokenKind::RightBrace, "Expected '}' after Spell block")?;
            InlineSpellBody::Block(statements)
        } else {
            return Err(FlowError::syntax(
                "Expected '->' or '{' after Spell parameters",
                self.peek().line,
                self.peek().column,
            ));
        };
        
        Ok(Expression::InlineSpell {
            params,
            param_types,
            return_type,
            body,
            line,
        })
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, FlowError> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
