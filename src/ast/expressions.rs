use crate::token::token::{Token, TokenType};

pub struct IntExpr {
  value: i64
}

impl IntExpr {
  pub fn expr() {}
}

pub struct FloatExpr {
  value: f64
}

impl FloatExpr {
  pub fn expr() {}
}

pub struct StringExpr {
  value: String
}

impl StringExpr {
  pub fn expr() {}
}

pub struct SymbolExpr {
  value: String
}

impl SymbolExpr {
  pub fn expr() {}
}

pub struct BinaryExpr {
  left: Expr,
  operator: Token,
  right: Expr,
}

impl BinaryExpr {
  pub fn expr() {}
}

pub struct PrefixExpr {
  operator: Token,
  right: Expr
}

impl PrefixExpr {
  pub fn expr() {}
}

pub struct AssignmentExpr {
  assigne: Expr,
  operator: Token,
  value: Expr,
}

impl AssignmentExpr {
  pub fn expr() {}
}