use crate::token::token::{Token, TokenType};
use crate::ast::ast;

pub struct Parser {
  tokens: Vec<Token>,
  pos: i64,
}

impl Parser {
  fn new(self, tokens: Vec<Token>) -> self {

    return Parser {
      tokens: tokens,
      pos: 0,
    }
  }

  fn parse(self) -> ast::BlockStmt {
    let mut body: Vec = Vec::new();

    for token in self.hasTokens() {
      body.push(parse_stmt())
    }

    return ast::BlockStmt {
      body: body
    }
  }

  fn currentToken(self) -> Token {
    return self.tokens[self.pos]
  }

  fn currentTokenType(self) -> TokenType {
    let mut token = self.currentToken();
    return token::r#type;
  }

  fn advance(self) -> Token {
    let mut tk = self.currentToken();
    self.pos += 1;
    return tk;
  }

  fn hasTokens(self) -> Bool {
    return self.pos < len(self.tokens) && self.currentTokenType != TokenType::EOF;
  }

  fn expectError(self, expectedType: TokenType) -> Token {
    let mut token = self.currentToken();
    let mut _type = token::r#type;

    if _type != expectedType {
      panic!("Expected {} but received {} instead",expectedType.to_string(),  _type.to_string())
    }

    return self.advance();
  }

  fn expect(expectedType: TokenType) -> Token {
    return self.expectError(expecedType);
  }

}