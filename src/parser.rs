use crate::lexer::TokenKind;
use std::iter::Peekable;

pub(crate) fn parse(tokens: logos::Lexer<'_, TokenKind>) -> Vec<Statement> {
    parse_inner(&mut tokens.peekable(), true)
}

fn parse_inner(
    tokens: &mut Peekable<logos::Lexer<'_, TokenKind>>,
    top_level: bool,
) -> Vec<Statement> {
    let mut statements = Vec::new();
    while let Some(token) = tokens.peek() {
        statements.push(match token {
            TokenKind::Sus => Statement::Sus,
            TokenKind::Vented => Statement::Vented,
            TokenKind::Sussy => Statement::Sussy,
            TokenKind::Electrical => Statement::Electrical,
            TokenKind::Who => {
                tokens.next();
                let s = parse_inner(tokens, false);
                assert_eq!(tokens.peek(), Some(&TokenKind::Where));
                Statement::Block(s)
            }
            TokenKind::Where if top_level => todo!(),
            TokenKind::Where => break,
            TokenKind::Red => Statement::Red,
            TokenKind::Blue => Statement::Blue,
            TokenKind::Purple => Statement::Purple,
            TokenKind::Green => Statement::Green,
            TokenKind::Yellow => Statement::Yellow,
            TokenKind::Cyan => Statement::Cyan,
            TokenKind::Black => Statement::Black,
            TokenKind::White => Statement::White,
            TokenKind::Brown => Statement::Brown,
            TokenKind::Lime => Statement::Lime,
            TokenKind::Pink => Statement::Pink,
            TokenKind::Orange => Statement::Orange,
            TokenKind::Error => panic!(), // Error handling 💯
        });
        tokens.next();
    }

    statements
}

#[derive(Debug)]
pub(crate) enum Statement {
    Sus,
    Vented,
    Sussy,
    Electrical,
    Block(Vec<Statement>),
    Red,
    Blue,
    Purple,
    Green,
    Yellow,
    Cyan,
    Black,
    White,
    Brown,
    Lime,
    Pink,
    Orange,
}
