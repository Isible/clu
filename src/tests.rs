#[cfg(test)]
mod tests {

    use crate::{files::FileHandler, errors::FileHandlerError};

    #[test]
    fn filehandler() -> Result<(), FileHandlerError> {
        let fh = FileHandler::new(&"tests/test.txt".into())?;
        assert_eq!(&fh.content, "test for filehandler");
        Ok(())
    }
}