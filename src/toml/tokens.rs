use std::fmt::Display;

#[derive(Debug)]
pub enum Token {
    Table(String),
    /// This represents a key in a key-value pair.
    Key(String),
    /// This represents a string key or value in a key-value pair.
    String(String),
    Boolean(bool),
    // TODO: Use large nums lib for this once that is finished
    Integer(i64),
    Date(String),
    Assign,
    Dot,
    Newline,
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Table(s) => write!(f, "Table({})", s),
            Token::Key(s) => write!(f, "Key({})", s),
            Token::String(s) => write!(f, "String({})", s),
            Token::Assign => write!(f, "Assign"),
            Token::Dot => write!(f, "Dot"),
            Token::Newline => write!(f, "Newline"),
            Token::Integer(i) => write!(f, "Integer({})", i),
            Token::Boolean(b) => write!(f, "Boolean({})", b),
            Token::Date(s) => write!(f, "Date({})", s),
            Token::EOF => write!(f, "EOF"),
        }
    }
}
