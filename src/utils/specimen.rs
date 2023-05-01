use std::fs;
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

pub fn load(path: &std::path::Path) -> Specimen {
    let mut specimen = Specimen::default();

    specimen.content = load_content(path);
    specimen.name = extract_filename(path);
    specimen.fingerprint.sha256 = get_sha256(&specimen.content);
    specimen.fingerprint.sha1 = get_sha1(&specimen.content);
    specimen.fingerprint.md5 = get_md5(&specimen.content);

    return specimen;
}

fn load_content(path: &std::path::Path) -> Vec<u8> {
    return fs::read(path).unwrap();
}

fn extract_filename(path: &std::path::Path) -> String {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    return String::from(file_name);
}

fn get_sha256(content: &Vec<u8>) -> String{
    let hash = Sha256::digest(content);
    return format!("{:x}", hash);
}

fn get_sha1(content: &Vec<u8>) -> String{
    let hash = Sha1::digest(content);
    return format!("{:x}", hash);
}

fn get_md5(content: &Vec<u8>) -> String{
    let hash = Md5::digest(content);
    return format!("{:x}", hash);
}