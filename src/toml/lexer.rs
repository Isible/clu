use super::tokens::{Token, TomlLiteral};

pub struct Lexer {
    pub(crate) input: String,
    pub(crate) pos: usize,
    pub(crate) read_pos: usize,
    pub(crate) ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            pos: 0,
            read_pos: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    pub fn tokenize(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            Some('\n') => Token::Newline,
            Some('[') => Token::LBrac,
            Some(']') => Token::RBrac,
            Some('{') => Token::LCurly,
            Some('}') => Token::RCurly,
            Some('=') => Token::Assign,
            Some('.') => Token::Dot,
            Some('"') | Some('\'') => Token::Literal(TomlLiteral::String(self.read_string())),
            Some(ch) => {
                if ch.is_numeric() {
                    let num = self.read_number();
                    if num.contains('.') {
                        Token::Literal(TomlLiteral::Float(num.parse().unwrap()))
                    } else {
                        Token::Literal(TomlLiteral::Integer(num.parse().unwrap()))
                    }
                } else {
                    let key = self.read_key();
                    match key.as_str() {
                        "true" => Token::Literal(TomlLiteral::Boolean(true)),
                        "false" => Token::Literal(TomlLiteral::Boolean(false)),
                        _ => Token::Ident(key),
                    }
                }
            }
            None => Token::EOF,
        };
        self.read_char();
        tok
    }

    fn read_key(&mut self) -> String {
        let start = self.pos;
        while let Some(ch) = self.ch {
            let next = if let Some(ch) = self.input.chars().nth(self.read_pos) {
                ch
            } else {
                break;
            };
            if !next.is_alphanumeric() && next != '_' && next != '-' {
                break;
            }
            self.read_char();
        }
        self.input[start..self.read_pos].to_string()
    }

    fn read_table(&mut self) -> String {
        let start = self.read_pos;
        loop {
            self.read_char();
            if self.ch == Some(']') {
                break;
            }
        }
        self.input[start..self.pos].to_string()
    }

    fn read_string(&mut self) -> String {
        let quot_marks = if let Some(ch) = self.ch {
            if ch == '"' {
                true
            } else {
                false
            }
        } else {
            false
        };
        let start = self.pos;
        loop {
            self.read_char();
            if self.ch == Some('"') && quot_marks {
                break;
            } else if self.ch == Some('\'') && !quot_marks {
                break;
            }
        }
        self.input[start + 1..self.pos].to_string()
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input.chars().nth(self.read_pos).unwrap());
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn read_number(&mut self) -> String {
        let start = self.pos;
        while let Some(ch) = self.ch {
            if !ch.is_numeric() && ch != '.' {
                break;
            }
            self.read_char();
        }
        self.input[start..self.pos].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == Some(' ') || self.ch == Some('\t') {
            self.read_char();
        }
    }
}
