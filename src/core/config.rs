use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::prelude::*;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    obsidian_vault: String
}

const MAIN_FOLDER: &str = "Schadenfreude";
const CONFIG_FILE: &str = ".config.toml";

impl Config {
    
    pub fn load(&mut self) {
        let main_folder = get_path();
        let config_file = Path::new(&main_folder).join(CONFIG_FILE);

        if main_folder.is_dir()  == false { 
            match fs::create_dir(&main_folder) {
                Ok(()) => println!("Directory created"),
                Err(e) => println!("{:?}", e)
            }
        }
        
        if config_file.is_file() == false {
            match self.create_config_file(&config_file) {
                Ok(()) => println!("Config is file"),
                Err(e) => println!("{:?}", e)
            }
        }

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

pub fn get_path() -> PathBuf {
    let home_folder = dirs::home_dir().unwrap();
    return Path::new(&home_folder).join(MAIN_FOLDER);
}
