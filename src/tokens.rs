#[derive(Debug, PartialEq)]
pub enum Token {
    STRING(String),
    NUMBER(f64),
    BOOLEAN(bool),
    NULL,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    COMMA,
    COLON,
}
