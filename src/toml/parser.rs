use std::mem;

use super::{
    ast::{KeyVal, TomlExpr, TomlStmt},
    lexer::Lexer,
    tokens::{Token, TomlLiteral},
};
use crate::expect_peek;

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,

    pub cur_tok: Token,
    pub next_tok: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let cur_tok = lexer.tokenize();
        let next_tok = lexer.tokenize();
        Self {
            lexer: lexer,
            cur_tok,
            next_tok,
        }
    }

    pub fn parse_stmt(&mut self) -> Option<TomlStmt> {
        match &self.cur_tok {
            Token::LBrac => Some(self.parse_table()),
            Token::Ident(_) | Token::Literal(TomlLiteral::String(_)) => Some(self.parse_key_val()),
            Token::Newline => {
                self.next_tok();
                self.parse_stmt()
            }
            Token::EOF => None,
            tok => panic!(
                "Unexpected token: {:?}. Cannot parse this into a statement",
                tok
            ),
        }
    }

    fn parse_expr(&mut self) -> TomlExpr {
        match &self.cur_tok {
            Token::LBrac => todo!(),
            Token::LCurly => todo!(),
            Token::Literal(lit) => TomlExpr::Val(lit.clone()),
            tok => panic!(
                "Unexpected token: {:?}. Cannot parse this into an expression",
                tok
            ),
        }
    }

    fn parse_table(&mut self) -> TomlStmt {
        self.next_tok();
        let name = match self.next_tok {
            Token::Dot => self.parse_path(),
            _ => vec![self.cur_tok.to_string()],
        };
        self.next_tok();
        self.next_tok();
        // skip newline
        self.next_tok();
        let block = self.parse_table_block();
        TomlStmt::Table { name, body: block }
    }

    /// First token should be first stmt
    fn parse_table_block(&mut self) -> Vec<KeyVal> {
        let mut stmts = Vec::new();
        while self.cur_tok != Token::LBrac {
            match self.cur_tok {
                Token::Newline => {
                    self.next_tok();
                    continue;
                }
                Token::EOF => return stmts,
                _ => {
                    stmts.push(match self.parse_key_val() {
                        TomlStmt::KeyVal(stmt) => stmt,
                        _ => return stmts,
                    });
                }
            }
        }
        stmts
    }

    fn parse_key_val(&mut self) -> TomlStmt {
        let key = match self.next_tok {
            Token::Dot => self.parse_path(),
            _ => vec![self.cur_tok.to_string()],
        };
        expect_peek!(self.next_tok, Token::Assign);
        self.next_tok();
        expect_peek!(self.next_tok, Token::Literal(_));
        self.next_tok();
        let val = self.parse_expr();
        self.next_tok();
        TomlStmt::KeyVal(KeyVal { key, val })
    }

    fn parse_path(&mut self) -> Vec<String> {
        let mut path = vec![self.cur_tok.to_string()];
        while self.next_tok == Token::Dot {
            self.next_tok();
            expect_peek!(self.next_tok, Token::Ident(_));
            self.next_tok();
            path.push(self.cur_tok.to_string());
        }
        path
    }

    fn skip_newline(&mut self) {
        while self.cur_tok == Token::Newline {
            self.next_tok();
        }
    }

    fn next_tok(&mut self) {
        mem::swap(&mut self.cur_tok, &mut self.next_tok);
        self.next_tok = self.lexer.tokenize();
    }
}

#[macro_export]
macro_rules! expect_peek {
    ($expression:expr, $pattern:pat) => {
        match $expression {
            $pattern => (),
            _ => panic!(
                "Expected next token to be {}, got {:?} instead",
                stringify!($pattern),
                $expression
            ),
        }
    };
}
