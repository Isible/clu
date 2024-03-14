use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Key(String),
    Value(String),
    Table(String),
    Array(String),
    Assign,
    Newline,
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Key(key) => write!(f, "Key: {}", key),
            Token::Value(value) => write!(f, "Value: {}", value),
            Token::Table(table) => write!(f, "Table: {}", table),
            Token::Array(array) => write!(f, "Array: {}", array),
            Token::Newline => write!(f, "Newline"),
            Token::EOF => write!(f, "EOF"),
            Token::Assign => write!(f, "Assign"),
        }
    }
}