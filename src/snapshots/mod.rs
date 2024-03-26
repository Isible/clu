use std::{fs, path::PathBuf};

pub struct SnapshotTest {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
}

impl SnapshotTest {
    /// `path` takes the path to the file that clu
    /// should create a snapshot of.
    pub fn new(name: &str, path: PathBuf) -> Self {
        Self {
            name: name.into(),
            path,
        }
    }

    pub fn setup_dir(&self) {
        if fs::metadata("snapshots").is_err() {
            fs::create_dir("snapshots").unwrap_or_else(|_| {
                panic!("Could not create directory for snapshots");
            });
        }
    }

    pub fn create_snapshot(&self) {
        let path: PathBuf = format!("snapshots/{}.snap", &self.name).into();
        let content_file = fs::read_to_string(&self.path).unwrap_or_else(|_| {
            panic!("Could not read file at path: {:?}", &self.path);
        });
        let content_snapshot = if path.exists() {
            fs::read_to_string(&path).unwrap_or_else(|_| {
                panic!("Could not read snapshot file");
            })
        } else {
            fs::write(path, content_file).unwrap_or_else(|_| {
                panic!("Could not write snapshot to file");
            });
            return;
        };
        let split = content_file.split('\n').collect::<Vec<&str>>();
        dbg!(&split);
        if content_file != content_snapshot {
            fs::write(&path, content_file).unwrap_or_else(|_| {
                panic!("Could not write snapshot to file");
            });
            println!("Changes!")
        }
        self.create_properties_file();
    }

    fn create_properties_file(&self) {
        let path: PathBuf = format!("snapshots/{}-props.toml", &self.name).into();
        let content = format!("creation_date={}", chrono::Local::now());
        fs::write(&path, content).unwrap_or_else(|_| {
            panic!("Could not write properties file");
        });
    }
}
