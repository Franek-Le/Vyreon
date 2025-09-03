use crate::ast::ast;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BindingPower {
    Default = 0,
    Comma,
    Assignment,
    Logical,
    Relational,
    Additive,
    Multiplicative,
    Unary,
    Call,
    Member,
    Primary,
}

pub type StmtHandler = fn(p: &mut Parser) -> Box<dyn Stmt>;
pub type NudHandler = fn(p: &mut Parser) -> Box<dyn Expr>;
pub type LedHandler = fn(p: &mut Parser, left: Box<dyn Expr>, bp: BindingPower) -> Box<dyn Expr>;

pub type StmtLookup = HashMap<TokenKind, StmtHandler>;
pub type NudLookup = HashMap<TokenKind, NudHandler>;
pub type LedLookup = HashMap<TokenKind, LedHandler>;
pub type BpLookup = HashMap<TokenKind, BindingPower>;

pub static mut BP_LU: Option<BpLookup> = None;
pub static mut NUD_LU: Option<NudLookup> = None;
pub static mut LED_LU: Option<LedLookup> = None;
pub static mut STMT_LU: Option<StmtLookup> = None;

pub fn led(_type: Token, bp: BindingPower, led_fn: LedHandler) {
  BP_LU[_type] = bp;
  LED_LU[_type] = led_fn;
}

pub fn nud(_type: Token, nud_fn: NudHandler) {
  NUD_LU[_type] = nud_fn;
}

pub fn stmt(_type: Token, stmt_fn: StmtHandler) {
  BP_LU[_type] = Default;
  STMT_LU[_type] = stmt_fn;
}

fn createTokenLookups() {
	led(TokenType::EQUALS, assignment, parse_assignment_expr);
	led(TokenType::PLUS_EQUALS, assignment, parse_assignment_expr);
	led(TokenType::MINUS_EQUALS, assignment, parse_assignment_expr);
  led(TokenType::MULTIPLY_EQUALS, assignment, parse_assignment_expr);
  led(TokenType::DIVIDE_EQUALS, assignment, parse_assignment_expr);

	// Logical
	led(TokenType::AND, logical, parse_binary_expr);
	led(TokenType::OR, logical, parse_binary_expr);
	led(TokenType::DOT_DOT, logical, parse_binary_expr);

	// Relational
	led(TokenType::LESS_THAN, relational, parse_binary_expr);
	led(TokenType::LESS_THAN_EQUALS, relational, parse_binary_expr);
	led(TokenType::GREATER_THAN, relational, parse_binary_expr);
	led(TokenType::GREATER_THAN_EQUALS, relational, parse_binary_expr);
  led(TokenType::NOT, relational, parse_binary_expr);
	led(TokenType::NOT_EQUALS, relational, parse_binary_expr);

	// Additive & Multiplicative
	led(TokenType::PLUS, additive, parse_binary_expr);
	led(TokenType::MINUS, additive, parse_binary_expr);

	led(TokenType::MULTIPLY, multiplicative, parse_binary_expr);
	led(TokenType::DIVIDE, multiplicative, parse_binary_expr);
	led(TokenType::MOD, multiplicative, parse_binary_expr);

	// Literals & Symbols
	nud(TokenType::NUMBER, parse_primary_expr);
	nud(TokenType::STRING, parse_primary_expr);
	nud(TokenType::IDENTIFIER, parse_primary_expr);
	nud(TokenType::LEFT_PAREN, parse_grouping_expr);
	nud(TokenType::MINUS, parse_prefix_expr);

	// Statements
	stmt(TokenType::CONST, parse_var_decl_stmt);
	stmt(TokenType::LET, parse_var_decl_stmt);
}
