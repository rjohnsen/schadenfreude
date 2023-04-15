use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Specimen {
    pub content: Vec<u8>,
    pub name: String
}

impl Specimen {
    pub fn load(&mut self, specimen_path: &String) {
        self.content = Self::load_content(&specimen_path);
        self.name = Self::extract_filename(&specimen_path);
    }

    fn load_content(filepath: &String) -> Vec<u8> {
        return fs::read(&filepath).unwrap();
    }

    fn extract_filename(filepath: &String) -> String {
        let fname = Path::new(&filepath).file_name().unwrap().to_str().unwrap();
        return String::from(fname);
    }
}