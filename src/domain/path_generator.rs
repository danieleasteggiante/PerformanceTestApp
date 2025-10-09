use std::env;
use std::path::Path;

pub(crate) struct PathGenerator {
    path: String,
}

impl PathGenerator {
    pub fn from_string(base_path: &str) -> String {
        let current_dir = format!("{}{}", env::current_dir().unwrap().display(), base_path);
        if !Path::new(current_dir.as_str()).exists() {
            panic!("File not found at path: {}", base_path);
        }
        current_dir    
    }
}