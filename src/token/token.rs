
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TokenType {
  EOF,

  NULL,
  INT,
  FLOAT,
  CHAR,
  STRING,
  BOOL,

  IDENTIFIER,

  EQUALS,
  EQUALS_EQUALS,
  NOT,
  NOT_EQUALS,

  GREATER_THAN,
  GREATER_THAN_EQUALS,
  LESS_THAN,
  LESS_THAN_EQUALS,

  OR,
  AND,

  DOT,
  DOT_DOT,
  SEMICOLON,
  COLON,
  QUESTION,
  COMMA,

  LEFT_PAREN,
  RIGHT_PAREN,
  LEFT_BRACKET,
  RIGHT_BRACKET,
  LEFT_CURLY,
  RIGHT_CURLY,

  PLUS,
  PLUS_EQUALS,
  MINUS,
  MINUS_EQUALS,
  MULTIPLY,
  MULTIPLY_EQUALS,
  DIVIDE,
  DIVIDE_EQUALS,
  MOD,

  LET,
	CONST,
	CLASS,
	NEW,
	IMPORT,
	FROM,
	FUNCTION,
	IF,
	ELSE,
	FOREACH,
	WHILE,
	FOR,
	EXPORT,
	TYPEOF,
	IN,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
  pub r#type: TokenType,
  pub value: String,
}