mod file_handler;
mod literal;

#[cfg(test)]
mod tests {
    use crate::file_handler::{FileHandler, BuiltinExtensions};


    #[test]
    fn test() {
        FileHandler::new("test".to_string());
    }
}
