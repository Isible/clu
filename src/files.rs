use std::{fs::File, io::Read, fmt::Display};

use crate::errors::{self, FileHandlerError};

#[derive(Debug)]
pub struct FileHandler {
    pub file_name: String,
    pub file_path: String,
    pub full_path: String,
    pub content: String,
}

impl FileHandler {
    pub fn new(path: &str) -> Result<Self, FileHandlerError> {
        let mut split_path: Vec<&str> = path.split('/').collect();
        let file_name = match split_path.pop() {
            Some(name) => (*name).to_owned(),
            None => return Err(FileHandlerError(Box::from(errors::FileNotFoundError(format!("The provided path: {} does not contain a valid file name.", path))))),
        };
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => return Err(FileHandlerError(Box::from(err))),
        };
        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(err) => return Err(FileHandlerError(Box::from(err))),
        };
        Ok(Self {
            file_name: file_name,
            file_path: split_path.join(""),
            full_path: path.to_owned(),
            content: buf,
        })
    }
}

impl Display for FileHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileHandler {{ file_name: {}, file_path: {}, full_path: {}, content: {} }}", self.file_name, self.file_path, self.full_path, self.content)
    }
}
