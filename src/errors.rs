use std::fmt::Display;

use crate::impl_error;

#[derive(Debug)]
pub struct FileHandlerError(pub Box<dyn Error>);

impl Error for FileHandlerError {}

impl Display for FileHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct FileNotFoundError(pub String);

impl_error!(FileNotFoundError);

impl Display for FileNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File at path: {} not found", self.0)
    }
}