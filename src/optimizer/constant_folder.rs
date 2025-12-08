use crate::parser::ast::*;
use std::sync::Arc;

/// Constant folding optimizer - evaluates constant expressions at compile time
pub struct ConstantFolder;

impl ConstantFolder {
    pub fn new() -> Self {
        ConstantFolder
    }

    pub fn fold(&self, program: Program) -> Program {
        Program {
            imports: program.imports,
            statements: program.statements.into_iter()
                .map(|stmt| self.fold_statement(stmt))
                .collect(),
        }
    }

    fn fold_statement(&self, stmt: Statement) -> Statement {
        match stmt {
            Statement::Let { name, type_annotation, value, is_exported, line } => {
                Statement::Let {
                    name,
                    type_annotation,
                    value: self.fold_expression(value),
                    is_exported,
                    line,
                }
            }
            Statement::Seal { name, type_annotation, value, is_exported, line } => {
                Statement::Seal {
                    name,
                    type_annotation,
                    value: self.fold_expression(value),
                    is_exported,
                    line,
                }
            }
            Statement::Assignment { name, value, line } => {
                Statement::Assignment {
                    name,
                    value: self.fold_expression(value),
                    line,
                }
            }
            Statement::FunctionDecl { name, params, return_type, body, sigils, is_exported, line } => {
                Statement::FunctionDecl {
                    name,
                    params,
                    return_type,
                    body: body.into_iter().map(|s| self.fold_statement(s)).collect(),
                    sigils,
                    is_exported,
                    line,
                }
            }
            Statement::Return { value, line } => {
                Statement::Return {
                    value: value.map(|v| self.fold_expression(v)),
                    line,
                }
            }
            Statement::Expression { expr, line } => {
                Statement::Expression {
                    expr: self.fold_expression(expr),
                    line,
                }
            }
            Statement::Stance { condition, then_branch, shift_branches, abandon_branch, line } => {
                Statement::Stance {
                    condition: self.fold_expression(condition),
                    then_branch: then_branch.into_iter().map(|s| self.fold_statement(s)).collect(),
                    shift_branches: shift_branches.into_iter().map(|(cond, block)| {
                        (self.fold_expression(cond), block.into_iter().map(|s| self.fold_statement(s)).collect())
                    }).collect(),
                    abandon_branch: abandon_branch.map(|block| block.into_iter().map(|s| self.fold_statement(s)).collect()),
                    line,
                }
            }
            Statement::Aura { value, cases, otherwise, line } => {
                Statement::Aura {
                    value: self.fold_expression(value),
                    cases: cases.into_iter().map(|(pattern, stmts)| {
                        (self.fold_expression(pattern), stmts.into_iter().map(|s| self.fold_statement(s)).collect())
                    }).collect(),
                    otherwise: otherwise.map(|stmts| stmts.into_iter().map(|s| self.fold_statement(s)).collect()),
                    line,
                }
            }
            Statement::Phase { kind, body, line } => {
                let folded_kind = match kind {
                    PhaseKind::Count { variable, from, to } => {
                        PhaseKind::Count {
                            variable,
                            from: self.fold_expression(from),
                            to: self.fold_expression(to),
                        }
                    }
                    PhaseKind::Until { condition } => {
                        PhaseKind::Until {
                            condition: self.fold_expression(condition),
                        }
                    }
                    PhaseKind::ForEach { variable, collection } => {
                        PhaseKind::ForEach {
                            variable,
                            collection: self.fold_expression(collection),
                        }
                    }
                    PhaseKind::Forever => PhaseKind::Forever,
                };
                Statement::Phase {
                    kind: folded_kind,
                    body: body.into_iter().map(|s| self.fold_statement(s)).collect(),
                    line,
                }
            }
            Statement::Ritual { name, params, return_type, body, is_exported, line } => {
                Statement::Ritual {
                    name,
                    params,
                    return_type,
                    body: body.into_iter().map(|s| self.fold_statement(s)).collect(),
                    is_exported,
                    line,
                }
            }
            Statement::Attempt { body, rescue_clauses, finally_block, line } => {
                Statement::Attempt {
                    body: body.into_iter().map(|s| self.fold_statement(s)).collect(),
                    rescue_clauses: rescue_clauses.into_iter().map(|clause| {
                        RescueClause {
                            error_type: clause.error_type,
                            binding: clause.binding,
                            retry_count: clause.retry_count,
                            body: clause.body.into_iter().map(|s| self.fold_statement(s)).collect(),
                        }
                    }).collect(),
                    finally_block: finally_block.map(|block| block.into_iter().map(|s| self.fold_statement(s)).collect()),
                    line,
                }
            }
            Statement::Ward { body, line } => {
                Statement::Ward {
                    body: body.into_iter().map(|s| self.fold_statement(s)).collect(),
                    line,
                }
            }
            // Other statements don't need folding
            other => other,
        }
    }

    fn fold_expression(&self, expr: Expression) -> Expression {
        match expr {
            // Binary operations - the main optimization target
            Expression::Binary { left, operator, right } => {
                let left = self.fold_expression(*left);
                let right = self.fold_expression(*right);

                // Try to evaluate if both sides are constants
                if let Some(result) = self.try_fold_binary(&left, operator, &right) {
                    return result;
                }

                Expression::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }
            }

            // Unary operations
            Expression::Unary { operator, operand } => {
                let operand = self.fold_expression(*operand);

                if let Some(result) = self.try_fold_unary(operator, &operand) {
                    return result;
                }

                Expression::Unary {
                    operator,
                    operand: Box::new(operand),
                }
            }

            // Array literals
            Expression::Array { elements } => {
                Expression::Array {
                    elements: elements.into_iter().map(|e| self.fold_expression(e)).collect(),
                }
            }

            // Relic literals
            Expression::Relic { entries } => {
                Expression::Relic {
                    entries: entries.into_iter().map(|(k, v)| (k, self.fold_expression(v))).collect(),
                }
            }

            // NEW: Sigil Instantiation
            Expression::SigilInstance { sigil_name, fields, line } => {
                Expression::SigilInstance {
                    sigil_name,
                    fields: fields.into_iter().map(|(k, v)| (k, self.fold_expression(v))).collect(),
                    line,
                }
            }

            // Function calls
            Expression::Call { callee, arguments } => {
                Expression::Call {
                    callee: Box::new(self.fold_expression(*callee)),
                    arguments: arguments.into_iter().map(|a| self.fold_expression(a)).collect(),
                }
            }

            // Method calls
            Expression::MethodCall { object, method, arguments } => {
                Expression::MethodCall {
                    object: Box::new(self.fold_expression(*object)),
                    method,
                    arguments: arguments.into_iter().map(|a| self.fold_expression(a)).collect(),
                }
            }

            // Index access
            Expression::Index { object, index } => {
                Expression::Index {
                    object: Box::new(self.fold_expression(*object)),
                    index: Box::new(self.fold_expression(*index)),
                }
            }

            // Inline spells
            Expression::InlineSpell { params, param_types, return_type, body, line } => {
                let folded_body = match body {
                    InlineSpellBody::Expression(expr) => {
                        InlineSpellBody::Expression(Box::new(self.fold_expression(*expr)))
                    }
                    InlineSpellBody::Block(stmts) => {
                        InlineSpellBody::Block(stmts.into_iter().map(|s| self.fold_statement(s)).collect())
                    }
                };
                Expression::InlineSpell {
                    params,
                    param_types,
                    return_type,
                    body: folded_body,
                    line,
                }
            }

            // Literals and identifiers don't need folding
            other => other,
        }
    }

    /// Try to fold a binary operation if both operands are constants
    fn try_fold_binary(&self, left: &Expression, op: BinaryOp, right: &Expression) -> Option<Expression> {
        use BinaryOp::*;

        match (left, right) {
            // Arithmetic on numbers
            (Expression::Number(a), Expression::Number(b)) => {
                let result = match op {
                    Add => a + b,
                    Subtract => a - b,
                    Multiply => a * b,
                    Divide => {
                        if *b == 0.0 {
                            return None; // Don't fold division by zero
                        }
                        a / b
                    }
                    Modulo => {
                        if *b == 0.0 {
                            return None;
                        }
                        a % b
                    }
                    Greater => return Some(Expression::Boolean(a > b)),
                    Less => return Some(Expression::Boolean(a < b)),
                    GreaterEq => return Some(Expression::Boolean(a >= b)),
                    LessEq => return Some(Expression::Boolean(a <= b)),
                    IsEqual => return Some(Expression::Boolean((a - b).abs() < f64::EPSILON)),
                    NotEqual => return Some(Expression::Boolean((a - b).abs() >= f64::EPSILON)),
                    _ => return None,
                };
                Some(Expression::Number(result))
            }

            // String concatenation
            (Expression::String(a), Expression::String(b)) if matches!(op, Add) => {
                Some(Expression::String(format!("{}{}", a, b)))
            }

            // Boolean operations
            (Expression::Boolean(a), Expression::Boolean(b)) => {
                let result = match op {
                    Both => *a && *b,
                    Either => *a || *b,
                    IsEqual => a == b,
                    NotEqual => a != b,
                    _ => return None,
                };
                Some(Expression::Boolean(result))
            }

            _ => None,
        }
    }

    /// Try to fold a unary operation if operand is constant
    fn try_fold_unary(&self, op: UnaryOp, operand: &Expression) -> Option<Expression> {
        match (op, operand) {
            (UnaryOp::Minus, Expression::Number(n)) => Some(Expression::Number(-n)),
            (UnaryOp::Negate, Expression::Boolean(b)) => Some(Expression::Boolean(!b)),
            _ => None,
        }
    }
}
