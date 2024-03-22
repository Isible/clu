use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Token {
    LBrac,
    RBrac,
    LCurly,
    RCurly,
    Ident(String),
    Literal(TomlLiteral),
    Assign,
    Dot,
    Newline,
    EOF,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum TomlLiteral {
    String(String),
    // TODO: Migrate numbers to large numbers
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Date(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LBrac => write!(f, "["),
            Token::RBrac => write!(f, "]"),
            Token::LCurly => write!(f, "{{"),
            Token::RCurly => write!(f, "}}"),
            Token::Ident(ident) => ident.fmt(f),
            Token::Literal(lit) => lit.fmt(f),
            Token::Assign => write!(f, "="),
            Token::Dot => write!(f, "."),
            Token::Newline => write!(f, "Newline"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

impl Display for TomlLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlLiteral::String(s) => write!(f, "{}", s),
            TomlLiteral::Integer(i) => write!(f, "{}", i),
            TomlLiteral::Float(fl) => write!(f, "{}", fl),
            TomlLiteral::Boolean(b) => write!(f, "{}", b),
            TomlLiteral::Date(d) => write!(f, "{}", d),
        }
    }
}
