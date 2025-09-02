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
            let token_type = match current {
                '(' => TokenType::LEFT_PAREN,
                ')' => TokenType::RIGHT_PAREN,
                '[' => TokenType::LEFT_BRACKET,
                ']' => TokenType::RIGHT_BRACKET,
                '{' => TokenType::LEFT_CURLY,
                '}' => TokenType::RIGHT_CURLY,
                '?' => TokenType::QUESTION,
                '=' => TokenType::EQUALS,
                '+' => TokenType::PLUS,
                '-' => TokenType::MINUS,
                '*' => TokenType::MULTIPLY,
                '/' => TokenType::DIVIDE,
                '%' => TokenType::MOD,
                '.' => TokenType::DOT,
                ',' => TokenType::COMMA,
                ';' => TokenType::SEMICOLON,
                ':' => TokenType::COLON,
                '!' => TokenType::NOT,
                _ => {
                    if (current.to_string().parse::<f64>().is_ok()) {
                        let num: string = ""
                        
                    }
                },
            };
            result.push(Token {
                r#type: token_type,
                value: current.to_string(),
            });
            self.advance();
        }
        
        result.push(Token {
          r#type: TokenType::EOF,
          value: "EOF".to_string(),
        });

        return result
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
}