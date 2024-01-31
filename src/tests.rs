#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::{files::FileHandler, errors::FileHandlerError, map};

    #[test]
    fn filehandler() -> Result<(), FileHandlerError> {
        let fh = FileHandler::new("tests/test.txt")?;
        assert_eq!(&fh.content, "test for filehandler");
        Ok(())
    }

    #[test]
    fn map_macro() {
        let macro_map = map!("first" => 1, "second" => 2, "third" => 3);
        let mut manual_map = HashMap::new();
        manual_map.insert("first", 1);
        manual_map.insert("second", 2);
        manual_map.insert("third", 3);
        assert_eq!(macro_map, manual_map)
    }
}