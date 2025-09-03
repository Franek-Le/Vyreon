use crate::token::token::{Token, TokenType};

pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        let chars: Vec<char> = src.chars().collect();
        Lexer {
            source: chars,
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();

        while self.position < self.source.len() {
            let current = self.source[self.position];

            match current {
                '(' => result.push(Token {r#type: TokenType::LEFT_PAREN, value: "(".to_string(), }),
                ')' => result.push(Token {r#type: TokenType::RIGHT_PAREN, value: ")".to_string(), }),
                '[' => result.push(Token {r#type: TokenType::LEFT_BRACKET, value: "[".to_string(), }),
                ']' => result.push(Token {r#type: TokenType::RIGHT_BRACKET, value: "]".to_string(), }),
                '{' => result.push(Token {r#type: TokenType::LEFT_CURLY, value: "{".to_string(), }),
                '}' => result.push(Token {r#type: TokenType::RIGHT_CURLY, value: "}".to_string(), }),
                '+' => result.push(Token {r#type: TokenType::PLUS, value: "+".to_string(), }),
                '-' => result.push(Token {r#type: TokenType::MINUS, value: "-".to_string(), }),
                '*' => result.push(Token {r#type: TokenType::MULTIPLY, value: "*".to_string(), }),
                '/' => result.push(Token {r#type: TokenType::DIVIDE, value: "/".to_string(), }),
                '%' => result.push(Token {r#type: TokenType::MOD, value: "%".to_string(), }),
                '.' => result.push(Token {r#type: TokenType::DOT, value: ".".to_string(), }),
                ',' => result.push(Token {r#type: TokenType::COMMA, value: ",".to_string(), }),
                ':' => result.push(Token {r#type: TokenType::COLON, value: ":".to_string(), }),
                ';' => result.push(Token {r#type: TokenType::SEMICOLON, value: ";".to_string(), }),
                '?' => result.push(Token {r#type: TokenType::QUESTION, value: "?".to_string(), }),
                '=' => result.push(Token {r#type: TokenType::EQUALS, value: "=".to_string(), }),
                '!' => result.push(Token {r#type: TokenType::NOT, value: "!".to_string(), }),
                '<' => result.push(Token {r#type: TokenType::LESS_THAN, value: "<".to_string(), }),
                '>' => result.push(Token {r#type: TokenType:: GREATER_THAN, value: ">".to_string(), }),

                _ => {
                    if current.is_alphabetic() || current == '_' {
                        let ident = self.consume_identifier();
                        let token_type = match ident.as_str() {
                            "const" => TokenType::CONST,
                            "var" => TokenType::LET,

                            "if" => TokenType::IF,
                            "else" => TokenType::ELSE,
                            "or" => TokenType::OR,
                            "and" => TokenType::AND,

                            "new" => TokenType:: NEW,
                            "class" => TokenType::CLASS,

                            "function" => TokenType::FUNCTION,
                            "return" => TokenType::RETURN,

                            "for" => TokenType::FOR,
                            "foreach" => TokenType::FOREACH,
                            "while" => TokenType::WHILE,

                            "import" => TokenType::IMPORT,
                            "export" => TokenType::EXPORT,
                            "from" => TokenType::FROM,

                            "struct" => TokenType::STRUCT,
                            "interface" => TokenType::INTERFACE,
                            "enum" => TokenType::ENUM,
                            
                            "typeof" => TokenType::TYPEOF,
                            _ => TokenType::IDENTIFIER,
                        };

                        result.push(Token {
                            r#type: token_type,
                            value: ident,
                        });
                    } else if current.is_numeric() {
                        let number = self.consume_number();
                        
                        let mut token_type: TokenType = TokenType::INT;

                        if number.contains('.') {
                            token_type = TokenType::FLOAT;
                        }

                        result.push(Token {
                            r#type: token_type,
                            value: number,
                        });
                    } else if current.is_whitespace() {
                        self.advance();
                        continue;
                    } else {
                        panic!("Unrecognized Character Error: {}", current.to_string());                    }
                }
            }

            self.advance();
        }

        result.push(Token {
            r#type: TokenType::EOF,
            value: "EOF".to_string(),
        });

        result
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    #[allow(dead_code)]
    fn peek(&self) -> Option<char> {
        if self.position + 1 < self.source.len() {
            Some(self.source[self.position + 1])
        } else {
            None
        }
    }

    fn consume_identifier(&mut self) -> String {
        let start = self.position;

        while self.position < self.source.len() && (self.source[self.position].is_alphanumeric() || self.source[self.position] == '_') {
            self.position += 1;
        }

        self.source[start..self.position].iter().collect()
    }

    fn consume_number(&mut self) -> String {
        let start = self.position;
        let mut has_dot = false;

        while self.position < self.source.len() {
            let ch = self.source[self.position];

            if ch.is_digit(10) {
                self.position += 1;
            } else if ch == '.' && !has_dot {
                // Peek next character to ensure it's a digit (to allow floats like "3.14")
                if self.position + 1 < self.source.len() && self.source[self.position + 1].is_digit(10) {
                    has_dot = true;
                    self.position += 1; // consume the dot
                } else {
                    break; // stop if dot is not followed by digit (e.g., "3.")
                }
            } else {
                break;
            }
        }

        self.source[start..self.position].iter().collect()
    }


}
