use crate::impl_trait;

/// Trait to mark a data structure like an enum as valid tokens
pub trait Token {
}

// Built in tokens to cover most needs
pub enum Tokens {
    Function,
    Struct,
    Class,
    Trait,
    Interface,
    Enum,
    Type,
    Const,
    Var,
    Let,

    If,
    Switch,
    Match,
    When,
    Case,
    While,
    For,
    Loop,

    Return,
    Break,

    Public,
    Private,
    Global,
    Local,

    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),

    // TODO: Special characters
    LParent,
    RParent,
    LBrac,
    RBrac,
    LCurly,
    RCurly,

    Colon,
    Semicolon,
    Comma,
    Dot,
    Pipe,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Ampersand,
    Dollar,
    Tilde,
    Apostrophe,
    Quotmark,
    Hashtag,
    At,
    LessThan,
    GreaterThan,
    ExclamMark,
    QuestionMark,
    Assign,
    Equals,
    NotEquals,
    GreaterOrEqual,
    LessOrEqual,
    Percent,
    Arrow,
    ThickArrow,
    Section,
}

impl_trait!(Token => Tokens);