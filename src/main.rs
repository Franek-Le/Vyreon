mod lexer;
mod token;

use crate::lexer::lexer::Lexer;
use crate::token::token::Token;

fn main() {
    let mut l: Lexer = Lexer::new("+-*/%?{}[]()");
    let mut tokens: Vec<Token> = l.tokenize();

    for token in tokens.iter_mut() {
        println!("{:?}", token);
    }
}