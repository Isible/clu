#[cfg(test)]
mod tests {
    use crate::files::FileHandler;

    #[test]
    fn filehandler() {
        let fh = FileHandler::new("tests/test.txt".into());
        assert_eq!(&fh.content, "test for filehandler")
    }
}