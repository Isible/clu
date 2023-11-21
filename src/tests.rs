#[cfg(test)]
mod tests {
    use crate::{file_handler::{FileHandler, BuiltinExtensions}, error::{Error, throw}};


    #[test]
    fn test_filehandler() {
        let expected = FileHandler {
            name: "test".to_string(),
            extension: Box::from(BuiltinExtensions::TXT),
            path: "tests/test.txt".to_string(),
            content: "test".to_string(),
        };
        let file_handler = FileHandler::new("tests/test.txt".to_string());
        assert_eq!(expected.extension.literal(), file_handler.extension.literal());
        assert_eq!(expected.path, file_handler.path);
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
                Some(vec![String::from("Additional Context for the t)est")])
            }

            fn tip(&self) -> Option<String> {
                None
            }
        }

        throw(TestError)
    }
}