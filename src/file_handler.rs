#![allow(unused)]

use std::{
    error::Error,
    fmt::{Debug, Display},
    fs::{self, DirEntry, File},
    io::Read,
};

use crate::{
    literal::Literal,
    util,
};

/// The file handler allows you to easily
/// manage config, source and other files.
///
/// It provides extensible mechanics for extesnions
/// and tracking file locations
pub struct FileHandler {
    pub name: String,
    pub extension: Box<dyn Extension>,
    pub path: String,
    pub full_path: String,
    pub content: String,
}

impl Display for FileHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FileHandler {{ name: {}, extension: {}, path: {}, content: {} }}",
            self.name,
            self.extension.literal(),
            self.full_path,
            self.content
        )
    }
}

impl FileHandler {
    pub fn new_with_extension(
        path: String,
        extension: Box<dyn Extension>,
    ) -> Result<Self, FileHandlerError> {
        let paths: Vec<&str> = path.split("/").collect();
        // name with extension
        let name_raw: Vec<&str> = match paths.last() {
            Some(name_raw) => name_raw.split(".").collect(),
            // TODO: Throw actual error once error library is implemented
            None =>  return Err(FileHandlerError::new(path)),
        };
        // name without extension
        let name = match name_raw.first() {
            Some(name) => name.to_string(),
            // TODO: Throw actual error once error library is implemented
            None => return Err(FileHandlerError::new(path)),
        };
        let mut content_buffer = String::new();
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return Err(FileHandlerError::new(path)),
        };
        file.read_to_string(&mut content_buffer);

        let full_path_len = path.len();
        // length of full path - length of the name and extension - remove the '/'
        let path_len = full_path_len - name.len() - extension.literal().to_string().len() - 2;

        let dir_path = &path[0..path_len];

        Ok(Self {
            name,
            path: dir_path.to_string(),
            extension,
            full_path: path,
            content: content_buffer,
        })
    }

    pub fn new(path: String) -> Result<Self, FileHandlerError> {
        let paths: Vec<&str> = path.split("/").collect();
        // name with extension
        let name: Vec<&str> = match paths.last() {
            Some(name) => name.split(".").collect(),
            // TODO: Throw actual error once error library is implemented
            None => return Err(FileHandlerError::new(path)),
        };
        let extension_str = match name.last() {
            Some(extension) => extension,
            // TODO: Throw actual error once error library is implemented
            None => return Err(FileHandlerError::new(path)),
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
    fn literal(&self) -> String {
        match self {
            BuiltinExtensions::JSON => "json",
            BuiltinExtensions::TXT => "txt",
            BuiltinExtensions::MD => "md",
            BuiltinExtensions::TOML => "toml",
            BuiltinExtensions::XML => "xml",
            BuiltinExtensions::UNRECOGNIZED(lit) => lit,
        }.to_string()
    }
}

#[derive(Debug)]
pub struct FileHandlerError {
    path: String,
}

impl FileHandlerError {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl Display for FileHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("FileHandlerError{{ {} }}", self.path))
    }
}

impl Error for FileHandlerError {}
