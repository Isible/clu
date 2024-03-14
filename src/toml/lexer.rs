use super::tokens::Token;

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
            Some('[') => Token::Table(self.read_table()),
            Some('"') | Some('\'') => Token::String(self.read_string()),
            Some('=') => Token::Assign,
            Some('.') => Token::Dot,
            None => Token::EOF,
            _ => Token::Key(self.read_key()),
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

    fn skip_whitespace(&mut self) {
        while self.ch == Some(' ') || self.ch == Some('\t') {
            self.read_char();
        }
    }
}
