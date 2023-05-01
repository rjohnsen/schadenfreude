use std::fs;
use std::path::Path;
use sha256::{digest, try_digest};

#[derive(Debug, Default)]
pub struct Specimen {
    pub content: Vec<u8>,
    pub name: String,
    pub sha256: String,
}

impl Specimen {
    pub fn load(&mut self, path: &std::path::Path) {
        self.content = self.load_content(path);
        self.name = self.extract_filename(path);
        self.sha256 = self.get_sha256(path);
    }

    fn load_content(&mut self, path: &std::path::Path) -> Vec<u8> {
        return fs::read(path).unwrap();
    }
    
    fn extract_filename(&mut self, path: &std::path::Path) -> String {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        return String::from(file_name);
    }

    fn get_sha256(&mut self, path: &std::path::Path) -> String{
        return try_digest(path).unwrap();
    }
}