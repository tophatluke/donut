#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Integer(usize),
    Assignment,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Equal,
    NotEqual,
    LT,
    GT,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Illegal,
}
