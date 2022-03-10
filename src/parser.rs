use crate::lexer::TokenKind;

pub(crate) fn parse(mut tokens: impl Iterator<Item = TokenKind>) -> Vec<Statement> {
    parse_inner(tokens, true)
}

fn parse_inner(mut tokens: impl Iterator<Item = TokenKind>, top_level: bool) -> Vec<Statement> {
    let mut statements = Vec::new();
    while let Some(token) = tokens.next() {
        statements.push(match token {
            TokenKind::Sus => Statement::Sus,
            TokenKind::Vented => Statement::Vented,
            TokenKind::Electrical => Statement::Electrical,
            TokenKind::Who => {
                let s = parse_inner(&mut tokens, false);
                assert_eq!(tokens.next(), Some(TokenKind::Where));
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
            TokenKind::Error => todo!(), // Error handling 💯
        });
    }

    statements
}

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
