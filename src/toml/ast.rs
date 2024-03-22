use std::collections::HashMap;

use super::tokens::TomlLiteral;

#[derive(Debug)]
pub enum TomlStmt {
    Table { name: Vec<String>, body: Vec<KeyVal> },
    KeyVal(KeyVal),
}

#[derive(Debug)]
pub enum TomlExpr {
    Table {
        content: HashMap<TomlExpr, TomlExpr>,
    },
    Array {
        content: Vec<TomlExpr>,
    },
    Val(TomlLiteral),
}

#[derive(Debug)]
pub struct KeyVal {
    pub key: Vec<String>,
    pub val: TomlExpr,
}
