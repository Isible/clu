#[cfg(test)]
mod tests {
    use crate::{file_handler::{FileHandler, BuiltinExtensions}, error::{Error, throw}};

    #[test]
    fn test_filehandler() {
        let expected = FileHandler {
            name: "test".to_string(),
            path: "test".to_string(),
            extension: Box::from(BuiltinExtensions::TXT),
            full_path: "tests/test.txt".to_string(),
            content: "test".to_string(),
        };
        let file_handler = FileHandler::new(&"tests/test.txt".to_string()).expect("Failed to get file handler");
        println!("path: {}", file_handler.path);
        assert_eq!(expected.extension.literal(), file_handler.extension.literal());
        assert_eq!(expected.full_path, file_handler.full_path);
        assert_eq!(expected.name, file_handler.name);
        assert_eq!(expected.content, file_handler.content);
    }

    #[test]
    fn test_errors() {
        struct TestError;

        impl Error for TestError {
            fn name(&self) -> &str {
                "test"
            }

            fn desc(&self) -> String {
                String::from("Just a test")
            }

            fn additional_ctx(&self) -> Option<Vec<String>> {
                Some(vec![String::from("Additional Context for the test")])
            }

            fn tip(&self) -> Option<String> {
                None
            }
        }

        throw(TestError, false);
    }
}