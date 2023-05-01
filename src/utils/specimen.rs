use std::fs;
use std::path::Path;
use sha2::Sha256;
use sha2::Digest;
use sha1::Sha1;
use md5::Md5;

#[derive(Debug, Default)]
pub struct Specimen {
    pub content: Vec<u8>,
    pub name: String,
    pub fingerprint:Fingerprint
}

#[derive(Debug, Default)]
pub struct Fingerprint {
    pub sha256: String,
    pub sha1: String,
    pub md5: String
}

impl Specimen {
    pub fn load(&mut self, path: &std::path::Path) {
        self.content = self.load_content(path);
        self.name = self.extract_filename(path);
        self.fingerprint.sha256 = self.get_sha256();
        self.fingerprint.sha1 = self.get_sha1();
        self.fingerprint.md5 = self.get_md5();
    }

    fn load_content(&mut self, path: &std::path::Path) -> Vec<u8> {
        return fs::read(path).unwrap();
    }
    
    fn extract_filename(&mut self, path: &std::path::Path) -> String {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        return String::from(file_name);
    }

    fn get_sha256(&mut self) -> String{
        let hash = Sha256::digest(&self.content);
        return format!("{:x}", hash);
    }

    fn get_sha1(&mut self) -> String{
        let hash = Sha1::digest(&self.content);
        return format!("{:x}", hash);
    }

    fn get_md5(&mut self) -> String{
        let hash = Md5::digest(&self.content);
        return format!("{:x}", hash);
    }
}