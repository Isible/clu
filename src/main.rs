use crate::error::{Error, throw};

mod error;

fn main() {
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
            Some("hehhehehe".to_string())
        }
    }

    throw(TestError)
}
