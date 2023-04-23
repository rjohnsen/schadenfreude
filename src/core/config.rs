use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    obsidian_vault: String
}

impl Config {
    const MAIN_FOLDER: &str = "Schadenfreude";
    const CONFIG_FILE: &str = ".config.toml";
    
    pub fn load(&mut self) {
        let home_folder = dirs::home_dir().unwrap();
        let main_folder = Path::new(&home_folder).join(Config::MAIN_FOLDER);
        let config_file = Path::new(&main_folder).join(Config::CONFIG_FILE);

        if main_folder.is_dir()  == false { fs::create_dir(&main_folder); }
        if config_file.is_file() == false { self.create_config_file(&config_file); }

        let content = std::fs::read_to_string(config_file).unwrap();    
        let config: Config = toml::from_str::<Config>(&content).unwrap();
        *self = config;
    }

    fn create_config_file(&mut self, path: &std::path::PathBuf) -> std::io::Result<()> {
        let binding = toml::to_string(&self).unwrap();
        let toml = binding.as_bytes();
        let mut file = File::create(path)?;

        file.write_all(toml)?;
        Ok(())
    }
}
