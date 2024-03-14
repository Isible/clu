#[cfg(test)]
mod tests {

    use std::{collections::HashMap, env, fs, path::PathBuf};

    use crate::{
        errors::FileHandlerError,
        files::FileHandler,
        map,
        snapshots::SnapshotTest,
        toml::{lexer::Lexer, tokens::Token},
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

    #[test]
    fn toml() {
        let content = fs::read("tests/test.toml").expect("Failed to read from file");
        let mut lexer = Lexer::new(String::from_utf8(content).unwrap());
        for _ in 0..10 {
            let tok = lexer.tokenize();
            println!("{}", tok);
        }
    }
}
