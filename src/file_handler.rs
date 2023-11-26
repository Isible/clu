#![allow(unused)]

use std::{
    fmt::{Debug, Display},
    fs::{self, DirEntry, File},
    io::Read,
};

use crate::{
    error::{throw, Error},
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
    pub fn new_with_extension(path: String, extension: Box<dyn Extension>) -> Self {
        let paths: Vec<&str> = path.split("/").collect();
        // name with extension
        let name_raw: Vec<&str> = match paths.last() {
            Some(name_raw) => name_raw.split(".").collect(),
            // TODO: Throw actual error once error library is implemented
            None => todo!(),
        };
        // name without extension
        let name = match name_raw.first() {
            Some(name) => name.to_string(),
            // TODO: Throw actual error once error library is implemented
            None => todo!(),
        };
        let mut content_buffer = String::new();
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                throw(FileNotFoundError::new(path), true);
                panic!()
            }
        };
        file.read_to_string(&mut content_buffer);

        let full_path_len = path.len();
        // length of full path - length of the name and extension - remove the '/'
        let path_len = full_path_len - name.len() - extension.literal().to_string().len() - 2;

        let dir_path = &path[0..path_len];

        Self {
            name,
            path: dir_path.to_string(),
            extension,
            full_path: path,
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

pub struct FileNotFoundError {
    pub provided_path: String,
    pub similar_path: Option<String>,
}

type OptionalDirEntry = Option<Vec<Result<DirEntry, std::io::Error>>>;

impl FileNotFoundError {
    pub fn new(path: String) -> Self {
        let mut dirs: Vec<&str> = path.split("/").collect();
        let name = dirs.pop().unwrap();

        let new_path = dirs.join("/");

        let file_names: Vec<String> = match fs::read_dir(&new_path) {
            Ok(entries) => entries
                .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
                .collect(),
            Err(_) => {
                panic!("Error reading directory");
            }
        };

        let similar_path = util::find_most_similar_string(name, file_names);

        Self {
            provided_path: path,
            similar_path,
        }
    }
}

impl Error for FileNotFoundError {
    fn name(&self) -> &str {
        "File not found error"
    }

    fn desc(&self) -> String {
        format!(
            "A file with the name `{}` cannot be found",
            self.provided_path
        )
    }

    fn additional_ctx(&self) -> Option<Vec<String>> {
        match &self.similar_path {
            Some(sim_path) => Some(vec![format!("Found {} instead", sim_path)]),
            None => None,
        }
    }

    fn tip(&self) -> Option<String> {
        None
    }
}

pub struct FailedToOpenFileError {}
