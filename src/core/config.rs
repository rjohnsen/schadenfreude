use std::fs::File;
use std::io::prelude::*;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    obsidian_vault: String
}

impl Config {
    pub fn create_new(&mut self, path: std::path::PathBuf) -> std::io::Result<()> {
        let binding = toml::to_string(&self).unwrap();
        let toml = binding.as_bytes();
        let mut file = File::create(path)?;

        file.write_all(toml)?;
        Ok(())
    }

    pub fn load(&mut self, path: std::path::PathBuf) {
        let contents = std::fs::read_to_string(path);
        println!("{:?}", contents);
    }
}
