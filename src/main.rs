mod lexer;
mod token;

use crate::lexer::lexer::Lexer;
use crate::token::token::Token;

fn main() {
    let mut l: Lexer = Lexer::new("+-*/%?{}[]()123123123    5999.4 hello world _hello world class");
    let mut tokens: Vec<Token> = l.tokenize();

    for token in tokens.iter_mut() {
        println!("{:?}", token);
    }
}