use std::{fs::File, io::Read, fmt::Display};

#[derive(Debug)]
pub struct FileHandler {
    pub file_name: String,
    pub file_path: String,
    pub full_path: String,
    pub content: String,
}

impl FileHandler {
    pub fn new(path: String) -> Self {
        let mut split_path: Vec<&str> = path.split('/').collect();
        let file_name = match split_path.pop() {
            Some(name) => (*name).to_owned(),
            None => panic!("The provided path: {} does not contain a valid file name.", path),
        };
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => panic!("Failed to open file from path: {}. This might be caused due to restrictive file permissions or an incorrect file path", path),
        };
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Failed to write file content to a string");
        Self {
            file_name,
            file_path: split_path.join(""),
            full_path: path,
            content: buf,
        }
    }
}

impl Display for FileHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileHandler {{ file_name: {}, file_path: {}, full_path: {}, content: {} }}", self.file_name, self.file_path, self.full_path, self.content)
    }
}
