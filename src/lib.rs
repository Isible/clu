mod file_handler;
mod literal;

#[cfg(test)]
mod tests {
    use crate::file_handler::{FileHandler, BuiltinExtensions};


    #[test]
    fn test() {
        let expected = FileHandler {
            name: "test".to_string(),
            extension: Box::from(BuiltinExtensions::TXT),
            path: "tests/test.txt".to_string(),
        };
        let file_handler = FileHandler::new("tests/test.txt".to_string());
        assert_eq!(expected.extension.literal(), file_handler.extension.literal());
        assert_eq!(expected.path, file_handler.path);
        assert_eq!(expected.name, file_handler.name);
    }
}
