#![allow(unused)]

use std::{fmt::Debug, fs::File, io::Read};

use crate::literal::Literal;

pub struct FileHandler {
    pub name: String,
    pub extension: Box<dyn Extension>,
    pub path: String,
    pub content: String,
}

impl FileHandler {
    pub fn new_with_extension(path: String, extension: Box<dyn Extension>) -> Self {
        let paths: Vec<&str> = path.split("/").collect();
        // name with extension
        let name_raw: Vec<&str> = match paths.last() {
            Some(name_raw) => name_raw.split(".").collect(),
            // TODO: Throw actual error once error library is implemented
            None => todo!(),
        };
        let name = match name_raw.first() {
            Some(name) => name.to_string(),
            // TODO: Throw actual error once error library is implemented
            None => todo!(),
        };
        let mut content_buffer = String::new();
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => todo!(),
        };
        file.read_to_string(&mut content_buffer);
        Self {
            name,
            extension,
            path,
            content: content_buffer,
        }
    }

    pub fn new(path: String) -> Self {
        let paths: Vec<&str> = path.split("/").collect();
        // name with extension
        let name: Vec<&str> = match paths.last() {
            Some(name) => name.split(".").collect(),
            // TODO: Throw actual error once error library is implemented
            None => todo!(),
        };
        let extension_str = match name.last() {
            Some(extension) => extension,
            // TODO: Throw actual error once error library is implemented
            None => todo!(),
        };

        let extension = Box::from(BuiltinExtensions::from_literal(extension_str));

        Self::new_with_extension(path, extension)
    }
}

pub trait Extension: Literal {}

pub enum BuiltinExtensions {
    TXT,
    MD,
    TOML,
    XML,
    JSON,
    UNRECOGNIZED(String),
}

impl BuiltinExtensions {
    fn from_literal(literal: &str) -> Self {
        match literal {
            "json" => Self::JSON,
            "md" => Self::MD,
            "txt" => Self::TXT,
            "toml" => Self::TOML,
            "xml" => Self::XML,
            _ => Self::UNRECOGNIZED(literal.to_string()),
        }
    }
}

impl Extension for BuiltinExtensions {}

impl Literal for BuiltinExtensions {
    fn literal(&self) -> &str {
        match self {
            BuiltinExtensions::JSON => "json",
            BuiltinExtensions::TXT => "txt",
            BuiltinExtensions::MD => "md",
            BuiltinExtensions::TOML => "toml",
            BuiltinExtensions::XML => "xml",
            BuiltinExtensions::UNRECOGNIZED(lit) => lit,
        }
    }
}
