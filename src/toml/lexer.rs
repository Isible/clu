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
        match self.ch {
            Some('\n') => {
                self.read_char();
                Token::Newline
            }
            Some('[') => Token::Table(self.read_table()),
            None => Token::EOF,
            _ => Token::Key(self.read_key()),
        }
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

    fn read_key(&mut self) -> String {
        let start = self.pos;
        while let Some(ch) = self.ch {
            if ch == '\n' {
                break;
            }
            self.read_char();
        }
        self.input[start..self.pos].to_string()
    }

    fn read_table(&mut self) -> String {
        let start = self.pos;
        loop {
            self.read_char();
            if self.ch == Some(']') {
                break;
            }
        }
        self.read_char();
        self.input[start+1..self.pos - 1].to_string()
    }
}
