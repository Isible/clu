#![macro_use]
pub mod tokens;

use std::collections::HashMap;

use self::tokens::Token;

pub struct BaseLexer {
    pub input: String,
    pub tokens: HashMap<String, Box<dyn Token>>,
}
