#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use crate::{
        errors::FileHandlerError,
        files::FileHandler,
        map,
        snapshots::SnapshotTest,
    };

    #[test]
    fn filehandler() -> Result<(), FileHandlerError> {
        let fh = FileHandler::new("tests/test.txt")?;
        assert_eq!(&fh.content, "test for filehandler");
        Ok(())
    }

    #[test]
    fn map_macro() {
        let macro_map = map!("first" => 1, "second" => 2, "third" => 3);
        let mut manual_map = map!();
        manual_map.insert("first", 1);
        manual_map.insert("second", 2);
        manual_map.insert("third", 3);
        assert_eq!(macro_map, manual_map)
    }

    #[test]
    fn snapshots() {
        let snapshot = SnapshotTest::new("testing", PathBuf::from("tests/main.c"));
        snapshot.setup_dir();
        snapshot.create_snapshot();
    }
}
