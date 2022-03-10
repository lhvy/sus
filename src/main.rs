use eval::eval;
use lexer::TokenKind;
use logos::Logos;

mod eval;
mod lexer;
mod parser;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap();

    let code = std::fs::read_to_string(path).unwrap();
    let lexer = TokenKind::lexer(&code);
    let statements = parser::parse(lexer);
    eval(&statements);
}
