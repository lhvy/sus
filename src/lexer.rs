use logos::Logos;

#[derive(Logos, PartialEq, Debug)]
pub(crate) enum TokenKind {
    #[token("SUS")]
    Sus,

    #[token("VENTED")]
    Vented,

    #[token("ELECTRICAL")]
    Electrical,

    #[regex("WHO\\??")]
    Who,

    #[token("WHERE\\??")]
    Where,

    #[token("RED")]
    Red,

    #[token("BLUE")]
    Blue,

    #[token("PURPLE")]
    Purple,

    #[token("GREEN")]
    Green,

    #[token("YELLOW")]
    Yellow,

    #[token("CYAN")]
    Cyan,

    #[token("BLACK")]
    Black,

    #[token("WHITE")]
    White,

    #[token("BROWN")]
    Brown,

    #[token("LIME")]
    Lime,

    #[token("PINK")]
    Pink,

    #[token("ORANGE")]
    Orange,

    #[regex("[ \n\t]+", logos::skip)]
    #[error]
    Error,
}
